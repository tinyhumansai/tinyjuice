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
use crate::compressors::{
    code::CodeStubTransform,
    compressor_for,
    diff::DiffNoiseTransform,
    generic_compressor,
    html::HtmlExtractTransform,
    json::{SmartCrusherRowsTransform, SmartCrusherTableTransform},
    log::{LogTemplateTransform, SignalLogTransform},
    search::SearchTransform,
    text::TextCrusherTransform,
};
use crate::detect::detect_content_kind;
use crate::pipeline::{
    BloatEstimate, OffloadTransform, PipelineInput, PipelineReport, PipelineSkipReason,
    ReformatTransform, TypedPipelineOutput, estimate_bloat, run_typed_pipeline,
};
use crate::policy::{ShellCompactionPolicy, ShellPolicyDecision, apply_shell_compaction_policy};
use crate::savings;
use crate::tokens::estimate_tokens_with;
use crate::types::{
    CompressInput, CompressOptions, CompressOutput, CompressedOutput, ContentHint, ContentKind,
    ReadIntent,
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

/// Policy-aware variant of [`route`] for hosts that expose shell-policy config.
pub async fn route_with_shell_policy(
    input: CompressInput<'_>,
    opts: &CompressOptions,
    shell_policy: ShellCompactionPolicy,
) -> CompressedOutput {
    route_with_store_report_shell_policy(input, opts, &cache::GlobalCcrStore, shell_policy)
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

/// Store-injected, policy-aware variant of [`route`].
pub async fn route_with_store_shell_policy(
    input: CompressInput<'_>,
    opts: &CompressOptions,
    store: &dyn CcrStore,
    shell_policy: ShellCompactionPolicy,
) -> CompressedOutput {
    route_with_store_report_shell_policy(input, opts, store, shell_policy)
        .await
        .0
}

/// Store-injected variant of [`route`] that also returns a redacted pipeline
/// report.
pub async fn route_with_store_report(
    input: CompressInput<'_>,
    opts: &CompressOptions,
    store: &dyn CcrStore,
) -> (CompressedOutput, PipelineReport) {
    route_with_store_report_shell_policy(
        input,
        opts,
        store,
        ShellCompactionPolicy::AllowSafeInventory,
    )
    .await
}

/// Store-injected, policy-aware variant of [`route_with_store_report`].
pub async fn route_with_store_report_shell_policy(
    mut input: CompressInput<'_>,
    opts: &CompressOptions,
    store: &dyn CcrStore,
    shell_policy: ShellCompactionPolicy,
) -> (CompressedOutput, PipelineReport) {
    let content = input.content;
    let original_bytes = content.len();

    let log_floor = opts
        .min_bytes_to_compress_log
        .min(opts.min_bytes_to_compress);
    if !opts.router_enabled || original_bytes < log_floor {
        let kind = if opts.router_enabled {
            input.hint.explicit.unwrap_or(ContentKind::PlainText)
        } else {
            detect_content_kind(content, input.hint)
        };
        let res = CompressedOutput::passthrough(content.to_string(), kind);
        let skip = if opts.router_enabled {
            PipelineSkipReason::BelowMinBytes
        } else {
            PipelineSkipReason::RouterDisabled
        };
        let report = PipelineReport::passthrough(kind, original_bytes, skip)
            .with_bloat_estimate(estimate_bloat(content, kind));
        return (res, report);
    }

    let kind = detect_content_kind(content, input.hint);
    let bloat_estimate = estimate_bloat(content, kind);
    if is_exact_file_read(input.hint) {
        let res = CompressedOutput::passthrough(content.to_string(), kind);
        let report = PipelineReport::passthrough(
            kind,
            original_bytes,
            PipelineSkipReason::Other("exact_file_read"),
        )
        .with_bloat_estimate(bloat_estimate);
        return (res, report);
    }
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
        shell_policy,
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
    if should_skip_low_bloat(&input, bloat_estimate.score) {
        let res = CompressedOutput::passthrough(content.to_string(), kind);
        let report =
            PipelineReport::passthrough(kind, original_bytes, PipelineSkipReason::LowBloat)
                .with_bloat_estimate(bloat_estimate);
        return (res, report);
    }
    input.kind = kind;
    let original_tokens = estimate_tokens_with(content, opts.chars_per_token);
    let ccr_for_typed = opts.ccr_enabled && original_tokens as usize >= opts.ccr_min_tokens;

    if let Some((res, report)) = try_typed_route(
        &input,
        opts,
        store,
        bloat_estimate,
        original_tokens,
        ccr_for_typed,
    ) {
        return (res, report);
    }

    // Between the log floor and the global floor only log-like content
    // proceeds: detected logs, or command output headed for the rule engine.
    // Everything else keeps the historical global floor.
    if original_bytes < opts.min_bytes_to_compress {
        let log_like = kind == ContentKind::Log || input.command.is_some();
        if !log_like || original_bytes < opts.min_bytes_to_compress_log {
            let res = CompressedOutput::passthrough(content.to_string(), kind);
            let report = PipelineReport::passthrough(
                kind,
                original_bytes,
                PipelineSkipReason::BelowMinBytes,
            )
            .with_bloat_estimate(bloat_estimate);
            return (res, report);
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

    let compacted_body_tokens = estimate_tokens_with(&out.text, opts.chars_per_token);
    let heavy_crush = compacted_body_tokens <= original_tokens / 2
        && original_tokens as usize >= opts.ccr_min_tokens / 4;
    let ccr_for_call =
        opts.ccr_enabled && (original_tokens as usize >= opts.ccr_min_tokens || heavy_crush);
    if out.lossy && !ccr_for_call && !opts.lossy_without_ccr {
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
    // result is being compressed for). Token counts are estimated; byte counts
    // are measured directly from this reducer invocation.
    savings::record_event(
        savings::SavingsRecord::estimated_compaction(
            kind,
            out.kind,
            original_tokens,
            compacted_tokens,
        )
        .with_bytes(original_bytes as u64, compacted_bytes as u64)
        .with_lossy(out.lossy)
        .with_ccr_token_present(ccr_token.is_some()),
    );

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

fn try_typed_route(
    input: &CompressInput<'_>,
    opts: &CompressOptions,
    store: &dyn CcrStore,
    bloat_estimate: BloatEstimate,
    original_tokens: u64,
    ccr_for_call: bool,
) -> Option<(CompressedOutput, PipelineReport)> {
    if has_command_context(input) {
        return None;
    }

    let pipeline_input = PipelineInput::from(input);
    let typed = match input.kind {
        ContentKind::Json => {
            let table = SmartCrusherTableTransform;
            let rows = input
                .hint
                .query
                .as_deref()
                .filter(|query| !query.trim().is_empty())
                .map(|query| SmartCrusherRowsTransform::new().with_query(query))
                .unwrap_or_default();
            let reformats: [&dyn ReformatTransform; 1] = [&table];
            let offloads: Vec<&dyn OffloadTransform> = if ccr_for_call {
                vec![&rows]
            } else {
                Vec::new()
            };
            run_typed_pipeline(pipeline_input, &reformats, &offloads, store)
        }
        ContentKind::Diff => {
            if !ccr_for_call {
                return None;
            }
            let transform = DiffNoiseTransform::default();
            let reformats: [&dyn ReformatTransform; 0] = [];
            let offloads: [&dyn OffloadTransform; 1] = [&transform];
            run_typed_pipeline(pipeline_input, &reformats, &offloads, store)
        }
        ContentKind::Log => {
            let template = LogTemplateTransform;
            let signal = SignalLogTransform;
            let reformats: [&dyn ReformatTransform; 1] = [&template];
            let offloads: Vec<&dyn OffloadTransform> = if ccr_for_call {
                vec![&signal]
            } else {
                Vec::new()
            };
            run_typed_pipeline(pipeline_input, &reformats, &offloads, store)
        }
        ContentKind::Search => {
            if !opts.search_enabled || !ccr_for_call {
                return None;
            }
            let transform = input
                .hint
                .query
                .as_deref()
                .filter(|query| !query.trim().is_empty())
                .map(|query| SearchTransform::new().with_query(query))
                .unwrap_or_default();
            let reformats: [&dyn ReformatTransform; 0] = [];
            let offloads: [&dyn OffloadTransform; 1] = [&transform];
            run_typed_pipeline(pipeline_input, &reformats, &offloads, store)
        }
        ContentKind::Html => {
            if !opts.html_enabled || !ccr_for_call {
                return None;
            }
            let transform = HtmlExtractTransform;
            let reformats: [&dyn ReformatTransform; 0] = [];
            let offloads: [&dyn OffloadTransform; 1] = [&transform];
            run_typed_pipeline(pipeline_input, &reformats, &offloads, store)
        }
        ContentKind::Code => {
            if !opts.code_enabled || !ccr_for_call {
                return None;
            }
            let ReadIntent::Stub(mode) = &input.hint.read_intent else {
                return None;
            };
            let mut transform = CodeStubTransform::new(mode.clone());
            if let Some(extension) = input.hint.extension.as_deref() {
                transform = transform.with_extension(extension);
            }
            let reformats: [&dyn ReformatTransform; 0] = [];
            let offloads: [&dyn OffloadTransform; 1] = [&transform];
            run_typed_pipeline(pipeline_input, &reformats, &offloads, store)
        }
        ContentKind::PlainText => {
            if opts.ml_text_enabled || !ccr_for_call {
                return None;
            }
            let transform = input
                .hint
                .query
                .as_deref()
                .filter(|query| !query.trim().is_empty())
                .map(|query| TextCrusherTransform::new(opts.clone()).with_query(query))
                .unwrap_or_else(|| TextCrusherTransform::new(opts.clone()));
            let reformats: [&dyn ReformatTransform; 0] = [];
            let offloads: [&dyn OffloadTransform; 1] = [&transform];
            run_typed_pipeline(pipeline_input, &reformats, &offloads, store)
        }
    };

    finalize_typed_output(input, typed, bloat_estimate, original_tokens)
}

fn finalize_typed_output(
    input: &CompressInput<'_>,
    typed: TypedPipelineOutput,
    bloat_estimate: BloatEstimate,
    original_tokens: u64,
) -> Option<(CompressedOutput, PipelineReport)> {
    let original_bytes = input.content.len();
    if typed.report.applied_steps.is_empty() || typed.text.len() >= original_bytes {
        return None;
    }

    let (body, recovery_footer, ccr_token) = if typed.lossy {
        let token = typed.ccr_token.clone()?;
        let footer = cache::recovery_footer(&token, original_bytes, true);
        let mut text = typed.text.clone();
        text.push_str(&footer);
        if text.len() >= original_bytes {
            let res = CompressedOutput::passthrough(input.content.to_string(), input.kind);
            let report = PipelineReport::passthrough(
                input.kind,
                original_bytes,
                PipelineSkipReason::FooterWouldGrow,
            )
            .with_bloat_estimate(bloat_estimate);
            return Some((res, report));
        }
        (typed.text, Some(footer), Some(token))
    } else {
        (typed.text, None, None)
    };

    let text = if let Some(footer) = recovery_footer.as_deref() {
        let mut text = body.clone();
        text.push_str(footer);
        text
    } else {
        body.clone()
    };

    let compacted_bytes = text.len();
    let compacted_tokens = estimate_tokens_with(&text, 4.0);
    log::info!(
        "[tokenjuice] compacted kind={} compressor={} lossy={} {}->{} bytes (~{}->{} tok)",
        input.kind.as_str(),
        typed.kind.as_str(),
        typed.lossy,
        original_bytes,
        compacted_bytes,
        original_tokens,
        compacted_tokens,
    );

    savings::record_event(
        savings::SavingsRecord::estimated_compaction(
            input.kind,
            typed.kind,
            original_tokens,
            compacted_tokens,
        )
        .with_bytes(original_bytes as u64, compacted_bytes as u64)
        .with_lossy(typed.lossy)
        .with_ccr_token_present(ccr_token.is_some()),
    );

    let mut report = typed.report;
    report.compacted_bytes = compacted_bytes;
    report.bloat_estimate = Some(bloat_estimate);
    report.ccr_tokens = ccr_token.clone().into_iter().collect();
    report.lossy = typed.lossy;
    report.skip_reason = None;

    let res = CompressedOutput {
        text,
        body,
        recovery_footer,
        content_kind: input.kind,
        compressor: typed.kind,
        lossy: typed.lossy,
        applied: true,
        ccr_token,
        original_bytes,
        compacted_bytes,
    };
    Some((res, report))
}

/// Build a [`CompressorKind`] label-free passthrough quickly (used by callers
/// that only need to detect without compressing).
pub fn detect_only(content: &str, hint: &ContentHint) -> ContentKind {
    detect_content_kind(content, hint)
}

fn is_exact_file_read(hint: &ContentHint) -> bool {
    matches!(hint.read_intent, crate::types::ReadIntent::Exact)
        && hint
            .source_tool
            .as_deref()
            .is_some_and(|tool| matches!(tool, "file_read" | "read_file" | "fs_read"))
}

fn should_skip_low_bloat(input: &CompressInput<'_>, bloat_score: u8) -> bool {
    if bloat_score > 0 {
        return false;
    }
    if has_command_context(input) {
        return false;
    }
    if input
        .hint
        .query
        .as_deref()
        .is_some_and(|query| !query.trim().is_empty())
    {
        return false;
    }
    !matches!(input.hint.read_intent, crate::types::ReadIntent::Stub(_))
}

fn has_command_context(input: &CompressInput<'_>) -> bool {
    input.command.is_some() || input.argv.as_ref().is_some_and(|argv| !argv.is_empty())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ShellCompactionPolicy;
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
            res.text.contains("tinyjuice_retrieve"),
            "footer marker present: {}",
            res.text
        );
    }

    #[tokio::test]
    async fn diff_noise_output_offloads_original_for_recovery() {
        let mut original = String::from("diff --git a/Cargo.lock b/Cargo.lock\n");
        original.push_str("@@ -1,800 +1,900 @@\n");
        for i in 0..700 {
            original.push_str(&format!("+ new dependency entry {i} checksum {}\n", i * 17));
        }
        for i in 0..350 {
            original.push_str(&format!("- old dependency entry {i} checksum {}\n", i * 19));
        }
        let hint = ContentHint {
            explicit: Some(ContentKind::Diff),
            ..Default::default()
        };
        let store = cache::MemoryCcrStore::new(10, 1_000_000);

        let res = compress_content_with_store(&original, Some(hint), &opts(), &store).await;

        assert!(res.applied);
        assert!(res.lossy);
        assert_eq!(res.content_kind, ContentKind::Diff);
        assert!(res.text.contains("reason=lockfile"), "{}", res.text);
        let token = res.ccr_token.as_deref().expect("offloaded");
        assert_eq!(store.get(token).as_deref(), Some(original.as_str()));
    }

    #[tokio::test]
    async fn textcrusher_output_offloads_original_for_recovery() {
        let mut original = String::new();
        for i in 0..80 {
            original.push_str(&format!(
                "routine service note {i} describes ordinary progress through the worker queue.\n\n"
            ));
        }
        original.push_str(
            "ERROR sync.worker.v2 failed for REQUEST_ID 9F42 after retry 17 in api.worker.example.\n\n",
        );
        for i in 80..160 {
            original.push_str(&format!(
                "routine service note {i} describes ordinary progress through the worker queue.\n\n"
            ));
        }
        let hint = ContentHint {
            explicit: Some(ContentKind::PlainText),
            query: Some("sync.worker.v2 REQUEST_ID".to_string()),
            ..Default::default()
        };
        let store = cache::MemoryCcrStore::new(10, 1_000_000);
        let mut opts = opts();
        opts.ccr_min_tokens = 1;

        let res = compress_content_with_store(&original, Some(hint), &opts, &store).await;

        assert!(res.applied);
        assert!(res.lossy);
        assert_eq!(res.content_kind, ContentKind::PlainText);
        assert_eq!(res.compressor, CompressorKind::TextCrusher);
        assert!(
            res.body.contains("ERROR sync.worker.v2 failed"),
            "{}",
            res.body
        );
        let token = res.ccr_token.as_deref().expect("offloaded");
        assert_eq!(store.get(token).as_deref(), Some(original.as_str()));
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
        assert!(
            !res.body.contains("tinyjuice_retrieve"),
            "body must not contain footer"
        );
        assert!(
            footer.contains("tinyjuice_retrieve"),
            "footer carries marker: {footer}"
        );
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
        assert_eq!(report.applied_steps[0].name, "smartcrusher_rows");
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
    async fn report_records_low_bloat_skip_before_compressor_work() {
        let content = (0..180)
            .map(|i| format!("unique observation {i} has ordinary prose without repeated bulk."))
            .collect::<Vec<_>>()
            .join("\n");
        let hint = ContentHint {
            explicit: Some(ContentKind::PlainText),
            ..Default::default()
        };
        let (res, report) = compress_content_with_store_report(
            &content,
            Some(hint),
            &opts(),
            &cache::GlobalCcrStore,
        )
        .await;

        assert!(!res.applied);
        assert_eq!(res.text, content);
        assert_eq!(report.skip_reason, Some(PipelineSkipReason::LowBloat));
        assert_eq!(
            report
                .bloat_estimate
                .map(|estimate| estimate.reason.as_str()),
            Some("low_signal")
        );
        assert!(report.applied_steps.is_empty());
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
    async fn shell_policy_skip_all_keeps_command_output_raw() {
        let content = "line one\nline two\n".repeat(120);
        let hint = ContentHint {
            source_tool: Some("shell".to_owned()),
            ..Default::default()
        };
        let input = CompressInput {
            content: &content,
            kind: ContentKind::PlainText,
            hint: &hint,
            exit_code: None,
            command: Some("rg --files".to_owned()),
            argv: None,
            original_bytes: content.len(),
        };

        let store = cache::MemoryCcrStore::new(10, 1_000_000);
        let (res, report) = route_with_store_report_shell_policy(
            input,
            &opts(),
            &store,
            ShellCompactionPolicy::SkipAll,
        )
        .await;

        assert!(!res.applied);
        assert_eq!(res.text, content);
        assert_eq!(
            report.skip_reason,
            Some(PipelineSkipReason::Other("skip_all"))
        );
    }

    #[tokio::test]
    async fn shell_policy_compact_all_allows_file_content_commands() {
        let mut rows = Vec::new();
        for i in 0..120 {
            rows.push(format!(
                r#"{{"id":{i},"name":"account_{i}","email":"a{i}@ex.com","tier":"gold"}}"#
            ));
        }
        let content = format!("[{}]", rows.join(","));
        let hint = ContentHint {
            source_tool: Some("shell".to_owned()),
            extension: Some("json".to_owned()),
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

        let mut opts = opts();
        opts.ccr_min_tokens = 1;
        let store = cache::MemoryCcrStore::new(10, 1_000_000);
        let (res, report) = route_with_store_report_shell_policy(
            input,
            &opts,
            &store,
            ShellCompactionPolicy::CompactAll,
        )
        .await;

        assert!(res.applied);
        assert_eq!(res.content_kind, ContentKind::Json);
        assert_ne!(res.text, content);
        assert_eq!(report.skip_reason, None);
    }

    #[tokio::test]
    async fn file_read_hint_is_exact_by_default_even_with_code_extension() {
        let content = (0..120)
            .map(|i| format!("pub fn generated_{i}() {{ println!(\"{i}\"); }}"))
            .collect::<Vec<_>>()
            .join("\n");
        let hint = ContentHint {
            source_tool: Some("file_read".to_owned()),
            extension: Some("rs".to_owned()),
            ..Default::default()
        };
        let store = cache::MemoryCcrStore::new(10, 1_000_000);
        let (res, report) =
            compress_content_with_store_report(&content, Some(hint), &opts(), &store).await;

        assert!(!res.applied);
        assert_eq!(res.text, content);
        assert_eq!(
            report.skip_reason,
            Some(PipelineSkipReason::Other("exact_file_read"))
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

    fn big_json_array() -> String {
        let mut rows = Vec::new();
        for i in 0..120 {
            rows.push(format!(
                r#"{{"id":{i},"name":"account_{i}","email":"a{i}@ex.com","tier":"gold"}}"#
            ));
        }
        format!("[{}]", rows.join(","))
    }

    /// A log that the signal path *drops* down to its errors — a genuine
    /// information-dropping payload (not a faithful reshape).
    fn big_log() -> String {
        let mut log = String::new();
        for i in 0..200 {
            if i == 137 {
                log.push_str(&format!(
                    "2026-07-05T09:00:00Z ERROR worker request failed: upstream timeout id={i}\n"
                ));
            } else {
                log.push_str(&format!(
                    "2026-07-05T09:00:00Z INFO worker handled request {i} in 20ms\n"
                ));
            }
        }
        log
    }

    fn log_hint() -> ContentHint {
        ContentHint {
            explicit: Some(ContentKind::Log),
            ..Default::default()
        }
    }

    /// The opt-in escape hatch still works: a host that sets
    /// `lossy_without_ccr = true` gets marked-but-unrecoverable lossy output
    /// when CCR is off.
    #[tokio::test]
    async fn lossy_compression_works_with_opt_in_without_ccr() {
        let mut o = opts();
        o.ccr_enabled = false;
        o.lossy_without_ccr = true; // explicit opt-in, not the default
        let original = big_log();
        let res = compress_content(&original, Some(log_hint()), &o).await;
        assert_eq!(res.content_kind, ContentKind::Log);
        assert!(res.applied, "opt-in lossy compression applies without CCR");
        assert!(res.text.len() < original.len());
        assert!(res.ccr_token.is_none(), "no recovery token without CCR");
        assert!(
            !res.text.contains("⟦tj:"),
            "no dangling footer: {}",
            res.text
        );
    }

    /// The default may still apply lossless reformats when CCR is disabled; it
    /// only declines information-dropping lossy output that cannot be recovered.
    #[tokio::test]
    async fn default_allows_lossless_without_ccr() {
        let mut o = opts();
        o.ccr_enabled = false;
        assert!(!o.lossy_without_ccr, "default is strict");
        let original = big_log();
        let res = compress_content(&original, Some(log_hint()), &o).await;
        assert!(res.applied, "lossless reformat should apply: {}", res.text);
        assert!(!res.lossy, "without CCR output must not drop data");
        assert!(res.ccr_token.is_none());
        assert!(res.text.len() < original.len());
    }

    /// A large JSON array with CCR off declines lossy row sampling under the
    /// strict default, because the sampled-away rows would not be recoverable.
    #[tokio::test]
    async fn json_declines_lossy_sampling_without_ccr() {
        let mut o = opts();
        o.ccr_enabled = false;
        assert!(!o.lossy_without_ccr, "strict default");
        let original = big_json_array();
        let res = compress_content(&original, None, &o).await;
        assert_eq!(res.content_kind, ContentKind::Json);
        assert!(!res.applied, "unrecoverable lossy view declined: {res:?}");
        assert_eq!(res.text, original);
        assert!(res.ccr_token.is_none());
    }

    /// A faithful, information-preserving reshape (HTML extraction here) is not
    /// lossy, so it ships even with CCR off — reshaping is always allowed, only
    /// *dropping* needs a recovery path.
    #[tokio::test]
    async fn faithful_reshape_ships_without_ccr() {
        let mut o = opts();
        o.ccr_enabled = false;
        assert!(!o.lossy_without_ccr);
        let mut html = String::from("<html><head><title>Status</title></head><body>");
        for i in 0..80 {
            html.push_str(&format!(
                "<div class=\"row\"><span>service {i} is healthy</span></div>"
            ));
        }
        html.push_str("</body></html>");
        let hint = ContentHint {
            mime: Some("text/html".into()),
            explicit: Some(ContentKind::Html),
            ..Default::default()
        };
        let res = compress_content(&html, Some(hint), &o).await;
        assert_eq!(res.content_kind, ContentKind::Html);
        assert!(
            res.applied,
            "faithful reshape ships without CCR: {}",
            res.text
        );
        assert!(!res.lossy, "extraction is information-preserving");
        assert!(res.ccr_token.is_none(), "no recovery needed for a reshape");
        // Every service line's text survives the reshape.
        for i in 0..80 {
            assert!(
                res.text.contains(&format!("service {i} is healthy")),
                "row {i} kept: {}",
                res.text
            );
        }
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

    /// Code must never be cut when it can't be recovered: with CCR off and the
    /// default options (`lossy_without_ccr = false`), collapsing a function
    /// body drops information with no retrieval token, so the router passes the
    /// source through verbatim instead.
    #[tokio::test]
    async fn code_is_not_cut_without_ccr() {
        let mut o = opts();
        o.ccr_enabled = false;
        assert!(
            !o.lossy_without_ccr,
            "default declines info-dropping output without CCR"
        );
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
        assert!(
            !tokens.is_empty(),
            "recoverable tokens present: {}",
            res.text
        );
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
    async fn small_json_below_global_floor_can_reformat_losslessly() {
        // Typed lossless reformats are allowed before the global lossy floor.
        let mut rows = Vec::new();
        for i in 0..20 {
            rows.push(format!(
                r#"{{"id":{i},"name":"account_{i}","email":"a{i}@ex.com","tier":"gold"}}"#
            ));
        }
        let original = format!("[{}]", rows.join(","));
        assert!(original.len() > 512 && original.len() < 2048);
        let res = compress_content(&original, None, &CompressOptions::default()).await;
        assert!(res.applied, "small JSON can reformat: {res:?}");
        assert!(!res.lossy);
        assert!(res.ccr_token.is_none());
        assert!(res.text.len() < original.len());
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
    async fn heavy_crush_below_ccr_min_tokens_can_apply_losslessly() {
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
        if res.lossy {
            let token = res.ccr_token.expect("lossy output must offload");
            assert_eq!(cache::retrieve(&token).as_deref(), Some(original.as_str()));
        } else {
            assert!(
                res.ccr_token.is_none(),
                "lossless output should not require recovery"
            );
        }
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
