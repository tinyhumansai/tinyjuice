//! Live agent evaluation of TinyJuice's REPL tools, on a tinyagents harness.
//!
//! ```text
//! OPENROUTER_API_KEY=... cargo run --release --manifest-path crates/tinyjuice-examples/Cargo.toml -- \
//!     [--arm raw|summary|summary_fast|repl]... [--scenario NAME]... [--reps N] \
//!     [--model ID] [--summarizer-model ID]
//! ```
//!
//! Each run gives the agent one payload tool (whose output goes through TinyJuice
//! per arm) and, for `repl`, the `juice_*` tools. It reports wall time, time spent
//! inside TinyJuice, model and tool calls, tokens, and whether the answer is right.

mod fixtures;
mod summarizer;
mod tools;

use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Instant;

use tinyagents_harness::runtime::AgentHarness;
use tinyinference_llm::message::Message;
use tinyinference_llm::providers::openai::OpenAiModel;
use tinyjuice::repl::{ReplLimits, tools::repl_tools};
use tinyjuice::types::CompressOptions;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Arm {
    /// Payload goes to the model untouched.
    Raw,
    /// The old behaviour: LLM summary, 3000 output tokens, no timeout.
    Summary,
    /// Summary with the new defaults: 1000 tokens, 8 s timeout, preview-plus-handle fallback.
    SummaryFast,
    /// Preview plus handle, inspected with the `juice_*` tools. No summarizer model.
    Repl,
}

impl Arm {
    fn parse(s: &str) -> Option<Self> {
        Some(match s {
            "raw" => Self::Raw,
            "summary" => Self::Summary,
            "summary_fast" => Self::SummaryFast,
            "repl" => Self::Repl,
            _ => return None,
        })
    }

    fn name(self) -> &'static str {
        match self {
            Self::Raw => "raw",
            Self::Summary => "summary",
            Self::SummaryFast => "summary_fast",
            Self::Repl => "repl",
        }
    }

    fn options(self) -> CompressOptions {
        let mut opts = CompressOptions::default();
        match self {
            Self::Raw => {}
            Self::Summary => {
                opts.llm_summary_enabled = true;
                opts.llm_summary_mode = tinyjuice::types::LlmSummaryMode::Auto;
                opts.llm_summary_max_output_tokens = 3_000;
                opts.llm_summary_timeout_ms = 0;
            }
            Self::SummaryFast => {
                opts.llm_summary_enabled = true;
                opts.llm_summary_mode = tinyjuice::types::LlmSummaryMode::Auto;
            }
            Self::Repl => opts.repl_handle = true,
        }
        opts
    }
}

const SYSTEM: &str = "You answer questions about tool outputs. Call the provided tool to get the data, then answer concisely. \
Large outputs may be replaced by a short preview with a handle; when that happens, use the juice_* tools with that handle \
(juice_find with a mode, juice_extract, juice_summarize) instead of asking for the output again. Do not guess.";

fn values(args: &[String], flag: &str) -> Vec<String> {
    args.windows(2).filter(|w| w[0] == flag).map(|w| w[1].clone()).collect()
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let key = std::env::var("OPENROUTER_API_KEY").map_err(|_| anyhow::anyhow!("set OPENROUTER_API_KEY"))?;
    let model = values(&args, "--model").pop().unwrap_or_else(|| "deepseek/deepseek-v4.1-flash".into());
    let sum_model = values(&args, "--summarizer-model").pop().unwrap_or_else(|| "z-ai/glm-5.3-flash".into());
    let reps: usize = values(&args, "--reps").pop().map_or(1, |r| r.parse().unwrap_or(1));
    let arms: Vec<Arm> = match values(&args, "--arm") {
        a if a.is_empty() => vec![Arm::Raw, Arm::Summary, Arm::SummaryFast, Arm::Repl],
        a => a.iter().filter_map(|s| Arm::parse(s)).collect(),
    };
    let wanted = values(&args, "--scenario");

    tinyjuice::llm::configure_callback(Some(summarizer::callback(sum_model.clone(), key.clone())));
    println!("agent={model} summarizer={sum_model} reps={reps}");
    println!(
        "{:<22} {:<13} {:>3} {:>8} {:>8} {:>6} {:>6} {:>9} {:>9}  {:<7} tools",
        "scenario", "arm", "rep", "wall_ms", "juice_ms", "model", "tools", "in_tok", "out_tok", "correct"
    );

    for scenario in fixtures::SCENARIOS.iter().filter(|s| wanted.is_empty() || wanted.iter().any(|w| w == s.name)) {
        let payload = (scenario.payload)();
        for &arm in &arms {
            for rep in 0..reps {
                tinyjuice::configure(arm.options());
                let compaction_ms = Arc::new(AtomicU64::new(0));
                let mut harness: AgentHarness<()> = AgentHarness::new();
                harness
                    .register_model(
                        "agent",
                        Arc::new(
                            OpenAiModel::new(key.clone())
                                .with_base_url("https://openrouter.ai/api/v1")
                                .with_model(model.clone()),
                        ),
                    )
                    .set_default_model("agent")
                    .register_tool(Arc::new(tools::PayloadTool {
                        name: scenario.tool,
                        description: scenario.tool_description,
                        payload: payload.clone(),
                        arm,
                        question: scenario.question,
                        run_id: format!("{}-{}-{rep}", scenario.name, arm.name()),
                        compaction_ms: compaction_ms.clone(),
                    }));
                if arm != Arm::Raw {
                    let store = Arc::new(tinyjuice::cache::GlobalCcrStore);
                    for tool in repl_tools(store, ReplLimits::default()) {
                        harness.register_tool(Arc::from(tool));
                    }
                }
                let started = Instant::now();
                let outcome = harness
                    .invoke_default(&(), vec![Message::system(SYSTEM), Message::user(scenario.question)])
                    .await;
                let wall = started.elapsed().as_millis();
                match outcome {
                    Ok(run) => {
                        let answer = run.text().unwrap_or_default();
                        let correct = answer.to_lowercase().contains(&scenario.expected.to_lowercase());
                        println!(
                            "{:<22} {:<13} {:>3} {:>8} {:>8} {:>6} {:>6} {:>9} {:>9}  {:<7} {}",
                            scenario.name,
                            arm.name(),
                            rep,
                            wall,
                            compaction_ms.load(Ordering::Relaxed),
                            run.model_calls,
                            run.tool_calls,
                            run.usage.usage.input_tokens,
                            run.usage.usage.output_tokens,
                            if correct { "yes" } else { "NO" },
                            run.executed_tools.join(",")
                        );
                    }
                    Err(e) => println!("{:<22} {:<13} {:>3} {:>8} run failed: {e}", scenario.name, arm.name(), rep, wall),
                }
            }
        }
    }
    Ok(())
}
