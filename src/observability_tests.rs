use super::*;

#[test]
fn breakdown_separates_static_prefix_from_compressible_context() {
    let breakdown = ContextBreakdown::new(vec![
        ContextBucket::system_prompt(100),
        ContextBucket::tool_definitions(200),
        ContextBucket::conversation(700),
        ContextBucket::memory(50),
    ])
    .with_context_max_tokens(2_000);

    assert_eq!(breakdown.estimated_total_tokens, 1_050);
    assert_eq!(breakdown.static_prefix_tokens(), 300);
    assert_eq!(breakdown.compression_candidate_tokens(), 750);
    assert_eq!(breakdown.effective_prompt_tokens(), 1_050);
    assert_eq!(breakdown.usage_ratio(), Some(0.525));
}

#[test]
fn measured_prompt_tokens_override_rough_estimate() {
    let breakdown = ContextBreakdown::new(vec![
        ContextBucket::system_prompt(100),
        ContextBucket::conversation(400),
    ])
    .with_measured_prompt_tokens(800);

    assert_eq!(breakdown.estimated_total_tokens, 500);
    assert_eq!(breakdown.effective_prompt_tokens(), 800);
}

#[test]
fn bucket_measured_tokens_override_bucket_estimate() {
    let breakdown = ContextBreakdown::new(vec![
        ContextBucket::system_prompt(100),
        ContextBucket::conversation(400).with_measured_tokens(250),
    ]);

    assert_eq!(breakdown.effective_prompt_tokens(), 350);
    assert_eq!(breakdown.compression_candidate_tokens(), 250);
}

#[test]
fn serialized_breakdown_contains_no_raw_prompt_fields() {
    let breakdown = ContextBreakdown::new(vec![
        ContextBucket::estimated(ContextBucketKind::Skills, 42)
            .with_item_count(3)
            .with_byte_count(1_024),
    ]);
    let json = serde_json::to_string(&breakdown).unwrap();

    assert!(json.contains("skills"));
    assert!(!json.contains("content"));
    assert!(!json.contains("promptText"));
    assert!(!json.contains("raw"));
}
