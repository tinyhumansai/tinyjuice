<OPENHUMAN_ROOT>/src/main.rs:297:/// `src/openhuman/memory/safety/mod.rs`.
<OPENHUMAN_ROOT>/src/lib.rs:14:pub use openhuman::memory_store::{MemoryClient, MemoryState};
<OPENHUMAN_ROOT>/src/bin/gmail_backfill_3d.rs:1://! Backfill the last N days of Gmail into the memory-tree content store.
<OPENHUMAN_ROOT>/src/bin/gmail_backfill_3d.rs:5://! [`EmailThread`], ingests it through `ingest_page_into_memory_tree` (which
<OPENHUMAN_ROOT>/src/bin/gmail_backfill_3d.rs:39:use openhuman_core::openhuman::composio::providers::gmail::ingest::ingest_page_into_memory_tree;
<OPENHUMAN_ROOT>/src/bin/gmail_backfill_3d.rs:44:use openhuman_core::openhuman::memory_queue::drain_until_idle;
<OPENHUMAN_ROOT>/src/bin/gmail_backfill_3d.rs:45:use openhuman_core::openhuman::memory_store::chunks::store::{
<OPENHUMAN_ROOT>/src/bin/gmail_backfill_3d.rs:48:use openhuman_core::openhuman::memory_store::content::read::{
<OPENHUMAN_ROOT>/src/bin/gmail_backfill_3d.rs:55:    about = "Backfill last N days of Gmail into the memory-tree content store (.md files + SQLite)."
<OPENHUMAN_ROOT>/src/bin/gmail_backfill_3d.rs:123:        wipe_memory_tree_state(&config)?;
<OPENHUMAN_ROOT>/src/bin/gmail_backfill_3d.rs:172:    let content_root = config.memory_tree_content_root();
<OPENHUMAN_ROOT>/src/bin/gmail_backfill_3d.rs:244:            ingest_page_into_memory_tree(&config, &owner, None, &messages).await?;
<OPENHUMAN_ROOT>/src/bin/gmail_backfill_3d.rs:326:/// Wipe `<workspace>/memory_tree/chunks.db` (+ wal/shm) and
<OPENHUMAN_ROOT>/src/bin/gmail_backfill_3d.rs:331:fn wipe_memory_tree_state(config: &Config) -> Result<()> {
<OPENHUMAN_ROOT>/src/bin/gmail_backfill_3d.rs:332:    let mt_dir = config.workspace_dir.join("memory_tree");
<OPENHUMAN_ROOT>/src/bin/gmail_backfill_3d.rs:341:    let content_root = config.memory_tree_content_root();
<OPENHUMAN_ROOT>/src/bin/gmail_backfill_3d.rs:355:    let content_root = config.memory_tree_content_root();
<OPENHUMAN_ROOT>/src/bin/gmail_backfill_3d.rs:425:    let content_root = config.memory_tree_content_root();
<OPENHUMAN_ROOT>/src/bin/slack_backfill.rs:18://!   unconfigured — `memory/tree/ingest` soft-falls-back per call.
<OPENHUMAN_ROOT>/src/bin/slack_backfill.rs:24://! export OPENHUMAN_MEMORY_EMBED_ENDPOINT=http://localhost:11434
<OPENHUMAN_ROOT>/src/bin/slack_backfill.rs:25://! export OPENHUMAN_MEMORY_EMBED_MODEL=nomic-embed-text
<OPENHUMAN_ROOT>/src/bin/slack_backfill.rs:26://! export OPENHUMAN_MEMORY_EXTRACT_ENDPOINT=http://localhost:11434
<OPENHUMAN_ROOT>/src/bin/slack_backfill.rs:27://! export OPENHUMAN_MEMORY_EXTRACT_MODEL=qwen2.5:0.5b
<OPENHUMAN_ROOT>/src/bin/slack_backfill.rs:28://! export OPENHUMAN_MEMORY_SUMMARISE_ENDPOINT=http://localhost:11434
<OPENHUMAN_ROOT>/src/bin/slack_backfill.rs:29://! export OPENHUMAN_MEMORY_SUMMARISE_MODEL=llama3.1:8b
<OPENHUMAN_ROOT>/src/bin/slack_backfill.rs:30://! export RUST_LOG=info,openhuman_core::openhuman::composio::providers::slack=debug,openhuman_core::openhuman::memory=debug
<OPENHUMAN_ROOT>/src/bin/slack_backfill.rs:54:use openhuman_core::openhuman::memory;
<OPENHUMAN_ROOT>/src/bin/slack_backfill.rs:137:    /// touching the memory tree.
<OPENHUMAN_ROOT>/src/bin/slack_backfill.rs:151:    // memory-tree pipeline, the slack ingestion ops layer, …).
<OPENHUMAN_ROOT>/src/bin/slack_backfill.rs:174:    // `memory_tree.embedding_*`, `llm_extractor_*`, and
<OPENHUMAN_ROOT>/src/bin/slack_backfill.rs:187:    // Bootstrap the memory global so `SyncState` KV reads/writes work
<OPENHUMAN_ROOT>/src/bin/slack_backfill.rs:190:    memory::global::init(config.workspace_dir.clone())
<OPENHUMAN_ROOT>/src/bin/slack_backfill.rs:191:        .map_err(|e| anyhow::anyhow!("[slack_backfill] memory::global::init failed: {e}"))?;
<OPENHUMAN_ROOT>/src/bin/slack_backfill.rs:214:        use openhuman_core::openhuman::memory::ingest_pipeline::ingest_chat;
<OPENHUMAN_ROOT>/src/bin/slack_backfill.rs:215:        use openhuman_core::openhuman::memory_sync::canonicalize::chat::{ChatBatch, ChatMessage};
<OPENHUMAN_ROOT>/src/bin/slack_backfill.rs:302:        // memory tree or burning extra quota on retries.
<OPENHUMAN_ROOT>/src/bin/slack_backfill.rs:509:            &config.memory_tree.embedding_endpoint,
<OPENHUMAN_ROOT>/src/bin/slack_backfill.rs:510:            &config.memory_tree.embedding_model,
<OPENHUMAN_ROOT>/src/bin/slack_backfill.rs:513:            &config.memory_tree.llm_extractor_endpoint,
<OPENHUMAN_ROOT>/src/bin/slack_backfill.rs:514:            &config.memory_tree.llm_extractor_model,
<OPENHUMAN_ROOT>/src/bin/slack_backfill.rs:517:            &config.memory_tree.llm_summariser_endpoint,
<OPENHUMAN_ROOT>/src/bin/slack_backfill.rs:518:            &config.memory_tree.llm_summariser_model,
<OPENHUMAN_ROOT>/src/bin/slack_backfill.rs:529:            match memory::global::client_if_ready() {
<OPENHUMAN_ROOT>/src/bin/slack_backfill.rs:545:                    log::warn!("[slack_backfill] memory client not ready; skipping --reset-state")
<OPENHUMAN_ROOT>/src/bin/memory_tree_init_smoke.rs:1://! Manual stress smoke for the memory_tree schema-init race fix.
<OPENHUMAN_ROOT>/src/bin/memory_tree_init_smoke.rs:3://! Spins N concurrent threads racing into `memory::tree::store::with_connection`
<OPENHUMAN_ROOT>/src/bin/memory_tree_init_smoke.rs:15://!   cargo run --bin memory-tree-init-smoke -- 32
<OPENHUMAN_ROOT>/src/bin/memory_tree_init_smoke.rs:19://!   cargo run --bin memory-tree-init-smoke -- 32
<OPENHUMAN_ROOT>/src/bin/memory_tree_init_smoke.rs:33:use openhuman_core::openhuman::memory_store::chunks::store::with_connection;
<OPENHUMAN_ROOT>/src/bin/memory_tree_init_smoke.rs:63:    let db_path = workspace.join("memory_tree").join("chunks.db");
<OPENHUMAN_ROOT>/src/core/jsonrpc_tests.rs:89:/// `memory::tree::jobs::start` + `composio::start_periodic_sync` +
<OPENHUMAN_ROOT>/src/core/jsonrpc_tests.rs:224:async fn invoke_memory_init_accepts_empty_params() {
<OPENHUMAN_ROOT>/src/core/jsonrpc_tests.rs:228:    let result = invoke_method(default_state(), "openhuman.memory_init", json!({})).await;
<OPENHUMAN_ROOT>/src/core/jsonrpc_tests.rs:238:async fn invoke_memory_list_namespaces_rejects_unknown_param() {
<OPENHUMAN_ROOT>/src/core/jsonrpc_tests.rs:241:        "openhuman.memory_list_namespaces",
<OPENHUMAN_ROOT>/src/core/jsonrpc_tests.rs:250:async fn invoke_memory_query_namespace_missing_namespace_fails() {
<OPENHUMAN_ROOT>/src/core/jsonrpc_tests.rs:253:        "openhuman.memory_query_namespace",
<OPENHUMAN_ROOT>/src/core/jsonrpc_tests.rs:262:async fn invoke_memory_recall_memories_rejects_unknown_param() {
<OPENHUMAN_ROOT>/src/core/jsonrpc_tests.rs:265:        "openhuman.memory_recall_memories",
<OPENHUMAN_ROOT>/src/core/event_bus/bus.rs:59:/// (e.g., an agent turn completed, a memory was stored).
<OPENHUMAN_ROOT>/src/core/subconscious_cli.rs:113:        // Init memory client
<OPENHUMAN_ROOT>/src/core/subconscious_cli.rs:114:        let _ = crate::openhuman::memory::global::init(config.workspace_dir.clone());
<OPENHUMAN_ROOT>/src/core/subconscious_cli.rs:144:        // Create engine and run tick. The engine pulls its own memory_diff /
<OPENHUMAN_ROOT>/src/core/subconscious_cli.rs:145:        // context state from the workspace — no memory client to pass in.
<OPENHUMAN_ROOT>/src/core/subconscious_cli.rs:146:        let engine = crate::openhuman::subconscious::memory_instance(&config);
<OPENHUMAN_ROOT>/src/core/subconscious_cli.rs:165:                        conn, "memory",
<OPENHUMAN_ROOT>/src/core/subconscious_cli.rs:205:                crate::openhuman::subconscious::store::get_last_tick_at(conn, "memory")
<OPENHUMAN_ROOT>/src/core/event_bind_tokens.rs:19://! This module owns only the in-memory store; the RPC handler that mints
<OPENHUMAN_ROOT>/src/core/event_bus/mod.rs:5://! modules (like memory, skills, and agents) to communicate without
<OPENHUMAN_ROOT>/src/core/event_bus/README.md:13:- `pub enum DomainEvent` — `events.rs` — `#[non_exhaustive]` catalog of events; current variants cover Agent (`AgentTurnStarted/Completed`, `AgentError`), Memory (`MemoryStored`, `MemoryRecalled`), Channels (`ChannelInboundMessage`, `ChannelMessageReceived/Processed`, `ChannelReactionReceived/Sent`, `ChannelConnected/Disconnected`), Cron (`CronJobTriggered/Completed`, `CronDeliveryRequested`), Skills, Tools, Webhooks, and System.
<OPENHUMAN_ROOT>/src/core/event_bus/README.md:32:- `src/openhuman/memory/conversations/bus.rs` — conversation persistence subscriber.
<OPENHUMAN_ROOT>/CONTRIBUTING-BEGINNERS.md:33:| `src/`           | Rust               | The backend brain — logic, memory, RPC |
<OPENHUMAN_ROOT>/src/core/auth.rs:6://! 1. **In-memory handoff (preferred for the in-process core)** —
<OPENHUMAN_ROOT>/src/core/auth.rs:15://! 2. **Env-as-config fallback** — when no in-memory token is supplied,
<OPENHUMAN_ROOT>/src/core/auth.rs:20://!    handing it to the binary in-memory.
<OPENHUMAN_ROOT>/src/core/auth.rs:26://! Once set, the in-memory `OnceLock` is the single source of truth — all
<OPENHUMAN_ROOT>/src/core/auth.rs:145:/// per-launch bearer to the embedded server via an internal in-memory handle
<OPENHUMAN_ROOT>/src/core/auth.rs:154:/// the token over in-memory, so env-as-config is the appropriate transport.
<OPENHUMAN_ROOT>/src/core/auth.rs:157:/// I/O). When absent and no in-memory token was seeded, `init_rpc_token`
<OPENHUMAN_ROOT>/src/core/auth.rs:165:/// to the embedded server via the internal in-memory handle (see
<OPENHUMAN_ROOT>/src/core/auth.rs:188:    // validates the original in-memory value — that would cause clients
<OPENHUMAN_ROOT>/src/core/auth.rs:199:    // for an in-memory handoff instead.
<OPENHUMAN_ROOT>/src/core/auth.rs:223:/// **In-memory handoff path** — used by the Tauri shell to inject the bearer
<OPENHUMAN_ROOT>/src/core/auth.rs:232:/// the in-memory bearer mid-life would 401 every in-flight client).
<OPENHUMAN_ROOT>/src/core/auth.rs:249:    log::info!("[auth] core RPC token loaded via in-memory handoff (no env crossing)");
<OPENHUMAN_ROOT>/src/core/auth.rs:261:/// supplied token is non-empty and matches the in-memory expected value.
<OPENHUMAN_ROOT>/src/core/auth.rs:532:    /// that `get_rpc_token` reads — i.e. the in-memory handoff path produces
<OPENHUMAN_ROOT>/src/core/auth.rs:550:        // not error, must not flip the in-memory value.
<OPENHUMAN_ROOT>/src/core/auth.rs:557:            "second init_rpc_token_with_value must not flip the in-memory bearer"
<OPENHUMAN_ROOT>/src/core/cli.rs:75:            crate::openhuman::memory_tree::tree_runtime::cli::run_tree_summarizer_command(
<OPENHUMAN_ROOT>/src/core/cli.rs:79:        "memory" => crate::core::memory_cli::run_memory_command(&args[1..]),
<OPENHUMAN_ROOT>/src/core/cli.rs:563:        "  openhuman mcp [-v|--verbose]              (stdio MCP server; read-only memory tools)"
<OPENHUMAN_ROOT>/src/core/mod.rs:20:pub mod memory_cli;
<OPENHUMAN_ROOT>/src/core/mod.rs:36:    /// Domain/group identifier, e.g. `memory`, `config`, `credentials`.
<OPENHUMAN_ROOT>/src/core/mod.rs:53:    /// Canonical dotted name for routing, e.g. `memory.doc_put`.
<OPENHUMAN_ROOT>/src/core/mod.rs:134:        let s = mk("memory", "doc_put");
<OPENHUMAN_ROOT>/src/core/mod.rs:135:        assert_eq!(s.method_name(), "memory.doc_put");
<OPENHUMAN_ROOT>/src/core/mod.rs:142:        let s = mk("memory", "doc_put");
<OPENHUMAN_ROOT>/src/core/mod.rs:143:        assert_eq!(s.method_name(), "memory.doc_put");
<OPENHUMAN_ROOT>/src/core/mod.rs:146:            "openhuman.memory_doc_put"
<OPENHUMAN_ROOT>/src/core/dispatch.rs:17:///    which handles all registered controllers (memory, skills, etc.).
<OPENHUMAN_ROOT>/src/core/dispatch.rs:119:/// canonical handler exists (#3565: `openhuman.memory_tree_create_namespace`).
<OPENHUMAN_ROOT>/src/core/dispatch.rs:131:    "openhuman.memory_tree_create_namespace",
<OPENHUMAN_ROOT>/src/core/dispatch.rs:344:        assert!(try_core_dispatch(&state, "openhuman.memory_list_namespaces", json!({})).is_none());
<OPENHUMAN_ROOT>/src/core/dispatch.rs:387:            "openhuman.memory_tree_create_namespace",
<OPENHUMAN_ROOT>/src/core/dispatch.rs:400:        assert!(!is_known_probe_method("memory_tree_create_namespace"));
<OPENHUMAN_ROOT>/src/core/logging.rs:518:        std::env::set_var("OPENHUMAN_LOG_FILE_CONSTRAINTS", "rpc, , agent ,memory");
<OPENHUMAN_ROOT>/src/core/logging.rs:520:        assert_eq!(parsed, vec!["rpc", "agent", "memory"]);
<OPENHUMAN_ROOT>/src/core/memory_cli.rs:1://! `openhuman memory` — CLI for memory ingestion, graph inspection, and debugging.
<OPENHUMAN_ROOT>/src/core/memory_cli.rs:3://! Provides direct access to the memory system from the command line, including
<OPENHUMAN_ROOT>/src/core/memory_cli.rs:8://!   openhuman memory ingest  <file|->  [--namespace <ns>] [--key <key>] [--title <title>] [-v]
<OPENHUMAN_ROOT>/src/core/memory_cli.rs:9://!   openhuman memory docs    [--namespace <ns>]
<OPENHUMAN_ROOT>/src/core/memory_cli.rs:10://!   openhuman memory graph   [--namespace <ns>] [--subject <s>] [--predicate <p>]
<OPENHUMAN_ROOT>/src/core/memory_cli.rs:11://!   openhuman memory query   --namespace <ns> --query <text> [--limit <n>]
<OPENHUMAN_ROOT>/src/core/memory_cli.rs:12://!   openhuman memory namespaces
<OPENHUMAN_ROOT>/src/core/memory_cli.rs:18:use crate::openhuman::memory::ingestion::{MemoryIngestionConfig, MemoryIngestionRequest};
<OPENHUMAN_ROOT>/src/core/memory_cli.rs:19:use crate::openhuman::memory_store::NamespaceDocumentInput;
<OPENHUMAN_ROOT>/src/core/memory_cli.rs:21:/// Entry point for `openhuman memory <subcommand>`.
<OPENHUMAN_ROOT>/src/core/memory_cli.rs:22:pub fn run_memory_command(args: &[String]) -> Result<()> {
<OPENHUMAN_ROOT>/src/core/memory_cli.rs:24:        print_memory_help();
<OPENHUMAN_ROOT>/src/core/memory_cli.rs:36:            "unknown memory subcommand '{other}'. Run `openhuman memory --help`."
<OPENHUMAN_ROOT>/src/core/memory_cli.rs:45:/// `openhuman memory ingest <file|-> [options]`
<OPENHUMAN_ROOT>/src/core/memory_cli.rs:74:                println!("Usage: openhuman memory ingest <file|-> [options]");
<OPENHUMAN_ROOT>/src/core/memory_cli.rs:111:        "[memory:cli] ingesting document: namespace={namespace}, key={doc_key}, title={doc_title}, \
<OPENHUMAN_ROOT>/src/core/memory_cli.rs:121:        let client = create_memory_client().await?;
<OPENHUMAN_ROOT>/src/core/memory_cli.rs:135:            taint: crate::openhuman::memory::MemoryTaint::Internal,
<OPENHUMAN_ROOT>/src/core/memory_cli.rs:138:        let ingestion_config = MemoryIngestionConfig::default();
<OPENHUMAN_ROOT>/src/core/memory_cli.rs:141:            "[memory:cli] starting ingestion with model={}",
<OPENHUMAN_ROOT>/src/core/memory_cli.rs:146:            .ingest_doc(MemoryIngestionRequest {
<OPENHUMAN_ROOT>/src/core/memory_cli.rs:175:/// `openhuman memory docs [--namespace <ns>]`
<OPENHUMAN_ROOT>/src/core/memory_cli.rs:191:                println!("Usage: openhuman memory docs [--namespace <ns>] [-v]");
<OPENHUMAN_ROOT>/src/core/memory_cli.rs:205:        let client = create_memory_client().await?;
<OPENHUMAN_ROOT>/src/core/memory_cli.rs:216:/// `openhuman memory graph [--namespace <ns>] [--subject <s>] [--predicate <p>]`
<OPENHUMAN_ROOT>/src/core/memory_cli.rs:241:                    "Usage: openhuman memory graph [--namespace <ns>] [--subject <s>] [--predicate <p>] [-v]"
<OPENHUMAN_ROOT>/src/core/memory_cli.rs:256:        let client = create_memory_client().await?;
<OPENHUMAN_ROOT>/src/core/memory_cli.rs:271:/// `openhuman memory query --namespace <ns> --query <text> [--limit <n>]`
<OPENHUMAN_ROOT>/src/core/memory_cli.rs:299:                    "Usage: openhuman memory query --namespace <ns> --query <text> [--limit <n>] [-v]"
<OPENHUMAN_ROOT>/src/core/memory_cli.rs:318:        let client = create_memory_client().await?;
<OPENHUMAN_ROOT>/src/core/memory_cli.rs:329:/// `openhuman memory namespaces`
<OPENHUMAN_ROOT>/src/core/memory_cli.rs:336:                println!("Usage: openhuman memory namespaces [-v]");
<OPENHUMAN_ROOT>/src/core/memory_cli.rs:350:        let client = create_memory_client().await?;
<OPENHUMAN_ROOT>/src/core/memory_cli.rs:360:/// `openhuman memory clear --namespace <ns>`
<OPENHUMAN_ROOT>/src/core/memory_cli.rs:376:                println!("Usage: openhuman memory clear --namespace <ns> [-v]");
<OPENHUMAN_ROOT>/src/core/memory_cli.rs:393:        let client = create_memory_client().await?;
<OPENHUMAN_ROOT>/src/core/memory_cli.rs:398:        eprintln!("[memory:cli] namespace '{namespace}' cleared");
<OPENHUMAN_ROOT>/src/core/memory_cli.rs:436:async fn create_memory_client() -> Result<crate::openhuman::memory_store::MemoryClientRef> {
<OPENHUMAN_ROOT>/src/core/memory_cli.rs:440:    crate::openhuman::memory::global::init(config.workspace_dir).map_err(anyhow::Error::msg)
<OPENHUMAN_ROOT>/src/core/memory_cli.rs:443:fn print_memory_help() {
<OPENHUMAN_ROOT>/src/core/memory_cli.rs:444:    println!("Usage: openhuman memory <subcommand> [options]");
<OPENHUMAN_ROOT>/src/core/memory_cli.rs:455:    println!("  openhuman memory ingest notes.md -n my-project -v");
<OPENHUMAN_ROOT>/src/core/memory_cli.rs:456:    println!("  echo 'Alice works on ProjectX' | openhuman memory ingest - -n test -v");
<OPENHUMAN_ROOT>/src/core/memory_cli.rs:457:    println!("  openhuman memory graph -n my-project");
<OPENHUMAN_ROOT>/src/core/memory_cli.rs:458:    println!("  openhuman memory docs -n my-project");
<OPENHUMAN_ROOT>/src/core/memory_cli.rs:459:    println!("  openhuman memory query -n my-project -q 'who works on what?'");
<OPENHUMAN_ROOT>/src/core/event_bus/events_tests.rs:83:        // Memory
<OPENHUMAN_ROOT>/src/core/event_bus/events_tests.rs:85:            DomainEvent::MemoryStored {
<OPENHUMAN_ROOT>/src/core/event_bus/events_tests.rs:90:            "memory",
<OPENHUMAN_ROOT>/src/core/event_bus/events_tests.rs:93:            DomainEvent::MemoryRecalled {
<OPENHUMAN_ROOT>/src/core/event_bus/events_tests.rs:97:            "memory",
<OPENHUMAN_ROOT>/src/core/event_bus/events_tests.rs:468:        // Memory tree
<OPENHUMAN_ROOT>/src/core/event_bus/events_tests.rs:478:            "memory",
<OPENHUMAN_ROOT>/README.md:52:> OpenHuman is not AGI. But it is a meaningful architectural step closer, with better memory, better orchestration, and better tooling.
<OPENHUMAN_ROOT>/README.md:64:OpenHuman is three things most assistants aren't: **a brain** that builds a persistent, local memory of your world; **a fantastic orchestrator** that runs fleets of agents on durable graphs; and **a deep researcher** that sweeps your data and the web before you finish asking. Every bullet links to the deeper writeup in the [docs](https://tinyhumans.gitbook.io/openhuman/).
<OPENHUMAN_ROOT>/README.md:68:- **[Memory Tree](https://tinyhumans.gitbook.io/openhuman/features/memory-tree) + [Obsidian Wiki](https://tinyhumans.gitbook.io/openhuman/features/obsidian-wiki)**: your data compressed into scored Markdown trees in SQLite on your machine, mirrored as an [Obsidian vault](https://x.com/karpathy/status/2039805659525644595) you can open and edit. No vector-soup black box.
<OPENHUMAN_ROOT>/README.md:83:- **[SuperContext](https://tinyhumans.gitbook.io/openhuman/features/super-context)**: a research scout sweeps your memory and files before the model reads your first message. No cold starts.
<OPENHUMAN_ROOT>/README.md:100: <img src="./gitbooks/.gitbook/assets/memory.png" alt="OpenHuman context-building diagram">
<OPENHUMAN_ROOT>/README.md:103:> OpenHuman summarizes and compresses all your documents, emails & chats; and creates a memory graph that lets your agent remember everything about you.
<OPENHUMAN_ROOT>/README.md:105:OpenHuman skips the wait. Connect your accounts, let [auto-fetch](https://tinyhumans.gitbook.io/openhuman/features/integrations/auto-fetch) pull data locally on a 20-minute loop, and then have [Memory Trees](https://tinyhumans.gitbook.io/openhuman/features/memory-tree) compress everything into Markdown files stored intelligently in a [Karpathy-style Obsidian wiki](https://tinyhumans.gitbook.io/openhuman/features/obsidian-wiki).
<OPENHUMAN_ROOT>/README.md:109:Already self-host [agentmemory](https://github.com/rohitg00/agentmemory) across other coding agents? OpenHuman ships an optional `Memory` backend that proxies to it. Set `memory.backend = "agentmemory"` in `config.toml` and the same durable store powers OpenHuman alongside Claude Code, Cursor, Codex, and OpenCode. See the [agentmemory backend](https://tinyhumans.gitbook.io/openhuman/features/obsidian-wiki/agentmemory-backend) page for setup.
<OPENHUMAN_ROOT>/README.md:139:High-level comparison (products evolve, so verify against each vendor). OpenHuman is built to **minimize vendor sprawl**, keep **workflow knowledge on-device**, and give the agent a **persistent memory** of your data, not only chat.
<OPENHUMAN_ROOT>/README.md:146:| **Memory**             | ✅ Chat-scoped    | ⚠️ Plugin-reliant | ✅ Self-learning  | 🚀 Memory Tree + Obsidian vault, optional [agentmemory](https://github.com/rohitg00/agentmemory) backend |
<OPENHUMAN_ROOT>/README.md:148:| **Auto-fetch**         | 🚫 None           | 🚫 None           | 🚫 None           | ✅ 20-min sync into memory                                                                               |
<OPENHUMAN_ROOT>/src/core/jsonrpc.rs:1165:        // Spawn sweep loop — bounds memory under sustained traffic.
<OPENHUMAN_ROOT>/src/core/jsonrpc.rs:1520:/// set to the domain name (agent, tool, memory, etc.).
<OPENHUMAN_ROOT>/src/core/jsonrpc.rs:1711:/// When the caller already holds the per-launch RPC bearer in memory (the
<OPENHUMAN_ROOT>/src/core/jsonrpc.rs:1788:    // Initialize the global MemoryClient so composio providers
<OPENHUMAN_ROOT>/src/core/jsonrpc.rs:1790:    // and so any subsystem that calls `memory::global::client_if_ready()`
<OPENHUMAN_ROOT>/src/core/jsonrpc.rs:1792:    // "[composio:gmail] memory client not ready".
<OPENHUMAN_ROOT>/src/core/jsonrpc.rs:1798:        // `Config::default()` and initialised the memory + whatsapp_data
<OPENHUMAN_ROOT>/src/core/jsonrpc.rs:1802:        // init entirely so memory stays explicitly *uninitialised* —
<OPENHUMAN_ROOT>/src/core/jsonrpc.rs:1803:        // callers then get a clear "memory client not ready" error rather
<OPENHUMAN_ROOT>/src/core/jsonrpc.rs:1818:                match crate::openhuman::memory::global::init(cfg.workspace_dir.clone()) {
<OPENHUMAN_ROOT>/src/core/jsonrpc.rs:1820:                        "[boot] memory::global initialized (workspace={})",
<OPENHUMAN_ROOT>/src/core/jsonrpc.rs:1823:                    Err(e) => log::warn!("[boot] memory::global init failed: {e}"),
<OPENHUMAN_ROOT>/src/core/jsonrpc.rs:1827:                // of an in-memory FIFO (survives restarts + delegation hops).
<OPENHUMAN_ROOT>/src/core/jsonrpc.rs:1884:                    "[boot] memory::global + whatsapp_data init SKIPPED — \
<OPENHUMAN_ROOT>/src/core/jsonrpc.rs:1885:                     Config::load_or_init failed ({e:#}). Memory persistence is \
<OPENHUMAN_ROOT>/src/core/jsonrpc.rs:1932:    //   - An in-memory bearer supplied by the embedded caller via the
<OPENHUMAN_ROOT>/src/core/jsonrpc.rs:1940:    // an embedded caller binds on a non-loopback host with an in-memory
<OPENHUMAN_ROOT>/src/core/jsonrpc.rs:1943:        let has_in_memory_token = rpc_token
<OPENHUMAN_ROOT>/src/core/jsonrpc.rs:1955:        //   - in-memory handoff from the embedded caller (`rpc_token`), or
<OPENHUMAN_ROOT>/src/core/jsonrpc.rs:1962:        let has_explicit_token = has_in_memory_token || has_env_token;
<OPENHUMAN_ROOT>/src/core/jsonrpc.rs:1967:                 bearer in-memory via the embedded core handle) to secure the RPC endpoint.",
<OPENHUMAN_ROOT>/src/core/jsonrpc.rs:2275:        crate::openhuman::memory_conversations::register_conversation_persistence_subscriber(
<OPENHUMAN_ROOT>/src/core/jsonrpc.rs:2278:        crate::openhuman::memory::sync::register_sync_stage_bridge(&config);
<OPENHUMAN_ROOT>/src/core/jsonrpc.rs:2288:        // Workspace-kind memory sources (GitHub repos, folders, RSS, web
<OPENHUMAN_ROOT>/src/core/jsonrpc.rs:2292:        crate::openhuman::memory_sync::workspace::start_workspace_periodic_sync();
<OPENHUMAN_ROOT>/src/core/jsonrpc.rs:2303:        // Seed memory_sources with active Composio connections so the
<OPENHUMAN_ROOT>/src/core/jsonrpc.rs:2304:        // user sees their connected integrations as memory sources by
<OPENHUMAN_ROOT>/src/core/jsonrpc.rs:2307:            crate::openhuman::memory_sources::reconcile::ensure_composio_sources().await;
<OPENHUMAN_ROOT>/src/core/jsonrpc.rs:2362:        crate::openhuman::memory_queue::start(config.clone());
<OPENHUMAN_ROOT>/src/core/jsonrpc.rs:2660:        // JSON-RPC bearer (`OPENHUMAN_CORE_TOKEN` / the in-memory
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
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:259:    MemoryStored {
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:264:    /// A memory recall query completed.
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:265:    MemoryRecalled { query: String, hit_count: usize },
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:266:    /// A memory sync was requested for a specific channel or all channels.
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:268:    /// Published by `openhuman.memory_sync_channel` (channel_id = Some(...)) and
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:269:    /// `openhuman.memory_sync_all` (channel_id = None). No consumers exist yet —
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:271:    /// requests. See `src/openhuman/memory/ops.rs` for the RPC handlers.
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:272:    MemorySyncRequested { channel_id: Option<String> },
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:273:    /// A high-level memory sync orchestration stage changed.
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:275:    /// Emitted by the `memory` domain so the frontend can surface progress
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:278:    /// `source_id` is the originating memory-source id (from
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:279:    /// `memory_sources`) when the event can be attributed to a specific
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:282:    /// when the event originates from a non-memory-source sync path (e.g. a
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:285:    MemorySyncStageChanged {
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:291:        /// Originating memory-source id for frontend per-row indicator
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:293:        /// specific `MemorySourceEntry`.
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:296:    /// A memory ingestion job started running on the local extraction LLM.
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:298:    /// [`Self::MemoryIngestionCompleted`] follows when the job finishes.
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:299:    MemoryIngestionStarted {
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:305:    /// A memory ingestion job finished (successfully or with an error).
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:306:    MemoryIngestionCompleted {
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:314:    // ── Memory Diff ─────────────────────────────────────────────────────
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:315:    /// A snapshot of a memory source's chunk state was captured.
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:316:    MemoryDiffSnapshotTaken {
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:324:    MemoryDiffComputed {
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:334:    MemoryDiffMarkedRead {
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:837:    /// Fine-grained progress during the memory tree build pipeline.
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:839:    MemoryTreeBuildProgress {
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:891:    // ── Memory tree ─────────────────────────────────────────────────────
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:893:    /// fully canonicalised and its chunks written to the memory tree.
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:895:    /// Emitted by `memory::tree::ingest::persist()` after the chunk upsert
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:1307:            | Self::MemoryStored { .. }
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:1308:            | Self::MemoryRecalled { .. }
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:1309:            | Self::MemorySyncRequested { .. }
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:1310:            | Self::MemorySyncStageChanged { .. }
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:1311:            | Self::MemoryIngestionStarted { .. }
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:1312:            | Self::MemoryIngestionCompleted { .. }
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:1314:            | Self::MemoryDiffSnapshotTaken { .. }
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:1315:            | Self::MemoryDiffComputed { .. }
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:1316:            | Self::MemoryDiffMarkedRead { .. } => "memory",
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:1367:            | Self::MemoryTreeBuildProgress { .. } => "tree_summarizer",
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:1471:            Self::MemoryStored { .. } => "MemoryStored",
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:1472:            Self::MemoryRecalled { .. } => "MemoryRecalled",
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:1473:            Self::MemorySyncRequested { .. } => "MemorySyncRequested",
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:1474:            Self::MemorySyncStageChanged { .. } => "MemorySyncStageChanged",
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:1475:            Self::MemoryIngestionStarted { .. } => "MemoryIngestionStarted",
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:1476:            Self::MemoryIngestionCompleted { .. } => "MemoryIngestionCompleted",
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:1478:            Self::MemoryDiffSnapshotTaken { .. } => "MemoryDiffSnapshotTaken",
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:1479:            Self::MemoryDiffComputed { .. } => "MemoryDiffComputed",
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:1480:            Self::MemoryDiffMarkedRead { .. } => "MemoryDiffMarkedRead",
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:1522:            Self::MemoryTreeBuildProgress { .. } => "MemoryTreeBuildProgress",
<OPENHUMAN_ROOT>/src/core/socketio.rs:311:    /// uses it to reopen the full parent↔subagent conversation from memory.
<OPENHUMAN_ROOT>/src/core/socketio.rs:629:    let io_memory_sync = io.clone();
<OPENHUMAN_ROOT>/src/core/socketio.rs:927:    // 8. Memory sync stage + tree-build progress → broadcast to all clients
<OPENHUMAN_ROOT>/src/core/socketio.rs:942:                        "[socketio] event_bus not initialised after {}s — memory_sync bridge giving up",
<OPENHUMAN_ROOT>/src/core/socketio.rs:956:                        "[socketio] dropped {} event_bus events due to lag (memory_sync bridge)",
<OPENHUMAN_ROOT>/src/core/socketio.rs:964:                crate::core::event_bus::DomainEvent::MemorySyncStageChanged {
<OPENHUMAN_ROOT>/src/core/socketio.rs:978:                        // source_id is the memory-source row id for frontend per-row
<OPENHUMAN_ROOT>/src/core/socketio.rs:983:                    let _ = io_memory_sync.emit("memory:sync_stage", &payload);
<OPENHUMAN_ROOT>/src/core/socketio.rs:997:                    let _ = io_memory_sync.emit("memory:tree_progress", &payload);
<OPENHUMAN_ROOT>/src/core/socketio.rs:1007:                    let _ = io_memory_sync.emit("memory:tree_completed", &payload);
<OPENHUMAN_ROOT>/src/core/socketio.rs:1009:                crate::core::event_bus::DomainEvent::MemoryTreeBuildProgress {
<OPENHUMAN_ROOT>/src/core/socketio.rs:1025:                    let _ = io_memory_sync.emit("memory:build_progress", &payload);
<OPENHUMAN_ROOT>/src/core/socketio.rs:1039:                    let _ = io_memory_sync.emit("init:progress", &payload);
<OPENHUMAN_ROOT>/src/core/socketio.rs:1049:                    let _ = io_memory_sync.emit("init:completed", &payload);
<OPENHUMAN_ROOT>/src/core/socketio.rs:1072:                    let _ = io_memory_sync.emit("flow:run_progress", &payload);
<OPENHUMAN_ROOT>/src/core/socketio.rs:1073:                    let _ = io_memory_sync.emit("flow_run_progress", &payload);
<OPENHUMAN_ROOT>/src/core/socketio.rs:1078:        log::debug!("[socketio] memory_sync bridge stopped");
<OPENHUMAN_ROOT>/plan.md:32:  frontend E2E (WDIO + Playwright), Rust unit (agent/memory; channels/providers/platform;
<OPENHUMAN_ROOT>/plan.md:51:| ✅ | `app/src/services/api/{graphCentralityApi,graphCohesionApi,memoryFreshnessApi,connectionPathApi,memoryTimelineApi,entityAssociationsApi,namespaceOverviewApi}.test.ts` | the copy-pasted `exposes the public surface` test in each (7 files) | typeof-only assertions on aggregate objects **no consumer imports** (tabs import the named exports directly); the functions are behaviorally tested above in each file. Do one grep-and-delete pass. |
<OPENHUMAN_ROOT>/plan.md:78:| `src/openhuman/memory/schema_tests.rs` | registry-sync + unknown-fn tests | **False duplicate.** `memory/schema/` (singular, `memory_tree` namespace) and `memory/schemas/` (plural, `memory` namespace) are two distinct live registries with disjoint function sets. These are the *only* parity/unknown-fn guards for the `memory_tree` controller surface. |
<OPENHUMAN_ROOT>/plan.md:120:3. **`memory/read_rpc/admin.rs::delete_source_rpc` — ZERO tests** on a 427-line destructive
<OPENHUMAN_ROOT>/plan.md:135:   token; `tauri-commands.spec.ts` is happy-path only. Also add the in-memory token-handoff
<OPENHUMAN_ROOT>/plan.md:148:- **Memory `path_scope` invariant**: two source_ids sharing a path_scope must summarize into one
<OPENHUMAN_ROOT>/plan.md:162:  tool result renders → new thread → memory recall of the earlier interaction. Individual pieces
<OPENHUMAN_ROOT>/plan.md:182:- Port native-free WDIO-only specs (memory-sync-schedule, skill-activation-persistence,
<OPENHUMAN_ROOT>/plan.md:309:   park/decide/TTL, encryption round-trip, memory ingest→recall→delete_source, wallet/web3
<OPENHUMAN_ROOT>/plan.md:340:| Performance/load | No benchmarks anywhere (no criterion/k6) | Start with criterion micro-benches on memory ingest + embeddings; defer load tests. |
<OPENHUMAN_ROOT>/plan.md:382:- [~] **`delete_source_rpc` cascade** — ALREADY COVERED. `memory_store/chunks/store_tests.rs` has
<OPENHUMAN_ROOT>/plan.md:432:  in the authoring env): memory two-source-one-tree `path_scope` invariant; broad hostile-webhook
<OPENHUMAN_ROOT>/plan.md:464:`AuthProfilesStore`, `threads::ops`, `memory_sources::sync`) across 5–8 files under different
<OPENHUMAN_ROOT>/plan.md:474:  (`app_credentials_threads_round24_…`, `…sources_round26_…`, `…memory_sources_raw_coverage…`,
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
<OPENHUMAN_ROOT>/src/core/all.rs:235:    // Memory tree ingestion layer (#707 — canonicalised chunks with provenance)
<OPENHUMAN_ROOT>/src/core/all.rs:236:    controllers.extend(crate::openhuman::memory_tree::all_memory_tree_registered_controllers());
<OPENHUMAN_ROOT>/src/core/all.rs:237:    // Memory tree retrieval layer (#710 — LLM-callable read tools over the tree)
<OPENHUMAN_ROOT>/src/core/all.rs:238:    controllers.extend(crate::openhuman::memory_tree::all_retrieval_registered_controllers());
<OPENHUMAN_ROOT>/src/core/all.rs:239:    // Slack → memory-tree ingestion engine (per-message ingest, no bucketing)
<OPENHUMAN_ROOT>/src/core/all.rs:241:        crate::openhuman::composio::providers::slack::all_slack_memory_registered_controllers(),
<OPENHUMAN_ROOT>/src/core/all.rs:243:    // Per-connection memory sync status, controls, and progress (#1136)
<OPENHUMAN_ROOT>/src/core/all.rs:245:        crate::openhuman::memory_sync::sync_status::all_memory_sync_status_registered_controllers(),
<OPENHUMAN_ROOT>/src/core/all.rs:247:    // Memory sources — user-configured data connectors registry
<OPENHUMAN_ROOT>/src/core/all.rs:249:        .extend(crate::openhuman::memory_sources::all_memory_sources_registered_controllers());
<OPENHUMAN_ROOT>/src/core/all.rs:250:    // Memory diff — snapshot-based change tracking for memory sources
<OPENHUMAN_ROOT>/src/core/all.rs:251:    controllers.extend(crate::openhuman::memory_diff::all_memory_diff_registered_controllers());
<OPENHUMAN_ROOT>/src/core/all.rs:291:    controllers.extend(crate::openhuman::memory_tree::all_tree_summarizer_registered_controllers());
<OPENHUMAN_ROOT>/src/core/all.rs:436:    schemas.extend(crate::openhuman::memory::all_memory_controller_schemas());
<OPENHUMAN_ROOT>/src/core/all.rs:437:    schemas.extend(crate::openhuman::memory_goals::all_memory_goals_controller_schemas());
<OPENHUMAN_ROOT>/src/core/all.rs:439:    schemas.extend(crate::openhuman::memory_tree::all_memory_tree_controller_schemas());
<OPENHUMAN_ROOT>/src/core/all.rs:440:    schemas.extend(crate::openhuman::memory_tree::all_retrieval_controller_schemas());
<OPENHUMAN_ROOT>/src/core/all.rs:442:        crate::openhuman::composio::providers::slack::all_slack_memory_controller_schemas(),
<OPENHUMAN_ROOT>/src/core/all.rs:445:        crate::openhuman::memory_sync::sync_status::all_memory_sync_status_controller_schemas(),
<OPENHUMAN_ROOT>/src/core/all.rs:447:    schemas.extend(crate::openhuman::memory_sources::all_memory_sources_controller_schemas());
<OPENHUMAN_ROOT>/src/core/all.rs:448:    schemas.extend(crate::openhuman::memory_diff::all_memory_diff_controller_schemas());
<OPENHUMAN_ROOT>/src/core/all.rs:467:    schemas.extend(crate::openhuman::memory_tree::all_tree_summarizer_controller_schemas());
<OPENHUMAN_ROOT>/src/core/all.rs:576:        "memory" => Some("Document storage, vector search, key-value store, and knowledge graph."),
<OPENHUMAN_ROOT>/src/core/all.rs:577:        "memory_goals" => Some(
<OPENHUMAN_ROOT>/src/core/all.rs:583:        "memory_tree" => Some(
<OPENHUMAN_ROOT>/src/core/all.rs:584:            "Canonical chunk ingestion, provenance capture, and chunk retrieval for source-grounded memory.",
<OPENHUMAN_ROOT>/src/core/all.rs:586:        "memory_sync" => Some(
<OPENHUMAN_ROOT>/src/core/all.rs:587:            "Per-connection memory sync status, user enable toggle, and live progress for the desktop UI.",
<OPENHUMAN_ROOT>/src/core/all.rs:589:        "memory_sources" => Some(
<OPENHUMAN_ROOT>/src/core/all.rs:590:            "User-configured data connectors (Composio, folders, GitHub repos, RSS, web pages) that feed memory.",
<OPENHUMAN_ROOT>/src/core/all.rs:592:        "memory_diff" => Some(
<OPENHUMAN_ROOT>/src/core/all.rs:593:            "Snapshot-based change tracking for memory sources — capture state, compute diffs, and surface changes to agents.",
<OPENHUMAN_ROOT>/src/openhuman/provider_surfaces/schemas.rs:11:use crate::openhuman::memory::EmptyRequest;
<OPENHUMAN_ROOT>/pnpm-lock.yaml:3806:    deprecated: This module is not supported, and leaks memory. Do not use it. Check out lru-cache if you want a good and tested way to coalesce async requests by a key value, which is much more comprehensive and powerful.
<OPENHUMAN_ROOT>/Cargo.toml:21:name = "memory-tree-init-smoke"
<OPENHUMAN_ROOT>/Cargo.toml:22:path = "src/bin/memory_tree_init_smoke.rs"
<OPENHUMAN_ROOT>/Cargo.toml:69:# TinyCortex — Rust core for the memory engine (store/chunks/tree/retrieval/
<OPENHUMAN_ROOT>/Cargo.toml:71:# below to `vendor/tinycortex`. OpenHuman's memory subsystem migrates onto this
<OPENHUMAN_ROOT>/Cargo.toml:114:# Line-level text diffs for the memory_diff module (modified-item unified diffs).
<OPENHUMAN_ROOT>/Cargo.toml:116:# Git-backed change ledger for the memory_diff module: snapshots are commits,
<OPENHUMAN_ROOT>/Cargo.toml:141:# Wipe master keys / decrypted secret buffers from memory on drop (audit C9).
<OPENHUMAN_ROOT>/Cargo.toml:189:# index (`memory_conversations::tokenize`). NFKC unifies CJK half/full-
<OPENHUMAN_ROOT>/Cargo.toml:253:# baseline compiles this provider against wacore's in-memory Backend until a
<OPENHUMAN_ROOT>/Cargo.toml:279:    "Win32_System_Memory",
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
<OPENHUMAN_ROOT>/tests/channels_web_startup_raw_coverage_e2e.rs:252:        memory_entries: vec![TestMemoryEntry {
<OPENHUMAN_ROOT>/docs/agent-workflows/pilot-batch-example.json:10:      "title": "tighten memory namespace migration logging",
<OPENHUMAN_ROOT>/docs/agent-workflows/pilot-batch-example.json:11:      "branch": "cursor/a01-9001-memory-namespace-logging",
<OPENHUMAN_ROOT>/docs/agent-workflows/pilot-batch-example.json:12:      "owned_paths": ["src/openhuman/memory/"],
<OPENHUMAN_ROOT>/docs/tinycortex-drift-ledger.md:5:touched an engine-mapping memory module after the port line, and classifies each as:
<OPENHUMAN_ROOT>/docs/tinycortex-drift-ledger.md:36:- **≥ 2026-06-25 is captured.** Host `feat(memory_diff): back change ledger with git instead of
<OPENHUMAN_ROOT>/docs/tinycortex-drift-ledger.md:39:  (`vendor/tinycortex/src/memory/diff/diff.rs:98`, `ledger.rs`) — i.e. the post-06-25 shape. So the
<OPENHUMAN_ROOT>/docs/tinycortex-drift-ledger.md:40:  port base includes the 06-25 memory_diff work.
<OPENHUMAN_ROOT>/docs/tinycortex-drift-ledger.md:41:- **< 2026-06-28 for engine features.** Host `feat(memory): track summary-only wiki git history`
<OPENHUMAN_ROOT>/docs/tinycortex-drift-ledger.md:42:  (`6395f642e`, 06-28) added `memory_store/content/wiki_git/`. The crate has **no** `wiki_git` file
<OPENHUMAN_ROOT>/docs/tinycortex-drift-ledger.md:53:Scan: `git log --since=2026-06-20 -- src/openhuman/memory_store memory_tree memory_queue memory_diff
<OPENHUMAN_ROOT>/docs/tinycortex-drift-ledger.md:54:memory_goals memory_entities memory_graph memory_archivist memory_conversations memory_sources`,
<OPENHUMAN_ROOT>/docs/tinycortex-drift-ledger.md:62:| D1 | `007a99b62` (06-30) `perf(memory_conversations): rank before cloning hits in cross-thread search` | `memory_conversations/inverted_index.rs` | Rank matches on cheap borrowed keys (`(doc_id:u32, matched:usize, created_at:&str)`), truncate to `limit`, **then** materialize the KB-sized `CrossThreadHit`. Order-equivalent to score ranking. | **ABSENT.** `vendor/tinycortex/src/memory/conversations/inverted_index.rs:286–301` builds the full `CrossThreadHit` (with `content.clone()`, `message_id.clone()`, `created_at.clone()`) for **every** matched doc, then `sort_by(score)` + `truncate`. Pre-fix clone-then-rank shape. | `conversations::inverted_index` — port the rank-before-materialize refactor + its `ranks_by_score_then_recency_before_truncating` test. |
<OPENHUMAN_ROOT>/docs/tinycortex-drift-ledger.md:63:| D2 | `d7bee77e3` (06-30) `fix(memory-queue): classify host-FS I/O errors to stop the tree_jobs Sentry flood` | `memory_queue/worker.rs` | Adds `is_host_io_error(&anyhow::Error) -> bool` classifying **persistent** host-FS failures (EIO/ENOSPC/EROFS) distinct from transient SQLite busy/I-O, so the worker backs off and reports Sentry **once** instead of ~10k events/50min (Sentry CORE-RUST-19J). | **PARTIAL.** `vendor/tinycortex/src/memory/queue/worker.rs:89–107` has `is_sqlite_io_transient` (transient family) but **no** `is_host_io_error` (persistent host-FS family). | `queue::worker` — port the `is_host_io_error` predicate + its unit tests (EIO/ENOSPC/EROFS, context-layer, text fallback). **Only the predicate.** The Sentry-once emission and the `mark_storage_degraded` flag are host-owned (see D2-host below). |
<OPENHUMAN_ROOT>/docs/tinycortex-drift-ledger.md:64:| D3 | `c43f79641` (07-03) (within TinyAgents migration) | `memory_store/vectors/store.rs` | `count()` reads `COUNT(*)` as `i64` and converts via `usize::try_from(...).context(...)` instead of `row.get::<usize>` directly — robustness against platform `usize`/`i64` mismatch. | **ABSENT.** `vendor/tinycortex/src/memory/store/vectors/store.rs:370–380` still does `let count: usize = ... row.get(0)` then `Ok(count)`. | `store::vectors::store` — small; port the `i64` + `try_from` guard. |
<OPENHUMAN_ROOT>/docs/tinycortex-drift-ledger.md:77:| `0304d145f` (07-03) | `memory/tools/store.rs`, `memory/tools/forget.rs` | Agent tools | Tool contract/prompt text; agent tools stay host (plan §1). |
<OPENHUMAN_ROOT>/docs/tinycortex-drift-ledger.md:78:| `7bf18562a` (06-30) | `memory/read_rpc/{types,vault}.rs` | RPC read surface | `read_rpc` stays host; JSON-RPC surface. |
<OPENHUMAN_ROOT>/docs/tinycortex-drift-ledger.md:79:| `f84eec533` (06-30) | `memory_conversations/bus.rs` | Event bus | `bus.rs` = `EventHandler` impls, host-owned by canonical module shape. |
<OPENHUMAN_ROOT>/docs/tinycortex-drift-ledger.md:80:| `6edaa77b1` (06-29) | `memory_tree/score/embed/openai_compat.rs` | Embedding **compute** | Network-calling embedding backend; the crate abstracts compute behind `EmbeddingBackend` and "never makes a network call". Wires into the W1 `embeddings.rs` seam. |
<OPENHUMAN_ROOT>/docs/tinycortex-drift-ledger.md:81:| `d7bee77e3` (06-30) [D2-host] | `memory_tree/health/{mod,doctor}.rs` (`mark_storage_degraded`/`clear_storage_degraded`), `memory_tree/tree/rpc.rs` | Health signal + RPC | Degraded-state flag + Sentry wiring + doctor RPC. Crate defers tree health entirely (see gap audit); this is the host-side consumer of D2's predicate. |
<OPENHUMAN_ROOT>/docs/tinycortex-drift-ledger.md:82:| `c43f79641` (07-03) | `memory_search/{vector,tools}/*`, `memory_sync/composio/*` | Agent tools / live sync | Import-path churn from the TinyAgents cutover + live-sync; not engine semantics. |
<OPENHUMAN_ROOT>/docs/tinycortex-drift-ledger.md:88:| `6395f642e` (06-28) `feat(memory): track summary-only wiki git history` | `memory_store/content/wiki_git/` (mod + tests, ~690 LOC), plus a seal-time hook in `memory_tree/ingest.rs` + `memory_tree/tree/bucket_seal.rs` | `vendor/tinycortex/src/memory/store/content/mod.rs:19–20`: *"The Obsidian-vault registry (`content::obsidian*`) and the git-backed wiki mirror (`content::wiki_git`) pull host config and git surfaces beyond this."* The crate explicitly leaves `wiki_git` **and** `obsidian*`/`obsidian_registry` host-side (host `memory_store/content/mod.rs:17,18,23`). |
<OPENHUMAN_ROOT>/docs/tinycortex-drift-ledger.md:92:host surfaces it does not own. So `wiki_git`, `obsidian`, `obsidian_registry` join `memory_sync` as
<OPENHUMAN_ROOT>/docs/tinycortex-drift-ledger.md:95:(`vendor/tinycortex/src/memory/tree/bucket_seal.rs` exposes no post-seal sink). That is tracked as an
<OPENHUMAN_ROOT>/docs/tinycortex-drift-ledger.md:105:| `memory_store` (chunks, content, vectors, kv, entity_index, safety) | `store/`, `chunks/` | **D3** (vectors count guard). `wiki_git`/`obsidian*` host-retained (not drift). | W3 | **OPEN** (D3) |
<OPENHUMAN_ROOT>/docs/tinycortex-drift-ledger.md:106:| `memory_tree` (tree, retrieval, score, summarise) | `tree/`, `retrieval/`, `score/` | none (health/rpc/embed-compute are host-owned) | W5 | **CLEAR** |
<OPENHUMAN_ROOT>/docs/tinycortex-drift-ledger.md:107:| `memory_queue` | `queue/` | **D2** (predicate) | W4 | **OPEN** (D2) |
<OPENHUMAN_ROOT>/docs/tinycortex-drift-ledger.md:108:| `memory_conversations` | `conversations/` | **D1** (rank-before-clone) | W7 | **OPEN** (D1) |
<OPENHUMAN_ROOT>/docs/tinycortex-drift-ledger.md:109:| `memory_diff` | `diff/` | none (git-ledger captured) | W7 | **CLEAR** |
<OPENHUMAN_ROOT>/docs/tinycortex-drift-ledger.md:110:| `memory_entities` | `entities/` | none | W7 | **CLEAR** |
<OPENHUMAN_ROOT>/docs/tinycortex-drift-ledger.md:111:| `memory_graph` | `graph/` | none | W7 | **CLEAR** |
<OPENHUMAN_ROOT>/docs/tinycortex-drift-ledger.md:112:| `memory_goals` | `goals/` | none | W7 | **CLEAR** |
<OPENHUMAN_ROOT>/docs/tinycortex-drift-ledger.md:113:| `memory_archivist` | `archivist/` | none | W7 | **CLEAR** |
<OPENHUMAN_ROOT>/docs/tinycortex-drift-ledger.md:114:| `memory_sources` (registry + local readers) | `sources/` | none | W7 | **CLEAR** |
<OPENHUMAN_ROOT>/docs/tinycortex-drift-ledger.md:115:| `memory_tools` (engine part) | `tool_memory/` | none | W7 | **CLEAR** |
<OPENHUMAN_ROOT>/docs/tinycortex-drift-ledger.md:116:| `memory_search` (`vector`, `scoring` engine parts; `tools` are host) | `retrieval/`, `score/` | none (churn only) | W5 | **CLEAR** (classify tools vs engine in W5) |
<OPENHUMAN_ROOT>/docs/tinycortex-drift-ledger.md:119:tinycortex PR. Nothing else drifted. `memory_search` is a mixed module not in the plan's move table —
<OPENHUMAN_ROOT>/tests/memory_sources_readers_round21_raw_coverage_e2e.rs:5:use openhuman_core::openhuman::memory_sources::readers::SourceReader;
<OPENHUMAN_ROOT>/tests/memory_sources_readers_round21_raw_coverage_e2e.rs:6:use openhuman_core::openhuman::memory_sources::{ContentType, MemorySourceEntry, SourceKind};
<OPENHUMAN_ROOT>/tests/memory_sources_readers_round21_raw_coverage_e2e.rs:44:        .prefix("memory-sources-readers-round21-")
<OPENHUMAN_ROOT>/tests/memory_sources_readers_round21_raw_coverage_e2e.rs:57:fn source_entry(id: &str, kind: SourceKind) -> MemorySourceEntry {
<OPENHUMAN_ROOT>/tests/memory_sources_readers_round21_raw_coverage_e2e.rs:58:    MemorySourceEntry {
<OPENHUMAN_ROOT>/tests/memory_sources_readers_round21_raw_coverage_e2e.rs:121:    let reader = openhuman_core::openhuman::memory_sources::readers::rss::RssReader;
<OPENHUMAN_ROOT>/tests/memory_sources_readers_round21_raw_coverage_e2e.rs:127:            &MemorySourceEntry {
<OPENHUMAN_ROOT>/tests/memory_sources_readers_round21_raw_coverage_e2e.rs:147:            &MemorySourceEntry {
<OPENHUMAN_ROOT>/tests/memory_sources_readers_round21_raw_coverage_e2e.rs:166:            &MemorySourceEntry {
<OPENHUMAN_ROOT>/tests/memory_sources_readers_round21_raw_coverage_e2e.rs:190:    let reader = openhuman_core::openhuman::memory_sources::readers::github::GithubReader;
<OPENHUMAN_ROOT>/tests/memory_sources_readers_round21_raw_coverage_e2e.rs:191:    let entry = MemorySourceEntry {
<OPENHUMAN_ROOT>/src/core/observability.rs:134:    /// The memory-store chunk DB's per-path circuit breaker is currently open
<OPENHUMAN_ROOT>/src/core/observability.rs:142:    MemoryStoreBreakerOpen,
<OPENHUMAN_ROOT>/src/core/observability.rs:251:    /// rollback journal that needs no shared memory (issue #3231). This kind
<OPENHUMAN_ROOT>/src/core/observability.rs:257:    /// cant-open / shared-memory IO text, so transient `database is locked`
<OPENHUMAN_ROOT>/src/core/observability.rs:540:    // Deterministic user-config state, re-emitted on every memory re-embed;
<OPENHUMAN_ROOT>/src/core/observability.rs:549:    // embeddings endpoint 400s `Model <id> does not exist` on every memory
<OPENHUMAN_ROOT>/src/core/observability.rs:588:    if is_memory_store_breaker_open(&lower) {
<OPENHUMAN_ROOT>/src/core/observability.rs:589:        return Some(ExpectedErrorKind::MemoryStoreBreakerOpen);
<OPENHUMAN_ROOT>/src/core/observability.rs:654:/// `memory_store::unified::documents::tx.commit()` during
<OPENHUMAN_ROOT>/src/core/observability.rs:655:/// `openhuman.memory_doc_ingest`). `SQLITE_FULL` has only two causes:
<OPENHUMAN_ROOT>/src/core/observability.rs:661:/// the five words `"database or disk is full"` — Our local memory-store write
<OPENHUMAN_ROOT>/src/core/observability.rs:663:/// `"clear_namespace commit tx: ..."` in `memory_store::unified::documents`),
<OPENHUMAN_ROOT>/src/core/observability.rs:667:/// TAURI-RUST-4R8, `memory_queue::store::claim_next` on `mem_tree_jobs`); here
<OPENHUMAN_ROOT>/src/core/observability.rs:798:/// shared-memory IO text — *not* `database is locked`, which the store retries
<OPENHUMAN_ROOT>/src/core/observability.rs:832:/// embeddings endpoint (e.g. a chat-only provider like DeepSeek). Every memory
<OPENHUMAN_ROOT>/src/core/observability.rs:855:/// Deterministic user-config state, re-emitted on every memory re-embed; the
<OPENHUMAN_ROOT>/src/core/observability.rs:887:/// Detect the memory-store chunk DB's circuit-breaker-open message that
<OPENHUMAN_ROOT>/src/core/observability.rs:888:/// `memory_store::chunks::store::get_or_init_connection` emits via
<OPENHUMAN_ROOT>/src/core/observability.rs:892:/// `memory_tree::tree::rpc::pipeline_status_rpc`):
<OPENHUMAN_ROOT>/src/core/observability.rs:895:/// chunk aggregates: [memory_tree] circuit breaker open for <path>: too many consecutive init failures
<OPENHUMAN_ROOT>/src/core/observability.rs:898:/// The `[memory_tree]` tag is the anchor — it's specific to the chunk-store
<OPENHUMAN_ROOT>/src/core/observability.rs:902:/// mentions the `[memory_tree]` prefix doesn't get swallowed.
<OPENHUMAN_ROOT>/src/core/observability.rs:903:fn is_memory_store_breaker_open(lower: &str) -> bool {
<OPENHUMAN_ROOT>/src/core/observability.rs:904:    lower.contains("[memory_tree]") && lower.contains("circuit breaker open")
<OPENHUMAN_ROOT>/src/core/observability.rs:2084:        ExpectedErrorKind::MemoryStoreBreakerOpen => {
<OPENHUMAN_ROOT>/src/core/observability.rs:2088:                kind = "memory_store_breaker_open",
<OPENHUMAN_ROOT>/src/core/observability.rs:2089:                "[observability] {domain}.{operation} skipped expected memory-store circuit-breaker-open error"
<OPENHUMAN_ROOT>/src/core/observability.rs:2200:            // `subconscious::store` already prevents the shared-memory variant;
<OPENHUMAN_ROOT>/src/core/observability.rs:3613:            "Embedding API error (400 Bad Request): {\"error\":{\"message\":\"Model nvidia/nemotron-3-super-120b-a12b does not exist\",\"code\":400}} — this model isn't an embeddings model; pick an embeddings-capable model in Settings → Memory",
<OPENHUMAN_ROOT>/src/core/observability.rs:3634:            "Embedding API error (400 Bad Request): {\"error\":{\"code\":400,\"message\":\"BatchEmbedContentsRequest.model: unexpected model name format\",\"status\":\"INVALID_ARGUMENT\"}} — Gemini needs the embeddings model id in `models/<name>` form (e.g. `models/text-embedding-004`); fix it in Settings → Memory",
<OPENHUMAN_ROOT>/src/core/observability.rs:4034:            // memory/query/walk.rs:292 — debug-level memory walk, not a failure.
<OPENHUMAN_ROOT>/src/core/observability.rs:4035:            "[memory_tree_walk] turn=3 LLM gave up (empty response)",
<OPENHUMAN_ROOT>/src/core/observability.rs:4051:    fn classifies_memory_store_breaker_open() {
<OPENHUMAN_ROOT>/src/core/observability.rs:4055:        // `memory_tree::tree::rpc::pipeline_status_rpc`'s `chunk aggregates: …`
<OPENHUMAN_ROOT>/src/core/observability.rs:4059:            "[memory_tree] circuit breaker open for /home/u/.openhuman/workspace/memory_tree/chunks.db: too many consecutive init failures",
<OPENHUMAN_ROOT>/src/core/observability.rs:4062:            r"chunk aggregates: [memory_tree] circuit breaker open for C:\Users\u\.openhuman\users\6a09\workspace\memory_tree\chunks.db: too many consecutive init failures",
<OPENHUMAN_ROOT>/src/core/observability.rs:4065:            r"rpc.invoke_method failed: chunk aggregates: [memory_tree] circuit breaker open for /home/u/.openhuman/workspace/memory_tree/chunks.db: too many consecutive init failures",
<OPENHUMAN_ROOT>/src/core/observability.rs:4069:                Some(ExpectedErrorKind::MemoryStoreBreakerOpen),
<OPENHUMAN_ROOT>/src/core/observability.rs:4070:                "should classify memory-store breaker-open: {raw}"
<OPENHUMAN_ROOT>/src/core/observability.rs:4094:            // `memory_store::unified::documents::tx.commit()` during
<OPENHUMAN_ROOT>/src/core/observability.rs:4095:            // `openhuman.memory_doc_ingest`, in the same burst that emits
<OPENHUMAN_ROOT>/src/core/observability.rs:4100:            // TAURI-RUST-4R8, `memory_queue::store::claim_next` on
<OPENHUMAN_ROOT>/src/core/observability.rs:4123:            expected_error_kind("not enough memory to allocate buffer"),
<OPENHUMAN_ROOT>/src/core/observability.rs:4337:            "failed to run subconscious schema DDL: disk I/O error: Error code 4618: I/O error within the xShmMap method (trying to open a new shared-memory segment)",
<OPENHUMAN_ROOT>/src/core/observability.rs:4360:            "failed to open memory DB: unable to open the database file",
<OPENHUMAN_ROOT>/src/core/observability.rs:4377:            "memory queue write failed: database table is locked",
<OPENHUMAN_ROOT>/src/core/observability.rs:4390:        // Generic "circuit breaker open" without the `[memory_tree]` anchor
<OPENHUMAN_ROOT>/src/core/observability.rs:4397:        // The `[memory_tree]` tag alone is not enough — must co-occur with
<OPENHUMAN_ROOT>/src/core/observability.rs:4400:            expected_error_kind("[memory_tree] failed to run schema DDL: disk full"),
<OPENHUMAN_ROOT>/src/core/observability.rs:5312:                 provider in Settings → Memory"
<OPENHUMAN_ROOT>/src/core/observability.rs:6841:            "provider API error (500): internal error, insufficient memory"
<OPENHUMAN_ROOT>/src/core/observability.rs:6878:            "provider API error (500): internal error, insufficient memory"
<OPENHUMAN_ROOT>/src/core/observability.rs:6950:            "ollama API error (500 Internal Server Error): {\"error\":\"out of memory\"}"
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
<OPENHUMAN_ROOT>/tests/agent_harness_raw_coverage_e2e.rs:104:    ) -> Result<Vec<MemoryEntry>> {
<OPENHUMAN_ROOT>/tests/agent_harness_raw_coverage_e2e.rs:108:    async fn get(&self, _namespace: &str, _key: &str) -> Result<Option<MemoryEntry>> {
<OPENHUMAN_ROOT>/tests/agent_harness_raw_coverage_e2e.rs:115:        _category: Option<&MemoryCategory>,
<OPENHUMAN_ROOT>/tests/agent_harness_raw_coverage_e2e.rs:117:    ) -> Result<Vec<MemoryEntry>> {
<OPENHUMAN_ROOT>/tests/agent_harness_raw_coverage_e2e.rs:138:        "stub-memory"
<OPENHUMAN_ROOT>/tests/agent_harness_raw_coverage_e2e.rs:251:        .memory(Arc::new(StubMemory))
<OPENHUMAN_ROOT>/tests/agent_harness_raw_coverage_e2e.rs:262:        .omit_memory_md(true)
<OPENHUMAN_ROOT>/tests/agent_harness_raw_coverage_e2e.rs:288:        memory: Arc::new(StubMemory),
<OPENHUMAN_ROOT>/tests/agent_harness_raw_coverage_e2e.rs:291:        memory_context: Arc::new(Some("parent memory context".to_string())),
<OPENHUMAN_ROOT>/tests/agent_harness_raw_coverage_e2e.rs:310:        omit_memory_context: false,
<OPENHUMAN_ROOT>/tests/agent_harness_raw_coverage_e2e.rs:314:        omit_memory_md: true,
<OPENHUMAN_ROOT>/tests/agent_harness_raw_coverage_e2e.rs:328:        trigger_memory_agent: Default::default(),
<OPENHUMAN_ROOT>/tests/agent_harness_raw_coverage_e2e.rs:446:            && message.content.contains("parent memory context")
<OPENHUMAN_ROOT>/tests/agent_harness_raw_coverage_e2e.rs:563:omit_memory_context = true
<OPENHUMAN_ROOT>/tests/agent_harness_raw_coverage_e2e.rs:567:omit_memory_md = true
<OPENHUMAN_ROOT>/tests/agent_harness_raw_coverage_e2e.rs:585:omit_memory_context = true
