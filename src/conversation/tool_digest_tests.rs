use super::*;
use crate::conversation::ToolCall;

#[test]
fn shrink_json_string_leaves_preserves_valid_json_shape() {
    let value = serde_json::json!({
        "path": "src/lib.rs",
        "body": "abcdefghijklmnopqrstuvwxyz",
        "nested": ["short", {"long": "0123456789abcdef"}]
    });
    let shrunk = shrink_json_string_leaves(&value, 8);
    assert_eq!(shrunk["path"], "src/lib...[truncated 3 chars]");
    assert!(
        shrunk["nested"][1]["long"]
            .as_str()
            .unwrap()
            .contains("[truncated")
    );
    serde_json::to_string(&shrunk).expect("valid json");
}

#[test]
fn duplicate_read_file_outputs_keep_newest_full_copy() {
    let body = "same file body\n".repeat(100);
    let messages = vec![
        ConversationMessage::assistant_tool_calls(vec![ToolCall::new(
            "old",
            "read_file",
            r#"{"path":"src/lib.rs"}"#,
        )]),
        ConversationMessage::tool_results(vec![ToolResultMessage::new("old", body.clone())]),
        ConversationMessage::assistant_tool_calls(vec![ToolCall::new(
            "new",
            "read_file",
            r#"{"path":"src/lib.rs"}"#,
        )]),
        ConversationMessage::tool_results(vec![ToolResultMessage::new("new", body.clone())]),
    ];

    let (out, report) = digest_old_tool_results(
        &messages,
        ToolDigestOptions {
            max_full_result_chars: 10,
            keep_recent_tool_results: 1,
            max_argument_string_chars: 80,
        },
    );

    let ConversationMessage::ToolResults(old_results) = &out[1] else {
        panic!("expected tool results");
    };
    let ConversationMessage::ToolResults(new_results) = &out[3] else {
        panic!("expected tool results");
    };
    assert!(old_results[0].content.contains("duplicate"));
    assert_eq!(new_results[0].content, body);
    assert_eq!(report.entries[0].duplicate_of.as_deref(), Some("new"));
}

#[test]
fn old_terminal_output_becomes_digest_with_command_exit_and_lines() {
    let messages = vec![
        ConversationMessage::assistant_tool_calls(vec![ToolCall::new(
            "cmd",
            "shell",
            r#"{"command":"cargo test"}"#,
        )]),
        ConversationMessage::tool_results(vec![ToolResultMessage::new(
            "cmd",
            "line\n".repeat(100),
        )]),
        ConversationMessage::user("next"),
    ];

    let (out, report) = digest_old_tool_results(
        &messages,
        ToolDigestOptions {
            max_full_result_chars: 10,
            keep_recent_tool_results: 0,
            max_argument_string_chars: 80,
        },
    );

    let ConversationMessage::ToolResults(results) = &out[1] else {
        panic!("expected tool results");
    };
    assert!(results[0].content.contains("tool=shell"));
    assert!(results[0].content.contains("command=cargo test"));
    assert!(results[0].content.contains("lines=100"));
    assert_eq!(report.entries.len(), 1);
}

#[test]
fn sensitive_values_are_redacted_before_digesting() {
    let messages = vec![
        ConversationMessage::assistant_tool_calls(vec![ToolCall::new(
            "cmd",
            "shell",
            r#"{"command":"curl -H 'Authorization: Bearer SECRET123' https://api.example/?client_secret=SECRET789","access_token":"SECRET123","nested":{"refreshSecret":"SECRET456","private_key":"SECRETKEY","session_id":"SID999"}}"#,
        )]),
        ConversationMessage::tool_results(vec![ToolResultMessage::new(
            "cmd",
            "line\n".repeat(100),
        )]),
    ];

    let (out, _) = digest_old_tool_results(
        &messages,
        ToolDigestOptions {
            max_full_result_chars: 10,
            keep_recent_tool_results: 0,
            max_argument_string_chars: 120,
        },
    );

    let ConversationMessage::AssistantToolCalls { tool_calls, .. } = &out[0] else {
        panic!("expected calls");
    };
    let ConversationMessage::ToolResults(results) = &out[1] else {
        panic!("expected results");
    };
    assert!(!tool_calls[0].arguments.contains("SECRET123"));
    assert!(!tool_calls[0].arguments.contains("SECRET456"));
    assert!(!tool_calls[0].arguments.contains("SECRETKEY"));
    assert!(!tool_calls[0].arguments.contains("SID999"));
    assert!(!tool_calls[0].arguments.contains("SECRET789"));
    assert!(!results[0].content.contains("SECRET123"));
    assert!(!results[0].content.contains("SECRET789"));
    assert!(tool_calls[0].arguments.contains("[redacted]"));
}
