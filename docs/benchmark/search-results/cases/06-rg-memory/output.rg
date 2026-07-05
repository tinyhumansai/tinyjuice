[search: 500 match(es) across 41 file(s) · top 5 per file · full set via retrieve footer]
<OPENHUMAN_ROOT>/src/main.rs:297:/// `src/openhuman/memory/safety/mod.rs`.
<OPENHUMAN_ROOT>/src/lib.rs:14:pub use openhuman::memory_store::{MemoryClient, MemoryState};
<OPENHUMAN_ROOT>/src/bin/gmail_backfill_3d.rs:1://! Backfill the last N days of Gmail into the memory-tree content store.
<OPENHUMAN_ROOT>/src/bin/gmail_backfill_3d.rs:5://! [`EmailThread`], ingests it through `ingest_page_into_memory_tree` (which
<OPENHUMAN_ROOT>/src/bin/gmail_backfill_3d.rs:39:use openhuman_core::openhuman::composio::providers::gmail::ingest::ingest_page_into_memory_tree;
<OPENHUMAN_ROOT>/src/bin/gmail_backfill_3d.rs:44:use openhuman_core::openhuman::memory_queue::drain_until_idle;
<OPENHUMAN_ROOT>/src/bin/gmail_backfill_3d.rs:45:use openhuman_core::openhuman::memory_store::chunks::store::{
[+11 more match(es) in <OPENHUMAN_ROOT>/src/bin/gmail_backfill_3d.rs ⟦tj:b9b15f4db203fc39322f272dd7fd3419⟧]
<OPENHUMAN_ROOT>/src/bin/slack_backfill.rs:18://!   unconfigured — `memory/tree/ingest` soft-falls-back per call.
<OPENHUMAN_ROOT>/src/bin/slack_backfill.rs:24://! export OPENHUMAN_MEMORY_EMBED_ENDPOINT=http://localhost:11434
<OPENHUMAN_ROOT>/src/bin/slack_backfill.rs:25://! export OPENHUMAN_MEMORY_EMBED_MODEL=nomic-embed-text
<OPENHUMAN_ROOT>/src/bin/slack_backfill.rs:26://! export OPENHUMAN_MEMORY_EXTRACT_ENDPOINT=http://localhost:11434
<OPENHUMAN_ROOT>/src/bin/slack_backfill.rs:191:        .map_err(|e| anyhow::anyhow!("[slack_backfill] memory::global::init failed: {e}"))?;
[+21 more match(es) in <OPENHUMAN_ROOT>/src/bin/slack_backfill.rs ⟦tj:dde6af0e11e8601e250244103d6a5e65⟧]
<OPENHUMAN_ROOT>/src/bin/memory_tree_init_smoke.rs:1://! Manual stress smoke for the memory_tree schema-init race fix.
<OPENHUMAN_ROOT>/src/bin/memory_tree_init_smoke.rs:3://! Spins N concurrent threads racing into `memory::tree::store::with_connection`
<OPENHUMAN_ROOT>/src/bin/memory_tree_init_smoke.rs:15://!   cargo run --bin memory-tree-init-smoke -- 32
<OPENHUMAN_ROOT>/src/bin/memory_tree_init_smoke.rs:19://!   cargo run --bin memory-tree-init-smoke -- 32
<OPENHUMAN_ROOT>/src/bin/memory_tree_init_smoke.rs:33:use openhuman_core::openhuman::memory_store::chunks::store::with_connection;
[+1 more match(es) in <OPENHUMAN_ROOT>/src/bin/memory_tree_init_smoke.rs ⟦tj:9597d14a49e082fa6871593559678497⟧]
<OPENHUMAN_ROOT>/src/core/jsonrpc_tests.rs:89:/// `memory::tree::jobs::start` + `composio::start_periodic_sync` +
<OPENHUMAN_ROOT>/src/core/jsonrpc_tests.rs:224:async fn invoke_memory_init_accepts_empty_params() {
<OPENHUMAN_ROOT>/src/core/jsonrpc_tests.rs:228:    let result = invoke_method(default_state(), "openhuman.memory_init", json!({})).await;
<OPENHUMAN_ROOT>/src/core/jsonrpc_tests.rs:238:async fn invoke_memory_list_namespaces_rejects_unknown_param() {
<OPENHUMAN_ROOT>/src/core/jsonrpc_tests.rs:241:        "openhuman.memory_list_namespaces",
[+4 more match(es) in <OPENHUMAN_ROOT>/src/core/jsonrpc_tests.rs ⟦tj:3a725e2601699a63bd2916acf29a544b⟧]
<OPENHUMAN_ROOT>/src/core/logging.rs:518:        std::env::set_var("OPENHUMAN_LOG_FILE_CONSTRAINTS", "rpc, , agent ,memory");
<OPENHUMAN_ROOT>/src/core/logging.rs:520:        assert_eq!(parsed, vec!["rpc", "agent", "memory"]);
<OPENHUMAN_ROOT>/src/core/memory_cli.rs:1://! `openhuman memory` — CLI for memory ingestion, graph inspection, and debugging.
<OPENHUMAN_ROOT>/src/core/memory_cli.rs:3://! Provides direct access to the memory system from the command line, including
<OPENHUMAN_ROOT>/src/core/memory_cli.rs:8://!   openhuman memory ingest  <file|->  [--namespace <ns>] [--key <key>] [--title <title>] [-v]
<OPENHUMAN_ROOT>/src/core/memory_cli.rs:9://!   openhuman memory docs    [--namespace <ns>]
<OPENHUMAN_ROOT>/src/core/memory_cli.rs:440:    crate::openhuman::memory::global::init(config.workspace_dir).map_err(anyhow::Error::msg)
[+41 more match(es) in <OPENHUMAN_ROOT>/src/core/memory_cli.rs ⟦tj:6e8649052d8f5f9de5aa388cd0ab5ab0⟧]
<OPENHUMAN_ROOT>/README.md:52:> OpenHuman is not AGI. But it is a meaningful architectural step closer, with better memory, better orchestration, and better tooling.
<OPENHUMAN_ROOT>/README.md:64:OpenHuman is three things most assistants aren't: **a brain** that builds a persistent, local memory of your world; **a fantastic orchestrator** that runs fleets of agents on durable graphs; and **a deep researcher** that sweeps your data and the web before you finish asking. Every bullet links to the deeper writeup in the [docs](https://tinyhumans.gitbook.io/openhuman/).
<OPENHUMAN_ROOT>/README.md:68:- **[Memory Tree](https://tinyhumans.gitbook.io/openhuman/features/memory-tree) + [Obsidian Wiki](https://tinyhumans.gitbook.io/openhuman/features/obsidian-wiki)**: your data compressed into scored Markdown trees in SQLite on your machine, mirrored as an [Obsidian vault](https://x.com/karpathy/status/2039805659525644595) you can open and edit. No vector-soup black box.
<OPENHUMAN_ROOT>/README.md:83:- **[SuperContext](https://tinyhumans.gitbook.io/openhuman/features/super-context)**: a research scout sweeps your memory and files before the model reads your first message. No cold starts.
<OPENHUMAN_ROOT>/README.md:100: <img src="./gitbooks/.gitbook/assets/memory.png" alt="OpenHuman context-building diagram">
[+6 more match(es) in <OPENHUMAN_ROOT>/README.md ⟦tj:4217da367cd1081a16067c67d9f7a053⟧]
<OPENHUMAN_ROOT>/src/core/event_bind_tokens.rs:19://! This module owns only the in-memory store; the RPC handler that mints
<OPENHUMAN_ROOT>/plan.md:32:  frontend E2E (WDIO + Playwright), Rust unit (agent/memory; channels/providers/platform;
<OPENHUMAN_ROOT>/plan.md:51:| ✅ | `app/src/services/api/{graphCentralityApi,graphCohesionApi,memoryFreshnessApi,connectionPathApi,memoryTimelineApi,entityAssociationsApi,namespaceOverviewApi}.test.ts` | the copy-pasted `exposes the public surface` test in each (7 files) | typeof-only assertions on aggregate objects **no consumer imports** (tabs import the named exports directly); the functions are behaviorally tested above in each file. Do one grep-and-delete pass. |
<OPENHUMAN_ROOT>/plan.md:78:| `src/openhuman/memory/schema_tests.rs` | registry-sync + unknown-fn tests | **False duplicate.** `memory/schema/` (singular, `memory_tree` namespace) and `memory/schemas/` (plural, `memory` namespace) are two distinct live registries with disjoint function sets. These are the *only* parity/unknown-fn guards for the `memory_tree` controller surface. |
<OPENHUMAN_ROOT>/plan.md:120:3. **`memory/read_rpc/admin.rs::delete_source_rpc` — ZERO tests** on a 427-line destructive
<OPENHUMAN_ROOT>/plan.md:135:   token; `tauri-commands.spec.ts` is happy-path only. Also add the in-memory token-handoff
[+9 more match(es) in <OPENHUMAN_ROOT>/plan.md ⟦tj:947ca269e097e907a62d550c31c1ec63⟧]
<OPENHUMAN_ROOT>/CONTRIBUTING-BEGINNERS.md:33:| `src/`           | Rust               | The backend brain — logic, memory, RPC |
<OPENHUMAN_ROOT>/src/core/legacy_aliases.rs:102:        "openhuman.update_memory_settings",
<OPENHUMAN_ROOT>/src/core/legacy_aliases.rs:103:        "openhuman.config_update_memory_settings",
<OPENHUMAN_ROOT>/src/core/legacy_aliases.rs:552:            resolve_legacy("openhuman.memory_list_namespaces"),
<OPENHUMAN_ROOT>/src/core/legacy_aliases.rs:553:            "openhuman.memory_list_namespaces"
<OPENHUMAN_ROOT>/src/core/event_bus/bus.rs:59:/// (e.g., an agent turn completed, a memory was stored).
<OPENHUMAN_ROOT>/src/core/all_tests.rs:90:    let s = schema("memory", "doc_put", vec![]);
<OPENHUMAN_ROOT>/src/core/all_tests.rs:91:    assert_eq!(rpc_method_name(&s), "openhuman.memory_doc_put");
<OPENHUMAN_ROOT>/src/core/all_tests.rs:106:    assert!(namespace_description("memory").is_some());
<OPENHUMAN_ROOT>/src/core/all_tests.rs:107:    assert!(namespace_description("memory_tree").is_some());
<OPENHUMAN_ROOT>/src/core/all_tests.rs:318:        "memory",
<OPENHUMAN_ROOT>/src/core/cli.rs:75:            crate::openhuman::memory_tree::tree_runtime::cli::run_tree_summarizer_command(
<OPENHUMAN_ROOT>/src/core/cli.rs:79:        "memory" => crate::core::memory_cli::run_memory_command(&args[1..]),
<OPENHUMAN_ROOT>/src/core/cli.rs:563:        "  openhuman mcp [-v|--verbose]              (stdio MCP server; read-only memory tools)"
<OPENHUMAN_ROOT>/src/core/jsonrpc.rs:1165:        // Spawn sweep loop — bounds memory under sustained traffic.
<OPENHUMAN_ROOT>/src/core/jsonrpc.rs:1520:/// set to the domain name (agent, tool, memory, etc.).
<OPENHUMAN_ROOT>/src/core/jsonrpc.rs:1803:        // callers then get a clear "memory client not ready" error rather
<OPENHUMAN_ROOT>/src/core/jsonrpc.rs:1823:                    Err(e) => log::warn!("[boot] memory::global init failed: {e}"),
<OPENHUMAN_ROOT>/src/core/jsonrpc.rs:1885:                     Config::load_or_init failed ({e:#}). Memory persistence is \
[+25 more match(es) in <OPENHUMAN_ROOT>/src/core/jsonrpc.rs ⟦tj:2a2fd1f69b6f4bf4d2ec86a039352209⟧]
<OPENHUMAN_ROOT>/src/core/subconscious_cli.rs:113:        // Init memory client
<OPENHUMAN_ROOT>/src/core/subconscious_cli.rs:114:        let _ = crate::openhuman::memory::global::init(config.workspace_dir.clone());
<OPENHUMAN_ROOT>/src/core/subconscious_cli.rs:144:        // Create engine and run tick. The engine pulls its own memory_diff /
<OPENHUMAN_ROOT>/src/core/subconscious_cli.rs:145:        // context state from the workspace — no memory client to pass in.
<OPENHUMAN_ROOT>/src/core/subconscious_cli.rs:146:        let engine = crate::openhuman::subconscious::memory_instance(&config);
[+2 more match(es) in <OPENHUMAN_ROOT>/src/core/subconscious_cli.rs ⟦tj:3d8170e15c665955a1b993555caec50c⟧]
<OPENHUMAN_ROOT>/Dockerfile:15:# keeps peak rustc memory lower than `release`; override with
<OPENHUMAN_ROOT>/docs/README.ko.md:66:- **[메모리 트리(Memory Tree)](https://tinyhumans.gitbook.io/openhuman/features/memory-tree) + [Obsidian 위키](https://tinyhumans.gitbook.io/openhuman/features/obsidian-wiki)**: 당신의 데이터는 점수가 매겨진 Markdown 트리로 압축되어 당신의 머신에 있는 SQLite에 저장되고, 열어서 직접 편집할 수 있는 [Obsidian 볼트](https://x.com/karpathy/status/2039805659525644595)로 미러링됩니다. 벡터 수프 같은 블랙박스가 아닙니다.
<OPENHUMAN_ROOT>/docs/README.ko.md:98: <img src="../gitbooks/.gitbook/assets/memory.png" alt="OpenHuman 컨텍스트 구축 다이어그램">
<OPENHUMAN_ROOT>/docs/README.ko.md:103:OpenHuman은 기다림을 생략합니다. 계정을 연결하고, [자동 가져오기](https://tinyhumans.gitbook.io/openhuman/features/integrations/auto-fetch)가 20분 주기로 데이터를 로컬로 가져오게 한 다음, [메모리 트리](https://tinyhumans.gitbook.io/openhuman/features/memory-tree)가 모든 것을 [Karpathy 스타일의 Obsidian 위키](https://tinyhumans.gitbook.io/openhuman/features/obsidian-wiki)에 지능적으로 저장된 Markdown 파일로 압축하게 하세요.
<OPENHUMAN_ROOT>/docs/README.ko.md:107:이미 다른 코딩 에이전트에서 [agentmemory](https://github.com/rohitg00/agentmemory)를 자체 호스팅하고 있나요? OpenHuman은 이를 프록시하는 선택적 `Memory` 백엔드를 제공합니다. `config.toml`에서 `memory.backend = "agentmemory"`를 설정하면 동일한 내구성 있는 저장소가 Claude Code, Cursor, Codex, OpenCode와 함께 OpenHuman을 구동합니다. 설정 방법은 [agentmemory 백엔드](https://tinyhumans.gitbook.io/openhuman/features/obsidian-wiki/agentmemory-backend) 페이지를 참조하세요.
<OPENHUMAN_ROOT>/docs/README.ko.md:144:| **메모리**         | ✅ 채팅 범위 한정 | ⚠️ 플러그인 의존  | ✅ 자기 학습      | 🚀 메모리 트리 + Obsidian 볼트, 선택적 [agentmemory](https://github.com/rohitg00/agentmemory) 백엔드 |
<OPENHUMAN_ROOT>/src/core/dispatch.rs:17:///    which handles all registered controllers (memory, skills, etc.).
<OPENHUMAN_ROOT>/src/core/dispatch.rs:119:/// canonical handler exists (#3565: `openhuman.memory_tree_create_namespace`).
<OPENHUMAN_ROOT>/src/core/dispatch.rs:131:    "openhuman.memory_tree_create_namespace",
<OPENHUMAN_ROOT>/src/core/dispatch.rs:344:        assert!(try_core_dispatch(&state, "openhuman.memory_list_namespaces", json!({})).is_none());
<OPENHUMAN_ROOT>/src/core/dispatch.rs:387:            "openhuman.memory_tree_create_namespace",
[+1 more match(es) in <OPENHUMAN_ROOT>/src/core/dispatch.rs ⟦tj:3b7f98646fca600aa55dcb72902adae1⟧]
<OPENHUMAN_ROOT>/src/core/mod.rs:20:pub mod memory_cli;
<OPENHUMAN_ROOT>/src/core/mod.rs:36:    /// Domain/group identifier, e.g. `memory`, `config`, `credentials`.
<OPENHUMAN_ROOT>/src/core/mod.rs:53:    /// Canonical dotted name for routing, e.g. `memory.doc_put`.
<OPENHUMAN_ROOT>/src/core/mod.rs:134:        let s = mk("memory", "doc_put");
<OPENHUMAN_ROOT>/src/core/mod.rs:135:        assert_eq!(s.method_name(), "memory.doc_put");
[+3 more match(es) in <OPENHUMAN_ROOT>/src/core/mod.rs ⟦tj:4539e889fbd294545be6ca40160097a7⟧]
<OPENHUMAN_ROOT>/tests/memory_roundtrip_e2e.rs:1://! Memory subsystem round-trip integration test (#773 PR-A).
<OPENHUMAN_ROOT>/tests/memory_roundtrip_e2e.rs:4://! against a real local memory client backed by the workspace store under a
<OPENHUMAN_ROOT>/tests/memory_roundtrip_e2e.rs:7://! Counterpart to `app/test/e2e/specs/memory-roundtrip.spec.ts` which exercises
<OPENHUMAN_ROOT>/tests/memory_roundtrip_e2e.rs:11://! Run with: `cargo test --test memory_roundtrip_e2e`
<OPENHUMAN_ROOT>/tests/memory_roundtrip_e2e.rs:18:use openhuman_core::openhuman::memory::ops::{
[+12 more match(es) in <OPENHUMAN_ROOT>/tests/memory_roundtrip_e2e.rs ⟦tj:10015c0b6068b6fbddb9d76c005715ae⟧]
<OPENHUMAN_ROOT>/src/core/event_bus/mod.rs:5://! modules (like memory, skills, and agents) to communicate without
<OPENHUMAN_ROOT>/docs/tinycortex-drift-ledger.md:5:touched an engine-mapping memory module after the port line, and classifies each as:
<OPENHUMAN_ROOT>/docs/tinycortex-drift-ledger.md:36:- **≥ 2026-06-25 is captured.** Host `feat(memory_diff): back change ledger with git instead of
<OPENHUMAN_ROOT>/docs/tinycortex-drift-ledger.md:39:  (`vendor/tinycortex/src/memory/diff/diff.rs:98`, `ledger.rs`) — i.e. the post-06-25 shape. So the
<OPENHUMAN_ROOT>/docs/tinycortex-drift-ledger.md:40:  port base includes the 06-25 memory_diff work.
<OPENHUMAN_ROOT>/docs/tinycortex-drift-ledger.md:63:| D2 | `d7bee77e3` (06-30) `fix(memory-queue): classify host-FS I/O errors to stop the tree_jobs Sentry flood` | `memory_queue/worker.rs` | Adds `is_host_io_error(&anyhow::Error) -> bool` classifying **persistent** host-FS failures (EIO/ENOSPC/EROFS) distinct from transient SQLite busy/I-O, so the worker backs off and reports Sentry **once** instead of ~10k events/50min (Sentry CORE-RUST-19J). | **PARTIAL.** `vendor/tinycortex/src/memory/queue/worker.rs:89–107` has `is_sqlite_io_transient` (transient family) but **no** `is_host_io_error` (persistent host-FS family). | `queue::worker` — port the `is_host_io_error` predicate + its unit tests (EIO/ENOSPC/EROFS, context-layer, text fallback). **Only the predicate.** The Sentry-once emission and the `mark_storage_degraded` flag are host-owned (see D2-host below). |
[+28 more match(es) in <OPENHUMAN_ROOT>/docs/tinycortex-drift-ledger.md ⟦tj:3d8cd11e71e177ab70e45d78b85ad0eb⟧]
<OPENHUMAN_ROOT>/src/core/types.rs:86:    /// The name of the method to be invoked (e.g., `openhuman.memory_doc_put`).
<OPENHUMAN_ROOT>/src/core/event_bus/README.md:13:- `pub enum DomainEvent` — `events.rs` — `#[non_exhaustive]` catalog of events; current variants cover Agent (`AgentTurnStarted/Completed`, `AgentError`), Memory (`MemoryStored`, `MemoryRecalled`), Channels (`ChannelInboundMessage`, `ChannelMessageReceived/Processed`, `ChannelReactionReceived/Sent`, `ChannelConnected/Disconnected`), Cron (`CronJobTriggered/Completed`, `CronDeliveryRequested`), Skills, Tools, Webhooks, and System.
<OPENHUMAN_ROOT>/src/core/event_bus/README.md:32:- `src/openhuman/memory/conversations/bus.rs` — conversation persistence subscriber.
<OPENHUMAN_ROOT>/tests/agent_memory_loader_public.rs:3:use openhuman_core::openhuman::agent_memory::memory_loader::{DefaultMemoryLoader, MemoryLoader};
<OPENHUMAN_ROOT>/tests/agent_memory_loader_public.rs:4:use openhuman_core::openhuman::memory::{Memory, MemoryCategory, MemoryEntry};
<OPENHUMAN_ROOT>/tests/agent_memory_loader_public.rs:7:struct ScriptedMemory {
<OPENHUMAN_ROOT>/tests/agent_memory_loader_public.rs:8:    primary: Vec<MemoryEntry>,
<OPENHUMAN_ROOT>/tests/agent_memory_loader_public.rs:9:    working: Vec<MemoryEntry>,
[+35 more match(es) in <OPENHUMAN_ROOT>/tests/agent_memory_loader_public.rs ⟦tj:8ca501ec41a2d9384147e64db8a701bd⟧]
<OPENHUMAN_ROOT>/docs/README.zh-CN.md:66:- **[记忆树](https://tinyhumans.gitbook.io/openhuman/features/memory-tree) + [Obsidian Wiki](https://tinyhumans.gitbook.io/openhuman/features/obsidian-wiki)**：你的数据被压缩为带评分的 Markdown 树，存储在你本机的 SQLite 中，并镜像为一个你可以打开和编辑的 [Obsidian 仓库](https://x.com/karpathy/status/2039805659525644595)。没有向量浓汤式的黑箱。
<OPENHUMAN_ROOT>/docs/README.zh-CN.md:98: <img src="../gitbooks/.gitbook/assets/memory.png" alt="OpenHuman 上下文构建示意图">
<OPENHUMAN_ROOT>/docs/README.zh-CN.md:103:OpenHuman 跳过了等待期。连接你的账户，让[自动拉取](https://tinyhumans.gitbook.io/openhuman/features/integrations/auto-fetch)以 20 分钟为周期将数据拉到本地，然后由[记忆树](https://tinyhumans.gitbook.io/openhuman/features/memory-tree)将所有内容压缩为 Markdown 文件，智能存储在一个 [Karpathy 风格的 Obsidian wiki](https://tinyhumans.gitbook.io/openhuman/features/obsidian-wiki) 中。
<OPENHUMAN_ROOT>/docs/README.zh-CN.md:107:已经在其他编码智能体中自托管 [agentmemory](https://github.com/rohitg00/agentmemory)？OpenHuman 提供可选的 `Memory` 后端来代理它：在 `config.toml` 中设置 `memory.backend = "agentmemory"`，同一个持久化存储将同时服务于 OpenHuman 和 Claude Code、Cursor、Codex、OpenCode。详见 [agentmemory 后端](https://tinyhumans.gitbook.io/openhuman/features/obsidian-wiki/agentmemory-backend)页面。
<OPENHUMAN_ROOT>/docs/README.zh-CN.md:144:| **记忆**       | ✅ 对话范围      | ⚠️ 依赖插件 | ✅ 自学习    | 🚀 记忆树 + Obsidian 仓库，可选 [agentmemory](https://github.com/rohitg00/agentmemory) 后端 |
<OPENHUMAN_ROOT>/src/core/all.rs:4://! controllers (e.g., memory, skills, config) and providing a unified
<OPENHUMAN_ROOT>/src/core/all.rs:43:    /// Returns the canonical RPC method name for this controller (e.g., `openhuman.memory_doc_put`).
<OPENHUMAN_ROOT>/src/core/all.rs:138:    // Persistent agent profiles (flavours): name, soul, memory sources, skills, MCP, connectors.
<OPENHUMAN_ROOT>/src/core/all.rs:230:    controllers.extend(crate::openhuman::memory::all_memory_registered_controllers());
<OPENHUMAN_ROOT>/src/core/all.rs:232:    controllers.extend(crate::openhuman::memory_goals::all_memory_goals_registered_controllers());
[+32 more match(es) in <OPENHUMAN_ROOT>/src/core/all.rs ⟦tj:cec59b65b8cb394ac011efdab7d00a87⟧]
<OPENHUMAN_ROOT>/src/core/socketio.rs:311:    /// uses it to reopen the full parent↔subagent conversation from memory.
<OPENHUMAN_ROOT>/src/core/socketio.rs:629:    let io_memory_sync = io.clone();
<OPENHUMAN_ROOT>/src/core/socketio.rs:927:    // 8. Memory sync stage + tree-build progress → broadcast to all clients
<OPENHUMAN_ROOT>/src/core/socketio.rs:942:                        "[socketio] event_bus not initialised after {}s — memory_sync bridge giving up",
<OPENHUMAN_ROOT>/src/core/socketio.rs:956:                        "[socketio] dropped {} event_bus events due to lag (memory_sync bridge)",
[+12 more match(es) in <OPENHUMAN_ROOT>/src/core/socketio.rs ⟦tj:b140c2c9618be90a6b5f5cb822557ba3⟧]
<OPENHUMAN_ROOT>/fastlane/metadata/en-US/description.txt:3:Pair your phone with OpenHuman on your computer, then keep the conversation going from your pocket. Ask questions, send quick voice notes, and stay connected to the same assistant that understands your desktop context, memory, and connected tools.
<OPENHUMAN_ROOT>/fastlane/metadata/en-US/description.txt:11:- Keep your assistant anchored to the desktop core that owns your memory and integrations
<OPENHUMAN_ROOT>/fastlane/metadata/en-US/description.txt:19:- Your long-term workspace, memory, and integration state remain managed by your OpenHuman desktop setup
<OPENHUMAN_ROOT>/pnpm-lock.yaml:3806:    deprecated: This module is not supported, and leaks memory. Do not use it. Check out lru-cache if you want a good and tested way to coalesce async requests by a key value, which is much more comprehensive and powerful.
<OPENHUMAN_ROOT>/AGENTS.md:25:- **Core runs in-process** as a tokio task (sidecar removed PR #1061). Lifecycle: `core_process::CoreProcessHandle` in `app/src-tauri/src/core_process.rs`. Frontend RPC → `http://127.0.0.1:<port>/rpc` with per-launch hex bearer handed in-memory via `run_server_embedded_with_ready(rpc_token: Some(_))`. Renderer reads bearer via `core_rpc_token` Tauri command. `OPENHUMAN_CORE_TOKEN` still honoured for CLI/docker/cloud. Set `OPENHUMAN_CORE_REUSE_EXISTING=1` for external core debugging.
<OPENHUMAN_ROOT>/AGENTS.md:178:Domains: `about_app`, `accessibility`, `agent`, `app_state`, `approval`, `autocomplete`, `billing`, `channels`, `composio`, `config`, `context`, `cost`, `credentials`, `cron`, `doctor`, `embeddings`, `encryption`, `health`, `heartbeat`, `integrations`, `learning`, `local_ai`, `meet`, `meet_agent`, `memory`, `migration`, `node_runtime`, `notifications`, `overlay`, `people`, `prompt_injection`, `provider_surfaces`, `providers`, `redirect_links`, `referral`, `routing`, `scheduler_gate`, `screen_intelligence`, `security`, `service`, `skills`, `socket`, `subconscious`, `team`, `text_input`, `threads`, `tokenjuice`, `tool_timeout`, `tools`, `tree_summarizer`, `update`, `voice`, `wallet`, `webhooks`, `webview_accounts`, `webview_apis`, `webview_notifications`.
<OPENHUMAN_ROOT>/AGENTS.md:186:- **Memory source identity**: per-item IDs are dedupe keys only; set `metadata.path_scope` to stable collection scope.
<OPENHUMAN_ROOT>/AGENTS.md:221:Domains: `agent`, `memory`, `channel`, `cron`, `skill`, `tool`, `webhook`, `system`.
<OPENHUMAN_ROOT>/src/core/auth.rs:6://! 1. **In-memory handoff (preferred for the in-process core)** —
<OPENHUMAN_ROOT>/src/core/auth.rs:15://! 2. **Env-as-config fallback** — when no in-memory token is supplied,
<OPENHUMAN_ROOT>/src/core/auth.rs:20://!    handing it to the binary in-memory.
<OPENHUMAN_ROOT>/src/core/auth.rs:26://! Once set, the in-memory `OnceLock` is the single source of truth — all
<OPENHUMAN_ROOT>/src/core/auth.rs:550:        // not error, must not flip the in-memory value.
[+12 more match(es) in <OPENHUMAN_ROOT>/src/core/auth.rs ⟦tj:68a04a7ca456bd72c181381a422b5672⟧]
<OPENHUMAN_ROOT>/tests/agent_archivist_debug_round21_raw_coverage_e2e.rs:19:use openhuman_core::openhuman::memory::{
<OPENHUMAN_ROOT>/tests/agent_archivist_debug_round21_raw_coverage_e2e.rs:20:    Memory, MemoryCategory, MemoryEntry, NamespaceSummary, RecallOpts,
<OPENHUMAN_ROOT>/tests/agent_archivist_debug_round21_raw_coverage_e2e.rs:22:use openhuman_core::openhuman::memory_store::{events, fts5, profile, segments};
<OPENHUMAN_ROOT>/tests/agent_archivist_debug_round21_raw_coverage_e2e.rs:91:struct StubMemory;
<OPENHUMAN_ROOT>/tests/agent_archivist_debug_round21_raw_coverage_e2e.rs:94:impl Memory for StubMemory {
[+12 more match(es) in <OPENHUMAN_ROOT>/tests/agent_archivist_debug_round21_raw_coverage_e2e.rs ⟦tj:d0dd434a56c6a385e10fad0c43e9dacb⟧]
<OPENHUMAN_ROOT>/docs/README.de.md:66:- **[Memory Tree](https://tinyhumans.gitbook.io/openhuman/features/memory-tree) + [Obsidian-Wiki](https://tinyhumans.gitbook.io/openhuman/features/obsidian-wiki)**: deine Daten, komprimiert in bewertete Markdown-Bäume in SQLite auf deiner Maschine, gespiegelt als [Obsidian-Vault](https://x.com/karpathy/status/2039805659525644595), das du öffnen und editieren kannst. Keine Vektor-Suppen-Blackbox.
<OPENHUMAN_ROOT>/docs/README.de.md:98: <img src="../gitbooks/.gitbook/assets/memory.png" alt="Diagramm zum OpenHuman-Kontextaufbau">
<OPENHUMAN_ROOT>/docs/README.de.md:101:> OpenHuman fasst all deine Dokumente, E-Mails und Chats zusammen, komprimiert sie und legt einen Memory Graph an, mit dem dein Agent sich alles über dich merken kann.
<OPENHUMAN_ROOT>/docs/README.de.md:103:OpenHuman überspringt die Wartezeit. Verbinde deine Accounts, lass [Auto-Fetch](https://tinyhumans.gitbook.io/openhuman/features/integrations/auto-fetch) die Daten lokal in einer 20-Minuten-Schleife abholen, und [Memory Trees](https://tinyhumans.gitbook.io/openhuman/features/memory-tree) komprimieren alles in Markdown-Dateien, intelligent abgelegt in einem [Obsidian-Wiki im Karpathy-Stil](https://tinyhumans.gitbook.io/openhuman/features/obsidian-wiki).
<OPENHUMAN_ROOT>/docs/README.de.md:107:Du hostest [agentmemory](https://github.com/rohitg00/agentmemory) bereits selbst für andere Coding-Agenten? OpenHuman bringt ein optionales `Memory`-Backend mit, das dorthin proxyt: setze `memory.backend = "agentmemory"` in `config.toml`, und derselbe persistente Store treibt OpenHuman zusammen mit Claude Code, Cursor, Codex und OpenCode an. Setup-Details auf der Seite zum [agentmemory-Backend](https://tinyhumans.gitbook.io/openhuman/features/obsidian-wiki/agentmemory-backend).
[+2 more match(es) in <OPENHUMAN_ROOT>/docs/README.de.md ⟦tj:b3d281e208924376afc9c029bdfcd0fc⟧]
<OPENHUMAN_ROOT>/tests/agent_turn_builder_leftovers_raw_coverage_e2e.rs:7:use openhuman_core::openhuman::context::session_memory::SessionMemoryConfig;
<OPENHUMAN_ROOT>/tests/agent_turn_builder_leftovers_raw_coverage_e2e.rs:11:use openhuman_core::openhuman::memory::{
<OPENHUMAN_ROOT>/tests/agent_turn_builder_leftovers_raw_coverage_e2e.rs:12:    Memory, MemoryCategory, MemoryEntry, NamespaceSummary, RecallOpts,
<OPENHUMAN_ROOT>/tests/agent_turn_builder_leftovers_raw_coverage_e2e.rs:120:struct RecordingMemory {
<OPENHUMAN_ROOT>/tests/agent_turn_builder_leftovers_raw_coverage_e2e.rs:121:    stores: Mutex<Vec<(String, String, String, MemoryCategory)>>,
[+19 more match(es) in <OPENHUMAN_ROOT>/tests/agent_turn_builder_leftovers_raw_coverage_e2e.rs ⟦tj:61323c83946291c6c671f9e72ff2339a⟧]
<OPENHUMAN_ROOT>/vendor/tinycortex/src/lib.rs:3://! TinyCortex is the memory engine extracted from OpenHuman as a standalone,
<OPENHUMAN_ROOT>/vendor/tinycortex/src/lib.rs:8://! The public surface lives under [`memory`]. See `docs/openhuman-memory-engine-spec.md`
<OPENHUMAN_ROOT>/vendor/tinycortex/src/lib.rs:11:pub mod memory;
<OPENHUMAN_ROOT>/vendor/tinychannels/src/context.rs:21:pub const MEMORY_CONTEXT_MAX_ENTRIES: usize = 4;
<OPENHUMAN_ROOT>/vendor/tinychannels/src/context.rs:22:pub const MEMORY_CONTEXT_ENTRY_MAX_CHARS: usize = 800;
<OPENHUMAN_ROOT>/vendor/tinychannels/src/context.rs:23:pub const MEMORY_CONTEXT_MAX_CHARS: usize = 4_000;
<OPENHUMAN_ROOT>/vendor/tinychannels/src/context.rs:56:pub struct MemoryEntry {
<OPENHUMAN_ROOT>/vendor/tinychannels/src/context.rs:63:pub trait Memory: Send + Sync {
[+31 more match(es) in <OPENHUMAN_ROOT>/vendor/tinychannels/src/context.rs ⟦tj:43d66d3ef8069a635b2cca07e669854f⟧]
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:215:    // ── Memory ──────────────────────────────────────────────────────────
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:217:    /// is not installed, so the memory pipeline fell back to an alternative.
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:219:    /// Published by `memory_store::factories` (once per process via the
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:258:    /// A memory entry was stored.
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:305:    /// A memory ingestion job finished (successfully or with an error).
[+26 more match(es) in <OPENHUMAN_ROOT>/src/core/event_bus/events.rs ⟦tj:460e52d8b49529be7de98d72c060e883⟧]
[omitted blocks are individually retrievable: call tinyjuice_retrieve with the token inside an omission marker to expand just that block]

[compacted tool output — this is a PARTIAL view; the full original (69958 bytes) is available by calling tinyjuice_retrieve with token "516021797c6ccd54a724f64d42fa29e9" (marker ⟦tj:516021797c6ccd54a724f64d42fa29e9⟧)]