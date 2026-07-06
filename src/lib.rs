//! Pluggable token compression for OpenHuman and other Rust hosts.
//!
//! TinyJuice owns the reusable TokenJuice compression engine. Hosts provide
//! configuration mapping, optional ML callbacks, savings attribution, and their
//! own RPC/tool surfaces.

pub mod cache;
pub mod cache_hints;
pub mod classify;
pub mod compress;
pub mod compressor;
pub mod compressors;
pub mod config;
pub mod conversation;
pub mod detect;
mod error;
pub mod ml;
pub mod observability;
pub mod openhuman;
pub mod pipeline;
pub mod policy;
pub mod reduce;
pub mod relevance;
pub mod rules;
pub mod savings;
pub mod text;
pub mod tokens;
pub mod tool_integration;
pub mod types;
mod util;

pub use cache_hints::{
    CacheMarkerPlacement, PromptCacheHint, PromptCacheTtl, StaticPrefixCacheHint,
    ToolSchemaForCache, anthropic_cache_hints, stable_prefix_cache_key, static_prefix_cache_hint,
};
pub use compress::{
    compress_content, compress_content_with_store, compress_content_with_store_report, route,
    route_with_shell_policy, route_with_store, route_with_store_report,
    route_with_store_report_shell_policy, route_with_store_shell_policy,
};
pub use compressor::{
    CompressionInput, CompressionOutput, CompressionReport, Compressor, PassthroughCompressor,
};
pub use compressors::code::stub_code;
pub use compressors::web_extract::{
    reduce_web_extract, reduce_web_extract_batch_with_store, reduce_web_extract_with_store,
    replace_inline_base64_images,
};
pub use compressors::{compressor_for, generic_compressor};
pub use config::CompressionConfig;
pub use conversation::{
    ChatMessage, ConversationBudget, ConversationMessage, DeterministicSummaryOptions,
    EventMetadata, EvidencePolicy, EvidenceRef, Finding, HeadProtection, OmissionReport,
    PartialConversationSplit, SUMMARY_END_MARKER, SUMMARY_METADATA_KIND, StructuredSummary,
    SubagentEvent, SubagentEventRole, SubagentSummaryInput, SubagentSummaryOutput, SummaryError,
    SummaryErrorKind, SummaryFailureAction, SummaryFailureDecision, SummaryFailurePolicy,
    SummaryInsertionReport, SummaryProvider, SummaryRequest, TailBudgetSelection, ToolCall,
    ToolDigestEntry, ToolDigestOptions, ToolDigestReport, ToolResultMessage,
    align_tail_start_for_tool_boundaries, build_summary_prompt, compaction_summary_message,
    decide_summary_failure, deterministic_fallback_summary, digest_old_tool_results,
    effective_input_window, estimate_message_tokens, format_subagent_summary_markdown,
    is_compaction_summary_message, latest_real_user_index, latest_visible_assistant_index,
    normalize_summary_text, protected_head_end, redact_sensitive_json,
    rehydrate_summary_from_message, rejoin_partial_conversation, render_structured_summary,
    sanitize_orphan_tool_messages, select_tail_by_budget, shrink_json_string_leaves,
    split_for_last_user_exchanges, split_partial_conversation, summarize_subagent_transcript,
    tail_start_for_last_user_exchanges, threshold_tokens, upsert_compaction_summary,
};
pub use detect::detect_content_kind;
pub use error::{TinyJuiceError, TinyJuiceResult};
pub use observability::{ContextBreakdown, ContextBucket, ContextBucketKind};
pub use pipeline::{
    OffloadOutput, OffloadTransform, PipelineInput, PipelineReport, PipelineSkipReason,
    PipelineStep, ReformatTransform, TransformOutput, TypedPipelineOutput, run_typed_pipeline,
};
pub use policy::{ShellCompactionPolicy, ShellPolicyDecision, apply_shell_compaction_policy};
pub use reduce::reduce_execution_with_rules;
pub use relevance::{Bm25Corpus, Bm25DocumentScore};
pub use rules::{LoadRuleOptions, load_builtin_rules, load_rules};
pub use tool_integration::{
    CompactionStats, compact_output, compact_output_with_policy, compact_tool_output_with_policy,
    configure, current_options, install_config,
};
pub use types::{
    AgentTokenjuiceCompression, CodeElision, CodeStubOutput, CompactResult, CompressInput,
    CompressOptions, CompressOutput, CompressedOutput, CompressorKind, ContentHint, ContentKind,
    LineRange, ParseStatus, ReadIntent, ReduceOptions, StubMode, SymbolSummary, ToolExecutionInput,
    WebExtractBatchInput, WebExtractFormat, WebExtractOptions, WebExtractReduceInput,
    WebExtractReduction,
};

#[cfg(test)]
#[path = "text_tests.rs"]
mod text_tests;
