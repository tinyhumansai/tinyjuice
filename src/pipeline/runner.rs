use crate::cache::CcrStore;
use crate::pipeline::{
    OffloadTransform, PipelineInput, PipelineReport, PipelineSkipReason, PipelineStep,
    ReformatTransform,
};
use crate::types::CompressorKind;

/// Output from a typed pipeline run.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypedPipelineOutput {
    pub text: String,
    pub kind: CompressorKind,
    pub lossy: bool,
    pub ccr_token: Option<String>,
    pub report: PipelineReport,
}

/// Run a small typed transform pipeline.
///
/// Reformat transforms run first and may update the current body without CCR.
/// Offload transforms run after reformats, but the [`PipelineInput`] still
/// carries `original_content`, so lossy transforms can retain the true original
/// rather than an intermediate reformat.
pub fn run_typed_pipeline(
    input: PipelineInput<'_>,
    reformat_transforms: &[&dyn ReformatTransform],
    offload_transforms: &[&dyn OffloadTransform],
    store: &dyn CcrStore,
) -> TypedPipelineOutput {
    let mut current = input.content.to_string();
    let mut kind = CompressorKind::None;
    let mut applied_steps = Vec::new();
    let mut skipped_steps = Vec::new();
    let mut offload_declined = false;

    for transform in reformat_transforms {
        let step = PipelineStep {
            name: transform.name(),
            compressor: None,
        };
        let current_input = PipelineInput {
            content: &current,
            original_content: input.original_content,
            content_kind: input.content_kind,
            original_bytes: input.original_bytes,
        };
        if !transform.applies_to(&current_input) {
            skipped_steps.push(step);
            continue;
        }
        let Some(output) = transform.apply(&current_input) else {
            skipped_steps.push(step);
            continue;
        };
        if output.text.len() >= current.len() {
            skipped_steps.push(PipelineStep {
                name: transform.name(),
                compressor: Some(output.kind),
            });
            continue;
        }
        kind = output.kind;
        current = output.text;
        applied_steps.push(PipelineStep {
            name: transform.name(),
            compressor: Some(output.kind),
        });
    }

    for transform in offload_transforms {
        let current_input = PipelineInput {
            content: &current,
            original_content: input.original_content,
            content_kind: input.content_kind,
            original_bytes: input.original_bytes,
        };
        if transform.estimate_bloat(&current_input) <= 0.0 {
            skipped_steps.push(PipelineStep {
                name: transform.name(),
                compressor: None,
            });
            continue;
        }
        let Some(output) = transform.apply(&current_input, store) else {
            offload_declined = true;
            skipped_steps.push(PipelineStep {
                name: transform.name(),
                compressor: None,
            });
            continue;
        };
        let text = output.text().to_string();
        let token = output.token().to_string();
        kind = output.kind();
        applied_steps.push(PipelineStep {
            name: transform.name(),
            compressor: Some(kind),
        });
        let report = PipelineReport {
            content_kind: input.content_kind,
            original_bytes: input.original_bytes,
            compacted_bytes: text.len(),
            bloat_estimate: None,
            applied_steps,
            skipped_steps,
            ccr_tokens: vec![token.clone()],
            lossy: true,
            skip_reason: None,
        };
        return TypedPipelineOutput {
            text,
            kind,
            lossy: true,
            ccr_token: Some(token),
            report,
        };
    }

    let skip_reason = if applied_steps.is_empty() {
        Some(if offload_declined {
            PipelineSkipReason::CcrNotRetained
        } else {
            PipelineSkipReason::NoCompressor
        })
    } else {
        None
    };
    let report = PipelineReport {
        content_kind: input.content_kind,
        original_bytes: input.original_bytes,
        compacted_bytes: current.len(),
        bloat_estimate: None,
        applied_steps,
        skipped_steps,
        ccr_tokens: Vec::new(),
        lossy: false,
        skip_reason,
    };
    TypedPipelineOutput {
        text: current,
        kind,
        lossy: false,
        ccr_token: None,
        report,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cache::MemoryCcrStore;
    use crate::pipeline::{OffloadOutput, TransformOutput};
    use crate::types::ContentKind;

    struct SpaceReformat;

    impl ReformatTransform for SpaceReformat {
        fn name(&self) -> &'static str {
            "space_reformat"
        }

        fn applies_to(&self, input: &PipelineInput<'_>) -> bool {
            input.content_kind == ContentKind::PlainText && input.content.contains("  ")
        }

        fn apply(&self, input: &PipelineInput<'_>) -> Option<TransformOutput> {
            Some(TransformOutput::new(
                input
                    .content
                    .split_whitespace()
                    .collect::<Vec<_>>()
                    .join(" "),
                CompressorKind::Generic,
            ))
        }
    }

    struct PrefixOffload;

    impl OffloadTransform for PrefixOffload {
        fn name(&self) -> &'static str {
            "prefix_offload"
        }

        fn estimate_bloat(&self, input: &PipelineInput<'_>) -> f32 {
            if input.content.len() > 12 { 1.0 } else { 0.0 }
        }

        fn apply(&self, input: &PipelineInput<'_>, store: &dyn CcrStore) -> Option<OffloadOutput> {
            let body = input.content.chars().take(8).collect::<String>();
            OffloadOutput::from_retained_put(
                body,
                CompressorKind::Generic,
                store.put(input.original_content),
            )
        }
    }

    fn input(content: &str) -> PipelineInput<'_> {
        PipelineInput {
            content,
            original_content: content,
            content_kind: ContentKind::PlainText,
            original_bytes: content.len(),
        }
    }

    #[test]
    fn no_transforms_returns_passthrough_report() {
        let store = MemoryCcrStore::new(4, 1024);
        let out = run_typed_pipeline(input("plain"), &[], &[], &store);

        assert_eq!(out.text, "plain");
        assert!(!out.lossy);
        assert_eq!(
            out.report.skip_reason,
            Some(PipelineSkipReason::NoCompressor)
        );
        assert!(out.report.applied_steps.is_empty());
    }

    #[test]
    fn reformat_transform_runs_without_ccr() {
        let store = MemoryCcrStore::new(4, 1024);
        let reformat: [&dyn ReformatTransform; 1] = [&SpaceReformat];
        let out = run_typed_pipeline(input("alpha   beta   gamma"), &reformat, &[], &store);

        assert_eq!(out.text, "alpha beta gamma");
        assert!(!out.lossy);
        assert_eq!(out.kind, CompressorKind::Generic);
        assert_eq!(out.report.applied_steps[0].name, "space_reformat");
        assert!(out.report.ccr_tokens.is_empty());
    }

    #[test]
    fn offload_transform_requires_retained_original() {
        let store = MemoryCcrStore::new(4, 1024);
        let offload: [&dyn OffloadTransform; 1] = [&PrefixOffload];
        let original = "alpha beta gamma delta";
        let out = run_typed_pipeline(input(original), &[], &offload, &store);

        let token = out.ccr_token.expect("offload retained original");
        assert_eq!(out.text, "alpha be");
        assert!(out.lossy);
        assert_eq!(store.get(&token).as_deref(), Some(original));
        assert_eq!(out.report.ccr_tokens, vec![token]);
    }

    #[test]
    fn mixed_reformat_and_offload_retains_true_original() {
        let store = MemoryCcrStore::new(4, 1024);
        let reformat: [&dyn ReformatTransform; 1] = [&SpaceReformat];
        let offload: [&dyn OffloadTransform; 1] = [&PrefixOffload];
        let original = "alpha   beta   gamma   delta";
        let out = run_typed_pipeline(input(original), &reformat, &offload, &store);

        let token = out.ccr_token.expect("offload retained original");
        assert_eq!(out.text, "alpha be");
        assert_eq!(out.report.applied_steps.len(), 2);
        assert_eq!(store.get(&token).as_deref(), Some(original));
    }

    #[test]
    fn offload_store_failure_declines_lossy_output() {
        let store = MemoryCcrStore::new(1, 4);
        let offload: [&dyn OffloadTransform; 1] = [&PrefixOffload];
        let original = "alpha beta gamma delta";
        let out = run_typed_pipeline(input(original), &[], &offload, &store);

        assert_eq!(out.text, original);
        assert!(!out.lossy);
        assert_eq!(out.ccr_token, None);
        assert_eq!(
            out.report.skip_reason,
            Some(PipelineSkipReason::CcrNotRetained)
        );
    }
}
