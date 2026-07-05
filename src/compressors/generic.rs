//! Generic line-oriented fallback compressor.
//!
//! Used by the router when a specialised compressor declines (or the ML text
//! compressor is unavailable). It runs the rule engine's `generic/fallback`
//! head/tail summariser — but **only for command output**. For domain-tool
//! payloads (no derived command/argv) it declines, preserving the long-standing
//! guard that large structured tool results must reach the downstream
//! progressive-disclosure handoff rather than being blindly head/tail clamped.

use async_trait::async_trait;

use super::Compressor;
use crate::compressors::log::compress_command_fallback;
use crate::types::{CompressInput, CompressOptions, CompressOutput, CompressorKind};

pub struct GenericCompressor;

#[async_trait]
impl Compressor for GenericCompressor {
    fn kind(&self) -> CompressorKind {
        CompressorKind::Generic
    }

    async fn compress(
        &self,
        input: &CompressInput<'_>,
        opts: &CompressOptions,
    ) -> Option<CompressOutput> {
        let has_command =
            input.command.is_some() || input.argv.as_ref().is_some_and(|a| !a.is_empty());
        if !has_command {
            // Domain-tool payload — decline rather than blind-truncate.
            return None;
        }
        compress_command_fallback(input, opts)
    }
}

#[cfg(test)]
mod tests {
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
}
