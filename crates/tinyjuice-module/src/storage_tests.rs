//! Tests for the CCR durable tier on host storage: blobs on the host port,
//! expiry, the in-memory fallback, and single-worker safety.

use super::*;
use tinybus::broker::Broker;
use tinybus::transport::memory::MemoryBus;
use tinyjuice::cache::store::short_hash;
use tinystoragedrivers_busproxy::{PerCallerEndpoint, ServedStorage, serve};
use tinystoragedrivers_client::StorageServer;
use tinystoragedrivers_core::{MemoryStorage, Scope, ScopedStorage};

/// Serializes the tests that `install`, which reconfigures process-global cache
/// state.
pub(crate) static INSTALL: tokio::sync::Mutex<()> = tokio::sync::Mutex::const_new(());

type TestResult = Result<(), Box<dyn std::error::Error>>;

struct Rig {
    module: Connection,
    backend: MemoryStorage,
    _served: Option<ServedStorage>,
}

async fn rig(serve_storage: bool) -> Result<Rig, Box<dyn std::error::Error>> {
    let bus = MemoryBus::new();
    Broker::new().spawn(bus.clone());
    let backend = MemoryStorage::new();
    let host = Connection::connect(bus.connect().await?).await?;
    let served = if serve_storage {
        let bound = backend.clone();
        let endpoint = PerCallerEndpoint::new(move |_sender| {
            let scoped: ScopedStorage = bound.for_scope(&Scope::local()).ok()?;
            Some(StorageServer::new(scoped))
        });
        Some(serve(&host, Arc::new(endpoint)).await?)
    } else {
        None
    };
    let module = Connection::connect(bus.connect().await?).await?;
    // Keep the host connection alive for the life of the broker.
    std::mem::forget(host);
    Ok(Rig {
        module,
        backend,
        _served: served,
    })
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn originals_land_as_blobs_on_the_host_port() -> TestResult {
    let rig = rig(true).await?;
    let store = durable_tier(rig.module.clone(), Some(Duration::from_secs(600)))
        .await
        .ok_or("host serves storage")?;
    let content = "ccr original on the host blob port\nsecond line";
    let put = store.put(content);
    assert!(put.retained());

    let host_blobs = Arc::clone(rig.backend.for_scope(&Scope::local())?.blobs());
    let key = format!("{}/{}", tinyjuice_drivers::CCR_PREFIX, short_hash(content));
    let blob = host_blobs.get(&key).await?.ok_or("blob is on the host")?;
    assert_eq!(blob.bytes, content.as_bytes());

    assert_eq!(store.get(put.token()).as_deref(), Some(content));
    assert_eq!(
        store
            .get_range(put.token(), 1, 2, RangeUnit::Lines)
            .as_deref(),
        Some("second line")
    );
    Ok(())
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn without_a_storage_host_there_is_no_durable_tier() -> TestResult {
    let rig = rig(false).await?;
    assert!(durable_tier(rig.module.clone(), None).await.is_none());
    Ok(())
}

/// The module has two workers at most; a store call made directly from async
/// code must not wait on a bus reply only that worker could deliver.
#[tokio::test(flavor = "multi_thread", worker_threads = 1)]
async fn store_calls_on_the_only_worker_do_not_deadlock() -> TestResult {
    let rig = rig(true).await?;
    let store = durable_tier(rig.module.clone(), None)
        .await
        .ok_or("host serves storage")?;
    tokio::time::timeout(Duration::from_secs(20), async {
        let put = store.put("single worker original");
        assert!(put.retained());
        assert_eq!(
            store.get(put.token()).as_deref(),
            Some("single worker original")
        );
    })
    .await?;
    Ok(())
}

fn install_request(durable: bool) -> tinyjuice_bus::wire::InstallRequest {
    tinyjuice_bus::wire::InstallRequest {
        options: tinyjuice::types::CompressOptions::default(),
        max_cache_entries: 8,
        max_cache_bytes: 1024 * 1024,
        ccr_ttl_secs: Some(600),
        disk_tier_root: durable.then(|| "ignored-host-path".to_string()),
    }
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn install_moves_the_disk_tier_onto_host_blobs_and_off_again() -> TestResult {
    use crate::service::Compression;
    let _install = INSTALL.lock().await;
    let rig = rig(true).await?;
    let service = Compression {
        connection: Some(rig.module.clone()),
    };
    let host_blobs = Arc::clone(rig.backend.for_scope(&Scope::local())?.blobs());

    service.install(install_request(true)).await?;
    let content = "install wired durable tier fixture original";
    let token = tinyjuice::cache::offload(content);
    let key = format!("{}/{token}", tinyjuice_drivers::CCR_PREFIX);
    assert!(
        host_blobs.get(&key).await?.is_some(),
        "offload mirrored to host"
    );

    // Switched off: new originals stay in memory only.
    service.install(install_request(false)).await?;
    let memory_only = "install cleared durable tier fixture original";
    let token = tinyjuice::cache::offload(memory_only);
    let key = format!("{}/{token}", tinyjuice_drivers::CCR_PREFIX);
    assert!(host_blobs.get(&key).await?.is_none(), "nothing written");
    Ok(())
}

/// A blob handle that reports no optional abilities (no expiry), over memory.
#[derive(Debug)]
struct NoExpiry(Arc<dyn BlobStore>);

#[async_trait::async_trait]
impl BlobStore for NoExpiry {
    async fn put(
        &self,
        key: &str,
        bytes: Vec<u8>,
        content_type: Option<&str>,
    ) -> tinystoragedrivers_core::Result<tinystoragedrivers_core::BlobMeta> {
        self.0.put(key, bytes, content_type).await
    }

    async fn get(
        &self,
        key: &str,
    ) -> tinystoragedrivers_core::Result<Option<tinystoragedrivers_core::Blob>> {
        self.0.get(key).await
    }

    async fn get_range(
        &self,
        key: &str,
        range: std::ops::Range<u64>,
    ) -> tinystoragedrivers_core::Result<Option<Vec<u8>>> {
        self.0.get_range(key, range).await
    }

    async fn head(
        &self,
        key: &str,
    ) -> tinystoragedrivers_core::Result<Option<tinystoragedrivers_core::BlobMeta>> {
        self.0.head(key).await
    }

    async fn delete(&self, key: &str) -> tinystoragedrivers_core::Result<bool> {
        self.0.delete(key).await
    }

    async fn list(
        &self,
        prefix: &str,
    ) -> tinystoragedrivers_core::Result<Vec<tinystoragedrivers_core::BlobMeta>> {
        self.0.list(prefix).await
    }
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn a_handle_that_cannot_expire_blobs_gets_no_ttl_tier_but_a_keep_forever_one() -> TestResult {
    let backend = MemoryStorage::new();
    let blobs: Arc<dyn BlobStore> = Arc::new(NoExpiry(Arc::clone(
        backend.for_scope(&Scope::local())?.blobs(),
    )));
    assert!(
        tier_on(Arc::clone(&blobs), Some(Duration::from_secs(60)))
            .await
            .is_none()
    );
    assert!(tier_on(blobs, None).await.is_some());
    Ok(())
}

/// Off a multi-thread runtime there is no worker to hand off, so the call runs
/// in place.
#[tokio::test(flavor = "current_thread")]
async fn store_calls_run_in_place_on_a_current_thread_runtime() {
    struct Fixed;
    impl CcrStore for Fixed {
        fn put(&self, content: &str) -> CcrPutResult {
            tinyjuice::cache::store::MemoryCcrStore::default().put(content)
        }
        fn get(&self, _token: &str) -> Option<String> {
            Some("fixed".into())
        }
        fn get_range(&self, _: &str, _: usize, _: usize, _: RangeUnit) -> Option<String> {
            Some("range".into())
        }
    }
    let store = OffWorker(Fixed);
    assert!(store.put("x").retained());
    assert_eq!(store.get("t").as_deref(), Some("fixed"));
    assert_eq!(
        store.get_range("t", 0, 1, RangeUnit::Lines).as_deref(),
        Some("range")
    );
}

#[test]
fn the_module_grant_is_its_schema_at_module_scope() {
    use tinystoragedrivers_core::schema::{Item, ScopeClass};
    let schema = crate::storage_schema();
    let grant: Vec<(&str, ScopeClass)> = schema
        .items
        .iter()
        .map(|item| match item {
            Item::Blobs { prefix, scope } => (prefix.as_str(), *scope),
            other => panic!("the module stores only blobs, got {other:?}"),
        })
        .collect();
    assert_eq!(grant, [("juice/ccr/", ScopeClass::Module)]);
}
