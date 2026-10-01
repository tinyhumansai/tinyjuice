use super::*;

#[test]
fn round_trips() {
    let original = "ccr round-trip unique payload alpha ".repeat(50);
    let hash = offload(&original);
    assert_eq!(hash.len(), HASH_BYTES * 2);
    assert_eq!(retrieve(&hash).as_deref(), Some(original.as_str()));
}

#[test]
fn idempotent_hash() {
    let a = offload("ccr idempotent unique payload bravo content");
    let b = offload("ccr idempotent unique payload bravo content");
    assert_eq!(a, b);
}

#[test]
fn missing_hash_is_none() {
    assert!(retrieve("ffffffffffffffffffffffffffffffff").is_none());
}

#[test]
fn byte_cap_evicts_oldest() {
    let mut inner = Inner::default();
    // 10 entries of 100 bytes; cap at 500 bytes ⇒ keep ~5 newest.
    for i in 0..10 {
        inner.insert(format!("h{i}"), "x".repeat(100), 1000, 500);
    }
    assert!(
        inner.total_bytes <= 500,
        "byte cap held: {}",
        inner.total_bytes
    );
    assert!(!inner.map.contains_key("h0"), "oldest evicted");
    assert!(inner.map.contains_key("h9"), "newest retained");
}

#[test]
fn oversized_single_entry_is_not_retained() {
    // One entry larger than the byte cap can't be kept; insert reports
    // non-retention so the router won't advertise it as recoverable.
    let mut inner = Inner::default();
    let retained = inner.insert("big".into(), "x".repeat(200), 100, 100);
    assert!(!retained, "oversized single entry must report not-retained");
    assert!(!inner.map.contains_key("big"));
    assert_eq!(inner.total_bytes, 0);
}

#[test]
fn rejects_path_traversal_tokens() {
    // Non-hex / wrong-length tokens are rejected before any disk join.
    assert!(!is_valid_token("../../state/config.toml"));
    assert!(!is_valid_token("..%2f..%2fetc"));
    assert!(!is_valid_token("deadbeef")); // too short
    assert!(!is_valid_token(&"g".repeat(32))); // non-hex
    assert!(is_valid_token(&"a1b2c3d4".repeat(4))); // 32 hex chars
    // retrieve() returns None for an invalid token regardless of cache state.
    assert!(retrieve("../../state/config.toml").is_none());
}

#[test]
fn within_cap_entry_is_retained() {
    let mut inner = Inner::default();
    assert!(inner.insert("ok".into(), "x".repeat(50), 100, 100));
    assert!(inner.map.contains_key("ok"));
}

#[test]
fn entry_cap_evicts_oldest() {
    let mut inner = Inner::default();
    for i in 0..60 {
        inner.insert(format!("e{i}"), format!("content-{i}"), 50, usize::MAX);
    }
    assert!(inner.map.len() <= 50);
    assert!(!inner.map.contains_key("e0"));
}

#[test]
fn reoffloading_existing_entry_refreshes_ttl_and_order() {
    let mut inner = Inner::default();
    assert!(inner.insert("old".into(), "old payload".into(), 2, usize::MAX));
    assert!(inner.insert("fresh".into(), "fresh payload".into(), 2, usize::MAX));
    let stale_created = Instant::now() - Duration::from_secs(60);
    inner.map.get_mut("old").unwrap().created = stale_created;

    assert!(inner.insert("old".into(), "old payload".into(), 2, usize::MAX));
    assert!(
        inner.map["old"].created > stale_created,
        "existing entry timestamp should refresh"
    );
    assert_eq!(inner.order.back().map(String::as_str), Some("old"));

    assert!(inner.insert("third".into(), "third payload".into(), 2, usize::MAX));
    assert!(inner.map.contains_key("old"));
    assert!(!inner.map.contains_key("fresh"));
}

#[test]
fn range_retrieval_lines_and_bytes() {
    let original = "line0\nline1\nline2\nline3\nline4";
    let hash = offload(original);
    assert_eq!(
        retrieve_range(&hash, 1, 3, RangeUnit::Lines).as_deref(),
        Some("line1\nline2")
    );
    assert_eq!(
        retrieve_range(&hash, 0, 5, RangeUnit::Bytes).as_deref(),
        Some("line0")
    );
    // Out-of-bounds end clamps.
    assert_eq!(
        retrieve_range(&hash, 4, 999, RangeUnit::Lines).as_deref(),
        Some("line4")
    );
}

#[test]
fn memory_store_is_isolated_from_global_cache() {
    let store = MemoryCcrStore::new(10, 10_000);
    let original = "isolated memory store payload delta ".repeat(20);
    let put = store.put(&original);

    assert!(put.retained());
    assert_eq!(store.get(put.token()).as_deref(), Some(original.as_str()));
    assert_eq!(retrieve(put.token()), None, "global cache must not see it");
}

#[test]
fn memory_store_rejects_oversized_originals() {
    let store = MemoryCcrStore::new(10, 16);
    let original = "oversized isolated payload echo ".repeat(20);
    let put = store.put(&original);

    assert!(!put.retained());
    assert_eq!(store.get(put.token()), None);
    assert_eq!(store.stats(), (0, 0));
}

#[test]
fn memory_store_rejects_malformed_tokens_before_disk_lookup() {
    let dir = std::env::temp_dir().join(format!(
        "tj-memory-ccr-{}",
        short_hash("memory-store-malformed-token")
    ));
    let _ = std::fs::remove_dir_all(&dir);
    let store = MemoryCcrStore::new(10, 10_000).with_disk_root(dir.clone());

    assert_eq!(store.get("../../state/config.toml"), None);
    assert_eq!(
        store.get_range("../../state/config.toml", 0, 1, RangeUnit::Lines),
        None
    );

    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn disk_tier_survives_memory_miss() {
    let dir = std::env::temp_dir().join(format!("tj-ccr-{}", short_hash("disk-test-seed")));
    let _ = std::fs::remove_dir_all(&dir);
    enable_disk_tier(dir.clone());
    let original = "disk tier unique payload charlie ".repeat(40);
    let hash = offload(&original);
    // Simulate memory eviction by clearing the in-memory map directly.
    {
        let mut inner = global().lock().unwrap_or_else(|p| p.into_inner());
        inner.map.remove(&hash);
    }
    assert_eq!(
        retrieve(&hash).as_deref(),
        Some(original.as_str()),
        "disk fallback"
    );
    // Disable the tier for other tests and clean up.
    *disk_root().write().unwrap() = None;
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn offload_with_hash_round_trips() {
    let original = "ccr precomputed-hash unique payload delta ".repeat(30);
    let hash = short_hash(&original);
    assert!(offload_checked_with_hash(&hash, &original));
    assert_eq!(retrieve(&hash).as_deref(), Some(original.as_str()));
    // Matches the hashing path exactly.
    assert_eq!(offload_checked(&original).0, hash);
}

/// Write a token-named file with a back-dated mtime.
fn write_aged(dir: &Path, name: &str, bytes: usize, age: Duration) -> PathBuf {
    let path = dir.join(name);
    std::fs::write(&path, "x".repeat(bytes)).unwrap();
    let file = std::fs::File::options().write(true).open(&path).unwrap();
    file.set_modified(SystemTime::now() - age).unwrap();
    path
}

fn token(fill: char) -> String {
    fill.to_string().repeat(32)
}

#[test]
fn gc_removes_expired_keeps_fresh() {
    let dir = tempfile::tempdir().unwrap();
    let old = write_aged(dir.path(), &token('a'), 10, Duration::from_secs(600));
    let fresh = write_aged(dir.path(), &token('b'), 20, Duration::ZERO);

    let stats = gc_disk_dir(dir.path(), Some(Duration::from_secs(60)), None).unwrap();
    assert_eq!(stats.removed, 1);
    assert_eq!(stats.freed_bytes, 10);
    assert_eq!(stats.kept, 1);
    assert_eq!(stats.kept_bytes, 20);
    assert!(!old.exists());
    assert!(fresh.exists());
}

#[test]
fn gc_evicts_oldest_over_budget() {
    let dir = tempfile::tempdir().unwrap();
    let oldest = write_aged(dir.path(), &token('a'), 100, Duration::from_secs(30));
    let middle = write_aged(dir.path(), &token('b'), 100, Duration::from_secs(20));
    let newest = write_aged(dir.path(), &token('c'), 100, Duration::from_secs(10));

    let stats = gc_disk_dir(dir.path(), None, Some(250)).unwrap();
    assert_eq!(stats.removed, 1, "one eviction brings 300 under 250");
    assert_eq!(stats.freed_bytes, 100);
    assert_eq!(stats.kept, 2);
    assert!(!oldest.exists(), "oldest-by-mtime goes first");
    assert!(middle.exists());
    assert!(newest.exists());
}

#[test]
fn gc_ignores_non_token_files_and_missing_dir() {
    let dir = tempfile::tempdir().unwrap();
    let stray = dir.path().join("README.txt");
    std::fs::write(&stray, "not a ccr entry").unwrap();

    let stats = gc_disk_dir(dir.path(), Some(Duration::ZERO), Some(0)).unwrap();
    assert_eq!(stats, GcStats::default());
    assert!(stray.exists(), "non-token files are never touched");

    let missing = dir.path().join("does-not-exist");
    assert_eq!(
        gc_disk_dir(&missing, None, None).unwrap(),
        GcStats::default()
    );
}

#[test]
fn gc_protects_named_entry_from_budget_eviction() {
    let dir = tempfile::tempdir().unwrap();
    let protected_name = token('a');
    let protected = write_aged(dir.path(), &protected_name, 100, Duration::from_secs(30));
    let newer = write_aged(dir.path(), &token('b'), 100, Duration::from_secs(10));

    let stats = gc_dir_protecting(dir.path(), None, Some(150), Some(&protected_name)).unwrap();
    assert!(
        protected.exists(),
        "protected entry survives even when oldest"
    );
    assert!(!newer.exists(), "eviction falls through to the next-oldest");
    assert_eq!(stats.removed, 1);
    assert_eq!(stats.kept, 1);
}

#[test]
fn disk_entry_ttl_uses_file_mtime() {
    let dir = std::env::temp_dir().join(format!("tj-ccr-ttl-{}", short_hash("disk-ttl")));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    let path = dir.join("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa");
    std::fs::write(&path, "disk ttl payload").unwrap();

    assert!(!disk_entry_expired(&path, None));
    assert!(!disk_entry_expired(&path, Some(Duration::from_secs(60))));
    assert!(disk_entry_expired(&path, Some(Duration::ZERO)));

    let _ = std::fs::remove_dir_all(&dir);
}
