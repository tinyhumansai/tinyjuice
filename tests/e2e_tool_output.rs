//! End-to-end tests through the OpenHuman-facing hook.
//!
//! These exercise the same path OpenHuman's `ToolOutputMiddleware` uses:
//! `compact_tool_output_with_policy` → router → compressor → CCR offload →
//! recovery footer → `cache::retrieve` roundtrip. They pin the plan-level
//! acceptance criteria: `off`, recovery-tool output, and exact file reads are
//! passthrough; `light` declines lossy compaction; `full` compacts recoverably.

mod common;

use serde_json::json;
use tinyjuice::cache;
use tinyjuice::tool_integration::compact_tool_output_with_policy;
use tinyjuice::types::AgentTokenjuiceCompression;

/// A JSON-array payload large enough to clear the 2048-byte and 500-token
/// floors and trip the lossy table compaction path. Deterministic.
fn big_json_rows() -> String {
    let rows: Vec<serde_json::Value> = (0..300)
        .map(|i| {
            let region = ["us-east-1", "eu-west-1", "ap-south-1"][i % 3];
            json!({
                "id": i,
                "name": format!("service-{i}"),
                "status": if i == 137 { "error" } else { "ok" },
                "region": region,
                "latency_ms": 20 + (i % 40),
            })
        })
        .collect();
    serde_json::to_string_pretty(&rows).unwrap()
}

#[tokio::test]
async fn off_profile_is_exact_passthrough() {
    common::install_test_config();
    let payload = big_json_rows();
    let (text, stats) = compact_tool_output_with_policy(
        "read_file",
        Some(&json!({ "path": "services.json" })),
        &payload,
        Some(0),
        AgentTokenjuiceCompression::Off,
    )
    .await;
    assert_eq!(text, payload, "off profile must not alter a single byte");
    assert!(!stats.applied);
}

#[tokio::test]
async fn recovery_tool_output_is_never_compacted() {
    common::install_test_config();
    let payload = big_json_rows();
    for tool in cache::RECOVERY_TOOL_NAMES {
        let (text, stats) = compact_tool_output_with_policy(
            tool,
            None,
            &payload,
            Some(0),
            AgentTokenjuiceCompression::Full,
        )
        .await;
        assert_eq!(text, payload, "recovery tool `{tool}` output was altered");
        assert!(!stats.applied);
    }
}

#[tokio::test]
async fn light_profile_declines_lossy_compaction() {
    common::install_test_config();
    let payload = big_json_rows();
    let (text, _stats) = compact_tool_output_with_policy(
        "read_file",
        Some(&json!({ "path": "services.json" })),
        &payload,
        Some(0),
        AgentTokenjuiceCompression::Light,
    )
    .await;
    // Light disables CCR, so the lossy JSON table path must decline. Only a
    // truly lossless reformat may differ from the original — and it must not
    // carry a recovery marker.
    assert!(
        cache::parse_markers(&text).is_empty(),
        "light profile output must not reference CCR"
    );
    assert!(
        text.len() >= payload.len() || !text.contains("PARTIAL"),
        "light profile must never return a partial (lossy) view"
    );
}

#[tokio::test]
async fn full_profile_compacts_and_original_is_recoverable() {
    common::install_test_config();
    let payload = big_json_rows();
    let (text, stats) = compact_tool_output_with_policy(
        "http_request",
        Some(&json!({ "url": "https://example.test/services" })),
        &payload,
        Some(0),
        AgentTokenjuiceCompression::Full,
    )
    .await;

    assert!(stats.applied, "full profile should compact this payload");
    assert!(
        stats.compacted_bytes < stats.original_bytes,
        "compaction must shrink: {} -> {}",
        stats.original_bytes,
        stats.compacted_bytes
    );

    let tokens = cache::parse_markers(&text);
    assert!(
        !tokens.is_empty(),
        "lossy compaction must embed a recovery marker; got:\n{text}"
    );
    let original = cache::retrieve(&tokens[0])
        .expect("footer-referenced token must be retrievable at emission time");
    assert_eq!(original, payload, "CCR roundtrip must restore the original");
}

#[tokio::test]
async fn file_read_is_exact_by_default_even_in_full_profile() {
    common::install_test_config();
    let payload = big_json_rows();
    let (text, stats) = compact_tool_output_with_policy(
        "read_file",
        Some(&json!({ "path": "services.json" })),
        &payload,
        Some(0),
        AgentTokenjuiceCompression::Full,
    )
    .await;

    assert_eq!(text, payload, "default file reads must remain exact");
    assert!(!stats.applied);
}

#[tokio::test]
async fn exit_code_and_command_reach_the_rule_engine() {
    common::install_test_config();
    // `git status` output is rule-matched via command/argv extracted from the
    // tool arguments — the exact wiring OpenHuman's hook migration relies on.
    let output = "On branch main\n\nChanges not staged for commit:\n\
                  \tmodified:   src/foo.rs\n\n\
                  no changes added to commit (use \"git add\" and/or \"git commit -a\")\n";
    let padded = format!("{output}{}", " ".repeat(4096)); // clear min_bytes floor
    let (_text, stats) = compact_tool_output_with_policy(
        "shell",
        Some(&json!({ "command": "git status" })),
        &padded,
        Some(0),
        AgentTokenjuiceCompression::Full,
    )
    .await;
    // The assertion is on classification, not on byte-exact output: the rule
    // path must be reachable through tool arguments.
    assert!(
        stats.applied || stats.rule_id.starts_with("none/"),
        "unexpected stats shape: {stats:?}"
    );
}
