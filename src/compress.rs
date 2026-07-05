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
use crate::cache::CcrStore;
use crate::compressors::{compressor_for, generic_compressor};
use crate::detect::detect_content_kind;
use crate::pipeline::{PipelineReport, PipelineSkipReason, estimate_bloat};
use crate::policy::{ShellCompactionPolicy, ShellPolicyDecision, apply_shell_compaction_policy};
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
    compress_content_with_store_report(content, hint, opts, &cache::GlobalCcrStore)
        .await
        .0
}

/// Store-injected variant of [`compress_content`].
pub async fn compress_content_with_store(
    content: &str,
    hint: Option<ContentHint>,
    opts: &CompressOptions,
    store: &dyn CcrStore,
) -> CompressedOutput {
    compress_content_with_store_report(content, hint, opts, store)
        .await
        .0
}

/// Store-injected variant of [`compress_content`] that also returns a redacted
/// pipeline report.
pub async fn compress_content_with_store_report(
    content: &str,
    hint: Option<ContentHint>,
    opts: &CompressOptions,
    store: &dyn CcrStore,
) -> (CompressedOutput, PipelineReport) {
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
    route_with_store_report(input, opts, store).await
}

/// Core router: detect (unless the input already carries a resolved kind via the
/// hint's explicit override), pick the compressor honouring config gates, run
/// it, and apply CCR offload + footer.
pub async fn route(input: CompressInput<'_>, opts: &CompressOptions) -> CompressedOutput {
    route_with_store_report(input, opts, &cache::GlobalCcrStore)
        .await
        .0
}

/// Store-injected variant of [`route`].
pub async fn route_with_store(
    input: CompressInput<'_>,
    opts: &CompressOptions,
    store: &dyn CcrStore,
) -> CompressedOutput {
    route_with_store_report(input, opts, store).await.0
}

/// Store-injected variant of [`route`] that also returns a redacted pipeline
/// report.
pub async fn route_with_store_report(
    mut input: CompressInput<'_>,
    opts: &CompressOptions,
    store: &dyn CcrStore,
) -> (CompressedOutput, PipelineReport) {
    let content = input.content;
    let original_bytes = content.len();

    if !opts.router_enabled {
        let kind = detect_content_kind(content, input.hint);
        let res = CompressedOutput::passthrough(content.to_string(), kind);
        let report =
            PipelineReport::passthrough(kind, original_bytes, PipelineSkipReason::RouterDisabled)
                .with_bloat_estimate(estimate_bloat(content, kind));
        return (res, report);
    }

    if original_bytes < opts.min_bytes_to_compress {
        let kind = detect_content_kind(content, input.hint);
        let res = CompressedOutput::passthrough(content.to_string(), kind);
        let report =
            PipelineReport::passthrough(kind, original_bytes, PipelineSkipReason::BelowMinBytes)
                .with_bloat_estimate(estimate_bloat(content, kind));
        return (res, report);
    }

    let kind = detect_content_kind(content, input.hint);
    let bloat_estimate = estimate_bloat(content, kind);
    let shell_policy_decision = apply_shell_compaction_policy(
        &crate::types::ToolExecutionInput {
            tool_name: input
                .hint
                .source_tool
                .clone()
                .unwrap_or_else(|| "shell".to_owned()),
            command: input.command.clone(),
            argv: input.argv.clone(),
            stdout: Some(content.to_owned()),
            exit_code: input.exit_code,
            ..Default::default()
        },
        ShellCompactionPolicy::AllowSafeInventory,
    );
    if !matches!(shell_policy_decision, ShellPolicyDecision::Compact) {
        let res = CompressedOutput::passthrough(content.to_string(), kind);
        let report = PipelineReport::passthrough(
            kind,
            original_bytes,
            PipelineSkipReason::Other(shell_policy_decision.as_str()),
        )
        .with_bloat_estimate(bloat_estimate);
        return (res, report);
    }
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
        let res = CompressedOutput::passthrough(content.to_string(), kind);
        let report =
            PipelineReport::passthrough(kind, original_bytes, PipelineSkipReason::NoCompressor)
                .with_bloat_estimate(bloat_estimate);
        return (res, report);
    };
    if out.text.len() >= original_bytes {
        let res = CompressedOutput::passthrough(content.to_string(), kind);
        let report =
            PipelineReport::passthrough(kind, original_bytes, PipelineSkipReason::NoSavings)
                .with_bloat_estimate(bloat_estimate);
        return (res, report);
    }

    // CCR threshold: only offload (and therefore only allow *lossy* compaction)
    // when the input is large enough to be worth caching. Below the token
    // threshold a lossy result can't be made recoverable, so pass it through;
    // lossless reformats are still allowed without an offload.
    let original_tokens = estimate_tokens(content);
    let ccr_for_call = opts.ccr_enabled && original_tokens as usize >= opts.ccr_min_tokens;
    if out.lossy && !ccr_for_call {
        let res = CompressedOutput::passthrough(content.to_string(), kind);
        let report =
            PipelineReport::passthrough(kind, original_bytes, PipelineSkipReason::CcrDisabled)
                .with_bloat_estimate(bloat_estimate);
        return (res, report);
    }

    // Offload the original and expose the recovery footer separately. `text`
    // below remains the compatibility output (body + footer), while hosts with
    // their own caps can truncate `body` and reattach `recovery_footer`.
    let (body, recovery_footer, ccr_token) = if ccr_for_call {
        let put = store.put(content);
        if !put.retained() {
            // The original is too large to keep in memory (over the byte cap)
            // and the disk tier isn't on, so it can't be recovered. A lossy view
            // would be irreversible — decline it. A lossless reformat is still
            // safe to return, just without a (dangling) recovery footer.
            if out.lossy {
                let res = CompressedOutput::passthrough(content.to_string(), kind);
                let report = PipelineReport::passthrough(
                    kind,
                    original_bytes,
                    PipelineSkipReason::CcrNotRetained,
                )
                .with_bloat_estimate(bloat_estimate);
                return (res, report);
            }
            (out.text, None, None)
        } else {
            let token = put.token().to_string();
            let footer = cache::recovery_footer(&token, original_bytes, out.lossy);
            let mut text = out.text.clone();
            text.push_str(&footer);
            // The footer adds bytes — if it tipped us over the original size, bail.
            if text.len() >= original_bytes {
                let res = CompressedOutput::passthrough(content.to_string(), kind);
                let report = PipelineReport::passthrough(
                    kind,
                    original_bytes,
                    PipelineSkipReason::FooterWouldGrow,
                )
                .with_bloat_estimate(bloat_estimate);
                return (res, report);
            }
            (out.text, Some(footer), Some(token))
        }
    } else {
        (out.text, None, None)
    };

    let text = if let Some(footer) = recovery_footer.as_deref() {
        let mut text = body.clone();
        text.push_str(footer);
        text
    } else {
        body.clone()
    };

    let compacted_bytes = text.len();
    let compacted_tokens = estimate_tokens(&text);
    log::info!(
        "[tokenjuice] compacted kind={} compressor={} lossy={} {}->{} bytes (~{}->{} tok)",
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

    let res = CompressedOutput {
        text,
        body,
        recovery_footer,
        content_kind: kind,
        compressor: out.kind,
        lossy: out.lossy,
        applied: true,
        ccr_token,
        original_bytes,
        compacted_bytes,
    };
    let report = PipelineReport::applied(
        kind,
        original_bytes,
        compacted_bytes,
        out.kind,
        out.lossy,
        res.ccr_token.clone(),
    )
    .with_bloat_estimate(bloat_estimate);
    (res, report)
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
    async fn exposes_body_and_recovery_footer_separately() {
        let mut rows = Vec::new();
        for i in 0..120 {
            rows.push(format!(
                r#"{{"id":{i},"name":"isolated_account_{i}","email":"iso{i}@ex.com","tier":"platinum"}}"#
            ));
        }
        let original = format!("[{}]", rows.join(","));
        let res = compress_content(&original, None, &opts()).await;

        assert!(res.applied);
        let footer = res
            .recovery_footer
            .as_deref()
            .expect("offloaded output exposes footer");
        assert!(!res.body.contains("⟦tj:"), "body must not contain footer");
        assert!(footer.contains("⟦tj:"), "footer carries marker: {footer}");
        assert_eq!(res.text, format!("{}{}", res.body, footer));
        assert_eq!(res.compacted_bytes, res.text.len());
    }

    #[tokio::test]
    async fn store_injected_route_uses_isolated_ccr_store() {
        let store = cache::MemoryCcrStore::new(10, 1_000_000);
        let mut rows = Vec::new();
        for i in 0..120 {
            rows.push(format!(
                r#"{{"id":{i},"name":"store_only_account_{i}","email":"store{i}@ex.com","tier":"silver"}}"#
            ));
        }
        let original = format!("[{}]", rows.join(","));
        let res = compress_content_with_store(&original, None, &opts(), &store).await;

        assert!(res.applied);
        let token = res.ccr_token.as_deref().expect("offloaded");
        assert_eq!(store.get(token).as_deref(), Some(original.as_str()));
        assert_eq!(cache::retrieve(token), None, "global cache must not see it");
    }

    #[tokio::test]
    async fn report_records_applied_step_and_ccr_token() {
        let store = cache::MemoryCcrStore::new(10, 1_000_000);
        let mut rows = Vec::new();
        for i in 0..120 {
            rows.push(format!(
                r#"{{"id":{i},"name":"report_account_{i}","email":"report{i}@ex.com","tier":"bronze"}}"#
            ));
        }
        let original = format!("[{}]", rows.join(","));
        let (res, report) =
            compress_content_with_store_report(&original, None, &opts(), &store).await;

        assert!(res.applied);
        assert_eq!(report.content_kind, ContentKind::Json);
        assert_eq!(report.original_bytes, original.len());
        assert_eq!(report.compacted_bytes, res.text.len());
        assert_eq!(report.applied_steps.len(), 1);
        assert_eq!(
            report.applied_steps[0].compressor,
            Some(CompressorKind::SmartCrusher)
        );
        assert_eq!(
            report
                .bloat_estimate
                .map(|estimate| estimate.reason.as_str()),
            Some("json_rows")
        );
        assert_eq!(report.ccr_tokens, vec![res.ccr_token.unwrap()]);
        assert_eq!(report.skip_reason, None);
    }

    #[tokio::test]
    async fn report_records_redacted_skip_reason() {
        let mut o = opts();
        o.router_enabled = false;
        let (res, report) = compress_content_with_store_report(
            "sensitive raw payload",
            None,
            &o,
            &cache::GlobalCcrStore,
        )
        .await;

        assert!(!res.applied);
        assert_eq!(report.skip_reason, Some(PipelineSkipReason::RouterDisabled));
        assert!(report.bloat_estimate.is_some());
        assert!(report.applied_steps.is_empty());
        assert!(report.ccr_tokens.is_empty());
    }

    #[tokio::test]
    async fn shell_policy_keeps_exact_file_read_raw_even_with_extension_hint() {
        let content = (0..120)
            .map(|i| format!("fn generated_{i}() {{ println!(\"{i}\"); }}"))
            .collect::<Vec<_>>()
            .join("\n");
        let hint = ContentHint {
            source_tool: Some("shell".to_owned()),
            extension: Some("rs".to_owned()),
            ..Default::default()
        };
        let input = CompressInput {
            content: &content,
            kind: ContentKind::PlainText,
            hint: &hint,
            exit_code: None,
            command: Some("cat src/lib.rs".to_owned()),
            argv: None,
            original_bytes: content.len(),
        };

        let store = cache::MemoryCcrStore::new(10, 1_000_000);
        let (res, report) = route_with_store_report(input, &opts(), &store).await;

        assert!(!res.applied);
        assert_eq!(res.text, content);
        assert_eq!(
            report.skip_reason,
            Some(PipelineSkipReason::Other("skip_file_content"))
        );
    }

    #[tokio::test]
    async fn lossy_output_declines_when_injected_store_cannot_retain() {
        let store = cache::MemoryCcrStore::new(10, 16);
        let mut rows = Vec::new();
        for i in 0..120 {
            rows.push(format!(
                r#"{{"id":{i},"name":"account_{i}","email":"a{i}@ex.com","tier":"gold"}}"#
            ));
        }
        let original = format!("[{}]", rows.join(","));
        let res = compress_content_with_store(&original, None, &opts(), &store).await;

        assert!(!res.applied);
        assert_eq!(res.text, original);
        assert!(res.ccr_token.is_none());
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
