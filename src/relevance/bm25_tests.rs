use super::*;

#[test]
fn exact_identifier_match_scores_above_generic_terms() {
    let corpus = Bm25Corpus::new([
        "general retry handling around worker state",
        "panic in sync_worker_v2 for account id 42",
        "worker completed normally after queue drain",
    ]);

    let scores = corpus.score_all("sync_worker_v2");

    assert!(scores[1].score > scores[0].score, "{scores:?}");
    assert!(scores[1].score > scores[2].score, "{scores:?}");
}

#[test]
fn tokenization_keeps_dotted_and_email_identifiers() {
    let tokens = tokenize("api.worker.example failed for ops@example.com");

    assert!(tokens.contains(&"api.worker.example".to_string()));
    assert!(tokens.contains(&"ops@example.com".to_string()));
}
