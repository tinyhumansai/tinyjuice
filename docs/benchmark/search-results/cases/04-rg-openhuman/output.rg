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
<OPENHUMAN_ROOT>/CONTRIBUTING.md:1:# Contributing to OpenHuman
<OPENHUMAN_ROOT>/CONTRIBUTING.md:3:Thank you for your interest in contributing to OpenHuman. This guide is the fast path for getting a fresh checkout running locally, validating changes, and opening a pull request without having to piece together setup notes from multiple files.
<OPENHUMAN_ROOT>/CONTRIBUTING.md:28:- Check [open issues](https://github.com/tinyhumansai/openhuman/issues) and discussions before starting work.
<OPENHUMAN_ROOT>/CONTRIBUTING.md:103:- **Desktop development** needs the vendored Tauri/CEF setup. The preferred entrypoint is `pnpm --filter openhuman-app dev:app`, which ensures the vendored Tauri CLI is installed and configures `CEF_PATH`.
<OPENHUMAN_ROOT>/CONTRIBUTING.md:105:- **Windows 10 WSL + classic X11 forwarding** is unsupported for the desktop app. The Tauri/CEF stack can hang, render blank windows, or crash before useful app logs are available. Use native Windows development, or Windows 11 WSLg if you need a Linux GUI workflow. OpenHuman logs a startup warning when it detects WSL with `DISPLAY` set but no `WAYLAND_DISPLAY`/WSLg markers.
[+20 more match(es) in <OPENHUMAN_ROOT>/CONTRIBUTING.md]
<OPENHUMAN_ROOT>/SECURITY.md:5:We provide security updates for the following versions of OpenHuman:
<OPENHUMAN_ROOT>/SECURITY.md:13:We recommend always running the [latest release](https://github.com/tinyhumansai/openhuman/releases/latest). OpenHuman is in early beta; older versions may not receive patches.
<OPENHUMAN_ROOT>/SECURITY.md:22:2. Email the maintainers with a clear description of the issue, steps to reproduce, and impact. You can reach us via the contact details listed in the [OpenHuman organization](https://github.com/openhumanxyz) or repository.
<OPENHUMAN_ROOT>/SECURITY.md:41:Out-of-scope for this process: general bugs, feature requests, and issues in third-party services we integrate with (e.g., Telegram, Notion) unless they are specific to how OpenHuman uses them.
<OPENHUMAN_ROOT>/SECURITY.md:53:Thank you for helping keep OpenHuman and its users safe.
<OPENHUMAN_ROOT>/AGENTS.md:1:# OpenHuman
<OPENHUMAN_ROOT>/AGENTS.md:13:| **`app/`**              | pnpm workspace `openhuman-app`: Vite + React (`app/src/`), Tauri desktop host (`app/src-tauri/`), Vitest tests                |
<OPENHUMAN_ROOT>/AGENTS.md:14:| **`src/`** (root)       | Rust lib crate `openhuman` + `openhuman-core` CLI binary (`src/main.rs`) — `src/core/` (transport), `src/openhuman/*` domains |
<OPENHUMAN_ROOT>/AGENTS.md:101:The `[autonomy]` block (`src/openhuman/config/schema/autonomy.rs`) drives `SecurityPolicy` (`src/openhuman/security/policy.rs`). Tiers: `readonly` / `supervised` / `full` × `workspace_only` × `trusted_roots` × `allow_tool_install`. Edit via `config.update_autonomy_settings` RPC or Settings → Agent access.
<OPENHUMAN_ROOT>/AGENTS.md:268:Update `src/openhuman/about_app/` when adding/removing/renaming user-facing features. Define E2E scenarios up front covering happy paths, failures, auth gates.
[+23 more match(es) in <OPENHUMAN_ROOT>/AGENTS.md]
<OPENHUMAN_ROOT>/INSTALL.md:1:# Installing OpenHuman
<OPENHUMAN_ROOT>/INSTALL.md:3:Download installers from [tinyhumans.ai/openhuman](https://tinyhumans.ai/openhuman?utm_source=github&utm_medium=readme) or from the [GitHub Releases](https://github.com/tinyhumansai/openhuman/releases/latest) page. For terminal installs, the native package paths below are preferred because they use your OS package manager or native installer where available.
<OPENHUMAN_ROOT>/INSTALL.md:13:brew install openhuman
<OPENHUMAN_ROOT>/INSTALL.md:19:# Download OpenHuman_<version>_amd64.deb or OpenHuman_<version>_arm64.deb
<OPENHUMAN_ROOT>/INSTALL.md:31:> **Linux:** the AppImage can crash on launch under Wayland, miss host system libraries such as `libgbm.so.1`, or fail on Arch-based distros with `sharun: Interpreter not found!`. See [#2463](https://github.com/tinyhumansai/openhuman/issues/2463) for the cause and env-var workarounds. The `.deb` package above avoids those failure modes on Debian/Ubuntu by letting apt resolve runtime dependencies.
[+9 more match(es) in <OPENHUMAN_ROOT>/INSTALL.md]
<OPENHUMAN_ROOT>/Dockerfile:2:# OpenHuman Core — multi-stage Docker build
<OPENHUMAN_ROOT>/Dockerfile:3:# Produces a minimal image running the `openhuman-core` binary (JSON-RPC server).
<OPENHUMAN_ROOT>/Dockerfile:5:# Build:   docker build -t openhuman-core .
<OPENHUMAN_ROOT>/Dockerfile:6:# Run:     docker run -p 7788:7788 --env-file .env openhuman-core
<OPENHUMAN_ROOT>/Dockerfile:57:    cargo build --profile "${CARGO_PROFILE}" --bin openhuman-core 2>/dev/null || true && \
[+13 more match(es) in <OPENHUMAN_ROOT>/Dockerfile]
<OPENHUMAN_ROOT>/package.json:2:  "name": "openhuman-repo",
<OPENHUMAN_ROOT>/package.json:9:    "build": "pnpm --filter openhuman-app build",
<OPENHUMAN_ROOT>/package.json:15:    "compile": "pnpm --filter openhuman-app compile",
<OPENHUMAN_ROOT>/package.json:16:    "dev": "pnpm --filter openhuman-app dev",
<OPENHUMAN_ROOT>/package.json:17:    "dev:app": "pnpm --filter openhuman-app dev:app",
[+21 more match(es) in <OPENHUMAN_ROOT>/package.json]
<OPENHUMAN_ROOT>/Cargo.lock:4343:name = "openhuman"
<OPENHUMAN_ROOT>/README.md:1:<h1 align="center">OpenHuman</h1>
<OPENHUMAN_ROOT>/README.md:9:		<img src="https://trendshift.io/api/badge/repositories/23680" alt="tinyhumansai%2Fopenhuman | Trendshift" style="width: 250px; height: 55px;" width="250" height="55"/>
<OPENHUMAN_ROOT>/README.md:71:- **[Goals & Todos](https://tinyhumans.gitbook.io/openhuman/features/goals-and-todos)**: long-term goals, durable per-thread goals, and a shared kanban board per conversation.
<OPENHUMAN_ROOT>/README.md:72:- **[TokenJuice](https://tinyhumans.gitbook.io/openhuman/features/token-compression)**: tool output compressed before it hits the model: same information, up to 80% fewer tokens. A brain this big would be unaffordable without it.
<OPENHUMAN_ROOT>/README.md:92:- **[Privacy & security](https://tinyhumans.gitbook.io/openhuman/features/privacy-and-security)**: on-device encrypted data, approval gate, OS-keyring secrets, and opt-in sandboxing. There is also **[Privacy Mode](https://tinyhumans.gitbook.io/openhuman/features/privacy-mode)**: flip one switch and no inference leaves your machine, enforced in the Rust core.
[+53 more match(es) in <OPENHUMAN_ROOT>/README.md]
<OPENHUMAN_ROOT>/CONTRIBUTING-BEGINNERS.md:1:# Beginner's Guide to Contributing to OpenHuman
<OPENHUMAN_ROOT>/CONTRIBUTING-BEGINNERS.md:27:OpenHuman is a desktop AI assistant app. The codebase has three main parts:
<OPENHUMAN_ROOT>/CONTRIBUTING-BEGINNERS.md:251:1. Go to [github.com/tinyhumansai/openhuman](https://github.com/tinyhumansai/openhuman)
<OPENHUMAN_ROOT>/CONTRIBUTING-BEGINNERS.md:253:3. This creates your own copy at `github.com/YOUR_USERNAME/openhuman`
<OPENHUMAN_ROOT>/CONTRIBUTING-BEGINNERS.md:258:git clone https://github.com/YOUR_USERNAME/openhuman.git
[+14 more match(es) in <OPENHUMAN_ROOT>/CONTRIBUTING-BEGINNERS.md]
<OPENHUMAN_ROOT>/src/core/jsonrpc_tests.rs:21:        let lock = crate::openhuman::config::TEST_ENV_LOCK
<OPENHUMAN_ROOT>/src/core/jsonrpc_tests.rs:647:fn is_session_expired_error_matches_openhuman_backend_path_401() {
<OPENHUMAN_ROOT>/src/core/jsonrpc_tests.rs:723:fn is_session_expired_error_matches_openhuman_session_expired_body() {
<OPENHUMAN_ROOT>/src/core/jsonrpc_tests.rs:727:        r#"OpenHuman API error (401 Unauthorized): {"success":false,"error":"Session expired. Please log in again."}"#
<OPENHUMAN_ROOT>/src/core/jsonrpc_tests.rs:864:        !message.contains("__OPENHUMAN_STRUCTURED_RPC_ERROR_V1__"),
[+66 more match(es) in <OPENHUMAN_ROOT>/src/core/jsonrpc_tests.rs]
<OPENHUMAN_ROOT>/docs/README.ko.md:1:<h1 align="center">OpenHuman</h1>
<OPENHUMAN_ROOT>/docs/README.ko.md:9:		<img src="https://trendshift.io/api/badge/repositories/23680" alt="tinyhumansai%2Fopenhuman | Trendshift" style="width: 250px; height: 55px;" width="250" height="55"/>
<OPENHUMAN_ROOT>/docs/README.ko.md:69:- **[목표 및 할 일(Goals & Todos)](https://tinyhumans.gitbook.io/openhuman/features/goals-and-todos)**: 장기 목표, 스레드별 지속 목표, 그리고 대화별 공유 칸반 보드를 제공합니다.
<OPENHUMAN_ROOT>/docs/README.ko.md:70:- **[TokenJuice](https://tinyhumans.gitbook.io/openhuman/features/token-compression)**: 도구 출력은 모델에 닿기 전에 압축되어, 동일한 정보가 최대 80% 적은 토큰으로 전달됩니다. 이것 없이는 이만큼 큰 두뇌를 감당할 수 없을 것입니다.
<OPENHUMAN_ROOT>/docs/README.ko.md:90:- **[개인 정보 보호 및 보안](https://tinyhumans.gitbook.io/openhuman/features/privacy-and-security)**: 기기 내 암호화된 데이터, 승인 게이트, OS 키링 시크릿, 선택적 샌드박싱, 그리고 **[Privacy Mode](https://tinyhumans.gitbook.io/openhuman/features/privacy-mode)**: 스위치 하나로 어떤 추론도 당신의 머신을 떠나지 않으며, Rust 코어에서 강제됩니다.
[+52 more match(es) in <OPENHUMAN_ROOT>/docs/README.ko.md]
<OPENHUMAN_ROOT>/docker-compose.yml:1:# OpenHuman Core — Docker Compose for self-hosted cloud deploy.
<OPENHUMAN_ROOT>/docker-compose.yml:3:# Brings up the headless Rust core (`openhuman-core`) on :7788, persists the
<OPENHUMAN_ROOT>/docker-compose.yml:9:#      OPENHUMAN_CORE_TOKEN; the latter is required for any client that calls
<OPENHUMAN_ROOT>/docker-compose.yml:15:# instead of building, replace `build:` with `image: ghcr.io/.../openhuman-core:<tag>`.
<OPENHUMAN_ROOT>/docker-compose.yml:18:  openhuman-core:
[+3 more match(es) in <OPENHUMAN_ROOT>/docker-compose.yml]

[compacted tool output — this is a PARTIAL view; the full original (76783 bytes) is available by calling tokenjuice_retrieve with token "7e6fd2d5f4dda8dc30022aa809577a58" (marker ⟦tj:7e6fd2d5f4dda8dc30022aa809577a58⟧)]