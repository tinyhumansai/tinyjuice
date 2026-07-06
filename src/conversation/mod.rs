//! Provider-neutral conversation compaction primitives.
//!
//! This module is intentionally pure library code: it does not know about
//! OpenHuman runtime types, provider payloads, filesystems, or model calls.

mod boundary;
mod budget;
mod subagent;
mod summary;
mod tool_digest;
mod types;

pub use boundary::{
    PartialConversationSplit, align_tail_start_for_tool_boundaries, latest_real_user_index,
    latest_visible_assistant_index, rejoin_partial_conversation, sanitize_orphan_tool_messages,
    split_partial_conversation,
};
pub use budget::{
    ConversationBudget, HeadProtection, TailBudgetSelection, effective_input_window,
    estimate_message_tokens, protected_head_end, select_tail_by_budget, threshold_tokens,
};
pub use subagent::{
    EventMetadata, EvidencePolicy, EvidenceRef, Finding, OmissionReport, SubagentEvent,
    SubagentEventRole, SubagentSummaryInput, SubagentSummaryOutput,
    format_subagent_summary_markdown, summarize_subagent_transcript,
};
pub use summary::{
    DeterministicSummaryOptions, SUMMARY_END_MARKER, SUMMARY_METADATA_KIND, StructuredSummary,
    SummaryError, SummaryErrorKind, SummaryFailureAction, SummaryFailureDecision,
    SummaryFailurePolicy, SummaryInsertionReport, SummaryProvider, SummaryRequest,
    build_summary_prompt, compaction_summary_message, decide_summary_failure,
    deterministic_fallback_summary, is_compaction_summary_message, normalize_summary_text,
    rehydrate_summary_from_message, render_structured_summary, upsert_compaction_summary,
};
pub use tool_digest::{
    ToolDigestEntry, ToolDigestOptions, ToolDigestReport, digest_old_tool_results,
    redact_sensitive_json, shrink_json_string_leaves,
};
pub use types::{ChatMessage, ConversationMessage, ToolCall, ToolResultMessage};
