//! The values the `TinyJuice` compression interface carries.
//!
//! Every type here appears in a request or a response on
//! [`BUS_NAME`](crate::names::BUS_NAME). They are moved out of the `tinyjuice`
//! library, not copied from it: the library re-exports them from this crate, so
//! there is one definition and a host linking only this crate is looking at the
//! same bytes the module validates against.

use serde::{Deserialize, Serialize};

/// Per-agent TokenJuice profile.
///
/// `Auto` is resolved by the agent definition layer. TokenJuice itself treats
/// `Auto` like `Full` so non-agent callers keep the global `[tinyjuice]`
/// behaviour unless they explicitly pass a narrower profile.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum AgentTokenjuiceCompression {
    /// Let the agent definition/runtime choose. Coding agents resolve this to
    /// [`Self::Light`]; other agents resolve to [`Self::Full`].
    #[default]
    Auto,
    /// Use the process-global TokenJuice configuration unchanged.
    Full,
    /// Keep only non-lossy reductions; disables CCR-backed lossy compaction.
    Light,
    /// Bypass TokenJuice for this agent's tool results.
    Off,
}

impl AgentTokenjuiceCompression {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Auto => "auto",
            Self::Full => "full",
            Self::Light => "light",
            Self::Off => "off",
        }
    }
}

// ---------------------------------------------------------------------------
// Content Router (TokenJuice 2.0) — content-kind detection + compressor dispatch
// ---------------------------------------------------------------------------

/// The kind of content a blob holds, as decided by the detector. Drives which
/// the compressor the router picks.
///
/// Inspired by Headroom's content router: each kind has a specialised
/// compressor tuned to preserve the signal that kind carries (errors in logs,
/// changed hunks in diffs, signatures in code, …).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ContentKind {
    /// JSON array/object payload → tabular SmartCrusher.
    Json,
    /// Source code → AST/heuristic signature keeper.
    Code,
    /// Build / test / lint log → keep failures, drop passing noise.
    Log,
    /// grep / ripgrep style `path:line:content` matches → relevance rank.
    Search,
    /// Unified git diff / patch → keep changed hunks, collapse context.
    Diff,
    /// HTML document → strip markup to readable text.
    Html,
    /// Anything else → ML text compressor (if enabled) or pass-through.
    PlainText,
}

impl ContentKind {
    /// Stable lower-case label for logs / RPC / stats.
    pub fn as_str(self) -> &'static str {
        match self {
            ContentKind::Json => "json",
            ContentKind::Code => "code",
            ContentKind::Log => "log",
            ContentKind::Search => "search",
            ContentKind::Diff => "diff",
            ContentKind::Html => "html",
            ContentKind::PlainText => "plain_text",
        }
    }
}

impl std::str::FromStr for ContentKind {
    type Err = ();

    /// The exact inverse of [`as_str`](Self::as_str).
    ///
    /// This is contract surface rather than a convenience: `Compact` answers
    /// with [`CompactResponse`](crate::wire::CompactResponse), which flattens
    /// the kind to its `as_str` spelling, so a caller that wants the enum back
    /// has to parse that string. Without the inverse living beside the forward
    /// direction, every host writes the table itself and one of them gets a
    /// spelling wrong — `plain_text` here is not the `plainText` that serde
    /// produces, and nothing but this pairing says so.
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Ok(match value {
            "json" => Self::Json,
            "code" => Self::Code,
            "log" => Self::Log,
            "search" => Self::Search,
            "diff" => Self::Diff,
            "html" => Self::Html,
            "plain_text" => Self::PlainText,
            _ => return Err(()),
        })
    }
}

/// A caller-supplied prior about a blob's content, so the detector doesn't have
/// to work from scratch. Any field may be `None`; the detector resolves what it
/// can and falls back to structural heuristics. An `explicit` kind is a hard
/// override and skips detection entirely.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContentHint {
    /// MIME type if known (`text/html`, `application/json`, …).
    pub mime: Option<String>,
    /// File extension without the dot (`rs`, `ts`, `py`, `json`, `html`, `diff`).
    pub extension: Option<String>,
    /// The agent-level tool that produced the content (`grep`, `run_tests`, …).
    pub source_tool: Option<String>,
    /// A search/query string associated with the content, when known (used by
    /// the search compressor to rank matches by query-term density).
    pub query: Option<String>,
    /// Hard override — when set, detection returns this kind verbatim.
    pub explicit: Option<ContentKind>,
    /// Whether file-read content must remain exact or may be reduced to a code stub.
    #[serde(default)]
    pub read_intent: ReadIntent,
}

/// Caller intent for read-like sources.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum ReadIntent {
    #[default]
    Exact,
    Stub(StubMode),
}

/// Source-code stub mode.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", tag = "kind", content = "value")]
pub enum StubMode {
    #[default]
    SignaturesOnly,
    PublicApi,
    MatchedSymbols(Vec<String>),
    ExpandAroundLines(Vec<LineRange>),
}

/// One-based inclusive line range.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct LineRange {
    pub start: usize,
    pub end: usize,
}
impl LineRange {
    pub fn new(start: usize, end: usize) -> Self {
        Self {
            start: start.max(1),
            end: end.max(start.max(1)),
        }
    }
    pub fn intersects(self, other: Self) -> bool {
        self.start <= other.end && other.start <= self.end
    }
}

/// Parser path used for a source-code stub.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ParseStatus {
    TreeSitter,
    HeuristicFallback,
}

/// Symbol surfaced in a source-code stub.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SymbolSummary {
    pub name: String,
    pub kind: String,
    pub start_line: usize,
    pub end_line: usize,
    pub public: bool,
}

/// Source range omitted from a stub.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CodeElision {
    pub start_line: usize,
    pub end_line: usize,
    pub reason: String,
}

/// Structured source-code stub result.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CodeStubOutput {
    pub text: String,
    pub symbols: Vec<SymbolSummary>,
    pub elisions: Vec<CodeElision>,
    pub parse_status: ParseStatus,
}

/// Format of already-extracted web content handed to the web reducer.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WebExtractFormat {
    #[default]
    Markdown,
    Text,
    Html,
}
impl WebExtractFormat {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Markdown => "markdown",
            Self::Text => "text",
            Self::Html => "html",
        }
    }
}

/// One already-extracted web page to reduce.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WebExtractReduceInput {
    pub url: String,
    #[serde(default)]
    pub title: Option<String>,
    pub content: String,
    #[serde(default)]
    pub format: WebExtractFormat,
    #[serde(default)]
    pub provider: Option<String>,
    #[serde(default)]
    pub char_limit: Option<usize>,
    #[serde(default)]
    pub metadata: serde_json::Map<String, serde_json::Value>,
}

/// Batch shape for multi-URL extractors.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WebExtractBatchInput {
    pub pages: Vec<WebExtractReduceInput>,
    #[serde(default)]
    pub default_char_limit: Option<usize>,
    #[serde(default)]
    pub max_combined_inline_chars: Option<usize>,
}

/// Web extraction truncation knobs.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct WebExtractOptions {
    pub char_limit: usize,
    pub min_char_limit: usize,
    pub max_char_limit: usize,
    pub head_ratio: f32,
    pub convert_base64_images: bool,
    pub max_combined_inline_chars: usize,
}
impl Default for WebExtractOptions {
    fn default() -> Self {
        Self {
            char_limit: 15_000,
            min_char_limit: 2_000,
            max_char_limit: 500_000,
            head_ratio: 0.75,
            convert_base64_images: true,
            max_combined_inline_chars: 100_000,
        }
    }
}

/// Metadata-only reduction report for a web page.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WebExtractReduction {
    pub text: String,
    pub body: String,
    #[serde(default)]
    pub recovery_footer: Option<String>,
    #[serde(default)]
    pub ccr_token: Option<String>,
    pub source_host: Option<String>,
    pub source_url_hash: String,
    #[serde(default)]
    pub title: Option<String>,
    pub format: WebExtractFormat,
    pub original_chars: usize,
    pub inline_chars: usize,
    pub head_chars: usize,
    pub tail_chars: usize,
    pub omitted_chars: usize,
    pub truncated: bool,
    pub full_text_retained: bool,
    pub base64_images_replaced: usize,
}

impl ContentHint {
    /// Convenience: a hint carrying only the producing tool name.
    pub fn for_tool(tool_name: impl Into<String>) -> Self {
        Self {
            source_tool: Some(tool_name.into()),
            ..Default::default()
        }
    }
}

/// Which compressor actually produced an output. Recorded in stats / logs so a
/// human (or the debug controller) can see what the router chose.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum CompressorKind {
    /// JSON array→table crusher.
    SmartCrusher,
    /// AST/heuristic code-signature keeper.
    Code,
    /// Log keep-failures compressor (and the rule engine for command output).
    Log,
    /// Search relevance ranker.
    Search,
    /// Unified-diff context collapser.
    Diff,
    /// HTML→text extractor.
    Html,
    /// ML (Python/ModernBERT) plain-text compressor.
    MlText,
    /// Deterministic extractive plain-text compressor.
    TextCrusher,
    /// Line-oriented head/tail fallback.
    Generic,
    /// No compressor fired — pass-through.
    None,
}

impl CompressorKind {
    /// Stable lower-case label for stats / logs / RPC.
    pub fn as_str(self) -> &'static str {
        match self {
            CompressorKind::SmartCrusher => "smartcrusher",
            CompressorKind::Code => "code",
            CompressorKind::Log => "log",
            CompressorKind::Search => "search",
            CompressorKind::Diff => "diff",
            CompressorKind::Html => "html",
            CompressorKind::MlText => "ml_text",
            CompressorKind::TextCrusher => "textcrusher",
            CompressorKind::Generic => "generic",
            CompressorKind::None => "none",
        }
    }
}

impl std::str::FromStr for CompressorKind {
    type Err = ();

    /// The exact inverse of [`as_str`](Self::as_str), for the same reason
    /// [`ContentKind`]'s exists: `Compact` reports the compressor as that
    /// spelling, not as the `smartCrusher` serde emits.
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Ok(match value {
            "smartcrusher" => Self::SmartCrusher,
            "code" => Self::Code,
            "log" => Self::Log,
            "search" => Self::Search,
            "diff" => Self::Diff,
            "html" => Self::Html,
            "ml_text" => Self::MlText,
            "generic" => Self::Generic,
            "none" => Self::None,
            _ => return Err(()),
        })
    }
}

/// Knobs for the router and compressors, built by the caller from the
/// `[tinyjuice]` config block. TokenJuice stays decoupled from the config
/// schema crate by taking this plain struct rather than `Config`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct CompressOptions {
    /// Master switch — when false, `tinyjuice::compress_content`
    /// is a pass-through.
    pub router_enabled: bool,
    /// Whether to offload originals to CCR and emit retrieval markers.
    pub ccr_enabled: bool,
    /// Per-compressor toggles.
    pub search_enabled: bool,
    pub code_enabled: bool,
    pub html_enabled: bool,
    /// Whether the ML plain-text compressor may be used (further gated at
    /// runtime by Python/runtime_python_server availability).
    pub ml_text_enabled: bool,
    /// Outputs below this many bytes are never compressed.
    pub min_bytes_to_compress: usize,
    /// Lower size floor for log-like content (detected `Log` kind or command
    /// output routed through the rule engine). Test/build failure logs are
    /// often only ~1–2 KB yet compress extremely well (a Vitest failure rule
    /// reaches ~79% on sub-2 KB fixtures), so gating them behind the global
    /// `min_bytes_to_compress` floor leaves real savings on the table. Content
    /// in `[min_bytes_to_compress_log, min_bytes_to_compress)` is detected and
    /// compressed only when it is log-like; every other kind keeps the global
    /// floor.
    pub min_bytes_to_compress_log: usize,
    /// CCR only fires (offload original + lossy compression) when the input is
    /// estimated to be at least this many tokens. Below it, the result passes
    /// through (lossless reformats may still apply without offload). Lets small
    /// tool results skip the cache entirely.
    ///
    /// This is the primary knob, but it is ratio-aware rather than a hard
    /// cliff: when a compression is heavily lossy (the compacted text is at
    /// most half the original tokens) CCR also fires for inputs down to a
    /// quarter of this threshold. A heavy crush on a small input drops a large
    /// *fraction* of the content, so recoverability matters there even though
    /// the absolute token count is modest — while trivially small inputs
    /// (below a quarter of the threshold) still skip the cache entirely.
    pub ccr_min_tokens: usize,
    /// Allow *information-dropping* compression when CCR is not in play
    /// (disabled, below `ccr_min_tokens`, or the original couldn't be
    /// retained). Faithful reformats (JSON tables/minify, HTML→text) are
    /// information-preserving and always ship regardless of this flag; it only
    /// governs compressors that drop content (logs, diffs, search, code
    /// bodies, sampled JSON rows). Default `false`: without a recovery token
    /// those pass through untouched rather than emit a partial view the caller
    /// can't get back. Set `true` to allow marked-but-unrecoverable lossy
    /// output (dropped content still carries explicit `[... omitted ...]`
    /// markers, it just isn't retrievable).
    pub lossy_without_ccr: bool,
    /// Maximum inline character count for the generic/rule fallback path.
    pub max_inline_chars: Option<usize>,
    /// Target output/input byte ratio for source-code compression. When set
    /// (e.g. `Some(0.4)`), the code compressor collapses eligible bodies
    /// largest-first and stops once the projected output is at or below this
    /// fraction of the input, leaving the remaining bodies fully intact.
    /// `None` (the default) collapses every eligible body.
    pub code_target_ratio: Option<f32>,
    /// Average characters per token used by the router's token estimates
    /// (gating and savings accounting). The default 4.0 matches the standard
    /// English-text heuristic; callers whose payloads skew denser (CJK text,
    /// minified JSON) or sparser can calibrate. With the default value the
    /// historical ceiling-division estimate is kept bit-for-bit; a custom
    /// value uses round-half-up.
    pub chars_per_token: f32,
}

impl Default for CompressOptions {
    fn default() -> Self {
        Self {
            router_enabled: true,
            ccr_enabled: true,
            search_enabled: true,
            code_enabled: true,
            html_enabled: true,
            ml_text_enabled: false,
            min_bytes_to_compress: 2048,
            min_bytes_to_compress_log: 512,
            ccr_min_tokens: 500,
            // Without CCR, only information-preserving output ships: faithful
            // reformats (JSON tables/minify, HTML→text) still apply, but any
            // compressor that *drops* information (logs, diffs, search, code
            // bodies, sampled JSON rows) passes through untouched rather than
            // emitting an unrecoverable partial view. A host can opt back into
            // marked-but-unrecoverable lossy output by flipping this to true.
            lossy_without_ccr: false,
            max_inline_chars: None,
            code_target_ratio: None,
            chars_per_token: 4.0,
        }
    }
}

/// The result of the universal `tinyjuice::compress_content`
/// entry point: the compacted text (with any CCR footer already appended), plus
/// metadata for callers/stats.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CompressedOutput {
    /// Final text to inline into context (includes the retrieval footer when lossy).
    pub text: String,
    /// Compacted body without the recovery footer.
    #[serde(default)]
    pub body: String,
    /// Recovery footer to append after host-side truncation, if any.
    #[serde(default)]
    pub recovery_footer: Option<String>,
    /// The detected content kind.
    pub content_kind: ContentKind,
    /// Which compressor fired (`None` ⇒ pass-through).
    pub compressor: CompressorKind,
    /// Whether the output dropped data.
    pub lossy: bool,
    /// True if the router actually changed the content.
    pub applied: bool,
    /// CCR token for the offloaded original, if one was stored.
    pub ccr_token: Option<String>,
    /// Original byte length.
    pub original_bytes: usize,
    /// Compacted byte length (of `text`).
    pub compacted_bytes: usize,
}

impl CompressedOutput {
    /// Build a pass-through result that didn't change `content`.
    pub fn passthrough(content: String, kind: ContentKind) -> Self {
        let len = content.len();
        Self {
            body: content.clone(),
            text: content,
            recovery_footer: None,
            content_kind: kind,
            compressor: CompressorKind::None,
            lossy: false,
            applied: false,
            ccr_token: None,
            original_bytes: len,
            compacted_bytes: len,
        }
    }
}
