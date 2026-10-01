use super::*;

#[test]
fn tiny_sets_are_kept_whole() {
    let items = ["alpha one", "beta two", "gamma three"];
    assert_eq!(compute_optimal_k(&items, 1, 10), 3);
}

#[test]
fn redundant_items_yield_small_k() {
    let items: Vec<String> = (0..24)
        .map(|_| "connection timeout retrying request to upstream host".to_string())
        .collect();
    let refs: Vec<&str> = items.iter().map(String::as_str).collect();
    let k = compute_optimal_k(&refs, 2, 15);
    assert_eq!(k, 2, "identical items collapse to the floor");
}

#[test]
fn diverse_items_yield_k_near_n() {
    let vocab = [
        "database migration applied schema version",
        "user login rejected invalid credential token",
        "cache eviction pressure exceeded memory budget",
        "worker heartbeat missed scheduler requeued job",
        "tls handshake negotiated cipher suite protocol",
        "disk io latency spiked during checkpoint flush",
        "feature flag rollout gated cohort percentage",
        "payment webhook signature verified merchant order",
        "dns resolution fallback secondary nameserver query",
        "kernel oom killer reaped container process group",
        "metrics exporter scraped endpoint histogram bucket",
        "queue backlog drained consumer lag recovered",
        "certificate renewal scheduled expiry threshold reached",
        "replica election promoted follower after partition",
        "rate limiter shed excess burst traffic upstream",
        "search index rebuilt analyzer tokenizer updated",
        "session cookie rotated secure attribute enforced",
        "thread pool saturated queued tasks rejected",
        "object storage multipart upload part etag",
        "grpc deadline exceeded retry budget exhausted",
    ];
    let refs: Vec<&str> = vocab.to_vec();
    let k = compute_optimal_k(&refs, 2, 15);
    assert_eq!(k, 15, "fully diverse items should reach the ceiling");
}

#[test]
fn front_loaded_information_finds_early_knee() {
    // Four information-dense items up front, then a long tail of
    // near-boilerplate variations that add almost no new bigrams.
    let mut items: Vec<String> = vec![
        "FAIL src/auth/session.test.ts expected token refresh to rotate secrets".to_string(),
        "AssertionError expected 401 unauthorized received 200 ok at handler".to_string(),
        "stack trace at validateSession auth/session.ts line 118 column 9".to_string(),
        "caused by expired signing key rotation missed grace window".to_string(),
    ];
    for _ in 0..20 {
        items.push("retry attempt failed with timeout waiting for upstream".to_string());
    }
    let refs: Vec<&str> = items.iter().map(String::as_str).collect();
    let k = compute_optimal_k(&refs, 2, 20);
    assert!(
        (2..=8).contains(&k),
        "knee should land near the dense head, got {k}"
    );
}

#[test]
fn result_is_clamped_to_bounds() {
    let items: Vec<String> = (0..30).map(|_| "same line again".to_string()).collect();
    let refs: Vec<&str> = items.iter().map(String::as_str).collect();
    assert_eq!(compute_optimal_k(&refs, 5, 12), 5);
}
