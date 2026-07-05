[search: 500 match(es) across 32 file(s) · top 5 per file · full set via retrieve footer]
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
<OPENHUMAN_ROOT>/CONTRIBUTING.md:1:# Contributing to OpenHuman
<OPENHUMAN_ROOT>/CONTRIBUTING.md:3:Thank you for your interest in contributing to OpenHuman. This guide is the fast path for getting a fresh checkout running locally, validating changes, and opening a pull request without having to piece together setup notes from multiple files.
<OPENHUMAN_ROOT>/CONTRIBUTING.md:28:- Check [open issues](https://github.com/tinyhumansai/openhuman/issues) and discussions before starting work.
<OPENHUMAN_ROOT>/CONTRIBUTING.md:103:- **Desktop development** needs the vendored Tauri/CEF setup. The preferred entrypoint is `pnpm --filter openhuman-app dev:app`, which ensures the vendored Tauri CLI is installed and configures `CEF_PATH`.
<OPENHUMAN_ROOT>/CONTRIBUTING.md:105:- **Windows 10 WSL + classic X11 forwarding** is unsupported for the desktop app. The Tauri/CEF stack can hang, render blank windows, or crash before useful app logs are available. Use native Windows development, or Windows 11 WSLg if you need a Linux GUI workflow. OpenHuman logs a startup warning when it detects WSL with `DISPLAY` set but no `WAYLAND_DISPLAY`/WSLg markers.
[+20 more match(es) in <OPENHUMAN_ROOT>/CONTRIBUTING.md]
<OPENHUMAN_ROOT>/CONTRIBUTING-BEGINNERS.md:1:# Beginner's Guide to Contributing to OpenHuman
<OPENHUMAN_ROOT>/CONTRIBUTING-BEGINNERS.md:27:OpenHuman is a desktop AI assistant app. The codebase has three main parts:
<OPENHUMAN_ROOT>/CONTRIBUTING-BEGINNERS.md:251:1. Go to [github.com/tinyhumansai/openhuman](https://github.com/tinyhumansai/openhuman)
<OPENHUMAN_ROOT>/CONTRIBUTING-BEGINNERS.md:253:3. This creates your own copy at `github.com/YOUR_USERNAME/openhuman`
<OPENHUMAN_ROOT>/CONTRIBUTING-BEGINNERS.md:258:git clone https://github.com/YOUR_USERNAME/openhuman.git
[+14 more match(es) in <OPENHUMAN_ROOT>/CONTRIBUTING-BEGINNERS.md]
<OPENHUMAN_ROOT>/SECURITY.md:5:We provide security updates for the following versions of OpenHuman:
<OPENHUMAN_ROOT>/SECURITY.md:13:We recommend always running the [latest release](https://github.com/tinyhumansai/openhuman/releases/latest). OpenHuman is in early beta; older versions may not receive patches.
<OPENHUMAN_ROOT>/SECURITY.md:22:2. Email the maintainers with a clear description of the issue, steps to reproduce, and impact. You can reach us via the contact details listed in the [OpenHuman organization](https://github.com/openhumanxyz) or repository.
<OPENHUMAN_ROOT>/SECURITY.md:41:Out-of-scope for this process: general bugs, feature requests, and issues in third-party services we integrate with (e.g., Telegram, Notion) unless they are specific to how OpenHuman uses them.
<OPENHUMAN_ROOT>/SECURITY.md:53:Thank you for helping keep OpenHuman and its users safe.
<OPENHUMAN_ROOT>/pnpm-workspace.yaml:5:  # a `postinstall` that downloads a pre-built openhuman binary from a GitHub
<OPENHUMAN_ROOT>/src/core/jsonrpc_tests.rs:21:        let lock = crate::openhuman::config::TEST_ENV_LOCK
<OPENHUMAN_ROOT>/src/core/jsonrpc_tests.rs:647:fn is_session_expired_error_matches_openhuman_backend_path_401() {
<OPENHUMAN_ROOT>/src/core/jsonrpc_tests.rs:723:fn is_session_expired_error_matches_openhuman_session_expired_body() {
<OPENHUMAN_ROOT>/src/core/jsonrpc_tests.rs:727:        r#"OpenHuman API error (401 Unauthorized): {"success":false,"error":"Session expired. Please log in again."}"#
<OPENHUMAN_ROOT>/src/core/jsonrpc_tests.rs:864:        !message.contains("__OPENHUMAN_STRUCTURED_RPC_ERROR_V1__"),
[+66 more match(es) in <OPENHUMAN_ROOT>/src/core/jsonrpc_tests.rs]
<OPENHUMAN_ROOT>/Cargo.toml:2:name = "openhuman"
<OPENHUMAN_ROOT>/Cargo.toml:51:# TinyJuice — host-agnostic TokenJuice compression engine. OpenHuman keeps
<OPENHUMAN_ROOT>/Cargo.toml:52:# config/RPC/tool/runtime adapters in `src/openhuman/tokenjuice/` and patches
<OPENHUMAN_ROOT>/Cargo.toml:81:# the language-agnostic brace-depth heuristic. See src/openhuman/tokenjuice/compressors/code.rs.
<OPENHUMAN_ROOT>/Cargo.toml:397:#     debuginfo, so panics/backtraces in OpenHuman code still resolve to
[+20 more match(es) in <OPENHUMAN_ROOT>/Cargo.toml]
<OPENHUMAN_ROOT>/package.json:2:  "name": "openhuman-repo",
<OPENHUMAN_ROOT>/package.json:9:    "build": "pnpm --filter openhuman-app build",
<OPENHUMAN_ROOT>/package.json:15:    "compile": "pnpm --filter openhuman-app compile",
<OPENHUMAN_ROOT>/package.json:16:    "dev": "pnpm --filter openhuman-app dev",
<OPENHUMAN_ROOT>/package.json:17:    "dev:app": "pnpm --filter openhuman-app dev:app",
[+21 more match(es) in <OPENHUMAN_ROOT>/package.json]
<OPENHUMAN_ROOT>/docker-compose.yml:1:# OpenHuman Core — Docker Compose for self-hosted cloud deploy.
<OPENHUMAN_ROOT>/docker-compose.yml:3:# Brings up the headless Rust core (`openhuman-core`) on :7788, persists the
<OPENHUMAN_ROOT>/docker-compose.yml:9:#      OPENHUMAN_CORE_TOKEN; the latter is required for any client that calls
<OPENHUMAN_ROOT>/docker-compose.yml:15:# instead of building, replace `build:` with `image: ghcr.io/.../openhuman-core:<tag>`.
<OPENHUMAN_ROOT>/docker-compose.yml:18:  openhuman-core:
[+12 more match(es) in <OPENHUMAN_ROOT>/docker-compose.yml]
<OPENHUMAN_ROOT>/e2e/docker-compose.yml:6:#   - same image: ghcr.io/tinyhumansai/openhuman_ci:latest
<OPENHUMAN_ROOT>/e2e/docker-compose.yml:14:#     bash -lc "pnpm install --frozen-lockfile && pnpm --filter openhuman-app test:e2e:build"
<OPENHUMAN_ROOT>/e2e/docker-compose.yml:33:    image: ${OPENHUMAN_CI_IMAGE:-ghcr.io/tinyhumansai/openhuman_ci:latest}
<OPENHUMAN_ROOT>/e2e/docker-compose.yml:88:    image: ${OPENHUMAN_CI_IMAGE:-ghcr.io/tinyhumansai/openhuman_ci:latest}
<OPENHUMAN_ROOT>/src/core/types.rs:86:    /// The name of the method to be invoked (e.g., `openhuman.memory_doc_put`).
<OPENHUMAN_ROOT>/src/core/types.rs:140:    /// The current version of the OpenHuman core binary, usually from `CARGO_PKG_VERSION`.
<OPENHUMAN_ROOT>/src/core/types.rs:154:/// - [`HostKind::Cli`] — `openhuman-core` standalone binary spawned by an
<OPENHUMAN_ROOT>/src/core/types.rs:163:/// `OPENHUMAN_DOCKER=1`). Tauri-shell callers MUST pass `TauriShell`
<OPENHUMAN_ROOT>/src/core/types.rs:179:        if std::env::var("OPENHUMAN_DOCKER")
[+3 more match(es) in <OPENHUMAN_ROOT>/src/core/types.rs]
<OPENHUMAN_ROOT>/src/core/auth.rs:16://!    [`init_rpc_token`] reads `OPENHUMAN_CORE_TOKEN` from the environment.
<OPENHUMAN_ROOT>/src/core/auth.rs:56://!   `openhuman::inference::http::EXTERNAL_OPENAI_COMPAT_PROVIDER`.
<OPENHUMAN_ROOT>/src/core/auth.rs:71:use crate::openhuman::config::Config;
<OPENHUMAN_ROOT>/src/core/auth.rs:72:use crate::openhuman::credentials::AuthService;
<OPENHUMAN_ROOT>/src/core/auth.rs:73:use crate::openhuman::inference::http::EXTERNAL_OPENAI_COMPAT_PROVIDER;
[+11 more match(es) in <OPENHUMAN_ROOT>/src/core/auth.rs]
<OPENHUMAN_ROOT>/plan.md:3:Multi-agent audit of the OpenHuman test surface (2,367 files / ~25,900 test declarations per
<OPENHUMAN_ROOT>/plan.md:15:   `src/openhuman/security/policy/command_checks.rs` + `path_checks.rs` (the documented
<OPENHUMAN_ROOT>/plan.md:55:| ✅ | `src/openhuman/routing/factory.rs` | `factory_constructs_without_panic_when_runtime_enabled`, `factory_llamacpp_provider_constructs_without_panic`, `factory_custom_openai_provider_constructs_without_panic`, `factory_lm_studio_provider_constructs_without_panic` | Skeptic traced the whole construction path — provably infallible (pure struct init), so the tests cannot fail. One test's comment claims to verify probe-URL selection but the body asserts nothing; fields are private so strengthening is blocked. |
<OPENHUMAN_ROOT>/plan.md:60:| ⚠️ | `src/openhuman/routing/telemetry.rs` | `emit_does_not_panic` ×3 variants | Fire-and-forget calls with no assertion. |
<OPENHUMAN_ROOT>/plan.md:94:| ✅ | `src/openhuman/hooks/../useDaemonLifecycle.test.ts` (`app/src/hooks/__tests__/`) | Pins exact `console.log` strings as an effect-rerun proxy. Listener-count assertions are legit — keep them. | Drop the log-text pinning; keep listener/startDaemon observable assertions. |
[+11 more match(es) in <OPENHUMAN_ROOT>/plan.md]
<OPENHUMAN_ROOT>/Dockerfile:2:# OpenHuman Core — multi-stage Docker build
<OPENHUMAN_ROOT>/Dockerfile:3:# Produces a minimal image running the `openhuman-core` binary (JSON-RPC server).
<OPENHUMAN_ROOT>/Dockerfile:5:# Build:   docker build -t openhuman-core .
<OPENHUMAN_ROOT>/Dockerfile:6:# Run:     docker run -p 7788:7788 --env-file .env openhuman-core
<OPENHUMAN_ROOT>/Dockerfile:57:    cargo build --profile "${CARGO_PROFILE}" --bin openhuman-core 2>/dev/null || true && \
[+13 more match(es) in <OPENHUMAN_ROOT>/Dockerfile]
<OPENHUMAN_ROOT>/e2e/run-local.sh:30:    "${RUN[@]}" bash -lc "pnpm --filter openhuman-app test:e2e:build"
<OPENHUMAN_ROOT>/e2e/run-local.sh:51:    "${RUN[@]}" bash -lc "pnpm --filter openhuman-app test:e2e:build"
<OPENHUMAN_ROOT>/e2e/docker-local-bootstrap.sh:5:# The CI image (`ghcr.io/tinyhumansai/openhuman_ci:latest`) intentionally
<OPENHUMAN_ROOT>/e2e/docker-local-bootstrap.sh:97:# repo (e.g. `../secrets/openhuman/.env`). Inside this container only
<OPENHUMAN_ROOT>/e2e/docker-local-bootstrap.sh:111:  # source via OPENHUMAN_DOTENV_FILE / VITE handles env loading itself.
<OPENHUMAN_ROOT>/e2e/docker-local-bootstrap.sh:113:  stub="${HOME}/openhuman-e2e-$(echo "$target" | tr '/' '_').env"
<OPENHUMAN_ROOT>/src/core/event_bus/testing.rs:20://! [`crate::openhuman::agent::bus::mock_agent_run_turn`]) compose on top of
<OPENHUMAN_ROOT>/src/core/event_bus/testing.rs:24://! Tests in **any** module of `openhuman_core` can `use
<OPENHUMAN_ROOT>/src/core/event_bus/testing.rs:61:/// [`crate::openhuman::agent::bus::use_real_agent_handler`] that need the
<OPENHUMAN_ROOT>/src/core/event_bus/testing.rs:110:/// [`crate::openhuman::agent::bus::mock_agent_run_turn`]) should compose
<OPENHUMAN_ROOT>/fastlane/metadata/en-US/description.txt:1:OpenHuman for iPhone is the mobile companion for the OpenHuman desktop app.
<OPENHUMAN_ROOT>/fastlane/metadata/en-US/description.txt:3:Pair your phone with OpenHuman on your computer, then keep the conversation going from your pocket. Ask questions, send quick voice notes, and stay connected to the same assistant that understands your desktop context, memory, and connected tools.
<OPENHUMAN_ROOT>/fastlane/metadata/en-US/description.txt:7:- Pair securely with your OpenHuman desktop app by scanning a QR code
<OPENHUMAN_ROOT>/fastlane/metadata/en-US/description.txt:13:OpenHuman is built for people who want an AI assistant that can remember, reason, and work with the tools they already use. The iPhone app is intentionally lightweight: it is a remote control and conversation surface for your desktop OpenHuman runtime.
<OPENHUMAN_ROOT>/fastlane/metadata/en-US/description.txt:19:- Your long-term workspace, memory, and integration state remain managed by your OpenHuman desktop setup
[+2 more match(es) in <OPENHUMAN_ROOT>/fastlane/metadata/en-US/description.txt]
<OPENHUMAN_ROOT>/e2e/docker-entrypoint.sh:46:# Run the provided command (default: yarn workspace openhuman-app test:e2e:all)
<OPENHUMAN_ROOT>/src/core/all.rs:1://! Registry and dispatch logic for all OpenHuman controllers.
<OPENHUMAN_ROOT>/src/core/all.rs:43:    /// Returns the canonical RPC method name for this controller (e.g., `openhuman.memory_doc_put`).
<OPENHUMAN_ROOT>/src/core/all.rs:92:            handler: crate::openhuman::voice::cli::run_standalone_subcommand,
<OPENHUMAN_ROOT>/src/core/all.rs:108:    controllers.extend(crate::openhuman::about_app::all_about_app_registered_controllers());
<OPENHUMAN_ROOT>/src/core/all.rs:158:    controllers.extend(crate::openhuman::security::all_security_registered_controllers());
[+56 more match(es) in <OPENHUMAN_ROOT>/src/core/all.rs]

[compacted tool output — this is a PARTIAL view; the full original (59524 bytes) is available by calling tokenjuice_retrieve with token "45af3d3d94678eb06dd8b2bc3f38029d" (marker ⟦tj:45af3d3d94678eb06dd8b2bc3f38029d⟧)]