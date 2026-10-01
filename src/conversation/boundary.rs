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
    messages.iter().rposition(is_real_user_message)
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

/// Select the retained-tail start for the last `exchange_count` real user turns.
///
/// Real user turns exclude hidden messages and internal compaction summaries.
/// If there are fewer real user turns than requested, the whole unprotected
/// region after `head_end` is retained. The returned start is moved backward
/// when needed to keep retained tool results with their parent tool calls.
pub fn tail_start_for_last_user_exchanges(
    messages: &[ConversationMessage],
    head_end: usize,
    exchange_count: usize,
) -> usize {
    let head_end = head_end.min(messages.len());
    if exchange_count == 0 || head_end >= messages.len() {
        return messages.len();
    }

    let mut seen = 0usize;
    for idx in (head_end..messages.len()).rev() {
        if is_real_user_message(&messages[idx]) {
            seen += 1;
            if seen == exchange_count {
                return align_tail_start_for_tool_boundaries(messages, idx).max(head_end);
            }
        }
    }

    head_end
}

/// Split a conversation while retaining the last `exchange_count` real user turns.
pub fn split_for_last_user_exchanges(
    messages: &[ConversationMessage],
    head_end: usize,
    exchange_count: usize,
) -> PartialConversationSplit {
    let tail_start = tail_start_for_last_user_exchanges(messages, head_end, exchange_count);
    split_partial_conversation(messages, head_end, tail_start)
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

fn is_real_user_message(message: &ConversationMessage) -> bool {
    matches!(
        message,
        ConversationMessage::Chat(chat)
            if chat.role == "user" && !chat.is_compaction_summary && !chat.hidden
    )
}

#[cfg(test)]
#[path = "boundary_tests.rs"]
mod tests;
