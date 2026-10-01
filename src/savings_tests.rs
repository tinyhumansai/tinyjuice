use std::sync::{Arc, Mutex};

use super::*;

#[test]
fn rich_record_serializes_only_metadata() {
    let record =
        SavingsRecord::estimated_compaction(ContentKind::Log, CompressorKind::Log, 120, 40)
            .with_bytes(480, 160)
            .with_lossy(true)
            .with_ccr_token_present(true)
            .with_rule_id("cargo-test")
            .with_skip_reason("no_raw_content_here")
            .with_measured_usage(ModelUsage {
                input_tokens: 10,
                output_tokens: 20,
                cache_read_tokens: 5,
                cache_creation_tokens: 7,
            });

    let json = serde_json::to_string(&record).expect("record serializes");
    assert!(json.contains("\"accountingClass\":\"estimated\""));
    assert!(json.contains("\"source\":\"live\""));
    assert!(json.contains("\"contentKind\":\"log\""));
    assert!(json.contains("\"compressor\":\"log\""));
    assert!(!json.contains("secret tool output"));
    assert!(!json.contains("prompt"));
}

#[test]
fn fixture_benchmark_records_are_labeled_separately() {
    let record = SavingsRecord::estimated_compaction(
        ContentKind::Json,
        CompressorKind::SmartCrusher,
        80,
        30,
    )
    .with_source(SavingsRecordSource::FixtureBenchmark)
    .with_bytes(320, 120);

    let json = serde_json::to_string(&record).expect("record serializes");
    assert!(json.contains("\"source\":\"fixture_benchmark\""));
    assert!(!json.contains("fixture raw content"));
}

#[test]
fn record_event_invokes_rich_and_legacy_recorders() {
    let rich_records = Arc::new(Mutex::new(Vec::new()));
    let legacy_records = Arc::new(Mutex::new(Vec::new()));

    let rich_records_clone = Arc::clone(&rich_records);
    configure_record_recorder(Some(Arc::new(move |record| {
        rich_records_clone
            .lock()
            .expect("rich records lock")
            .push(record);
    })));

    let legacy_records_clone = Arc::clone(&legacy_records);
    configure_recorder(Some(Arc::new(
        move |kind, compressor, original, compacted| {
            legacy_records_clone
                .lock()
                .expect("legacy records lock")
                .push((kind, compressor, original, compacted));
        },
    )));

    record_event(
        SavingsRecord::estimated_compaction(
            ContentKind::Json,
            CompressorKind::SmartCrusher,
            100,
            25,
        )
        .with_bytes(400, 100)
        .with_ccr_token_present(true),
    );

    configure_record_recorder(None);
    configure_recorder(None);

    let rich = rich_records.lock().expect("rich records lock");
    assert_eq!(rich.len(), 1);
    assert_eq!(rich[0].accounting_class, AccountingClass::Estimated);
    assert_eq!(rich[0].original_bytes, Some(400));
    assert!(rich[0].ccr_token_present);

    let legacy = legacy_records.lock().expect("legacy records lock");
    assert_eq!(
        legacy.as_slice(),
        &[(ContentKind::Json, CompressorKind::SmartCrusher, 100, 25)]
    );
}

#[test]
fn no_recorder_is_a_noop() {
    configure_recorder(None);
    record(ContentKind::Json, CompressorKind::SmartCrusher, 100, 25);
}

#[test]
fn configured_recorder_receives_compaction_event() {
    let events = Arc::new(Mutex::new(Vec::new()));
    let captured = Arc::clone(&events);
    configure_recorder(Some(Arc::new(
        move |kind, compressor, original, compacted| {
            captured
                .lock()
                .unwrap()
                .push((kind, compressor, original, compacted));
        },
    )));

    record(ContentKind::Log, CompressorKind::Log, 400, 120);

    assert!(
        events
            .lock()
            .unwrap()
            .contains(&(ContentKind::Log, CompressorKind::Log, 400, 120))
    );
    configure_recorder(None);
}
