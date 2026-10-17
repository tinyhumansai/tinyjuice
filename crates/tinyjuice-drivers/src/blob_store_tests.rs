use super::*;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use tinystoragedrivers_core::{Capability, MemoryStorage, Scope, StorageBackend};

use crate::faulty::Faulty;

fn clocked() -> (MemoryStorage, Arc<AtomicU64>) {
    let now = Arc::new(AtomicU64::new(1_000));
    let reading = Arc::clone(&now);
    (
        MemoryStorage::with_clock(Arc::new(move || reading.load(Ordering::SeqCst))),
        now,
    )
}

fn handle(storage: &MemoryStorage) -> Arc<dyn BlobStore> {
    Arc::clone(storage.for_scope(&Scope::local()).unwrap().blobs())
}

fn faulty(storage: &MemoryStorage, ttl: bool) -> (Arc<dyn BlobStore>, Arc<AtomicBool>) {
    let fail = Arc::new(AtomicBool::new(false));
    let blobs: Arc<dyn BlobStore> = Arc::new(Faulty {
        inner: handle(storage),
        fail: Arc::clone(&fail),
        ttl,
    });
    (blobs, fail)
}

#[test]
fn an_original_round_trips_under_its_content_token() {
    let storage = MemoryStorage::new();
    let store = BlobCcrStore::new(handle(&storage), None).unwrap();
    let put = store.put("hello world");
    assert!(put.retained());
    assert_eq!(put.token(), short_hash("hello world"));
    assert_eq!(store.get(put.token()).as_deref(), Some("hello world"));
    let listed = tinystoragedrivers_core::Blocking::new()
        .unwrap()
        .run({
            let blobs = handle(&storage);
            async move { blobs.list(CCR_PREFIX).await }
        })
        .unwrap()
        .unwrap();
    assert_eq!(listed.len(), 1);
    assert_eq!(listed[0].key, format!("{CCR_PREFIX}/{}", put.token()));
}

#[test]
fn identical_content_is_idempotent_and_distinct_content_is_separate() {
    let storage = MemoryStorage::new();
    let store = BlobCcrStore::new(handle(&storage), None).unwrap();
    let a = store.put("same");
    let b = store.put("same");
    let c = store.put("other");
    assert_eq!(a.token(), b.token());
    assert_ne!(a.token(), c.token());
    assert_eq!(store.get(c.token()).as_deref(), Some("other"));
}

#[test]
fn an_original_expires_when_its_ttl_has_elapsed() {
    let (storage, now) = clocked();
    let store = BlobCcrStore::new(handle(&storage), Some(Duration::from_millis(500))).unwrap();
    let put = store.put("short lived");
    assert!(put.retained());
    now.store(1_499, Ordering::SeqCst);
    assert_eq!(store.get(put.token()).as_deref(), Some("short lived"));
    now.store(1_500, Ordering::SeqCst);
    assert_eq!(store.get(put.token()), None);
}

#[test]
fn re_offloading_restarts_the_lifetime() {
    let (storage, now) = clocked();
    let store = BlobCcrStore::new(handle(&storage), Some(Duration::from_millis(500))).unwrap();
    let put = store.put("refreshed");
    now.store(1_400, Ordering::SeqCst);
    store.put("refreshed");
    now.store(1_800, Ordering::SeqCst);
    assert_eq!(store.get(put.token()).as_deref(), Some("refreshed"));
    now.store(1_900, Ordering::SeqCst);
    assert_eq!(store.get(put.token()), None);
}

#[test]
fn a_zero_ttl_is_never_advertised_as_retained() {
    let storage = MemoryStorage::new();
    let store = BlobCcrStore::new(handle(&storage), Some(Duration::ZERO)).unwrap();
    let put = store.put("gone on arrival");
    assert!(!put.retained());
    assert_eq!(store.get(put.token()), None);
}

#[test]
fn a_ttl_needs_a_handle_that_can_expire_blobs() {
    let storage = MemoryStorage::new();
    let (blobs, _) = faulty(&storage, false);
    let err = BlobCcrStore::new(Arc::clone(&blobs), Some(Duration::from_secs(5))).unwrap_err();
    assert_eq!(
        err.kind(),
        tinystoragedrivers_core::ErrorKind::Unsupported(Capability::BlobTtl)
    );
    assert!(BlobCcrStore::new(blobs, None).is_ok());
}

#[test]
fn malformed_tokens_never_reach_the_backend() {
    let storage = MemoryStorage::new();
    let store = BlobCcrStore::new(handle(&storage), None).unwrap();
    for token in [
        "",
        "../../state/config.toml",
        "tinyjuice_ccr/abc",
        "juice/ccr/abc",
        "zz",
        &"g".repeat(32),
    ] {
        assert_eq!(store.get(token), None, "{token}");
        assert!(!store.delete(token), "{token}");
    }
}

#[test]
fn ranges_clamp_like_the_in_memory_store() {
    let storage = MemoryStorage::new();
    let store = BlobCcrStore::new(handle(&storage), None).unwrap();
    let put = store.put("one\ntwo\nthree\nfour");
    let t = put.token();
    assert_eq!(
        store.get_range(t, 1, 3, RangeUnit::Lines).as_deref(),
        Some("two\nthree")
    );
    assert_eq!(
        store.get_range(t, 0, 3, RangeUnit::Bytes).as_deref(),
        Some("one")
    );
    assert_eq!(
        store.get_range(t, 3, 3, RangeUnit::Bytes).as_deref(),
        Some("")
    );
    assert_eq!(store.get_range("nothex", 0, 3, RangeUnit::Bytes), None);
}

#[test]
fn non_ascii_bytes_round_trip_and_ranges_stay_on_char_boundaries() {
    let storage = MemoryStorage::new();
    let store = BlobCcrStore::new(handle(&storage), None).unwrap();
    let put = store.put("héllo");
    assert_eq!(store.get(put.token()).as_deref(), Some("héllo"));
    // byte 2 is inside the two-byte 'é'
    assert_eq!(
        store
            .get_range(put.token(), 0, 2, RangeUnit::Bytes)
            .as_deref(),
        Some("h")
    );
}

#[test]
fn delete_reports_whether_the_original_existed() {
    let storage = MemoryStorage::new();
    let store = BlobCcrStore::new(handle(&storage), None).unwrap();
    let put = store.put("to delete");
    assert!(store.delete(put.token()));
    assert!(!store.delete(put.token()));
    assert_eq!(store.get(put.token()), None);
}

#[test]
fn backend_failures_are_not_retained_and_read_as_absent() {
    let storage = MemoryStorage::new();
    let (blobs, fail) = faulty(&storage, true);
    let store = BlobCcrStore::new(blobs, Some(Duration::from_secs(60))).unwrap();
    let ok = store.put("stored while healthy");
    assert!(ok.retained());
    fail.store(true, Ordering::SeqCst);
    assert!(!store.put("lost").retained());
    assert_eq!(store.get(ok.token()), None);
    assert!(!store.delete(ok.token()));
    fail.store(false, Ordering::SeqCst);
    assert_eq!(
        store.get(ok.token()).as_deref(),
        Some("stored while healthy")
    );
}

#[test]
fn a_plain_put_path_is_used_without_a_ttl_even_when_failing_ttl_calls() {
    let storage = MemoryStorage::new();
    let (blobs, _) = faulty(&storage, false);
    let store = BlobCcrStore::new(blobs, None).unwrap();
    assert!(store.put("no lifetime").retained());
}

#[tokio::test(flavor = "multi_thread")]
async fn it_works_from_inside_a_tokio_runtime() {
    let storage = MemoryStorage::new();
    let store = BlobCcrStore::new(handle(&storage), None).unwrap();
    let put = store.put("inside runtime");
    assert_eq!(store.get(put.token()).as_deref(), Some("inside runtime"));
}

#[test]
fn debug_hides_the_handle() {
    let storage = MemoryStorage::new();
    let store = BlobCcrStore::new(handle(&storage), None).unwrap();
    assert!(format!("{store:?}").starts_with("BlobCcrStore"));
}

#[tokio::test]
async fn a_migrated_original_is_readable_through_the_store() {
    use tinyjuice::cache::store::{CcrStore, short_hash};
    let backend = MemoryStorage::new();
    let storage = backend.for_scope(&Scope::local()).unwrap();
    let token = short_hash("legacy original");
    storage
        .blobs()
        .put(
            &format!("tinyjuice_ccr/{token}"),
            b"legacy original".to_vec(),
            Some("text/plain; charset=utf-8"),
        )
        .await
        .unwrap();
    tinystoragedrivers_core::schema::Registry::compose([crate::storage_schema()])
        .unwrap()
        .migrate(
            &storage,
            &tinystoragedrivers_core::schema::MigrateOptions::new(),
        )
        .await
        .unwrap();

    let blobs = std::sync::Arc::clone(storage.blobs());
    let got =
        tokio::task::spawn_blocking(move || BlobCcrStore::new(blobs, None).unwrap().get(&token))
            .await
            .unwrap();
    assert_eq!(got.as_deref(), Some("legacy original"));
}
