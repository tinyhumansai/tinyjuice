use super::*;
use crate::types::{ContentHint, ContentKind};

fn input<'a>(content: &'a str, hint: &'a ContentHint) -> CompressInput<'a> {
    CompressInput {
        content,
        kind: ContentKind::PlainText,
        hint,
        exit_code: None,
        command: None,
        argv: None,
        original_bytes: content.len(),
    }
}

#[tokio::test]
async fn disabled_option_declines_without_touching_callback() {
    let hint = ContentHint::default();
    let opts = CompressOptions {
        ml_text_enabled: false,
        ..Default::default()
    };
    let compressor = MlTextCompressor;

    let output = compressor
        .compress(&input("plain text", &hint), &opts)
        .await;

    assert!(output.is_none());
}

#[tokio::test]
async fn enabled_option_declines_when_no_callback_is_available() {
    crate::ml::configure_callback(None);
    let hint = ContentHint::default();
    let opts = CompressOptions {
        ml_text_enabled: true,
        ..Default::default()
    };
    let compressor = MlTextCompressor;

    let output = compressor
        .compress(&input("plain text", &hint), &opts)
        .await;

    assert!(output.is_none());
}
