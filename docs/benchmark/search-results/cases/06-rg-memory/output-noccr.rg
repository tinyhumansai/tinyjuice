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
<OPENHUMAN_ROOT>/src/core/event_bind_tokens.rs:19://! This module owns only the in-memory store; the RPC handler that mints
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
<OPENHUMAN_ROOT>/src/core/subconscious_cli.rs:113:        // Init memory client
<OPENHUMAN_ROOT>/src/core/subconscious_cli.rs:114:        let _ = crate::openhuman::memory::global::init(config.workspace_dir.clone());
<OPENHUMAN_ROOT>/src/core/subconscious_cli.rs:144:        // Create engine and run tick. The engine pulls its own memory_diff /
<OPENHUMAN_ROOT>/src/core/subconscious_cli.rs:145:        // context state from the workspace — no memory client to pass in.
<OPENHUMAN_ROOT>/src/core/subconscious_cli.rs:146:        let engine = crate::openhuman::subconscious::memory_instance(&config);
<OPENHUMAN_ROOT>/src/core/subconscious_cli.rs:165:                        conn, "memory",
<OPENHUMAN_ROOT>/src/core/subconscious_cli.rs:205:                crate::openhuman::subconscious::store::get_last_tick_at(conn, "memory")
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
<OPENHUMAN_ROOT>/src/core/dispatch.rs:400:        assert!(!is_known_probe_method("memory_tree_create_namespace"));
<OPENHUMAN_ROOT>/src/core/mod.rs:20:pub mod memory_cli;
<OPENHUMAN_ROOT>/src/core/mod.rs:36:    /// Domain/group identifier, e.g. `memory`, `config`, `credentials`.
<OPENHUMAN_ROOT>/src/core/mod.rs:53:    /// Canonical dotted name for routing, e.g. `memory.doc_put`.
<OPENHUMAN_ROOT>/src/core/mod.rs:134:        let s = mk("memory", "doc_put");
<OPENHUMAN_ROOT>/src/core/mod.rs:135:        assert_eq!(s.method_name(), "memory.doc_put");
<OPENHUMAN_ROOT>/src/core/mod.rs:142:        let s = mk("memory", "doc_put");
<OPENHUMAN_ROOT>/src/core/mod.rs:143:        assert_eq!(s.method_name(), "memory.doc_put");
<OPENHUMAN_ROOT>/src/core/mod.rs:146:            "openhuman.memory_doc_put"
<OPENHUMAN_ROOT>/tests/memory_roundtrip_e2e.rs:1://! Memory subsystem round-trip integration test (#773 PR-A).
<OPENHUMAN_ROOT>/tests/memory_roundtrip_e2e.rs:4://! against a real local memory client backed by the workspace store under a
<OPENHUMAN_ROOT>/tests/memory_roundtrip_e2e.rs:7://! Counterpart to `app/test/e2e/specs/memory-roundtrip.spec.ts` which exercises
<OPENHUMAN_ROOT>/tests/memory_roundtrip_e2e.rs:11://! Run with: `cargo test --test memory_roundtrip_e2e`
<OPENHUMAN_ROOT>/tests/memory_roundtrip_e2e.rs:18:use openhuman_core::openhuman::memory::ops::{
<OPENHUMAN_ROOT>/tests/memory_roundtrip_e2e.rs:19:    clear_namespace, doc_put, memory_recall_context, memory_recall_memories, ClearNamespaceParams,
<OPENHUMAN_ROOT>/tests/memory_roundtrip_e2e.rs:22:use openhuman_core::openhuman::memory::rpc_models::{RecallContextRequest, RecallMemoriesRequest};
<OPENHUMAN_ROOT>/tests/memory_roundtrip_e2e.rs:63:const NS: &str = "memory-roundtrip-e2e-773";
<OPENHUMAN_ROOT>/tests/memory_roundtrip_e2e.rs:65:const TITLE: &str = "Memory roundtrip canary";
<OPENHUMAN_ROOT>/tests/memory_roundtrip_e2e.rs:66:const CONTENT: &str = "OpenHuman memory roundtrip canary fact #773";
<OPENHUMAN_ROOT>/tests/memory_roundtrip_e2e.rs:124:    let recall_outcome = memory_recall_memories(recall_request())
<OPENHUMAN_ROOT>/tests/memory_roundtrip_e2e.rs:126:        .expect("memory_recall_memories rpc");
<OPENHUMAN_ROOT>/tests/memory_roundtrip_e2e.rs:136:/// block, not only in the raw memory list view.
<OPENHUMAN_ROOT>/tests/memory_roundtrip_e2e.rs:148:    let recall_outcome = memory_recall_context(recall_context_request())
<OPENHUMAN_ROOT>/tests/memory_roundtrip_e2e.rs:150:        .expect("memory_recall_context rpc");
<OPENHUMAN_ROOT>/tests/memory_roundtrip_e2e.rs:243:    let pre = memory_recall_memories(recall_request())
<OPENHUMAN_ROOT>/tests/memory_roundtrip_e2e.rs:265:    let post = memory_recall_memories(recall_request())
<OPENHUMAN_ROOT>/src/core/event_bus/mod.rs:5://! modules (like memory, skills, and agents) to communicate without
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
<OPENHUMAN_ROOT>/src/core/types.rs:86:    /// The name of the method to be invoked (e.g., `openhuman.memory_doc_put`).
<OPENHUMAN_ROOT>/src/core/event_bus/README.md:13:- `pub enum DomainEvent` — `events.rs` — `#[non_exhaustive]` catalog of events; current variants cover Agent (`AgentTurnStarted/Completed`, `AgentError`), Memory (`MemoryStored`, `MemoryRecalled`), Channels (`ChannelInboundMessage`, `ChannelMessageReceived/Processed`, `ChannelReactionReceived/Sent`, `ChannelConnected/Disconnected`), Cron (`CronJobTriggered/Completed`, `CronDeliveryRequested`), Skills, Tools, Webhooks, and System.
<OPENHUMAN_ROOT>/src/core/event_bus/README.md:32:- `src/openhuman/memory/conversations/bus.rs` — conversation persistence subscriber.
<OPENHUMAN_ROOT>/tests/agent_memory_loader_public.rs:3:use openhuman_core::openhuman::agent_memory::memory_loader::{DefaultMemoryLoader, MemoryLoader};
<OPENHUMAN_ROOT>/tests/agent_memory_loader_public.rs:4:use openhuman_core::openhuman::memory::{Memory, MemoryCategory, MemoryEntry};
<OPENHUMAN_ROOT>/tests/agent_memory_loader_public.rs:7:struct ScriptedMemory {
<OPENHUMAN_ROOT>/tests/agent_memory_loader_public.rs:8:    primary: Vec<MemoryEntry>,
<OPENHUMAN_ROOT>/tests/agent_memory_loader_public.rs:9:    working: Vec<MemoryEntry>,
<OPENHUMAN_ROOT>/tests/agent_memory_loader_public.rs:13:impl Memory for ScriptedMemory {
<OPENHUMAN_ROOT>/tests/agent_memory_loader_public.rs:19:        _category: MemoryCategory,
<OPENHUMAN_ROOT>/tests/agent_memory_loader_public.rs:29:        _opts: openhuman_core::openhuman::memory::RecallOpts<'_>,
<OPENHUMAN_ROOT>/tests/agent_memory_loader_public.rs:30:    ) -> Result<Vec<MemoryEntry>> {
<OPENHUMAN_ROOT>/tests/agent_memory_loader_public.rs:38:    async fn get(&self, _namespace: &str, _key: &str) -> Result<Option<MemoryEntry>> {
<OPENHUMAN_ROOT>/tests/agent_memory_loader_public.rs:45:        _category: Option<&MemoryCategory>,
<OPENHUMAN_ROOT>/tests/agent_memory_loader_public.rs:47:    ) -> Result<Vec<MemoryEntry>> {
<OPENHUMAN_ROOT>/tests/agent_memory_loader_public.rs:57:    ) -> Result<Vec<openhuman_core::openhuman::memory::NamespaceSummary>> {
<OPENHUMAN_ROOT>/tests/agent_memory_loader_public.rs:74:fn entry(key: &str, content: &str, score: Option<f64>) -> MemoryEntry {
<OPENHUMAN_ROOT>/tests/agent_memory_loader_public.rs:75:    MemoryEntry {
<OPENHUMAN_ROOT>/tests/agent_memory_loader_public.rs:80:        category: MemoryCategory::Conversation,
<OPENHUMAN_ROOT>/tests/agent_memory_loader_public.rs:89:async fn loader_skips_primary_recall_and_filters_working_memory() -> Result<()> {
<OPENHUMAN_ROOT>/tests/agent_memory_loader_public.rs:90:    // The open-ended `[Memory context]` recall block was removed: it duplicated
<OPENHUMAN_ROOT>/tests/agent_memory_loader_public.rs:91:    // what the memory tree + memory search tool already cover, and would echo
<OPENHUMAN_ROOT>/tests/agent_memory_loader_public.rs:93:    // emits the bounded `[User working memory]` block.
<OPENHUMAN_ROOT>/tests/agent_memory_loader_public.rs:94:    let memory: Arc<dyn Memory> = Arc::new(ScriptedMemory {
<OPENHUMAN_ROOT>/tests/agent_memory_loader_public.rs:105:    let context = DefaultMemoryLoader::new(5, 0.4)
<OPENHUMAN_ROOT>/tests/agent_memory_loader_public.rs:107:        .load_context(memory.as_ref(), "hello")
<OPENHUMAN_ROOT>/tests/agent_memory_loader_public.rs:110:    assert!(!context.contains("[Memory context]"));
<OPENHUMAN_ROOT>/tests/agent_memory_loader_public.rs:113:    assert!(context.contains("[User working memory]"));
<OPENHUMAN_ROOT>/tests/agent_memory_loader_public.rs:120:async fn loader_can_return_only_working_memory_when_primary_is_empty() -> Result<()> {
<OPENHUMAN_ROOT>/tests/agent_memory_loader_public.rs:121:    let memory: Arc<dyn Memory> = Arc::new(ScriptedMemory {
<OPENHUMAN_ROOT>/tests/agent_memory_loader_public.rs:126:    let context = DefaultMemoryLoader::default()
<OPENHUMAN_ROOT>/tests/agent_memory_loader_public.rs:127:        .load_context(memory.as_ref(), "hello")
<OPENHUMAN_ROOT>/tests/agent_memory_loader_public.rs:130:    assert!(!context.contains("[Memory context]"));
<OPENHUMAN_ROOT>/tests/agent_memory_loader_public.rs:131:    assert!(context.contains("[User working memory]"));
<OPENHUMAN_ROOT>/tests/agent_memory_loader_public.rs:138:    // Primary `[Memory context]` recall is no longer injected, so any
<OPENHUMAN_ROOT>/tests/agent_memory_loader_public.rs:140:    // Tight budgets that can't fit the `[User working memory]` header should
<OPENHUMAN_ROOT>/tests/agent_memory_loader_public.rs:142:    let memory: Arc<dyn Memory> = Arc::new(ScriptedMemory {
<OPENHUMAN_ROOT>/tests/agent_memory_loader_public.rs:147:    let header = "[User working memory]\n";
<OPENHUMAN_ROOT>/tests/agent_memory_loader_public.rs:148:    let empty = DefaultMemoryLoader::new(1, 0.4)
<OPENHUMAN_ROOT>/tests/agent_memory_loader_public.rs:150:        .load_context(memory.as_ref(), "hello")
<OPENHUMAN_ROOT>/tests/agent_memory_loader_public.rs:155:    let bounded = DefaultMemoryLoader::new(1, 0.4)
<OPENHUMAN_ROOT>/tests/agent_memory_loader_public.rs:157:        .load_context(memory.as_ref(), "hello")
<OPENHUMAN_ROOT>/tests/agent_memory_loader_public.rs:159:    assert!(bounded.contains("[User working memory]"));
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
<OPENHUMAN_ROOT>/tests/agent_archivist_debug_round21_raw_coverage_e2e.rs:19:use openhuman_core::openhuman::memory::{
<OPENHUMAN_ROOT>/tests/agent_archivist_debug_round21_raw_coverage_e2e.rs:20:    Memory, MemoryCategory, MemoryEntry, NamespaceSummary, RecallOpts,
<OPENHUMAN_ROOT>/tests/agent_archivist_debug_round21_raw_coverage_e2e.rs:22:use openhuman_core::openhuman::memory_store::{events, fts5, profile, segments};
<OPENHUMAN_ROOT>/tests/agent_archivist_debug_round21_raw_coverage_e2e.rs:91:struct StubMemory;
<OPENHUMAN_ROOT>/tests/agent_archivist_debug_round21_raw_coverage_e2e.rs:94:impl Memory for StubMemory {
<OPENHUMAN_ROOT>/tests/agent_archivist_debug_round21_raw_coverage_e2e.rs:96:        "round21-memory"
<OPENHUMAN_ROOT>/tests/agent_archivist_debug_round21_raw_coverage_e2e.rs:104:        _category: MemoryCategory,
<OPENHUMAN_ROOT>/tests/agent_archivist_debug_round21_raw_coverage_e2e.rs:115:    ) -> Result<Vec<MemoryEntry>> {
<OPENHUMAN_ROOT>/tests/agent_archivist_debug_round21_raw_coverage_e2e.rs:119:    async fn get(&self, _namespace: &str, _key: &str) -> Result<Option<MemoryEntry>> {
<OPENHUMAN_ROOT>/tests/agent_archivist_debug_round21_raw_coverage_e2e.rs:126:        _category: Option<&MemoryCategory>,
<OPENHUMAN_ROOT>/tests/agent_archivist_debug_round21_raw_coverage_e2e.rs:128:    ) -> Result<Vec<MemoryEntry>> {
<OPENHUMAN_ROOT>/tests/agent_archivist_debug_round21_raw_coverage_e2e.rs:180:    let conn = Connection::open_in_memory().expect("in-memory sqlite");
<OPENHUMAN_ROOT>/tests/agent_archivist_debug_round21_raw_coverage_e2e.rs:243:        omit_memory_context: false,
<OPENHUMAN_ROOT>/tests/agent_archivist_debug_round21_raw_coverage_e2e.rs:247:        omit_memory_md: true,
<OPENHUMAN_ROOT>/tests/agent_archivist_debug_round21_raw_coverage_e2e.rs:248:        trigger_memory_agent: Default::default(),
<OPENHUMAN_ROOT>/tests/agent_archivist_debug_round21_raw_coverage_e2e.rs:291:        memory: Arc::new(StubMemory),
<OPENHUMAN_ROOT>/tests/agent_archivist_debug_round21_raw_coverage_e2e.rs:294:        memory_context: Arc::new(Some("parent memory".to_string())),
<OPENHUMAN_ROOT>/docs/README.de.md:66:- **[Memory Tree](https://tinyhumans.gitbook.io/openhuman/features/memory-tree) + [Obsidian-Wiki](https://tinyhumans.gitbook.io/openhuman/features/obsidian-wiki)**: deine Daten, komprimiert in bewertete Markdown-Bäume in SQLite auf deiner Maschine, gespiegelt als [Obsidian-Vault](https://x.com/karpathy/status/2039805659525644595), das du öffnen und editieren kannst. Keine Vektor-Suppen-Blackbox.
<OPENHUMAN_ROOT>/docs/README.de.md:98: <img src="../gitbooks/.gitbook/assets/memory.png" alt="Diagramm zum OpenHuman-Kontextaufbau">
<OPENHUMAN_ROOT>/docs/README.de.md:101:> OpenHuman fasst all deine Dokumente, E-Mails und Chats zusammen, komprimiert sie und legt einen Memory Graph an, mit dem dein Agent sich alles über dich merken kann.
<OPENHUMAN_ROOT>/docs/README.de.md:103:OpenHuman überspringt die Wartezeit. Verbinde deine Accounts, lass [Auto-Fetch](https://tinyhumans.gitbook.io/openhuman/features/integrations/auto-fetch) die Daten lokal in einer 20-Minuten-Schleife abholen, und [Memory Trees](https://tinyhumans.gitbook.io/openhuman/features/memory-tree) komprimieren alles in Markdown-Dateien, intelligent abgelegt in einem [Obsidian-Wiki im Karpathy-Stil](https://tinyhumans.gitbook.io/openhuman/features/obsidian-wiki).
<OPENHUMAN_ROOT>/docs/README.de.md:107:Du hostest [agentmemory](https://github.com/rohitg00/agentmemory) bereits selbst für andere Coding-Agenten? OpenHuman bringt ein optionales `Memory`-Backend mit, das dorthin proxyt: setze `memory.backend = "agentmemory"` in `config.toml`, und derselbe persistente Store treibt OpenHuman zusammen mit Claude Code, Cursor, Codex und OpenCode an. Setup-Details auf der Seite zum [agentmemory-Backend](https://tinyhumans.gitbook.io/openhuman/features/obsidian-wiki/agentmemory-backend).
<OPENHUMAN_ROOT>/docs/README.de.md:144:| **Memory**             | ✅ chat-gebunden      | ⚠️ plugin-abhängig | ✅ selbstlernend   | 🚀 Memory Tree + Obsidian-Vault, optional [agentmemory](https://github.com/rohitg00/agentmemory)-Backend |
<OPENHUMAN_ROOT>/docs/README.de.md:146:| **Auto-Fetch**         | 🚫 keiner             | 🚫 keiner          | 🚫 keiner          | ✅ 20-Min.-Sync ins Memory                                                                               |
<OPENHUMAN_ROOT>/tests/agent_turn_builder_leftovers_raw_coverage_e2e.rs:7:use openhuman_core::openhuman::context::session_memory::SessionMemoryConfig;
<OPENHUMAN_ROOT>/tests/agent_turn_builder_leftovers_raw_coverage_e2e.rs:11:use openhuman_core::openhuman::memory::{
<OPENHUMAN_ROOT>/tests/agent_turn_builder_leftovers_raw_coverage_e2e.rs:12:    Memory, MemoryCategory, MemoryEntry, NamespaceSummary, RecallOpts,
<OPENHUMAN_ROOT>/tests/agent_turn_builder_leftovers_raw_coverage_e2e.rs:120:struct RecordingMemory {
<OPENHUMAN_ROOT>/tests/agent_turn_builder_leftovers_raw_coverage_e2e.rs:121:    stores: Mutex<Vec<(String, String, String, MemoryCategory)>>,
<OPENHUMAN_ROOT>/tests/agent_turn_builder_leftovers_raw_coverage_e2e.rs:124:impl RecordingMemory {
<OPENHUMAN_ROOT>/tests/agent_turn_builder_leftovers_raw_coverage_e2e.rs:133:impl Memory for RecordingMemory {
<OPENHUMAN_ROOT>/tests/agent_turn_builder_leftovers_raw_coverage_e2e.rs:135:        "round20-recording-memory"
<OPENHUMAN_ROOT>/tests/agent_turn_builder_leftovers_raw_coverage_e2e.rs:143:        category: MemoryCategory,
<OPENHUMAN_ROOT>/tests/agent_turn_builder_leftovers_raw_coverage_e2e.rs:160:    ) -> Result<Vec<MemoryEntry>> {
<OPENHUMAN_ROOT>/tests/agent_turn_builder_leftovers_raw_coverage_e2e.rs:161:        Ok(vec![MemoryEntry {
<OPENHUMAN_ROOT>/tests/agent_turn_builder_leftovers_raw_coverage_e2e.rs:162:            id: "round20-memory".to_string(),
<OPENHUMAN_ROOT>/tests/agent_turn_builder_leftovers_raw_coverage_e2e.rs:166:            category: MemoryCategory::Core,
<OPENHUMAN_ROOT>/tests/agent_turn_builder_leftovers_raw_coverage_e2e.rs:177:    async fn get(&self, _namespace: &str, _key: &str) -> Result<Option<MemoryEntry>> {
<OPENHUMAN_ROOT>/tests/agent_turn_builder_leftovers_raw_coverage_e2e.rs:184:        _category: Option<&MemoryCategory>,
<OPENHUMAN_ROOT>/tests/agent_turn_builder_leftovers_raw_coverage_e2e.rs:186:    ) -> Result<Vec<MemoryEntry>> {
<OPENHUMAN_ROOT>/tests/agent_turn_builder_leftovers_raw_coverage_e2e.rs:394:        .memory(RecordingMemory::new())
<OPENHUMAN_ROOT>/tests/agent_turn_builder_leftovers_raw_coverage_e2e.rs:445:        .memory(RecordingMemory::new())
<OPENHUMAN_ROOT>/tests/agent_turn_builder_leftovers_raw_coverage_e2e.rs:490:async fn session_memory_threshold_path_runs_only_after_successful_turn() {
<OPENHUMAN_ROOT>/tests/agent_turn_builder_leftovers_raw_coverage_e2e.rs:492:    let (_temp, workspace_path) = workspace("session-memory-threshold");
<OPENHUMAN_ROOT>/tests/agent_turn_builder_leftovers_raw_coverage_e2e.rs:525:        .memory(RecordingMemory::new())
<OPENHUMAN_ROOT>/tests/agent_turn_builder_leftovers_raw_coverage_e2e.rs:539:            session_memory: SessionMemoryConfig {
<OPENHUMAN_ROOT>/tests/agent_turn_builder_leftovers_raw_coverage_e2e.rs:552:        .turn("trigger session memory thresholds")
<OPENHUMAN_ROOT>/tests/agent_turn_builder_leftovers_raw_coverage_e2e.rs:566:        .memory(RecordingMemory::new())
<OPENHUMAN_ROOT>/vendor/tinycortex/src/lib.rs:3://! TinyCortex is the memory engine extracted from OpenHuman as a standalone,
<OPENHUMAN_ROOT>/vendor/tinycortex/src/lib.rs:8://! The public surface lives under [`memory`]. See `docs/openhuman-memory-engine-spec.md`
<OPENHUMAN_ROOT>/vendor/tinycortex/src/lib.rs:11:pub mod memory;
<OPENHUMAN_ROOT>/vendor/tinychannels/src/context.rs:21:pub const MEMORY_CONTEXT_MAX_ENTRIES: usize = 4;
<OPENHUMAN_ROOT>/vendor/tinychannels/src/context.rs:22:pub const MEMORY_CONTEXT_ENTRY_MAX_CHARS: usize = 800;
<OPENHUMAN_ROOT>/vendor/tinychannels/src/context.rs:23:pub const MEMORY_CONTEXT_MAX_CHARS: usize = 4_000;
<OPENHUMAN_ROOT>/vendor/tinychannels/src/context.rs:56:pub struct MemoryEntry {
<OPENHUMAN_ROOT>/vendor/tinychannels/src/context.rs:63:pub trait Memory: Send + Sync {
<OPENHUMAN_ROOT>/vendor/tinychannels/src/context.rs:64:    async fn recall(&self, query: &str, limit: usize) -> anyhow::Result<Vec<MemoryEntry>>;
<OPENHUMAN_ROOT>/vendor/tinychannels/src/context.rs:71:pub fn conversation_memory_key(msg: &ChannelMessage) -> String {
<OPENHUMAN_ROOT>/vendor/tinychannels/src/context.rs:127:pub fn should_skip_memory_context_entry(key: &str, content: &str) -> bool {
<OPENHUMAN_ROOT>/vendor/tinychannels/src/context.rs:132:    content.chars().count() > MEMORY_CONTEXT_MAX_CHARS
<OPENHUMAN_ROOT>/vendor/tinychannels/src/context.rs:151:pub async fn build_memory_context(
<OPENHUMAN_ROOT>/vendor/tinychannels/src/context.rs:152:    mem: &dyn Memory,
<OPENHUMAN_ROOT>/vendor/tinychannels/src/context.rs:166:            if included >= MEMORY_CONTEXT_MAX_ENTRIES {
<OPENHUMAN_ROOT>/vendor/tinychannels/src/context.rs:170:            if should_skip_memory_context_entry(&entry.key, &entry.content) {
<OPENHUMAN_ROOT>/vendor/tinychannels/src/context.rs:174:            let content = if entry.content.chars().count() > MEMORY_CONTEXT_ENTRY_MAX_CHARS {
<OPENHUMAN_ROOT>/vendor/tinychannels/src/context.rs:175:                truncate_with_ellipsis(&entry.content, MEMORY_CONTEXT_ENTRY_MAX_CHARS)
<OPENHUMAN_ROOT>/vendor/tinychannels/src/context.rs:182:            if used_chars + line_chars > MEMORY_CONTEXT_MAX_CHARS {
<OPENHUMAN_ROOT>/vendor/tinychannels/src/context.rs:187:                context.push_str("[Memory context]\n");
<OPENHUMAN_ROOT>/vendor/tinychannels/src/context.rs:207:    struct MockMemory {
<OPENHUMAN_ROOT>/vendor/tinychannels/src/context.rs:208:        entries: Vec<MemoryEntry>,
<OPENHUMAN_ROOT>/vendor/tinychannels/src/context.rs:212:    impl Memory for MockMemory {
<OPENHUMAN_ROOT>/vendor/tinychannels/src/context.rs:213:        async fn recall(&self, _query: &str, _limit: usize) -> anyhow::Result<Vec<MemoryEntry>> {
<OPENHUMAN_ROOT>/vendor/tinychannels/src/context.rs:233:        assert_eq!(conversation_memory_key(&telegram), "telegram_alice_m1");
<OPENHUMAN_ROOT>/vendor/tinychannels/src/context.rs:269:    fn memory_context_skip_and_overflow_detection_match_openhuman_hints() {
<OPENHUMAN_ROOT>/vendor/tinychannels/src/context.rs:270:        assert!(should_skip_memory_context_entry("note_history", "short"));
<OPENHUMAN_ROOT>/vendor/tinychannels/src/context.rs:271:        assert!(should_skip_memory_context_entry(
<OPENHUMAN_ROOT>/vendor/tinychannels/src/context.rs:273:            &"x".repeat(MEMORY_CONTEXT_MAX_CHARS + 1)
<OPENHUMAN_ROOT>/vendor/tinychannels/src/context.rs:275:        assert!(!should_skip_memory_context_entry("note", "short"));
<OPENHUMAN_ROOT>/vendor/tinychannels/src/context.rs:284:    async fn build_memory_context_filters_entries_and_truncates_content() {
<OPENHUMAN_ROOT>/vendor/tinychannels/src/context.rs:285:        let memory = MockMemory {
<OPENHUMAN_ROOT>/vendor/tinychannels/src/context.rs:287:                MemoryEntry {
<OPENHUMAN_ROOT>/vendor/tinychannels/src/context.rs:292:                MemoryEntry {
<OPENHUMAN_ROOT>/vendor/tinychannels/src/context.rs:297:                MemoryEntry {
<OPENHUMAN_ROOT>/vendor/tinychannels/src/context.rs:302:                MemoryEntry {
<OPENHUMAN_ROOT>/vendor/tinychannels/src/context.rs:304:                    content: "x".repeat(MEMORY_CONTEXT_ENTRY_MAX_CHARS + 50),
<OPENHUMAN_ROOT>/vendor/tinychannels/src/context.rs:310:        let rendered = build_memory_context(&memory, "hello", 0.4).await;
<OPENHUMAN_ROOT>/vendor/tinychannels/src/context.rs:311:        assert!(rendered.contains("[Memory context]"));
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
