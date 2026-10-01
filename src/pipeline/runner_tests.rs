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
