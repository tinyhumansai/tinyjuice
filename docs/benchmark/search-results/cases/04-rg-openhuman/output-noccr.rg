[search: 500 match(es) across 25 file(s) · top 5-12 per file (adaptive) · full set via retrieve footer]
<OPENHUMAN_ROOT>/src/rpc/structured_error.rs:28:pub const STRUCTURED_RPC_ERROR_SENTINEL: &str = "__OPENHUMAN_STRUCTURED_RPC_ERROR_V1__:";
<OPENHUMAN_ROOT>/src/rpc/structured_error.rs:93:        assert!(StructuredRpcError::decode("__OPENHUMAN_STRUCTURED_RPC_ERROR_V1__").is_none());
<OPENHUMAN_ROOT>/src/rpc/dispatch.rs:31:        let result = try_dispatch("openhuman.security_policy_info", json!({})).await;
<OPENHUMAN_ROOT>/src/api/jwt.rs:6:pub use crate::openhuman::credentials::session_support::get_session_token;
<OPENHUMAN_ROOT>/src/api/jwt.rs:7:pub use crate::openhuman::credentials::{APP_SESSION_PROVIDER, DEFAULT_AUTH_PROFILE_NAME};
<OPENHUMAN_ROOT>/src/api/rest_tests.rs:225:    std::env::set_var("OPENHUMAN_TAURI_VERSION", "9.8.7-shell+test");
<OPENHUMAN_ROOT>/src/api/rest_tests.rs:231:    std::env::remove_var("OPENHUMAN_TAURI_VERSION");
<OPENHUMAN_ROOT>/src/api/rest_tests.rs:244:// Regression: OPENHUMAN-TAURI-8K / Sentry issue 7473650958.
<OPENHUMAN_ROOT>/src/api/rest_tests.rs:318:    // Telegram path — matches OPENHUMAN-TAURI-2Y shape.
<OPENHUMAN_ROOT>/src/api/rest_tests.rs:363:    // OPENHUMAN-TAURI-4K8: 401 on any authed backend endpoint must surface a
<OPENHUMAN_ROOT>/src/api/config.rs:15://! 404 against the local runner — see Sentry cluster `OPENHUMAN-TAURI-51/-80/-7Z`.
<OPENHUMAN_ROOT>/src/api/config.rs:49:/// Staging hosted-API root. Activated when `OPENHUMAN_APP_ENV=staging` (or
<OPENHUMAN_ROOT>/src/api/config.rs:54:pub const APP_ENV_VAR: &str = "OPENHUMAN_APP_ENV";
<OPENHUMAN_ROOT>/src/api/config.rs:59:pub const VITE_APP_ENV_VAR: &str = "VITE_OPENHUMAN_APP_ENV";
<OPENHUMAN_ROOT>/src/api/config.rs:67:pub const OPENHUMAN_INFERENCE_PATH: &str = "/openai/v1/chat/completions";
<OPENHUMAN_ROOT>/src/api/config.rs:95:/// 2. [`effective_api_url`]`(api_url_override)` + [`OPENHUMAN_INFERENCE_PATH`] —
<OPENHUMAN_ROOT>/src/api/config.rs:117:        OPENHUMAN_INFERENCE_PATH,
<OPENHUMAN_ROOT>/src/api/config.rs:142:/// **and** does not [`looks_like_openhuman_backend_endpoint`]. In that case
<OPENHUMAN_ROOT>/src/api/config.rs:151:/// `OPENHUMAN-TAURI-51 / -80 / -7Z` — Ollama users saw every integration
<OPENHUMAN_ROOT>/src/api/config.rs:158:        let is_openhuman = looks_like_openhuman_backend_endpoint(u);
<OPENHUMAN_ROOT>/src/api/config.rs:161:        // local-AI nor an OpenHuman backend, so without this check the override
<OPENHUMAN_ROOT>/src/api/config.rs:412:                "[api/config] api_url parse failed during OpenHuman backend classification"
[+38 more match(es) in <OPENHUMAN_ROOT>/src/api/config.rs]
<OPENHUMAN_ROOT>/src/api/rest.rs:22:    /// `OPENHUMAN-TAURI-2Y` (~454 events on `/channels/telegram/messages/<id>`).
<OPENHUMAN_ROOT>/src/api/rest.rs:33:    /// flow; the auth domain owns recovery. Targets `OPENHUMAN-TAURI-4K8`
<OPENHUMAN_ROOT>/src/api/rest.rs:81:/// silently fall through to `report_error` (OPENHUMAN-TAURI-R7).
<OPENHUMAN_ROOT>/src/api/rest.rs:137:            let keys = crate::openhuman::util::truncate_at_byte_boundary(
<OPENHUMAN_ROOT>/src/api/rest.rs:185:    // The Tauri shell sets `OPENHUMAN_TAURI_VERSION` to its own package version
<OPENHUMAN_ROOT>/src/api/rest.rs:188:    if let Ok(raw) = std::env::var("OPENHUMAN_TAURI_VERSION") {
<OPENHUMAN_ROOT>/src/api/rest.rs:199:    // Linux → rustls. See [`crate::openhuman::tls::tls_client_builder`].
<OPENHUMAN_ROOT>/src/api/rest.rs:200:    crate::openhuman::tls::tls_client_builder()
<OPENHUMAN_ROOT>/src/api/rest.rs:639:            // avoid Sentry noise. Targets `OPENHUMAN-TAURI-4K8` (mascot TTS
<OPENHUMAN_ROOT>/src/api/rest.rs:666:            // `report_error`. Targets `OPENHUMAN-TAURI-2Y` (~454 events).
<OPENHUMAN_ROOT>/src/api/rest.rs:686:                // without propagating a typed error. Targets OPENHUMAN-TAURI-R7.
<OPENHUMAN_ROOT>/src/api/rest.rs:713:                && crate::openhuman::inference::provider::is_budget_exhausted_message(&text);
<OPENHUMAN_ROOT>/src/main.rs:1://! The entry point for the OpenHuman core application.
<OPENHUMAN_ROOT>/src/main.rs:6://! - Dispatching command-line arguments to the core logic in `openhuman_core`.
<OPENHUMAN_ROOT>/src/main.rs:23:    // `OPENHUMAN_DOTENV_PATH`; this early call handles the common default
<OPENHUMAN_ROOT>/src/main.rs:58:            // `openhuman::inference::provider::ops::should_report_provider_http_failure`
<OPENHUMAN_ROOT>/src/main.rs:61:            if openhuman_core::core::observability::is_transient_provider_http_failure(&event) {
<OPENHUMAN_ROOT>/src/main.rs:75:            if openhuman_core::core::observability::is_backend_error_code_event(&event) {
<OPENHUMAN_ROOT>/src/main.rs:83:            if openhuman_core::core::observability::is_transient_provider_transport_failure(&event)
<OPENHUMAN_ROOT>/src/main.rs:129:            if openhuman_core::core::observability::is_transient_backend_api_failure(&event)
<OPENHUMAN_ROOT>/src/main.rs:130:                || openhuman_core::core::observability::is_transient_integrations_failure(&event)
<OPENHUMAN_ROOT>/src/main.rs:132:                || openhuman_core::core::observability::is_skill_install_user_fetch_failure(&event)
<OPENHUMAN_ROOT>/src/main.rs:142:            if openhuman_core::core::observability::is_skills_install_client_error_event(&event) {
<OPENHUMAN_ROOT>/src/main.rs:155:            // lives at the call sites (`openhuman::inference::provider::ops::api_error`
[+30 more match(es) in <OPENHUMAN_ROOT>/src/main.rs]
<OPENHUMAN_ROOT>/src/lib.rs:1://! Core library for the OpenHuman platform.
<OPENHUMAN_ROOT>/src/lib.rs:3://! This crate provides the central logic for the OpenHuman core binary, including:
<OPENHUMAN_ROOT>/src/lib.rs:6://! - Domain-specific logic for the OpenHuman agent runtime.
<OPENHUMAN_ROOT>/src/lib.rs:10:pub mod openhuman;
<OPENHUMAN_ROOT>/src/lib.rs:13:pub use openhuman::config::DaemonConfig;
<OPENHUMAN_ROOT>/src/lib.rs:14:pub use openhuman::memory_store::{MemoryClient, MemoryState};
<OPENHUMAN_ROOT>/src/lib.rs:18:/// This is the primary entry point for the OpenHuman binary, delegating to the
<OPENHUMAN_ROOT>/src/lib.rs:30:    openhuman::service::apply_startup_restart_delay_from_env();
<OPENHUMAN_ROOT>/src/lib.rs:31:    openhuman::keyring::init_master_key();
<OPENHUMAN_ROOT>/src/bin/gmail_backfill_3d.rs:15://! - Signed-in openhuman session JWT in the same workspace the desktop app
<OPENHUMAN_ROOT>/src/bin/gmail_backfill_3d.rs:36:use openhuman_core::openhuman::composio::client::{
<OPENHUMAN_ROOT>/src/bin/gmail_backfill_3d.rs:39:use openhuman_core::openhuman::composio::providers::gmail::ingest::ingest_page_into_memory_tree;
<OPENHUMAN_ROOT>/src/bin/gmail_backfill_3d.rs:40:use openhuman_core::openhuman::composio::providers::registry::{
<OPENHUMAN_ROOT>/src/bin/gmail_backfill_3d.rs:43:use openhuman_core::openhuman::config::Config;
<OPENHUMAN_ROOT>/src/bin/gmail_backfill_3d.rs:44:use openhuman_core::openhuman::memory_queue::drain_until_idle;
<OPENHUMAN_ROOT>/src/bin/gmail_backfill_3d.rs:45:use openhuman_core::openhuman::memory_store::chunks::store::{
<OPENHUMAN_ROOT>/src/bin/gmail_backfill_3d.rs:48:use openhuman_core::openhuman::memory_store::content::read::{
<OPENHUMAN_ROOT>/src/bin/inference_probe.rs:23://!   OPENHUMAN_APP_ENV=staging \
<OPENHUMAN_ROOT>/src/bin/inference_probe.rs:24://!   RUST_LOG=info,openhuman_core::openhuman::agent=debug,openhuman_core::openhuman::inference=debug \
<OPENHUMAN_ROOT>/src/bin/inference_probe.rs:33:use openhuman_core::openhuman::agent::Agent;
<OPENHUMAN_ROOT>/src/bin/inference_probe.rs:34:use openhuman_core::openhuman::config::Config;
<OPENHUMAN_ROOT>/src/bin/inference_probe.rs:35:use openhuman_core::openhuman::inference::provider::create_chat_provider;
<OPENHUMAN_ROOT>/src/bin/inference_probe.rs:36:use openhuman_core::openhuman::inference::provider::traits::{ChatMessage, ChatRequest};
<OPENHUMAN_ROOT>/src/bin/inference_probe.rs:37:use openhuman_core::openhuman::tools::traits::ToolSpec;
<OPENHUMAN_ROOT>/src/bin/slack_backfill.rs:11://! - A working openhuman install (same workspace dir the desktop app
<OPENHUMAN_ROOT>/src/bin/slack_backfill.rs:23://! export OPENHUMAN_WORKSPACE=/path/to/workspace      # must match desktop app
<OPENHUMAN_ROOT>/src/bin/slack_backfill.rs:24://! export OPENHUMAN_MEMORY_EMBED_ENDPOINT=http://localhost:11434
<OPENHUMAN_ROOT>/src/bin/slack_backfill.rs:25://! export OPENHUMAN_MEMORY_EMBED_MODEL=nomic-embed-text
<OPENHUMAN_ROOT>/src/bin/slack_backfill.rs:26://! export OPENHUMAN_MEMORY_EXTRACT_ENDPOINT=http://localhost:11434
<OPENHUMAN_ROOT>/src/bin/slack_backfill.rs:27://! export OPENHUMAN_MEMORY_EXTRACT_MODEL=qwen2.5:0.5b
<OPENHUMAN_ROOT>/src/bin/slack_backfill.rs:28://! export OPENHUMAN_MEMORY_SUMMARISE_ENDPOINT=http://localhost:11434
<OPENHUMAN_ROOT>/src/bin/slack_backfill.rs:29://! export OPENHUMAN_MEMORY_SUMMARISE_MODEL=llama3.1:8b
<OPENHUMAN_ROOT>/src/bin/slack_backfill.rs:30://! export RUST_LOG=info,openhuman_core::openhuman::composio::providers::slack=debug,openhuman_core::openhuman::memory=debug
<OPENHUMAN_ROOT>/src/bin/slack_backfill.rs:42:use openhuman_core::openhuman::composio::client::{
<OPENHUMAN_ROOT>/src/bin/slack_backfill.rs:45:use openhuman_core::openhuman::composio::providers::registry::{
<OPENHUMAN_ROOT>/src/bin/slack_backfill.rs:48:use openhuman_core::openhuman::composio::providers::slack::run_backfill_via_search;
[+8 more match(es) in <OPENHUMAN_ROOT>/src/bin/slack_backfill.rs]
<OPENHUMAN_ROOT>/src/bin/harness_subagent_audit.rs:3://! This binary intentionally uses the user's real OpenHuman config and live
<OPENHUMAN_ROOT>/src/bin/harness_subagent_audit.rs:24:use openhuman_core::openhuman::agent::progress::AgentProgress;
<OPENHUMAN_ROOT>/src/bin/harness_subagent_audit.rs:25:use openhuman_core::openhuman::agent::Agent;
<OPENHUMAN_ROOT>/src/bin/harness_subagent_audit.rs:26:use openhuman_core::openhuman::agent_orchestration::harness_audit::{
<OPENHUMAN_ROOT>/src/bin/harness_subagent_audit.rs:29:use openhuman_core::openhuman::config::Config;
<OPENHUMAN_ROOT>/src/bin/harness_subagent_audit.rs:246:    eprintln!("[harness_subagent_audit] loading live OpenHuman config");
<OPENHUMAN_ROOT>/src/bin/memory_tree_init_smoke.rs:14://! OPENHUMAN_WORKSPACE=/tmp/mt-smoke \
<OPENHUMAN_ROOT>/src/bin/memory_tree_init_smoke.rs:18://! OPENHUMAN_WORKSPACE=/tmp/mt-smoke \
<OPENHUMAN_ROOT>/src/bin/memory_tree_init_smoke.rs:32:use openhuman_core::openhuman::config::Config;
<OPENHUMAN_ROOT>/src/bin/memory_tree_init_smoke.rs:33:use openhuman_core::openhuman::memory_store::chunks::store::with_connection;
<OPENHUMAN_ROOT>/src/bin/memory_tree_init_smoke.rs:40:    let workspace = match std::env::var("OPENHUMAN_WORKSPACE") {
<OPENHUMAN_ROOT>/src/bin/memory_tree_init_smoke.rs:43:            log::error!("OPENHUMAN_WORKSPACE must be set to a writable directory");
<OPENHUMAN_ROOT>/src/core/jsonrpc_tests.rs:21:        let lock = crate::openhuman::config::TEST_ENV_LOCK
<OPENHUMAN_ROOT>/src/core/jsonrpc_tests.rs:101:/// To run manually: `cargo test --lib -p openhuman -- --ignored
<OPENHUMAN_ROOT>/src/core/jsonrpc_tests.rs:106:    let _signed_out_restore = crate::openhuman::scheduler_gate::SignedOutTestGuard::set(false);
<OPENHUMAN_ROOT>/src/core/jsonrpc_tests.rs:120:            "OPENHUMAN_WORKSPACE",
<OPENHUMAN_ROOT>/src/core/jsonrpc_tests.rs:123:        ("OPENHUMAN_DISABLE_CHANNEL_LISTENERS", OsString::from("1")),
<OPENHUMAN_ROOT>/src/core/jsonrpc_tests.rs:125:            "OPENHUMAN_CORE_TOKEN",
<OPENHUMAN_ROOT>/src/core/jsonrpc_tests.rs:153:    let result = invoke_method(default_state(), "openhuman.health_snapshot", json!({}))
<OPENHUMAN_ROOT>/src/core/jsonrpc_tests.rs:161:    let err = invoke_method(default_state(), "openhuman.encrypt_secret", json!({}))
<OPENHUMAN_ROOT>/src/core/jsonrpc_tests.rs:647:fn is_session_expired_error_matches_openhuman_backend_path_401() {
<OPENHUMAN_ROOT>/src/core/jsonrpc_tests.rs:723:fn is_session_expired_error_matches_openhuman_session_expired_body() {
<OPENHUMAN_ROOT>/src/core/jsonrpc_tests.rs:727:        r#"OpenHuman API error (401 Unauthorized): {"success":false,"error":"Session expired. Please log in again."}"#
<OPENHUMAN_ROOT>/src/core/jsonrpc_tests.rs:864:        !message.contains("__OPENHUMAN_STRUCTURED_RPC_ERROR_V1__"),
[+59 more match(es) in <OPENHUMAN_ROOT>/src/core/jsonrpc_tests.rs]
<OPENHUMAN_ROOT>/src/core/logging.rs:1://! Logging for `openhuman run` (and other CLI paths that need stderr output).
<OPENHUMAN_ROOT>/src/core/logging.rs:6://!   * [`init_for_cli_run`] — stderr only, used by `openhuman run` / CLI
<OPENHUMAN_ROOT>/src/core/logging.rs:9://!     `<data_dir>/logs/openhuman-YYYY-MM-DD.log`, used by the Tauri shell
<OPENHUMAN_ROOT>/src/core/logging.rs:33:/// the active `openhuman-YYYY-MM-DD.log`, which on Windows is required
<OPENHUMAN_ROOT>/src/core/logging.rs:55:    /// Silence other modules; only `openhuman_core::openhuman::autocomplete::*` emits logs.
<OPENHUMAN_ROOT>/src/core/logging.rs:59:/// Custom log formatter for the OpenHuman CLI.
<OPENHUMAN_ROOT>/src/core/logging.rs:126:/// Shortens a Rust module path (e.g., `openhuman_core::rpc` -> `rpc`).
<OPENHUMAN_ROOT>/src/core/logging.rs:135:    std::env::var("OPENHUMAN_LOG_FILE_CONSTRAINTS")
<OPENHUMAN_ROOT>/src/core/logging.rs:215:///     `<data_dir>/logs/openhuman-YYYY-MM-DD.log` so packaged GUI builds —
<OPENHUMAN_ROOT>/src/core/logging.rs:245:                    .filename_prefix("openhuman")
<OPENHUMAN_ROOT>/src/core/logging.rs:329:/// Drop the file appender's worker guard so the rolling `openhuman-*.log`
<OPENHUMAN_ROOT>/src/core/logging.rs:374:            format!("off,openhuman_core::openhuman::autocomplete={level}")
[+10 more match(es) in <OPENHUMAN_ROOT>/src/core/logging.rs]
<OPENHUMAN_ROOT>/src/core/memory_cli.rs:1://! `openhuman memory` — CLI for memory ingestion, graph inspection, and debugging.
<OPENHUMAN_ROOT>/src/core/memory_cli.rs:8://!   openhuman memory ingest  <file|->  [--namespace <ns>] [--key <key>] [--title <title>] [-v]
<OPENHUMAN_ROOT>/src/core/memory_cli.rs:9://!   openhuman memory docs    [--namespace <ns>]
<OPENHUMAN_ROOT>/src/core/memory_cli.rs:10://!   openhuman memory graph   [--namespace <ns>] [--subject <s>] [--predicate <p>]
<OPENHUMAN_ROOT>/src/core/memory_cli.rs:11://!   openhuman memory query   --namespace <ns> --query <text> [--limit <n>]
<OPENHUMAN_ROOT>/src/core/memory_cli.rs:12://!   openhuman memory namespaces
<OPENHUMAN_ROOT>/src/core/memory_cli.rs:18:use crate::openhuman::memory::ingestion::{MemoryIngestionConfig, MemoryIngestionRequest};
<OPENHUMAN_ROOT>/src/core/memory_cli.rs:19:use crate::openhuman::memory_store::NamespaceDocumentInput;
<OPENHUMAN_ROOT>/src/core/memory_cli.rs:21:/// Entry point for `openhuman memory <subcommand>`.
<OPENHUMAN_ROOT>/src/core/memory_cli.rs:36:            "unknown memory subcommand '{other}'. Run `openhuman memory --help`."
<OPENHUMAN_ROOT>/src/core/memory_cli.rs:45:/// `openhuman memory ingest <file|-> [options]`
<OPENHUMAN_ROOT>/src/core/memory_cli.rs:440:    crate::openhuman::memory::global::init(config.workspace_dir).map_err(anyhow::Error::msg)
[+20 more match(es) in <OPENHUMAN_ROOT>/src/core/memory_cli.rs]
<OPENHUMAN_ROOT>/src/core/cli.rs:1://! Command-line interface for the OpenHuman core binary.
<OPENHUMAN_ROOT>/src/core/cli.rs:30:Contribute & Star us on GitHub: https://github.com/tinyhumansai/openhuman
<OPENHUMAN_ROOT>/src/core/cli.rs:67:        "mcp" | "mcp-server" => crate::openhuman::mcp_server::run_stdio_from_cli(&args[1..]),
<OPENHUMAN_ROOT>/src/core/cli.rs:71:            crate::openhuman::screen_intelligence::cli::run_screen_intelligence_command(&args[1..])
<OPENHUMAN_ROOT>/src/core/cli.rs:73:        "text-input" => crate::openhuman::text_input::cli::run_text_input_command(&args[1..]),
<OPENHUMAN_ROOT>/src/core/cli.rs:75:            crate::openhuman::memory_tree::tree_runtime::cli::run_tree_summarizer_command(
<OPENHUMAN_ROOT>/src/core/cli.rs:91:        // Generic namespace dispatcher: `openhuman <namespace> <function> ...`
<OPENHUMAN_ROOT>/src/core/cli.rs:104:/// `OPENHUMAN_CORE_SENTRY_DSN` env var (or the legacy `OPENHUMAN_SENTRY_DSN`
<OPENHUMAN_ROOT>/src/core/cli.rs:128:                println!("Usage: openhuman sentry-test [--message <text>] [--panic]");
<OPENHUMAN_ROOT>/src/core/cli.rs:186:        panic!("openhuman sentry-test intentional panic");
<OPENHUMAN_ROOT>/src/core/cli.rs:205:                anyhow::anyhow!("failed to load dotenv from OPENHUMAN_DOTENV_PATH={path}: {e}")
<OPENHUMAN_ROOT>/src/core/cli.rs:569:    println!("  openhuman sentry-test [--message <text>] [--panic]  (verify Sentry wiring)");
[+25 more match(es) in <OPENHUMAN_ROOT>/src/core/cli.rs]
<OPENHUMAN_ROOT>/README.md:1:<h1 align="center">OpenHuman</h1>
<OPENHUMAN_ROOT>/README.md:9:		<img src="https://trendshift.io/api/badge/repositories/23680" alt="tinyhumansai%2Fopenhuman | Trendshift" style="width: 250px; height: 55px;" width="250" height="55"/>
<OPENHUMAN_ROOT>/README.md:11:	<a href="https://www.producthunt.com/products/openhuman?embed=true&amp;utm_source=badge-top-post-badge&amp;utm_medium=badge&amp;utm_campaign=badge-openhuman" target="_blank" rel="noopener noreferrer">
<OPENHUMAN_ROOT>/README.md:12:		<img alt="OpenHuman - An open source AI harness built with the human in mind | Product Hunt" width="250" height="54" src="https://api.producthunt.com/widgets/embed-image/v1/top-post-badge.svg?post_id=1136902&amp;theme=light&amp;period=daily&amp;t=1778916022823">
<OPENHUMAN_ROOT>/README.md:14:		<a href="https://www.producthunt.com/products/openhuman?embed=true&amp;utm_source=badge-top-post-badge&amp;utm_medium=badge&amp;utm_campaign=badge-openhuman" target="_blank" rel="noopener noreferrer">
<OPENHUMAN_ROOT>/README.md:15:			<img alt="OpenHuman - An open source AI harness built with the human in mind | Product Hunt" width="250" height="54" src="https://api.producthunt.com/widgets/embed-image/v1/top-post-badge.svg?post_id=1136902&amp;theme=light&amp;period=weekly&amp;t=1779351403565">
<OPENHUMAN_ROOT>/README.md:19: <a href="https://www.producthunt.com/products/openhuman?embed=true&amp;utm_source=badge-top-post-topic-badge&amp;utm_medium=badge&amp;utm_campaign=badge-openhuman" target="_blank" rel="noopener noreferrer">
<OPENHUMAN_ROOT>/README.md:20:  <img alt="OpenHuman - An open source AI harness built with the human in mind | Product Hunt" width="250" height="54" src="https://api.producthunt.com/widgets/embed-image/v1/top-post-topic-badge.svg?post_id=1136902&amp;theme=light&amp;period=weekly&amp;topic_id=268&amp;t=1779351808756">
<OPENHUMAN_ROOT>/README.md:22:  <a href="https://www.producthunt.com/products/openhuman?embed=true&amp;utm_source=badge-top-post-topic-badge&amp;utm_medium=badge&amp;utm_campaign=badge-openhuman" target="_blank" rel="noopener noreferrer">
<OPENHUMAN_ROOT>/README.md:71:- **[Goals & Todos](https://tinyhumans.gitbook.io/openhuman/features/goals-and-todos)**: long-term goals, durable per-thread goals, and a shared kanban board per conversation.
<OPENHUMAN_ROOT>/README.md:72:- **[TokenJuice](https://tinyhumans.gitbook.io/openhuman/features/token-compression)**: tool output compressed before it hits the model: same information, up to 80% fewer tokens. A brain this big would be unaffordable without it.
<OPENHUMAN_ROOT>/README.md:92:- **[Privacy & security](https://tinyhumans.gitbook.io/openhuman/features/privacy-and-security)**: on-device encrypted data, approval gate, OS-keyring secrets, and opt-in sandboxing. There is also **[Privacy Mode](https://tinyhumans.gitbook.io/openhuman/features/privacy-mode)**: flip one switch and no inference leaves your machine, enforced in the Rust core.
[+46 more match(es) in <OPENHUMAN_ROOT>/README.md]
<OPENHUMAN_ROOT>/plan.md:3:Multi-agent audit of the OpenHuman test surface (2,367 files / ~25,900 test declarations per
<OPENHUMAN_ROOT>/plan.md:15:   `src/openhuman/security/policy/command_checks.rs` + `path_checks.rs` (the documented
<OPENHUMAN_ROOT>/plan.md:17:   and `src/openhuman/encryption/core.rs` (Argon2id + AES-256-GCM primitives) have **zero unit
<OPENHUMAN_ROOT>/plan.md:52:| ✅ | `src/openhuman/agent/harness/harness_gap_tests.rs` | `datetime_section_is_static_grounding_rule_not_a_volatile_timestamp` | Strict subset of `agent/prompts/mod_tests.rs::datetime_section_is_static_grounding_rule_without_volatile_timestamp`; the file's own header lists item 6 as covered elsewhere. |
<OPENHUMAN_ROOT>/plan.md:55:| ✅ | `src/openhuman/routing/factory.rs` | `factory_constructs_without_panic_when_runtime_enabled`, `factory_llamacpp_provider_constructs_without_panic`, `factory_custom_openai_provider_constructs_without_panic`, `factory_lm_studio_provider_constructs_without_panic` | Skeptic traced the whole construction path — provably infallible (pure struct init), so the tests cannot fail. One test's comment claims to verify probe-URL selection but the body asserts nothing; fields are private so strengthening is blocked. |
<OPENHUMAN_ROOT>/plan.md:58:| ⚠️ | `src/openhuman/threads/ops_tests.rs` | `sanitize_generated_title_*`, `collapse_whitespace_*`, `title_log_fingerprint_*`, `title_from_user_message_*` | `threads/title.rs` inline tests already cover the same functions with equivalent cases. Keep the copies in `title.rs` (the owning module). |
<OPENHUMAN_ROOT>/plan.md:59:| ⚠️ | `src/openhuman/channels/providers/whatsapp_tests.rs` | 7 × `whatsapp_parse_<type>_message_skipped` | All hit the identical `type != "text" → continue` branch; collapse to one parameterized loop over the type strings. |
<OPENHUMAN_ROOT>/plan.md:60:| ⚠️ | `src/openhuman/routing/telemetry.rs` | `emit_does_not_panic` ×3 variants | Fire-and-forget calls with no assertion. |
<OPENHUMAN_ROOT>/plan.md:78:| `src/openhuman/memory/schema_tests.rs` | registry-sync + unknown-fn tests | **False duplicate.** `memory/schema/` (singular, `memory_tree` namespace) and `memory/schemas/` (plural, `memory` namespace) are two distinct live registries with disjoint function sets. These are the *only* parity/unknown-fn guards for the `memory_tree` controller surface. |
<OPENHUMAN_ROOT>/plan.md:79:| `src/openhuman/channels/providers/qq_tests.rs` | `test_name` | Sole coverage of `QQChannel::name()`, which keys routing (`routes.rs:345`) and the channel map (`runtime/startup.rs:701`). A rename would ship uncaught. |
<OPENHUMAN_ROOT>/plan.md:80:| `src/openhuman/provider_surfaces/schemas.rs` | `all_schemas_returns_two` etc. | Weak but the only guard that the registration lists are populated. **Improve** (see §3), don't delete. |
<OPENHUMAN_ROOT>/plan.md:94:| ✅ | `src/openhuman/hooks/../useDaemonLifecycle.test.ts` (`app/src/hooks/__tests__/`) | Pins exact `console.log` strings as an effect-rerun proxy. Listener-count assertions are legit — keep them. | Drop the log-text pinning; keep listener/startDaemon observable assertions. |
[+4 more match(es) in <OPENHUMAN_ROOT>/plan.md]
<OPENHUMAN_ROOT>/docker-compose.yml:1:# OpenHuman Core — Docker Compose for self-hosted cloud deploy.
<OPENHUMAN_ROOT>/docker-compose.yml:3:# Brings up the headless Rust core (`openhuman-core`) on :7788, persists the
<OPENHUMAN_ROOT>/docker-compose.yml:9:#      OPENHUMAN_CORE_TOKEN; the latter is required for any client that calls
<OPENHUMAN_ROOT>/docker-compose.yml:15:# instead of building, replace `build:` with `image: ghcr.io/.../openhuman-core:<tag>`.
<OPENHUMAN_ROOT>/docker-compose.yml:18:  openhuman-core:
<OPENHUMAN_ROOT>/docker-compose.yml:22:    image: openhuman-core:local
<OPENHUMAN_ROOT>/docker-compose.yml:23:    container_name: openhuman-core
<OPENHUMAN_ROOT>/docker-compose.yml:33:      - "${OPENHUMAN_CORE_PORT:-7788}:7788"
<OPENHUMAN_ROOT>/docker-compose.yml:41:      OPENHUMAN_CORE_HOST: 0.0.0.0
<OPENHUMAN_ROOT>/docker-compose.yml:42:      OPENHUMAN_CORE_PORT: "7788"
<OPENHUMAN_ROOT>/docker-compose.yml:43:      OPENHUMAN_WORKSPACE: /home/openhuman/.openhuman
<OPENHUMAN_ROOT>/docker-compose.yml:44:      XDG_CACHE_HOME: /home/openhuman/.openhuman/cache
[+5 more match(es) in <OPENHUMAN_ROOT>/docker-compose.yml]
<OPENHUMAN_ROOT>/CONTRIBUTING-BEGINNERS.md:1:# Beginner's Guide to Contributing to OpenHuman
<OPENHUMAN_ROOT>/CONTRIBUTING-BEGINNERS.md:27:OpenHuman is a desktop AI assistant app. The codebase has three main parts:
<OPENHUMAN_ROOT>/CONTRIBUTING-BEGINNERS.md:251:1. Go to [github.com/tinyhumansai/openhuman](https://github.com/tinyhumansai/openhuman)
<OPENHUMAN_ROOT>/CONTRIBUTING-BEGINNERS.md:253:3. This creates your own copy at `github.com/YOUR_USERNAME/openhuman`
<OPENHUMAN_ROOT>/CONTRIBUTING-BEGINNERS.md:258:git clone https://github.com/YOUR_USERNAME/openhuman.git
<OPENHUMAN_ROOT>/CONTRIBUTING-BEGINNERS.md:259:cd openhuman
<OPENHUMAN_ROOT>/CONTRIBUTING-BEGINNERS.md:267:git remote add upstream https://github.com/tinyhumansai/openhuman.git
<OPENHUMAN_ROOT>/CONTRIBUTING-BEGINNERS.md:274:# origin    https://github.com/YOUR_USERNAME/openhuman.git  ← your fork
<OPENHUMAN_ROOT>/CONTRIBUTING-BEGINNERS.md:275:# upstream  https://github.com/tinyhumansai/openhuman.git   ← the original
<OPENHUMAN_ROOT>/CONTRIBUTING-BEGINNERS.md:314:pnpm --filter openhuman-app dev:app
<OPENHUMAN_ROOT>/CONTRIBUTING-BEGINNERS.md:323:1. Go to [github.com/tinyhumansai/openhuman/issues](https://github.com/tinyhumansai/openhuman/issues)
<OPENHUMAN_ROOT>/CONTRIBUTING-BEGINNERS.md:470:### Desktop build fails (`pnpm --filter openhuman-app dev:app`)
[+7 more match(es) in <OPENHUMAN_ROOT>/CONTRIBUTING-BEGINNERS.md]
<OPENHUMAN_ROOT>/src/core/event_bus/testing.rs:20://! [`crate::openhuman::agent::bus::mock_agent_run_turn`]) compose on top of
<OPENHUMAN_ROOT>/src/core/event_bus/testing.rs:24://! Tests in **any** module of `openhuman_core` can `use
<OPENHUMAN_ROOT>/src/core/event_bus/testing.rs:61:/// [`crate::openhuman::agent::bus::use_real_agent_handler`] that need the
<OPENHUMAN_ROOT>/src/core/event_bus/testing.rs:110:/// [`crate::openhuman::agent::bus::mock_agent_run_turn`]) should compose
<OPENHUMAN_ROOT>/src/core/all_tests.rs:91:    assert_eq!(rpc_method_name(&s), "openhuman.memory_doc_put");
<OPENHUMAN_ROOT>/src/core/all_tests.rs:101:    assert_eq!(rc.rpc_method_name(), "openhuman.billing_get_balance");
<OPENHUMAN_ROOT>/src/core/all_tests.rs:192:    let schema = schema_for_rpc_method("openhuman.health_snapshot");
<OPENHUMAN_ROOT>/src/core/all_tests.rs:201:    let schema = schema_for_rpc_method("openhuman.security_policy_info");
<OPENHUMAN_ROOT>/src/core/all_tests.rs:210:    let schema = schema_for_rpc_method("openhuman.mcp_audit_list");
<OPENHUMAN_ROOT>/src/core/all_tests.rs:222:    let schema = schema_for_rpc_method("openhuman.orchestration_pairing_link_session");
<OPENHUMAN_ROOT>/src/core/all_tests.rs:250:    assert!(schema_for_rpc_method("openhuman.nonexistent_method_xyz").is_none());
<OPENHUMAN_ROOT>/src/core/all_tests.rs:256:    assert_eq!(method.as_deref(), Some("openhuman.health_snapshot"));
<OPENHUMAN_ROOT>/src/core/all_tests.rs:543:    // be rejected to prevent `openhuman.   _fn` nonsense RPC method names.
<OPENHUMAN_ROOT>/src/core/all_tests.rs:594:    let out = try_invoke_registered_rpc("openhuman.not_a_real_method_xyz_123", Map::new()).await;
<OPENHUMAN_ROOT>/src/core/all_tests.rs:600:    // `openhuman.health_snapshot` is registered at startup and takes no
<OPENHUMAN_ROOT>/src/core/all_tests.rs:608:    let out = try_invoke_registered_rpc("openhuman.security_policy_info", Map::new())
[+2 more match(es) in <OPENHUMAN_ROOT>/src/core/all_tests.rs]
<OPENHUMAN_ROOT>/package.json:2:  "name": "openhuman-repo",
<OPENHUMAN_ROOT>/package.json:9:    "build": "pnpm --filter openhuman-app build",
<OPENHUMAN_ROOT>/package.json:15:    "compile": "pnpm --filter openhuman-app compile",
<OPENHUMAN_ROOT>/package.json:16:    "dev": "pnpm --filter openhuman-app dev",
<OPENHUMAN_ROOT>/package.json:17:    "dev:app": "pnpm --filter openhuman-app dev:app",
<OPENHUMAN_ROOT>/package.json:18:    "dev:app:win": "pnpm --filter openhuman-app dev:app:win",
<OPENHUMAN_ROOT>/package.json:20:    "dev:cef": "pnpm --filter openhuman-app dev:cef",
<OPENHUMAN_ROOT>/package.json:21:    "format": "pnpm --filter openhuman-app format",
<OPENHUMAN_ROOT>/package.json:22:    "format:check": "pnpm --filter openhuman-app format:check",
<OPENHUMAN_ROOT>/package.json:23:    "knip": "pnpm --filter openhuman-app knip",
<OPENHUMAN_ROOT>/package.json:24:    "knip:production": "pnpm --filter openhuman-app knip:production",
<OPENHUMAN_ROOT>/package.json:25:    "lint": "pnpm --filter openhuman-app lint",
[+14 more match(es) in <OPENHUMAN_ROOT>/package.json]
<OPENHUMAN_ROOT>/src/core/legacy_aliases.rs:26:    // `openhuman.<namespace>_<function>` form was established.
<OPENHUMAN_ROOT>/src/core/legacy_aliases.rs:27:    ("channels.list", "openhuman.channels_list"),
<OPENHUMAN_ROOT>/src/core/legacy_aliases.rs:33:    // `mcp_clients.list` sorts before all `openhuman.*` entries (m < o).
<OPENHUMAN_ROOT>/src/core/legacy_aliases.rs:34:    ("mcp_clients.list", "openhuman.mcp_clients_installed_list"),
<OPENHUMAN_ROOT>/src/core/legacy_aliases.rs:35:    ("openhuman.channels.list", "openhuman.channels_list"),
<OPENHUMAN_ROOT>/src/core/legacy_aliases.rs:37:        "openhuman.get_analytics_settings",
<OPENHUMAN_ROOT>/src/core/legacy_aliases.rs:38:        "openhuman.config_get_analytics_settings",
<OPENHUMAN_ROOT>/src/core/legacy_aliases.rs:41:        "openhuman.get_composio_trigger_settings",
<OPENHUMAN_ROOT>/src/core/legacy_aliases.rs:42:        "openhuman.config_get_composio_trigger_settings",
<OPENHUMAN_ROOT>/src/core/legacy_aliases.rs:45:        "openhuman.get_dashboard_settings",
<OPENHUMAN_ROOT>/src/core/legacy_aliases.rs:46:        "openhuman.config_get_dashboard_settings",
<OPENHUMAN_ROOT>/src/core/legacy_aliases.rs:48:    ("openhuman.get_config", "openhuman.config_get"),
[+2 more match(es) in <OPENHUMAN_ROOT>/src/core/legacy_aliases.rs]