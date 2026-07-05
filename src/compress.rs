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
use crate::tokens::estimate_tokens_with;
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

    // The global floor gates everything; the (lower) log floor lets small
    // failure logs through. Below the smaller of the two, bail before paying
    // for detection (which parses entire JSON blobs) — the kind is only a
    // label on a passthrough.
    let log_floor = opts
        .min_bytes_to_compress_log
        .min(opts.min_bytes_to_compress);
    if !opts.router_enabled || original_bytes < log_floor {
        let kind = input.hint.explicit.unwrap_or(ContentKind::PlainText);
        return CompressedOutput::passthrough(content.to_string(), kind);
    }

    let kind = detect_content_kind(content, input.hint);
    input.kind = kind;

    // Between the log floor and the global floor only log-like content
    // proceeds: detected logs, or command output headed for the rule engine.
    // Everything else keeps the historical global floor.
    if original_bytes < opts.min_bytes_to_compress {
        let log_like = kind == ContentKind::Log || input.command.is_some();
        if !log_like || original_bytes < opts.min_bytes_to_compress_log {
            return CompressedOutput::passthrough(content.to_string(), kind);
        }
    }

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
    // caching, or when the compression is heavily lossy on a non-trivial input
    // (rationale on `CompressOptions::ccr_min_tokens`). When CCR is not in
    // play a lossy result carries no recovery footer; whether that is
    // acceptable is the caller's call via `lossy_without_ccr` (dropped content
    // is still explicitly marked with omission markers — it just isn't
    // retrievable), except for code, which is never emitted lossy without a
    // recovery path (see `lossy_ok_without_ccr` below).
    let original_tokens = estimate_tokens_with(content, opts.chars_per_token);
    let compacted_body_tokens = estimate_tokens_with(&out.text, opts.chars_per_token);
    let heavy_crush = compacted_body_tokens <= original_tokens / 2
        && original_tokens as usize >= opts.ccr_min_tokens / 4;
    let ccr_for_call =
        opts.ccr_enabled && (original_tokens as usize >= opts.ccr_min_tokens || heavy_crush);
    // Code is special. A lossy code view collapses whole function bodies, and
    // those lines are only recoverable when CCR is on (each collapsed body
    // carries a per-block retrieval token, offloaded regardless of the
    // file-level footer). Truncating a log to a head/tail is a bounded, marked
    // loss; eating the middle of a function with no way to get it back is not.
    // So code never rides the `lossy_without_ccr` escape hatch — with CCR off
    // it passes through verbatim rather than dropping code the caller can't
    // recover.
    let unrecoverable_code = kind == ContentKind::Code && !opts.ccr_enabled;
    let lossy_ok_without_ccr = opts.lossy_without_ccr && !unrecoverable_code;
    if out.lossy && !ccr_for_call && !lossy_ok_without_ccr {
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
        let retained = cache::offload_checked_with_hash(&token, content);
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
    let compacted_tokens = estimate_tokens_with(&text, opts.chars_per_token);
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

    /// A collapsible source file with many long function bodies. Big enough to
    /// clear the global floor and to have bodies worth collapsing.
    fn big_code_file() -> String {
        let mut src = String::from("use std::collections::HashMap;\n\n");
        for f in 0..4 {
            src.push_str(&format!("pub fn worker_{f}(items: &[i32]) -> i32 {{\n"));
            for i in 0..30 {
                src.push_str(&format!(
                    "    let tmp_{f}_{i} = items.iter().sum::<i32>() + {i};\n"
                ));
            }
            src.push_str(&format!("    tmp_{f}_0\n}}\n\n"));
        }
        src
    }

    fn code_hint() -> ContentHint {
        ContentHint {
            explicit: Some(ContentKind::Code),
            extension: Some("rs".into()),
            ..Default::default()
        }
    }

    /// Code must never be cut when it can't be recovered: with CCR off, even
    /// though `lossy_without_ccr` is on (the default), the router passes the
    /// source through verbatim rather than dropping function bodies with no
    /// retrieval token.
    #[tokio::test]
    async fn code_is_not_cut_without_ccr() {
        let mut o = opts();
        o.ccr_enabled = false;
        assert!(o.lossy_without_ccr, "escape hatch is on by default");
        let original = big_code_file();
        let res = compress_content(&original, Some(code_hint()), &o).await;
        assert_eq!(res.content_kind, ContentKind::Code);
        assert!(
            !res.applied,
            "unrecoverable code must pass through, not be cut: {}",
            res.text
        );
        assert_eq!(res.text, original, "source returned verbatim");
        assert!(
            res.text.contains("tmp_1_15"),
            "no body lines eaten: {}",
            res.text
        );
    }

    /// With CCR on, code still collapses — every omitted body is individually
    /// retrievable, so cutting it is safe.
    #[tokio::test]
    async fn code_collapses_with_ccr() {
        let o = opts();
        assert!(o.ccr_enabled);
        let original = big_code_file();
        let res = compress_content(&original, Some(code_hint()), &o).await;
        assert_eq!(res.content_kind, ContentKind::Code);
        assert!(res.applied, "CCR makes the collapse recoverable");
        assert!(res.text.len() < original.len());
        assert!(res.text.contains("pub fn worker_0"), "signatures kept");
        // Every collapsed body carries a per-block retrieval token.
        let tokens = cache::parse_markers(&res.text);
        assert!(!tokens.is_empty(), "recoverable tokens present: {}", res.text);
        let body = cache::retrieve(&tokens[0]).expect("stored");
        assert!(body.contains("tmp_0_15"), "full body recoverable: {body}");
    }

    #[tokio::test]
    async fn small_log_above_log_floor_is_compressed() {
        // Default floors: global 2048, log 512. A ~1.8 KB failure log sits
        // between them and must still be compressed.
        let mut log = String::new();
        for i in 0..45 {
            log.push_str(&format!("info sync {i}\n"));
        }
        for i in 0..60 {
            log.push_str(&format!("error shard down {i}\n"));
        }
        assert!(log.len() > 512 && log.len() < 2048, "len={}", log.len());

        let res = compress_content(&log, None, &CompressOptions::default()).await;
        assert!(res.applied, "small log must compress: {res:?}");
        assert_eq!(res.content_kind, ContentKind::Log);
        assert!(res.text.len() < log.len());
    }

    #[tokio::test]
    async fn small_command_output_above_log_floor_runs_rules() {
        // Command output between the log floor and the global floor still
        // reaches the rule engine.
        let mut lines = vec!["On branch main".to_string()];
        for index in 0..30 {
            lines.push(format!("\tmodified:   src/some_module/file_{index}.rs"));
        }
        let content = lines.join("\n");
        assert!(content.len() > 512 && content.len() < 2048);
        let hint = ContentHint::default();
        let input = CompressInput {
            content: &content,
            kind: ContentKind::PlainText,
            hint: &hint,
            exit_code: Some(0),
            command: Some("git status".to_string()),
            argv: Some(vec!["git".to_string(), "status".to_string()]),
            original_bytes: content.len(),
        };
        let res = route(input, &CompressOptions::default()).await;
        assert!(res.applied, "command output must compress: {res:?}");
        assert!(res.text.len() < content.len());
    }

    #[tokio::test]
    async fn small_json_below_global_floor_still_passes_through() {
        // Non-log content keeps the historical 2048-byte floor.
        let mut rows = Vec::new();
        for i in 0..20 {
            rows.push(format!(
                r#"{{"id":{i},"name":"account_{i}","email":"a{i}@ex.com","tier":"gold"}}"#
            ));
        }
        let original = format!("[{}]", rows.join(","));
        assert!(original.len() > 512 && original.len() < 2048);
        let res = compress_content(&original, None, &CompressOptions::default()).await;
        assert!(!res.applied, "small JSON stays passthrough: {res:?}");
        assert_eq!(res.text, original);
    }

    #[tokio::test]
    async fn tiny_log_below_log_floor_passes_through() {
        let log = "error failed to start\nwarning low disk\n".repeat(3);
        assert!(log.len() < 512);
        let res = compress_content(&log, None, &CompressOptions::default()).await;
        assert!(!res.applied);
        assert_eq!(res.text, log);
    }

    #[tokio::test]
    async fn heavy_crush_below_ccr_min_tokens_still_offloads() {
        // ~470 estimated tokens — under the flat ccr_min_tokens (500) but well
        // over its quarter — crushed to less than half the tokens: the
        // ratio-aware gate must offload so the original stays retrievable.
        let mut o = opts();
        assert_eq!(o.ccr_min_tokens, 500);
        o.min_bytes_to_compress = 64;
        // Long repeated keys, short values: the tabular re-render (keys
        // emitted once) crushes this to well under half the tokens.
        let mut rows = Vec::new();
        for i in 0..18 {
            rows.push(format!(
                r#"{{"identifier":{i},"account_name":"a{i}","email_address":"a{i}@x.io","subscription_tier":"g","current_status":"ok"}}"#
            ));
        }
        let original = format!("[{}]", rows.join(","));
        let original_tokens = crate::tokens::estimate_tokens(&original) as usize;
        assert!(
            (125..500).contains(&original_tokens),
            "tokens={original_tokens}"
        );
        let res = compress_content(&original, None, &o).await;
        assert!(res.applied, "response: {res:?}");
        let token = res.ccr_token.expect("ratio-aware gate offloads");
        assert_eq!(cache::retrieve(&token).as_deref(), Some(original.as_str()));
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
