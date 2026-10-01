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
#[path = "runner_tests.rs"]
mod tests;
