use super::*;
use crate::rules::load_builtin_rules;

#[test]
fn reduce_json_accepts_direct_payload() {
    let rules = load_builtin_rules();
    let response = reduce_json_str(
        r#"{
                "toolName": "bash",
                "argv": ["git", "status"],
                "stdout": "On branch main\n\nChanges not staged for commit:\n\tmodified:   src/lib.rs\n"
            }"#,
        &rules,
    )
    .expect("direct payload");

    assert_eq!(response.inline_text, "Changes not staged:\nM: src/lib.rs");
    assert_eq!(
        response.classification.matched_reducer.as_deref(),
        Some("git/status")
    );
    assert!(response.metadata.is_none());
    assert!(response.trace.is_none());
}

#[test]
fn reduce_json_accepts_envelope_payload_with_options() {
    let rules = load_builtin_rules();
    let response = reduce_json_str(
        r#"{
                "input": {
                    "toolName": "bash",
                    "argv": ["some_tool"],
                    "stdout": "alpha\nbeta\ngamma\ndelta\nepsilon\nzeta"
                },
                "options": {
                    "maxInlineChars": 24,
                    "trace": true,
                    "recordStats": true
                }
            }"#,
        &rules,
    )
    .expect("envelope payload");

    assert!(response.inline_text.len() <= 24, "{response:#?}");
    assert_eq!(
        response.metadata,
        Some(ReduceJsonMetadata {
            no_omit_requested: false,
            store_requested: false,
            record_stats_requested: true,
            ccr: None,
        })
    );
    let trace = response.trace.expect("trace");
    assert_eq!(trace.tool_name, "bash");
    assert_eq!(trace.argv0.as_deref(), Some("some_tool"));
    assert_eq!(trace.max_inline_chars, Some(24));
    assert_eq!(trace.matched_reducer.as_deref(), Some("generic/fallback"));
}

#[test]
fn reduce_json_store_option_returns_retrievable_ccr_ref() {
    let rules = load_builtin_rules();
    let original = (0..80)
        .map(|i| format!("line {i}: verbose report details"))
        .collect::<Vec<_>>()
        .join("\n");
    let response = reduce_json_request(
        ReduceJsonRequest::Envelope(ReduceJsonEnvelope {
            input: ToolExecutionInput {
                tool_name: "bash".to_owned(),
                argv: Some(vec!["custom-report".to_owned()]),
                stdout: Some(original.clone()),
                ..ToolExecutionInput::default()
            },
            options: ReduceOptions {
                store: Some(true),
                max_inline_chars: Some(80),
                ..ReduceOptions::default()
            },
        }),
        &rules,
    )
    .expect("stored payload");

    let metadata = response.metadata.expect("store metadata");
    assert!(metadata.store_requested);
    let ccr = metadata.ccr.expect("ccr ref");
    assert_eq!(ccr.original_chars, original.chars().count());
    assert_eq!(
        crate::cache::retrieve(&ccr.token).as_deref(),
        Some(original.as_str())
    );
    assert!(!response.inline_text.contains(&ccr.token));
}

#[test]
fn reduce_json_raw_mode_returns_raw_text() {
    let rules = load_builtin_rules();
    let response = reduce_json_str(
        r#"{
                "input": {
                    "toolName": "bash",
                    "argv": ["git", "status"],
                    "stdout": "On branch main\n"
                },
                "options": { "raw": true }
            }"#,
        &rules,
    )
    .expect("raw payload");

    assert_eq!(response.inline_text, "On branch main\n");
}

#[test]
fn reduce_json_invalid_classifier_falls_back_to_matching() {
    let rules = load_builtin_rules();
    let response = reduce_json_str(
        r#"{
                "input": {
                    "toolName": "bash",
                    "argv": ["git", "status"],
                    "stdout": "On branch main\n"
                },
                "options": { "classifier": "missing/rule" }
            }"#,
        &rules,
    )
    .expect("invalid classifier fallback");

    assert_eq!(
        response.classification.matched_reducer.as_deref(),
        Some("git/status")
    );
}

#[test]
fn reduce_json_rejects_malformed_payload_without_raw_echo() {
    let rules = load_builtin_rules();
    let error = reduce_json_str(r#"{"toolName":"bash","stdout":"secret""#, &rules)
        .expect_err("invalid json");

    assert_eq!(error.code, "invalid_json");
    assert!(!error.message.contains("secret"));
}

#[test]
fn reduce_json_rejects_nul_bytes() {
    let rules = load_builtin_rules();
    let error = reduce_json_str("{\"toolName\":\"bash\"}\0", &rules).expect_err("nul byte");

    assert_eq!(error.code, "nul_byte");
}
