use std::collections::HashSet;

use crate::conversation::{ConversationMessage, ToolCall, ToolResultMessage};

/// A conversation split into a protected head, compactible middle, and retained tail.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PartialConversationSplit {
    pub head: Vec<ConversationMessage>,
    pub middle: Vec<ConversationMessage>,
    pub tail: Vec<ConversationMessage>,
    pub head_end: usize,
    pub tail_start: usize,
}

/// Move a retained-tail start backward when needed so retained tool results
/// keep their parent assistant tool-call message.
pub fn align_tail_start_for_tool_boundaries(
    messages: &[ConversationMessage],
    tail_start: usize,
) -> usize {
    let mut start = tail_start.min(messages.len());
    loop {
        let retained_calls = call_ids_in_range(messages, start, messages.len());
        let missing_result_parent = messages[start..].iter().find_map(|message| match message {
            ConversationMessage::ToolResults(results) => results
                .iter()
                .find(|result| !retained_calls.contains(&result.tool_call_id))
                .and_then(|result| find_parent_call_index(messages, start, &result.tool_call_id)),
            _ => None,
        });

        let Some(parent_idx) = missing_result_parent else {
            return start;
        };
        if parent_idx >= start {
            return start;
        }
        start = parent_idx;
    }
}

/// Remove provider-invalid orphan tool messages from a retained conversation.
pub fn sanitize_orphan_tool_messages(messages: &[ConversationMessage]) -> Vec<ConversationMessage> {
    let all_results = result_ids_in_range(messages, 0, messages.len());
    let all_calls = call_ids_in_range(messages, 0, messages.len());

    messages
        .iter()
        .filter_map(|message| match message {
            ConversationMessage::AssistantToolCalls {
                text,
                tool_calls,
                metadata,
            } => {
                let kept: Vec<ToolCall> = tool_calls
                    .iter()
                    .filter(|call| all_results.contains(&call.id))
                    .cloned()
                    .collect();
                if kept.is_empty() && text.as_deref().unwrap_or("").is_empty() {
                    None
                } else {
                    Some(ConversationMessage::AssistantToolCalls {
                        text: text.clone(),
                        tool_calls: kept,
                        metadata: metadata.clone(),
                    })
                }
            }
            ConversationMessage::ToolResults(results) => {
                let kept: Vec<ToolResultMessage> = results
                    .iter()
                    .filter(|result| all_calls.contains(&result.tool_call_id))
                    .cloned()
                    .collect();
                (!kept.is_empty()).then_some(ConversationMessage::ToolResults(kept))
            }
            ConversationMessage::Chat(_) => Some(message.clone()),
        })
        .collect()
}

/// Latest user message that is not an internal compaction summary.
pub fn latest_real_user_index(messages: &[ConversationMessage]) -> Option<usize> {
    messages.iter().rposition(|message| match message {
        ConversationMessage::Chat(chat) => {
            chat.role == "user" && !chat.is_compaction_summary && !chat.hidden
        }
        _ => false,
    })
}

/// Latest visible assistant reply that is not an internal compaction summary.
pub fn latest_visible_assistant_index(messages: &[ConversationMessage]) -> Option<usize> {
    messages.iter().rposition(|message| match message {
        ConversationMessage::Chat(chat) => {
            chat.role == "assistant"
                && !chat.content.trim().is_empty()
                && !chat.is_compaction_summary
                && !chat.hidden
        }
        ConversationMessage::AssistantToolCalls { text, .. } => {
            text.as_deref().is_some_and(|text| !text.trim().is_empty())
        }
        _ => false,
    })
}

/// Split a conversation into protected head, compactible middle, and retained tail.
///
/// `head_end` and `tail_start` are clamped to valid message boundaries. If they
/// overlap, the tail starts at `head_end`, leaving an empty middle.
pub fn split_partial_conversation(
    messages: &[ConversationMessage],
    head_end: usize,
    tail_start: usize,
) -> PartialConversationSplit {
    let head_end = head_end.min(messages.len());
    let tail_start = tail_start.clamp(head_end, messages.len());
    PartialConversationSplit {
        head: messages[..head_end].to_vec(),
        middle: messages[head_end..tail_start].to_vec(),
        tail: messages[tail_start..].to_vec(),
        head_end,
        tail_start,
    }
}

/// Rejoin a partial split after replacing the compactible middle.
///
/// The final transcript is sanitized so retained tool results have retained
/// parent calls, and retained parent calls without results lose their tool-call
/// payload unless they also have visible assistant text.
pub fn rejoin_partial_conversation(
    split: &PartialConversationSplit,
    replacement_middle: impl IntoIterator<Item = ConversationMessage>,
) -> Vec<ConversationMessage> {
    let mut messages =
        Vec::with_capacity(split.head.len() + split.middle.len().min(1) + split.tail.len());
    messages.extend(split.head.iter().cloned());
    messages.extend(replacement_middle);
    messages.extend(split.tail.iter().cloned());
    sanitize_orphan_tool_messages(&messages)
}

fn call_ids_in_range(
    messages: &[ConversationMessage],
    start: usize,
    end: usize,
) -> HashSet<String> {
    messages[start.min(messages.len())..end.min(messages.len())]
        .iter()
        .flat_map(|message| match message {
            ConversationMessage::AssistantToolCalls { tool_calls, .. } => {
                tool_calls.iter().map(|call| call.id.clone()).collect()
            }
            _ => Vec::new(),
        })
        .collect()
}

fn result_ids_in_range(
    messages: &[ConversationMessage],
    start: usize,
    end: usize,
) -> HashSet<String> {
    messages[start.min(messages.len())..end.min(messages.len())]
        .iter()
        .flat_map(|message| match message {
            ConversationMessage::ToolResults(results) => results
                .iter()
                .map(|result| result.tool_call_id.clone())
                .collect(),
            _ => Vec::new(),
        })
        .collect()
}

fn find_parent_call_index(
    messages: &[ConversationMessage],
    before: usize,
    call_id: &str,
) -> Option<usize> {
    messages[..before.min(messages.len())]
        .iter()
        .rposition(|message| match message {
            ConversationMessage::AssistantToolCalls { tool_calls, .. } => {
                tool_calls.iter().any(|call| call.id == call_id)
            }
            _ => false,
        })
}

#[cfg(test)]
mod tests {
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
}
