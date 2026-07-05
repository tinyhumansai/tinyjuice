[search: 500 match(es) across 26 file(s) · top 5 per file · full set via retrieve footer]
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
<OPENHUMAN_ROOT>/src/core/logging.rs:96:                Level::TRACE => Style::new().fg(Color::Magenta).dimmed().paint(tag),
<OPENHUMAN_ROOT>/src/core/logging.rs:518:        std::env::set_var("OPENHUMAN_LOG_FILE_CONSTRAINTS", "rpc, , agent ,memory");
<OPENHUMAN_ROOT>/src/core/logging.rs:520:        assert_eq!(parsed, vec!["rpc", "agent", "memory"]);
<OPENHUMAN_ROOT>/plan.md:3:Multi-agent audit of the OpenHuman test surface (2,367 files / ~25,900 test declarations per
<OPENHUMAN_ROOT>/plan.md:32:  frontend E2E (WDIO + Playwright), Rust unit (agent/memory; channels/providers/platform;
<OPENHUMAN_ROOT>/plan.md:52:| ✅ | `src/openhuman/agent/harness/harness_gap_tests.rs` | `datetime_section_is_static_grounding_rule_not_a_volatile_timestamp` | Strict subset of `agent/prompts/mod_tests.rs::datetime_section_is_static_grounding_rule_without_volatile_timestamp`; the file's own header lists item 6 as covered elsewhere. |
<OPENHUMAN_ROOT>/plan.md:91:| ✅ | `src/openhuman/agent/prompts/mod_tests.rs::grounding_contract_requires_exact_numeric_evidence` | Pins 5 verbatim prose substrings of the grounding contract — breaks on any copywriting pass. | Behavioral guarantee ("contract appended on every build path") already covered by the marker-based test; convert this to a single explicitly-labeled wording-lock, or assert stable structural markers. |
<OPENHUMAN_ROOT>/plan.md:92:| ✅ | `src/openhuman/agent/prompts/mod_tests.rs::identity_section_creates_missing_workspace_files` | Also string-matches SOUL.md brand-voice prose (`"Don't validate FUD"`). | Split: (a) files created + seeded from the checked-in template (compare against template file content); (b) a narrow, labeled brand-voice lock if the phrase must stay pinned. |
[+5 more match(es) in <OPENHUMAN_ROOT>/plan.md]
<OPENHUMAN_ROOT>/README.md:64:OpenHuman is three things most assistants aren't: **a brain** that builds a persistent, local memory of your world; **a fantastic orchestrator** that runs fleets of agents on durable graphs; and **a deep researcher** that sweeps your data and the web before you finish asking. Every bullet links to the deeper writeup in the [docs](https://tinyhumans.gitbook.io/openhuman/).
<OPENHUMAN_ROOT>/README.md:76:- **[Workflows](https://tinyhumans.gitbook.io/openhuman/features/workflows)**: the agent proposes the automation; you review it on a canvas and save. Durable, trigger-driven, approval-gated runs on open-source [tinyflows](https://github.com/tinyhumansai/tinyflows).
<OPENHUMAN_ROOT>/README.md:77:- **[A harness that finishes the job](https://tinyhumans.gitbook.io/openhuman/developing/architecture/agent-harness)**: checkpointed graph runs on open-source [tinyagents](https://github.com/tinyhumansai/tinyagents). Stuck agents get steered, halted ones return a root cause, and every run replays with real per-call costs.
<OPENHUMAN_ROOT>/README.md:78:- **[A split brain, always on](https://tinyhumans.gitbook.io/openhuman/features/orchestration)**: a fast reflex agent triages inbound traffic while a deep reasoning core delegates to worker fleets, steered by the subconscious.
<OPENHUMAN_ROOT>/README.md:79:- **[An agent economy](https://tinyhumans.gitbook.io/openhuman/features/tinyplace)**: a `@handle` on [tiny.place](https://tiny.place), Signal-encrypted agent-to-agent orchestration, x402 USDC bounties and trading. Keys never touch disk.
[+21 more match(es) in <OPENHUMAN_ROOT>/README.md]
<OPENHUMAN_ROOT>/src/core/runtime.rs:3://! A single agent turn is a very large async state machine (system prompt +
<OPENHUMAN_ROOT>/src/core/runtime.rs:5://! to a sub-agent runs another full turn one level down. Even with the inner
<OPENHUMAN_ROOT>/src/core/runtime.rs:6://! sub-agent future boxed, that nesting overflows tokio's default 2 MiB
<OPENHUMAN_ROOT>/src/core/runtime.rs:12://! an agent turn (the desktop Tauri host's runtime, `agent_cli`, the rest of
<OPENHUMAN_ROOT>/src/core/runtime.rs:14://! in sync; downstream call sites should set `.thread_stack_size(AGENT_WORKER_STACK_BYTES)`
[+2 more match(es) in <OPENHUMAN_ROOT>/src/core/runtime.rs]
<OPENHUMAN_ROOT>/CONTRIBUTING-BEGINNERS.md:19:- [Optional — Let an AI coding agent guide you](#optional--let-an-ai-coding-agent-guide-you)
<OPENHUMAN_ROOT>/CONTRIBUTING-BEGINNERS.md:402:## Optional — Let an AI coding agent guide you
<OPENHUMAN_ROOT>/CONTRIBUTING-BEGINNERS.md:404:If you use Claude Code, Cursor, AmpCode, Codex, or another coding agent, you can paste this prompt after cloning the repo:
<OPENHUMAN_ROOT>/CONTRIBUTING-BEGINNERS.md:410:AGENTS.md: https://raw.githubusercontent.com/tinyhumansai/openhuman/main/AGENTS.md
<OPENHUMAN_ROOT>/CONTRIBUTING-BEGINNERS.md:416:The agent should still ask before destructive actions like deleting files, resetting branches, or force-pushing. You are responsible for reviewing the final diff before opening a PR.
<OPENHUMAN_ROOT>/src/core/legacy_aliases.rs:130:        "openhuman.local_ai_agent_chat",
<OPENHUMAN_ROOT>/src/core/legacy_aliases.rs:131:        "openhuman.inference_agent_chat",
<OPENHUMAN_ROOT>/src/core/legacy_aliases.rs:134:        "openhuman.local_ai_agent_chat_simple",
<OPENHUMAN_ROOT>/src/core/legacy_aliases.rs:135:        "openhuman.inference_agent_chat_simple",
<OPENHUMAN_ROOT>/src/core/cli.rs:17:/// Debug/e2e agent paths can build deep async poll stacks while assembling
<OPENHUMAN_ROOT>/src/core/cli.rs:18:/// prompts, provider requests, and sub-agent tool loops.
<OPENHUMAN_ROOT>/src/core/cli.rs:83:        "agent" => {
<OPENHUMAN_ROOT>/src/core/cli.rs:85:                "[cli] dispatching to agent subcommand, args={:?}",
<OPENHUMAN_ROOT>/src/core/cli.rs:88:            crate::core::agent_cli::run_agent_command(&args[1..])
[+8 more match(es) in <OPENHUMAN_ROOT>/src/core/cli.rs]
<OPENHUMAN_ROOT>/src/core/jsonrpc.rs:110:                // here so the message is left untouched for direct (agent-tool)
<OPENHUMAN_ROOT>/src/core/jsonrpc.rs:386:/// can run (the feed, signal/messaging, etc. — backend `GraphQLAuth::Agent`
<OPENHUMAN_ROOT>/src/core/jsonrpc.rs:2139:                    log::warn!("[cron] boot seed of proactive agent jobs failed: {e}");
<OPENHUMAN_ROOT>/src/core/jsonrpc.rs:2498:        Err(err) => log::warn!("[runtime] failed to settle orphaned agent runs: {err}"),
<OPENHUMAN_ROOT>/src/core/jsonrpc.rs:2544:            "[runtime] AgentDefinitionRegistry::init_global failed: {err} — \
[+50 more match(es) in <OPENHUMAN_ROOT>/src/core/jsonrpc.rs]
<OPENHUMAN_ROOT>/src/core/event_bus/bus.rs:59:/// (e.g., an agent turn completed, a memory was stored).
<OPENHUMAN_ROOT>/src/core/mod.rs:9:pub mod agent_cli;
<OPENHUMAN_ROOT>/docs/README.ko.md:75:- **[일을 끝까지 마무리하는 하네스](https://tinyhumans.gitbook.io/openhuman/developing/architecture/agent-harness)**: 오픈 소스 [tinyagents](https://github.com/tinyhumansai/tinyagents) 기반의 체크포인트 그래프 실행입니다. 막힌 에이전트는 방향을 조정받고, 중단된 에이전트는 근본 원인을 돌려주며, 모든 실행은 호출별 실제 비용과 함께 재생됩니다.
<OPENHUMAN_ROOT>/docs/README.ko.md:83:- **[미팅 에이전트](https://tinyhumans.gitbook.io/openhuman/features/mascot/meeting-agents)**: 얼굴과 목소리를 가지고 **Meet, Zoom, Teams, Webex**에 참여합니다. 캘린더에서 자동으로 참여하고, 실시간 자막을 스트리밍하며, 이름이 불리면 대답하고, 요약과 액션 아이템을 정리합니다.
<OPENHUMAN_ROOT>/docs/README.ko.md:107:이미 다른 코딩 에이전트에서 [agentmemory](https://github.com/rohitg00/agentmemory)를 자체 호스팅하고 있나요? OpenHuman은 이를 프록시하는 선택적 `Memory` 백엔드를 제공합니다. `config.toml`에서 `memory.backend = "agentmemory"`를 설정하면 동일한 내구성 있는 저장소가 Claude Code, Cursor, Codex, OpenCode와 함께 OpenHuman을 구동합니다. 설정 방법은 [agentmemory 백엔드](https://tinyhumans.gitbook.io/openhuman/features/obsidian-wiki/agentmemory-backend) 페이지를 참조하세요.
<OPENHUMAN_ROOT>/docs/README.ko.md:119:- **루프가 아닌 그래프**: 턴은 [tinyagents](https://github.com/tinyhumansai/tinyagents) 기반의 체크포인트가 있는 그래프로 실행되어, 사람을 위해 일시 정지하고, 재시작에도 살아남고, 실행 중간에 재개합니다.
<OPENHUMAN_ROOT>/docs/README.ko.md:139:|                    | Claude Cowork     | OpenClaw          | Hermes Agent      | OpenHuman                                                                                            |
[+2 more match(es) in <OPENHUMAN_ROOT>/docs/README.ko.md]
<OPENHUMAN_ROOT>/src/core/event_bus/testing.rs:20://! [`crate::openhuman::agent::bus::mock_agent_run_turn`]) compose on top of
<OPENHUMAN_ROOT>/src/core/event_bus/testing.rs:61:/// [`crate::openhuman::agent::bus::use_real_agent_handler`] that need the
<OPENHUMAN_ROOT>/src/core/event_bus/testing.rs:62:/// real agent handler installed without racing against a stub-installing
<OPENHUMAN_ROOT>/src/core/event_bus/testing.rs:110:/// [`crate::openhuman::agent::bus::mock_agent_run_turn`]) should compose
<OPENHUMAN_ROOT>/src/core/agent_cli.rs:1://! `openhuman agent` — developer CLI for inspecting agent definitions and
<OPENHUMAN_ROOT>/src/core/agent_cli.rs:6://! agent definitions / tool registry and printing something.
<OPENHUMAN_ROOT>/src/core/agent_cli.rs:9://!   openhuman agent dump-prompt --agent <id> [--toolkit <slug>] [--workspace <path>] [--json] [--with-tools] [-v]
<OPENHUMAN_ROOT>/src/core/agent_cli.rs:10://!     (--toolkit is REQUIRED when --agent is `integrations_agent`.)
<OPENHUMAN_ROOT>/src/core/agent_cli.rs:11://!   openhuman agent dump-all --out <dir> [--workspace <path>] [--model <name>] [-v]
[+66 more match(es) in <OPENHUMAN_ROOT>/src/core/agent_cli.rs]
<OPENHUMAN_ROOT>/src/core/event_bus/native_request.rs:31://! register_native_global::<AgentTurnRequest, AgentTurnResponse, _, _>(
<OPENHUMAN_ROOT>/src/core/event_bus/native_request.rs:32://!     "agent.run_turn",
<OPENHUMAN_ROOT>/src/core/event_bus/native_request.rs:34://!         let text = run_agent_turn(/* ... */).await
<OPENHUMAN_ROOT>/src/core/event_bus/native_request.rs:36://!         Ok(AgentTurnResponse::new(text))
<OPENHUMAN_ROOT>/src/core/event_bus/native_request.rs:41://! let resp: AgentTurnResponse = request_native_global(
[+3 more match(es) in <OPENHUMAN_ROOT>/src/core/event_bus/native_request.rs]
<OPENHUMAN_ROOT>/docs/tinycortex-drift-ledger.md:9:- **HOST-OWNED** — the change lives in a layer that stays in OpenHuman (RPC, agent tools,
<OPENHUMAN_ROOT>/docs/tinycortex-drift-ledger.md:64:| D3 | `c43f79641` (07-03) (within TinyAgents migration) | `memory_store/vectors/store.rs` | `count()` reads `COUNT(*)` as `i64` and converts via `usize::try_from(...).context(...)` instead of `row.get::<usize>` directly — robustness against platform `usize`/`i64` mismatch. | **ABSENT.** `vendor/tinycortex/src/memory/store/vectors/store.rs:370–380` still does `let count: usize = ... row.get(0)` then `Ok(count)`. | `store::vectors::store` — small; port the `i64` + `try_from` guard. |
<OPENHUMAN_ROOT>/docs/tinycortex-drift-ledger.md:77:| `0304d145f` (07-03) | `memory/tools/store.rs`, `memory/tools/forget.rs` | Agent tools | Tool contract/prompt text; agent tools stay host (plan §1). |
<OPENHUMAN_ROOT>/docs/tinycortex-drift-ledger.md:82:| `c43f79641` (07-03) | `memory_search/{vector,tools}/*`, `memory_sync/composio/*` | Agent tools / live sync | Import-path churn from the TinyAgents cutover + live-sync; not engine semantics. |
<OPENHUMAN_ROOT>/docs/tinycortex-drift-ledger.md:120:its `tools/` stay host (agent tools), its `vector`/`scoring` are engine (W5) — flagged for the gap audit.
<OPENHUMAN_ROOT>/src/core/event_bus/subscriber.rs:17:    /// or `Some(&["agent", "cron"])` to receive only matching domains.
<OPENHUMAN_ROOT>/docs/README.zh-CN.md:75:- **[能把事情做完的执行框架](https://tinyhumans.gitbook.io/openhuman/developing/architecture/agent-harness)**：基于开源 [tinyagents](https://github.com/tinyhumansai/tinyagents) 的检查点式图运行：卡住的智能体会被引导回正轨，中止的会交回根因，每次运行都可回放并附带真实的每次调用成本。
<OPENHUMAN_ROOT>/docs/README.zh-CN.md:83:- **[会议智能体](https://tinyhumans.gitbook.io/openhuman/features/mascot/meeting-agents)**：带着一张脸和一副嗓音加入 **Meet、Zoom、Teams 和 Webex**：根据日历自动入会、实时输出转写字幕、被点名时回答、归档摘要和行动项。
<OPENHUMAN_ROOT>/docs/README.zh-CN.md:107:已经在其他编码智能体中自托管 [agentmemory](https://github.com/rohitg00/agentmemory)？OpenHuman 提供可选的 `Memory` 后端来代理它：在 `config.toml` 中设置 `memory.backend = "agentmemory"`，同一个持久化存储将同时服务于 OpenHuman 和 Claude Code、Cursor、Codex、OpenCode。详见 [agentmemory 后端](https://tinyhumans.gitbook.io/openhuman/features/obsidian-wiki/agentmemory-backend)页面。
<OPENHUMAN_ROOT>/docs/README.zh-CN.md:119:- **图，而非循环**：每一轮对话都作为带检查点的图在 [tinyagents](https://github.com/tinyhumansai/tinyagents) 上运行，可暂停等待人工介入、可在重启后存活、可从运行中途恢复。
<OPENHUMAN_ROOT>/docs/README.zh-CN.md:139:|                | Claude Cowork    | OpenClaw    | Hermes Agent | OpenHuman                                                                                   |
[+2 more match(es) in <OPENHUMAN_ROOT>/docs/README.zh-CN.md]
<OPENHUMAN_ROOT>/src/core/event_bus/mod.rs:5://! modules (like memory, skills, and agents) to communicate without
<OPENHUMAN_ROOT>/src/core/socketio.rs:123:    /// `"provider"` | `"openhuman_budget"` | `"agent_loop"`
<OPENHUMAN_ROOT>/src/core/socketio.rs:139:    /// agent loop) and for transport failures without a provider prefix.
<OPENHUMAN_ROOT>/src/core/socketio.rs:197:    /// Sub-agent specific progress detail. Populated on
<OPENHUMAN_ROOT>/src/core/socketio.rs:1212:                    log::debug!("[socketio] broadcast agent_meetings:error");
<OPENHUMAN_ROOT>/src/core/socketio.rs:1213:                    let _ = io_agent_meetings.emit("agent_meetings:error", &payload);
[+60 more match(es) in <OPENHUMAN_ROOT>/src/core/socketio.rs]

[compacted tool output — this is a PARTIAL view; the full original (66103 bytes) is available by calling tokenjuice_retrieve with token "e980425e292a5f93139fce4d2fcc8fcf" (marker ⟦tj:e980425e292a5f93139fce4d2fcc8fcf⟧)]