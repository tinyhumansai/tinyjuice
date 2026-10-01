use super::*;
use crate::conversation::{ChatMessage, ToolCall};

#[test]
fn retained_tool_result_aligns_to_parent_call() {
    let messages = vec![
        ConversationMessage::user("ask"),
        ConversationMessage::assistant_tool_calls(vec![ToolCall::new("a", "read_file", "{}")]),
        ConversationMessage::tool_results(vec![ToolResultMessage::new("a", "file body")]),
        ConversationMessage::assistant("done"),
    ];

    assert_eq!(align_tail_start_for_tool_boundaries(&messages, 2), 1);
}

#[test]
fn boundary_alignment_rechecks_after_moving_to_parent_call() {
    let messages = vec![
        ConversationMessage::user("ask"),
        ConversationMessage::assistant_tool_calls(vec![ToolCall::new("a", "read_file", "{}")]),
        ConversationMessage::assistant_tool_calls(vec![ToolCall::new("b", "shell", "{}")]),
        ConversationMessage::tool_results(vec![ToolResultMessage::new("a", "file body")]),
        ConversationMessage::tool_results(vec![ToolResultMessage::new("b", "shell body")]),
        ConversationMessage::assistant("done"),
    ];

    assert_eq!(align_tail_start_for_tool_boundaries(&messages, 4), 1);
}

#[test]
fn generated_tail_boundaries_retain_parent_tool_calls() {
    for round_count in 1..=6 {
        for grouped_results in [false, true] {
            for trailing_assistant in [false, true] {
                let messages =
                    generated_tool_history(round_count, grouped_results, trailing_assistant);

                for tail_start in 0..=messages.len() {
                    let aligned = align_tail_start_for_tool_boundaries(&messages, tail_start);
                    let retained_calls = call_ids_in_range(&messages, aligned, messages.len());

                    for message in &messages[aligned..] {
                        if let ConversationMessage::ToolResults(results) = message {
                            for result in results {
                                assert!(
                                    retained_calls.contains(&result.tool_call_id),
                                    "missing parent for {} after aligning tail_start={} to {} in {:?}",
                                    result.tool_call_id,
                                    tail_start,
                                    aligned,
                                    messages
                                );
                            }
                        }
                    }
                }
            }
        }
    }
}

#[test]
fn orphan_tool_result_is_removed() {
    let messages = vec![
        ConversationMessage::tool_results(vec![ToolResultMessage::new("missing", "body")]),
        ConversationMessage::assistant("done"),
    ];
    let sanitized = sanitize_orphan_tool_messages(&messages);
    assert_eq!(sanitized, vec![ConversationMessage::assistant("done")]);
}

#[test]
fn anchors_skip_internal_summary_messages() {
    let messages = vec![
        ConversationMessage::Chat(ChatMessage::compaction_summary("user", "old ask")),
        ConversationMessage::user("real ask"),
        ConversationMessage::Chat(ChatMessage::compaction_summary("assistant", "summary")),
        ConversationMessage::assistant("visible"),
    ];

    assert_eq!(latest_real_user_index(&messages), Some(1));
    assert_eq!(latest_visible_assistant_index(&messages), Some(3));
}

#[test]
fn partial_split_clamps_overlapping_boundaries() {
    let messages = vec![
        ConversationMessage::system("sys"),
        ConversationMessage::user("ask"),
        ConversationMessage::assistant("answer"),
    ];

    let split = split_partial_conversation(&messages, 2, 1);

    assert_eq!(split.head, messages[..2].to_vec());
    assert!(split.middle.is_empty());
    assert_eq!(split.tail, messages[2..].to_vec());
    assert_eq!(split.head_end, 2);
    assert_eq!(split.tail_start, 2);
}

#[test]
fn partial_rejoin_replaces_middle_and_preserves_edges() {
    let messages = vec![
        ConversationMessage::system("sys"),
        ConversationMessage::user("old"),
        ConversationMessage::assistant("old answer"),
        ConversationMessage::user("new"),
    ];
    let split = split_partial_conversation(&messages, 1, 3);

    let rejoined =
        rejoin_partial_conversation(&split, vec![ConversationMessage::assistant("summary")]);

    assert_eq!(
        rejoined,
        vec![
            ConversationMessage::system("sys"),
            ConversationMessage::assistant("summary"),
            ConversationMessage::user("new"),
        ]
    );
}

#[test]
fn partial_rejoin_sanitizes_orphaned_tool_pairs() {
    let messages = vec![
        ConversationMessage::user("ask"),
        ConversationMessage::assistant_tool_calls(vec![ToolCall::new("a", "read_file", "{}")]),
        ConversationMessage::tool_results(vec![ToolResultMessage::new("a", "body")]),
        ConversationMessage::assistant("done"),
    ];
    let split = split_partial_conversation(&messages, 0, 3);

    let rejoined =
        rejoin_partial_conversation(&split, vec![ConversationMessage::assistant("summary")]);

    assert_eq!(
        rejoined,
        vec![
            ConversationMessage::assistant("summary"),
            ConversationMessage::assistant("done"),
        ]
    );
}

#[test]
fn last_user_exchange_tail_starts_at_nth_recent_real_user() {
    let messages = vec![
        ConversationMessage::system("sys"),
        ConversationMessage::user("old ask"),
        ConversationMessage::assistant("old answer"),
        ConversationMessage::user("middle ask"),
        ConversationMessage::assistant("middle answer"),
        ConversationMessage::user("latest ask"),
        ConversationMessage::assistant("latest answer"),
    ];

    assert_eq!(tail_start_for_last_user_exchanges(&messages, 1, 2), 3);
}

#[test]
fn last_user_exchange_tail_skips_summaries_and_hidden_users() {
    let mut hidden_user = ChatMessage::user("hidden ask");
    hidden_user.hidden = true;
    let messages = vec![
        ConversationMessage::system("sys"),
        ConversationMessage::user("old ask"),
        ConversationMessage::assistant("old answer"),
        ConversationMessage::Chat(hidden_user),
        ConversationMessage::Chat(ChatMessage::compaction_summary("user", "summary ask")),
        ConversationMessage::user("latest ask"),
        ConversationMessage::assistant("latest answer"),
    ];

    let split = split_for_last_user_exchanges(&messages, 1, 1);

    assert_eq!(split.middle, messages[1..5].to_vec());
    assert_eq!(split.tail, messages[5..].to_vec());
    assert_eq!(split.tail_start, 5);
}

#[test]
fn last_user_exchange_tail_keeps_unprotected_region_when_count_exceeds_history() {
    let messages = vec![
        ConversationMessage::system("sys"),
        ConversationMessage::user("only ask"),
        ConversationMessage::assistant("answer"),
    ];

    let split = split_for_last_user_exchanges(&messages, 1, 3);

    assert!(split.middle.is_empty());
    assert_eq!(split.tail, messages[1..].to_vec());
    assert_eq!(split.tail_start, 1);
}

#[test]
fn zero_user_exchanges_retains_no_tail() {
    let messages = vec![
        ConversationMessage::system("sys"),
        ConversationMessage::user("ask"),
        ConversationMessage::assistant("answer"),
    ];

    let split = split_for_last_user_exchanges(&messages, 1, 0);

    assert_eq!(split.middle, messages[1..].to_vec());
    assert!(split.tail.is_empty());
    assert_eq!(split.tail_start, messages.len());
}

fn generated_tool_history(
    round_count: usize,
    grouped_results: bool,
    trailing_assistant: bool,
) -> Vec<ConversationMessage> {
    let mut messages = vec![
        ConversationMessage::system("sys"),
        ConversationMessage::user("ask"),
    ];

    if grouped_results {
        for idx in 0..round_count {
            messages.push(ConversationMessage::assistant_tool_calls(vec![
                ToolCall::new(
                    format!("call-{idx}"),
                    "shell",
                    format!(r#"{{"command":"echo {idx}"}}"#),
                ),
            ]));
        }
        messages.push(ConversationMessage::tool_results(
            (0..round_count)
                .map(|idx| ToolResultMessage::new(format!("call-{idx}"), format!("out {idx}")))
                .collect(),
        ));
    } else {
        for idx in 0..round_count {
            messages.push(ConversationMessage::assistant_tool_calls(vec![
                ToolCall::new(
                    format!("call-{idx}"),
                    "shell",
                    format!(r#"{{"command":"echo {idx}"}}"#),
                ),
            ]));
            messages.push(ConversationMessage::tool_results(vec![
                ToolResultMessage::new(format!("call-{idx}"), format!("out {idx}")),
            ]));
        }
    }

    if trailing_assistant {
        messages.push(ConversationMessage::assistant("done"));
    }
    messages
}
