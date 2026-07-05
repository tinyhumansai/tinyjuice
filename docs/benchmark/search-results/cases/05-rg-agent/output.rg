[search: 500 match(es) across 20 file(s) · top 5-12 per file (adaptive) · full set via retrieve footer]
<OPENHUMAN_ROOT>/src/api/config.rs:14://! caused every `/auth/*`, `/agent-integrations/*`, and `/voice/*` request to
<OPENHUMAN_ROOT>/src/api/config.rs:103:/// so `/auth/*`, `/voice/*`, and `/agent-integrations/*` never accidentally
<OPENHUMAN_ROOT>/src/api/config.rs:460:/// (`/auth/me`, `/agent-integrations/…`) which then land on
<OPENHUMAN_ROOT>/src/api/config.rs:502:/// | `https://api.tinyhumans.ai/openai/v1/…`   | `/agent-integrations/foo` | `https://api.tinyhumans.ai/agent-integrations/foo`  ← path replaced   |
<OPENHUMAN_ROOT>/src/api/config.rs:678:             /agent-integrations/* requests don't 404 against your local LLM"
<OPENHUMAN_ROOT>/src/api/config.rs:800:        // /agent-integrations/* calls.
<OPENHUMAN_ROOT>/src/api/config.rs:804:                "/agent-integrations/composio/toolkits",
<OPENHUMAN_ROOT>/src/api/config.rs:806:            "https://api.tinyhumans.ai/agent-integrations/composio/toolkits"
<OPENHUMAN_ROOT>/src/api/config.rs:812:        let expected = "https://api.tinyhumans.ai/agent-integrations/composio/toolkits";
<OPENHUMAN_ROOT>/src/api/config.rs:816:                "/agent-integrations/composio/toolkits"
<OPENHUMAN_ROOT>/src/api/config.rs:823:                "/agent-integrations/composio/toolkits"
<OPENHUMAN_ROOT>/src/api/config.rs:834:                "/agent-integrations/composio/tools?toolkits=gmail"
[+3 more match(es) in <OPENHUMAN_ROOT>/src/api/config.rs ⟦tj:de23244384d305db23a1f3c54dd41ba0⟧]
<OPENHUMAN_ROOT>/src/api/rest.rs:919:    /// Signals "the agent is typing…" on a channel that supports it
<OPENHUMAN_ROOT>/src/main.rs:111:            // the agent re-report routes through `TransientUpstreamHttp`, but the
<OPENHUMAN_ROOT>/src/main.rs:120:            // `agent::harness::session::runtime::run_single`,
<OPENHUMAN_ROOT>/src/main.rs:123:            // deterministic agent-state outcome surfaced to the user via
<OPENHUMAN_ROOT>/src/lib.rs:6://! - Domain-specific logic for the OpenHuman agent runtime.
<OPENHUMAN_ROOT>/src/bin/inference_probe.rs:7://! - `--mode harness` (default): build a real `Agent::from_config()`
<OPENHUMAN_ROOT>/src/bin/inference_probe.rs:24://!   RUST_LOG=info,openhuman_core::openhuman::agent=debug,openhuman_core::openhuman::inference=debug \
<OPENHUMAN_ROOT>/src/bin/inference_probe.rs:33:use openhuman_core::openhuman::agent::Agent;
<OPENHUMAN_ROOT>/src/bin/inference_probe.rs:74:        "[probe] config.agent.tool_dispatcher = {:?}",
<OPENHUMAN_ROOT>/src/bin/inference_probe.rs:75:        config.agent.tool_dispatcher
<OPENHUMAN_ROOT>/src/bin/inference_probe.rs:89:    let mut agent = Agent::from_config(config).context("Agent::from_config failed")?;
<OPENHUMAN_ROOT>/src/bin/inference_probe.rs:94:    agent.fetch_connected_integrations().await;
<OPENHUMAN_ROOT>/src/bin/inference_probe.rs:95:    let refreshed = agent.refresh_delegation_tools();
<OPENHUMAN_ROOT>/src/bin/inference_probe.rs:96:    let conn_count = agent.connected_integrations().len();
<OPENHUMAN_ROOT>/src/bin/inference_probe.rs:101:    eprintln!("[probe] visible tool count = {}", agent.tools().len());
<OPENHUMAN_ROOT>/src/bin/inference_probe.rs:102:    eprintln!("[probe] model = {}", agent.model_name());
<OPENHUMAN_ROOT>/src/bin/inference_probe.rs:109:        .context("agent.run_single failed")?;
[+2 more match(es) in <OPENHUMAN_ROOT>/src/bin/inference_probe.rs ⟦tj:5e49251a87c4d021099ff611f37d765f⟧]
<OPENHUMAN_ROOT>/src/bin/harness_subagent_audit.rs:27:    self, AuditSteerError, AuditSubagentSessionStore, DurableSubagentSession, DurableSubagentStatus,
<OPENHUMAN_ROOT>/src/bin/harness_subagent_audit.rs:91:    subagent_failed: Vec<SubagentFailedEvent>,
<OPENHUMAN_ROOT>/src/bin/harness_subagent_audit.rs:164:struct SubagentFailedEvent {
<OPENHUMAN_ROOT>/src/bin/harness_subagent_audit.rs:233:        eprintln!("[harness_subagent_audit] ERROR: {err:#}");
<OPENHUMAN_ROOT>/src/bin/harness_subagent_audit.rs:271:    let mut agent = Agent::from_config(&config).context("Agent::from_config failed")?;
<OPENHUMAN_ROOT>/src/bin/harness_subagent_audit.rs:328:            .with_context(|| format!("agent.run_single failed on turn {turn}"))?;
<OPENHUMAN_ROOT>/src/bin/harness_subagent_audit.rs:488:                                "[harness_subagent_audit] steer_attempt turn={} task_id={} delivered={} attempts={} elapsed_ms={} message_chars={} error={}",
<OPENHUMAN_ROOT>/src/bin/harness_subagent_audit.rs:531:            AgentProgress::SubagentFailed {
<OPENHUMAN_ROOT>/src/bin/harness_subagent_audit.rs:537:                    "[harness_subagent_audit] progress turn={} subagent_failed agent_id={} task_id={} error_chars={}",
<OPENHUMAN_ROOT>/src/bin/harness_subagent_audit.rs:546:                    .subagent_failed
<OPENHUMAN_ROOT>/src/bin/harness_subagent_audit.rs:547:                    .push(SubagentFailedEvent {
<OPENHUMAN_ROOT>/src/bin/harness_subagent_audit.rs:971:        "progress: parent_spawn_started={} parent_spawn_completed={} subagent_spawned={} subagent_completed={} subagent_failed={} subagent_tool_started={} subagent_tool_completed={}",
[+158 more match(es) in <OPENHUMAN_ROOT>/src/bin/harness_subagent_audit.rs ⟦tj:e7429c5d97861b652415c730b16345fa⟧]
<OPENHUMAN_ROOT>/src/core/jsonrpc_tests.rs:93:/// semaphore, `GLOBAL_REGISTRY` agent.run_turn handler, `STARTED`
<OPENHUMAN_ROOT>/src/core/jsonrpc_tests.rs:697:    let msg = r#"[composio] list_connections failed: Backend returned 500 Internal Server Error for GET https://api.tinyhumans.ai/agent-integrations/composio/connections: 401 {"error":{"message":"Invalid API key: ak_o1Og5*****","code":10401,"slug":"HTTP_Unauthorized","status":401}}"#;
<OPENHUMAN_ROOT>/src/core/event_bus/bus.rs:59:/// (e.g., an agent turn completed, a memory was stored).
<OPENHUMAN_ROOT>/CONTRIBUTING.md:7:For deeper architecture and subsystem references, use the GitBook under [`gitbooks/developing/`](gitbooks/developing/). For coding-agent and repository-specific implementation rules, see [`AGENTS.md`](AGENTS.md) and [`CLAUDE.md`](CLAUDE.md).
<OPENHUMAN_ROOT>/CONTRIBUTING.md:201:If you only changed docs in a normal local workflow, `pnpm format:check` is usually the only validation you need. AI-authored or remote-agent PRs must still fill in the AI Authored PR Metadata section of the PR template and report any blocked commands with the exact command and error.
<OPENHUMAN_ROOT>/CONTRIBUTING.md:220:- For AI-authored or remote-agent PRs, also fill in the AI Authored PR Metadata section of the PR template.
<OPENHUMAN_ROOT>/CONTRIBUTING.md:249:├── AGENTS.md               # Coding-agent repo rules
<OPENHUMAN_ROOT>/CONTRIBUTING.md:303:If you are contributing through a coding agent or remote environment, include the metadata required by the PR template and the Codex PR checklist.
<OPENHUMAN_ROOT>/CONTRIBUTING.md:309:- Use the controller registry and domain module structure described in [`AGENTS.md`](AGENTS.md) for new Rust functionality.
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:57:    /// An error occurred during agent processing.
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:58:    AgentError {
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:85:    /// A sub-agent failed (max iterations, provider error, missing
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:88:    SubagentFailed {
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:119:    /// High-level orchestration observed a child agent failure.
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:120:    AgentOrchestrationFailed {
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:1287:            | Self::AgentError { .. }
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:1290:            | Self::SubagentFailed { .. }
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:1294:            | Self::AgentOrchestrationFailed { .. }
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:1452:            Self::AgentError { .. } => "AgentError",
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:1455:            Self::SubagentFailed { .. } => "SubagentFailed",
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:1459:            Self::AgentOrchestrationFailed { .. } => "AgentOrchestrationFailed",
[+89 more match(es) in <OPENHUMAN_ROOT>/src/core/event_bus/events.rs ⟦tj:aa820297fa9746d672fca7c294264dea⟧]
<OPENHUMAN_ROOT>/src/core/observability.rs:12://! being logged at error level never reach Sentry. The agent-turn path is the
<OPENHUMAN_ROOT>/src/core/observability.rs:13://! canonical example — `run_single` used to publish a `DomainEvent::AgentError`
<OPENHUMAN_ROOT>/src/core/observability.rs:101:    /// error is raised again by `agent.run_single` /
<OPENHUMAN_ROOT>/src/core/observability.rs:196:    /// `AgentError::EmptyProviderResponse` + `AgentError::skips_sentry()`
<OPENHUMAN_ROOT>/src/core/observability.rs:510:    // `report_error_or_expected`, the `domain=agent` half of the flood). Routed
<OPENHUMAN_ROOT>/src/core/observability.rs:614:    // `AgentError::skips_sentry()` in PR #2790; this covers the
<OPENHUMAN_ROOT>/src/core/observability.rs:1100:/// error sending request for url (http://127.0.0.1:18474/agent-integrations/composio/connections)
<OPENHUMAN_ROOT>/src/core/observability.rs:1460:/// `"Backend returned 400 Bad Request for POST https://api.tinyhumans.ai/agent-integrations/composio/authorize: Composio authorization failed: 400 …"`
<OPENHUMAN_ROOT>/src/core/observability.rs:1798:/// `AgentError::EmptyProviderResponse` was flattened to a `String` at the
<OPENHUMAN_ROOT>/src/core/observability.rs:2135:            // this via the typed `AgentError::skips_sentry()` (PR #2790);
<OPENHUMAN_ROOT>/src/core/observability.rs:2509:/// `openhuman::agent::error::MAX_ITERATIONS_ERROR_PREFIX`).
<OPENHUMAN_ROOT>/src/core/observability.rs:2532:        .any(crate::openhuman::agent::error::is_max_iterations_error)
[+103 more match(es) in <OPENHUMAN_ROOT>/src/core/observability.rs ⟦tj:3b657f00f274ff1fb8ad578079d092dc⟧]
<OPENHUMAN_ROOT>/src/core/event_bus/mod.rs:5://! modules (like memory, skills, and agents) to communicate without
<OPENHUMAN_ROOT>/src/core/event_bus/README.md:13:- `pub enum DomainEvent` — `events.rs` — `#[non_exhaustive]` catalog of events; current variants cover Agent (`AgentTurnStarted/Completed`, `AgentError`), Memory (`MemoryStored`, `MemoryRecalled`), Channels (`ChannelInboundMessage`, `ChannelMessageReceived/Processed`, `ChannelReactionReceived/Sent`, `ChannelConnected/Disconnected`), Cron (`CronJobTriggered/Completed`, `CronDeliveryRequested`), Skills, Tools, Webhooks, and System.
<OPENHUMAN_ROOT>/src/core/event_bus/README.md:31:- `src/openhuman/agent/bus.rs`, `agent/triage/{events,evaluator,escalation}.rs`, `agent_registry/tools/{dispatch,spawn_subagent}.rs` — agent + sub-agent events.
<OPENHUMAN_ROOT>/src/core/event_bus/README.md:41:## Emission policy (tinyagents migration, 05.3)
<OPENHUMAN_ROOT>/src/core/event_bus/README.md:43:The canonical run record is the TinyAgents event journal + status store
<OPENHUMAN_ROOT>/src/core/event_bus/README.md:44:(`StoreEventJournal` / `FileStatusStore`, wired in `tinyagents/journal.rs`),
<OPENHUMAN_ROOT>/src/core/event_bus/README.md:45:not this bus. When adding agent-run instrumentation:
<OPENHUMAN_ROOT>/src/core/event_bus/README.md:48:  compression, tool-exposure, and steering signals ride the TinyAgents
<OPENHUMAN_ROOT>/src/core/event_bus/README.md:49:  `AgentEvent` stream and are persisted to the journal; a UI reconstructs a
<OPENHUMAN_ROOT>/src/core/event_bus/README.md:55:  module boundary the crate stream does not serve. Subagent lifecycle
<OPENHUMAN_ROOT>/src/core/event_bus/README.md:57:  `agent_orchestration::subagent_events` (05.2), never hand-rolled
<OPENHUMAN_ROOT>/src/core/event_bus/README.md:58:  `publish_global(DomainEvent::Subagent*)`.
<OPENHUMAN_ROOT>/package.json:51:    "agent-batch": "node scripts/agent-batch/cli.mjs",
<OPENHUMAN_ROOT>/package.json:52:    "agent-batch:test": "node --test scripts/agent-batch/__tests__/lib.test.mjs scripts/agent-batch/__tests__/cli.test.mjs",
<OPENHUMAN_ROOT>/src/core/event_bus/testing.rs:20://! [`crate::openhuman::agent::bus::mock_agent_run_turn`]) compose on top of
<OPENHUMAN_ROOT>/src/core/event_bus/testing.rs:61:/// [`crate::openhuman::agent::bus::use_real_agent_handler`] that need the
<OPENHUMAN_ROOT>/src/core/event_bus/testing.rs:62:/// real agent handler installed without racing against a stub-installing
<OPENHUMAN_ROOT>/src/core/event_bus/testing.rs:110:/// [`crate::openhuman::agent::bus::mock_agent_run_turn`]) should compose
<OPENHUMAN_ROOT>/src/core/event_bus/events_tests.rs:6:        // Agent
<OPENHUMAN_ROOT>/src/core/event_bus/events_tests.rs:8:            DomainEvent::AgentTurnStarted {
<OPENHUMAN_ROOT>/src/core/event_bus/events_tests.rs:12:            "agent",
<OPENHUMAN_ROOT>/src/core/event_bus/events_tests.rs:23:            DomainEvent::AgentError {
<OPENHUMAN_ROOT>/src/core/event_bus/events_tests.rs:52:            DomainEvent::SubagentFailed {
[+18 more match(es) in <OPENHUMAN_ROOT>/src/core/event_bus/events_tests.rs ⟦tj:262ea71b2dd616c5da581f1c2168a502⟧]
<OPENHUMAN_ROOT>/src/core/mod.rs:9:pub mod agent_cli;
<OPENHUMAN_ROOT>/CONTRIBUTING-BEGINNERS.md:19:- [Optional — Let an AI coding agent guide you](#optional--let-an-ai-coding-agent-guide-you)
<OPENHUMAN_ROOT>/CONTRIBUTING-BEGINNERS.md:402:## Optional — Let an AI coding agent guide you
<OPENHUMAN_ROOT>/CONTRIBUTING-BEGINNERS.md:404:If you use Claude Code, Cursor, AmpCode, Codex, or another coding agent, you can paste this prompt after cloning the repo:
<OPENHUMAN_ROOT>/CONTRIBUTING-BEGINNERS.md:410:AGENTS.md: https://raw.githubusercontent.com/tinyhumansai/openhuman/main/AGENTS.md
<OPENHUMAN_ROOT>/CONTRIBUTING-BEGINNERS.md:416:The agent should still ask before destructive actions like deleting files, resetting branches, or force-pushing. You are responsible for reviewing the final diff before opening a PR.
<OPENHUMAN_ROOT>/plan.md:3:Multi-agent audit of the OpenHuman test surface (2,367 files / ~25,900 test declarations per
<OPENHUMAN_ROOT>/plan.md:32:  frontend E2E (WDIO + Playwright), Rust unit (agent/memory; channels/providers/platform;
<OPENHUMAN_ROOT>/plan.md:52:| ✅ | `src/openhuman/agent/harness/harness_gap_tests.rs` | `datetime_section_is_static_grounding_rule_not_a_volatile_timestamp` | Strict subset of `agent/prompts/mod_tests.rs::datetime_section_is_static_grounding_rule_without_volatile_timestamp`; the file's own header lists item 6 as covered elsewhere. |
<OPENHUMAN_ROOT>/plan.md:91:| ✅ | `src/openhuman/agent/prompts/mod_tests.rs::grounding_contract_requires_exact_numeric_evidence` | Pins 5 verbatim prose substrings of the grounding contract — breaks on any copywriting pass. | Behavioral guarantee ("contract appended on every build path") already covered by the marker-based test; convert this to a single explicitly-labeled wording-lock, or assert stable structural markers. |
<OPENHUMAN_ROOT>/plan.md:92:| ✅ | `src/openhuman/agent/prompts/mod_tests.rs::identity_section_creates_missing_workspace_files` | Also string-matches SOUL.md brand-voice prose (`"Don't validate FUD"`). | Split: (a) files created + seeded from the checked-in template (compare against template file content); (b) a narrow, labeled brand-voice lock if the phrase must stay pinned. |
<OPENHUMAN_ROOT>/plan.md:141:- **Approval gate × agent turn**: harness-level test that a Write/Destructive-class turn parks
<OPENHUMAN_ROOT>/plan.md:164:- **Approval-gate Playwright mirror**: `agent-harness-behaviors.spec.ts` exists only in the slower
<OPENHUMAN_ROOT>/plan.md:166:- **AgentAccessPanel tier cross-check**: which sub-controls are enabled/hidden per autonomy tier.
<OPENHUMAN_ROOT>/plan.md:427:  onboarding pages, AgentAccessPanel (23 tests), and every P2 slice reducer
<OPENHUMAN_ROOT>/plan.md:499:  `council_registry`, `audio_toolkit`, `agent_experience`, `http_host`, `skill_runtime`,
<OPENHUMAN_ROOT>/README.md:64:OpenHuman is three things most assistants aren't: **a brain** that builds a persistent, local memory of your world; **a fantastic orchestrator** that runs fleets of agents on durable graphs; and **a deep researcher** that sweeps your data and the web before you finish asking. Every bullet links to the deeper writeup in the [docs](https://tinyhumans.gitbook.io/openhuman/).
<OPENHUMAN_ROOT>/README.md:76:- **[Workflows](https://tinyhumans.gitbook.io/openhuman/features/workflows)**: the agent proposes the automation; you review it on a canvas and save. Durable, trigger-driven, approval-gated runs on open-source [tinyflows](https://github.com/tinyhumansai/tinyflows).
<OPENHUMAN_ROOT>/README.md:77:- **[A harness that finishes the job](https://tinyhumans.gitbook.io/openhuman/developing/architecture/agent-harness)**: checkpointed graph runs on open-source [tinyagents](https://github.com/tinyhumansai/tinyagents). Stuck agents get steered, halted ones return a root cause, and every run replays with real per-call costs.
<OPENHUMAN_ROOT>/README.md:78:- **[A split brain, always on](https://tinyhumans.gitbook.io/openhuman/features/orchestration)**: a fast reflex agent triages inbound traffic while a deep reasoning core delegates to worker fleets, steered by the subconscious.
<OPENHUMAN_ROOT>/README.md:79:- **[An agent economy](https://tinyhumans.gitbook.io/openhuman/features/tinyplace)**: a `@handle` on [tiny.place](https://tiny.place), Signal-encrypted agent-to-agent orchestration, x402 USDC bounties and trading. Keys never touch disk.
<OPENHUMAN_ROOT>/README.md:85:- **[Meeting agents](https://tinyhumans.gitbook.io/openhuman/features/mascot/meeting-agents)**: joins **Meet, Zoom, Teams, and Webex** with a face and a voice. It auto-joins from your calendar, streams a live transcript, answers by name, and files a summary with action items.
<OPENHUMAN_ROOT>/README.md:87:- **[17 messaging channels](https://tinyhumans.gitbook.io/openhuman/features/channels)**: Telegram, Discord, Slack, WhatsApp, Signal, iMessage… plus **native email** (IMAP IDLE + SMTP). Your agent reaches you where you already are.
<OPENHUMAN_ROOT>/README.md:91:- **Simple, UI-first & Human**: install to working agent in a few clicks, with no config files and no terminal. And it has [a face](https://tinyhumans.gitbook.io/openhuman/features/mascot): a mascot that speaks, reacts, and remembers you.
<OPENHUMAN_ROOT>/README.md:97:OpenHuman is the first agent harness that gets to know you in minutes. Inspired by [Karpathy's LLM Knowledgebase](https://x.com/karpathy/status/2039805659525644595). Most agents start cold. Hermes learns by watching you work; OpenClaw waits for plugins to ferry context in. Either way, you spend days or weeks before the agent knows enough about your stack to be genuinely useful.
<OPENHUMAN_ROOT>/README.md:103:> OpenHuman summarizes and compresses all your documents, emails & chats; and creates a memory graph that lets your agent remember everything about you.
<OPENHUMAN_ROOT>/README.md:107:In just one sync pass, the agent has full (compressed) context of your inbox, your calendar, your repos, your docs, your messages. No training period. No "give it a few weeks.". It becomes you, controlled by you.
<OPENHUMAN_ROOT>/README.md:109:Already self-host [agentmemory](https://github.com/rohitg00/agentmemory) across other coding agents? OpenHuman ships an optional `Memory` backend that proxies to it. Set `memory.backend = "agentmemory"` in `config.toml` and the same durable store powers OpenHuman alongside Claude Code, Cursor, Codex, and OpenCode. See the [agentmemory backend](https://tinyhumans.gitbook.io/openhuman/features/obsidian-wiki/agentmemory-backend) page for setup.
[+2 more match(es) in <OPENHUMAN_ROOT>/README.md ⟦tj:7be320ba6847129bcbe9223eb5358b18⟧]
[omitted blocks are individually retrievable: call tinyjuice_retrieve with the token inside an omission marker to expand just that block]

[compacted tool output — this is a PARTIAL view; the full original (62231 bytes) is available by calling tinyjuice_retrieve with token "737e6f05ce724e2b934b01e3874a3332" (marker ⟦tj:737e6f05ce724e2b934b01e3874a3332⟧)]