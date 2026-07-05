//! ML plain-text compressor ("Kompress") — trait slot.
//!
//! Plain text has no structural skeleton to exploit, so high-quality
//! compression needs a learned model (Headroom uses ModernBERT token
//! classification to drop low-salience spans). That path runs the `kompress`
//! backend of the shared `runtime_python_server` and is **opt-in** behind the
//! `tokenjuice.ml_compression_enabled` config flag.
//!
//! This module is the [`Compressor`] slot; it delegates to
//! [`crate::ml`]. Whenever the flag is off or the Python
//! runtime is unavailable, `compress` declines so the router falls back to the
//! generic compressor — never an error in the agent loop.

use async_trait::async_trait;

use super::Compressor;
use crate::compressors::text::compress_ml_with_tag_protection;
use crate::types::{CompressInput, CompressOptions, CompressOutput, CompressorKind};

pub struct MlTextCompressor;

#[async_trait]
impl Compressor for MlTextCompressor {
    fn kind(&self) -> CompressorKind {
        CompressorKind::MlText
    }

    async fn compress(
        &self,
        input: &CompressInput<'_>,
        opts: &CompressOptions,
    ) -> Option<CompressOutput> {
        compress_ml_with_tag_protection(input.content, opts).await
    }
}

#[cfg(test)]
mod tests {
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
}
