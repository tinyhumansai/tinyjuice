//! Per-content-kind compressors and the registry that maps a [`ContentKind`]
//! to the [`Compressor`] that handles it.
//!
//! Each compressor preserves the signal its kind carries — errors in logs,
//! changed hunks in diffs, signatures in code, anomalous rows in JSON — and
//! drops the rest. Whole-payload recovery belongs to the router
//! ([`crate::compress`]), which offloads the original to the CCR cache and
//! appends a retrieval footer. In addition, compressors may offload each
//! *omitted block* individually (via [`block_token`]) so its omission marker
//! carries a token that retrieves exactly that block — gated on
//! `opts.ccr_enabled`.

pub(crate) mod adaptive;
pub(crate) mod anchors;
pub mod code;
pub mod diff;
pub mod generic;
pub mod html;
pub mod json;
pub mod log;
pub mod log_template;
pub mod ml_text;
pub mod search;
pub mod signals;

use async_trait::async_trait;

use crate::types::{CompressInput, CompressOptions, CompressOutput, CompressorKind, ContentKind};

/// A content-aware compressor. Implementations are stateless and zero-sized;
/// the registry hands out `&'static` references.
#[async_trait]
pub trait Compressor: Send + Sync {
    /// Which [`CompressorKind`] this is (for stats/logs).
    fn kind(&self) -> CompressorKind;

    /// Compress `input`. Return `None` to decline (the router passes the
    /// original through). `Some` carries the compacted body and whether data
    /// was dropped. Async so the ML compressor can talk to its Python sidecar
    /// without a blocking bridge; native compressors complete synchronously.
    async fn compress(
        &self,
        input: &CompressInput<'_>,
        opts: &CompressOptions,
    ) -> Option<CompressOutput>;
}

static JSON_COMPRESSOR: json::JsonCompressor = json::JsonCompressor;
static CODE_COMPRESSOR: code::CodeCompressor = code::CodeCompressor;
static LOG_COMPRESSOR: log::LogCompressor = log::LogCompressor;
static SEARCH_COMPRESSOR: search::SearchCompressor = search::SearchCompressor;
static DIFF_COMPRESSOR: diff::DiffCompressor = diff::DiffCompressor;
static HTML_COMPRESSOR: html::HtmlCompressor = html::HtmlCompressor;
static ML_TEXT_COMPRESSOR: ml_text::MlTextCompressor = ml_text::MlTextCompressor;
static GENERIC_COMPRESSOR: generic::GenericCompressor = generic::GenericCompressor;

/// Map a detected [`ContentKind`] to the compressor that handles it.
///
/// `PlainText` routes to the ML compressor; whether it actually runs is gated
/// by `opts.ml_text_enabled` (and runtime Python/runtime_python_server
/// availability), and it falls back to [`generic::GenericCompressor`] otherwise
/// — that gating lives in [`crate::compress`], so this
/// function is a pure static mapping.
pub fn compressor_for(kind: ContentKind) -> &'static dyn Compressor {
    match kind {
        ContentKind::Json => &JSON_COMPRESSOR,
        ContentKind::Code => &CODE_COMPRESSOR,
        ContentKind::Log => &LOG_COMPRESSOR,
        ContentKind::Search => &SEARCH_COMPRESSOR,
        ContentKind::Diff => &DIFF_COMPRESSOR,
        ContentKind::Html => &HTML_COMPRESSOR,
        ContentKind::PlainText => &ML_TEXT_COMPRESSOR,
    }
}

/// The generic line-oriented fallback compressor (head/tail summariser). Used
/// by the router when a specialised compressor declines or is disabled.
pub fn generic_compressor() -> &'static dyn Compressor {
    &GENERIC_COMPRESSOR
}

/// One-line note appended to compacted output when any omitted block carries
/// its own retrieval token.
pub(crate) const BLOCK_NOTE: &str = "\n[omitted blocks are individually retrievable: call tinyjuice_retrieve with the token inside an omission marker to expand just that block]";

/// Offload an omitted block to CCR and return the marker text to embed in the
/// omission placeholder (` ⟦tj:<hash>⟧`), or an empty string when per-block
/// tokens are disabled or the block couldn't be retained. The store is
/// content-addressed and idempotent, so identical blocks share one entry.
pub(crate) fn block_token(block: &str, enabled: bool) -> String {
    if !enabled {
        return String::new();
    }
    let (token, retained) = crate::cache::offload_checked(block);
    if retained {
        format!(" {}", crate::cache::format_marker(&token))
    } else {
        String::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn registry_maps_all_content_kinds_to_expected_compressors() {
        let cases = [
            (ContentKind::Json, CompressorKind::SmartCrusher),
            (ContentKind::Code, CompressorKind::Code),
            (ContentKind::Log, CompressorKind::Log),
            (ContentKind::Search, CompressorKind::Search),
            (ContentKind::Diff, CompressorKind::Diff),
            (ContentKind::Html, CompressorKind::Html),
            (ContentKind::PlainText, CompressorKind::MlText),
        ];

        for (kind, expected) in cases {
            assert_eq!(compressor_for(kind).kind(), expected);
        }
        assert_eq!(generic_compressor().kind(), CompressorKind::Generic);
    }
}
