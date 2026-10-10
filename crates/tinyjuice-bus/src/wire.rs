//! The request and response envelopes that exist only on the wire.
//!
//! [`types`](crate::types) holds values the `tinyjuice` library also uses.
//! These do not exist in it: they are the shapes the interface wraps those
//! values in, and they lived as private structs inside the module adapter,
//! where a host had no way to reach them and re-declared them instead.

use serde::{Deserialize, Serialize};

use crate::types::{AgentTokenjuiceCompression, CompressOptions};

/// The one-shot configuration a host installs before its first compression.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InstallRequest {
    /// Router and compressor knobs, built by the host from its own config.
    pub options: CompressOptions,
    /// Ceiling on how many originals the CCR cache retains.
    pub max_cache_entries: usize,
    /// Ceiling on the bytes those originals may occupy.
    pub max_cache_bytes: usize,
    /// How long a retrievable original stays retrievable. `None` keeps the
    /// module's own default.
    pub ccr_ttl_secs: Option<u64>,
    /// Where the disk tier writes, when the host wants one. `None` keeps the
    /// cache in memory only.
    pub disk_tier_root: Option<String>,
}

/// What `CompactWith` takes: `Compact`'s positional arguments plus the
/// per-call context the summary stage needs.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CompactRequest {
    /// The tool's output.
    pub content: String,
    /// The agent-level tool name.
    pub tool_name: String,
    /// Whether the content router runs. `false` returns `content` untouched
    /// unless the summary stage (gated by `llm_summary_enabled`,
    /// `llm_summary_mode = Auto` and a `context_token`) writes a summary.
    #[serde(default = "enabled_by_default")]
    pub enabled: bool,
    /// The agent's compaction profile.
    #[serde(default = "full_profile")]
    pub profile: AgentTokenjuiceCompression,
    /// The tool call's JSON arguments, for command, extension and query hints.
    #[serde(default)]
    pub arguments: Option<serde_json::Value>,
    /// What the calling model said it needs from this result. Steers the
    /// summary, and ranks text when a deterministic compressor runs instead.
    #[serde(default)]
    pub focus: Option<String>,
    /// Opaque to the module. Passed back unchanged in
    /// [`GenerateRequest::context_token`] so the host can run the summary
    /// under the turn that made the call. `None` means the host has no turn to
    /// offer, and the summary stage is skipped.
    #[serde(default)]
    pub context_token: Option<String>,
    /// Scopes summary reuse, typically the conversation id. Two scopes never
    /// share a cached summary.
    #[serde(default)]
    pub scope: Option<String>,
}

fn enabled_by_default() -> bool {
    true
}

fn full_profile() -> AgentTokenjuiceCompression {
    AgentTokenjuiceCompression::Full
}

/// What the module sends the host's `Generate` member.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GenerateRequest {
    /// The [`CompactRequest::context_token`] this call serves.
    pub context_token: String,
    /// What the call is for, so a host can route or meter it. Currently only
    /// `"tool_output_summary"`.
    pub purpose: String,
    /// The system prompt, written by the module.
    pub system: String,
    /// The single user message.
    pub prompt: String,
    /// Ceiling on the reply's length.
    pub max_output_tokens: u32,
}

/// What `Compact` answers with.
///
/// Distinct from [`CompressedOutput`](crate::types::CompressedOutput) because
/// the compaction path reports token counts the compression path does not, and
/// flattens `content_kind` and `compressor` to their string spellings.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CompactResponse {
    /// The compacted text, with any retrieval footer already appended.
    pub text: String,
    /// Byte length of the input.
    pub original_bytes: usize,
    /// Byte length of `text`.
    pub compacted_bytes: usize,
    /// Which rule fired, or the empty string when none did.
    pub rule_id: String,
    /// Whether anything actually changed.
    pub applied: bool,
    /// The detected content kind, as its wire spelling.
    pub content_kind: String,
    /// The compressor that produced `text`, as its wire spelling.
    pub compressor: String,
    /// Estimated tokens in the input.
    pub original_tokens: u64,
    /// Estimated tokens in `text`.
    pub compacted_tokens: u64,
    /// A model-facing notice the host should prefix after its own caps. Set
    /// when the summary stage applied and could not produce a summary.
    #[serde(default)]
    pub notice: Option<String>,
}

/// What a [`RetrieveRange`] is measured in.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum RangeUnit {
    /// Byte offsets into the stored original.
    Bytes,
    /// Zero-based line numbers.
    Lines,
}

/// A half-open slice of a stored original.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RetrieveRange {
    /// Inclusive start, in [`unit`](Self::unit)s.
    pub start: usize,
    /// Exclusive end, in [`unit`](Self::unit)s.
    pub end: usize,
    /// What `start` and `end` count.
    pub unit: RangeUnit,
}

/// What the module is currently holding.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CacheStats {
    /// Number of retained originals.
    pub entries: usize,
    /// Bytes those originals occupy.
    pub bytes: usize,
}

/// Content to inspect without transferring a cached original back to the host.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum QueryTarget {
    /// Resolve the original from the module's CCR store.
    Handle { token: String },
    /// Inspect artifact content the host has already authorized and read.
    Content { content: String },
}

/// A bounded REPL query executed inside the module.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QueryRequest {
    /// Stored handle or supplied artifact content; never a filesystem path.
    pub target: QueryTarget,
    /// Typed operation, including its optional scope.
    pub op: crate::repl::ReplOp,
    /// Output and pattern compilation caps.
    #[serde(default)]
    pub limits: crate::repl::ReplLimits,
    /// Turn-bound model callback ticket for an on-demand summary.
    #[serde(default)]
    pub context_token: Option<String>,
    /// Scope for summary reuse and failure suppression.
    #[serde(default)]
    pub scope: Option<String>,
}

/// Structured failures of a REPL operation, distinct from transport failures.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(tag = "kind", content = "detail", rename_all = "snake_case")]
pub enum QueryError {
    /// The module no longer has this original.
    HandleNotFound,
    /// Supplied content exceeds the module's fixed input ceiling.
    InputTooLarge,
    /// The operation contains an invalid pattern or slice.
    InvalidPattern(String),
    /// A search query was empty.
    EmptyQuery,
    /// The compiled module lacks this optional operation.
    Unsupported(String),
}

/// A REPL result or an operation error, encoded as a single bus reply.
pub type QueryResponse = Result<crate::repl::ReplOutput, QueryError>;

/// Largest supplied artifact content accepted by `Query` (the host file-read cap).
pub const MAX_QUERY_CONTENT_BYTES: usize = 10 * 1024 * 1024;

/// Largest HTML input accepted by `ExtractHtml` (the web extractor input cap).
pub const MAX_HTML_INPUT_BYTES: usize = 8 * 1024 * 1024;

/// Structured HTML extraction failures, separate from transport errors.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum HtmlError {
    /// The supplied input exceeds the fixed parser ceiling.
    InputTooLarge,
}

/// Markdown or a structured extraction error.
pub type HtmlResponse = Result<String, HtmlError>;
