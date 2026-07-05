[search: 500 match(es) across 25 file(s) · top 5 per file · full set via retrieve footer]
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
<OPENHUMAN_ROOT>/src/api/config.rs:412:                "[api/config] api_url parse failed during OpenHuman backend classification"
[+45 more match(es) in <OPENHUMAN_ROOT>/src/api/config.rs]
<OPENHUMAN_ROOT>/src/api/rest.rs:22:    /// `OPENHUMAN-TAURI-2Y` (~454 events on `/channels/telegram/messages/<id>`).
<OPENHUMAN_ROOT>/src/api/rest.rs:33:    /// flow; the auth domain owns recovery. Targets `OPENHUMAN-TAURI-4K8`
<OPENHUMAN_ROOT>/src/api/rest.rs:81:/// silently fall through to `report_error` (OPENHUMAN-TAURI-R7).
<OPENHUMAN_ROOT>/src/api/rest.rs:666:            // `report_error`. Targets `OPENHUMAN-TAURI-2Y` (~454 events).
<OPENHUMAN_ROOT>/src/api/rest.rs:686:                // without propagating a typed error. Targets OPENHUMAN-TAURI-R7.
[+7 more match(es) in <OPENHUMAN_ROOT>/src/api/rest.rs]
<OPENHUMAN_ROOT>/src/main.rs:58:            // `openhuman::inference::provider::ops::should_report_provider_http_failure`
<OPENHUMAN_ROOT>/src/main.rs:61:            if openhuman_core::core::observability::is_transient_provider_http_failure(&event) {
<OPENHUMAN_ROOT>/src/main.rs:75:            if openhuman_core::core::observability::is_backend_error_code_event(&event) {
<OPENHUMAN_ROOT>/src/main.rs:83:            if openhuman_core::core::observability::is_transient_provider_transport_failure(&event)
<OPENHUMAN_ROOT>/src/main.rs:129:            if openhuman_core::core::observability::is_transient_backend_api_failure(&event)
[+37 more match(es) in <OPENHUMAN_ROOT>/src/main.rs]
<OPENHUMAN_ROOT>/src/lib.rs:1://! Core library for the OpenHuman platform.
<OPENHUMAN_ROOT>/src/lib.rs:3://! This crate provides the central logic for the OpenHuman core binary, including:
<OPENHUMAN_ROOT>/src/lib.rs:6://! - Domain-specific logic for the OpenHuman agent runtime.
<OPENHUMAN_ROOT>/src/lib.rs:10:pub mod openhuman;
<OPENHUMAN_ROOT>/src/lib.rs:13:pub use openhuman::config::DaemonConfig;
[+4 more match(es) in <OPENHUMAN_ROOT>/src/lib.rs]
<OPENHUMAN_ROOT>/src/bin/gmail_backfill_3d.rs:15://! - Signed-in openhuman session JWT in the same workspace the desktop app
<OPENHUMAN_ROOT>/src/bin/gmail_backfill_3d.rs:36:use openhuman_core::openhuman::composio::client::{
<OPENHUMAN_ROOT>/src/bin/gmail_backfill_3d.rs:39:use openhuman_core::openhuman::composio::providers::gmail::ingest::ingest_page_into_memory_tree;
<OPENHUMAN_ROOT>/src/bin/gmail_backfill_3d.rs:40:use openhuman_core::openhuman::composio::providers::registry::{
<OPENHUMAN_ROOT>/src/bin/gmail_backfill_3d.rs:43:use openhuman_core::openhuman::config::Config;
[+3 more match(es) in <OPENHUMAN_ROOT>/src/bin/gmail_backfill_3d.rs]
<OPENHUMAN_ROOT>/src/bin/inference_probe.rs:23://!   OPENHUMAN_APP_ENV=staging \
<OPENHUMAN_ROOT>/src/bin/inference_probe.rs:24://!   RUST_LOG=info,openhuman_core::openhuman::agent=debug,openhuman_core::openhuman::inference=debug \
<OPENHUMAN_ROOT>/src/bin/inference_probe.rs:33:use openhuman_core::openhuman::agent::Agent;
<OPENHUMAN_ROOT>/src/bin/inference_probe.rs:34:use openhuman_core::openhuman::config::Config;
<OPENHUMAN_ROOT>/src/bin/inference_probe.rs:35:use openhuman_core::openhuman::inference::provider::create_chat_provider;
[+2 more match(es) in <OPENHUMAN_ROOT>/src/bin/inference_probe.rs]
<OPENHUMAN_ROOT>/src/bin/slack_backfill.rs:11://! - A working openhuman install (same workspace dir the desktop app
<OPENHUMAN_ROOT>/src/bin/slack_backfill.rs:23://! export OPENHUMAN_WORKSPACE=/path/to/workspace      # must match desktop app
<OPENHUMAN_ROOT>/src/bin/slack_backfill.rs:24://! export OPENHUMAN_MEMORY_EMBED_ENDPOINT=http://localhost:11434
<OPENHUMAN_ROOT>/src/bin/slack_backfill.rs:25://! export OPENHUMAN_MEMORY_EMBED_MODEL=nomic-embed-text
<OPENHUMAN_ROOT>/src/bin/slack_backfill.rs:26://! export OPENHUMAN_MEMORY_EXTRACT_ENDPOINT=http://localhost:11434
[+15 more match(es) in <OPENHUMAN_ROOT>/src/bin/slack_backfill.rs]
<OPENHUMAN_ROOT>/src/bin/harness_subagent_audit.rs:3://! This binary intentionally uses the user's real OpenHuman config and live
<OPENHUMAN_ROOT>/src/bin/harness_subagent_audit.rs:24:use openhuman_core::openhuman::agent::progress::AgentProgress;
<OPENHUMAN_ROOT>/src/bin/harness_subagent_audit.rs:25:use openhuman_core::openhuman::agent::Agent;
<OPENHUMAN_ROOT>/src/bin/harness_subagent_audit.rs:26:use openhuman_core::openhuman::agent_orchestration::harness_audit::{
<OPENHUMAN_ROOT>/src/bin/harness_subagent_audit.rs:29:use openhuman_core::openhuman::config::Config;
[+1 more match(es) in <OPENHUMAN_ROOT>/src/bin/harness_subagent_audit.rs]
<OPENHUMAN_ROOT>/src/bin/memory_tree_init_smoke.rs:14://! OPENHUMAN_WORKSPACE=/tmp/mt-smoke \
<OPENHUMAN_ROOT>/src/bin/memory_tree_init_smoke.rs:18://! OPENHUMAN_WORKSPACE=/tmp/mt-smoke \
<OPENHUMAN_ROOT>/src/bin/memory_tree_init_smoke.rs:32:use openhuman_core::openhuman::config::Config;
<OPENHUMAN_ROOT>/src/bin/memory_tree_init_smoke.rs:33:use openhuman_core::openhuman::memory_store::chunks::store::with_connection;
<OPENHUMAN_ROOT>/src/bin/memory_tree_init_smoke.rs:43:            log::error!("OPENHUMAN_WORKSPACE must be set to a writable directory");
[+1 more match(es) in <OPENHUMAN_ROOT>/src/bin/memory_tree_init_smoke.rs]
<OPENHUMAN_ROOT>/src/core/jsonrpc_tests.rs:21:        let lock = crate::openhuman::config::TEST_ENV_LOCK
<OPENHUMAN_ROOT>/src/core/jsonrpc_tests.rs:647:fn is_session_expired_error_matches_openhuman_backend_path_401() {
<OPENHUMAN_ROOT>/src/core/jsonrpc_tests.rs:723:fn is_session_expired_error_matches_openhuman_session_expired_body() {
<OPENHUMAN_ROOT>/src/core/jsonrpc_tests.rs:727:        r#"OpenHuman API error (401 Unauthorized): {"success":false,"error":"Session expired. Please log in again."}"#
<OPENHUMAN_ROOT>/src/core/jsonrpc_tests.rs:864:        !message.contains("__OPENHUMAN_STRUCTURED_RPC_ERROR_V1__"),
[+66 more match(es) in <OPENHUMAN_ROOT>/src/core/jsonrpc_tests.rs]
<OPENHUMAN_ROOT>/src/core/logging.rs:1://! Logging for `openhuman run` (and other CLI paths that need stderr output).
<OPENHUMAN_ROOT>/src/core/logging.rs:6://!   * [`init_for_cli_run`] — stderr only, used by `openhuman run` / CLI
<OPENHUMAN_ROOT>/src/core/logging.rs:9://!     `<data_dir>/logs/openhuman-YYYY-MM-DD.log`, used by the Tauri shell
<OPENHUMAN_ROOT>/src/core/logging.rs:33:/// the active `openhuman-YYYY-MM-DD.log`, which on Windows is required
<OPENHUMAN_ROOT>/src/core/logging.rs:55:    /// Silence other modules; only `openhuman_core::openhuman::autocomplete::*` emits logs.
[+17 more match(es) in <OPENHUMAN_ROOT>/src/core/logging.rs]
<OPENHUMAN_ROOT>/src/core/memory_cli.rs:1://! `openhuman memory` — CLI for memory ingestion, graph inspection, and debugging.
<OPENHUMAN_ROOT>/src/core/memory_cli.rs:8://!   openhuman memory ingest  <file|->  [--namespace <ns>] [--key <key>] [--title <title>] [-v]
<OPENHUMAN_ROOT>/src/core/memory_cli.rs:9://!   openhuman memory docs    [--namespace <ns>]
<OPENHUMAN_ROOT>/src/core/memory_cli.rs:10://!   openhuman memory graph   [--namespace <ns>] [--subject <s>] [--predicate <p>]
<OPENHUMAN_ROOT>/src/core/memory_cli.rs:440:    crate::openhuman::memory::global::init(config.workspace_dir).map_err(anyhow::Error::msg)
[+27 more match(es) in <OPENHUMAN_ROOT>/src/core/memory_cli.rs]
<OPENHUMAN_ROOT>/src/core/cli.rs:1://! Command-line interface for the OpenHuman core binary.
<OPENHUMAN_ROOT>/src/core/cli.rs:128:                println!("Usage: openhuman sentry-test [--message <text>] [--panic]");
<OPENHUMAN_ROOT>/src/core/cli.rs:186:        panic!("openhuman sentry-test intentional panic");
<OPENHUMAN_ROOT>/src/core/cli.rs:205:                anyhow::anyhow!("failed to load dotenv from OPENHUMAN_DOTENV_PATH={path}: {e}")
<OPENHUMAN_ROOT>/src/core/cli.rs:569:    println!("  openhuman sentry-test [--message <text>] [--panic]  (verify Sentry wiring)");
[+32 more match(es) in <OPENHUMAN_ROOT>/src/core/cli.rs]
<OPENHUMAN_ROOT>/README.md:1:<h1 align="center">OpenHuman</h1>
<OPENHUMAN_ROOT>/README.md:9:		<img src="https://trendshift.io/api/badge/repositories/23680" alt="tinyhumansai%2Fopenhuman | Trendshift" style="width: 250px; height: 55px;" width="250" height="55"/>
<OPENHUMAN_ROOT>/README.md:71:- **[Goals & Todos](https://tinyhumans.gitbook.io/openhuman/features/goals-and-todos)**: long-term goals, durable per-thread goals, and a shared kanban board per conversation.
<OPENHUMAN_ROOT>/README.md:72:- **[TokenJuice](https://tinyhumans.gitbook.io/openhuman/features/token-compression)**: tool output compressed before it hits the model: same information, up to 80% fewer tokens. A brain this big would be unaffordable without it.
<OPENHUMAN_ROOT>/README.md:92:- **[Privacy & security](https://tinyhumans.gitbook.io/openhuman/features/privacy-and-security)**: on-device encrypted data, approval gate, OS-keyring secrets, and opt-in sandboxing. There is also **[Privacy Mode](https://tinyhumans.gitbook.io/openhuman/features/privacy-mode)**: flip one switch and no inference leaves your machine, enforced in the Rust core.
[+53 more match(es) in <OPENHUMAN_ROOT>/README.md]
<OPENHUMAN_ROOT>/plan.md:3:Multi-agent audit of the OpenHuman test surface (2,367 files / ~25,900 test declarations per
<OPENHUMAN_ROOT>/plan.md:15:   `src/openhuman/security/policy/command_checks.rs` + `path_checks.rs` (the documented
<OPENHUMAN_ROOT>/plan.md:55:| ✅ | `src/openhuman/routing/factory.rs` | `factory_constructs_without_panic_when_runtime_enabled`, `factory_llamacpp_provider_constructs_without_panic`, `factory_custom_openai_provider_constructs_without_panic`, `factory_lm_studio_provider_constructs_without_panic` | Skeptic traced the whole construction path — provably infallible (pure struct init), so the tests cannot fail. One test's comment claims to verify probe-URL selection but the body asserts nothing; fields are private so strengthening is blocked. |
<OPENHUMAN_ROOT>/plan.md:60:| ⚠️ | `src/openhuman/routing/telemetry.rs` | `emit_does_not_panic` ×3 variants | Fire-and-forget calls with no assertion. |
<OPENHUMAN_ROOT>/plan.md:94:| ✅ | `src/openhuman/hooks/../useDaemonLifecycle.test.ts` (`app/src/hooks/__tests__/`) | Pins exact `console.log` strings as an effect-rerun proxy. Listener-count assertions are legit — keep them. | Drop the log-text pinning; keep listener/startDaemon observable assertions. |
[+11 more match(es) in <OPENHUMAN_ROOT>/plan.md]
<OPENHUMAN_ROOT>/docker-compose.yml:1:# OpenHuman Core — Docker Compose for self-hosted cloud deploy.
<OPENHUMAN_ROOT>/docker-compose.yml:3:# Brings up the headless Rust core (`openhuman-core`) on :7788, persists the
<OPENHUMAN_ROOT>/docker-compose.yml:9:#      OPENHUMAN_CORE_TOKEN; the latter is required for any client that calls
<OPENHUMAN_ROOT>/docker-compose.yml:15:# instead of building, replace `build:` with `image: ghcr.io/.../openhuman-core:<tag>`.
<OPENHUMAN_ROOT>/docker-compose.yml:18:  openhuman-core:
[+12 more match(es) in <OPENHUMAN_ROOT>/docker-compose.yml]
<OPENHUMAN_ROOT>/CONTRIBUTING-BEGINNERS.md:1:# Beginner's Guide to Contributing to OpenHuman
<OPENHUMAN_ROOT>/CONTRIBUTING-BEGINNERS.md:27:OpenHuman is a desktop AI assistant app. The codebase has three main parts:
<OPENHUMAN_ROOT>/CONTRIBUTING-BEGINNERS.md:251:1. Go to [github.com/tinyhumansai/openhuman](https://github.com/tinyhumansai/openhuman)
<OPENHUMAN_ROOT>/CONTRIBUTING-BEGINNERS.md:253:3. This creates your own copy at `github.com/YOUR_USERNAME/openhuman`
<OPENHUMAN_ROOT>/CONTRIBUTING-BEGINNERS.md:258:git clone https://github.com/YOUR_USERNAME/openhuman.git
[+14 more match(es) in <OPENHUMAN_ROOT>/CONTRIBUTING-BEGINNERS.md]
<OPENHUMAN_ROOT>/src/core/event_bus/testing.rs:20://! [`crate::openhuman::agent::bus::mock_agent_run_turn`]) compose on top of
<OPENHUMAN_ROOT>/src/core/event_bus/testing.rs:24://! Tests in **any** module of `openhuman_core` can `use
<OPENHUMAN_ROOT>/src/core/event_bus/testing.rs:61:/// [`crate::openhuman::agent::bus::use_real_agent_handler`] that need the
<OPENHUMAN_ROOT>/src/core/event_bus/testing.rs:110:/// [`crate::openhuman::agent::bus::mock_agent_run_turn`]) should compose
<OPENHUMAN_ROOT>/src/core/all_tests.rs:91:    assert_eq!(rpc_method_name(&s), "openhuman.memory_doc_put");
<OPENHUMAN_ROOT>/src/core/all_tests.rs:101:    assert_eq!(rc.rpc_method_name(), "openhuman.billing_get_balance");
<OPENHUMAN_ROOT>/src/core/all_tests.rs:192:    let schema = schema_for_rpc_method("openhuman.health_snapshot");
<OPENHUMAN_ROOT>/src/core/all_tests.rs:201:    let schema = schema_for_rpc_method("openhuman.security_policy_info");
<OPENHUMAN_ROOT>/src/core/all_tests.rs:608:    let out = try_invoke_registered_rpc("openhuman.security_policy_info", Map::new())
[+9 more match(es) in <OPENHUMAN_ROOT>/src/core/all_tests.rs]
<OPENHUMAN_ROOT>/package.json:2:  "name": "openhuman-repo",
<OPENHUMAN_ROOT>/package.json:9:    "build": "pnpm --filter openhuman-app build",
<OPENHUMAN_ROOT>/package.json:15:    "compile": "pnpm --filter openhuman-app compile",
<OPENHUMAN_ROOT>/package.json:16:    "dev": "pnpm --filter openhuman-app dev",
<OPENHUMAN_ROOT>/package.json:17:    "dev:app": "pnpm --filter openhuman-app dev:app",
[+21 more match(es) in <OPENHUMAN_ROOT>/package.json]
<OPENHUMAN_ROOT>/src/core/legacy_aliases.rs:26:    // `openhuman.<namespace>_<function>` form was established.
<OPENHUMAN_ROOT>/src/core/legacy_aliases.rs:27:    ("channels.list", "openhuman.channels_list"),
<OPENHUMAN_ROOT>/src/core/legacy_aliases.rs:33:    // `mcp_clients.list` sorts before all `openhuman.*` entries (m < o).
<OPENHUMAN_ROOT>/src/core/legacy_aliases.rs:34:    ("mcp_clients.list", "openhuman.mcp_clients_installed_list"),
<OPENHUMAN_ROOT>/src/core/legacy_aliases.rs:35:    ("openhuman.channels.list", "openhuman.channels_list"),
[+9 more match(es) in <OPENHUMAN_ROOT>/src/core/legacy_aliases.rs]

[compacted tool output — this is a PARTIAL view; the full original (65126 bytes) is available by calling tokenjuice_retrieve with token "ba24059e9c7dc5ab08a24a693877be1a" (marker ⟦tj:ba24059e9c7dc5ab08a24a693877be1a⟧)]