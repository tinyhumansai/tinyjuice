//! Live harness audit for reusable async sub-agent delegation.
//!
//! This binary intentionally uses the user's real OpenHuman config and live
//! provider/backend credentials. It records only sanitized progress metadata:
//! tool names, task/session ids, statuses, character counts, and elapsed times.
//! It does not print prompts, tool arguments, assistant replies, transcripts,
//! credentials, or integration payloads.
//!
//! Typical usage:
//!
//! ```sh
//! scripts/debug/harness-subagent-audit.sh --turns 2
//! ```

use std::collections::BTreeSet;
use std::sync::{
    atomic::{AtomicBool, AtomicUsize, Ordering},
    Arc, Mutex,
};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use anyhow::{Context, Result};
use clap::Parser;
use openhuman_core::openhuman::agent::progress::AgentProgress;
use openhuman_core::openhuman::agent::Agent;
use openhuman_core::openhuman::agent_orchestration::harness_audit::{
    self, AuditSteerError, AuditSubagentSessionStore, DurableSubagentSession, DurableSubagentStatus,
};
use openhuman_core::openhuman::config::Config;
use serde::Serialize;
use tokio::sync::mpsc;

#[derive(Parser, Debug)]
#[command(name = "harness-subagent-audit")]
struct Args {
    /// Sub-agent archetype to request from the orchestrator.
    #[arg(long, default_value = "researcher")]
    agent_id: String,

    /// Stable reusable task key. Defaults to audit-subagent-<unix-seconds>.
    #[arg(long)]
    task_key: Option<String>,

    /// Number of parent turns to run. Use 2 to audit same-key reuse.
    #[arg(long, default_value_t = 2)]
    turns: usize,

    /// Seconds to wait for the durable session to appear or settle.
    #[arg(long, default_value_t = 45)]
    wait_secs: u64,

    /// Require the final durable session status to leave running.
    #[arg(long)]
    require_completion: bool,

    /// Override the first parent prompt. The audit task key is not appended.
    #[arg(long)]
    prompt: Option<String>,

    /// Override the second parent prompt. Only used when --turns is at least 2.
    #[arg(long)]
    follow_up: Option<String>,

    /// Print sanitized JSON summary in addition to the human summary.
    #[arg(long)]
    json: bool,

    /// After the first async sub-agent spawn, steer the running child through its run queue.
    #[arg(long)]
    steer_mid_run: bool,

    /// Delay after SubagentSpawned before attempting the steer.
    #[arg(long, default_value_t = 250)]
    steer_delay_ms: u64,

    /// Seconds to retry resolving/registering the running child before steering fails.
    #[arg(long, default_value_t = 10)]
    steer_wait_secs: u64,

    /// Override the steering message. The message itself is never printed.
    #[arg(long)]
    steer_message: Option<String>,
}

#[derive(Debug, Default, Serialize)]
struct ProgressStats {
    parent_tool_started: Vec<ParentToolStarted>,
    parent_tool_completed: Vec<ParentToolCompleted>,
    subagent_spawned: Vec<SubagentSpawnedEvent>,
    subagent_completed: Vec<SubagentCompletedEvent>,
    subagent_failed: Vec<SubagentFailedEvent>,
    subagent_tool_started: Vec<SubagentToolEvent>,
    subagent_tool_completed: Vec<SubagentToolCompletedEvent>,
    steer_attempts: Vec<SteerAttemptEvent>,
    turn_completed: usize,
}

#[derive(Debug, Serialize)]
struct ParentToolStarted {
    turn: usize,
    call_id: String,
    tool_name: String,
    iteration: u32,
    argument_keys: Vec<String>,
}

#[derive(Debug, Serialize)]
struct ParentToolCompleted {
    turn: usize,
    call_id: String,
    tool_name: String,
    success: bool,
    output_chars: usize,
    elapsed_ms: u64,
    iteration: u32,
}

#[derive(Debug, Clone, Serialize)]
struct SubagentSpawnedEvent {
    turn: usize,
    agent_id: String,
    task_id: String,
    mode: String,
    dedicated_thread: bool,
    prompt_chars: usize,
    worker_thread_id: Option<String>,
    display_name: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
struct SteerAttemptEvent {
    turn: usize,
    agent_id: String,
    task_id: String,
    subagent_session_id: Option<String>,
    delivered: bool,
    error: Option<String>,
    attempts: usize,
    elapsed_ms: u128,
    message_chars: usize,
}

#[derive(Clone)]
struct SteerAuditConfig {
    store: AuditSubagentSessionStore,
    task_key: String,
    message: String,
    delay: Duration,
    wait_for: Duration,
    fired: Arc<AtomicBool>,
}

#[derive(Debug, Serialize)]
struct SubagentCompletedEvent {
    turn: usize,
    agent_id: String,
    task_id: String,
    elapsed_ms: u64,
    iterations: u32,
    output_chars: usize,
}

#[derive(Debug, Serialize)]
struct SubagentFailedEvent {
    turn: usize,
    agent_id: String,
    task_id: String,
    error_chars: usize,
}

#[derive(Debug, Serialize)]
struct SubagentToolEvent {
    turn: usize,
    agent_id: String,
    task_id: String,
    call_id: String,
    tool_name: String,
    iteration: u32,
}

#[derive(Debug, Serialize)]
struct SubagentToolCompletedEvent {
    turn: usize,
    agent_id: String,
    task_id: String,
    call_id: String,
    tool_name: String,
    success: bool,
    output_chars: usize,
    elapsed_ms: u64,
    iteration: u32,
}

#[derive(Debug, Clone, Serialize)]
struct SessionSummary {
    subagent_session_id: String,
    parent_session: String,
    parent_thread_id: Option<String>,
    worker_thread_id: Option<String>,
    agent_id: String,
    display_name: Option<String>,
    task_key: String,
    current_task_id: Option<String>,
    status: DurableSubagentStatus,
    reusable: bool,
    created_at: String,
    updated_at: String,
    last_used_at: String,
}

#[derive(Debug, Serialize)]
struct AuditSummary {
    task_key: String,
    agent_id: String,
    turns_requested: usize,
    assistant_reply_chars: Vec<usize>,
    progress: ProgressStats,
    sessions: Vec<SessionSummary>,
    checks: Vec<AuditCheck>,
    passed: bool,
}

#[derive(Debug, Serialize)]
struct AuditCheck {
    name: &'static str,
    passed: bool,
    detail: String,
}

#[tokio::main]
async fn main() { … 6 line(s) … }

async fn run() -> Result<()> { … 143 line(s) … }

async fn drain_progress(
    mut rx: mpsc::Receiver<AgentProgress>,
    stats: Arc<Mutex<ProgressStats>>,
    current_turn: Arc<AtomicUsize>,
    steer_config: Option<SteerAuditConfig>,
) { … 261 line(s) … }

async fn steer_after_spawn(
    config: SteerAuditConfig,
    spawned: SubagentSpawnedEvent,
) -> SteerAttemptEvent { … 63 line(s) … }

fn failed_steer_attempt(
    spawned: SubagentSpawnedEvent,
    error: Option<String>,
    attempts: usize,
    elapsed_ms: u128,
    message_chars: usize,
) -> SteerAttemptEvent { … 13 line(s) … }

fn find_session_for_task(
    store: &AuditSubagentSessionStore,
    task_key: &str,
    task_id: &str,
) -> Result<Option<SessionSummary>> { … 9 line(s) … }

fn argument_keys(value: &serde_json::Value) -> Vec<String> { … 6 line(s) … }

fn first_turn_prompt(agent_id: &str, task_key: &str) -> String { … 9 line(s) … }

fn second_turn_prompt(agent_id: &str, task_key: &str) -> String { … 9 line(s) … }

fn default_steer_message(task_key: &str) -> String { … 5 line(s) … }

async fn poll_matching_sessions(
    store: &AuditSubagentSessionStore,
    task_key: &str,
    wait_for: Duration,
    require_completion: bool,
) -> Result<Vec<SessionSummary>> { … 16 line(s) … }

fn load_matching_sessions(
    store: &AuditSubagentSessionStore,
    task_key: &str,
) -> Result<Vec<SessionSummary>> { … 13 line(s) … }

impl From<DurableSubagentSession> for SessionSummary {
    fn from(session: DurableSubagentSession) -> Self { … 17 line(s) … }
}

fn evaluate_checks(
    agent_id: &str,
    turns: usize,
    require_completion: bool,
    progress: &ProgressStats,
    sessions: &[SessionSummary],
) -> Vec<AuditCheck> { … 104 line(s) … }

fn is_spawn_tool(tool_name: &str) -> bool {
    tool_name == "spawn_subagent" || tool_name == "spawn_async_subagent"
}

fn take_stats(stats: Arc<Mutex<ProgressStats>>) -> ProgressStats { … 6 line(s) … }

fn print_human_summary(summary: &AuditSummary) { … 66 line(s) … }

fn unix_seconds() -> u64 { … 6 line(s) … }

[compacted tool output — this is a PARTIAL view; the full original (34749 bytes) is available by calling tokenjuice_retrieve with token "b3185402b63f34b01c54e265656b05e2" (marker ⟦tj:b3185402b63f34b01c54e265656b05e2⟧)]