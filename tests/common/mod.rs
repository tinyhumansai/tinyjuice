//! Shared helpers for integration tests.
//!
//! Global runtime state note: `tinyjuice::configure` and the CCR cache are
//! process-global, and Rust runs all tests in one binary across threads.
//! Every test must install the SAME options via [`install_test_config`]
//! (idempotent under parallelism) instead of test-specific option sets.

// Each tests/*.rs binary compiles this module; not every binary uses every
// helper, so per-binary dead-code warnings are expected noise.
#![allow(dead_code)]

use serde::Deserialize;
use std::path::PathBuf;
use tinyjuice::types::ToolExecutionInput;

/// One `tests/fixtures/*.fixture.json` file: a recorded tool execution and
/// the exact reduced text the rule engine must produce for it.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReduceFixture {
    pub description: String,
    pub input: ToolExecutionInput,
    pub expected_output: String,
}

/// Load every `*.fixture.json` under `tests/fixtures/`, sorted by filename so
/// failures are reported in a stable order.
pub fn load_reduce_fixtures() -> Vec<(PathBuf, ReduceFixture)> {
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures");
    let mut paths: Vec<PathBuf> = std::fs::read_dir(&dir)
        .unwrap_or_else(|e| panic!("cannot read {}: {e}", dir.display()))
        .filter_map(|entry| entry.ok().map(|e| e.path()))
        .filter(|p| {
            p.file_name()
                .and_then(|n| n.to_str())
                .is_some_and(|n| n.ends_with(".fixture.json"))
        })
        .collect();
    paths.sort();
    assert!(
        !paths.is_empty(),
        "no fixtures found in {} — the suite would silently test nothing",
        dir.display()
    );
    paths
        .into_iter()
        .map(|p| {
            let raw = std::fs::read_to_string(&p)
                .unwrap_or_else(|e| panic!("cannot read {}: {e}", p.display()));
            let fixture: ReduceFixture = serde_json::from_str(&raw)
                .unwrap_or_else(|e| panic!("invalid fixture {}: {e}", p.display()));
            (p, fixture)
        })
        .collect()
}

/// Install the default runtime options. Idempotent and identical across all
/// tests, so concurrent installs cannot race each other into different states.
#[allow(dead_code)] // each tests/*.rs binary compiles this module; not all use every helper
pub fn install_test_config() {
    tinyjuice::configure(tinyjuice::types::CompressOptions::default());
}
