use super::*;
use std::sync::Arc;

use crate::cache::{CcrStore, MemoryCcrStore};
use crate::pipeline::PipelineInput;
use crate::types::CompressOptions;

fn opts() -> CompressOptions {
    CompressOptions {
        min_bytes_to_compress: 64,
        ccr_min_tokens: 1,
        ..Default::default()
    }
}

fn ml_opts() -> CompressOptions {
    CompressOptions {
        ml_text_enabled: true,
        ..opts()
    }
}

#[test]
fn protector_ignores_normal_html_tags() {
    let protected = protect_custom_tags("<div><span>hello</span><br /></div>");

    assert_eq!(protected.text, "<div><span>hello</span><br /></div>");
    assert!(protected.replacements.is_empty());
}

#[test]
fn protector_handles_nested_and_self_closing_custom_tags() {
    let tag = "<workflow id=\"a\">\n<context-item value=\"1\" />\n</workflow>";
    let input = format!("before {tag} after");
    let protected = protect_custom_tags(&input);

    assert_eq!(protected.replacements.len(), 1);
    assert!(!protected.text.contains(tag));
    assert_eq!(
        protected.restore(&protected.text).as_deref(),
        Some(input.as_str())
    );
}

#[test]
fn protector_restores_duplicate_blocks_independently() {
    let input = "<task>first</task>\nbody\n<task>second</task>";
    let protected = protect_custom_tags(input);

    assert_eq!(protected.replacements.len(), 2);
    assert_ne!(
        protected.replacements[0].placeholder,
        protected.replacements[1].placeholder
    );
    let candidate = format!(
        "{} then {}",
        protected.replacements[1].placeholder, protected.replacements[0].placeholder
    );
    let restored = protected.restore(&candidate).expect("restore");

    assert_eq!(restored, "<task>second</task> then <task>first</task>");
}

#[test]
fn protector_chooses_uncollided_placeholder_prefix() {
    let input = "__TOKENJUICE_TAG_0_ already exists <custom-tag />";
    let protected = protect_custom_tags(input);

    assert_eq!(protected.replacements.len(), 1);
    assert!(
        protected.replacements[0]
            .placeholder
            .starts_with("__TOKENJUICE_TAG_1_"),
        "{protected:?}"
    );
    assert_eq!(protected.restore(&protected.text).as_deref(), Some(input));
}

#[test]
fn malformed_custom_tag_passes_without_panic() {
    let input = "before <workflow><inner> missing close";
    let protected = protect_custom_tags(input);

    assert_eq!(protected.text, input);
    assert!(protected.replacements.is_empty());
}

#[tokio::test]
async fn ml_path_restores_custom_tags_byte_for_byte() {
    let _guard = crate::ml::callback_test_guard().await;
    crate::ml::configure_callback(Some(Arc::new(|text, _opts| {
        Box::pin(async move {
            assert!(!text.contains("<workflow"));
            let placeholder = text
                .split_whitespace()
                .find(|part| part.contains("__TOKENJUICE_TAG_"))
                .expect("placeholder")
                .to_string();
            Ok(Some(format!("compressed {placeholder}")))
        })
    })));

    let tag = "<workflow id=\"alpha\">\n<context-item value=\"1\" />\n</workflow>";
    let input = format!("{tag}\n{}", "ordinary filler ".repeat(200));
    let out = compress_ml_with_tag_protection(&input, &ml_opts())
        .await
        .expect("ml output");
    crate::ml::configure_callback(None);

    assert_eq!(out.kind, CompressorKind::MlText);
    assert!(out.text.contains(tag), "{}", out.text);
}

#[tokio::test]
async fn ml_path_declines_when_callback_drops_placeholder() {
    let _guard = crate::ml::callback_test_guard().await;
    crate::ml::configure_callback(Some(Arc::new(|_text, _opts| {
        Box::pin(async move { Ok(Some("compressed without protected marker".to_string())) })
    })));

    let input = format!("<workflow>keep me</workflow>\n{}", "filler ".repeat(200));
    let out = compress_ml_with_tag_protection(&input, &ml_opts()).await;
    crate::ml::configure_callback(None);

    assert!(out.is_none());
}

#[test]
fn keeps_errors_and_identifiers_as_verbatim_spans() {
    let mut input = String::new();
    for i in 0..40 {
        input.push_str(&format!(
            "ordinary deployment progress line {i} with routine status information.\n\n"
        ));
    }
    input.push_str("ERROR sync.worker.v2 failed for REQUEST_ID 9F42 after retry 17.\n\n");
    for i in 40..80 {
        input.push_str(&format!(
            "ordinary deployment progress line {i} with routine status information.\n\n"
        ));
    }

    let out = compress_textcrusher(&input, Some("sync.worker.v2 REQUEST_ID"), &opts())
        .expect("compresses")
        .text;

    assert!(out.contains("ERROR sync.worker.v2 failed"), "{out}");
    for line in out.lines() {
        assert!(input.contains(line), "non-verbatim span: {line}");
    }
}

#[test]
fn suppresses_near_duplicate_prose_deterministically() {
    let mut input = String::new();
    for i in 0..80 {
        input.push_str(&format!(
            "worker queue processed duplicate status message for tenant alpha batch {i}.\n\n"
        ));
    }

    let first = compress_textcrusher(&input, None, &opts())
        .expect("compresses")
        .text;
    let second = compress_textcrusher(&input, None, &opts())
        .expect("compresses")
        .text;

    assert_eq!(first, second);
    assert!(first.lines().count() < 20, "{first}");
    assert!(first.len() < input.len());
}

#[test]
fn textcrusher_transform_requires_retained_ccr() {
    let mut input = String::new();
    for i in 0..40 {
        input.push_str(&format!(
            "ordinary deployment progress line {i} with routine status information.\n\n"
        ));
    }
    input.push_str("ERROR sync.worker.v2 failed for REQUEST_ID 9F42 after retry 17.\n\n");
    for i in 40..80 {
        input.push_str(&format!(
            "ordinary deployment progress line {i} with routine status information.\n\n"
        ));
    }
    let pipeline_input = PipelineInput {
        content: &input,
        original_content: &input,
        content_kind: ContentKind::PlainText,
        original_bytes: input.len(),
    };
    let transform = TextCrusherTransform::new(opts()).with_query("sync.worker.v2 REQUEST_ID");

    let rejecting_store = MemoryCcrStore::new(1, 1);
    assert!(transform.apply(&pipeline_input, &rejecting_store).is_none());

    let store = MemoryCcrStore::default();
    let out = transform
        .apply(&pipeline_input, &store)
        .expect("retained textcrusher output");

    assert_eq!(out.kind(), CompressorKind::TextCrusher);
    assert!(out.text().contains("ERROR sync.worker.v2 failed"));
    assert_eq!(store.get(out.token()).as_deref(), Some(input.as_str()));
}

#[test]
fn textcrusher_transform_skips_non_plain_input() {
    let input = PipelineInput {
        content: "diff --git a/a b/a",
        original_content: "diff --git a/a b/a",
        content_kind: ContentKind::Diff,
        original_bytes: "diff --git a/a b/a".len(),
    };
    let transform = TextCrusherTransform::default();
    let store = MemoryCcrStore::default();

    assert_eq!(transform.estimate_bloat(&input), 0.0);
    assert!(transform.apply(&input, &store).is_none());
}

#[test]
fn plain_unsalient_data_declines() {
    let sentences = [
        "soft morning light crossed the empty kitchen and settled on the table.",
        "a folded blanket rested beside the chair near the quiet window.",
        "the hallway carried a faint smell of cedar from the old cabinet.",
        "afternoon clouds gathered slowly above the roofs beyond the lane.",
        "a notebook remained open to a page of careful handwriting.",
        "the garden path curved around stones set deep in the soil.",
        "warm tea cooled beside a stack of letters tied with string.",
        "the room held a simple lamp and a shelf of travel books.",
        "a blue curtain moved lightly whenever the door opened.",
        "distant bells marked the hour across the sleeping square.",
        "the wooden floor showed pale marks from years of use.",
        "fresh linen hung across the rail in the narrow courtyard.",
        "a small map lay flat beneath a smooth glass paperweight.",
        "evening settled over the street with a gentle amber color.",
        "the desk drawer contained stamps envelopes and a fountain pen.",
        "quiet footsteps faded on the stair beyond the landing.",
        "the wall clock ticked steadily through the still apartment.",
        "a weathered basket sat near the hearth filled with kindling.",
        "the porch boards were dry after several days of sun.",
        "a clean bowl and spoon waited beside the folded napkin.",
    ];
    let mut input = String::new();
    for sentence in sentences {
        input.push_str(sentence);
        input.push_str("\n\n");
    }

    assert!(compress_textcrusher(&input, None, &opts()).is_none());
}
