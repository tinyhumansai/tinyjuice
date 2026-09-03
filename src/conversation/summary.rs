use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

use crate::conversation::{ChatMessage, ConversationMessage};

pub const SUMMARY_METADATA_KIND: &str = "tinyjuice.compaction_summary.v1";
pub const SUMMARY_END_MARKER: &str = "<END_TINYJUICE_COMPACTION_SUMMARY>";
const SUMMARY_PREFIX: &str = "Reference-only historical compaction summary.";

/// Structured handoff summary. Fields are historical context only; the latest
/// user turn remains authoritative after this summary is inserted.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StructuredSummary {
    #[serde(default)]
    pub historical_task_snapshot: Vec<String>,
    #[serde(default)]
    pub goal: Vec<String>,
    #[serde(default)]
    pub constraints_and_preferences: Vec<String>,
    #[serde(default)]
    pub completed_actions: Vec<String>,
    #[serde(default)]
    pub active_state: Vec<String>,
    #[serde(default)]
    pub historical_in_progress_state: Vec<String>,
    #[serde(default)]
    pub blockers: Vec<String>,
    #[serde(default)]
    pub key_decisions: Vec<String>,
    #[serde(default)]
    pub resolved_questions: Vec<String>,
    #[serde(default)]
    pub historical_pending_asks: Vec<String>,
    #[serde(default)]
    pub relevant_files: Vec<String>,
    #[serde(default)]
    pub historical_remaining_work: Vec<String>,
    #[serde(default)]
    pub critical_context: Vec<String>,
    #[serde(default)]
    pub locally_generated: bool,
    #[serde(default)]
    pub lower_confidence: bool,
}

/// Summary request handed to an adapter-owned LLM summarizer.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SummaryRequest {
    pub messages: Vec<ConversationMessage>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub previous_summary: Option<StructuredSummary>,
    #[serde(default = "default_summary_budget")]
    pub max_output_bytes: usize,
}

impl SummaryRequest {
    pub fn new(messages: Vec<ConversationMessage>) -> Self {
        Self {
            messages,
            previous_summary: None,
            max_output_bytes: default_summary_budget(),
        }
    }
}

/// Adapter boundary for model-backed summarization.
pub trait SummaryProvider {
    fn summarize(&self, request: SummaryRequest) -> Result<StructuredSummary, SummaryError>;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum SummaryErrorKind {
    NoProvider,
    ModelUnavailable,
    Timeout,
    MalformedOutput,
    Auth,
    Permission,
    Network,
    IneffectiveCompression,
    Other,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SummaryError {
    pub kind: SummaryErrorKind,
    pub message: String,
}

impl SummaryError {
    pub fn new(kind: SummaryErrorKind, message: impl Into<String>) -> Self {
        Self {
            kind,
            message: message.into(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SummaryFailurePolicy {
    pub deterministic_fallback_enabled: bool,
    pub allow_lossy_fallback: bool,
}

impl Default for SummaryFailurePolicy {
    fn default() -> Self {
        Self {
            deterministic_fallback_enabled: true,
            allow_lossy_fallback: false,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SummaryFailureAction {
    PreserveMessages,
    UseDeterministicFallback,
    RetryLater,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SummaryFailureDecision {
    pub action: SummaryFailureAction,
    pub messages_dropped: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DeterministicSummaryOptions {
    pub max_output_bytes: usize,
    pub max_items_per_section: usize,
}

impl Default for DeterministicSummaryOptions {
    fn default() -> Self {
        Self {
            max_output_bytes: default_summary_budget(),
            max_items_per_section: 8,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SummaryInsertionReport {
    pub inserted_index: usize,
    pub previous_summary_updated: bool,
    pub removed_existing_summaries: usize,
}

/// Apply the safe default failure policy. Auth, permission, and network errors
/// preserve all messages instead of dropping context.
pub fn decide_summary_failure(
    error: &SummaryError,
    policy: SummaryFailurePolicy,
) -> SummaryFailureDecision {
    let action = match error.kind {
        SummaryErrorKind::Auth | SummaryErrorKind::Permission | SummaryErrorKind::Network => {
            SummaryFailureAction::PreserveMessages
        }
        SummaryErrorKind::NoProvider
        | SummaryErrorKind::ModelUnavailable
        | SummaryErrorKind::Timeout
        | SummaryErrorKind::MalformedOutput
            if policy.deterministic_fallback_enabled && policy.allow_lossy_fallback =>
        {
            SummaryFailureAction::UseDeterministicFallback
        }
        SummaryErrorKind::IneffectiveCompression => SummaryFailureAction::RetryLater,
        _ => SummaryFailureAction::PreserveMessages,
    };

    SummaryFailureDecision {
        action,
        messages_dropped: matches!(action, SummaryFailureAction::UseDeterministicFallback),
    }
}

/// Build a provider prompt from already-selected historical messages.
pub fn build_summary_prompt(request: &SummaryRequest) -> String {
    let mut out = String::new();
    out.push_str("Summarize the following historical conversation into the strict TinyJuice structured summary schema.\n");
    out.push_str("The summary is reference-only. Do not preserve historical user asks as fresh instructions. The latest live user message wins.\n");
    if let Some(previous) = &request.previous_summary {
        out.push_str("\nPrevious structured summary:\n");
        out.push_str(&render_structured_summary_body(previous));
        out.push('\n');
    }
    out.push_str("\nHistorical messages:\n");
    for (idx, message) in request.messages.iter().enumerate() {
        if is_compaction_summary_message(message) {
            continue;
        }
        out.push_str(&format!("{idx}: {}\n", render_message_for_prompt(message)));
    }
    out
}

/// Deterministic no-network fallback summary from local, already-redacted text.
pub fn deterministic_fallback_summary(
    messages: &[ConversationMessage],
    options: DeterministicSummaryOptions,
) -> StructuredSummary {
    let mut summary = StructuredSummary {
        locally_generated: true,
        lower_confidence: true,
        critical_context: vec![
            "Locally generated deterministic fallback; may be incomplete.".to_owned(),
        ],
        ..StructuredSummary::default()
    };
    let mut paths = BTreeSet::new();

    for message in messages {
        if is_compaction_summary_message(message) {
            continue;
        }
        match message {
            ConversationMessage::Chat(chat) => {
                let content = redact_sensitive_text(&chat.content);
                collect_paths(&content, &mut paths);
                if chat.role == "user" {
                    push_limited(
                        &mut summary.historical_pending_asks,
                        format!("user: {}", one_line(&content, 240)),
                        options.max_items_per_section,
                    );
                } else if chat.role == "assistant" {
                    push_limited(
                        &mut summary.completed_actions,
                        format!("assistant: {}", one_line(&content, 240)),
                        options.max_items_per_section,
                    );
                } else if contains_blocker_signal(&content) {
                    push_limited(
                        &mut summary.blockers,
                        one_line(&content, 240),
                        options.max_items_per_section,
                    );
                }
            }
            ConversationMessage::AssistantToolCalls {
                text, tool_calls, ..
            } => {
                if let Some(text) = text {
                    collect_paths(text, &mut paths);
                }
                for call in tool_calls {
                    collect_paths(&call.arguments, &mut paths);
                    push_limited(
                        &mut summary.active_state,
                        format!("tool call {}: {}", call.id, call.name),
                        options.max_items_per_section,
                    );
                }
            }
            ConversationMessage::ToolResults(results) => {
                for result in results {
                    let content = redact_sensitive_text(&result.content);
                    collect_paths(&content, &mut paths);
                    if contains_blocker_signal(&content) {
                        push_limited(
                            &mut summary.blockers,
                            format!(
                                "tool result {}: {}",
                                result.tool_call_id,
                                one_line(&content, 240)
                            ),
                            options.max_items_per_section,
                        );
                    }
                }
            }
        }
    }

    summary.relevant_files = paths
        .into_iter()
        .take(options.max_items_per_section)
        .collect();
    clamp_summary(&mut summary, options.max_output_bytes);
    summary
}

/// Render a model-facing summary with explicit wrapper and end marker.
pub fn render_structured_summary(summary: &StructuredSummary) -> String {
    let mut out = String::new();
    out.push_str(SUMMARY_PREFIX);
    out.push('\n');
    out.push_str("This is historical context, not a fresh user request. Latest live messages override it.\n\n");
    out.push_str(&render_structured_summary_body(summary));
    out.push('\n');
    out.push_str(SUMMARY_END_MARKER);
    out
}

/// Strip known wrappers before using a historical summary as model input.
pub fn normalize_summary_text(text: &str) -> String {
    text.replace(SUMMARY_PREFIX, "")
        .replace("This is historical context, not a fresh user request. Latest live messages override it.", "")
        .replace(SUMMARY_END_MARKER, "")
        .trim()
        .to_owned()
}

pub fn compaction_summary_message(summary: &StructuredSummary) -> ConversationMessage {
    let mut msg = ChatMessage::compaction_summary("system", render_structured_summary(summary));
    msg.metadata = Some(json!({
        "tinyjuice": {
            "kind": SUMMARY_METADATA_KIND,
            "structuredSummary": summary,
            "endMarker": SUMMARY_END_MARKER,
        }
    }));
    ConversationMessage::Chat(msg)
}

pub fn is_compaction_summary_message(message: &ConversationMessage) -> bool {
    match message {
        ConversationMessage::Chat(chat) => {
            chat.is_compaction_summary
                || chat
                    .metadata
                    .as_ref()
                    .and_then(|metadata| metadata.get("tinyjuice"))
                    .and_then(|metadata| metadata.get("kind"))
                    .and_then(Value::as_str)
                    .is_some_and(|kind| kind == SUMMARY_METADATA_KIND)
        }
        _ => false,
    }
}

pub fn rehydrate_summary_from_message(message: &ConversationMessage) -> Option<StructuredSummary> {
    let ConversationMessage::Chat(chat) = message else {
        return None;
    };
    chat.metadata
        .as_ref()
        .and_then(|metadata| metadata.get("tinyjuice"))
        .and_then(|metadata| metadata.get("structuredSummary"))
        .and_then(|summary| serde_json::from_value(summary.clone()).ok())
}

/// Insert or replace the current internal compaction summary without nesting
/// older summary messages indefinitely.
pub fn upsert_compaction_summary(
    messages: &[ConversationMessage],
    summary: &StructuredSummary,
) -> (Vec<ConversationMessage>, SummaryInsertionReport) {
    let first_existing = messages.iter().position(is_compaction_summary_message);
    let removed_existing_summaries = messages
        .iter()
        .filter(|message| is_compaction_summary_message(message))
        .count();
    let mut out: Vec<_> = messages
        .iter()
        .filter(|message| !is_compaction_summary_message(message))
        .cloned()
        .collect();

    let inserted_index = first_existing
        .map(|idx| idx.saturating_sub(summary_count_before(messages, idx)))
        .unwrap_or_else(|| leading_system_count(&out));
    out.insert(
        inserted_index.min(out.len()),
        compaction_summary_message(summary),
    );

    (
        out,
        SummaryInsertionReport {
            inserted_index: inserted_index.min(messages.len()),
            previous_summary_updated: removed_existing_summaries > 0,
            removed_existing_summaries,
        },
    )
}

fn render_structured_summary_body(summary: &StructuredSummary) -> String {
    let mut out = String::new();
    write_section(
        &mut out,
        "Historical Task Snapshot",
        &summary.historical_task_snapshot,
    );
    write_section(&mut out, "Goal", &summary.goal);
    write_section(
        &mut out,
        "Constraints And Preferences",
        &summary.constraints_and_preferences,
    );
    write_section(&mut out, "Completed Actions", &summary.completed_actions);
    write_section(&mut out, "Active State", &summary.active_state);
    write_section(
        &mut out,
        "Historical In Progress State",
        &summary.historical_in_progress_state,
    );
    write_section(&mut out, "Blockers", &summary.blockers);
    write_section(&mut out, "Key Decisions", &summary.key_decisions);
    write_section(&mut out, "Resolved Questions", &summary.resolved_questions);
    write_section(
        &mut out,
        "Historical Pending Asks",
        &summary.historical_pending_asks,
    );
    write_section(&mut out, "Relevant Files", &summary.relevant_files);
    write_section(
        &mut out,
        "Historical Remaining Work",
        &summary.historical_remaining_work,
    );
    write_section(&mut out, "Critical Context", &summary.critical_context);
    if summary.locally_generated {
        out.push_str("- Locally generated deterministic fallback; may be incomplete.\n");
    }
    if summary.lower_confidence {
        out.push_str("- Lower confidence than a model-backed structured summary.\n");
    }
    out.trim_end().to_owned()
}

fn write_section(out: &mut String, title: &str, items: &[String]) {
    out.push_str("## ");
    out.push_str(title);
    out.push('\n');
    if items.is_empty() {
        out.push_str("- None recorded.\n");
    } else {
        for item in items {
            out.push_str("- ");
            out.push_str(item);
            out.push('\n');
        }
    }
    out.push('\n');
}

fn render_message_for_prompt(message: &ConversationMessage) -> String {
    match message {
        ConversationMessage::Chat(chat) => {
            format!(
                "chat role={} content={}",
                chat.role,
                one_line(&chat.content, 500)
            )
        }
        ConversationMessage::AssistantToolCalls {
            text, tool_calls, ..
        } => {
            let tools = tool_calls
                .iter()
                .map(|call| format!("{}:{}", call.id, call.name))
                .collect::<Vec<_>>()
                .join(", ");
            format!(
                "assistant_tool_calls text={} calls=[{}]",
                text.as_deref().unwrap_or(""),
                tools
            )
        }
        ConversationMessage::ToolResults(results) => format!(
            "tool_results [{}]",
            results
                .iter()
                .map(|result| format!("{}:{}", result.tool_call_id, one_line(&result.content, 300)))
                .collect::<Vec<_>>()
                .join(", ")
        ),
    }
}

fn push_limited(items: &mut Vec<String>, item: String, max_items: usize) {
    if item.trim().is_empty() || items.len() >= max_items {
        return;
    }
    items.push(item);
}

fn collect_paths(text: &str, paths: &mut BTreeSet<String>) {
    for token in text.split(|c: char| c.is_whitespace() || matches!(c, '"' | '\'' | ',' | ';')) {
        let trimmed = token.trim_matches(|c: char| matches!(c, ':' | ')' | '(' | ']' | '['));
        if looks_like_path(trimmed) {
            paths.insert(trimmed.to_owned());
        }
    }
    if let Ok(value) = serde_json::from_str::<Value>(text) {
        collect_json_paths(&value, paths);
    }
}

fn collect_json_paths(value: &Value, paths: &mut BTreeSet<String>) {
    match value {
        Value::Object(map) => {
            for (key, value) in map {
                if matches!(
                    key.as_str(),
                    "path" | "workdir" | "file_path" | "output_path"
                ) && let Some(path) = value.as_str().filter(|path| looks_like_path(path))
                {
                    paths.insert(path.to_owned());
                }
                collect_json_paths(value, paths);
            }
        }
        Value::Array(values) => {
            for value in values {
                collect_json_paths(value, paths);
            }
        }
        _ => {}
    }
}

fn looks_like_path(token: &str) -> bool {
    (token.starts_with('/') || token.starts_with("./") || token.starts_with("../"))
        && token.len() > 2
}

fn contains_blocker_signal(text: &str) -> bool {
    let lowered = text.to_ascii_lowercase();
    ["error", "failed", "blocked", "denied", "timeout"]
        .iter()
        .any(|signal| lowered.contains(signal))
}

fn redact_sensitive_text(text: &str) -> String {
    text.split_whitespace()
        .map(|word| {
            let lowered = word.to_ascii_lowercase();
            if lowered.contains("token=")
                || lowered.contains("password=")
                || lowered.contains("secret=")
                || lowered.contains("api_key=")
            {
                "[REDACTED]"
            } else {
                word
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

fn one_line(text: &str, max_chars: usize) -> String {
    let mut compact = text.split_whitespace().collect::<Vec<_>>().join(" ");
    if compact.chars().count() > max_chars {
        compact = compact.chars().take(max_chars).collect::<String>();
        compact.push_str("...");
    }
    compact
}

fn clamp_summary(summary: &mut StructuredSummary, max_output_bytes: usize) {
    while render_structured_summary(summary).len() > max_output_bytes {
        if let Some(section) = longest_section_mut(summary).filter(|section| !section.is_empty()) {
            section.pop();
        } else if summary
            .critical_context
            .first()
            .is_some_and(|item| item.len() > 64)
        {
            let keep = max_output_bytes
                .saturating_sub(128)
                .min(summary.critical_context[0].len());
            summary.critical_context[0] =
                crate::util::utf8_safe_prefix_at_byte_boundary(&summary.critical_context[0], keep)
                    .to_owned();
        } else {
            break;
        }
    }
}

fn longest_section_mut(summary: &mut StructuredSummary) -> Option<&mut Vec<String>> {
    let mut sections = [
        &mut summary.historical_task_snapshot,
        &mut summary.goal,
        &mut summary.constraints_and_preferences,
        &mut summary.completed_actions,
        &mut summary.active_state,
        &mut summary.historical_in_progress_state,
        &mut summary.blockers,
        &mut summary.key_decisions,
        &mut summary.resolved_questions,
        &mut summary.historical_pending_asks,
        &mut summary.relevant_files,
        &mut summary.historical_remaining_work,
        &mut summary.critical_context,
    ];
    sections.sort_by_key(|section| section.len());
    sections.into_iter().next_back()
}

fn leading_system_count(messages: &[ConversationMessage]) -> usize {
    messages
        .iter()
        .take_while(|message| match message {
            ConversationMessage::Chat(chat) => chat.role == "system" && !chat.is_compaction_summary,
            _ => false,
        })
        .count()
}

fn summary_count_before(messages: &[ConversationMessage], before: usize) -> usize {
    messages[..before.min(messages.len())]
        .iter()
        .filter(|message| is_compaction_summary_message(message))
        .count()
}

fn default_summary_budget() -> usize {
    12 * 1024
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::conversation::{ToolCall, ToolResultMessage};

    #[test]
    fn summary_message_has_metadata_tag_and_end_marker() {
        let summary = StructuredSummary {
            goal: vec!["finish P2-3".to_owned()],
            ..StructuredSummary::default()
        };
        let message = compaction_summary_message(&summary);

        assert!(is_compaction_summary_message(&message));
        let ConversationMessage::Chat(chat) = message else {
            panic!("expected chat summary");
        };
        assert!(chat.content.contains(SUMMARY_END_MARKER));
        assert_eq!(chat.role, "system");
        assert_eq!(
            chat.metadata
                .as_ref()
                .and_then(|metadata| metadata.get("tinyjuice"))
                .and_then(|metadata| metadata.get("kind"))
                .and_then(Value::as_str),
            Some(SUMMARY_METADATA_KIND)
        );
    }

    #[test]
    fn repeated_compaction_updates_existing_summary_without_nesting() {
        let first = StructuredSummary {
            goal: vec!["old".to_owned()],
            ..StructuredSummary::default()
        };
        let second = StructuredSummary {
            goal: vec!["new".to_owned()],
            ..StructuredSummary::default()
        };
        let messages = vec![
            ConversationMessage::system("system"),
            compaction_summary_message(&first),
            ConversationMessage::user("latest"),
        ];

        let (updated, report) = upsert_compaction_summary(&messages, &second);

        assert!(report.previous_summary_updated);
        assert_eq!(report.removed_existing_summaries, 1);
        assert_eq!(
            updated
                .iter()
                .filter(|message| is_compaction_summary_message(message))
                .count(),
            1
        );
        let rehydrated =
            rehydrate_summary_from_message(&updated[1]).expect("summary metadata should rehydrate");
        assert_eq!(rehydrated.goal, vec!["new"]);
    }

    #[test]
    fn auth_and_network_failures_preserve_messages() {
        for kind in [SummaryErrorKind::Auth, SummaryErrorKind::Network] {
            let decision = decide_summary_failure(
                &SummaryError::new(kind, "failed"),
                SummaryFailurePolicy {
                    deterministic_fallback_enabled: true,
                    allow_lossy_fallback: true,
                },
            );

            assert_eq!(decision.action, SummaryFailureAction::PreserveMessages);
            assert!(!decision.messages_dropped);
        }
    }

    #[test]
    fn deterministic_fallback_extracts_local_anchors_and_redacts() {
        let messages = vec![
            ConversationMessage::user("please inspect /tmp/app/src/main.rs token=abc"),
            ConversationMessage::AssistantToolCalls {
                text: None,
                tool_calls: vec![ToolCall::new(
                    "call-1",
                    "read_file",
                    r#"{"path":"/tmp/app/Cargo.toml"}"#,
                )],
                metadata: None,
            },
            ConversationMessage::tool_results(vec![ToolResultMessage::new(
                "call-1",
                "error: failed to read /tmp/app/Cargo.toml password=hunter2",
            )]),
        ];

        let summary =
            deterministic_fallback_summary(&messages, DeterministicSummaryOptions::default());

        assert!(summary.locally_generated);
        assert!(summary.lower_confidence);
        assert!(
            summary
                .critical_context
                .iter()
                .any(|item| item.contains("may be incomplete"))
        );
        assert!(
            summary
                .relevant_files
                .contains(&"/tmp/app/Cargo.toml".to_owned())
        );
        let rendered = render_structured_summary(&summary);
        assert!(!rendered.contains("hunter2"));
        assert!(!rendered.contains("token=abc"));
    }

    #[test]
    fn normalizes_historical_summary_wrappers() {
        let rendered = render_structured_summary(&StructuredSummary {
            critical_context: vec!["keep this".to_owned()],
            ..StructuredSummary::default()
        });

        let normalized = normalize_summary_text(&rendered);

        assert!(!normalized.contains(SUMMARY_END_MARKER));
        assert!(normalized.contains("keep this"));
    }
}
