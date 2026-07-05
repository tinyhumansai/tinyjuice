use crate::pipeline::BloatEstimate;
use crate::types::{CompressorKind, ContentKind};

/// A transform that ran or was considered by the pipeline.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PipelineStep {
    pub name: &'static str,
    pub compressor: Option<CompressorKind>,
}

/// Redacted reason the pipeline declined to transform content.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PipelineSkipReason {
    RouterDisabled,
    BelowMinBytes,
    NoCompressor,
    NoSavings,
    CcrDisabled,
    CcrNotRetained,
    FooterWouldGrow,
    RecoveryTool,
    Other(&'static str),
}

impl PipelineSkipReason {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::RouterDisabled => "router_disabled",
            Self::BelowMinBytes => "below_min_bytes",
            Self::NoCompressor => "no_compressor",
            Self::NoSavings => "no_savings",
            Self::CcrDisabled => "ccr_disabled",
            Self::CcrNotRetained => "ccr_not_retained",
            Self::FooterWouldGrow => "footer_would_grow",
            Self::RecoveryTool => "recovery_tool",
            Self::Other(reason) => reason,
        }
    }
}

/// Non-sensitive report for pipeline decisions.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PipelineReport {
    pub content_kind: ContentKind,
    pub original_bytes: usize,
    pub compacted_bytes: usize,
    pub bloat_estimate: Option<BloatEstimate>,
    pub applied_steps: Vec<PipelineStep>,
    pub skipped_steps: Vec<PipelineStep>,
    pub ccr_tokens: Vec<String>,
    pub lossy: bool,
    pub skip_reason: Option<PipelineSkipReason>,
}

impl PipelineReport {
    pub fn applied(
        content_kind: ContentKind,
        original_bytes: usize,
        compacted_bytes: usize,
        compressor: CompressorKind,
        lossy: bool,
        ccr_token: Option<String>,
    ) -> Self {
        Self {
            content_kind,
            original_bytes,
            compacted_bytes,
            bloat_estimate: None,
            applied_steps: vec![PipelineStep {
                name: "compat_router",
                compressor: Some(compressor),
            }],
            skipped_steps: Vec::new(),
            ccr_tokens: ccr_token.into_iter().collect(),
            lossy,
            skip_reason: None,
        }
    }

    pub fn passthrough(
        content_kind: ContentKind,
        original_bytes: usize,
        skip_reason: PipelineSkipReason,
    ) -> Self {
        Self {
            content_kind,
            original_bytes,
            compacted_bytes: original_bytes,
            bloat_estimate: None,
            applied_steps: Vec::new(),
            skipped_steps: Vec::new(),
            ccr_tokens: Vec::new(),
            lossy: false,
            skip_reason: Some(skip_reason),
        }
    }

    pub fn with_bloat_estimate(mut self, bloat_estimate: BloatEstimate) -> Self {
        self.bloat_estimate = Some(bloat_estimate);
        self
    }
}
