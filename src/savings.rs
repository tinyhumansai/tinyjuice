//! Host-provided savings attribution hook.
//!
//! The engine can report token deltas, but model pricing and persistence are
//! host policy. OpenHuman installs a recorder that maps these events onto its
//! cost model and dashboard snapshot.

use std::sync::{Arc, OnceLock, RwLock};

use crate::types::{CompressorKind, ContentKind};

/// Accounting confidence for a savings event.
///
/// The class describes the numbers in the record, not the compressor itself:
/// reducer byte counts are measured, token counts from [`crate::tokens`] are
/// estimates, and call-collapse events are counted.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AccountingClass {
    Counted,
    Measured,
    Estimated,
}

impl AccountingClass {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Counted => "counted",
            Self::Measured => "measured",
            Self::Estimated => "estimated",
        }
    }
}

/// Provider-reported usage fields hosts may attach when real model usage is
/// available.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ModelUsage {
    pub input_tokens: u64,
    pub output_tokens: u64,
    pub cache_read_tokens: u64,
    pub cache_creation_tokens: u64,
}

/// Non-sensitive attribution record for a TokenJuice savings event.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SavingsRecord {
    pub accounting_class: AccountingClass,
    pub content_kind: ContentKind,
    pub compressor: CompressorKind,
    pub original_tokens: u64,
    pub compacted_tokens: u64,
    pub original_bytes: Option<u64>,
    pub compacted_bytes: Option<u64>,
    pub lossy: bool,
    pub ccr_token_present: bool,
    pub rule_id: Option<String>,
    pub skip_reason: Option<String>,
    pub measured_usage: Option<ModelUsage>,
}

impl SavingsRecord {
    pub fn estimated_compaction(
        content_kind: ContentKind,
        compressor: CompressorKind,
        original_tokens: u64,
        compacted_tokens: u64,
    ) -> Self {
        Self {
            accounting_class: AccountingClass::Estimated,
            content_kind,
            compressor,
            original_tokens,
            compacted_tokens,
            original_bytes: None,
            compacted_bytes: None,
            lossy: false,
            ccr_token_present: false,
            rule_id: None,
            skip_reason: None,
            measured_usage: None,
        }
    }

    pub fn with_bytes(mut self, original_bytes: u64, compacted_bytes: u64) -> Self {
        self.original_bytes = Some(original_bytes);
        self.compacted_bytes = Some(compacted_bytes);
        self
    }

    pub fn with_lossy(mut self, lossy: bool) -> Self {
        self.lossy = lossy;
        self
    }

    pub fn with_ccr_token_present(mut self, ccr_token_present: bool) -> Self {
        self.ccr_token_present = ccr_token_present;
        self
    }

    pub fn with_rule_id(mut self, rule_id: impl Into<String>) -> Self {
        self.rule_id = Some(rule_id.into());
        self
    }

    pub fn with_skip_reason(mut self, skip_reason: impl Into<String>) -> Self {
        self.skip_reason = Some(skip_reason.into());
        self
    }

    pub fn with_measured_usage(mut self, measured_usage: ModelUsage) -> Self {
        self.measured_usage = Some(measured_usage);
        self
    }
}

pub type SavingsRecorder = dyn Fn(ContentKind, CompressorKind, u64, u64) + Send + Sync + 'static;
pub type SavingsRecordRecorder = dyn Fn(SavingsRecord) + Send + Sync + 'static;

fn recorder_cell() -> &'static RwLock<Option<Arc<SavingsRecorder>>> {
    static RECORDER: OnceLock<RwLock<Option<Arc<SavingsRecorder>>>> = OnceLock::new();
    RECORDER.get_or_init(|| RwLock::new(None))
}

fn record_recorder_cell() -> &'static RwLock<Option<Arc<SavingsRecordRecorder>>> {
    static RECORDER: OnceLock<RwLock<Option<Arc<SavingsRecordRecorder>>>> = OnceLock::new();
    RECORDER.get_or_init(|| RwLock::new(None))
}

/// Install or clear the host savings recorder.
pub fn configure_recorder(recorder: Option<Arc<SavingsRecorder>>) {
    *recorder_cell().write().unwrap_or_else(|p| p.into_inner()) = recorder;
}

/// Install or clear the host recorder for rich, class-labeled savings records.
pub fn configure_record_recorder(recorder: Option<Arc<SavingsRecordRecorder>>) {
    *record_recorder_cell()
        .write()
        .unwrap_or_else(|p| p.into_inner()) = recorder;
}

/// Record one compaction event. No-op when no host recorder is installed.
pub fn record(
    content_kind: ContentKind,
    compressor: CompressorKind,
    original_tokens: u64,
    compacted_tokens: u64,
) {
    record_event(SavingsRecord::estimated_compaction(
        content_kind,
        compressor,
        original_tokens,
        compacted_tokens,
    ));
}

/// Record one rich savings event. No-op when no host recorder is installed.
pub fn record_event(record: SavingsRecord) {
    let record_recorder = record_recorder_cell()
        .read()
        .unwrap_or_else(|p| p.into_inner())
        .clone();
    if let Some(recorder) = record_recorder {
        recorder(record.clone());
    }

    let recorder = recorder_cell()
        .read()
        .unwrap_or_else(|p| p.into_inner())
        .clone();
    if let Some(recorder) = recorder {
        recorder(
            record.content_kind,
            record.compressor,
            record.original_tokens,
            record.compacted_tokens,
        );
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Mutex;

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
        assert!(json.contains("\"contentKind\":\"log\""));
        assert!(json.contains("\"compressor\":\"log\""));
        assert!(!json.contains("secret tool output"));
        assert!(!json.contains("prompt"));
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
}
