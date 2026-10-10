//! End-to-end proof that the built TinyJuice cdylib is admitted and serves
//! compression and CCR retrieval through a real TinyBus broker.

use std::time::Duration;

use tinybus::Connection;
use tinybus::broker::Broker;
use tinybus::module::{ModuleHost, ModuleState};
use tinybus::transport::memory::MemoryBus;
use tinyjuice::types::CompressedOutput;
use tinyjuice_module::{BUS_NAME, OBJECT_PATH};

const EXPECTED_METHODS: &[&str] = tinyjuice_bus::METHODS;

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
        .call(
            "Retrieve",
            (token.clone(), Option::<serde_json::Value>::None),
        )
        .await
        .expect("retrieve should succeed");
    assert_eq!(recovered, Some(content));
    typed_queries_execute_in_the_loaded_artifact(&proxy, &token).await;

    assert!(matches!(modules.list()[0].state, ModuleState::Ready));

    compact_with_calls_back_to_the_host_for_a_focused_summary(&client, &proxy).await;
    broker_task.abort();
}

/// The host half of the summary stage, as a test host serves it: answers
/// `MlHost.Generate` with a fixed note and remembers the prompt it was sent.
#[derive(Clone)]
struct TestHost {
    prompts: std::sync::Arc<std::sync::Mutex<Vec<String>>>,
}

#[tinybus::interface(name = "ai.tinyhumans.tinyjuice.MlHost")]
impl TestHost {
    async fn generate(
        &self,
        request: tinyjuice_bus::wire::GenerateRequest,
    ) -> tinybus::Result<Option<String>> {
        self.prompts.lock().unwrap().push(request.prompt);
        Ok(Some("the limit is 60 requests a minute".to_string()))
    }
}

/// `CompactWith` over the same broker: the module asks the host's
/// `MlHost.Generate` for a summary written for the caller's focus. Runs inside
/// the one test above, because a process loads the cdylib once.
async fn compact_with_calls_back_to_the_host_for_a_focused_summary(
    client: &Connection,
    proxy: &tinybus::Proxy,
) {
    let host = TestHost {
        prompts: Default::default(),
    };
    client
        .serve_at(
            tinyjuice_bus::ML_HOST_PATH.try_into().expect("host path"),
            host.clone(),
        )
        .await
        .expect("host should serve");
    client
        .request_name(tinyjuice_bus::ML_HOST_NAME)
        .await
        .expect("host name");
    proxy
        .call::<()>(
            "Install",
            (serde_json::json!({
                "options": {
                    "ccrEnabled": true,
                    "llmSummaryEnabled": true,
                    "llmSummaryMode": "auto",
                    "llmSummaryThresholdTokens": 10
                },
                "maxCacheEntries": 8,
                "maxCacheBytes": 1048576,
                "ccrTtlSecs": null,
                "diskTierRoot": null
            }),),
        )
        .await
        .expect("install should succeed");

    let content = "Requests are rate limited per key. ".repeat(40);
    let response: tinyjuice_bus::wire::CompactResponse = proxy
        .call(
            "CompactWith",
            (serde_json::json!({
                "content": content,
                "toolName": "web_fetch",
                "enabled": false,
                "focus": "the rate limits",
                "contextToken": "turn-1",
                "scope": "e2e"
            }),),
        )
        .await
        .expect("CompactWith should succeed");

    assert_eq!(response.compressor, "llm_summary");
    assert!(
        response
            .text
            .starts_with("the limit is 60 requests a minute")
    );
    assert!(response.text.contains("juice_retrieve"));
    assert!(response.notice.is_none());
    let prompts = host.prompts.lock().unwrap().clone();
    assert_eq!(prompts.len(), 1);
    assert!(prompts[0].contains("Caller focus: the rate limits"));
    let query = tinyjuice_bus::wire::QueryRequest {
        target: tinyjuice_bus::wire::QueryTarget::Content {
            content: "Requests are rate limited per key. ".repeat(40),
        },
        op: serde_json::from_value(
            serde_json::json!({"op":"summarize","hint":"rate limits for this artifact"}),
        )
        .unwrap(),
        limits: Default::default(),
        context_token: Some("query-turn".into()),
        scope: Some("query-e2e".into()),
    };
    let reply: tinyjuice_bus::wire::QueryResponse = proxy.call("Query", (query,)).await.unwrap();
    assert!(
        matches!(reply, Ok(tinyjuice_bus::repl::ReplOutput::Text { text }) if text == "the limit is 60 requests a minute")
    );
    let prompts = host.prompts.lock().unwrap().clone();
    assert_eq!(prompts.len(), 2);
    assert!(prompts[1].contains("rate limits for this artifact"));
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

/// Every new member is called through the separately loaded artifact.
async fn typed_queries_execute_in_the_loaded_artifact(proxy: &tinybus::Proxy, token: &str) {
    use tinyjuice_bus::repl::{ReplLimits, ReplOp, ReplOutput};
    use tinyjuice_bus::wire::{QueryError, QueryRequest, QueryResponse, QueryTarget};
    let op: ReplOp =
        serde_json::from_value(serde_json::json!({"op": "find", "query": "ERROR"})).unwrap();
    let request = |target| QueryRequest {
        target,
        op: op.clone(),
        limits: ReplLimits::default(),
        context_token: None,
        scope: None,
    };
    let reply: QueryResponse = proxy
        .call(
            "Query",
            (request(QueryTarget::Handle {
                token: token.into(),
            }),),
        )
        .await
        .unwrap();
    let Ok(ReplOutput::Lines { hits, .. }) = reply else {
        panic!("expected stored log hits")
    };
    assert_eq!(hits.len(), 1);
    assert_eq!(hits[0].line, 138);
    let reply: QueryResponse = proxy
        .call(
            "Query",
            (request(QueryTarget::Content {
                content: "first\nERROR artifact\nlast".into(),
            }),),
        )
        .await
        .unwrap();
    let Ok(ReplOutput::Lines { hits, .. }) = reply else {
        panic!("expected artifact hits")
    };
    assert_eq!(hits[0].line, 2);
    assert_eq!(hits[0].text, "ERROR artifact");
    let gone: QueryResponse = proxy
        .call(
            "Query",
            (request(QueryTarget::Handle {
                token: "not-cached".into(),
            }),),
        )
        .await
        .unwrap();
    assert_eq!(gone, Err(QueryError::HandleNotFound));
    let before: tinyjuice_bus::wire::CacheStats = proxy.call("CacheStats", ()).await.unwrap();
    let mut capped = request(QueryTarget::Content {
        content: "ERROR one\nERROR two\nERROR three".into(),
    });
    capped.limits.max_hits = 1;
    let reply: QueryResponse = proxy.call("Query", (capped,)).await.unwrap();
    let Ok(ReplOutput::Lines { hits, truncated }) = reply else {
        panic!("expected capped lines")
    };
    assert_eq!(hits.len(), 1);
    assert_eq!(truncated, 2);
    let after: tinyjuice_bus::wire::CacheStats = proxy.call("CacheStats", ()).await.unwrap();
    assert_eq!(
        before.entries, after.entries,
        "supplied content must not create CCR entries"
    );
    assert_eq!(before.bytes, after.bytes);
    for (op, expected) in [(
        serde_json::json!({"op":"find","query":""}),
        QueryError::EmptyQuery,
    )] {
        let mut invalid = request(QueryTarget::Content {
            content: "artifact".into(),
        });
        invalid.op = serde_json::from_value(op).unwrap();
        let reply: QueryResponse = proxy.call("Query", (invalid,)).await.unwrap();
        assert_eq!(reply, Err(expected));
    }
    let mut invalid = request(QueryTarget::Content {
        content: "artifact".into(),
    });
    invalid.op =
        serde_json::from_value(serde_json::json!({"op":"find","mode":"regex","query":"["}))
            .unwrap();
    let reply: QueryResponse = proxy.call("Query", (invalid,)).await.unwrap();
    assert!(matches!(reply, Err(QueryError::InvalidPattern(_))));
    let mut unlimited = request(QueryTarget::Content {
        content: "ERROR bounded\n".repeat(100),
    });
    unlimited.limits.max_hits = usize::MAX;
    unlimited.limits.max_lines = usize::MAX;
    unlimited.limits.max_line_chars = usize::MAX;
    unlimited.limits.max_output_chars = usize::MAX;
    unlimited.limits.regex_size_limit = usize::MAX;
    let reply: QueryResponse = proxy.call("Query", (unlimited,)).await.unwrap();
    let Ok(ReplOutput::Lines { hits, truncated }) = reply else {
        panic!("expected bounded hits")
    };
    assert_eq!(hits.len(), 50);
    assert_eq!(truncated, 50);
    let mut context_query = request(QueryTarget::Content {
        content: "before\n".repeat(10) + "ERROR needle\n" + &"after\n".repeat(10),
    });
    context_query.op = serde_json::from_value(serde_json::json!({
        "op":"find", "mode":"grep", "query":"ERROR", "context":usize::MAX
    }))
    .unwrap();
    context_query.limits.max_lines = 3;
    let context_reply: QueryResponse = proxy.call("Query", (context_query,)).await.unwrap();
    let Ok(ReplOutput::Lines { hits, truncated }) = context_reply else {
        panic!("expected bounded contextual hits")
    };
    assert_eq!(hits.len(), 3);
    assert!(
        hits.iter()
            .any(|hit| hit.line == 11 && hit.text == "ERROR needle")
    );
    assert!(truncated > 0);
    let oversized_query = request(QueryTarget::Content {
        content: "x".repeat(tinyjuice_bus::wire::MAX_QUERY_CONTENT_BYTES + 1),
    });
    let oversized_reply: QueryResponse = proxy.call("Query", (oversized_query,)).await.unwrap();
    assert_eq!(oversized_reply, Err(QueryError::InputTooLarge));
    let oversized: tinyjuice_bus::wire::HtmlResponse = proxy
        .call(
            "ExtractHtml",
            ("x".repeat(tinyjuice_bus::wire::MAX_HTML_INPUT_BYTES + 1),),
        )
        .await
        .unwrap();
    assert_eq!(
        oversized,
        Err(tinyjuice_bus::wire::HtmlError::InputTooLarge)
    );
    let legacy: String = proxy
        .call("Repl", (token, r#"{"op":"find","query":"ERROR"}"#))
        .await
        .unwrap();
    assert_eq!(
        serde_json::from_str::<ReplOutput>(&legacy).unwrap(),
        ReplOutput::Lines {
            hits: vec![tinyjuice_bus::repl::Hit {
                line: 138,
                text: "ERROR request failed id=137".into()
            }],
            truncated: 0
        }
    );
    let markdown: tinyjuice_bus::wire::HtmlResponse = proxy.call("ExtractHtml", ("<html><body><h1>Heading</h1><p>Read <a href=\"/details\">details</a></p><script>secret()</script></body></html>",)).await.unwrap();
    let markdown = markdown.expect("bounded HTML extraction");
    assert!(markdown.contains("# Heading"));
    assert!(markdown.contains("[details](/details)"));
    assert!(!markdown.contains("secret()"));
}
