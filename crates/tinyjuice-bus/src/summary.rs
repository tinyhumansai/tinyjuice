//! Summary callback prompt and error vocabulary shared with hosts.

/// Prompt used by the tool-output summary callback.
pub const SYSTEM_PROMPT: &str = include_str!("summary_prompt.md");

/// Why a payload that qualified for a summary did not get one.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnavailableReason {
    /// Larger than `llm_summary_max_input_tokens`.
    PayloadTooLarge,
    /// Model summaries are disabled or the breaker is open for this scope.
    Disabled,
    /// The model call failed or its reply was empty or not smaller.
    Failed,
    /// The model call outlasted `llm_summary_timeout_ms`.
    TimedOut,
}

impl UnavailableReason {
    /// The model-facing notice for this reason.
    ///
    /// A host prefixes it to the tool result *after* its own caps have run, so
    /// a cap cannot cut it. Every variant ends with the same bare instruction —
    /// do not re-run the tool for a summary — because the reasonable response
    /// to an unsummarized dump is otherwise to call the tool again, which reads
    /// to a user as a hang. It carries no justifying clause: each one tried
    /// ("it will return the same result", "the full output is already here")
    /// was a claim this code cannot make for every tool.
    #[must_use]
    pub fn notice(self) -> &'static str {
        match self {
            Self::PayloadTooLarge => concat!(
                "[summarization unavailable — this output exceeds the summarizer's ",
                "size cap, so the tool output follows and may be truncated. ",
                "Do not re-run the tool for a summary.]"
            ),
            Self::Disabled => concat!(
                "[summarization unavailable — it is switched off for this session ",
                "after repeated failures, so the tool output follows. ",
                "Do not re-run the tool for a summary.]"
            ),
            Self::TimedOut => concat!(
                "[summarization timed out — the tool output follows. If a recovery handle ",
                "appears in its footer, inspect the stored output with the juice_* tools ",
                "using that handle. Do not re-run the tool for a summary.]"
            ),
            Self::Failed => concat!(
                "[summarization unavailable — the summarizer did not return a usable ",
                "summary for this result, so the tool output follows. ",
                "Do not re-run the tool for a summary.]"
            ),
        }
    }
}
