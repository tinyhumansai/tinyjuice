use std::sync::{Arc, Mutex};

use tinyjuice::rules::load_builtin_rules;
use tinyjuice::savings::{SavingsRecord, configure_record_recorder};
use tinyjuice::{
    CompressorKind, ContentKind, ReduceJsonEnvelope, ReduceJsonRequest, ReduceOptions,
    ToolExecutionInput, reduce_json_request,
};

#[test]
fn reduce_json_record_stats_emits_metadata_only_savings_record() {
    let records = Arc::new(Mutex::new(Vec::<SavingsRecord>::new()));
    let captured = Arc::clone(&records);
    configure_record_recorder(Some(Arc::new(move |record| {
        captured.lock().expect("records lock").push(record);
    })));

    let secret_payload = (0..100)
        .map(|i| format!("SECRET_DO_NOT_RECORD line {i} with verbose diagnostic text"))
        .collect::<Vec<_>>()
        .join("\n");
    let rules = load_builtin_rules();
    let response = reduce_json_request(
        ReduceJsonRequest::Envelope(ReduceJsonEnvelope {
            input: ToolExecutionInput {
                tool_name: "bash".to_owned(),
                argv: Some(vec!["custom-report".to_owned()]),
                stdout: Some(secret_payload),
                ..ToolExecutionInput::default()
            },
            options: ReduceOptions {
                record_stats: Some(true),
                max_inline_chars: Some(120),
                ..ReduceOptions::default()
            },
        }),
        &rules,
    )
    .expect("reduce json");

    configure_record_recorder(None);

    let records = records.lock().expect("records lock");
    assert_eq!(records.len(), 1);
    let record = &records[0];
    assert_eq!(record.content_kind, ContentKind::Log);
    assert_eq!(record.compressor, CompressorKind::Generic);
    assert_eq!(record.rule_id.as_deref(), Some("generic/fallback"));
    assert_eq!(record.original_bytes, Some(response.stats.raw_chars as u64));
    assert_eq!(
        record.compacted_bytes,
        Some(response.stats.reduced_chars as u64)
    );
    assert!(!record.ccr_token_present);

    let serialized = serde_json::to_string(record).expect("record serializes");
    assert!(!serialized.contains("SECRET_DO_NOT_RECORD"));
    assert!(!serialized.contains("custom-report"));
}
