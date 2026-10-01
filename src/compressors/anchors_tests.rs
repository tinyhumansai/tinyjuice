use super::*;

#[test]
fn extracts_uuid() {
    let a = extract_anchors("find migration 550e8400-e29b-41d4-a716-446655440000 please");
    assert!(a.contains(&"550e8400-e29b-41d4-a716-446655440000".to_string()));
}

#[test]
fn extracts_quoted_phrase_verbatim() {
    let a = extract_anchors(r#"why did "CONFLICT_PACKAGES" appear"#);
    assert!(a.contains(&"conflict_packages".to_string()));
}

#[test]
fn extracts_long_numeric_id() {
    let a = extract_anchors("show market 12345 and order 42");
    assert!(a.contains(&"12345".to_string()), "{a:?}");
    // 2-digit `42` is too short to anchor.
    assert!(!a.contains(&"42".to_string()), "{a:?}");
}

#[test]
fn extracts_host_and_email() {
    let a = extract_anchors("errors from api.example.com for user bob@corp.io");
    assert!(a.contains(&"api.example.com".to_string()), "{a:?}");
    assert!(a.contains(&"bob@corp.io".to_string()), "{a:?}");
}

#[test]
fn ignores_plain_words_and_decimals() {
    let a = extract_anchors("show me the latency 3.14 average value");
    assert!(a.is_empty(), "{a:?}");
}

#[test]
fn empty_query_is_empty() {
    assert!(extract_anchors("").is_empty());
    assert!(extract_anchors("   ").is_empty());
}
