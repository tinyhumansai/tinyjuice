//! Host-provided savings attribution hook.
//!
//! The engine can report token deltas, but model pricing and persistence are
//! host policy. OpenHuman installs a recorder that maps these events onto its
//! cost model and dashboard snapshot.

use std::sync::{Arc, OnceLock, RwLock};

use crate::types::{CompressorKind, ContentKind};

pub type SavingsRecorder = dyn Fn(ContentKind, CompressorKind, u64, u64) + Send + Sync + 'static;

fn recorder_cell() -> &'static RwLock<Option<Arc<SavingsRecorder>>> {
    static RECORDER: OnceLock<RwLock<Option<Arc<SavingsRecorder>>>> = OnceLock::new();
    RECORDER.get_or_init(|| RwLock::new(None))
}

/// Install or clear the host savings recorder.
pub fn configure_recorder(recorder: Option<Arc<SavingsRecorder>>) {
    *recorder_cell().write().unwrap_or_else(|p| p.into_inner()) = recorder;
}

/// Record one compaction event. No-op when no host recorder is installed.
pub fn record(
    content_kind: ContentKind,
    compressor: CompressorKind,
    original_tokens: u64,
    compacted_tokens: u64,
) {
    let recorder = recorder_cell()
        .read()
        .unwrap_or_else(|p| p.into_inner())
        .clone();
    if let Some(recorder) = recorder {
        recorder(content_kind, compressor, original_tokens, compacted_tokens);
    }
}
