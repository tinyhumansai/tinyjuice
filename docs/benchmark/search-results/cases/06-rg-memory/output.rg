[search: 500 match(es) across 45 file(s) · top 5 per file · full set via retrieve footer]
<OPENHUMAN_ROOT>/src/main.rs:297:/// `src/openhuman/memory/safety/mod.rs`.
<OPENHUMAN_ROOT>/src/lib.rs:14:pub use openhuman::memory_store::{MemoryClient, MemoryState};
<OPENHUMAN_ROOT>/src/bin/gmail_backfill_3d.rs:1://! Backfill the last N days of Gmail into the memory-tree content store.
<OPENHUMAN_ROOT>/src/bin/gmail_backfill_3d.rs:5://! [`EmailThread`], ingests it through `ingest_page_into_memory_tree` (which
<OPENHUMAN_ROOT>/src/bin/gmail_backfill_3d.rs:39:use openhuman_core::openhuman::composio::providers::gmail::ingest::ingest_page_into_memory_tree;
<OPENHUMAN_ROOT>/src/bin/gmail_backfill_3d.rs:44:use openhuman_core::openhuman::memory_queue::drain_until_idle;
<OPENHUMAN_ROOT>/src/bin/gmail_backfill_3d.rs:45:use openhuman_core::openhuman::memory_store::chunks::store::{
[+11 more match(es) in <OPENHUMAN_ROOT>/src/bin/gmail_backfill_3d.rs]
<OPENHUMAN_ROOT>/src/bin/slack_backfill.rs:18://!   unconfigured — `memory/tree/ingest` soft-falls-back per call.
<OPENHUMAN_ROOT>/src/bin/slack_backfill.rs:24://! export OPENHUMAN_MEMORY_EMBED_ENDPOINT=http://localhost:11434
<OPENHUMAN_ROOT>/src/bin/slack_backfill.rs:25://! export OPENHUMAN_MEMORY_EMBED_MODEL=nomic-embed-text
<OPENHUMAN_ROOT>/src/bin/slack_backfill.rs:26://! export OPENHUMAN_MEMORY_EXTRACT_ENDPOINT=http://localhost:11434
<OPENHUMAN_ROOT>/src/bin/slack_backfill.rs:191:        .map_err(|e| anyhow::anyhow!("[slack_backfill] memory::global::init failed: {e}"))?;
[+21 more match(es) in <OPENHUMAN_ROOT>/src/bin/slack_backfill.rs]
<OPENHUMAN_ROOT>/src/bin/memory_tree_init_smoke.rs:1://! Manual stress smoke for the memory_tree schema-init race fix.
<OPENHUMAN_ROOT>/src/bin/memory_tree_init_smoke.rs:3://! Spins N concurrent threads racing into `memory::tree::store::with_connection`
<OPENHUMAN_ROOT>/src/bin/memory_tree_init_smoke.rs:15://!   cargo run --bin memory-tree-init-smoke -- 32
<OPENHUMAN_ROOT>/src/bin/memory_tree_init_smoke.rs:19://!   cargo run --bin memory-tree-init-smoke -- 32
<OPENHUMAN_ROOT>/src/bin/memory_tree_init_smoke.rs:33:use openhuman_core::openhuman::memory_store::chunks::store::with_connection;
[+1 more match(es) in <OPENHUMAN_ROOT>/src/bin/memory_tree_init_smoke.rs]
<OPENHUMAN_ROOT>/src/core/jsonrpc_tests.rs:89:/// `memory::tree::jobs::start` + `composio::start_periodic_sync` +
<OPENHUMAN_ROOT>/src/core/jsonrpc_tests.rs:224:async fn invoke_memory_init_accepts_empty_params() {
<OPENHUMAN_ROOT>/src/core/jsonrpc_tests.rs:228:    let result = invoke_method(default_state(), "openhuman.memory_init", json!({})).await;
<OPENHUMAN_ROOT>/src/core/jsonrpc_tests.rs:238:async fn invoke_memory_list_namespaces_rejects_unknown_param() {
<OPENHUMAN_ROOT>/src/core/jsonrpc_tests.rs:241:        "openhuman.memory_list_namespaces",
[+4 more match(es) in <OPENHUMAN_ROOT>/src/core/jsonrpc_tests.rs]
<OPENHUMAN_ROOT>/src/core/event_bus/bus.rs:59:/// (e.g., an agent turn completed, a memory was stored).
<OPENHUMAN_ROOT>/src/core/subconscious_cli.rs:113:        // Init memory client
<OPENHUMAN_ROOT>/src/core/subconscious_cli.rs:114:        let _ = crate::openhuman::memory::global::init(config.workspace_dir.clone());
<OPENHUMAN_ROOT>/src/core/subconscious_cli.rs:144:        // Create engine and run tick. The engine pulls its own memory_diff /
<OPENHUMAN_ROOT>/src/core/subconscious_cli.rs:145:        // context state from the workspace — no memory client to pass in.
<OPENHUMAN_ROOT>/src/core/subconscious_cli.rs:146:        let engine = crate::openhuman::subconscious::memory_instance(&config);
[+2 more match(es) in <OPENHUMAN_ROOT>/src/core/subconscious_cli.rs]
<OPENHUMAN_ROOT>/src/core/event_bind_tokens.rs:19://! This module owns only the in-memory store; the RPC handler that mints
<OPENHUMAN_ROOT>/src/core/event_bus/mod.rs:5://! modules (like memory, skills, and agents) to communicate without
<OPENHUMAN_ROOT>/src/core/event_bus/README.md:13:- `pub enum DomainEvent` — `events.rs` — `#[non_exhaustive]` catalog of events; current variants cover Agent (`AgentTurnStarted/Completed`, `AgentError`), Memory (`MemoryStored`, `MemoryRecalled`), Channels (`ChannelInboundMessage`, `ChannelMessageReceived/Processed`, `ChannelReactionReceived/Sent`, `ChannelConnected/Disconnected`), Cron (`CronJobTriggered/Completed`, `CronDeliveryRequested`), Skills, Tools, Webhooks, and System.
<OPENHUMAN_ROOT>/src/core/event_bus/README.md:32:- `src/openhuman/memory/conversations/bus.rs` — conversation persistence subscriber.
<OPENHUMAN_ROOT>/CONTRIBUTING-BEGINNERS.md:33:| `src/`           | Rust               | The backend brain — logic, memory, RPC |
<OPENHUMAN_ROOT>/src/core/auth.rs:6://! 1. **In-memory handoff (preferred for the in-process core)** —
<OPENHUMAN_ROOT>/src/core/auth.rs:15://! 2. **Env-as-config fallback** — when no in-memory token is supplied,
<OPENHUMAN_ROOT>/src/core/auth.rs:20://!    handing it to the binary in-memory.
<OPENHUMAN_ROOT>/src/core/auth.rs:26://! Once set, the in-memory `OnceLock` is the single source of truth — all
<OPENHUMAN_ROOT>/src/core/auth.rs:550:        // not error, must not flip the in-memory value.
[+12 more match(es) in <OPENHUMAN_ROOT>/src/core/auth.rs]
<OPENHUMAN_ROOT>/src/core/cli.rs:75:            crate::openhuman::memory_tree::tree_runtime::cli::run_tree_summarizer_command(
<OPENHUMAN_ROOT>/src/core/cli.rs:79:        "memory" => crate::core::memory_cli::run_memory_command(&args[1..]),
<OPENHUMAN_ROOT>/src/core/cli.rs:563:        "  openhuman mcp [-v|--verbose]              (stdio MCP server; read-only memory tools)"
<OPENHUMAN_ROOT>/src/core/mod.rs:20:pub mod memory_cli;
<OPENHUMAN_ROOT>/src/core/mod.rs:36:    /// Domain/group identifier, e.g. `memory`, `config`, `credentials`.
<OPENHUMAN_ROOT>/src/core/mod.rs:53:    /// Canonical dotted name for routing, e.g. `memory.doc_put`.
<OPENHUMAN_ROOT>/src/core/mod.rs:134:        let s = mk("memory", "doc_put");
<OPENHUMAN_ROOT>/src/core/mod.rs:135:        assert_eq!(s.method_name(), "memory.doc_put");
[+3 more match(es) in <OPENHUMAN_ROOT>/src/core/mod.rs]
<OPENHUMAN_ROOT>/src/core/dispatch.rs:17:///    which handles all registered controllers (memory, skills, etc.).
<OPENHUMAN_ROOT>/src/core/dispatch.rs:119:/// canonical handler exists (#3565: `openhuman.memory_tree_create_namespace`).
<OPENHUMAN_ROOT>/src/core/dispatch.rs:131:    "openhuman.memory_tree_create_namespace",
<OPENHUMAN_ROOT>/src/core/dispatch.rs:344:        assert!(try_core_dispatch(&state, "openhuman.memory_list_namespaces", json!({})).is_none());
<OPENHUMAN_ROOT>/src/core/dispatch.rs:387:            "openhuman.memory_tree_create_namespace",
[+1 more match(es) in <OPENHUMAN_ROOT>/src/core/dispatch.rs]
<OPENHUMAN_ROOT>/src/core/logging.rs:518:        std::env::set_var("OPENHUMAN_LOG_FILE_CONSTRAINTS", "rpc, , agent ,memory");
<OPENHUMAN_ROOT>/src/core/logging.rs:520:        assert_eq!(parsed, vec!["rpc", "agent", "memory"]);
<OPENHUMAN_ROOT>/src/core/memory_cli.rs:1://! `openhuman memory` — CLI for memory ingestion, graph inspection, and debugging.
<OPENHUMAN_ROOT>/src/core/memory_cli.rs:3://! Provides direct access to the memory system from the command line, including
<OPENHUMAN_ROOT>/src/core/memory_cli.rs:8://!   openhuman memory ingest  <file|->  [--namespace <ns>] [--key <key>] [--title <title>] [-v]
<OPENHUMAN_ROOT>/src/core/memory_cli.rs:9://!   openhuman memory docs    [--namespace <ns>]
<OPENHUMAN_ROOT>/src/core/memory_cli.rs:440:    crate::openhuman::memory::global::init(config.workspace_dir).map_err(anyhow::Error::msg)
[+41 more match(es) in <OPENHUMAN_ROOT>/src/core/memory_cli.rs]
<OPENHUMAN_ROOT>/src/core/event_bus/events_tests.rs:83:        // Memory
<OPENHUMAN_ROOT>/src/core/event_bus/events_tests.rs:85:            DomainEvent::MemoryStored {
<OPENHUMAN_ROOT>/src/core/event_bus/events_tests.rs:90:            "memory",
<OPENHUMAN_ROOT>/src/core/event_bus/events_tests.rs:93:            DomainEvent::MemoryRecalled {
<OPENHUMAN_ROOT>/src/core/event_bus/events_tests.rs:97:            "memory",
[+2 more match(es) in <OPENHUMAN_ROOT>/src/core/event_bus/events_tests.rs]
<OPENHUMAN_ROOT>/README.md:52:> OpenHuman is not AGI. But it is a meaningful architectural step closer, with better memory, better orchestration, and better tooling.
<OPENHUMAN_ROOT>/README.md:64:OpenHuman is three things most assistants aren't: **a brain** that builds a persistent, local memory of your world; **a fantastic orchestrator** that runs fleets of agents on durable graphs; and **a deep researcher** that sweeps your data and the web before you finish asking. Every bullet links to the deeper writeup in the [docs](https://tinyhumans.gitbook.io/openhuman/).
<OPENHUMAN_ROOT>/README.md:68:- **[Memory Tree](https://tinyhumans.gitbook.io/openhuman/features/memory-tree) + [Obsidian Wiki](https://tinyhumans.gitbook.io/openhuman/features/obsidian-wiki)**: your data compressed into scored Markdown trees in SQLite on your machine, mirrored as an [Obsidian vault](https://x.com/karpathy/status/2039805659525644595) you can open and edit. No vector-soup black box.
<OPENHUMAN_ROOT>/README.md:83:- **[SuperContext](https://tinyhumans.gitbook.io/openhuman/features/super-context)**: a research scout sweeps your memory and files before the model reads your first message. No cold starts.
<OPENHUMAN_ROOT>/README.md:100: <img src="./gitbooks/.gitbook/assets/memory.png" alt="OpenHuman context-building diagram">
[+6 more match(es) in <OPENHUMAN_ROOT>/README.md]
<OPENHUMAN_ROOT>/src/core/jsonrpc.rs:1165:        // Spawn sweep loop — bounds memory under sustained traffic.
<OPENHUMAN_ROOT>/src/core/jsonrpc.rs:1520:/// set to the domain name (agent, tool, memory, etc.).
<OPENHUMAN_ROOT>/src/core/jsonrpc.rs:1803:        // callers then get a clear "memory client not ready" error rather
<OPENHUMAN_ROOT>/src/core/jsonrpc.rs:1823:                    Err(e) => log::warn!("[boot] memory::global init failed: {e}"),
<OPENHUMAN_ROOT>/src/core/jsonrpc.rs:1885:                     Config::load_or_init failed ({e:#}). Memory persistence is \
[+25 more match(es) in <OPENHUMAN_ROOT>/src/core/jsonrpc.rs]
<OPENHUMAN_ROOT>/src/core/types.rs:86:    /// The name of the method to be invoked (e.g., `openhuman.memory_doc_put`).
<OPENHUMAN_ROOT>/src/core/legacy_aliases.rs:102:        "openhuman.update_memory_settings",
<OPENHUMAN_ROOT>/src/core/legacy_aliases.rs:103:        "openhuman.config_update_memory_settings",
<OPENHUMAN_ROOT>/src/core/legacy_aliases.rs:552:            resolve_legacy("openhuman.memory_list_namespaces"),
<OPENHUMAN_ROOT>/src/core/legacy_aliases.rs:553:            "openhuman.memory_list_namespaces"
<OPENHUMAN_ROOT>/src/core/all_tests.rs:90:    let s = schema("memory", "doc_put", vec![]);
<OPENHUMAN_ROOT>/src/core/all_tests.rs:91:    assert_eq!(rpc_method_name(&s), "openhuman.memory_doc_put");
<OPENHUMAN_ROOT>/src/core/all_tests.rs:106:    assert!(namespace_description("memory").is_some());
<OPENHUMAN_ROOT>/src/core/all_tests.rs:107:    assert!(namespace_description("memory_tree").is_some());
<OPENHUMAN_ROOT>/src/core/all_tests.rs:318:        "memory",
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:215:    // ── Memory ──────────────────────────────────────────────────────────
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:217:    /// is not installed, so the memory pipeline fell back to an alternative.
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:219:    /// Published by `memory_store::factories` (once per process via the
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:258:    /// A memory entry was stored.
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:305:    /// A memory ingestion job finished (successfully or with an error).
[+50 more match(es) in <OPENHUMAN_ROOT>/src/core/event_bus/events.rs]
<OPENHUMAN_ROOT>/src/core/socketio.rs:311:    /// uses it to reopen the full parent↔subagent conversation from memory.
<OPENHUMAN_ROOT>/src/core/socketio.rs:629:    let io_memory_sync = io.clone();
<OPENHUMAN_ROOT>/src/core/socketio.rs:927:    // 8. Memory sync stage + tree-build progress → broadcast to all clients
<OPENHUMAN_ROOT>/src/core/socketio.rs:942:                        "[socketio] event_bus not initialised after {}s — memory_sync bridge giving up",
<OPENHUMAN_ROOT>/src/core/socketio.rs:956:                        "[socketio] dropped {} event_bus events due to lag (memory_sync bridge)",
[+12 more match(es) in <OPENHUMAN_ROOT>/src/core/socketio.rs]
<OPENHUMAN_ROOT>/plan.md:32:  frontend E2E (WDIO + Playwright), Rust unit (agent/memory; channels/providers/platform;
<OPENHUMAN_ROOT>/plan.md:51:| ✅ | `app/src/services/api/{graphCentralityApi,graphCohesionApi,memoryFreshnessApi,connectionPathApi,memoryTimelineApi,entityAssociationsApi,namespaceOverviewApi}.test.ts` | the copy-pasted `exposes the public surface` test in each (7 files) | typeof-only assertions on aggregate objects **no consumer imports** (tabs import the named exports directly); the functions are behaviorally tested above in each file. Do one grep-and-delete pass. |
<OPENHUMAN_ROOT>/plan.md:78:| `src/openhuman/memory/schema_tests.rs` | registry-sync + unknown-fn tests | **False duplicate.** `memory/schema/` (singular, `memory_tree` namespace) and `memory/schemas/` (plural, `memory` namespace) are two distinct live registries with disjoint function sets. These are the *only* parity/unknown-fn guards for the `memory_tree` controller surface. |
<OPENHUMAN_ROOT>/plan.md:120:3. **`memory/read_rpc/admin.rs::delete_source_rpc` — ZERO tests** on a 427-line destructive
<OPENHUMAN_ROOT>/plan.md:135:   token; `tauri-commands.spec.ts` is happy-path only. Also add the in-memory token-handoff
[+9 more match(es) in <OPENHUMAN_ROOT>/plan.md]
<OPENHUMAN_ROOT>/AGENTS.md:25:- **Core runs in-process** as a tokio task (sidecar removed PR #1061). Lifecycle: `core_process::CoreProcessHandle` in `app/src-tauri/src/core_process.rs`. Frontend RPC → `http://127.0.0.1:<port>/rpc` with per-launch hex bearer handed in-memory via `run_server_embedded_with_ready(rpc_token: Some(_))`. Renderer reads bearer via `core_rpc_token` Tauri command. `OPENHUMAN_CORE_TOKEN` still honoured for CLI/docker/cloud. Set `OPENHUMAN_CORE_REUSE_EXISTING=1` for external core debugging.
<OPENHUMAN_ROOT>/AGENTS.md:178:Domains: `about_app`, `accessibility`, `agent`, `app_state`, `approval`, `autocomplete`, `billing`, `channels`, `composio`, `config`, `context`, `cost`, `credentials`, `cron`, `doctor`, `embeddings`, `encryption`, `health`, `heartbeat`, `integrations`, `learning`, `local_ai`, `meet`, `meet_agent`, `memory`, `migration`, `node_runtime`, `notifications`, `overlay`, `people`, `prompt_injection`, `provider_surfaces`, `providers`, `redirect_links`, `referral`, `routing`, `scheduler_gate`, `screen_intelligence`, `security`, `service`, `skills`, `socket`, `subconscious`, `team`, `text_input`, `threads`, `tokenjuice`, `tool_timeout`, `tools`, `tree_summarizer`, `update`, `voice`, `wallet`, `webhooks`, `webview_accounts`, `webview_apis`, `webview_notifications`.
<OPENHUMAN_ROOT>/AGENTS.md:186:- **Memory source identity**: per-item IDs are dedupe keys only; set `metadata.path_scope` to stable collection scope.
<OPENHUMAN_ROOT>/AGENTS.md:221:Domains: `agent`, `memory`, `channel`, `cron`, `skill`, `tool`, `webhook`, `system`.
<OPENHUMAN_ROOT>/Dockerfile:15:# keeps peak rustc memory lower than `release`; override with
<OPENHUMAN_ROOT>/src/core/all.rs:4://! controllers (e.g., memory, skills, config) and providing a unified
<OPENHUMAN_ROOT>/src/core/all.rs:43:    /// Returns the canonical RPC method name for this controller (e.g., `openhuman.memory_doc_put`).
<OPENHUMAN_ROOT>/src/core/all.rs:138:    // Persistent agent profiles (flavours): name, soul, memory sources, skills, MCP, connectors.
<OPENHUMAN_ROOT>/src/core/all.rs:230:    controllers.extend(crate::openhuman::memory::all_memory_registered_controllers());
<OPENHUMAN_ROOT>/src/core/all.rs:232:    controllers.extend(crate::openhuman::memory_goals::all_memory_goals_registered_controllers());
[+32 more match(es) in <OPENHUMAN_ROOT>/src/core/all.rs]
<OPENHUMAN_ROOT>/src/openhuman/provider_surfaces/schemas.rs:11:use crate::openhuman::memory::EmptyRequest;
<OPENHUMAN_ROOT>/pnpm-lock.yaml:3806:    deprecated: This module is not supported, and leaks memory. Do not use it. Check out lru-cache if you want a good and tested way to coalesce async requests by a key value, which is much more comprehensive and powerful.
<OPENHUMAN_ROOT>/Cargo.toml:21:name = "memory-tree-init-smoke"
<OPENHUMAN_ROOT>/Cargo.toml:22:path = "src/bin/memory_tree_init_smoke.rs"
<OPENHUMAN_ROOT>/Cargo.toml:69:# TinyCortex — Rust core for the memory engine (store/chunks/tree/retrieval/
<OPENHUMAN_ROOT>/Cargo.toml:71:# below to `vendor/tinycortex`. OpenHuman's memory subsystem migrates onto this
<OPENHUMAN_ROOT>/Cargo.toml:114:# Line-level text diffs for the memory_diff module (modified-item unified diffs).
[+5 more match(es) in <OPENHUMAN_ROOT>/Cargo.toml]
<OPENHUMAN_ROOT>/docs/README.ko.md:66:- **[메모리 트리(Memory Tree)](https://tinyhumans.gitbook.io/openhuman/features/memory-tree) + [Obsidian 위키](https://tinyhumans.gitbook.io/openhuman/features/obsidian-wiki)**: 당신의 데이터는 점수가 매겨진 Markdown 트리로 압축되어 당신의 머신에 있는 SQLite에 저장되고, 열어서 직접 편집할 수 있는 [Obsidian 볼트](https://x.com/karpathy/status/2039805659525644595)로 미러링됩니다. 벡터 수프 같은 블랙박스가 아닙니다.
<OPENHUMAN_ROOT>/docs/README.ko.md:98: <img src="../gitbooks/.gitbook/assets/memory.png" alt="OpenHuman 컨텍스트 구축 다이어그램">
<OPENHUMAN_ROOT>/docs/README.ko.md:103:OpenHuman은 기다림을 생략합니다. 계정을 연결하고, [자동 가져오기](https://tinyhumans.gitbook.io/openhuman/features/integrations/auto-fetch)가 20분 주기로 데이터를 로컬로 가져오게 한 다음, [메모리 트리](https://tinyhumans.gitbook.io/openhuman/features/memory-tree)가 모든 것을 [Karpathy 스타일의 Obsidian 위키](https://tinyhumans.gitbook.io/openhuman/features/obsidian-wiki)에 지능적으로 저장된 Markdown 파일로 압축하게 하세요.
<OPENHUMAN_ROOT>/docs/README.ko.md:107:이미 다른 코딩 에이전트에서 [agentmemory](https://github.com/rohitg00/agentmemory)를 자체 호스팅하고 있나요? OpenHuman은 이를 프록시하는 선택적 `Memory` 백엔드를 제공합니다. `config.toml`에서 `memory.backend = "agentmemory"`를 설정하면 동일한 내구성 있는 저장소가 Claude Code, Cursor, Codex, OpenCode와 함께 OpenHuman을 구동합니다. 설정 방법은 [agentmemory 백엔드](https://tinyhumans.gitbook.io/openhuman/features/obsidian-wiki/agentmemory-backend) 페이지를 참조하세요.
<OPENHUMAN_ROOT>/docs/README.ko.md:144:| **메모리**         | ✅ 채팅 범위 한정 | ⚠️ 플러그인 의존  | ✅ 자기 학습      | 🚀 메모리 트리 + Obsidian 볼트, 선택적 [agentmemory](https://github.com/rohitg00/agentmemory) 백엔드 |
<OPENHUMAN_ROOT>/src/openhuman/provider_surfaces/ops.rs:3://! This initial cut keeps state in-memory so the RPC contract and UI wiring
<OPENHUMAN_ROOT>/src/openhuman/provider_surfaces/ops.rs:6:use crate::openhuman::memory::{ApiEnvelope, ApiMeta, EmptyRequest};
<OPENHUMAN_ROOT>/src/openhuman/connectivity/rpc.rs:485:/// sources are the env, the in-memory SocketManager state, and a TCP probe.
<OPENHUMAN_ROOT>/tests/channels_web_startup_raw_coverage_e2e.rs:4://! in-memory inputs so coverage reaches production branches without real
<OPENHUMAN_ROOT>/tests/channels_web_startup_raw_coverage_e2e.rs:11:    lock_agent_handler, run_dispatch_harness, DispatchHarnessOptions, TestMemoryEntry,
<OPENHUMAN_ROOT>/tests/channels_web_startup_raw_coverage_e2e.rs:33:    config.memory.auto_save = false;
<OPENHUMAN_ROOT>/tests/channels_web_startup_raw_coverage_e2e.rs:234:    // `dispatch_harness_covers_streaming_history_timeout_and_memory_paths` on
<OPENHUMAN_ROOT>/tests/channels_web_startup_raw_coverage_e2e.rs:245:async fn dispatch_harness_covers_streaming_history_timeout_and_memory_paths() {
[+1 more match(es) in <OPENHUMAN_ROOT>/tests/channels_web_startup_raw_coverage_e2e.rs]
<OPENHUMAN_ROOT>/docs/agent-workflows/pilot-batch-example.json:10:      "title": "tighten memory namespace migration logging",
<OPENHUMAN_ROOT>/docs/agent-workflows/pilot-batch-example.json:11:      "branch": "cursor/a01-9001-memory-namespace-logging",
<OPENHUMAN_ROOT>/docs/agent-workflows/pilot-batch-example.json:12:      "owned_paths": ["src/openhuman/memory/"],
<OPENHUMAN_ROOT>/docs/tinycortex-drift-ledger.md:5:touched an engine-mapping memory module after the port line, and classifies each as:
<OPENHUMAN_ROOT>/docs/tinycortex-drift-ledger.md:36:- **≥ 2026-06-25 is captured.** Host `feat(memory_diff): back change ledger with git instead of
<OPENHUMAN_ROOT>/docs/tinycortex-drift-ledger.md:39:  (`vendor/tinycortex/src/memory/diff/diff.rs:98`, `ledger.rs`) — i.e. the post-06-25 shape. So the
<OPENHUMAN_ROOT>/docs/tinycortex-drift-ledger.md:40:  port base includes the 06-25 memory_diff work.
<OPENHUMAN_ROOT>/docs/tinycortex-drift-ledger.md:63:| D2 | `d7bee77e3` (06-30) `fix(memory-queue): classify host-FS I/O errors to stop the tree_jobs Sentry flood` | `memory_queue/worker.rs` | Adds `is_host_io_error(&anyhow::Error) -> bool` classifying **persistent** host-FS failures (EIO/ENOSPC/EROFS) distinct from transient SQLite busy/I-O, so the worker backs off and reports Sentry **once** instead of ~10k events/50min (Sentry CORE-RUST-19J). | **PARTIAL.** `vendor/tinycortex/src/memory/queue/worker.rs:89–107` has `is_sqlite_io_transient` (transient family) but **no** `is_host_io_error` (persistent host-FS family). | `queue::worker` — port the `is_host_io_error` predicate + its unit tests (EIO/ENOSPC/EROFS, context-layer, text fallback). **Only the predicate.** The Sentry-once emission and the `mark_storage_degraded` flag are host-owned (see D2-host below). |
[+28 more match(es) in <OPENHUMAN_ROOT>/docs/tinycortex-drift-ledger.md]
<OPENHUMAN_ROOT>/tests/memory_sources_readers_round21_raw_coverage_e2e.rs:5:use openhuman_core::openhuman::memory_sources::readers::SourceReader;
<OPENHUMAN_ROOT>/tests/memory_sources_readers_round21_raw_coverage_e2e.rs:6:use openhuman_core::openhuman::memory_sources::{ContentType, MemorySourceEntry, SourceKind};
<OPENHUMAN_ROOT>/tests/memory_sources_readers_round21_raw_coverage_e2e.rs:44:        .prefix("memory-sources-readers-round21-")
<OPENHUMAN_ROOT>/tests/memory_sources_readers_round21_raw_coverage_e2e.rs:57:fn source_entry(id: &str, kind: SourceKind) -> MemorySourceEntry {
<OPENHUMAN_ROOT>/tests/memory_sources_readers_round21_raw_coverage_e2e.rs:58:    MemorySourceEntry {
[+6 more match(es) in <OPENHUMAN_ROOT>/tests/memory_sources_readers_round21_raw_coverage_e2e.rs]
<OPENHUMAN_ROOT>/src/core/observability.rs:589:        return Some(ExpectedErrorKind::MemoryStoreBreakerOpen);
<OPENHUMAN_ROOT>/src/core/observability.rs:895:/// chunk aggregates: [memory_tree] circuit breaker open for <path>: too many consecutive init failures
<OPENHUMAN_ROOT>/src/core/observability.rs:2084:        ExpectedErrorKind::MemoryStoreBreakerOpen => {
<OPENHUMAN_ROOT>/src/core/observability.rs:2089:                "[observability] {domain}.{operation} skipped expected memory-store circuit-breaker-open error"
<OPENHUMAN_ROOT>/src/core/observability.rs:3613:            "Embedding API error (400 Bad Request): {\"error\":{\"message\":\"Model nvidia/nemotron-3-super-120b-a12b does not exist\",\"code\":400}} — this model isn't an embeddings model; pick an embeddings-capable model in Settings → Memory",
[+48 more match(es) in <OPENHUMAN_ROOT>/src/core/observability.rs]
<OPENHUMAN_ROOT>/src/openhuman/provider_surfaces/README.md:3:Local assistive surfaces for third-party provider apps. This domain owns a normalized provider-event model and an in-memory **respond queue** that sits above embedded webviews (and future API-first integrations), so the assistant can surface actionable items (messages, mentions, etc.) from providers like LinkedIn/Gmail in a single local-first list. The current cut is an intentionally minimal scaffold: it exposes two RPC controllers, keeps state in process memory, and is wired into the controller registry ahead of behavioral/SQLite work.
<OPENHUMAN_ROOT>/src/openhuman/provider_surfaces/README.md:19:| `src/openhuman/provider_surfaces/store.rs` | In-memory persistence: process-global `RESPOND_QUEUE` (`OnceLock<Mutex<Vec<…>>>`), `upsert_queue_item`, `list_queue_items`, `clear_queue` (test-only). |
<OPENHUMAN_ROOT>/src/openhuman/provider_surfaces/README.md:53:In-memory only. State lives in a process-global `RESPOND_QUEUE` (`static OnceLock<Mutex<Vec<RespondQueueItem>>>`) in `store.rs`, prepend-ordered (newest-first), soft-capped at `MAX_QUEUE_ITEMS = 500` (oldest dropped from the tail). Upsert dedupes by composite id `provider:account_id:event_kind:entity_id`. Module docstrings flag SQLite-backed persistence for normalized events, queue state, and local drafts as follow-up work — not yet present.
<OPENHUMAN_ROOT>/src/openhuman/provider_surfaces/README.md:57:- `crate::openhuman::memory` — `ApiEnvelope`, `ApiMeta`, `EmptyRequest` (response envelope shape + empty-request type).
<OPENHUMAN_ROOT>/src/openhuman/provider_surfaces/README.md:71:- **Scaffold, not finished domain.** Docstrings across `mod.rs`/`ops.rs`/`store.rs` explicitly call this an initial cut: state is in-memory, SQLite store + drafts + provider-specific assistive actions are deferred.
<OPENHUMAN_ROOT>/gitbooks/overview/getting-started.md:4:  choose how AI runs), and run your first request against your own Memory Tree.
<OPENHUMAN_ROOT>/gitbooks/overview/getting-started.md:43:**Know what is local and what is managed.** Your Memory Tree database, Markdown vault, workspace config, and local runtime state live on your machine. The default setup still uses OpenHuman-hosted services for sign-in, model routing, managed integration OAuth/tool calls, and web search proxying. Use the custom setup paths if you want to bring your own model, search, or Composio credentials. Some hosted features and real-time integration triggers still require the managed backend.
<OPENHUMAN_ROOT>/gitbooks/overview/getting-started.md:68:The Memory tab has a **View vault in Obsidian** button. Click it to open `<workspace>/wiki/` in [Obsidian](https://obsidian.md). You can browse the agent's summaries, drop in your own notes, and even build manual links - the agent will pick up your edits on the next ingest. See [Obsidian-Style Memory](../features/obsidian-wiki/).
<OPENHUMAN_ROOT>/gitbooks/overview/getting-started.md:74:Now that the agent has memory and a model, the rest of the product is about giving it more surfaces:
<OPENHUMAN_ROOT>/gitbooks/overview/getting-started.md:76:* [**Meeting Agents**](../features/mascot/meeting-agents.md) - drop a Google Meet link in and the mascot joins as a real participant: it listens, takes notes into the Memory Tree, speaks back into the call, and uses tools live.
<OPENHUMAN_ROOT>/tests/channels_bus_presentation_raw_coverage_e2e.rs:3://! Uses debug-only seams plus in-memory web-channel events. No external
<OPENHUMAN_ROOT>/tests/channels_bus_presentation_raw_coverage_e2e.rs:9:use openhuman_core::openhuman::agent_memory::memory_loader::MemoryCitation;
<OPENHUMAN_ROOT>/tests/channels_bus_presentation_raw_coverage_e2e.rs:36:    let citation = MemoryCitation {
<OPENHUMAN_ROOT>/tests/agent_harness_raw_coverage_e2e.rs:17:use openhuman_core::openhuman::memory::{Memory, MemoryCategory, MemoryEntry, NamespaceSummary};
<OPENHUMAN_ROOT>/tests/agent_harness_raw_coverage_e2e.rs:84:struct StubMemory;
<OPENHUMAN_ROOT>/tests/agent_harness_raw_coverage_e2e.rs:87:impl Memory for StubMemory {
<OPENHUMAN_ROOT>/tests/agent_harness_raw_coverage_e2e.rs:93:        _category: MemoryCategory,
<OPENHUMAN_ROOT>/tests/agent_harness_raw_coverage_e2e.rs:103:        _opts: openhuman_core::openhuman::memory::RecallOpts<'_>,
[+16 more match(es) in <OPENHUMAN_ROOT>/tests/agent_harness_raw_coverage_e2e.rs]

[compacted tool output — this is a PARTIAL view; the full original (70012 bytes) is available by calling tokenjuice_retrieve with token "6d9c521e46e43ce49280bf56adb1c210" (marker ⟦tj:6d9c521e46e43ce49280bf56adb1c210⟧)]