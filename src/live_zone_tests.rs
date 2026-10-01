use super::*;

#[test]
fn splices_mutable_blocks_without_rewriting_frozen_prefix() {
    let input = r#"{"system":"stable","messages":[{"role":"user","content":"hello world"}]}"#;
    let frozen_end = input.find("\"messages\"").expect("messages key");
    let content_start = input.find("hello world").expect("content");
    let content_end = content_start + "hello world".len();
    let zone = LiveZone::new(
        "anthropic",
        "test",
        ByteRange::new(0, frozen_end),
        vec![ByteRange::new(frozen_end, input.len())],
    );

    let rewrite = splice_live_zone_replacements(
        input,
        &zone,
        &[LiveZoneReplacement::new(
            ByteRange::new(content_start, content_end),
            "hi",
        )],
    )
    .expect("splice succeeds");

    assert_eq!(&rewrite.text[..frozen_end], &input[..frozen_end]);
    assert!(rewrite.frozen_prefix_preserved);
    assert_eq!(rewrite.replaced_blocks, 1);
    assert!(rewrite.text.contains(r#""content":"hi""#));
}

#[test]
fn rejects_replacements_that_touch_frozen_prefix() {
    let input = "frozen mutable";
    let zone = LiveZone::new(
        "provider",
        "test",
        ByteRange::new(0, 6),
        vec![ByteRange::new(7, input.len())],
    );

    let error = splice_live_zone_replacements(
        input,
        &zone,
        &[LiveZoneReplacement::new(ByteRange::new(1, 4), "xxx")],
    )
    .expect_err("frozen replacement rejected");

    assert_eq!(error, LiveZoneError::FrozenRangeModified);
}

#[test]
fn byte_splicing_does_not_require_valid_json() {
    let input = r#"{"messages":[{"content":"alpha"}"#;
    let start = input.find("alpha").expect("alpha");
    let end = start + "alpha".len();
    let zone = LiveZone::new(
        "provider",
        "malformed_json_fixture",
        ByteRange::new(0, 0),
        vec![ByteRange::new(0, input.len())],
    );

    let rewrite = splice_live_zone_replacements(
        input,
        &zone,
        &[LiveZoneReplacement::new(ByteRange::new(start, end), "beta")],
    )
    .expect("byte splice succeeds");

    assert_eq!(rewrite.text, r#"{"messages":[{"content":"beta"}"#);
}

#[test]
fn no_mutable_blocks_and_no_replacements_is_noop() {
    let input = "fully frozen prompt";
    let zone = LiveZone::new(
        "provider",
        "no_marker",
        ByteRange::new(0, input.len()),
        Vec::new(),
    );

    let rewrite = splice_live_zone_replacements(input, &zone, &[]).expect("noop succeeds");

    assert_eq!(rewrite.text, input);
    assert!(rewrite.frozen_prefix_preserved);
    assert_eq!(rewrite.replaced_blocks, 0);
}

#[test]
fn volatile_detector_reports_redacted_findings_only() {
    let uuid = "550e8400-e29b-41d4-a716-446655440000";
    let jwt = "eyJhbGciOiJIUzI1NiJ9.eyJzdWIiOiIxMjM0NTY3ODkwIn0.K7gNU3sdo-OL0wNhqoVWhr3g6s1xYv72ol_7IrW0hFQ";
    let hash = "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef";
    let input = format!("id={uuid} ts=2026-07-06T12:34:56Z jwt={jwt} hash={hash}");

    let findings = detect_volatile_cache_values(&input, ByteRange::new(0, input.len()));

    assert_eq!(findings.len(), 4);
    assert!(findings.iter().any(|f| f.kind == VolatileValueKind::Uuid));
    assert!(
        findings
            .iter()
            .any(|f| f.kind == VolatileValueKind::IsoTimestamp)
    );
    assert!(findings.iter().any(|f| f.kind == VolatileValueKind::Jwt));
    assert!(
        findings
            .iter()
            .any(|f| f.kind == VolatileValueKind::Sha256Hex)
    );
    for finding in findings {
        assert!(finding.redacted_sample.starts_with('['));
        assert!(!finding.redacted_sample.contains(uuid));
        assert!(!finding.redacted_sample.contains(jwt));
        assert!(!finding.redacted_sample.contains(hash));
    }
}
