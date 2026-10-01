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

    let summary = deterministic_fallback_summary(&messages, DeterministicSummaryOptions::default());

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
