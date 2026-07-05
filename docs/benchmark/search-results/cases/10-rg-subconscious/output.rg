[search: 500 match(es) across 86 file(s) · top 5 per file · full set via retrieve footer]
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:150:    // ── Subconscious orchestrator ───────────────────────────────────────
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:151:    /// A subconscious trigger finished gate evaluation (promote or drop).
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:154:    SubconsciousTriggerProcessed {
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:1404:            Self::SubconsciousTriggerProcessed { .. } => "subconscious",
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:1463:            Self::SubconsciousTriggerProcessed { .. } => "SubconsciousTriggerProcessed",
<OPENHUMAN_ROOT>/README.md:70:- **[A subconscious](https://tinyhumans.gitbook.io/openhuman/features/subconscious)**: a background loop that diffs your world, advances your goals, and writes your morning briefing. Thinking continues after you stop typing.
<OPENHUMAN_ROOT>/README.md:78:- **[A split brain, always on](https://tinyhumans.gitbook.io/openhuman/features/orchestration)**: a fast reflex agent triages inbound traffic while a deep reasoning core delegates to worker fleets, steered by the subconscious.
<OPENHUMAN_ROOT>/AGENTS.md:178:Domains: `about_app`, `accessibility`, `agent`, `app_state`, `approval`, `autocomplete`, `billing`, `channels`, `composio`, `config`, `context`, `cost`, `credentials`, `cron`, `doctor`, `embeddings`, `encryption`, `health`, `heartbeat`, `integrations`, `learning`, `local_ai`, `meet`, `meet_agent`, `memory`, `migration`, `node_runtime`, `notifications`, `overlay`, `people`, `prompt_injection`, `provider_surfaces`, `providers`, `redirect_links`, `referral`, `routing`, `scheduler_gate`, `screen_intelligence`, `security`, `service`, `skills`, `socket`, `subconscious`, `team`, `text_input`, `threads`, `tokenjuice`, `tool_timeout`, `tools`, `tree_summarizer`, `update`, `voice`, `wallet`, `webhooks`, `webview_accounts`, `webview_apis`, `webview_notifications`.
<OPENHUMAN_ROOT>/src/core/cli.rs:80:        "subconscious" | "sub" => {
<OPENHUMAN_ROOT>/src/core/cli.rs:81:            crate::core::subconscious_cli::run_subconscious_command(&args[1..])
<OPENHUMAN_ROOT>/src/core/all.rs:282:    controllers.extend(crate::openhuman::subconscious::all_subconscious_registered_controllers());
<OPENHUMAN_ROOT>/src/core/all.rs:284:        crate::openhuman::subconscious_triggers::all_subconscious_triggers_registered_controllers(),
<OPENHUMAN_ROOT>/src/core/all.rs:461:    schemas.extend(crate::openhuman::subconscious::all_subconscious_controller_schemas());
<OPENHUMAN_ROOT>/src/core/all.rs:463:        crate::openhuman::subconscious_triggers::all_subconscious_triggers_controller_schemas(),
<OPENHUMAN_ROOT>/src/core/all.rs:615:            "Subconscious-orchestration read surface: chat windows (master/subconscious/per-session), message history, Master steering DMs, read state, and steering status.",
[+2 more match(es) in <OPENHUMAN_ROOT>/src/core/all.rs]
<OPENHUMAN_ROOT>/src/core/observability.rs:595:        return Some(ExpectedErrorKind::SubconsciousSchemaUnavailable);
<OPENHUMAN_ROOT>/src/core/observability.rs:794:/// Match subconscious-engine SQLite schema-init failures caused by the host
<OPENHUMAN_ROOT>/src/core/observability.rs:801:/// See [`ExpectedErrorKind::SubconsciousSchemaUnavailable`].
<OPENHUMAN_ROOT>/src/core/observability.rs:804:        || lower.contains("failed to open subconscious db");
<OPENHUMAN_ROOT>/src/core/observability.rs:2197:        ExpectedErrorKind::SubconsciousSchemaUnavailable => {
[+33 more match(es) in <OPENHUMAN_ROOT>/src/core/observability.rs]
<OPENHUMAN_ROOT>/docs/README.ko.md:68:- **[잠재의식(subconscious)](https://tinyhumans.gitbook.io/openhuman/features/subconscious)**: 당신의 세계의 변화를 비교 분석하고, 목표를 진전시키고, 아침 브리핑을 작성하는 백그라운드 루프입니다. 타이핑을 멈춘 후에도 생각은 계속됩니다.
<OPENHUMAN_ROOT>/src/core/subconscious_cli.rs:1://! `openhuman subconscious` — CLI for testing and debugging the subconscious loop.
<OPENHUMAN_ROOT>/src/core/subconscious_cli.rs:4://!   openhuman subconscious tick [--workspace <path>] [--mode simple|aggressive] [--verbose]
<OPENHUMAN_ROOT>/src/core/subconscious_cli.rs:5://!   openhuman subconscious status [--workspace <path>]
<OPENHUMAN_ROOT>/src/core/subconscious_cli.rs:126:                eprintln!("[subconscious] WARNING: no session token — cloud provider will fail");
<OPENHUMAN_ROOT>/src/core/subconscious_cli.rs:130:                eprintln!("[subconscious] WARNING: session token read failed: {e}");
[+29 more match(es) in <OPENHUMAN_ROOT>/src/core/subconscious_cli.rs]
<OPENHUMAN_ROOT>/src/core/jsonrpc.rs:2087:                    // Subconscious engine + heartbeat.
<OPENHUMAN_ROOT>/src/core/jsonrpc.rs:2089:                        log::info!("[subconscious] disabled by config (heartbeat.enabled = false)");
<OPENHUMAN_ROOT>/src/core/jsonrpc.rs:2091:                        match crate::openhuman::subconscious::registry::bootstrap_after_login()
<OPENHUMAN_ROOT>/src/core/jsonrpc.rs:2095:                                "[subconscious] bootstrapped on startup (existing session)"
<OPENHUMAN_ROOT>/src/core/jsonrpc.rs:2097:                            Err(e) => log::warn!("[subconscious] startup bootstrap failed: {e}"),
<OPENHUMAN_ROOT>/docs/README.zh-CN.md:68:- **[潜意识](https://tinyhumans.gitbook.io/openhuman/features/subconscious)**：一个后台循环，持续比对你的世界的变化、推进你的目标，并为你撰写晨间简报。在你停止输入之后，思考仍在继续。
<OPENHUMAN_ROOT>/src/core/mod.rs:26:pub mod subconscious_cli;
<OPENHUMAN_ROOT>/docs/TEST-COVERAGE-MATRIX.md:187:| 4.2.6 | Background-activity panel (chat-header Background tasks button)    | VU+WD    | `app/src/pages/conversations/hooks/useBackgroundActivity.test.ts`, `app/src/pages/conversations/components/__tests__/BackgroundActivityRows.test.tsx`, `app/test/e2e/specs/chat-background-activity-panel.spec.ts`                                                   | ✅     | View-only panel surfacing this chat's async sub-agents + global cron jobs, subconscious/heartbeat status, and memory syncing; freshness-only "Syncing now" labeling; E2E opens the panel and asserts its sections render and close                                                                                                                                                                                                                                                                                                          |
<OPENHUMAN_ROOT>/docs/TEST-COVERAGE-MATRIX.md:288:| 6.3.4 | Subconscious trigger pipeline (normalize → dedupe/rate → gate → queue) | RU+RI | `src/openhuman/subconscious_triggers/`, `tests/subconscious_triggers_e2e.rs`                                                                                                                                                                                                                                                      | ✅     | Event→Trigger normalization for cron/user/composio/sub-agent, dedupe TTL + per-source rate limit, LLM gate over `agent::triage`, priority queue with overflow eviction.                                                                                                                      |
<OPENHUMAN_ROOT>/docs/TEST-COVERAGE-MATRIX.md:289:| 6.3.5 | Long-lived subconscious orchestrator session                           | RU    | `src/openhuman/subconscious/session.rs`, `src/openhuman/subconscious/user_thread.rs`                                                                                                                                                                                                                                              | ✅     | Persistent compressed session backed by a reserved thread; `notify_user` handoff to the user-facing thread; mode→autonomy config parity.                                                                                                                                                     |
<OPENHUMAN_ROOT>/docs/TEST-COVERAGE-MATRIX.md:290:| 6.3.6 | Multi-party human↔subconscious↔sub-agent conversation                  | RI    | `tests/subconscious_conversation_e2e.rs`                                                                                                                                                                                                                                                                                          | ✅     | Scripted Gate/SessionExecutor seam drives delegate→sub-agent→merge, failure/retry, interleaving, dedupe, and rate-limit scenarios through the real orchestrator.                                                                                                                             |
<OPENHUMAN_ROOT>/docs/TEST-COVERAGE-MATRIX.md:291:| 6.3.7 | Full-stack trigger pipeline with mocked LLM                            | RI    | `tests/subconscious_fullstack_e2e.rs` (feature `e2e-test-support`)                                                                                                                                                                                                                                                                | ✅     | Real `GatePass`+`LongLivedSession`+`Agent`+sub-agent run against a provider-layer mock (no network); promote/drop, persistence, real `spawn_subagent`.                                                                                                                                       |
[+1 more match(es) in <OPENHUMAN_ROOT>/docs/TEST-COVERAGE-MATRIX.md]
<OPENHUMAN_ROOT>/docs/README.de.md:68:- **[Ein Unterbewusstsein](https://tinyhumans.gitbook.io/openhuman/features/subconscious)**: eine Hintergrundschleife, die Veränderungen in deiner Welt erkennt, deine Ziele vorantreibt und dein Morgen-Briefing schreibt. Das Denken geht weiter, auch wenn du längst nicht mehr tippst.
<OPENHUMAN_ROOT>/docs/README.ja-JP.md:68:- **[サブコンシャス](https://tinyhumans.gitbook.io/openhuman/features/subconscious)**: あなたの世界の差分を取り、ゴールを前進させ、モーニングブリーフィングを書くバックグラウンドループです。あなたが入力をやめた後も思考は続きます。
<OPENHUMAN_ROOT>/docs/tinycortex-migration-spec.md:33:`MemoryTaint` drives external-effect-tool gating (a tainted subconscious turn must refuse
<OPENHUMAN_ROOT>/docs/tinycortex-memory-migration-plan.md:109:**W2 — Types & trait cutover.** `memory/traits.rs` becomes re-exports of `tinycortex` types (per 0.5 decision). All 30+ external consumers (`agent/harness`, `learning`, `channels/runtime`, `subconscious`, `threads`, …) compile unchanged through the re-export. `sqlite_conn()` escape hatch on the host trait is reviewed: either upstreamed or kept as a host-side extension trait.
<OPENHUMAN_ROOT>/docs/README.ur-pk.md:82:- **[ایک لاشعور](https://tinyhumans.gitbook.io/openhuman/features/subconscious)**: ایک پس منظر لوپ جو آپ کی دنیا کا موازنہ کرتا ہے، آپ کے اہداف کو آگے بڑھاتا ہے، اور آپ کی صبح کی بریفنگ لکھتا ہے۔ آپ کے ٹائپ کرنا چھوڑنے کے بعد بھی سوچ جاری رہتی ہے۔
<OPENHUMAN_ROOT>/docs/plans/tinyflows-integration/triggers.md:7:Almost every automated behavior in OpenHuman is already shaped like *event → conditions → actions*: cron jobs, Composio app events, channel-message reactions, subconscious escalations, meeting follow-ups, notification triage. Today each has bespoke plumbing. tinyflows gives us one uniform substrate (a tinyagents graph per run, durable, observable, approval-gated), and the event bus (`src/core/event_bus/events.rs`, ~130 `DomainEvent` variants) is already the spine every one of those signals travels on.
<OPENHUMAN_ROOT>/docs/plans/tinyflows-integration/triggers.md:87:### 2.10 Subconscious escalations (`system`)
<OPENHUMAN_ROOT>/docs/plans/tinyflows-integration/triggers.md:89:Backing events: `SubconsciousTriggerProcessed`, `TriggerEvaluated`, `TriggerEscalated`.
<OPENHUMAN_ROOT>/docs/plans/tinyflows-integration/triggers.md:91:The subconscious domain is itself a mini automation engine (evaluate → escalate). Long-term convergence candidate: an escalation's *action* becomes "run flow X", making workflows the actuator layer for subconscious signals rather than a parallel system.
<OPENHUMAN_ROOT>/gitbooks/overview/getting-started.md:79:* [**Subconscious Loop**](../features/subconscious.md) - let the mascot keep working on standing tasks while you're away.
<OPENHUMAN_ROOT>/docs/plans/subconscious-factory/phase-4-factory-registry-rpc.md:3:Goal: the "make subconscious" surface — instantiate any set of worlds, drive
<OPENHUMAN_ROOT>/docs/plans/subconscious-factory/phase-4-factory-registry-rpc.md:11:pub enum SubconsciousKind { Memory, TinyPlace }
<OPENHUMAN_ROOT>/docs/plans/subconscious-factory/phase-4-factory-registry-rpc.md:13:impl SubconsciousKind {
<OPENHUMAN_ROOT>/docs/plans/subconscious-factory/phase-4-factory-registry-rpc.md:23:pub fn make_subconscious(kind: SubconsciousKind, config: &Config) -> SubconsciousInstance;
<OPENHUMAN_ROOT>/docs/plans/subconscious-factory/phase-4-factory-registry-rpc.md:26:`make_subconscious` is the *only* place profiles are constructed — tests and
[+8 more match(es) in <OPENHUMAN_ROOT>/docs/plans/subconscious-factory/phase-4-factory-registry-rpc.md]
<OPENHUMAN_ROOT>/docs/plans/subconscious-factory/README.md:1:# Subconscious factory — one reflection engine, many worlds
<OPENHUMAN_ROOT>/docs/plans/subconscious-factory/README.md:3:Redesign `src/openhuman/subconscious/` around the split-brain spec
<OPENHUMAN_ROOT>/docs/plans/subconscious-factory/README.md:4:(`state (1).md`, "Autonomous Closed-Loop LangGraph Harness"): the subconscious
<OPENHUMAN_ROOT>/docs/plans/subconscious-factory/README.md:9:instantiate a subconscious per *world*:
<OPENHUMAN_ROOT>/docs/plans/subconscious-factory/README.md:34:| Subconscious LLM / steering | `orchestration/steering.rs` + `ops::run_orchestration_review` — **but invoked inline from `subconscious::engine::tick_inner`** |
[+18 more match(es) in <OPENHUMAN_ROOT>/docs/plans/subconscious-factory/README.md]
<OPENHUMAN_ROOT>/docs/plans/subconscious-factory/phase-2-memory-profile.md:5:`SubconsciousInstance::new(MemoryProfile, config)`, and delete the old
<OPENHUMAN_ROOT>/docs/plans/subconscious-factory/phase-2-memory-profile.md:17:| `SUBCONSCIOUS_TOOL_CATALOG` | `profiles/memory.rs` |
<OPENHUMAN_ROOT>/docs/plans/subconscious-factory/phase-2-memory-profile.md:18:| `run_agent` (slim agent, `hint:subconscious`, Full autonomy, mode → iteration caps, user-message contract) | `MemoryProfile::reflect` |
<OPENHUMAN_ROOT>/docs/plans/subconscious-factory/phase-2-memory-profile.md:37:- `reflect`: returns `Reflection::Acted { response_chars }`; the SubconsciousMode
<OPENHUMAN_ROOT>/docs/plans/subconscious-factory/phase-2-memory-profile.md:45:`mod.rs` keeps exporting `SubconsciousEngine` (alias), `SubconsciousStatus`,
[+2 more match(es) in <OPENHUMAN_ROOT>/docs/plans/subconscious-factory/phase-2-memory-profile.md]
<OPENHUMAN_ROOT>/docs/plans/subconscious-factory/phase-7-ui.md:1:# Phase 7 — UI: see and interact with both subconscious kinds
<OPENHUMAN_ROOT>/docs/plans/subconscious-factory/phase-7-ui.md:4:triggerable in the app — in the **Subconscious tab** (both kinds' health and
<OPENHUMAN_ROOT>/docs/plans/subconscious-factory/phase-7-ui.md:9:orchestration tab already renders the pinned Subconscious chat window
<OPENHUMAN_ROOT>/docs/plans/subconscious-factory/phase-7-ui.md:10:(`chat.kind === 'subconscious'` in `TinyPlaceOrchestrationTab.tsx`), which is
<OPENHUMAN_ROOT>/docs/plans/subconscious-factory/phase-7-ui.md:13:## 7.1 Types + clients (`app/src/utils/tauriCommands/subconscious.ts`)
[+20 more match(es) in <OPENHUMAN_ROOT>/docs/plans/subconscious-factory/phase-7-ui.md]
<OPENHUMAN_ROOT>/docs/plans/subconscious-factory/phase-5-tests-and-docs.md:10:| ported | all of today's `engine_tests.rs` against `SubconsciousInstance<MemoryProfile>` | 2 |
<OPENHUMAN_ROOT>/docs/plans/subconscious-factory/phase-5-tests-and-docs.md:13:| factory | `enabled_kinds` gating per config; `make_subconscious` per kind | 4 |
<OPENHUMAN_ROOT>/docs/plans/subconscious-factory/phase-5-tests-and-docs.md:17:| json_rpc_e2e | extend `tests/json_rpc_e2e.rs`: `subconscious.status` shape incl. `instances` | 4 |
<OPENHUMAN_ROOT>/docs/plans/subconscious-factory/phase-5-tests-and-docs.md:25:- `src/openhuman/subconscious/README.md` — rewrite around the factory: the
<OPENHUMAN_ROOT>/docs/plans/subconscious-factory/phase-5-tests-and-docs.md:31:  driven by the tinyplace subconscious instance, not inlined in the memory
[+6 more match(es) in <OPENHUMAN_ROOT>/docs/plans/subconscious-factory/phase-5-tests-and-docs.md]
<OPENHUMAN_ROOT>/docs/plans/subconscious-factory/phase-1-profile-and-engine.md:1:# Phase 1 — `SubconsciousProfile` trait + generic instance runner
<OPENHUMAN_ROOT>/docs/plans/subconscious-factory/phase-1-profile-and-engine.md:4:this phase the existing `SubconsciousEngine` still works exactly as today; the
<OPENHUMAN_ROOT>/docs/plans/subconscious-factory/phase-1-profile-and-engine.md:10:/// One "world" a subconscious can be instantiated over.
<OPENHUMAN_ROOT>/docs/plans/subconscious-factory/phase-1-profile-and-engine.md:12:pub trait SubconsciousProfile: Send + Sync {
<OPENHUMAN_ROOT>/docs/plans/subconscious-factory/phase-1-profile-and-engine.md:82:## 1.2 `engine.rs` — `SubconsciousInstance` (generic runner **as a tinyagents graph**)
[+21 more match(es) in <OPENHUMAN_ROOT>/docs/plans/subconscious-factory/phase-1-profile-and-engine.md]
<OPENHUMAN_ROOT>/docs/plans/subconscious-factory/phase-3-tinyplace-profile.md:3:Goal: make the tiny.place/orchestration subconscious a first-class instance
<OPENHUMAN_ROOT>/docs/plans/subconscious-factory/phase-3-tinyplace-profile.md:25:  `create_chat_provider("subconscious")` chat, `SubconsciousTainted` origin,
<OPENHUMAN_ROOT>/docs/plans/subconscious-factory/phase-3-tinyplace-profile.md:27:  (`insert_steering_directive`, supersede prior, `record_subconscious_directive`
<OPENHUMAN_ROOT>/docs/plans/subconscious-factory/phase-3-tinyplace-profile.md:28:  into the local Subconscious window + event publish). Returns
<OPENHUMAN_ROOT>/docs/plans/subconscious-factory/phase-3-tinyplace-profile.md:36:- `origin`: always `SubconsciousTainted`.
[+3 more match(es) in <OPENHUMAN_ROOT>/docs/plans/subconscious-factory/phase-3-tinyplace-profile.md]
<OPENHUMAN_ROOT>/docs/plans/subconscious-factory/phase-6-tinyagents-reuse.md:3:The subconscious runner (phase 1.2) is built on `tinyagents::graph` — the same
<OPENHUMAN_ROOT>/docs/plans/subconscious-factory/phase-6-tinyagents-reuse.md:78:| 3 | Checkpoint GC / retention | **Already exists — adopted now.** `Checkpointer` already exposes `prune(thread_id, keep_last)` and `delete_thread(thread_id)` (default trait methods + sqlite/file impls, `checkpoint/mod.rs`). No upstream PR needed. `SubconsciousInstance::run_graph` now calls `delete_thread` on the tick's unique thread after the run returns, so `graph_checkpoints.db` stays bounded (test: `completed_ticks_leave_no_checkpoint_threads`). |
<OPENHUMAN_ROOT>/gitbooks/features/approval-gate.md:96:The gate is **interactive-only**. Background, triage, and cron turns carry no chat context, so there's nobody to answer a prompt. These turns are pre-authorized and pass straight through (no row, no event). Approval is only enforced for live chat turns. (The Subconscious loop has its own, separate escalation-card approval for *unsolicited* writes; see below.)
<OPENHUMAN_ROOT>/gitbooks/features/approval-gate.md:120:* [Subconscious Loop](subconscious.md): the background loop and its separate escalation approvals.
<OPENHUMAN_ROOT>/gitbooks/features/notifications-and-activity.md:70:| **Background Activity** | The subconscious engine: status bar, active tasks, approval cards, and the evaluation ledger   |
<OPENHUMAN_ROOT>/gitbooks/features/notifications-and-activity.md:73:The **Background Activity** tab embeds the subconscious loop's controls and activity log: its tick interval, mode, a manual **Run Now** trigger, and a chronological feed of every background task evaluation with a colored status dot. That loop is documented in full on the [Subconscious Loop](subconscious.md) page; the Activity hub is just its front door.
<OPENHUMAN_ROOT>/gitbooks/features/notifications-and-activity.md:94:* [Subconscious Loop](subconscious.md) covers the background engine behind the Background Activity tab.
<OPENHUMAN_ROOT>/gitbooks/features/super-context.md:72:* [Subconscious Loop](subconscious.md): the other side of "keeps thinking when you've stopped typing."
<OPENHUMAN_ROOT>/gitbooks/features/privacy-and-security.md:21:**Optional** [**Local AI**](model-routing/local-ai.md)**.** If you want embeddings and summary-tree building to stay on your machine, opt in. Heartbeat / learning / subconscious loops can be moved on-device the same way.
<OPENHUMAN_ROOT>/gitbooks/features/model-routing/local-ai.md:22:| **Subconscious**          | small chat model                  | `src/openhuman/subconscious/executor.rs` - background evaluation loop.                                                  |
<OPENHUMAN_ROOT>/gitbooks/features/model-routing/local-ai.md:71:| `local_ai.usage.subconscious`        | `false`  | Legacy preset/migration flag for the subconscious loop.                  |
<OPENHUMAN_ROOT>/gitbooks/features/model-routing/local-ai.md:82:The legacy `local_ai.usage.*` booleans are kept for presets and migration compatibility; they do not override the unified provider fields after migration. For deterministic routing, either set the workload provider field explicitly, or leave it unset / set it to `cloud` to force the default cloud route. The same provider-string pattern is used by `agentic_provider`, `coding_provider`, `memory_provider`, `embeddings_provider`, `heartbeat_provider`, `learning_provider`, and `subconscious_provider`.
<OPENHUMAN_ROOT>/gitbooks/features/model-routing/local-ai.md:101:- Keep background reflection ("subconscious") loops on-device for privacy-sensitive work.
<OPENHUMAN_ROOT>/gitbooks/features/obsidian-wiki/memory-diff.md:121:* [Subconscious Loop](../subconscious.md) - the background loop that reviews new memory changes for actionable items.
<OPENHUMAN_ROOT>/gitbooks/features/obsidian-wiki/retrieval.md:112:- [../subconscious.md](../subconscious.md) - the background loop that consumes recalled context.
<OPENHUMAN_ROOT>/src/openhuman/config/ops_tests.rs:1040:        usage_subconscious: Some(true),
<OPENHUMAN_ROOT>/src/openhuman/config/ops_tests.rs:1060:    assert!(cfg.local_ai.usage.subconscious);
<OPENHUMAN_ROOT>/src/openhuman/config/ops_tests.rs:1538:        subconscious_provider: Some(" provider-sub ".into()),
<OPENHUMAN_ROOT>/src/openhuman/config/ops_tests.rs:1553:    assert_eq!(cfg.subconscious_provider.as_deref(), Some("provider-sub"));
<OPENHUMAN_ROOT>/src/openhuman/config/ops_tests.rs:1567:        subconscious_provider: Some(" ".into()),
[+1 more match(es) in <OPENHUMAN_ROOT>/src/openhuman/config/ops_tests.rs]
<OPENHUMAN_ROOT>/src/openhuman/config/ops/model.rs:45:    pub subconscious_provider: Option<String>,
<OPENHUMAN_ROOT>/src/openhuman/config/ops/model.rs:85:    pub usage_subconscious: Option<bool>,
<OPENHUMAN_ROOT>/src/openhuman/config/ops/model.rs:239:    if let Some(s) = update.subconscious_provider {
<OPENHUMAN_ROOT>/src/openhuman/config/ops/model.rs:240:        config.subconscious_provider = normalise_provider(s);
<OPENHUMAN_ROOT>/src/openhuman/config/ops/model.rs:418:    if let Some(v) = update.usage_subconscious {
[+1 more match(es) in <OPENHUMAN_ROOT>/src/openhuman/config/ops/model.rs]
<OPENHUMAN_ROOT>/src/openhuman/config/ops/loader.rs:382:        "subconscious_provider": config.subconscious_provider,
<OPENHUMAN_ROOT>/gitbooks/features/integrations/triggers.md:92:- **`escalate`** is the heavy path. When the Triage agent decides the trigger needs real work, it hands off to the Orchestrator with a self-contained task description. The orchestrator has access to your full skill surface, tools, memory, and the [Subconscious Loop](../subconscious.md) outputs. From there it might:
<OPENHUMAN_ROOT>/gitbooks/features/integrations/triggers.md:138:* [Subconscious Loop](../subconscious.md), the background loop that uses trigger context and memory to plan ahead.
<OPENHUMAN_ROOT>/src/openhuman/config/ops/agent.rs:522:        "subconscious_enabled": level.subconscious_enabled(),
<OPENHUMAN_ROOT>/src/openhuman/config/ops/agent.rs:577:        "subconscious_enabled": level.subconscious_enabled(),
<OPENHUMAN_ROOT>/gitbooks/features/mascot/README.md:13:It is not a chrome ornament. The mascot is wired into the same pieces as the rest of the agent: voice, memory, the [subconscious loop](../subconscious.md), and the [Google Meet integration](../native-tools/voice.md). When the agent talks, the mascot is the one talking; when the agent is thinking, the mascot is the one thinking.
<OPENHUMAN_ROOT>/gitbooks/features/mascot/README.md:43:### It thinks in the background, the subconscious
<OPENHUMAN_ROOT>/gitbooks/features/mascot/README.md:45:Even when you've stopped typing, the mascot keeps thinking. The [Subconscious Loop](../subconscious.md) is a background tick that:
<OPENHUMAN_ROOT>/gitbooks/features/mascot/README.md:75:* [Subconscious Loop](../subconscious.md), what it thinks about while you're away.
<OPENHUMAN_ROOT>/src/openhuman/config/schemas_tests.rs:247:    m.insert("usage_subconscious".into(), Value::Bool(false));
<OPENHUMAN_ROOT>/src/openhuman/config/schemas_tests.rs:260:    assert_eq!(out.usage_subconscious, Some(false));
<OPENHUMAN_ROOT>/gitbooks/features/mascot/meeting-agents.md:54:- [**Subconscious Loop**](../subconscious.md) outputs - anything it has been working on in the background is already on hand.
<OPENHUMAN_ROOT>/gitbooks/features/mascot/meeting-agents.md:68:- It runs the **subconscious loop** between meetings - so when it joins your next call, it has already done the homework on what was promised in the last one.
<OPENHUMAN_ROOT>/gitbooks/features/channels.md:22:* **Outbound**: the agent's response is sent back through the same channel to your `reply_target`, threaded when the platform supports it. Channels can also deliver **proactively** (no incoming message to reply to) when fired by a [trigger](integrations/triggers.md), a cron job, or the [subconscious loop](subconscious.md). A channel only receives proactive sends if it advertises a default delivery target; channels without one are skipped rather than posted to an empty recipient.
<OPENHUMAN_ROOT>/gitbooks/features/channels.md:72:Open **Settings → Automation & Channels → Messaging Channels** to pick which channel is the **active route**: the one OpenHuman uses for proactive, recipient-less delivery (cron, triggers, subconscious). The default is the in-app **Web** chat until you change it. Setting a new default takes effect immediately, without restarting the channel runtime, and the panel shows which channel is currently active. Inbound messages always get answered on whatever channel they arrived on, regardless of the default route.
<OPENHUMAN_ROOT>/gitbooks/features/channels.md:80:* [Subconscious Loop](subconscious.md): the background loop that can reach you through the active channel.
<OPENHUMAN_ROOT>/src/openhuman/tools/ops.rs:582:    // checkpoint/last sync". Drives the subconscious tick's first stage and is
<OPENHUMAN_ROOT>/src/openhuman/tools/ops.rs:586:    // Subconscious user-facing handoff — notify_user proactive delivery.
<OPENHUMAN_ROOT>/src/openhuman/tools/ops.rs:587:    tools.extend(crate::openhuman::subconscious::user_thread::all_user_thread_tools());
<OPENHUMAN_ROOT>/gitbooks/features/tinyplace.md:28:Inbound sessions run through a **split-brain wake graph**: a fast reflex agent triages each message in seconds (reply immediately, or hand the deep reasoning core a concise brief), while the reasoning core does the real multi-step work and delegates to sub-agent workers. Long sessions stay bounded via 20:1 history compression and a rolling world-state diff, and your [subconscious loop](subconscious.md) periodically reviews the whole picture and injects a short steering directive to keep the layer aligned with *your* priorities.
<OPENHUMAN_ROOT>/gitbooks/features/tinyplace.md:43:* [Subconscious Loop](subconscious.md): the steering brain behind orchestration.
<OPENHUMAN_ROOT>/src/openhuman/config/schema/heartbeat_cron.rs:1://! Heartbeat, cron, and subconscious mode configuration.
<OPENHUMAN_ROOT>/src/openhuman/config/schema/heartbeat_cron.rs:6:/// Subconscious operating mode — controls tool access and tick frequency.
<OPENHUMAN_ROOT>/src/openhuman/config/schema/heartbeat_cron.rs:9:pub enum SubconsciousMode {
<OPENHUMAN_ROOT>/src/openhuman/config/schema/heartbeat_cron.rs:10:    /// Disabled — the subconscious loop does not run.
<OPENHUMAN_ROOT>/src/openhuman/config/schema/heartbeat_cron.rs:26:impl SubconsciousMode {
[+44 more match(es) in <OPENHUMAN_ROOT>/src/openhuman/config/schema/heartbeat_cron.rs]
<OPENHUMAN_ROOT>/gitbooks/features/subconscious.md:8:# Subconscious Loop
<OPENHUMAN_ROOT>/gitbooks/features/subconscious.md:26:│                  Subconscious Engine                    │
<OPENHUMAN_ROOT>/gitbooks/features/subconscious.md:172:Lives under **Intelligence → Subconscious**.
<OPENHUMAN_ROOT>/gitbooks/features/subconscious.md:192:The subconscious does more than housekeep. It **steers**. When your agent participates in [tiny.place orchestration sessions](tinyplace.md) (agent-to-agent collaboration), inbound traffic runs through a split-brain wake graph:
<OPENHUMAN_ROOT>/gitbooks/features/subconscious.md:198:On its periodic tick, the subconscious reviews that compressed history and world diff and injects a short, dense **steering directive** (capped at ~900 characters, expiring after ~20 reasoning cycles) into the reasoning core's system prompt. This keeps the always-on layer aligned with *your* goals. The subconscious itself is strictly offline: it never contacts anyone and never takes external actions; ticks that reacted to external changes run **tainted**, so the approval gate refuses external-effect tools.
[+1 more match(es) in <OPENHUMAN_ROOT>/gitbooks/features/subconscious.md]
<OPENHUMAN_ROOT>/src/openhuman/config/schema/orchestration.rs:70:    /// Cadence (minutes) of the `tinyplace` subconscious steering review — the
<OPENHUMAN_ROOT>/gitbooks/features/native-tools/agent-coordination.md:44:* [Subconscious Loop](../subconscious.md) - the always-on background agent thread.
<OPENHUMAN_ROOT>/vendor/tinycortex/src/memory/tree/store/hotness.rs:3://! Hotness is a read-only subconscious signal: it gates materialisation of a
<OPENHUMAN_ROOT>/src/openhuman/memory/traits.rs:11://! `ExternalSync` for unknown/corrupt values so the subconscious gate refuses
<OPENHUMAN_ROOT>/src/openhuman/memory/traits.rs:63:    /// [`MemoryTaint::ExternalSync`] so the subconscious gate can refuse
<OPENHUMAN_ROOT>/src/openhuman/memory/traits.rs:213:        // to MemoryTaint::Internal, so the gate's tainted-subconscious
<OPENHUMAN_ROOT>/src/openhuman/memory/traits.rs:247:        // restrictive `ExternalSync` so the subconscious gate refuses
<OPENHUMAN_ROOT>/src/openhuman/config/schema/local_ai.rs:26:    /// subconscious evaluation and execution.
<OPENHUMAN_ROOT>/src/openhuman/config/schema/local_ai.rs:28:    pub subconscious: bool,
<OPENHUMAN_ROOT>/src/openhuman/config/schema/local_ai.rs:37:            subconscious: false,
<OPENHUMAN_ROOT>/src/openhuman/config/schema/local_ai.rs:262:    /// **Deprecated** — read from `Config::workload_uses_local("subconscious")`.
<OPENHUMAN_ROOT>/src/openhuman/config/schema/local_ai.rs:263:    #[deprecated(note = "Use Config::workload_uses_local(\"subconscious\")")]
[+2 more match(es) in <OPENHUMAN_ROOT>/src/openhuman/config/schema/local_ai.rs]
<OPENHUMAN_ROOT>/gitbooks/features/orchestration.md:31:Inbound traffic hits a **fast reflex agent** that triages in seconds and hands a deep **reasoning core** a concise brief; the core does the multi-step work and delegates to workers. The [subconscious loop](subconscious.md) reviews compressed session history and injects steering directives, keeping the always-on layer aligned with your goals, while 20:1 compression keeps week-long sessions bounded.
<OPENHUMAN_ROOT>/gitbooks/features/orchestration.md:52:| Always-on | None | Split-brain reflex + reasoning core, subconscious steering |
<OPENHUMAN_ROOT>/gitbooks/features/orchestration.md:57:* [Workflows](workflows.md) · [Subconscious Loop](subconscious.md) · [tiny.place Agent Economy](tinyplace.md)
<OPENHUMAN_ROOT>/scripts/test-subconscious-ticks.sh:2:# End-to-end subconscious loop test with real local AI (Ollama).
<OPENHUMAN_ROOT>/scripts/test-subconscious-ticks.sh:9:FIXTURES="./tests/fixtures/subconscious"
<OPENHUMAN_ROOT>/scripts/test-subconscious-ticks.sh:23:echo "=== Subconscious Loop E2E Test ==="
<OPENHUMAN_ROOT>/scripts/test-subconscious-ticks.sh:28:OPENHUMAN_CORE_PORT="$RPC_PORT" OPENHUMAN_CORE_TOKEN="$RPC_TOKEN" "$CORE_BIN" serve > /tmp/subconscious-test.log 2>&1 &
<OPENHUMAN_ROOT>/scripts/test-subconscious-ticks.sh:81:echo "  PHASE 2: Subconscious Tick 1"
[+5 more match(es) in <OPENHUMAN_ROOT>/scripts/test-subconscious-ticks.sh]
<OPENHUMAN_ROOT>/gitbooks/features/personalization.md:123:* [Subconscious Loop](subconscious.md), the background engine that keeps thinking about your workspace between turns.
<OPENHUMAN_ROOT>/gitbooks/features/workflows.md:62:* [Subconscious Loop](subconscious.md): the background awareness layer that complements event-driven workflows.
<OPENHUMAN_ROOT>/gitbooks/features/goals-and-todos.md:41:**Autonomous idle continuation.** If a thread has an active goal and goes idle (no in-flight turn, no activity for a configured interval, e.g. 10 minutes), the [heartbeat](subconscious.md) can inject a single continuation turn that resumes the transcript and keeps working the objective. It's opt-in (`heartbeat.goal_continuation_enabled`) and guarded by a one-shot suppression flag per idle period, so the agent never self-drives into a loop.
<OPENHUMAN_ROOT>/gitbooks/features/goals-and-todos.md:77:* [Subconscious Loop](subconscious.md): the background loop that powers idle continuation and task evaluation.
<OPENHUMAN_ROOT>/gitbooks/SUMMARY.md:61:* [Subconscious Loop](features/subconscious.md)
<OPENHUMAN_ROOT>/src/openhuman/config/schema/load/migrate.rs:170:    rewrite(&mut config.subconscious_provider);
<OPENHUMAN_ROOT>/src/openhuman/config/schema/mod.rs:67:pub use heartbeat_cron::{CronConfig, HeartbeatConfig, SubconsciousMode};
<OPENHUMAN_ROOT>/vendor/tinycortex/gitbooks/conversations-and-archivist.md:110:spellings (`work` → `general`, `from_reflection` / `subconscious_tick` →
<OPENHUMAN_ROOT>/vendor/tinycortex/gitbooks/conversations-and-archivist.md:111:`subconscious`, `agent-task` / `worker` → `tasks`) and dedupes. A thread whose
<OPENHUMAN_ROOT>/gitbooks/developing/architecture/orchestration.md:1:# Subconscious orchestration layer
<OPENHUMAN_ROOT>/gitbooks/developing/architecture/orchestration.md:7:chat model, and runs an offline **subconscious** that reflects on how the world
<OPENHUMAN_ROOT>/gitbooks/developing/architecture/orchestration.md:11:Design spec: [`docs/arch-subconscious.md`](../../../docs/arch-subconscious.md) and
<OPENHUMAN_ROOT>/gitbooks/developing/architecture/orchestration.md:12:the staged plan under [`docs/plans/subconscious-orchestration/`](../../../docs/plans/subconscious-orchestration).
<OPENHUMAN_ROOT>/gitbooks/developing/architecture/orchestration.md:100:  directive, last subconscious tick, ingest-cursor lag, and last error. Message
[+7 more match(es) in <OPENHUMAN_ROOT>/gitbooks/developing/architecture/orchestration.md]
<OPENHUMAN_ROOT>/gitbooks/developing/README.md:64:For features still being built, the [Subconscious Loop](../features/subconscious.md) page covers the background task evaluation system end-to-end.
<OPENHUMAN_ROOT>/src/openhuman/config/schema/activity_level.rs:4://! cadence, heartbeat/subconscious toggles, and token budgets.
<OPENHUMAN_ROOT>/src/openhuman/config/schema/activity_level.rs:14:/// - Heartbeat & subconscious inference (disabled / enabled)
<OPENHUMAN_ROOT>/src/openhuman/config/schema/activity_level.rs:70:    /// Whether subconscious background reasoning should run.
<OPENHUMAN_ROOT>/src/openhuman/config/schema/activity_level.rs:71:    pub fn subconscious_enabled(self) -> bool {
<OPENHUMAN_ROOT>/src/openhuman/config/schema/types.rs:180:    /// cadence, heartbeat/subconscious toggles. See issue #3117.
<OPENHUMAN_ROOT>/src/openhuman/config/schema/types.rs:399:    /// Provider string for subconscious evaluation and drift checks.
<OPENHUMAN_ROOT>/src/openhuman/config/schema/types.rs:401:    pub subconscious_provider: Option<String>,
<OPENHUMAN_ROOT>/src/openhuman/config/schema/types.rs:612:    /// `"heartbeat"`, `"learning"`, `"subconscious"`.
<OPENHUMAN_ROOT>/src/openhuman/config/schema/types.rs:631:            "subconscious" => self.subconscious_provider.as_deref(),
[+1 more match(es) in <OPENHUMAN_ROOT>/src/openhuman/config/schema/types.rs]
<OPENHUMAN_ROOT>/vendor/tinycortex/src/memory/conversations/store_tests.rs:386:            id: "legacy-subconscious-thread".to_string(),
<OPENHUMAN_ROOT>/vendor/tinycortex/src/memory/conversations/store_tests.rs:387:            title: "Legacy Subconscious Chat".to_string(),
<OPENHUMAN_ROOT>/vendor/tinycortex/src/memory/conversations/store_tests.rs:391:                "subconscious_tick".to_string(),
<OPENHUMAN_ROOT>/vendor/tinycortex/src/memory/conversations/store_tests.rs:437:            .find(|t| t.id == "legacy-subconscious-thread")
<OPENHUMAN_ROOT>/vendor/tinycortex/src/memory/conversations/store_tests.rs:439:        assert_eq!(legacy.labels, vec!["subconscious"]);
<OPENHUMAN_ROOT>/vendor/tinycortex/src/memory/conversations/store.rs:204:            "from_reflection" | "subconscious_tick" => "subconscious".to_string(),
<OPENHUMAN_ROOT>/src/openhuman/channels/providers/web/session.rs:143:) -> Option<Vec<crate::openhuman::subconscious::SourceChunk>> {
<OPENHUMAN_ROOT>/tests/subconscious_fullstack_e2e.rs:5://! Unlike `subconscious_conversation_e2e.rs` (which injects scripted Gate /
<OPENHUMAN_ROOT>/tests/subconscious_fullstack_e2e.rs:18://! `cargo test --features e2e-test-support --test subconscious_fullstack_e2e -- --nocapture`
<OPENHUMAN_ROOT>/tests/subconscious_fullstack_e2e.rs:27:use openhuman_core::openhuman::config::schema::SubconsciousMode;
<OPENHUMAN_ROOT>/tests/subconscious_fullstack_e2e.rs:33:use openhuman_core::openhuman::subconscious::LongLivedSession;
<OPENHUMAN_ROOT>/tests/subconscious_fullstack_e2e.rs:34:use openhuman_core::openhuman::subconscious_triggers::{normalize, GatePass};
[+7 more match(es) in <OPENHUMAN_ROOT>/tests/subconscious_fullstack_e2e.rs]
<OPENHUMAN_ROOT>/src/openhuman/config/schemas/helpers.rs:74:    pub(super) subconscious_provider: Option<String>,
<OPENHUMAN_ROOT>/src/openhuman/config/schemas/helpers.rs:167:    pub(super) usage_subconscious: Option<bool>,
<OPENHUMAN_ROOT>/tests/config_auth_app_state_connectivity_e2e.rs:885:    config.subconscious_provider = Some("ollama:subconscious-local".into());
<OPENHUMAN_ROOT>/tests/config_auth_app_state_connectivity_e2e.rs:900:    assert!(config.workload_uses_local("subconscious"));
<OPENHUMAN_ROOT>/tests/config_auth_app_state_connectivity_e2e.rs:1093:            subconscious: true,
<OPENHUMAN_ROOT>/tests/config_auth_app_state_connectivity_e2e.rs:1106:        assert!(local_ai.use_local_for_subconscious());
<OPENHUMAN_ROOT>/tests/config_auth_app_state_connectivity_e2e.rs:2945:            "subconscious_provider": "worker-a-cloud:subconscious"
[+1 more match(es) in <OPENHUMAN_ROOT>/tests/config_auth_app_state_connectivity_e2e.rs]
<OPENHUMAN_ROOT>/src/openhuman/config/schemas/controllers.rs:399:            subconscious_provider: update.subconscious_provider,
<OPENHUMAN_ROOT>/src/openhuman/config/schemas/controllers.rs:558:            usage_subconscious: update.usage_subconscious,
<OPENHUMAN_ROOT>/src/openhuman/config/schemas/schema_defs.rs:100:                optional_string("subconscious_provider", "Provider string for subconscious evaluation."),
<OPENHUMAN_ROOT>/src/openhuman/config/schemas/schema_defs.rs:324:                    "usage_subconscious",
<OPENHUMAN_ROOT>/src/openhuman/config/schemas/schema_defs.rs:325:                    "Use the local model for subconscious evaluation (when runtime_enabled).",
<OPENHUMAN_ROOT>/src/openhuman/config/schemas/schema_defs.rs:620:            description: "Get the agent activity level (0–4) and its derived settings: sync cadence, heartbeat/subconscious toggles, token budget, estimated monthly cost.",
<OPENHUMAN_ROOT>/src/openhuman/tool_status/ops.rs:48:    //    subconscious-tainted, or the prompt's TTL expired. All are
<OPENHUMAN_ROOT>/vendor/tinycortex/paper/README.md:20:  - [Conscious and Subconscious Processing](#conscious-and-subconscious-processing)
<OPENHUMAN_ROOT>/vendor/tinycortex/paper/README.md:111:## Conscious and Subconscious Processing
<OPENHUMAN_ROOT>/vendor/tinycortex/paper/README.md:113:Another useful distinction is conscious versus subconscious processing. The conscious layer is task-facing and prompt-responsive. The subconscious layer is continuous and background: it consolidates experience, updates associations, and reweights salience between explicit tasks. This supports a key design choice in TinyCortex: recall quality depends on both query-time retrieval and ongoing maintenance cycles.
<OPENHUMAN_ROOT>/vendor/tinycortex/paper/README.md:151:Thoughts produced in Phase 2 are also **written back** as durable memory artifacts, so future cycles can retrieve both source evidence and prior latent-state summaries. Reweighting plus thought write-back implements subconscious consolidation between intervals.
<OPENHUMAN_ROOT>/vendor/tinycortex/paper/README.md:189:Biological inspiration from the human brain (Purkinje-style endogenous activity, Ebbinghaus-style decay, and conscious/subconscious separation) maps to three engineering requirements: selective integration, principled forgetting, and continuous consolidation.
<OPENHUMAN_ROOT>/vendor/tinycortex/paper/main.tex:212:\subsection{Conscious and Subconscious Processing}
<OPENHUMAN_ROOT>/vendor/tinycortex/paper/main.tex:214:Another useful distinction is conscious versus subconscious processing. The
<OPENHUMAN_ROOT>/vendor/tinycortex/paper/main.tex:215:conscious layer is task-facing and prompt-responsive. The subconscious layer
<OPENHUMAN_ROOT>/vendor/tinycortex/paper/main.tex:296:subconscious consolidation between intervals.
<OPENHUMAN_ROOT>/vendor/tinycortex/paper/main.tex:375:(Purkinje-style endogenous activity, Ebbinghaus-style decay, and conscious/subconscious separation)
<OPENHUMAN_ROOT>/tests/json_rpc_e2e.rs:5559:async fn json_rpc_subconscious_status_exposes_instances_and_trigger_takes_kind() {
<OPENHUMAN_ROOT>/tests/json_rpc_e2e.rs:5578:    // ── subconscious.status: legacy top-level fields + instances[] ──────────
<OPENHUMAN_ROOT>/tests/json_rpc_e2e.rs:5579:    let status = post_json_rpc(&rpc_base, 1101, "openhuman.subconscious_status", json!({})).await;
<OPENHUMAN_ROOT>/tests/json_rpc_e2e.rs:5580:    let result = assert_no_jsonrpc_error(&status, "subconscious_status");
<OPENHUMAN_ROOT>/tests/json_rpc_e2e.rs:5625:    let trig_result = assert_no_jsonrpc_error(&trig, "subconscious_trigger");
[+3 more match(es) in <OPENHUMAN_ROOT>/tests/json_rpc_e2e.rs]
<OPENHUMAN_ROOT>/tests/subconscious_conversation_e2e.rs:1://! Multi-party conversation e2e: **human ↔ subconscious orchestrator ↔ sub-agent**.
<OPENHUMAN_ROOT>/tests/subconscious_conversation_e2e.rs:8://! - **human → subconscious**: a `ChannelInboundMessage` is normalized,
<OPENHUMAN_ROOT>/tests/subconscious_conversation_e2e.rs:10://! - **subconscious → sub-agent**: the scripted session "spawns" a sub-agent
<OPENHUMAN_ROOT>/tests/subconscious_conversation_e2e.rs:13://! - **sub-agent → subconscious**: that conclusion is normalized, gated, and
<OPENHUMAN_ROOT>/tests/subconscious_conversation_e2e.rs:15://! - **subconscious → human**: the session calls the real `notify_user`,
[+12 more match(es) in <OPENHUMAN_ROOT>/tests/subconscious_conversation_e2e.rs]
<OPENHUMAN_ROOT>/tests/subconscious_triggers_e2e.rs:1://! End-to-end scenario simulation for the subconscious **trigger pipeline**.
<OPENHUMAN_ROOT>/tests/subconscious_triggers_e2e.rs:18://! `cargo test --test subconscious_triggers_e2e -- --nocapture`
<OPENHUMAN_ROOT>/tests/subconscious_triggers_e2e.rs:24:use openhuman_core::openhuman::subconscious::{
<OPENHUMAN_ROOT>/tests/subconscious_triggers_e2e.rs:27:use openhuman_core::openhuman::subconscious_triggers::gate::{apply_budget, map_triage_to_gate};
<OPENHUMAN_ROOT>/tests/subconscious_triggers_e2e.rs:28:use openhuman_core::openhuman::subconscious_triggers::{
[+7 more match(es) in <OPENHUMAN_ROOT>/tests/subconscious_triggers_e2e.rs]
<OPENHUMAN_ROOT>/src/openhuman/credentials/bus.rs:15://!    subconscious). Idempotent — repeat events are safe.
<OPENHUMAN_ROOT>/tests/fixtures/subconscious/README.md:1:# Subconscious Loop Test Fixtures
<OPENHUMAN_ROOT>/src/openhuman/credentials/ops.rs:455:    // the per-user path, seed the subconscious defaults and spawn the
<OPENHUMAN_ROOT>/src/openhuman/credentials/ops.rs:459:    if let Err(e) = crate::openhuman::subconscious::registry::bootstrap_after_login().await {
<OPENHUMAN_ROOT>/src/openhuman/credentials/ops.rs:460:        tracing::warn!(error = %e, "[subconscious] post-login bootstrap failed");
<OPENHUMAN_ROOT>/src/openhuman/credentials/ops.rs:461:        logs.push(format!("subconscious bootstrap warning: {e}"));
<OPENHUMAN_ROOT>/src/openhuman/credentials/ops.rs:463:        logs.push("subconscious engine bootstrapped".to_string());
[+2 more match(es) in <OPENHUMAN_ROOT>/src/openhuman/credentials/ops.rs]
<OPENHUMAN_ROOT>/src/openhuman/credentials/README.md:8:- On login: activate the user-scoped openhuman directory, purge pre-login (anonymous) conversation threads on first activation, bind memory/conversation persistence, bootstrap subconscious, and start login-gated services (local AI, voice, dictation, screen intelligence, autocomplete).
<OPENHUMAN_ROOT>/src/openhuman/credentials/README.md:9:- On logout / session-expiry: remove the JWT, clear the active-user marker, stop login-gated services, reset subconscious, and flip the scheduler-gate signed-out override.
<OPENHUMAN_ROOT>/src/openhuman/credentials/README.md:90:- `crate::openhuman::subconscious` — post-login bootstrap / user-switch reset.
<OPENHUMAN_ROOT>/src/openhuman/credentials/README.md:97:Many domains consume `AuthService` / session helpers / Composio-direct key, including: `src/core/{all,auth,jsonrpc}.rs` (controller wiring + auth gate), `src/api/jwt.rs`, `app_state/ops.rs` (session snapshot), `channels/*` (managed credentials), `composio/{client,ops}.rs` (BYO key), `config/schema/*`, `embeddings/cloud.rs`, `encryption/ops.rs`, `http_host/auth.rs`, `inference/*` (provider auth, OpenAI OAuth), `migrations/unify_ai_provider_settings.rs`, `referral/ops.rs`, `subconscious/engine.rs`, and `webhooks`.
<OPENHUMAN_ROOT>/scripts/test-rust-e2e.sh:59:  subconscious_e2e
<OPENHUMAN_ROOT>/scripts/i18n-find-english.ts:99:  "subconscious.interval.minutes",
<OPENHUMAN_ROOT>/scripts/i18n-find-english.ts:100:  "subconscious.interval.fifteenMinutes",
<OPENHUMAN_ROOT>/scripts/i18n-find-english.ts:101:  "subconscious.interval.fiveMinutes",
<OPENHUMAN_ROOT>/scripts/i18n-find-english.ts:102:  "subconscious.interval.tenMinutes",
<OPENHUMAN_ROOT>/scripts/i18n-find-english.ts:103:  "subconscious.interval.thirtyMinutes",
<OPENHUMAN_ROOT>/src/openhuman/approval/gate.rs:314:        // channel runtime, subconscious, cron, CLI) scope a typed
<OPENHUMAN_ROOT>/src/openhuman/approval/gate.rs:368:        // surface yet); trusted automation (cron, internal-only subconscious)
<OPENHUMAN_ROOT>/src/openhuman/approval/gate.rs:369:        // is allowed through unchanged; tainted subconscious — a tick whose
<OPENHUMAN_ROOT>/src/openhuman/approval/gate.rs:412:                source: TrustedAutomationSource::Subconscious,
<OPENHUMAN_ROOT>/src/openhuman/approval/gate.rs:435:                            "{POLICY_DENIED_MARKER} Tool '{tool_name}' rejected: subconscious turn \
[+7 more match(es) in <OPENHUMAN_ROOT>/src/openhuman/approval/gate.rs]

[compacted tool output — this is a PARTIAL view; the full original (80178 bytes) is available by calling tokenjuice_retrieve with token "ed38645759c1d01209ca7c0cde3f9991" (marker ⟦tj:ed38645759c1d01209ca7c0cde3f9991⟧)]