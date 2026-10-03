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
    if opts.repl_handle
        && opts.ccr_enabled
        && estimate_tokens_with(content, opts.chars_per_token) as usize >= opts.ccr_min_tokens
    {
        // Size-driven, so it runs ahead of the low-bloat skip.
        let put = store.put(content);
        if put.retained() {
            let token = put.token().to_string();
            let pre_existing = opts
                .repl_save_dir
                .as_deref()
                .is_some_and(|dir| dir.join(format!("{token}.txt")).is_file());
            let saved = opts
                .repl_save_dir
                .as_deref()
                .and_then(|dir| crate::repl::write_handle_file(dir, &token, content));
            let (body, footer, stats) = crate::repl::handle_view(
                content,
                &token,
                kind,
                opts.chars_per_token,
                opts.repl_preview_chars,
                saved.as_deref(),
            );
            let text = format!("{body}{footer}");
            if text.len() < original_bytes {
                let compacted_bytes = text.len();
                let res = CompressedOutput {
                    text,
                    body,
                    recovery_footer: Some(footer),
                    content_kind: kind,
                    compressor: crate::types::CompressorKind::Repl,
                    lossy: true,
                    applied: true,
                    ccr_token: Some(token),
                    original_bytes,
                    compacted_bytes,
                    stats: Some(stats),
                    saved_path: saved,
                };
                let report = PipelineReport::applied(
                    kind,
                    original_bytes,
                    compacted_bytes,
                    crate::types::CompressorKind::Repl,
                    true,
                    res.ccr_token.clone(),
                )
                .with_bloat_estimate(bloat_estimate);
                return (res, report);
            }
            // View rejected: do not leave an unreferenced sensitive copy behind.
            if let (Some(path), false) = (saved.as_deref(), pre_existing) {
                let _ = std::fs::remove_file(path);
            }
        }
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
            let footer =
                cache::recovery_footer_with(&token, original_bytes, out.lossy, opts.repl_handle);
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
        stats: None,
        saved_path: None,
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

    finalize_typed_output(input, typed, bloat_estimate, original_tokens, opts.repl_handle)
}

fn finalize_typed_output(
    input: &CompressInput<'_>,
    typed: TypedPipelineOutput,
    bloat_estimate: BloatEstimate,
    original_tokens: u64,
    repl: bool,
) -> Option<(CompressedOutput, PipelineReport)> {
    let original_bytes = input.content.len();
    if typed.report.applied_steps.is_empty() || typed.text.len() >= original_bytes {
        return None;
    }

    let (body, recovery_footer, ccr_token) = if typed.lossy {
        let token = typed.ccr_token.clone()?;
        let footer = cache::recovery_footer_with(&token, original_bytes, true, repl);
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
        stats: None,
        saved_path: None,
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
#[path = "compress_tests.rs"]
mod tests;
