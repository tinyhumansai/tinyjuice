//! End-to-end proof that the built TinyJuice cdylib is admitted and serves
//! compression and CCR retrieval through a real TinyBus broker.

use std::time::Duration;

use tinybus::Connection;
use tinybus::broker::Broker;
use tinybus::module::{ModuleHost, ModuleState};
use tinybus::transport::memory::MemoryBus;
use tinyjuice::types::CompressedOutput;
use tinyjuice_module::{BUS_NAME, OBJECT_PATH};

const EXPECTED_METHODS: &[&str] = &[
    "Install",
    "Detect",
    "Compress",
    "Compact",
    "Retrieve",
    "CacheStats",
];

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore = "requires TINYJUICE_TEST_MODULE to point at the built cdylib"]
async fn the_built_module_compresses_and_recovers_over_a_real_broker() {
    let artifact =
        std::env::var_os("TINYJUICE_TEST_MODULE").expect("TINYJUICE_TEST_MODULE must be set");
    let bus = MemoryBus::new();
    let broker = Broker::new();
    let broker_task = broker.spawn(bus.clone());
    let modules = ModuleHost::new(broker);

    let loaded = modules.load_file(artifact).expect("module should load");
    assert_eq!(loaded.name, "tinyjuice-module");
    assert_eq!(loaded.manifest.bus_name.as_str(), BUS_NAME);
    assert_eq!(loaded.manifest.object_path.as_str(), OBJECT_PATH);
    let methods: Vec<&str> = loaded
        .manifest
        .provides
        .iter()
        .flat_map(|interface| interface.methods.iter())
        .map(tinybus::MemberName::as_str)
        .collect();
    assert_eq!(methods, EXPECTED_METHODS);

    let client = Connection::connect(bus.connect().await.expect("memory transport"))
        .await
        .expect("client should connect");
    wait_until_serving(&client).await;
    let proxy = client
        .proxy(BUS_NAME, OBJECT_PATH, BUS_NAME)
        .expect("module proxy");

    proxy
        .call::<()>(
            "Install",
            (serde_json::json!({
                "options": {
                    "routerEnabled": true,
                    "ccrEnabled": true,
                    "searchEnabled": true,
                    "codeEnabled": true,
                    "htmlEnabled": true,
                    "mlTextEnabled": false,
                    "minBytesToCompress": 64,
                    "minBytesToCompressLog": 32,
                    "ccrMinTokens": 16,
                    "lossyWithoutCcr": false,
                    "maxInlineChars": null,
                    "codeTargetRatio": null,
                    "charsPerToken": 4.0
                },
                "maxCacheEntries": 8,
                "maxCacheBytes": 1048576,
                "ccrTtlSecs": null,
                "diskTierRoot": null
            }),),
        )
        .await
        .expect("install should succeed");

    let content = (0..200)
        .map(|index| {
            if index == 137 {
                format!("ERROR request failed id={index}\n")
            } else {
                format!("INFO request completed id={index}\n")
            }
        })
        .collect::<String>();
    let output: CompressedOutput = proxy
        .call(
            "Compress",
            (content.clone(), serde_json::json!({ "explicit": "log" })),
        )
        .await
        .expect("compress should succeed");
    assert!(output.applied);
    let token = output.ccr_token.expect("compression should be recoverable");
    let recovered: Option<String> = proxy
        .call("Retrieve", (token, Option::<serde_json::Value>::None))
        .await
        .expect("retrieve should succeed");
    assert_eq!(recovered, Some(content));
    assert!(matches!(modules.list()[0].state, ModuleState::Ready));
    broker_task.abort();
}

async fn wait_until_serving(client: &Connection) {
    tokio::time::timeout(Duration::from_secs(5), async {
        loop {
            if client
                .list_names()
                .await
                .expect("names should list")
                .iter()
                .any(|name| name.as_str() == BUS_NAME)
            {
                break;
            }
            tokio::task::yield_now().await;
        }
    })
    .await
    .expect("module should become ready");
}
