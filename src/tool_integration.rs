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

    let opts = match options_for_agent(profile) {
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
mod tests {
    use super::*;
    use serde_json::json;

    fn stringified_json_rows() -> String {
        let rows: Vec<_> = (0..80)
            .map(|i| {
                json!({
                    "id": i,
                    "status": "active",
                    "metadata": json!({
                        "owner": format!("team-{i}"),
                        "flags": { "retry": i % 2 == 0 }
                    })
                    .to_string()
                })
            })
            .collect();
        serde_json::to_string_pretty(&rows).expect("rows serialize")
    }

    #[tokio::test]
    async fn skips_short_output() {
        let (out, stats) = compact_tool_output_with_policy(
            "shell",
            None,
            "hello world",
            Some(0),
            AgentTokenjuiceCompression::Full,
        )
        .await;
        assert_eq!(out, "hello world");
        assert!(!stats.applied);
        assert_eq!(stats.original_bytes, 11);
    }

    #[tokio::test]
    async fn compacts_long_git_status_via_argv() {
        let mut lines = vec!["On branch main".to_owned()];
        for i in 0..200 {
            lines.push(format!("\tmodified:   src/file_{i}.rs"));
        }
        let output = lines.join("\n");
        let args = json!({"command": "git status"});
        let (compacted, stats) = compact_tool_output_with_policy(
            "shell",
            Some(&args),
            &output,
            Some(0),
            AgentTokenjuiceCompression::Full,
        )
        .await;
        assert!(stats.applied, "expected compaction, got {:?}", stats);
        assert!(compacted.len() < output.len());
        assert!(
            compacted.contains("M: src/file_0.rs"),
            "git/status rule should rewrite modified paths: {compacted}"
        );
        assert!(
            !compacted.contains("On branch main"),
            "git/status rule should remove branch boilerplate: {compacted}"
        );
    }

    #[tokio::test]
    async fn compacts_stringified_json_through_tool_adapter() {
        let output = stringified_json_rows();
        let args = json!({"path": "accounts.json"});

        let (compacted, stats) = compact_tool_output_with_policy(
            "web_fetch",
            Some(&args),
            &output,
            Some(0),
            AgentTokenjuiceCompression::Full,
        )
        .await;

        assert!(stats.applied, "expected SmartCrusher, got {:?}", stats);
        assert_eq!(stats.rule_id, "smartcrusher");
        assert!(compacted.contains("metadata.owner"), "{compacted}");
        assert!(compacted.contains("metadata.flags.retry"), "{compacted}");
        assert!(compacted.contains("team-7"), "{compacted}");
        assert!(compacted.contains("tinyjuice_retrieve"), "{compacted}");
        assert!(compacted.len() < output.len());
    }

    #[tokio::test]
    async fn passes_through_incompressible_output() {
        let unique_lines: Vec<String> = (0..200)
            .map(|i| format!("unique-payload-chunk-{i}-{}", "x".repeat(30)))
            .collect();
        let output = unique_lines.join("\n");
        let (returned, stats) = compact_tool_output_with_policy(
            "unknown_tool",
            None,
            &output,
            Some(0),
            AgentTokenjuiceCompression::Full,
        )
        .await;
        if !stats.applied {
            assert_eq!(returned, output);
        }
    }

    #[tokio::test]
    async fn disabled_flag_is_passthrough() {
        let big = "x".repeat(5000);
        assert_eq!(compact_output(big.clone(), "grep", false).await, big);
    }

    #[tokio::test]
    async fn light_agent_profile_declines_lossy_ccr_compaction() {
        let mut lines = vec!["On branch main".to_owned()];
        for i in 0..200 {
            lines.push(format!("\tmodified:   src/file_{i}.rs"));
        }
        let output = lines.join("\n");
        let args = json!({"command": "git status"});
        let (returned, stats) = compact_tool_output_with_policy(
            "shell",
            Some(&args),
            &output,
            Some(0),
            AgentTokenjuiceCompression::Light,
        )
        .await;
        assert_eq!(returned, output);
        assert!(!stats.applied);
    }

    #[tokio::test]
    async fn off_agent_profile_bypasses_router() {
        let big = "x".repeat(5000);
        let returned =
            compact_output_with_policy(big.clone(), "grep", true, AgentTokenjuiceCompression::Off)
                .await;
        assert_eq!(returned, big);
    }

    #[tokio::test]
    async fn auto_agent_profile_requires_host_resolution() {
        let mut lines = vec!["On branch main".to_owned()];
        for i in 0..200 {
            lines.push(format!("\tmodified:   src/file_{i}.rs"));
        }
        let output = lines.join("\n");
        let args = json!({"command": "git status"});
        let (returned, stats) = compact_tool_output_with_policy(
            "shell",
            Some(&args),
            &output,
            Some(0),
            AgentTokenjuiceCompression::Auto,
        )
        .await;

        assert_eq!(returned, output);
        assert!(!stats.applied);
        assert_eq!(stats.rule_id, "none/agent-profile-auto-unresolved");
    }

    #[test]
    fn extract_argv_handles_common_shapes() {
        let (cmd, argv) = extract_command_argv(Some(&json!({"command": "git status"})));
        assert_eq!(cmd.as_deref(), Some("git status"));
        assert_eq!(argv.unwrap(), vec!["git", "status"]);

        let (cmd, _) = extract_command_argv(Some(&json!({"command": "cargo", "args": ["test"]})));
        assert_eq!(cmd.as_deref(), Some("cargo test"));
    }

    #[test]
    fn extract_extension_and_query() {
        assert_eq!(
            extract_extension(Some(&json!({"path": "src/lib.rs"}))).as_deref(),
            Some("rs")
        );
        assert_eq!(
            extract_query(Some(&json!({"pattern": "foo bar"}))).as_deref(),
            Some("foo bar")
        );
    }

    /// Turn the summary stage on in the global options. Safe alongside the
    /// other tests here: none of them passes a context token, so the stage
    /// still declines for them.
    fn enable_llm_summary() {
        let mut opts = current_options();
        opts.llm_summary_enabled = true;
        opts.llm_summary_threshold_tokens = 10;
        configure(opts);
    }

    fn call<'a>(
        output: &'a str,
        profile: AgentTokenjuiceCompression,
        focus: Option<&'a str>,
    ) -> ToolOutputCall<'a> {
        ToolOutputCall {
            tool_name: "web_fetch",
            arguments: None,
            output,
            exit_code: None,
            profile,
            compaction_enabled: true,
            focus,
            context_token: Some("turn-7"),
            scope: Some("tool-integration"),
        }
    }

    #[tokio::test]
    async fn the_full_profile_summarizes_with_the_callers_focus() {
        let _guard = crate::llm::callback_test_guard().await;
        enable_llm_summary();
        let seen = std::sync::Arc::new(std::sync::Mutex::new(Vec::<String>::new()));
        let sink = seen.clone();
        crate::llm::configure_callback(Some(std::sync::Arc::new(
            move |request: crate::llm::GenerateRequest| {
                sink.lock().unwrap().push(request.prompt);
                Box::pin(async { Ok(Some("focused note".to_string())) })
            },
        )));

        let output = "integration full profile ".repeat(60);
        let report = compact_tool_output(call(
            &output,
            AgentTokenjuiceCompression::Full,
            Some("the rate limits"),
        ))
        .await;
        assert!(report.text.starts_with("focused note"));
        assert_eq!(report.stats.rule_id, "llm_summary");
        assert!(report.stats.applied);
        assert!(report.notice.is_none());
        assert!(seen.lock().unwrap()[0].contains("Caller focus: the rate limits"));
        crate::llm::configure_callback(None);
    }

    #[tokio::test]
    async fn the_light_profile_never_summarizes_unasked() {
        let _guard = crate::llm::callback_test_guard().await;
        enable_llm_summary();
        crate::llm::configure_callback(Some(std::sync::Arc::new(|_| {
            Box::pin(async { panic!("light without a context token must not reach the model") })
        })));
        let output = "integration light profile ".repeat(60);
        let report = compact_tool_output(ToolOutputCall {
            context_token: None,
            ..call(&output, AgentTokenjuiceCompression::Light, Some("anything"))
        })
        .await;
        assert_ne!(report.stats.rule_id, "llm_summary");
        crate::llm::configure_callback(None);
    }

    /// A host that binds a summary call to a light-profile result (OpenHuman's
    /// orchestrator, whose `coding` model hint resolves to `Light`) gets its
    /// summary, and the exact original stays retrievable from CCR.
    #[tokio::test]
    async fn the_light_profile_summarizes_when_the_host_asks() {
        let _guard = crate::llm::callback_test_guard().await;
        enable_llm_summary();
        let calls = std::sync::Arc::new(std::sync::atomic::AtomicUsize::new(0));
        let counter = calls.clone();
        crate::llm::configure_callback(Some(std::sync::Arc::new(move |_| {
            counter.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
            Box::pin(async { Ok(Some("light note".to_string())) })
        })));
        let output = "integration light profile asked ".repeat(60);
        let report = compact_tool_output(ToolOutputCall {
            scope: Some("tool-integration-light-asked"),
            ..call(&output, AgentTokenjuiceCompression::Light, None)
        })
        .await;
        assert_eq!(calls.load(std::sync::atomic::Ordering::SeqCst), 1);
        assert!(report.text.starts_with("light note"), "{}", report.text);
        assert_eq!(report.stats.rule_id, "llm_summary");
        assert!(report.stats.applied);
        assert!(
            report
                .text
                .contains(crate::cache::marker::RETRIEVE_TOOL_NAME),
            "the exact original stays retrievable: {}",
            report.text
        );
        crate::llm::configure_callback(None);
    }

    #[tokio::test]
    async fn a_failed_summary_falls_through_with_a_notice() {
        let _guard = crate::llm::callback_test_guard().await;
        enable_llm_summary();
        crate::llm::configure_callback(Some(std::sync::Arc::new(|_| {
            Box::pin(async { Err("offline".to_string()) })
        })));
        let output = "integration failed summary ".repeat(60);
        let report = compact_tool_output(ToolOutputCall {
            scope: Some("tool-integration-failure"),
            ..call(&output, AgentTokenjuiceCompression::Full, None)
        })
        .await;
        assert_ne!(report.stats.rule_id, "llm_summary");
        assert_eq!(
            report.notice,
            Some(crate::summarize::UnavailableReason::Failed.notice())
        );
        crate::llm::configure_callback(None);
    }

    #[tokio::test]
    async fn a_summary_runs_even_with_compaction_disabled() {
        let _guard = crate::llm::callback_test_guard().await;
        enable_llm_summary();
        crate::llm::configure_callback(Some(std::sync::Arc::new(|_| {
            Box::pin(async { Ok(Some("note".to_string())) })
        })));
        let output = "integration compaction disabled ".repeat(60);
        let report = compact_tool_output(ToolOutputCall {
            compaction_enabled: false,
            ..call(&output, AgentTokenjuiceCompression::Full, None)
        })
        .await;
        assert_eq!(report.stats.rule_id, "llm_summary");

        crate::llm::configure_callback(None);
        let report = compact_tool_output(ToolOutputCall {
            compaction_enabled: false,
            scope: Some("tool-integration-disabled"),
            ..call(&output, AgentTokenjuiceCompression::Full, None)
        })
        .await;
        assert_eq!(report.stats.rule_id, "none/disabled");
        assert_eq!(report.text, output);
    }
}
