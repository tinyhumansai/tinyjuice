#![allow(clippy::field_reassign_with_default)]
//! Live latency evaluation of the tool-output summary stage.
//!
//! Sends real payloads through `compact_tool_output` with a real model behind
//! the host `Generate` callback (OpenRouter, via `curl`), and prints wall time,
//! output size and whether a planted fact is still reachable.
//!
//! ```text
//! OPENROUTER_API_KEY=... cargo run --release --example live_summary_eval -- \
//!     --fixtures DIR [--arm NAME]... [--reps N] [--model ID]
//! ```
//!
//! Arms: `baseline` (summary on), `no_summary` (deterministic only),
//! `repl_handle` (preview + handle, no model). Payloads are read, never logged.

use std::{path::PathBuf, sync::Arc, time::Instant};

use tinyjuice::{
    llm::{GenerateCallback, GenerateRequest, configure_callback},
    repl::{ReplLimits, ReplOp, ReplOutput, run_op},
    tool_integration::{ToolOutputCall, compact_tool_output},
    types::{AgentTokenjuiceCompression, CompressOptions},
};

struct Fixture {
    name: &'static str,
    tool: &'static str,
    file: &'static str,
    needle: &'static str,
}

const FIXTURES: &[Fixture] = &[
    Fixture {
        name: "github_search_134k",
        tool: "GITHUB_SEARCH_CODE",
        file: "github_search_code_134k.json",
        needle: "",
    },
    Fixture {
        name: "notion_blocks_131k",
        tool: "NOTION_FETCH_BLOCK_CONTENTS",
        file: "notion_blocks.json",
        needle: "ROLLBACK_SAFE_MODE",
    },
    Fixture {
        name: "web_fetch_37k",
        tool: "web_fetch",
        file: "web_fetch_37k.html",
        needle: "",
    },
];

fn openrouter(model: String) -> Arc<GenerateCallback> {
    Arc::new(move |req: GenerateRequest| {
        let model = model.clone();
        Box::pin(async move {
            let key = std::env::var("OPENROUTER_API_KEY")
                .map_err(|_| "no OPENROUTER_API_KEY".to_string())?;
            let body = serde_json::json!({
                "model": model,
                "max_tokens": req.max_output_tokens,
                "messages": [
                    { "role": "system", "content": req.system },
                    { "role": "user", "content": req.prompt },
                ],
            });
            tokio::task::spawn_blocking(move || {
                let header = format!("header = \"Authorization: Bearer {key}\"\n");
                // The key goes through a 0600 curl config so it never shows in argv.
                let cfg = std::env::temp_dir().join(format!(
                    "tj-eval-{}-{}.cfg",
                    std::process::id(),
                    CFG_SEQ.fetch_add(1, std::sync::atomic::Ordering::Relaxed)
                ));
                {
                    use std::io::Write as _;
                    use std::os::unix::fs::OpenOptionsExt;
                    let mut f = std::fs::OpenOptions::new()
                        .write(true)
                        .create_new(true)
                        .mode(0o600)
                        .open(&cfg)
                        .map_err(|e| e.to_string())?;
                    f.write_all(header.as_bytes()).map_err(|e| e.to_string())?;
                }
                let mut child = std::process::Command::new("curl")
                    .args([
                        "-sS",
                        "--max-time",
                        "120",
                        "https://openrouter.ai/api/v1/chat/completions",
                        "-H",
                        "Content-Type: application/json",
                        "-K",
                    ])
                    .arg(&cfg)
                    .args(["-d", "@-"])
                    .stdin(std::process::Stdio::piped())
                    .stdout(std::process::Stdio::piped())
                    .stderr(std::process::Stdio::null())
                    .spawn()
                    .map_err(|e| e.to_string())?;
                use std::io::Write;
                child
                    .stdin
                    .take()
                    .unwrap()
                    .write_all(body.to_string().as_bytes())
                    .map_err(|e| e.to_string())?;
                let out = child.wait_with_output().map_err(|e| e.to_string())?;
                let _ = std::fs::remove_file(&cfg);
                let v: serde_json::Value =
                    serde_json::from_slice(&out.stdout).map_err(|e| e.to_string())?;
                v["choices"][0]["message"]["content"]
                    .as_str()
                    .map(|s| Some(s.to_string()))
                    .ok_or_else(|| "no content in reply".to_string())
            })
            .await
            .map_err(|e| e.to_string())?
        })
    })
}

fn arg_values(args: &[String], flag: &str) -> Vec<String> {
    args.windows(2)
        .filter(|w| w[0] == flag)
        .map(|w| w[1].clone())
        .collect()
}

#[tokio::main(flavor = "current_thread")]
static CFG_SEQ: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);

async fn main() {
    let args: Vec<String> = std::env::args().collect();
    let dir = PathBuf::from(
        arg_values(&args, "--fixtures")
            .pop()
            .expect("--fixtures DIR"),
    );
    let reps: usize = arg_values(&args, "--reps")
        .pop()
        .map_or(1, |r| r.parse().unwrap());
    let model = arg_values(&args, "--model")
        .pop()
        .unwrap_or_else(|| "z-ai/glm-5.3-flash".into());
    let mut arms = arg_values(&args, "--arm");
    if arms.is_empty() {
        arms = vec!["baseline".into(), "no_summary".into(), "repl_handle".into()];
    }
    configure_callback(Some(openrouter(model.clone())));
    println!("model={model} reps={reps}");
    println!(
        "{:<20} {:<14} {:>4} {:>10} {:>10} {:>9}  {:<14} fact",
        "fixture", "arm", "rep", "in_bytes", "out_bytes", "ms", "rule"
    );

    for fx in FIXTURES {
        let text = std::fs::read_to_string(dir.join(fx.file)).expect("fixture");
        let needle = if fx.needle.is_empty() {
            None
        } else {
            Some(fx.needle)
        };
        for arm in &arms {
            let mut opts = CompressOptions::default();
            // `baseline` reproduces the old behaviour: 3000 tokens, no timeout.
            // `capN` caps output tokens; `capN_tS` adds an S-second timeout.
            opts.llm_summary_enabled = arm == "baseline" || arm.starts_with("cap");
            opts.repl_handle = arm == "repl_handle";
            opts.llm_summary_max_output_tokens = 3_000;
            opts.llm_summary_timeout_ms = 0;
            if let Some(rest) = arm.strip_prefix("cap") {
                let mut parts = rest.split("_t");
                opts.llm_summary_max_output_tokens = parts.next().unwrap().parse().unwrap();
                if let Some(t) = parts.next() {
                    opts.llm_summary_timeout_ms = t.parse::<u64>().unwrap() * 1000;
                }
            }
            tinyjuice::configure(opts);
            for rep in 0..reps {
                let scope = format!("eval-{}-{arm}-{rep}", fx.name);
                let started = Instant::now();
                let report = compact_tool_output(ToolOutputCall {
                    tool_name: fx.tool,
                    arguments: None,
                    output: &text,
                    exit_code: None,
                    profile: AgentTokenjuiceCompression::Full,
                    compaction_enabled: true,
                    focus: Some("which findings mention rollback or retry budgets"),
                    context_token: Some("eval-turn"),
                    scope: Some(&scope),
                })
                .await;
                let ms = started.elapsed().as_millis();
                let fact = match needle {
                    None => "n/a".to_string(),
                    Some(n) if report.text.contains(n) => "inline".to_string(),
                    Some(n) => {
                        let token = tinyjuice::cache::marker::parse_markers(&report.text).pop();
                        let found = token.is_some_and(|t| {
                            matches!(
                                run_op(&tinyjuice::cache::GlobalCcrStore, &t, &ReplOp::Find { query: n.into(), mode: Default::default(), ignore_case: false, context: 0, top_k: None, scope: None, unit: Default::default() }, &ReplLimits::default()),
                                Ok(ReplOutput::Lines { hits, .. }) if !hits.is_empty()
                            )
                        });
                        if found {
                            "via-handle".into()
                        } else {
                            "LOST".into()
                        }
                    }
                };
                println!(
                    "{:<20} {:<14} {:>4} {:>10} {:>10} {:>9}  {:<14} {}",
                    fx.name,
                    arm,
                    rep,
                    text.len(),
                    report.text.len(),
                    ms,
                    report.stats.rule_id,
                    fact
                );
            }
        }
    }
}
