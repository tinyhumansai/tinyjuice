use crate::conversation::ConversationMessage;
use crate::tokens::estimate_tokens;

/// Request-window parameters for conversation compaction.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ConversationBudget {
    pub context_length: usize,
    pub max_output_tokens: usize,
    pub threshold_ratio: f32,
    pub minimum_context_floor: usize,
}

impl Default for ConversationBudget {
    fn default() -> Self {
        Self {
            context_length: 128_000,
            max_output_tokens: 16_384,
            threshold_ratio: 0.85,
            minimum_context_floor: 64_000,
        }
    }
}

/// Non-sensitive details for a tail-budget decision.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TailBudgetSelection {
    pub tail_start: usize,
    pub selected_tokens: usize,
    pub message_count: usize,
}

/// Options for preserving the beginning of a transcript.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HeadProtection {
    pub protect_system_prompt: bool,
    pub protect_first_n_messages: usize,
    pub decay_head_after_first_compaction: bool,
    pub compaction_count: usize,
}

impl Default for HeadProtection {
    fn default() -> Self {
        Self {
            protect_system_prompt: true,
            protect_first_n_messages: 0,
            decay_head_after_first_compaction: true,
            compaction_count: 0,
        }
    }
}

/// Input window after reserving output tokens.
pub fn effective_input_window(context_length: usize, max_output_tokens: usize) -> usize {
    context_length.saturating_sub(max_output_tokens)
}

/// Threshold at which conversation compaction should trigger.
pub fn threshold_tokens(budget: ConversationBudget) -> usize {
    let effective = effective_input_window(budget.context_length, budget.max_output_tokens);
    if effective == 0 {
        return 0;
    }

    let ratio = budget.threshold_ratio.clamp(0.01, 0.99);
    let ratio_threshold = ((effective as f32) * ratio).floor() as usize;
    let threshold = ratio_threshold.max(budget.minimum_context_floor);

    if threshold >= effective {
        ((effective as f32) * 0.85).floor().max(1.0) as usize
    } else {
        threshold
    }
}

/// Return the exclusive end index of the protected head.
pub fn protected_head_end(messages: &[ConversationMessage], options: HeadProtection) -> usize {
    let system_end = if options.protect_system_prompt {
        messages
            .iter()
            .take_while(|message| match message {
                ConversationMessage::Chat(chat) => chat.role == "system",
                _ => false,
            })
            .count()
    } else {
        0
    };

    let keep_initial_head =
        !options.decay_head_after_first_compaction || options.compaction_count == 0;
    let first_n_end = if keep_initial_head {
        options.protect_first_n_messages.min(messages.len())
    } else {
        0
    };

    system_end.max(first_n_end)
}

/// Rough token estimate for a provider-neutral message, including envelope
/// overhead and tool-call argument cost.
pub fn estimate_message_tokens(message: &ConversationMessage) -> usize {
    const ROLE_OVERHEAD: usize = 4;
    const TOOL_CALL_OVERHEAD: usize = 12;
    const TOOL_RESULT_OVERHEAD: usize = 8;

    match message {
        ConversationMessage::Chat(chat) => {
            ROLE_OVERHEAD
                + estimate_tokens(&chat.role) as usize
                + estimate_tokens(&chat.content) as usize
        }
        ConversationMessage::AssistantToolCalls {
            text, tool_calls, ..
        } => {
            let text_tokens = text.as_deref().map(estimate_tokens).unwrap_or(0) as usize;
            let tool_tokens = tool_calls
                .iter()
                .map(|call| {
                    TOOL_CALL_OVERHEAD
                        + estimate_tokens(&call.id) as usize
                        + estimate_tokens(&call.name) as usize
                        + estimate_tokens(&call.arguments) as usize
                })
                .sum::<usize>();
            ROLE_OVERHEAD + text_tokens + tool_tokens
        }
        ConversationMessage::ToolResults(results) => {
            ROLE_OVERHEAD
                + results
                    .iter()
                    .map(|result| {
                        TOOL_RESULT_OVERHEAD
                            + estimate_tokens(&result.tool_call_id) as usize
                            + estimate_tokens(&result.content) as usize
                    })
                    .sum::<usize>()
        }
    }
}

/// Select the start index of the recent tail to keep under a rough token
/// budget. Returns an index in `messages`.
pub fn select_tail_by_budget(
    messages: &[ConversationMessage],
    head_end: usize,
    token_budget: usize,
    min_tail_messages: usize,
    soft_ceiling_ratio: f32,
) -> usize {
    select_tail_by_budget_detail(
        messages,
        head_end,
        token_budget,
        min_tail_messages,
        soft_ceiling_ratio,
    )
    .tail_start
}

pub fn select_tail_by_budget_detail(
    messages: &[ConversationMessage],
    head_end: usize,
    token_budget: usize,
    min_tail_messages: usize,
    soft_ceiling_ratio: f32,
) -> TailBudgetSelection {
    let head_end = head_end.min(messages.len());
    if head_end >= messages.len() {
        return TailBudgetSelection {
            tail_start: messages.len(),
            selected_tokens: 0,
            message_count: 0,
        };
    }

    let region = &messages[head_end..];
    let region_tokens = region.iter().map(estimate_message_tokens).sum::<usize>();
    let soft_ceiling = ((token_budget as f32) * soft_ceiling_ratio.max(1.0)).ceil() as usize;
    if region_tokens <= soft_ceiling {
        return TailBudgetSelection {
            tail_start: head_end,
            selected_tokens: region_tokens,
            message_count: region.len(),
        };
    }

    let floor = min_tail_messages.min(messages.len() - head_end);
    let mut selected_tokens = 0usize;
    let mut selected_count = 0usize;
    let mut tail_start = messages.len();

    for idx in (head_end..messages.len()).rev() {
        let msg_tokens = estimate_message_tokens(&messages[idx]);
        let must_keep_for_floor = selected_count < floor;
        let within_budget = selected_tokens.saturating_add(msg_tokens) <= token_budget;
        let allow_single_overshoot =
            selected_count == 0 && msg_tokens <= soft_ceiling.max(token_budget);

        if must_keep_for_floor || within_budget || allow_single_overshoot {
            selected_tokens = selected_tokens.saturating_add(msg_tokens);
            selected_count += 1;
            tail_start = idx;
        } else {
            break;
        }
    }

    TailBudgetSelection {
        tail_start,
        selected_tokens,
        message_count: selected_count,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::conversation::{ChatMessage, ToolResultMessage};

    #[test]
    fn threshold_stays_below_small_effective_window() {
        let budget = ConversationBudget {
            context_length: 64_000,
            max_output_tokens: 8_000,
            threshold_ratio: 0.9,
            minimum_context_floor: 64_000,
        };
        let effective = effective_input_window(budget.context_length, budget.max_output_tokens);
        assert!(threshold_tokens(budget) < effective);
    }

    #[test]
    fn output_reservation_reduces_input_budget() {
        assert_eq!(effective_input_window(128_000, 16_000), 112_000);
    }

    #[test]
    fn huge_old_tool_result_does_not_force_preserving_recent_history() {
        let messages = vec![
            ConversationMessage::Chat(ChatMessage::system("sys")),
            ConversationMessage::tool_results(vec![ToolResultMessage::new(
                "old",
                "x".repeat(20_000),
            )]),
            ConversationMessage::user("latest ask"),
            ConversationMessage::assistant("latest answer"),
        ];

        let start = select_tail_by_budget(&messages, 1, 20, 2, 1.5);
        assert_eq!(start, 2);
    }

    #[test]
    fn short_transcript_selects_whole_region() {
        let messages = vec![
            ConversationMessage::system("sys"),
            ConversationMessage::user("hi"),
            ConversationMessage::assistant("hello"),
        ];
        assert_eq!(select_tail_by_budget(&messages, 1, 100, 2, 1.25), 1);
    }

    #[test]
    fn protected_head_decays_after_first_compaction_but_keeps_system() {
        let messages = vec![
            ConversationMessage::system("sys"),
            ConversationMessage::user("opening task"),
            ConversationMessage::assistant("ack"),
            ConversationMessage::user("latest"),
        ];

        let first = protected_head_end(
            &messages,
            HeadProtection {
                protect_first_n_messages: 3,
                compaction_count: 0,
                ..Default::default()
            },
        );
        let repeated = protected_head_end(
            &messages,
            HeadProtection {
                protect_first_n_messages: 3,
                compaction_count: 1,
                ..Default::default()
            },
        );

        assert_eq!(first, 3);
        assert_eq!(repeated, 1);
    }
}
