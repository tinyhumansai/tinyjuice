use std::collections::HashMap;

use serde_json::{Map, Value};
use sha2::{Digest, Sha256};

use crate::conversation::{ConversationMessage, ToolCall, ToolResultMessage};

/// Options for deterministic old tool-result digesting.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ToolDigestOptions {
    pub max_full_result_chars: usize,
    pub keep_recent_tool_results: usize,
    pub max_argument_string_chars: usize,
}

impl Default for ToolDigestOptions {
    fn default() -> Self {
        Self {
            max_full_result_chars: 2_000,
            keep_recent_tool_results: 4,
            max_argument_string_chars: 160,
        }
    }
}

/// Redacted report entry for a replaced tool result.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ToolDigestEntry {
    pub tool_call_id: String,
    pub tool_name: Option<String>,
    pub hash: String,
    pub original_bytes: usize,
    pub replacement_bytes: usize,
    pub duplicate_of: Option<String>,
}

/// Redacted report for a digest pass.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ToolDigestReport {
    pub entries: Vec<ToolDigestEntry>,
}

/// Shrink only string leaves in a JSON value, preserving valid JSON structure.
pub fn shrink_json_string_leaves(value: &Value, max_string_chars: usize) -> Value {
    match value {
        Value::String(s) => Value::String(shrink_string(s, max_string_chars)),
        Value::Array(items) => Value::Array(
            items
                .iter()
                .map(|item| shrink_json_string_leaves(item, max_string_chars))
                .collect(),
        ),
        Value::Object(map) => Value::Object(
            map.iter()
                .map(|(key, value)| {
                    (
                        key.clone(),
                        shrink_json_string_leaves(value, max_string_chars),
                    )
                })
                .collect(),
        ),
        _ => value.clone(),
    }
}

/// Redact JSON values under sensitive-looking keys, then shrink string leaves.
pub fn redact_sensitive_json(value: &Value, max_string_chars: usize) -> Value {
    redact_sensitive_json_inner(value, max_string_chars, None)
}

/// Replace older duplicate/large tool results with deterministic digests.
pub fn digest_old_tool_results(
    messages: &[ConversationMessage],
    options: ToolDigestOptions,
) -> (Vec<ConversationMessage>, ToolDigestReport) {
    let calls = collect_calls(messages, options.max_argument_string_chars);
    let occurrences = collect_result_occurrences(messages);
    let total_results = occurrences.len();
    let mut newest_by_hash: HashMap<String, usize> = HashMap::new();
    for (serial, _, _, result) in &occurrences {
        newest_by_hash.insert(stable_hash(&result.content), *serial);
    }

    let mut report = ToolDigestReport::default();
    let mut serial_cursor = 0usize;
    let mut out = Vec::with_capacity(messages.len());

    for message in messages {
        match message {
            ConversationMessage::ToolResults(results) => {
                let mut rendered = Vec::with_capacity(results.len());
                for result in results {
                    let serial = serial_cursor;
                    serial_cursor += 1;
                    let protected_recent =
                        total_results.saturating_sub(serial) <= options.keep_recent_tool_results;
                    let hash = stable_hash(&result.content);
                    let duplicate_of = newest_by_hash
                        .get(&hash)
                        .copied()
                        .filter(|newest| *newest != serial)
                        .and_then(|newest| {
                            occurrences
                                .iter()
                                .find(|(candidate_serial, _, _, _)| *candidate_serial == newest)
                                .map(|(_, _, _, newest_result)| newest_result.tool_call_id.clone())
                        });

                    let should_digest = !protected_recent
                        && (duplicate_of.is_some()
                            || result.content.chars().count() > options.max_full_result_chars);

                    if should_digest {
                        let call = calls.get(&result.tool_call_id);
                        let replacement =
                            render_digest(result, call, &hash, duplicate_of.as_deref());
                        report.entries.push(ToolDigestEntry {
                            tool_call_id: result.tool_call_id.clone(),
                            tool_name: call.map(|call| call.name.clone()),
                            hash: hash.clone(),
                            original_bytes: result.content.len(),
                            replacement_bytes: replacement.len(),
                            duplicate_of,
                        });
                        rendered.push(ToolResultMessage::new(
                            result.tool_call_id.clone(),
                            replacement,
                        ));
                    } else {
                        rendered.push(result.clone());
                    }
                }
                out.push(ConversationMessage::ToolResults(rendered));
            }
            ConversationMessage::AssistantToolCalls {
                text,
                tool_calls,
                metadata,
            } => out.push(ConversationMessage::AssistantToolCalls {
                text: text.clone(),
                tool_calls: tool_calls
                    .iter()
                    .map(|call| sanitize_tool_call(call, options.max_argument_string_chars))
                    .collect(),
                metadata: metadata.clone(),
            }),
            ConversationMessage::Chat(_) => out.push(message.clone()),
        }
    }

    (out, report)
}

#[derive(Debug, Clone)]
struct SanitizedToolCall {
    name: String,
    arguments: String,
}

fn collect_calls(
    messages: &[ConversationMessage],
    max_argument_string_chars: usize,
) -> HashMap<String, SanitizedToolCall> {
    let mut out = HashMap::new();
    for message in messages {
        if let ConversationMessage::AssistantToolCalls { tool_calls, .. } = message {
            for call in tool_calls {
                let sanitized = sanitize_tool_call(call, max_argument_string_chars);
                out.insert(
                    call.id.clone(),
                    SanitizedToolCall {
                        name: sanitized.name,
                        arguments: sanitized.arguments,
                    },
                );
            }
        }
    }
    out
}

fn sanitize_tool_call(call: &ToolCall, max_argument_string_chars: usize) -> ToolCall {
    let arguments = match serde_json::from_str::<Value>(&call.arguments) {
        Ok(value) => redact_sensitive_json(&value, max_argument_string_chars).to_string(),
        Err(_) => redact_sensitive_text(&call.arguments, max_argument_string_chars),
    };
    ToolCall {
        id: call.id.clone(),
        name: call.name.clone(),
        arguments,
    }
}

fn collect_result_occurrences(
    messages: &[ConversationMessage],
) -> Vec<(usize, usize, usize, ToolResultMessage)> {
    let mut out = Vec::new();
    let mut serial = 0usize;
    for (message_idx, message) in messages.iter().enumerate() {
        if let ConversationMessage::ToolResults(results) = message {
            for (result_idx, result) in results.iter().enumerate() {
                out.push((serial, message_idx, result_idx, result.clone()));
                serial += 1;
            }
        }
    }
    out
}

fn render_digest(
    result: &ToolResultMessage,
    call: Option<&SanitizedToolCall>,
    hash: &str,
    duplicate_of: Option<&str>,
) -> String {
    let bytes = result.content.len();
    let chars = result.content.chars().count();
    let lines = result.content.lines().count();
    let tool_name = call
        .map(|call| call.name.as_str())
        .unwrap_or("unknown_tool");

    if let Some(newer_id) = duplicate_of {
        return format!(
            "[tokenjuice tool-result duplicate: call_id={} newer_call_id={} tool={} bytes={} chars={} lines={} hash={}]",
            result.tool_call_id, newer_id, tool_name, bytes, chars, lines, hash
        );
    }

    let detail = call
        .map(|call| render_tool_detail(tool_name, &call.arguments))
        .unwrap_or_default();
    if detail.is_empty() {
        format!(
            "[tokenjuice tool-result digest: call_id={} tool={} bytes={} chars={} lines={} hash={}]",
            result.tool_call_id, tool_name, bytes, chars, lines, hash
        )
    } else {
        format!(
            "[tokenjuice tool-result digest: call_id={} tool={} {} bytes={} chars={} lines={} hash={}]",
            result.tool_call_id, tool_name, detail, bytes, chars, lines, hash
        )
    }
}

fn render_tool_detail(tool_name: &str, arguments: &str) -> String {
    let Ok(value) = serde_json::from_str::<Value>(arguments) else {
        return String::new();
    };
    let Some(obj) = value.as_object() else {
        return String::new();
    };

    match tool_name {
        "shell" | "bash" | "exec" => first_string(obj, &["command", "cmd"])
            .map(|command| format!("command={}", compact_label(command)))
            .unwrap_or_default(),
        "read_file" | "file_read" | "fs_read" => first_string(obj, &["path", "file", "uri"])
            .map(|path| format!("path={}", compact_label(path)))
            .unwrap_or_default(),
        "search" | "grep" | "ripgrep" => first_string(obj, &["pattern", "query"])
            .map(|pattern| format!("pattern={}", compact_label(pattern)))
            .unwrap_or_default(),
        "web_fetch" | "fetch" | "browser_navigate" => first_string(obj, &["url", "uri"])
            .map(|url| format!("url={}", compact_label(url)))
            .unwrap_or_default(),
        _ => String::new(),
    }
}

fn first_string<'a>(obj: &'a Map<String, Value>, keys: &[&str]) -> Option<&'a str> {
    keys.iter()
        .find_map(|key| obj.get(*key).and_then(Value::as_str))
}

fn compact_label(value: &str) -> String {
    shrink_string(&redact_sensitive_text(value, 80), 80).replace('\n', " ")
}

fn redact_sensitive_json_inner(value: &Value, max_string_chars: usize, key: Option<&str>) -> Value {
    if key.is_some_and(is_sensitive_key) {
        return Value::String("[redacted]".to_owned());
    }

    match value {
        Value::String(s) => Value::String(shrink_string(
            &redact_sensitive_text(s, max_string_chars),
            max_string_chars,
        )),
        Value::Array(items) => Value::Array(
            items
                .iter()
                .map(|item| redact_sensitive_json_inner(item, max_string_chars, None))
                .collect(),
        ),
        Value::Object(map) => Value::Object(
            map.iter()
                .map(|(key, value)| {
                    (
                        key.clone(),
                        redact_sensitive_json_inner(value, max_string_chars, Some(key)),
                    )
                })
                .collect(),
        ),
        _ => value.clone(),
    }
}

fn is_sensitive_key(key: &str) -> bool {
    let normalized = key
        .chars()
        .filter(|ch| ch.is_ascii_alphanumeric())
        .collect::<String>()
        .to_ascii_lowercase();
    matches!(
        normalized.as_str(),
        "password"
            | "passwd"
            | "pwd"
            | "secret"
            | "token"
            | "apikey"
            | "accesstoken"
            | "refreshtoken"
            | "clientsecret"
            | "privatekey"
            | "accesskey"
            | "sessionid"
            | "sessioncookie"
            | "credential"
            | "credentials"
            | "jwt"
            | "authorization"
            | "cookie"
            | "setcookie"
    ) || normalized.ends_with("token")
        || normalized.ends_with("secret")
        || normalized.ends_with("password")
}

fn shrink_string(s: &str, max_chars: usize) -> String {
    if max_chars == 0 {
        return String::new();
    }
    let char_count = s.chars().count();
    if char_count <= max_chars {
        return s.to_owned();
    }
    let keep = max_chars.saturating_sub(1);
    let prefix = s.chars().take(keep).collect::<String>();
    format!("{prefix}...[truncated {} chars]", char_count - keep)
}

fn redact_sensitive_text(text: &str, max_chars: usize) -> String {
    let mut out = text.to_owned();
    for marker in [
        "Bearer ",
        "bearer ",
        "token=",
        "password=",
        "secret=",
        "api_key=",
        "access_key=",
        "private_key=",
        "client_secret=",
        "session_id=",
    ] {
        out = redact_after_marker(&out, marker);
    }
    shrink_string(&out, max_chars)
}

fn redact_after_marker(text: &str, marker: &str) -> String {
    let mut remaining = text;
    let mut out = String::with_capacity(text.len());
    while let Some(idx) = remaining.find(marker) {
        let (before, after_before) = remaining.split_at(idx);
        out.push_str(before);
        out.push_str(marker);
        out.push_str("[redacted]");
        let after = &after_before[marker.len()..];
        let end = after
            .find(|ch: char| ch.is_whitespace() || matches!(ch, '\'' | '"' | '&' | ';'))
            .unwrap_or(after.len());
        remaining = &after[end..];
    }
    out.push_str(remaining);
    out
}

fn stable_hash(content: &str) -> String {
    let digest = Sha256::digest(content.as_bytes());
    hex::encode(&digest[..8])
}

#[cfg(test)]
mod tests {
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
}
