use super::*;
use crate::types::{ContentHint, ContentKind};

fn input<'a>(
    content: &'a str,
    hint: &'a ContentHint,
    command: Option<String>,
    argv: Option<Vec<String>>,
) -> CompressInput<'a> {
    CompressInput {
        content,
        kind: ContentKind::PlainText,
        hint,
        exit_code: Some(0),
        command,
        argv,
        original_bytes: content.len(),
    }
}

#[tokio::test]
async fn declines_domain_payload_without_command_context() {
    let hint = ContentHint::default();
    let opts = CompressOptions::default();
    let compressor = GenericCompressor;

    let output = compressor
        .compress(
            &input("line\n".repeat(100).as_str(), &hint, None, None),
            &opts,
        )
        .await;

    assert!(output.is_none());
}

#[tokio::test]
async fn runs_fallback_for_command_context() {
    let hint = ContentHint::default();
    let opts = CompressOptions {
        max_inline_chars: Some(80),
        ..Default::default()
    };
    let content = (0..40)
        .map(|i| format!("ordinary output line {i}"))
        .collect::<Vec<_>>()
        .join("\n");
    let compressor = GenericCompressor;

    let output = compressor
        .compress(
            &input(&content, &hint, Some("custom command".into()), None),
            &opts,
        )
        .await
        .expect("command output should use generic fallback");

    assert_eq!(output.kind, CompressorKind::Generic);
    assert!(output.lossy);
    assert!(output.text.len() < content.len());
}
