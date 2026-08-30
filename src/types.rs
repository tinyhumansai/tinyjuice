//! Core type definitions for the TokenJuice reduction engine.
//!
//! These types mirror the upstream TypeScript shapes so that upstream rule JSON
//! files can be loaded without modification.  All public types use
//! `#[serde(rename_all = "camelCase")]` and `#[serde(default)]` on optional
//! fields for maximum compatibility with the upstream schema.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ---------------------------------------------------------------------------
// Rule origin
// ---------------------------------------------------------------------------

/// Which configuration layer a rule was loaded from.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum RuleOrigin {
    Builtin,
    User,
    Project,
}

// ---------------------------------------------------------------------------
// Rule sub-types
// ---------------------------------------------------------------------------

/// Matching criteria for a rule.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RuleMatch {
    /// Match when `toolName` is one of these values.
    #[serde(default)]
    pub tool_names: Option<Vec<String>>,
    /// Match when `argv[0]` is one of these values.
    #[serde(default)]
    pub argv0: Option<Vec<String>>,
    /// All of these groups must each appear somewhere in `argv`.
    #[serde(default)]
    pub argv_includes: Option<Vec<Vec<String>>>,
    /// At least one of these groups must appear in `argv`.
    #[serde(default)]
    pub argv_includes_any: Option<Vec<Vec<String>>>,
    /// All of these strings must appear in `command`.
    #[serde(default)]
    pub command_includes: Option<Vec<String>>,
    /// At least one of these strings must appear in `command`.
    #[serde(default)]
    pub command_includes_any: Option<Vec<String>>,
}

/// Line-level filter patterns.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RuleFilters {
    /// Lines matching any pattern are removed.
    #[serde(default)]
    pub skip_patterns: Option<Vec<String>>,
    /// Only lines matching at least one pattern are kept (if any match).
    #[serde(default)]
    pub keep_patterns: Option<Vec<String>>,
}

/// Output transformation flags.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RuleTransforms {
    #[serde(default)]
    pub strip_ansi: Option<bool>,
    #[serde(default)]
    pub trim_empty_edges: Option<bool>,
    #[serde(default)]
    pub dedupe_adjacent: Option<bool>,
    #[serde(default)]
    pub pretty_print_json: Option<bool>,
}

/// Head/tail summarisation parameters.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RuleSummarize {
    #[serde(default)]
    pub head: Option<usize>,
    #[serde(default)]
    pub tail: Option<usize>,
}

/// A pattern-based line counter.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RuleCounter {
    pub name: String,
    pub pattern: String,
    /// Regex flags (e.g. `"i"` for case-insensitive). `u` is always added.
    #[serde(default)]
    pub flags: Option<String>,
}

/// Map output patterns to canned messages.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RuleOutputMatch {
    pub pattern: String,
    pub message: String,
    #[serde(default)]
    pub flags: Option<String>,
}

/// Failure-mode overrides.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RuleFailure {
    #[serde(default)]
    pub preserve_on_failure: Option<bool>,
    #[serde(default)]
    pub head: Option<usize>,
    #[serde(default)]
    pub tail: Option<usize>,
}

// ---------------------------------------------------------------------------
// JsonRule — the raw deserialized form
// ---------------------------------------------------------------------------

/// A rule as parsed from a JSON file (upstream `JsonRule`).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JsonRule {
    pub id: String,
    pub family: String,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub priority: Option<i32>,
    /// Message to return when output is empty after filtering.
    #[serde(default)]
    pub on_empty: Option<String>,
    #[serde(default)]
    pub match_output: Option<Vec<RuleOutputMatch>>,
    /// Whether counters run before or after keep-pattern filtering.
    /// Upstream default is `"postKeep"`.
    #[serde(default)]
    pub counter_source: Option<CounterSource>,
    pub r#match: RuleMatch,
    #[serde(default)]
    pub filters: Option<RuleFilters>,
    #[serde(default)]
    pub transforms: Option<RuleTransforms>,
    #[serde(default)]
    pub summarize: Option<RuleSummarize>,
    #[serde(default)]
    pub counters: Option<Vec<RuleCounter>>,
    #[serde(default)]
    pub failure: Option<RuleFailure>,
}

/// When to sample lines for counters — before or after keep-pattern filtering.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum CounterSource {
    PostKeep,
    PreKeep,
}

// ---------------------------------------------------------------------------
// CompiledRule — regex patterns pre-built
// ---------------------------------------------------------------------------

/// A compiled counter entry with the pattern pre-built.
#[derive(Debug, Clone)]
pub struct CompiledCounter {
    pub name: String,
    pub pattern: regex::Regex,
}

/// A compiled output-match entry.
#[derive(Debug, Clone)]
pub struct CompiledOutputMatch {
    pub pattern: regex::Regex,
    pub message: String,
}

/// The compiled form of a rule (regex patterns pre-built at load time).
#[derive(Debug, Clone)]
pub struct CompiledParts {
    pub skip_patterns: Vec<regex::Regex>,
    pub keep_patterns: Vec<regex::Regex>,
    pub counters: Vec<CompiledCounter>,
    pub output_matches: Vec<CompiledOutputMatch>,
}

/// A `JsonRule` paired with its pre-compiled regex patterns plus provenance.
#[derive(Debug, Clone)]
pub struct CompiledRule {
    pub rule: JsonRule,
    pub source: RuleOrigin,
    /// Filesystem path (or `"builtin:<id>"` for embedded rules).
    pub path: String,
    pub compiled: CompiledParts,
}

// ---------------------------------------------------------------------------
// ToolExecutionInput
// ---------------------------------------------------------------------------

/// Describes a tool invocation whose output is to be reduced.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ToolExecutionInput {
    pub tool_name: String,
    #[serde(default)]
    pub tool_call_id: Option<String>,
    #[serde(default)]
    pub run_id: Option<String>,
    #[serde(default)]
    pub command: Option<String>,
    #[serde(default)]
    pub argv: Option<Vec<String>>,
    #[serde(default)]
    pub args: Option<HashMap<String, serde_json::Value>>,
    #[serde(default)]
    pub cwd: Option<String>,
    #[serde(default)]
    pub partial: Option<bool>,
    #[serde(default)]
    pub stdout: Option<String>,
    #[serde(default)]
    pub stderr: Option<String>,
    #[serde(default)]
    pub combined_text: Option<String>,
    #[serde(default)]
    pub exit_code: Option<i32>,
    #[serde(default)]
    pub started_at: Option<f64>,
    #[serde(default)]
    pub finished_at: Option<f64>,
    #[serde(default)]
    pub duration_ms: Option<f64>,
    #[serde(default)]
    pub metadata: Option<HashMap<String, serde_json::Value>>,
}

// ---------------------------------------------------------------------------
// ReduceOptions
// ---------------------------------------------------------------------------

/// Options for the `reduce_execution` pipeline.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReduceOptions {
    /// Force a specific rule ID instead of auto-classification.
    #[serde(default)]
    pub classifier: Option<String>,
    /// Maximum inline character count (default: 1200).
    #[serde(default)]
    pub max_inline_chars: Option<usize>,
    /// Return raw text without reduction.
    #[serde(default)]
    pub raw: Option<bool>,
    /// Working directory for project-layer rule discovery.
    #[serde(default)]
    pub cwd: Option<String>,
    /// Protocol compatibility flag for callers that want no lossy omission.
    #[serde(default)]
    pub no_omit: Option<bool>,
    /// Request artifact/CCR storage when a protocol surface supports it.
    #[serde(default)]
    pub store: Option<bool>,
    /// Storage directory for protocol surfaces that support artifacts.
    #[serde(default)]
    pub store_dir: Option<String>,
    /// Request metadata-only reducer trace fields in protocol responses.
    #[serde(default)]
    pub trace: Option<bool>,
    /// Request metadata-only stats recording in protocol adapters.
    #[serde(default)]
    pub record_stats: Option<bool>,
}

// ---------------------------------------------------------------------------
// CompactResult
// ---------------------------------------------------------------------------

/// Statistics produced by the reduction pipeline.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReductionStats {
    pub raw_chars: usize,
    pub reduced_chars: usize,
    pub ratio: f64,
}

/// The classification decision made during reduction.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClassificationResult {
    pub family: String,
    pub confidence: f64,
    #[serde(default)]
    pub matched_reducer: Option<String>,
}

/// The output of `reduce_execution`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CompactResult {
    /// The compacted text to inline into LLM context.
    pub inline_text: String,
    /// A shorter preview (the intermediate summary before clamping).
    #[serde(default)]
    pub preview_text: Option<String>,
    /// Named counts extracted by counters.
    #[serde(default)]
    pub facts: Option<HashMap<String, usize>>,
    pub stats: ReductionStats,
    pub classification: ClassificationResult,
    /// CCR token for the original raw output when the store retained it.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ccr_token: Option<String>,
}

/// The values the compression interface carries, defined once in
/// [`tinyjuice_bus`] and re-exported here.
///
/// They used to be declared in this file. That made them the *library's*
/// types, which a host calling the loadable module could not reach — so
/// OpenHuman carried a 259-line hand-copy of them, and a field added on one
/// side was a decode failure on the other with nothing to catch it. Moving
/// them into the contract crate leaves one definition; this re-export is what
/// keeps every existing `tinyjuice::types::…` path resolving.
pub use tinyjuice_bus::types::{
    AgentTokenjuiceCompression, CodeElision, CodeStubOutput, CompressOptions, CompressedOutput,
    CompressorKind, ContentHint, ContentKind, LineRange, ParseStatus, ReadIntent, StubMode,
    SymbolSummary, WebExtractBatchInput, WebExtractFormat, WebExtractOptions,
    WebExtractReduceInput, WebExtractReduction,
};

/// Input handed to a [`crate::compressors::Compressor`].
/// Borrows the content to avoid copies on the hot path.
#[derive(Debug, Clone)]
pub struct CompressInput<'a> {
    /// The raw content to compress.
    pub content: &'a str,
    /// The detected (or hinted) content kind.
    pub kind: ContentKind,
    /// The original caller hint (carries `query`, `source_tool`, argv-derived
    /// command for the log/command path, …).
    pub hint: &'a ContentHint,
    /// Process exit code if this is command output — enables failure-preserving
    /// behaviour in the log compressor.
    pub exit_code: Option<i32>,
    /// Derived shell command (joined argv) for the log/command rule path, if any.
    pub command: Option<String>,
    /// Derived argv for the log/command rule path, if any.
    pub argv: Option<Vec<String>>,
    /// Original byte length (== `content.len()`; cached for convenience).
    pub original_bytes: usize,
}

/// Output of a compressor. `text` is the compacted body **without** any CCR
/// retrieval footer — the router ([`crate::compress`])
/// adds the marker after offloading the original.
#[derive(Debug, Clone)]
pub struct CompressOutput {
    /// The compacted body.
    pub text: String,
    /// True when data was dropped (vs. a faithful reformat). Changes the footer
    /// wording and whether the original is mandatory for fidelity.
    pub lossy: bool,
    /// Which compressor produced this.
    pub kind: CompressorKind,
    /// Optional named counts (e.g. error/warning tallies).
    pub facts: Option<HashMap<String, usize>>,
}

impl CompressOutput {
    /// A faithful reformat — every value preserved, only layout changed.
    pub fn reformatted(text: String, kind: CompressorKind) -> Self {
        Self {
            text,
            lossy: false,
            kind,
            facts: None,
        }
    }

    /// A lossy view — data was dropped; the original must be offloaded for recovery.
    pub fn lossy(text: String, kind: CompressorKind) -> Self {
        Self {
            text,
            lossy: true,
            kind,
            facts: None,
        }
    }
}

// ---------------------------------------------------------------------------
// RuleFixture — used by integration tests
// ---------------------------------------------------------------------------

/// A test fixture mirroring the upstream `RuleFixture` shape.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RuleFixture {
    pub input: ToolExecutionInput,
    pub expected_output: String,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub options: Option<ReduceOptions>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn agent_profile_labels_are_stable() {
        assert_eq!(AgentTokenjuiceCompression::Auto.as_str(), "auto");
        assert_eq!(AgentTokenjuiceCompression::Full.as_str(), "full");
        assert_eq!(AgentTokenjuiceCompression::Light.as_str(), "light");
        assert_eq!(AgentTokenjuiceCompression::Off.as_str(), "off");
    }

    #[test]
    fn content_kind_labels_are_stable() {
        let labels = [
            (ContentKind::Json, "json"),
            (ContentKind::Code, "code"),
            (ContentKind::Log, "log"),
            (ContentKind::Search, "search"),
            (ContentKind::Diff, "diff"),
            (ContentKind::Html, "html"),
            (ContentKind::PlainText, "plain_text"),
        ];

        for (kind, label) in labels {
            assert_eq!(kind.as_str(), label);
        }
    }

    #[test]
    fn compressor_kind_labels_are_stable() {
        let labels = [
            (CompressorKind::SmartCrusher, "smartcrusher"),
            (CompressorKind::Code, "code"),
            (CompressorKind::Log, "log"),
            (CompressorKind::Search, "search"),
            (CompressorKind::Diff, "diff"),
            (CompressorKind::Html, "html"),
            (CompressorKind::MlText, "ml_text"),
            (CompressorKind::Generic, "generic"),
            (CompressorKind::None, "none"),
        ];

        for (kind, label) in labels {
            assert_eq!(kind.as_str(), label);
        }
    }

    #[test]
    fn content_hint_for_tool_sets_only_source_tool() {
        let hint = ContentHint::for_tool("shell");

        assert_eq!(hint.source_tool.as_deref(), Some("shell"));
        assert!(hint.mime.is_none());
        assert!(hint.extension.is_none());
        assert!(hint.query.is_none());
        assert!(hint.explicit.is_none());
    }

    #[test]
    fn compress_output_constructors_set_lossiness_and_facts() {
        let reformatted = CompressOutput::reformatted("{}".into(), CompressorKind::SmartCrusher);
        assert!(!reformatted.lossy);
        assert_eq!(reformatted.kind, CompressorKind::SmartCrusher);
        assert!(reformatted.facts.is_none());

        let lossy = CompressOutput::lossy("partial".into(), CompressorKind::Log);
        assert!(lossy.lossy);
        assert_eq!(lossy.kind, CompressorKind::Log);
        assert!(lossy.facts.is_none());
    }

    #[test]
    fn compressed_output_passthrough_preserves_lengths_and_flags() {
        let output = CompressedOutput::passthrough("alpha".into(), ContentKind::PlainText);

        assert_eq!(output.text, "alpha");
        assert_eq!(output.content_kind, ContentKind::PlainText);
        assert_eq!(output.compressor, CompressorKind::None);
        assert!(!output.lossy);
        assert!(!output.applied);
        assert!(output.ccr_token.is_none());
        assert_eq!(output.original_bytes, 5);
        assert_eq!(output.compacted_bytes, 5);
    }
}
