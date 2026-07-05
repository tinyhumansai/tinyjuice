use std::collections::HashSet;

use crate::conversation::{ConversationMessage, ToolCall, ToolResultMessage};

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
}
