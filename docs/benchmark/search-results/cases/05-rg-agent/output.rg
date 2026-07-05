[search: 500 match(es) across 20 file(s) · top 5 per file · full set via retrieve footer]
<OPENHUMAN_ROOT>/src/api/config.rs:14://! caused every `/auth/*`, `/agent-integrations/*`, and `/voice/*` request to
<OPENHUMAN_ROOT>/src/api/config.rs:103:/// so `/auth/*`, `/voice/*`, and `/agent-integrations/*` never accidentally
<OPENHUMAN_ROOT>/src/api/config.rs:460:/// (`/auth/me`, `/agent-integrations/…`) which then land on
<OPENHUMAN_ROOT>/src/api/config.rs:502:/// | `https://api.tinyhumans.ai/openai/v1/…`   | `/agent-integrations/foo` | `https://api.tinyhumans.ai/agent-integrations/foo`  ← path replaced   |
<OPENHUMAN_ROOT>/src/api/config.rs:678:             /agent-integrations/* requests don't 404 against your local LLM"
[+10 more match(es) in <OPENHUMAN_ROOT>/src/api/config.rs ⟦tj:8a30994e9ec0aad6c492e17640aa6fab⟧]
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
[+9 more match(es) in <OPENHUMAN_ROOT>/src/bin/inference_probe.rs ⟦tj:f04dbcf39adf1c9406bc628b5c12bcad⟧]
<OPENHUMAN_ROOT>/src/bin/harness_subagent_audit.rs:27:    self, AuditSteerError, AuditSubagentSessionStore, DurableSubagentSession, DurableSubagentStatus,
<OPENHUMAN_ROOT>/src/bin/harness_subagent_audit.rs:91:    subagent_failed: Vec<SubagentFailedEvent>,
<OPENHUMAN_ROOT>/src/bin/harness_subagent_audit.rs:164:struct SubagentFailedEvent {
<OPENHUMAN_ROOT>/src/bin/harness_subagent_audit.rs:233:        eprintln!("[harness_subagent_audit] ERROR: {err:#}");
<OPENHUMAN_ROOT>/src/bin/harness_subagent_audit.rs:271:    let mut agent = Agent::from_config(&config).context("Agent::from_config failed")?;
[+165 more match(es) in <OPENHUMAN_ROOT>/src/bin/harness_subagent_audit.rs ⟦tj:b8566299ecc08717d8fa36a669bfba11⟧]
<OPENHUMAN_ROOT>/src/core/jsonrpc_tests.rs:93:/// semaphore, `GLOBAL_REGISTRY` agent.run_turn handler, `STARTED`
<OPENHUMAN_ROOT>/src/core/jsonrpc_tests.rs:697:    let msg = r#"[composio] list_connections failed: Backend returned 500 Internal Server Error for GET https://api.tinyhumans.ai/agent-integrations/composio/connections: 401 {"error":{"message":"Invalid API key: ak_o1Og5*****","code":10401,"slug":"HTTP_Unauthorized","status":401}}"#;
<OPENHUMAN_ROOT>/src/core/event_bus/bus.rs:59:/// (e.g., an agent turn completed, a memory was stored).
<OPENHUMAN_ROOT>/CONTRIBUTING.md:7:For deeper architecture and subsystem references, use the GitBook under [`gitbooks/developing/`](gitbooks/developing/). For coding-agent and repository-specific implementation rules, see [`AGENTS.md`](AGENTS.md) and [`CLAUDE.md`](CLAUDE.md).
<OPENHUMAN_ROOT>/CONTRIBUTING.md:201:If you only changed docs in a normal local workflow, `pnpm format:check` is usually the only validation you need. AI-authored or remote-agent PRs must still fill in the AI Authored PR Metadata section of the PR template and report any blocked commands with the exact command and error.
<OPENHUMAN_ROOT>/CONTRIBUTING.md:220:- For AI-authored or remote-agent PRs, also fill in the AI Authored PR Metadata section of the PR template.
<OPENHUMAN_ROOT>/CONTRIBUTING.md:249:├── AGENTS.md               # Coding-agent repo rules
<OPENHUMAN_ROOT>/CONTRIBUTING.md:303:If you are contributing through a coding agent or remote environment, include the metadata required by the PR template and the Codex PR checklist.
[+1 more match(es) in <OPENHUMAN_ROOT>/CONTRIBUTING.md ⟦tj:788df4d3444c71495e023c89b05774e2⟧]
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:57:    /// An error occurred during agent processing.
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:58:    AgentError {
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:85:    /// A sub-agent failed (max iterations, provider error, missing
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:88:    SubagentFailed {
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:119:    /// High-level orchestration observed a child agent failure.
[+96 more match(es) in <OPENHUMAN_ROOT>/src/core/event_bus/events.rs ⟦tj:6c09f54577a8dfd4b356722dff87e399⟧]
<OPENHUMAN_ROOT>/src/core/observability.rs:12://! being logged at error level never reach Sentry. The agent-turn path is the
<OPENHUMAN_ROOT>/src/core/observability.rs:13://! canonical example — `run_single` used to publish a `DomainEvent::AgentError`
<OPENHUMAN_ROOT>/src/core/observability.rs:101:    /// error is raised again by `agent.run_single` /
<OPENHUMAN_ROOT>/src/core/observability.rs:196:    /// `AgentError::EmptyProviderResponse` + `AgentError::skips_sentry()`
<OPENHUMAN_ROOT>/src/core/observability.rs:510:    // `report_error_or_expected`, the `domain=agent` half of the flood). Routed
[+110 more match(es) in <OPENHUMAN_ROOT>/src/core/observability.rs ⟦tj:54bbd27c23ce3d12ef351f2623e18167⟧]
<OPENHUMAN_ROOT>/src/core/event_bus/mod.rs:5://! modules (like memory, skills, and agents) to communicate without
<OPENHUMAN_ROOT>/src/core/event_bus/README.md:13:- `pub enum DomainEvent` — `events.rs` — `#[non_exhaustive]` catalog of events; current variants cover Agent (`AgentTurnStarted/Completed`, `AgentError`), Memory (`MemoryStored`, `MemoryRecalled`), Channels (`ChannelInboundMessage`, `ChannelMessageReceived/Processed`, `ChannelReactionReceived/Sent`, `ChannelConnected/Disconnected`), Cron (`CronJobTriggered/Completed`, `CronDeliveryRequested`), Skills, Tools, Webhooks, and System.
<OPENHUMAN_ROOT>/src/core/event_bus/README.md:31:- `src/openhuman/agent/bus.rs`, `agent/triage/{events,evaluator,escalation}.rs`, `agent_registry/tools/{dispatch,spawn_subagent}.rs` — agent + sub-agent events.
<OPENHUMAN_ROOT>/src/core/event_bus/README.md:41:## Emission policy (tinyagents migration, 05.3)
<OPENHUMAN_ROOT>/src/core/event_bus/README.md:43:The canonical run record is the TinyAgents event journal + status store
<OPENHUMAN_ROOT>/src/core/event_bus/README.md:48:  compression, tool-exposure, and steering signals ride the TinyAgents
[+6 more match(es) in <OPENHUMAN_ROOT>/src/core/event_bus/README.md ⟦tj:d2e65488b25a2028d5b453cb264c938d⟧]
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
[+5 more match(es) in <OPENHUMAN_ROOT>/plan.md ⟦tj:99eda843a2e3296f89f9c2e67a929ad2⟧]
<OPENHUMAN_ROOT>/README.md:64:OpenHuman is three things most assistants aren't: **a brain** that builds a persistent, local memory of your world; **a fantastic orchestrator** that runs fleets of agents on durable graphs; and **a deep researcher** that sweeps your data and the web before you finish asking. Every bullet links to the deeper writeup in the [docs](https://tinyhumans.gitbook.io/openhuman/).
<OPENHUMAN_ROOT>/README.md:76:- **[Workflows](https://tinyhumans.gitbook.io/openhuman/features/workflows)**: the agent proposes the automation; you review it on a canvas and save. Durable, trigger-driven, approval-gated runs on open-source [tinyflows](https://github.com/tinyhumansai/tinyflows).
<OPENHUMAN_ROOT>/README.md:77:- **[A harness that finishes the job](https://tinyhumans.gitbook.io/openhuman/developing/architecture/agent-harness)**: checkpointed graph runs on open-source [tinyagents](https://github.com/tinyhumansai/tinyagents). Stuck agents get steered, halted ones return a root cause, and every run replays with real per-call costs.
<OPENHUMAN_ROOT>/README.md:78:- **[A split brain, always on](https://tinyhumans.gitbook.io/openhuman/features/orchestration)**: a fast reflex agent triages inbound traffic while a deep reasoning core delegates to worker fleets, steered by the subconscious.
<OPENHUMAN_ROOT>/README.md:79:- **[An agent economy](https://tinyhumans.gitbook.io/openhuman/features/tinyplace)**: a `@handle` on [tiny.place](https://tiny.place), Signal-encrypted agent-to-agent orchestration, x402 USDC bounties and trading. Keys never touch disk.
[+9 more match(es) in <OPENHUMAN_ROOT>/README.md ⟦tj:0da82d216047ba3dd892f6052106dfe8⟧]
[omitted blocks are individually retrievable: call tinyjuice_retrieve with the token inside an omission marker to expand just that block]

[compacted tool output — this is a PARTIAL view; the full original (62231 bytes) is available by calling tinyjuice_retrieve with token "737e6f05ce724e2b934b01e3874a3332" (marker ⟦tj:737e6f05ce724e2b934b01e3874a3332⟧)]