[search: 500 match(es) across 24 file(s) · top 5 per file · full set via retrieve footer]
<OPENHUMAN_ROOT>/src/api/config.rs:14://! caused every `/auth/*`, `/agent-integrations/*`, and `/voice/*` request to
<OPENHUMAN_ROOT>/src/api/config.rs:103:/// so `/auth/*`, `/voice/*`, and `/agent-integrations/*` never accidentally
<OPENHUMAN_ROOT>/src/api/config.rs:460:/// (`/auth/me`, `/agent-integrations/…`) which then land on
<OPENHUMAN_ROOT>/src/api/config.rs:502:/// | `https://api.tinyhumans.ai/openai/v1/…`   | `/agent-integrations/foo` | `https://api.tinyhumans.ai/agent-integrations/foo`  ← path replaced   |
<OPENHUMAN_ROOT>/src/api/config.rs:678:             /agent-integrations/* requests don't 404 against your local LLM"
[+10 more match(es) in <OPENHUMAN_ROOT>/src/api/config.rs]
<OPENHUMAN_ROOT>/src/api/rest.rs:919:    /// Signals "the agent is typing…" on a channel that supports it
<OPENHUMAN_ROOT>/src/main.rs:111:            // the agent re-report routes through `TransientUpstreamHttp`, but the
<OPENHUMAN_ROOT>/src/main.rs:120:            // `agent::harness::session::runtime::run_single`,
<OPENHUMAN_ROOT>/src/main.rs:123:            // deterministic agent-state outcome surfaced to the user via
<OPENHUMAN_ROOT>/src/lib.rs:6://! - Domain-specific logic for the OpenHuman agent runtime.
<OPENHUMAN_ROOT>/src/bin/inference_probe.rs:7://! - `--mode harness` (default): build a real `Agent::from_config()`
<OPENHUMAN_ROOT>/src/bin/inference_probe.rs:24://!   RUST_LOG=info,openhuman_core::openhuman::agent=debug,openhuman_core::openhuman::inference=debug \
<OPENHUMAN_ROOT>/src/bin/inference_probe.rs:33:use openhuman_core::openhuman::agent::Agent;
<OPENHUMAN_ROOT>/src/bin/inference_probe.rs:89:    let mut agent = Agent::from_config(config).context("Agent::from_config failed")?;
<OPENHUMAN_ROOT>/src/bin/inference_probe.rs:109:        .context("agent.run_single failed")?;
[+9 more match(es) in <OPENHUMAN_ROOT>/src/bin/inference_probe.rs]
<OPENHUMAN_ROOT>/src/bin/harness_subagent_audit.rs:27:    self, AuditSteerError, AuditSubagentSessionStore, DurableSubagentSession, DurableSubagentStatus,
<OPENHUMAN_ROOT>/src/bin/harness_subagent_audit.rs:91:    subagent_failed: Vec<SubagentFailedEvent>,
<OPENHUMAN_ROOT>/src/bin/harness_subagent_audit.rs:164:struct SubagentFailedEvent {
<OPENHUMAN_ROOT>/src/bin/harness_subagent_audit.rs:233:        eprintln!("[harness_subagent_audit] ERROR: {err:#}");
<OPENHUMAN_ROOT>/src/bin/harness_subagent_audit.rs:271:    let mut agent = Agent::from_config(&config).context("Agent::from_config failed")?;
[+165 more match(es) in <OPENHUMAN_ROOT>/src/bin/harness_subagent_audit.rs]
<OPENHUMAN_ROOT>/src/core/jsonrpc_tests.rs:93:/// semaphore, `GLOBAL_REGISTRY` agent.run_turn handler, `STARTED`
<OPENHUMAN_ROOT>/src/core/jsonrpc_tests.rs:697:    let msg = r#"[composio] list_connections failed: Backend returned 500 Internal Server Error for GET https://api.tinyhumans.ai/agent-integrations/composio/connections: 401 {"error":{"message":"Invalid API key: ak_o1Og5*****","code":10401,"slug":"HTTP_Unauthorized","status":401}}"#;
<OPENHUMAN_ROOT>/src/core/event_bus/bus.rs:59:/// (e.g., an agent turn completed, a memory was stored).
<OPENHUMAN_ROOT>/src/core/event_bus/testing.rs:20://! [`crate::openhuman::agent::bus::mock_agent_run_turn`]) compose on top of
<OPENHUMAN_ROOT>/src/core/event_bus/testing.rs:61:/// [`crate::openhuman::agent::bus::use_real_agent_handler`] that need the
<OPENHUMAN_ROOT>/src/core/event_bus/testing.rs:62:/// real agent handler installed without racing against a stub-installing
<OPENHUMAN_ROOT>/src/core/event_bus/testing.rs:110:/// [`crate::openhuman::agent::bus::mock_agent_run_turn`]) should compose
<OPENHUMAN_ROOT>/CONTRIBUTING.md:7:For deeper architecture and subsystem references, use the GitBook under [`gitbooks/developing/`](gitbooks/developing/). For coding-agent and repository-specific implementation rules, see [`AGENTS.md`](AGENTS.md) and [`CLAUDE.md`](CLAUDE.md).
<OPENHUMAN_ROOT>/CONTRIBUTING.md:201:If you only changed docs in a normal local workflow, `pnpm format:check` is usually the only validation you need. AI-authored or remote-agent PRs must still fill in the AI Authored PR Metadata section of the PR template and report any blocked commands with the exact command and error.
<OPENHUMAN_ROOT>/CONTRIBUTING.md:220:- For AI-authored or remote-agent PRs, also fill in the AI Authored PR Metadata section of the PR template.
<OPENHUMAN_ROOT>/CONTRIBUTING.md:249:├── AGENTS.md               # Coding-agent repo rules
<OPENHUMAN_ROOT>/CONTRIBUTING.md:303:If you are contributing through a coding agent or remote environment, include the metadata required by the PR template and the Codex PR checklist.
[+1 more match(es) in <OPENHUMAN_ROOT>/CONTRIBUTING.md]
<OPENHUMAN_ROOT>/src/core/event_bus/events_tests.rs:6:        // Agent
<OPENHUMAN_ROOT>/src/core/event_bus/events_tests.rs:8:            DomainEvent::AgentTurnStarted {
<OPENHUMAN_ROOT>/src/core/event_bus/events_tests.rs:12:            "agent",
<OPENHUMAN_ROOT>/src/core/event_bus/events_tests.rs:23:            DomainEvent::AgentError {
<OPENHUMAN_ROOT>/src/core/event_bus/events_tests.rs:52:            DomainEvent::SubagentFailed {
[+18 more match(es) in <OPENHUMAN_ROOT>/src/core/event_bus/events_tests.rs]
<OPENHUMAN_ROOT>/src/core/observability.rs:12://! being logged at error level never reach Sentry. The agent-turn path is the
<OPENHUMAN_ROOT>/src/core/observability.rs:13://! canonical example — `run_single` used to publish a `DomainEvent::AgentError`
<OPENHUMAN_ROOT>/src/core/observability.rs:101:    /// error is raised again by `agent.run_single` /
<OPENHUMAN_ROOT>/src/core/observability.rs:196:    /// `AgentError::EmptyProviderResponse` + `AgentError::skips_sentry()`
<OPENHUMAN_ROOT>/src/core/observability.rs:510:    // `report_error_or_expected`, the `domain=agent` half of the flood). Routed
[+110 more match(es) in <OPENHUMAN_ROOT>/src/core/observability.rs]
<OPENHUMAN_ROOT>/package.json:51:    "agent-batch": "node scripts/agent-batch/cli.mjs",
<OPENHUMAN_ROOT>/package.json:52:    "agent-batch:test": "node --test scripts/agent-batch/__tests__/lib.test.mjs scripts/agent-batch/__tests__/cli.test.mjs",
<OPENHUMAN_ROOT>/src/core/event_bus/README.md:13:- `pub enum DomainEvent` — `events.rs` — `#[non_exhaustive]` catalog of events; current variants cover Agent (`AgentTurnStarted/Completed`, `AgentError`), Memory (`MemoryStored`, `MemoryRecalled`), Channels (`ChannelInboundMessage`, `ChannelMessageReceived/Processed`, `ChannelReactionReceived/Sent`, `ChannelConnected/Disconnected`), Cron (`CronJobTriggered/Completed`, `CronDeliveryRequested`), Skills, Tools, Webhooks, and System.
<OPENHUMAN_ROOT>/src/core/event_bus/README.md:31:- `src/openhuman/agent/bus.rs`, `agent/triage/{events,evaluator,escalation}.rs`, `agent_registry/tools/{dispatch,spawn_subagent}.rs` — agent + sub-agent events.
<OPENHUMAN_ROOT>/src/core/event_bus/README.md:41:## Emission policy (tinyagents migration, 05.3)
<OPENHUMAN_ROOT>/src/core/event_bus/README.md:43:The canonical run record is the TinyAgents event journal + status store
<OPENHUMAN_ROOT>/src/core/event_bus/README.md:48:  compression, tool-exposure, and steering signals ride the TinyAgents
[+6 more match(es) in <OPENHUMAN_ROOT>/src/core/event_bus/README.md]
<OPENHUMAN_ROOT>/src/core/event_bus/mod.rs:5://! modules (like memory, skills, and agents) to communicate without
<OPENHUMAN_ROOT>/src/core/socketio.rs:123:    /// `"provider"` | `"openhuman_budget"` | `"agent_loop"`
<OPENHUMAN_ROOT>/src/core/socketio.rs:139:    /// agent loop) and for transport failures without a provider prefix.
<OPENHUMAN_ROOT>/src/core/socketio.rs:197:    /// Sub-agent specific progress detail. Populated on
<OPENHUMAN_ROOT>/src/core/socketio.rs:1212:                    log::debug!("[socketio] broadcast agent_meetings:error");
<OPENHUMAN_ROOT>/src/core/socketio.rs:1213:                    let _ = io_agent_meetings.emit("agent_meetings:error", &payload);
[+65 more match(es) in <OPENHUMAN_ROOT>/src/core/socketio.rs]
<OPENHUMAN_ROOT>/src/core/mod.rs:9:pub mod agent_cli;
<OPENHUMAN_ROOT>/src/core/cli.rs:17:/// Debug/e2e agent paths can build deep async poll stacks while assembling
<OPENHUMAN_ROOT>/src/core/cli.rs:18:/// prompts, provider requests, and sub-agent tool loops.
<OPENHUMAN_ROOT>/src/core/cli.rs:83:        "agent" => {
<OPENHUMAN_ROOT>/src/core/cli.rs:85:                "[cli] dispatching to agent subcommand, args={:?}",
<OPENHUMAN_ROOT>/src/core/cli.rs:88:            crate::core::agent_cli::run_agent_command(&args[1..])
[+8 more match(es) in <OPENHUMAN_ROOT>/src/core/cli.rs]
<OPENHUMAN_ROOT>/CONTRIBUTING-BEGINNERS.md:19:- [Optional — Let an AI coding agent guide you](#optional--let-an-ai-coding-agent-guide-you)
<OPENHUMAN_ROOT>/CONTRIBUTING-BEGINNERS.md:402:## Optional — Let an AI coding agent guide you
<OPENHUMAN_ROOT>/CONTRIBUTING-BEGINNERS.md:404:If you use Claude Code, Cursor, AmpCode, Codex, or another coding agent, you can paste this prompt after cloning the repo:
<OPENHUMAN_ROOT>/CONTRIBUTING-BEGINNERS.md:410:AGENTS.md: https://raw.githubusercontent.com/tinyhumansai/openhuman/main/AGENTS.md
<OPENHUMAN_ROOT>/CONTRIBUTING-BEGINNERS.md:416:The agent should still ask before destructive actions like deleting files, resetting branches, or force-pushing. You are responsible for reviewing the final diff before opening a PR.
<OPENHUMAN_ROOT>/src/core/event_bus/native_request.rs:31://! register_native_global::<AgentTurnRequest, AgentTurnResponse, _, _>(
<OPENHUMAN_ROOT>/src/core/event_bus/native_request.rs:32://!     "agent.run_turn",
<OPENHUMAN_ROOT>/src/core/event_bus/native_request.rs:34://!         let text = run_agent_turn(/* ... */).await
<OPENHUMAN_ROOT>/src/core/event_bus/native_request.rs:36://!         Ok(AgentTurnResponse::new(text))
<OPENHUMAN_ROOT>/src/core/event_bus/native_request.rs:41://! let resp: AgentTurnResponse = request_native_global(
[+3 more match(es) in <OPENHUMAN_ROOT>/src/core/event_bus/native_request.rs]
<OPENHUMAN_ROOT>/README.md:64:OpenHuman is three things most assistants aren't: **a brain** that builds a persistent, local memory of your world; **a fantastic orchestrator** that runs fleets of agents on durable graphs; and **a deep researcher** that sweeps your data and the web before you finish asking. Every bullet links to the deeper writeup in the [docs](https://tinyhumans.gitbook.io/openhuman/).
<OPENHUMAN_ROOT>/README.md:76:- **[Workflows](https://tinyhumans.gitbook.io/openhuman/features/workflows)**: the agent proposes the automation; you review it on a canvas and save. Durable, trigger-driven, approval-gated runs on open-source [tinyflows](https://github.com/tinyhumansai/tinyflows).
<OPENHUMAN_ROOT>/README.md:77:- **[A harness that finishes the job](https://tinyhumans.gitbook.io/openhuman/developing/architecture/agent-harness)**: checkpointed graph runs on open-source [tinyagents](https://github.com/tinyhumansai/tinyagents). Stuck agents get steered, halted ones return a root cause, and every run replays with real per-call costs.
<OPENHUMAN_ROOT>/README.md:78:- **[A split brain, always on](https://tinyhumans.gitbook.io/openhuman/features/orchestration)**: a fast reflex agent triages inbound traffic while a deep reasoning core delegates to worker fleets, steered by the subconscious.
<OPENHUMAN_ROOT>/README.md:79:- **[An agent economy](https://tinyhumans.gitbook.io/openhuman/features/tinyplace)**: a `@handle` on [tiny.place](https://tiny.place), Signal-encrypted agent-to-agent orchestration, x402 USDC bounties and trading. Keys never touch disk.
[+21 more match(es) in <OPENHUMAN_ROOT>/README.md]
<OPENHUMAN_ROOT>/src/core/auth.rs:99:    // AgentBox marketplace surface — see `openhuman::agentbox::http`.
<OPENHUMAN_ROOT>/src/core/auth.rs:100:    // Mounted only when `OPENHUMAN_AGENTBOX_MODE=1`; the public-path entry is
<OPENHUMAN_ROOT>/src/core/auth.rs:110:    // AgentBox `GET /jobs/{job_id}` — `{job_id}` is a UUID per submission.
<OPENHUMAN_ROOT>/src/core/auth.rs:603:    fn agentbox_run_and_jobs_paths_are_public() {
<OPENHUMAN_ROOT>/src/core/auth.rs:604:        // AgentBox marketplace surface bypasses bearer auth (gated externally
[+1 more match(es) in <OPENHUMAN_ROOT>/src/core/auth.rs]
<OPENHUMAN_ROOT>/src/core/event_bus/subscriber.rs:17:    /// or `Some(&["agent", "cron"])` to receive only matching domains.
<OPENHUMAN_ROOT>/src/core/logging.rs:96:                Level::TRACE => Style::new().fg(Color::Magenta).dimmed().paint(tag),

[compacted tool output — this is a PARTIAL view; the full original (63209 bytes) is available by calling tokenjuice_retrieve with token "04febbde1859003dc3d6ded4db52f58a" (marker ⟦tj:04febbde1859003dc3d6ded4db52f58a⟧)]