//! Direct chat probe — exercise the full orchestrator harness end-to-end
//! with the live user config, run a single turn, and print whether the
//! harness actually emitted tool calls.
//!
//! Two modes:
//!
//! - `--mode harness` (default): build a real `Agent::from_config()`
//!   orchestrator and call `run_single("...")`. This is the production
//!   path — system prompt with tool catalog, Connected Integrations
//!   block, dispatcher selection, tool execution, all of it. Use this
//!   to verify whether tool calls fire for a given user prompt.
//!
//! - `--mode raw`: send a single hand-built request straight to the
//!   chat provider (no harness, no real tools). Useful to isolate
//!   "does the model itself follow P-format / native tool spec".
//!
//! # Usage
//!
//! ```sh
//! # Drive a real orchestrator turn — needs BACKEND_URL set so the
//! # integrations client can fetch the user's Connected Integrations.
//! BACKEND_URL=https://staging-api.tinyhumans.ai \
//!   OPENHUMAN_APP_ENV=staging \
//!   RUST_LOG=info,openhuman_core::openhuman::agent=debug,openhuman_core::openhuman::inference=debug \
//!   cargo run --bin inference-probe -- \
//!     --mode harness --prompt "hey list my top 5 emails"
//!
//! # Raw provider call (no harness):
//! cargo run --bin inference-probe -- --mode raw --raw-mode pformat
//! ```
use anyhow::{Context, Result};
use clap::Parser;
use openhuman_core::openhuman::agent::Agent;
use openhuman_core::openhuman::config::Config;
use openhuman_core::openhuman::inference::provider::create_chat_provider;
use openhuman_core::openhuman::inference::provider::traits::{ChatMessage, ChatRequest};
use openhuman_core::openhuman::tools::traits::ToolSpec;
use serde_json::json;

#[derive(Parser, Debug)]
#[command(name = "inference-probe")]
struct Args {
    /// "harness" — drive the real orchestrator harness end-to-end.
    /// "raw" — send a hand-built request directly to the chat provider.
    #[arg(long, default_value = "harness")]
    mode: String,

    /// Provider role for `--mode raw`. Ignored in harness mode.
    #[arg(long, default_value = "reasoning")]
    role: String,

    /// For `--mode raw`: "pformat" (no tools field, P-format protocol in
    /// system prompt) or "native" (structured `tools` + `tool_choice: auto`).
    #[arg(long, default_value = "pformat")]
    raw_mode: String,

    /// User prompt to send.
    #[arg(long, default_value = "hey list my top 5 emails")]
    prompt: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    let args = Args::parse();

    let config = Config::load_or_init()
        .await
        .context("loading user config (Config::load_or_init)")?;

    eprintln!("[probe] config.default_model = {:?}", config.default_model);
    eprintln!(
        "[probe] config.agent.tool_dispatcher = {:?}",
        config.agent.tool_dispatcher
    );

    match args.mode.as_str() {
        "harness" => run_harness(&config, &args.prompt).await,
        "raw" => run_raw(&config, &args.role, &args.raw_mode, &args.prompt).await,
        other => anyhow::bail!("unknown --mode {other:?}; want 'harness' or 'raw'"),
    }
}

async fn run_harness(config: &Config, prompt: &str) -> Result<()> {
    eprintln!("[probe] mode = harness (orchestrator)");
    eprintln!("[probe] prompt = {prompt:?}");
    { … 28 line(s) … ⟦tj:5ce916adfdaa2ca80cbce94071148b86⟧ }
    Ok(())
}

async fn run_raw(config: &Config, role: &str, raw_mode: &str, prompt: &str) -> Result<()> {
    let (provider, model_name) =
        create_chat_provider(role, config).context("create_chat_provider failed")?;
    { … 112 line(s) … ⟦tj:21548d9b2c455fc7599b2dc27489af20⟧ }
    Ok(())
}
[omitted blocks are individually retrievable: call tinyjuice_retrieve with the token inside an omission marker to expand just that block]

[compacted tool output — this is a PARTIAL view; the full original (8413 bytes) is available by calling tinyjuice_retrieve with token "598826d731037d2a00e9e59a518805d2" (marker ⟦tj:598826d731037d2a00e9e59a518805d2⟧)]