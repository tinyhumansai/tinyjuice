//! Glue between the agent tool loop and the TokenJuice content router.
//!
//! Exposes the entry points the agent loop calls after a tool returns output:
//!
//! - [`compact_tool_output_with_policy`] — full version with the tool's JSON
//!   arguments and exit code; derives a command/argv and content hint, routes
//!   through the content router, and returns `(text, CompactionStats)`.
//! - [`compact_output`] — minimal version (content + tool name + enable flag)
//!   for call sites that only have those, returning just the text.
//!
//! Both are **pass-through safe**: if compression doesn't meaningfully shrink
//! the payload, or the input is under the byte floor, or the router/CCR is
//! disabled, the original string is returned untouched.
//!
//! Runtime options (the `[tinyjuice]` config block) are installed once at
//! startup via [`configure`]; callers don't thread `Config` through.

use once_cell::sync::OnceCell;
use serde_json::Value;
use std::sync::RwLock;

use super::compress::route;
use super::types::{AgentTokenjuiceCompression, CompressInput, CompressOptions, ContentHint};

/// Skip compaction for outputs smaller than this (bytes) by default. Tiny
/// outputs have no headroom and risk distortion. Overridable per the config's
/// `min_bytes_to_compress` once [`configure`] runs.
const DEFAULT_MIN_COMPACT_INPUT_BYTES: usize = 512;

/// Process-global runtime options, installed from config at startup.
fn options_cell() -> &'static RwLock<CompressOptions> {
    static OPTS: OnceCell<RwLock<CompressOptions>> = OnceCell::new();
    OPTS.get_or_init(|| {
        RwLock::new(CompressOptions {
            min_bytes_to_compress: DEFAULT_MIN_COMPACT_INPUT_BYTES,
            ..Default::default()
        })
    })
}

/// Install the runtime [`CompressOptions`] (called once from config at startup).
/// Also configures the CCR cache limits/disk tier indirectly via the caller.
pub fn configure(opts: CompressOptions) {
    *options_cell().write().unwrap_or_else(|p| p.into_inner()) = opts;
}

/// Snapshot the current runtime options.
pub fn current_options() -> CompressOptions {
    options_cell()
        .read()
        .unwrap_or_else(|p| p.into_inner())
        .clone()
}

fn options_for_agent(profile: AgentTokenjuiceCompression) -> Result<CompressOptions, &'static str> {
    match profile {
        AgentTokenjuiceCompression::Off => Err("none/agent-profile-off"),
        AgentTokenjuiceCompression::Auto => Err("none/agent-profile-auto-unresolved"),
        AgentTokenjuiceCompression::Full => Ok(current_options()),
        AgentTokenjuiceCompression::Light => {
            let mut opts = current_options();
            // Coding agents need raw, exact tool text more than aggressive
            // token savings. With CCR off and lossy-without-CCR disallowed,
            // every lossy compressor declines in route(), while any truly
            // lossless reduction is still allowed.
            opts.ccr_enabled = false;
            opts.lossy_without_ccr = false;
            opts.ml_text_enabled = false;
            Ok(opts)
        }
    }
}

/// Install the full TokenJuice runtime configuration in one call at startup:
/// router/compressor options, CCR cache limits, and the optional on-disk tier.
/// Kept free of the config-schema type so `tinyjuice` stays decoupled — the
/// caller maps `Config.tinyjuice` into these primitives.
#[allow(clippy::too_many_arguments)]
pub fn install_config(
    options: CompressOptions,
    max_cache_entries: usize,
    max_cache_bytes: usize,
    ccr_ttl_secs: Option<u64>,
    disk_tier_root: Option<std::path::PathBuf>,
) {
    configure(options);
    super::cache::configure(max_cache_entries, max_cache_bytes, ccr_ttl_secs);
    // Enable or disable the disk tier to match the setting — a `None` here means
    // the user turned it off, so clear any previously-installed disk root rather
    // than leaving the process writing originals to disk until restart.
    match disk_tier_root {
        Some(root) => super::cache::enable_disk_tier(root),
        None => super::cache::disable_disk_tier(),
    }
    log::debug!("[tinyjuice] runtime config installed");
}

/// Statistics for a single compaction call (back-compat shape).
#[derive(Debug, Clone)]
pub struct CompactionStats {
    pub tool_name: String,
    pub original_bytes: usize,
    pub compacted_bytes: usize,
    /// The compressor kind (or `none/...`) that handled the output.
    pub rule_id: String,
    pub applied: bool,
}

impl CompactionStats {
    pub fn ratio(&self) -> f64 {
        if self.original_bytes == 0 {
            1.0
        } else {
            self.compacted_bytes as f64 / self.original_bytes as f64
        }
    }
}

/// Compact a tool call's output using an agent-level TokenJuice profile.
///
/// * `tool_name` — the agent-level tool name (`shell`, `grep`, `browser_navigate`).
/// * `arguments` — the raw JSON arguments; used to derive command/argv (for the
///   log/command rule path) and a file extension (for code/JSON/HTML hints).
/// * `output` — the captured tool output (already credential-scrubbed).
/// * `exit_code` — enables failure-preserving behaviour in the log compressor.
///
/// Returns `(text, stats)`. When `stats.applied == false` the text is the
/// untouched original.
pub async fn compact_tool_output_with_policy(
    tool_name: &str,
    arguments: Option<&Value>,
    output: &str,
    exit_code: Option<i32>,
    profile: AgentTokenjuiceCompression,
) -> (String, CompactionStats) {
    let report = compact_tool_output(ToolOutputCall {
        tool_name,
        arguments,
        output,
        exit_code,
        profile,
        compaction_enabled: true,
        focus: None,
        context_token: None,
        scope: None,
    })
    .await;
    (report.text, report.stats)
}

/// Everything [`compact_tool_output`] considers about one tool result.
#[derive(Debug, Clone, Copy)]
pub struct ToolOutputCall<'a> {
    pub tool_name: &'a str,
    /// The tool call's JSON arguments.
    pub arguments: Option<&'a Value>,
    /// The captured tool output (already credential-scrubbed).
    pub output: &'a str,
    pub exit_code: Option<i32>,
    pub profile: AgentTokenjuiceCompression,
    /// Whether the content router runs. `false` still allows the summary
    /// stage: a host can want a model-written summary of an oversized result
    /// without opting its agents into deterministic compaction.
    pub compaction_enabled: bool,
    /// What the caller said it needs from this result.
    pub focus: Option<&'a str>,
    /// Handed back to the host's `Generate`; `None` skips the summary stage.
    pub context_token: Option<&'a str>,
    /// Scopes summary reuse and the failure breaker.
    pub scope: Option<&'a str>,
}

/// What [`compact_tool_output`] produced.
#[derive(Debug, Clone)]
pub struct ToolOutputReport {
    pub text: String,
    pub stats: CompactionStats,
    /// A model-facing notice the host should prefix after its own caps: the
    /// summary stage applied and did not produce a summary.
    pub notice: Option<&'static str>,
}

/// Compact one tool result: the LLM summary stage first (the full profile, or
/// the light profile when the host passed a context token), then the content
/// router.
///
/// A successful summary is final — it already carries the recovery footer for
/// the original, and routing a model-written note through compressors built
/// for machine output would only damage it.
pub async fn compact_tool_output(call: ToolOutputCall<'_>) -> ToolOutputReport {
    compact_tool_output_inner(call, None).await
}

/// [`compact_tool_output`] with an optional explicit option set, so a caller
/// (or test) need not mutate the process-wide options.
async fn compact_tool_output_inner(
    call: ToolOutputCall<'_>,
    opts_override: Option<CompressOptions>,
) -> ToolOutputReport {
    let ToolOutputCall {
        tool_name,
        arguments,
        output,
        exit_code,
        profile,
        compaction_enabled,
        focus,
        context_token,
        scope,
    } = call;
    let original_bytes = output.len();

    let opts = match opts_override.map_or_else(|| options_for_agent(profile), Ok) {
        Ok(opts) => opts,
        Err(rule_id) => {
            log::debug!(
                "[tinyjuice] agent profile skipped compaction tool={} profile={} bytes={}",
                tool_name,
                profile.as_str(),
                original_bytes
            );
            return ToolOutputReport {
                text: output.to_string(),
                stats: CompactionStats {
                    tool_name: tool_name.to_string(),
                    original_bytes,
                    compacted_bytes: original_bytes,
                    rule_id: rule_id.to_string(),
                    applied: false,
                },
                notice: None,
            };
        }
    };

    // A recovery tool's output is the original we previously offloaded — never
    // re-compact it, or the agent could never see the full data it asked for.
    if super::cache::is_recovery_tool(tool_name) {
        return ToolOutputReport {
            text: output.to_string(),
            stats: CompactionStats {
                tool_name: tool_name.to_string(),
                original_bytes,
                compacted_bytes: original_bytes,
                rule_id: "none/recovery-tool".to_string(),
                applied: false,
            },
            notice: None,
        };
    }

    // The summary stage. `Full` always considers it. `Light` keeps exact tool
    // text from the router's compressors, but a host that bound a summary call
    // to this result (a context token) asked for one explicitly — OpenHuman
    // does so only for its orchestrator's oversized results — so it runs there
    // too, with CCR on: the summary is lossy, and the exact original must stay
    // retrievable. Without a token, `Light` never reaches the model.
    let mut notice = None;
    let mut repl_fallback = false;
    let summary_opts = match profile {
        AgentTokenjuiceCompression::Full => Some(opts.clone()),
        AgentTokenjuiceCompression::Light if context_token.is_some() => {
            let mut summary_opts = opts.clone();
            summary_opts.ccr_enabled = current_options().ccr_enabled;
            Some(summary_opts)
        }
        _ => None,
    };
    // An explicit handle mode is honored before any model call: an eligible
    // output gets its preview and handle rather than waiting on a summary.
    let handle_mode_applies = opts.repl_handle
        && opts.ccr_enabled
        && compaction_enabled
        && crate::tokens::estimate_tokens_with(output, opts.chars_per_token) as usize
            >= opts.ccr_min_tokens;
    let summary_opts = summary_opts.filter(|_| !handle_mode_applies);
    if let Some(summary_opts) = summary_opts {
        let outcome = super::summarize::maybe_summarize(
            super::summarize::SummaryInput {
                tool_name,
                content: output,
                focus,
                context_token,
                scope,
            },
            &summary_opts,
        )
        .await;
        match outcome {
            super::summarize::SummaryOutcome::Summarized { text, .. } => {
                let compacted_bytes = text.len();
                return ToolOutputReport {
                    text,
                    stats: CompactionStats {
                        tool_name: tool_name.to_string(),
                        original_bytes,
                        compacted_bytes,
                        rule_id: super::types::CompressorKind::LlmSummary
                            .as_str()
                            .to_string(),
                        applied: true,
                    },
                    notice: None,
                };
            }
            super::summarize::SummaryOutcome::NotNeeded => {}
            super::summarize::SummaryOutcome::Unavailable(reason) => {
                notice = Some(reason.notice());
                // A slow model must not turn into an unbounded dump: keep the
                // original in CCR and hand back a preview plus a handle.
                repl_fallback = reason == super::summarize::UnavailableReason::TimedOut;
            }
        }
    }

    if !compaction_enabled {
        return ToolOutputReport {
            text: output.to_string(),
            stats: CompactionStats {
                tool_name: tool_name.to_string(),
                original_bytes,
                compacted_bytes: original_bytes,
                rule_id: "none/disabled".to_string(),
                applied: false,
            },
            notice,
        };
    }

    let (command, argv) = extract_command_argv(arguments);
    let hint = ContentHint {
        source_tool: Some(tool_name.to_string()),
        extension: extract_extension(arguments),
        // With no query in the arguments, the caller's focus is the best
        // statement of what to keep, and the text ranker already reads it.
        query: extract_query(arguments).or_else(|| {
            focus
                .map(str::trim)
                .filter(|f| !f.is_empty())
                .map(str::to_string)
        }),
        ..Default::default()
    };

    let input = CompressInput {
        content: output,
        kind: super::types::ContentKind::PlainText,
        hint: &hint,
        exit_code,
        command,
        argv,
        original_bytes,
    };

    let mut opts = opts;
    if repl_fallback {
        opts.repl_handle = true;
        // The summary stage may have run below the CCR floor; the fallback must
        // still produce a handle.
        opts.ccr_min_tokens = 0;
        opts.ccr_enabled = current_options().ccr_enabled;
    }
    let res = route(input, &opts).await;
    let stats = CompactionStats {
        tool_name: tool_name.to_string(),
        original_bytes,
        compacted_bytes: res.compacted_bytes,
        rule_id: if res.applied {
            res.compressor.as_str().to_string()
        } else {
            format!("none/{}", res.content_kind.as_str())
        },
        applied: res.applied,
    };
    ToolOutputReport {
        text: res.text,
        stats,
        notice,
    }
}

/// Minimal compaction for call sites that only have content + tool name. The
/// `enabled` flag is an explicit kill-switch on top of the configured options.
pub async fn compact_output(content: String, tool_name: &str, enabled: bool) -> String {
    compact_output_with_policy(
        content,
        tool_name,
        enabled,
        AgentTokenjuiceCompression::Full,
    )
    .await
}

/// Minimal compaction with an agent-level TokenJuice profile.
pub async fn compact_output_with_policy(
    content: String,
    tool_name: &str,
    enabled: bool,
    profile: AgentTokenjuiceCompression,
) -> String {
    // The call-site `enabled` flag and the configured router switch are both
    // hard off-switches; either one short-circuits to the untouched original.
    if !enabled || !current_options().router_enabled {
        return content;
    }
    let (text, _stats) =
        compact_tool_output_with_policy(tool_name, None, &content, None, profile).await;
    text
}

/// Derive `(command, argv)` from a tool's JSON arguments.
fn extract_command_argv(arguments: Option<&Value>) -> (Option<String>, Option<Vec<String>>) {
    let Some(Value::Object(map)) = arguments else {
        return (None, None);
    };

    if let Some(Value::Array(arr)) = map.get("argv") {
        let argv: Vec<String> = arr
            .iter()
            .filter_map(|v| v.as_str().map(|s| s.to_owned()))
            .collect();
        if !argv.is_empty() {
            let command = argv.join(" ");
            return (Some(command), Some(argv));
        }
    }

    let cmd_str = map
        .get("command")
        .and_then(Value::as_str)
        .or_else(|| map.get("cmd").and_then(Value::as_str));

    if let Some(cmd) = cmd_str {
        if let Some(Value::Array(args)) = map.get("args") {
            let mut argv = vec![cmd.to_owned()];
            argv.extend(args.iter().filter_map(|v| v.as_str().map(|s| s.to_owned())));
            return (Some(format!("{cmd} {}", argv[1..].join(" "))), Some(argv));
        }
        let argv: Vec<String> = cmd.split_whitespace().map(|s| s.to_owned()).collect();
        return (Some(cmd.to_owned()), (!argv.is_empty()).then_some(argv));
    }

    (None, None)
}

/// Derive a file extension hint from common path-bearing argument shapes.
fn extract_extension(arguments: Option<&Value>) -> Option<String> {
    let Some(Value::Object(map)) = arguments else {
        return None;
    };
    let path = ["path", "file_path", "file", "filename"]
        .iter()
        .find_map(|k| map.get(*k).and_then(Value::as_str))?;
    let ext = std::path::Path::new(path)
        .extension()
        .and_then(|e| e.to_str())?;
    Some(ext.to_ascii_lowercase())
}

/// Derive a search-query hint from common query-bearing argument shapes.
fn extract_query(arguments: Option<&Value>) -> Option<String> {
    let Some(Value::Object(map)) = arguments else {
        return None;
    };
    ["query", "pattern", "search", "q", "regex"]
        .iter()
        .find_map(|k| map.get(*k).and_then(Value::as_str))
        .map(str::to_string)
}

#[cfg(test)]
#[path = "tool_integration_tests.rs"]
mod tests;
