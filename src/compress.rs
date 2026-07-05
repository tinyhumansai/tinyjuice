//! Universal content-aware compression entry point.
//!
//! [`compress_content`] is the broadly-usable function: hand it any blob and an
//! optional [`ContentHint`], and it detects the content kind, routes to the
//! right compressor, and — when the result drops data — offloads the original
//! to the CCR cache and appends a `⟦tj:<hash>⟧` retrieval footer so nothing is
//! ever silently lost. Use it for tool output, file reads, web/HTML fetches, or
//! any large payload headed for the model context.
//!
//! The tool-output adapter
//! [`crate::compact_tool_output_with_policy`] builds a
//! [`CompressInput`] with a derived command/argv and calls [`route`].

use crate::cache;
use crate::compressors::{compressor_for, generic_compressor};
use crate::detect::detect_content_kind;
use crate::savings;
use crate::tokens::estimate_tokens;
use crate::types::{
    CompressInput, CompressOptions, CompressOutput, CompressedOutput, ContentHint, ContentKind,
};

/// Compress arbitrary content. Detects the kind (honouring `hint`), routes to
/// the matching compressor, and offloads/marks the original via CCR when lossy.
///
/// Always pass-through safe: returns the original unchanged when the router is
/// disabled, the input is too small, the content kind has no enabled
/// compressor, or compression wouldn't shrink it.
pub async fn compress_content(
    content: &str,
    hint: Option<ContentHint>,
    opts: &CompressOptions,
) -> CompressedOutput {
    let hint = hint.unwrap_or_default();
    let input = CompressInput {
        content,
        kind: ContentKind::PlainText, // resolved inside route()
        hint: &hint,
        exit_code: None,
        command: None,
        argv: None,
        original_bytes: content.len(),
    };
    route(input, opts).await
}

/// Core router: detect (unless the input already carries a resolved kind via the
/// hint's explicit override), pick the compressor honouring config gates, run
/// it, and apply CCR offload + footer.
pub async fn route(mut input: CompressInput<'_>, opts: &CompressOptions) -> CompressedOutput {
    let content = input.content;
    let original_bytes = content.len();

    if !opts.router_enabled || original_bytes < opts.min_bytes_to_compress {
        // The kind is only a label on a passthrough — don't pay for full
        // detection (which parses entire JSON blobs) for content that won't
        // be compressed anyway.
        let kind = input.hint.explicit.unwrap_or(ContentKind::PlainText);
        return CompressedOutput::passthrough(content.to_string(), kind);
    }

    let kind = detect_content_kind(content, input.hint);
    input.kind = kind;

    // Resolve which compressor to try, honouring per-kind config gates.
    let primary: Option<&'static dyn crate::compressors::Compressor> = match kind {
        ContentKind::Search if !opts.search_enabled => None,
        ContentKind::Code if !opts.code_enabled => None,
        ContentKind::Html if !opts.html_enabled => None,
        _ => Some(compressor_for(kind)),
    };

    // Try the primary compressor; if it declines, fall back to the generic
    // head/tail path (which itself declines for non-command payloads).
    let mut produced: Option<CompressOutput> = match primary {
        Some(c) => c.compress(&input, opts).await,
        None => None,
    };
    // When the specialised compressor declines (including plain text with the
    // ML compressor off), fall back to the generic head/tail path. It runs the
    // rule engine for *command* output (so e.g. `git status` still compacts even
    // though it carries no log signal) and declines for domain-tool payloads.
    if produced.is_none() {
        produced = generic_compressor().compress(&input, opts).await;
    }

    let Some(out) = produced else {
        return CompressedOutput::passthrough(content.to_string(), kind);
    };
    if out.text.len() >= original_bytes {
        return CompressedOutput::passthrough(content.to_string(), kind);
    }

    // CCR threshold: only offload when the input is large enough to be worth
    // caching. When CCR is not in play a lossy result carries no recovery
    // footer; whether that is acceptable is the caller's call via
    // `lossy_without_ccr` (dropped content is still explicitly marked with
    // omission markers — it just isn't retrievable).
    let original_tokens = estimate_tokens(content);
    let ccr_for_call = opts.ccr_enabled && original_tokens as usize >= opts.ccr_min_tokens;
    if out.lossy && !ccr_for_call && !opts.lossy_without_ccr {
        return CompressedOutput::passthrough(content.to_string(), kind);
    }

    // Offload the original and append a recovery footer when CCR is in play.
    let (text, ccr_token) = if ccr_for_call {
        // The token is the content hash, so the footer is computable before
        // storing anything. If the footer tips the result to at least the
        // original size we pass through — checked first so the cache isn't
        // left holding an entry no output references.
        let token = cache::short_hash(content);
        let footer = cache::recovery_footer(&token, original_bytes, out.lossy);
        if out.text.len() + footer.len() >= original_bytes {
            return CompressedOutput::passthrough(content.to_string(), kind);
        }
        let (token, retained) = cache::offload_checked(content);
        if !retained {
            // The original is too large to keep in memory (over the byte cap)
            // and the disk tier isn't on, so it can't be recovered. A lossy
            // view is irreversible — decline it unless the caller allows
            // unrecoverable lossy output. Either way no (dangling) recovery
            // footer is attached.
            if out.lossy && !opts.lossy_without_ccr {
                return CompressedOutput::passthrough(content.to_string(), kind);
            }
            (out.text, None)
        } else {
            let mut text = out.text;
            text.push_str(&footer);
            (text, Some(token))
        }
    } else {
        (out.text, None)
    };

    let compacted_bytes = text.len();
    let compacted_tokens = estimate_tokens(&text);
    log::info!(
        "[tinyjuice] compacted kind={} compressor={} lossy={} {}->{} bytes (~{}->{} tok)",
        kind.as_str(),
        out.kind.as_str(),
        out.lossy,
        original_bytes,
        compacted_bytes,
        original_tokens,
        compacted_tokens,
    );

    // Record savings for the dashboard (tokens + cost saved for the LLM the
    // result is being compressed for).
    savings::record(kind, out.kind, original_tokens, compacted_tokens);

    CompressedOutput {
        text,
        content_kind: kind,
        compressor: out.kind,
        lossy: out.lossy,
        applied: true,
        ccr_token,
        original_bytes,
        compacted_bytes,
    }
}

/// Build a [`CompressorKind`] label-free passthrough quickly (used by callers
/// that only need to detect without compressing).
pub fn detect_only(content: &str, hint: &ContentHint) -> ContentKind {
    detect_content_kind(content, hint)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::CompressorKind;

    fn opts() -> CompressOptions {
        CompressOptions {
            min_bytes_to_compress: 64,
            ..Default::default()
        }
    }

    #[tokio::test]
    async fn routes_json_and_offloads() {
        let mut rows = Vec::new();
        for i in 0..120 {
            rows.push(format!(
                r#"{{"id":{i},"name":"account_{i}","email":"a{i}@ex.com","tier":"gold"}}"#
            ));
        }
        let original = format!("[{}]", rows.join(","));
        let res = compress_content(&original, None, &opts()).await;
        assert!(res.applied);
        assert_eq!(res.content_kind, ContentKind::Json);
        assert_eq!(res.compressor, CompressorKind::SmartCrusher);
        assert!(res.text.len() < original.len());
        let token = res.ccr_token.expect("offloaded");
        assert_eq!(cache::retrieve(&token).as_deref(), Some(original.as_str()));
        assert!(
            res.text.contains("⟦tj:"),
            "footer marker present: {}",
            res.text
        );
    }

    #[tokio::test]
    async fn small_input_passes_through() {
        let res = compress_content("tiny", None, &opts()).await;
        assert!(!res.applied);
        assert_eq!(res.text, "tiny");
    }

    #[tokio::test]
    async fn html_hint_extracts_text() {
        let mut html = String::from("<html><body>");
        for i in 0..60 {
            html.push_str(&format!("<div><span>cell number {i} content</span></div>"));
        }
        html.push_str("</body></html>");
        let hint = ContentHint {
            mime: Some("text/html".into()),
            ..Default::default()
        };
        let res = compress_content(&html, Some(hint), &opts()).await;
        assert!(res.applied);
        assert_eq!(res.content_kind, ContentKind::Html);
        assert!(res.text.contains("cell number 7 content"));
    }

    fn big_json_array() -> String {
        let mut rows = Vec::new();
        for i in 0..120 {
            rows.push(format!(
                r#"{{"id":{i},"name":"account_{i}","email":"a{i}@ex.com","tier":"gold"}}"#
            ));
        }
        format!("[{}]", rows.join(","))
    }

    #[tokio::test]
    async fn lossy_compression_works_without_ccr() {
        let mut o = opts();
        o.ccr_enabled = false;
        let original = big_json_array();
        let res = compress_content(&original, None, &o).await;
        assert!(res.applied, "lossy compression must apply without CCR");
        assert!(res.text.len() < original.len());
        assert!(res.ccr_token.is_none(), "no recovery token without CCR");
        assert!(
            !res.text.contains("⟦tj:"),
            "no dangling footer: {}",
            res.text
        );
    }

    #[tokio::test]
    async fn strict_mode_declines_lossy_without_ccr() {
        let mut o = opts();
        o.ccr_enabled = false;
        o.lossy_without_ccr = false;
        let original = big_json_array();
        let res = compress_content(&original, None, &o).await;
        assert!(!res.applied, "strict mode must decline unrecoverable lossy");
        assert_eq!(res.text, original);
    }

    #[tokio::test]
    async fn router_disabled_is_passthrough() {
        let mut o = opts();
        o.router_enabled = false;
        let big = "x".repeat(5000);
        let res = compress_content(&big, None, &o).await;
        assert!(!res.applied);
        assert_eq!(res.text, big);
    }
}
