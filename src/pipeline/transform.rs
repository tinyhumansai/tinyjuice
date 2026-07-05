use crate::cache::{CcrPutResult, CcrStore};
use crate::types::{CompressInput, CompressorKind, ContentKind};
use std::collections::HashMap;

/// Input passed to typed transforms.
#[derive(Debug, Clone, Copy)]
pub struct PipelineInput<'a> {
    pub content: &'a str,
    pub content_kind: ContentKind,
    pub original_bytes: usize,
}

impl<'a> From<&CompressInput<'a>> for PipelineInput<'a> {
    fn from(input: &CompressInput<'a>) -> Self {
        Self {
            content: input.content,
            content_kind: input.kind,
            original_bytes: input.original_bytes,
        }
    }
}

/// Lossless transform output.
#[derive(Debug, Clone)]
pub struct TransformOutput {
    pub text: String,
    pub kind: CompressorKind,
    pub facts: Option<HashMap<String, usize>>,
}

impl TransformOutput {
    pub fn new(text: String, kind: CompressorKind) -> Self {
        Self {
            text,
            kind,
            facts: None,
        }
    }
}

/// Lossy transform output with a verified retained CCR token.
#[derive(Debug, Clone)]
pub struct OffloadOutput {
    text: String,
    kind: CompressorKind,
    token: String,
    facts: Option<HashMap<String, usize>>,
}

impl OffloadOutput {
    /// Construct an offload output only from a retained CCR put result.
    pub fn from_retained_put(
        text: String,
        kind: CompressorKind,
        put: CcrPutResult,
    ) -> Option<Self> {
        put.retained().then(|| Self {
            text,
            kind,
            token: put.token().to_string(),
            facts: None,
        })
    }

    pub fn text(&self) -> &str {
        &self.text
    }

    pub fn kind(&self) -> CompressorKind {
        self.kind
    }

    pub fn token(&self) -> &str {
        &self.token
    }

    pub fn facts(&self) -> Option<&HashMap<String, usize>> {
        self.facts.as_ref()
    }
}

pub trait ReformatTransform {
    fn name(&self) -> &'static str;
    fn applies_to(&self, input: &PipelineInput<'_>) -> bool;
    fn apply(&self, input: &PipelineInput<'_>) -> Option<TransformOutput>;
}

pub trait OffloadTransform {
    fn name(&self) -> &'static str;
    fn estimate_bloat(&self, input: &PipelineInput<'_>) -> f32;
    fn apply(&self, input: &PipelineInput<'_>, store: &dyn CcrStore) -> Option<OffloadOutput>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn offload_output_requires_retained_ccr_put() {
        let token = "a1b2c3d4".repeat(4);
        let rejected = CcrPutResult::new(token.clone(), false);
        assert!(
            OffloadOutput::from_retained_put(
                "partial".to_string(),
                CompressorKind::Generic,
                rejected,
            )
            .is_none()
        );

        let retained = CcrPutResult::new(token.clone(), true);
        let out = OffloadOutput::from_retained_put(
            "partial".to_string(),
            CompressorKind::Generic,
            retained,
        )
        .expect("retained CCR put constructs offload output");
        assert_eq!(out.text(), "partial");
        assert_eq!(out.token(), token);
    }
}
