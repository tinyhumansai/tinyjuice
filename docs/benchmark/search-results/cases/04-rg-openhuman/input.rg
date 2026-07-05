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
<OPENHUMAN_ROOT>/src/api/config.rs:165:        // of the local-AI guard (OPENHUMAN-TAURI-51/-80/-7Z, Ollama).
<OPENHUMAN_ROOT>/src/api/config.rs:167:            crate::openhuman::config::schema::cloud_providers::endpoint_host(u).is_some_and(|h| {
<OPENHUMAN_ROOT>/src/api/config.rs:168:                crate::openhuman::config::schema::cloud_providers::host_is_builtin_cloud_provider(
<OPENHUMAN_ROOT>/src/api/config.rs:178:            is_openhuman,
<OPENHUMAN_ROOT>/src/api/config.rs:196:        // arm already covered Ollama (`OPENHUMAN-TAURI-51 / -80 / -7Z`); this
<OPENHUMAN_ROOT>/src/api/config.rs:207:        if (!is_local_ai && !is_inference_provider && !is_cloud_inference) || is_openhuman {
<OPENHUMAN_ROOT>/src/api/config.rs:229:    // `OPENHUMAN-TAURI-H6 / -HN`, issue #2075).
<OPENHUMAN_ROOT>/src/api/config.rs:238:/// runner rather than the hosted OpenHuman backend.
<OPENHUMAN_ROOT>/src/api/config.rs:304:/// an OpenHuman control-plane backend — so backend calls must NOT route there.
<OPENHUMAN_ROOT>/src/api/config.rs:310:/// recognised by [`looks_like_openhuman_backend_endpoint`] and must route.
<OPENHUMAN_ROOT>/src/api/config.rs:334:/// provider** base rather than the hosted OpenHuman backend.
<OPENHUMAN_ROOT>/src/api/config.rs:347:///    never an OpenHuman control-plane base. A bare `/v1/chat/completions` is
<OPENHUMAN_ROOT>/src/api/config.rs:359:    if looks_like_openhuman_backend_endpoint(trimmed) {
<OPENHUMAN_ROOT>/src/api/config.rs:391:/// Returns `true` when the URL's host is one of the known OpenHuman backends.
<OPENHUMAN_ROOT>/src/api/config.rs:396:fn looks_like_openhuman_backend_endpoint(url: &str) -> bool {
<OPENHUMAN_ROOT>/src/api/config.rs:404:                "[api/config] parsed api_url for OpenHuman backend classification"
<OPENHUMAN_ROOT>/src/api/config.rs:412:                "[api/config] api_url parse failed during OpenHuman backend classification"
<OPENHUMAN_ROOT>/src/api/config.rs:421:            "[api/config] api_url has no host — not classified as OpenHuman backend"
<OPENHUMAN_ROOT>/src/api/config.rs:426:    let is_openhuman = matches!(
<OPENHUMAN_ROOT>/src/api/config.rs:434:        is_openhuman,
<OPENHUMAN_ROOT>/src/api/config.rs:435:        "[api/config] OpenHuman backend classification complete"
<OPENHUMAN_ROOT>/src/api/config.rs:438:    is_openhuman
<OPENHUMAN_ROOT>/src/api/config.rs:469:/// every backend call — Sentry `OPENHUMAN-TAURI-H6 / -HN`, issue #2075.
<OPENHUMAN_ROOT>/src/api/config.rs:631:        option_env!("OPENHUMAN_APP_ENV"),
<OPENHUMAN_ROOT>/src/api/config.rs:632:        option_env!("VITE_OPENHUMAN_APP_ENV"),
<OPENHUMAN_ROOT>/src/api/config.rs:920:        // Sentry OPENHUMAN-TAURI-H6 / issue #2075.
<OPENHUMAN_ROOT>/src/api/config.rs:1119:    // ── openhuman_backend detection ───────────────────────────────────────────
<OPENHUMAN_ROOT>/src/api/config.rs:1122:    fn openhuman_backend_detection_accepts_hosted_api_paths() {
<OPENHUMAN_ROOT>/src/api/config.rs:1123:        assert!(looks_like_openhuman_backend_endpoint(
<OPENHUMAN_ROOT>/src/api/config.rs:1126:        assert!(looks_like_openhuman_backend_endpoint(
<OPENHUMAN_ROOT>/src/api/config.rs:1129:        assert!(!looks_like_openhuman_backend_endpoint(
<OPENHUMAN_ROOT>/src/api/config.rs:1132:        assert!(!looks_like_openhuman_backend_endpoint(
<OPENHUMAN_ROOT>/src/api/config.rs:1204:        // Our own hosted backend still passes through (is_openhuman short-circuit),
<OPENHUMAN_ROOT>/src/api/config.rs:1208:        // non-`/v1` base (see the `my-openhuman.example.com` case) to keep routing.
<OPENHUMAN_ROOT>/src/api/config.rs:1212:            "openhuman backend host must pass through"
<OPENHUMAN_ROOT>/src/api/config.rs:1285:    fn inference_provider_excludes_openhuman_backend_and_plain_hosts() {
<OPENHUMAN_ROOT>/src/api/config.rs:1293:        // A custom self-hosted OpenHuman backend (no provider host, no `/v1`
<OPENHUMAN_ROOT>/src/api/config.rs:1296:            "https://my-openhuman.example.com/"
<OPENHUMAN_ROOT>/src/api/config.rs:1336:        // Regression: OPENHUMAN-TAURI-H6 / -HN, issue #2075.
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
<OPENHUMAN_ROOT>/src/main.rs:30:    //   1. `OPENHUMAN_CORE_SENTRY_DSN` at runtime (preferred, namespaced name)
<OPENHUMAN_ROOT>/src/main.rs:31:    //   2. `OPENHUMAN_SENTRY_DSN` at runtime (legacy unprefixed name — kept
<OPENHUMAN_ROOT>/src/main.rs:37:        dsn: std::env::var("OPENHUMAN_CORE_SENTRY_DSN")
<OPENHUMAN_ROOT>/src/main.rs:40:            .or_else(|| std::env::var("OPENHUMAN_SENTRY_DSN").ok())
<OPENHUMAN_ROOT>/src/main.rs:42:            .or_else(|| option_env!("OPENHUMAN_CORE_SENTRY_DSN").map(|s| s.to_string()))
<OPENHUMAN_ROOT>/src/main.rs:44:            .or_else(|| option_env!("OPENHUMAN_SENTRY_DSN").map(|s| s.to_string()))
<OPENHUMAN_ROOT>/src/main.rs:56:            // Sentry — see OPENHUMAN-TAURI-2E (~1393 events), -84 (~1050),
<OPENHUMAN_ROOT>/src/main.rs:58:            // `openhuman::inference::provider::ops::should_report_provider_http_failure`
<OPENHUMAN_ROOT>/src/main.rs:61:            if openhuman_core::core::observability::is_transient_provider_http_failure(&event) {
<OPENHUMAN_ROOT>/src/main.rs:64:            if openhuman_core::core::observability::is_all_transient_provider_exhaustion_event(
<OPENHUMAN_ROOT>/src/main.rs:75:            if openhuman_core::core::observability::is_backend_error_code_event(&event) {
<OPENHUMAN_ROOT>/src/main.rs:83:            if openhuman_core::core::observability::is_transient_provider_transport_failure(&event)
<OPENHUMAN_ROOT>/src/main.rs:91:            if openhuman_core::core::observability::is_budget_event(&event) {
<OPENHUMAN_ROOT>/src/main.rs:99:            if openhuman_core::core::observability::is_insufficient_credits_event(&event) {
<OPENHUMAN_ROOT>/src/main.rs:106:            if openhuman_core::core::observability::is_quota_exhausted_event(&event) {
<OPENHUMAN_ROOT>/src/main.rs:115:            if openhuman_core::core::observability::is_ollama_cloud_internal_500_event(&event) {
<OPENHUMAN_ROOT>/src/main.rs:125:            // surface for it (OPENHUMAN-TAURI-99 / -98).
<OPENHUMAN_ROOT>/src/main.rs:126:            if openhuman_core::core::observability::is_max_iterations_event(&event) {
<OPENHUMAN_ROOT>/src/main.rs:129:            if openhuman_core::core::observability::is_transient_backend_api_failure(&event)
<OPENHUMAN_ROOT>/src/main.rs:130:                || openhuman_core::core::observability::is_transient_integrations_failure(&event)
<OPENHUMAN_ROOT>/src/main.rs:131:                || openhuman_core::core::observability::is_updater_transient_event(&event)
<OPENHUMAN_ROOT>/src/main.rs:132:                || openhuman_core::core::observability::is_skill_install_user_fetch_failure(&event)
<OPENHUMAN_ROOT>/src/main.rs:142:            if openhuman_core::core::observability::is_skills_install_client_error_event(&event) {
<OPENHUMAN_ROOT>/src/main.rs:148:            // site that bypasses it. Targets OPENHUMAN-TAURI-R7 (28 events).
<OPENHUMAN_ROOT>/src/main.rs:149:            if openhuman_core::core::observability::is_channel_message_not_found_event(&event) {
<OPENHUMAN_ROOT>/src/main.rs:155:            // lives at the call sites (`openhuman::inference::provider::ops::api_error`
<OPENHUMAN_ROOT>/src/main.rs:160:            // shape — keeping OPENHUMAN-TAURI-25 / -1Q / -27 / -1G off
<OPENHUMAN_ROOT>/src/main.rs:163:            // `openhuman.auth_get_me` RPC. The primary fix in
<OPENHUMAN_ROOT>/src/main.rs:170:            if openhuman_core::core::observability::is_auth_get_me_opaque_transport_event(&event) {
<OPENHUMAN_ROOT>/src/main.rs:177:            if openhuman_core::core::observability::is_session_expired_event(&event) {
<OPENHUMAN_ROOT>/src/main.rs:207:                    openhuman_core::openhuman::app_state::peek_cached_current_user_identity()
<OPENHUMAN_ROOT>/src/main.rs:233:    if let Err(err) = openhuman_core::run_core_from_args(&args) {
<OPENHUMAN_ROOT>/src/main.rs:256:/// Canonical release tag: `openhuman@<version>[+<short_sha>]`.
<OPENHUMAN_ROOT>/src/main.rs:264:    let sha = option_env!("OPENHUMAN_BUILD_SHA").unwrap_or("").trim();
<OPENHUMAN_ROOT>/src/main.rs:267:        format!("openhuman@{version}")
<OPENHUMAN_ROOT>/src/main.rs:269:        format!("openhuman@{version}+{sha_short}")
<OPENHUMAN_ROOT>/src/main.rs:275:/// Honors `OPENHUMAN_APP_ENV` at runtime (`staging` / `production`) so the
<OPENHUMAN_ROOT>/src/main.rs:279:    if let Ok(value) = std::env::var("OPENHUMAN_APP_ENV") {
<OPENHUMAN_ROOT>/src/main.rs:297:/// `src/openhuman/memory/safety/mod.rs`.
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
<OPENHUMAN_ROOT>/src/bin/slack_backfill.rs:49:use openhuman_core::openhuman::composio::providers::{ProviderContext, SyncReason};
<OPENHUMAN_ROOT>/src/bin/slack_backfill.rs:50:use openhuman_core::openhuman::composio::types::{
<OPENHUMAN_ROOT>/src/bin/slack_backfill.rs:53:use openhuman_core::openhuman::config::Config;
<OPENHUMAN_ROOT>/src/bin/slack_backfill.rs:54:use openhuman_core::openhuman::memory;
<OPENHUMAN_ROOT>/src/bin/slack_backfill.rs:119:    /// 30 unless `OPENHUMAN_SLACK_BACKFILL_DAYS` overrides.
<OPENHUMAN_ROOT>/src/bin/slack_backfill.rs:161:    // `RUST_LOG` (e.g. `RUST_LOG=info,openhuman_core=debug`).
<OPENHUMAN_ROOT>/src/bin/slack_backfill.rs:214:        use openhuman_core::openhuman::memory::ingest_pipeline::ingest_chat;
<OPENHUMAN_ROOT>/src/bin/slack_backfill.rs:215:        use openhuman_core::openhuman::memory_sync::canonicalize::chat::{ChatBatch, ChatMessage};
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
<OPENHUMAN_ROOT>/CONTRIBUTING.md:1:# Contributing to OpenHuman
<OPENHUMAN_ROOT>/CONTRIBUTING.md:3:Thank you for your interest in contributing to OpenHuman. This guide is the fast path for getting a fresh checkout running locally, validating changes, and opening a pull request without having to piece together setup notes from multiple files.
<OPENHUMAN_ROOT>/CONTRIBUTING.md:28:- Check [open issues](https://github.com/tinyhumansai/openhuman/issues) and discussions before starting work.
<OPENHUMAN_ROOT>/CONTRIBUTING.md:103:- **Desktop development** needs the vendored Tauri/CEF setup. The preferred entrypoint is `pnpm --filter openhuman-app dev:app`, which ensures the vendored Tauri CLI is installed and configures `CEF_PATH`.
<OPENHUMAN_ROOT>/CONTRIBUTING.md:104:- **Linux desktop builds** require extra system packages beyond Node/Rust. Follow the distro-specific Tauri dependency list before running desktop commands, then use the OpenHuman scripts below. For deeper platform troubleshooting, see [`gitbooks/developing/getting-set-up.md`](gitbooks/developing/getting-set-up.md).
<OPENHUMAN_ROOT>/CONTRIBUTING.md:105:- **Windows 10 WSL + classic X11 forwarding** is unsupported for the desktop app. The Tauri/CEF stack can hang, render blank windows, or crash before useful app logs are available. Use native Windows development, or Windows 11 WSLg if you need a Linux GUI workflow. OpenHuman logs a startup warning when it detects WSL with `DISPLAY` set but no `WAYLAND_DISPLAY`/WSLg markers.
<OPENHUMAN_ROOT>/CONTRIBUTING.md:107:- **macOS desktop builds** require a one-time codesigning cert. After cloning, run `bash scripts/setup-dev-codesign.sh` once to create the local "OpenHuman Dev Signer" self-signed certificate that Tauri uses when bundling dev builds. Without it, `pnpm --filter openhuman-app dev:app` fails at the bundle/sign step with `OpenHuman Dev Signer: no identity found`.
<OPENHUMAN_ROOT>/CONTRIBUTING.md:108:- **Skills development** happens in the separate [`tinyhumansai/openhuman-skills`](https://github.com/tinyhumansai/openhuman-skills) repository. This repo consumes built skill bundles from GitHub or a local override path; it does not vendor the skills source as a submodule.
<OPENHUMAN_ROOT>/CONTRIBUTING.md:125:git clone git@github.com:YOUR_USERNAME/openhuman.git
<OPENHUMAN_ROOT>/CONTRIBUTING.md:126:cd openhuman
<OPENHUMAN_ROOT>/CONTRIBUTING.md:127:git remote add upstream git@github.com:tinyhumansai/openhuman.git
<OPENHUMAN_ROOT>/CONTRIBUTING.md:141:OpenHuman uses two environment templates:
<OPENHUMAN_ROOT>/CONTRIBUTING.md:156:- **Desktop work**: leave `OPENHUMAN_CORE_TOKEN` blank for local child-mode development unless you are intentionally wiring an external core. The shell manages the embedded core token flow.
<OPENHUMAN_ROOT>/CONTRIBUTING.md:157:- **Core RPC / standalone core work**: `OPENHUMAN_CORE_PORT=7788` and `OPENHUMAN_CORE_RPC_URL=http://127.0.0.1:7788/rpc` are already documented in the root template and are the normal local defaults.
<OPENHUMAN_ROOT>/CONTRIBUTING.md:174:pnpm --filter openhuman-app dev:app
<OPENHUMAN_ROOT>/CONTRIBUTING.md:180:cargo run --manifest-path Cargo.toml --bin openhuman-core
<OPENHUMAN_ROOT>/CONTRIBUTING.md:186:- `pnpm --filter openhuman-app dev:app`: full desktop app flow with Tauri + CEF.
<OPENHUMAN_ROOT>/CONTRIBUTING.md:187:- `cargo run --bin openhuman-core`: core/RPC work when you want the Rust server without the desktop shell.
<OPENHUMAN_ROOT>/CONTRIBUTING.md:226:- `~/.openhuman/`: default workspace for the Rust core and local app data.
<OPENHUMAN_ROOT>/CONTRIBUTING.md:227:- `~/.openhuman-staging/`: staging workspace when `OPENHUMAN_APP_ENV=staging`.
<OPENHUMAN_ROOT>/CONTRIBUTING.md:240:openhuman/
<OPENHUMAN_ROOT>/CONTRIBUTING.md:245:├── src/                    # Rust core crate and openhuman-core binary
<OPENHUMAN_ROOT>/CONTRIBUTING.md:261:- Fork [tinyhumansai/openhuman](https://github.com/tinyhumansai/openhuman) and push branches to your fork.
<OPENHUMAN_ROOT>/CONTRIBUTING.md:298:2. Open a pull request against `tinyhumansai/openhuman:main`.
<OPENHUMAN_ROOT>/CONTRIBUTING.md:313:Thank you for contributing to OpenHuman.
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
<OPENHUMAN_ROOT>/CONTRIBUTING-BEGINNERS.md:393:1. Go to your fork on GitHub: `github.com/YOUR_USERNAME/openhuman`
<OPENHUMAN_ROOT>/CONTRIBUTING-BEGINNERS.md:395:3. Make sure the PR targets **`tinyhumansai/openhuman:main`** (not your fork)
<OPENHUMAN_ROOT>/CONTRIBUTING-BEGINNERS.md:407:I want to make my first contribution to OpenHuman. First read these upstream docs:
<OPENHUMAN_ROOT>/CONTRIBUTING-BEGINNERS.md:409:CONTRIBUTING.md: https://raw.githubusercontent.com/tinyhumansai/openhuman/main/CONTRIBUTING.md
<OPENHUMAN_ROOT>/CONTRIBUTING-BEGINNERS.md:410:AGENTS.md: https://raw.githubusercontent.com/tinyhumansai/openhuman/main/AGENTS.md
<OPENHUMAN_ROOT>/CONTRIBUTING-BEGINNERS.md:411:CLAUDE.md: https://raw.githubusercontent.com/tinyhumansai/openhuman/main/CLAUDE.md
<OPENHUMAN_ROOT>/CONTRIBUTING-BEGINNERS.md:470:### Desktop build fails (`pnpm --filter openhuman-app dev:app`)
<OPENHUMAN_ROOT>/CONTRIBUTING-BEGINNERS.md:482:Thank you for contributing to OpenHuman!
<OPENHUMAN_ROOT>/SECURITY.md:5:We provide security updates for the following versions of OpenHuman:
<OPENHUMAN_ROOT>/SECURITY.md:13:We recommend always running the [latest release](https://github.com/tinyhumansai/openhuman/releases/latest). OpenHuman is in early beta; older versions may not receive patches.
<OPENHUMAN_ROOT>/SECURITY.md:22:2. Email the maintainers with a clear description of the issue, steps to reproduce, and impact. You can reach us via the contact details listed in the [OpenHuman organization](https://github.com/openhumanxyz) or repository.
<OPENHUMAN_ROOT>/SECURITY.md:41:Out-of-scope for this process: general bugs, feature requests, and issues in third-party services we integrate with (e.g., Telegram, Notion) unless they are specific to how OpenHuman uses them.
<OPENHUMAN_ROOT>/SECURITY.md:53:Thank you for helping keep OpenHuman and its users safe.
<OPENHUMAN_ROOT>/pnpm-workspace.yaml:5:  # a `postinstall` that downloads a pre-built openhuman binary from a GitHub
<OPENHUMAN_ROOT>/src/core/jsonrpc_tests.rs:21:        let lock = crate::openhuman::config::TEST_ENV_LOCK
<OPENHUMAN_ROOT>/src/core/jsonrpc_tests.rs:101:/// To run manually: `cargo test --lib -p openhuman -- --ignored
<OPENHUMAN_ROOT>/src/core/jsonrpc_tests.rs:106:    let _signed_out_restore = crate::openhuman::scheduler_gate::SignedOutTestGuard::set(false);
<OPENHUMAN_ROOT>/src/core/jsonrpc_tests.rs:120:            "OPENHUMAN_WORKSPACE",
<OPENHUMAN_ROOT>/src/core/jsonrpc_tests.rs:123:        ("OPENHUMAN_DISABLE_CHANNEL_LISTENERS", OsString::from("1")),
<OPENHUMAN_ROOT>/src/core/jsonrpc_tests.rs:125:            "OPENHUMAN_CORE_TOKEN",
<OPENHUMAN_ROOT>/src/core/jsonrpc_tests.rs:153:    let result = invoke_method(default_state(), "openhuman.health_snapshot", json!({}))
<OPENHUMAN_ROOT>/src/core/jsonrpc_tests.rs:161:    let err = invoke_method(default_state(), "openhuman.encrypt_secret", json!({}))
<OPENHUMAN_ROOT>/src/core/jsonrpc_tests.rs:171:        "openhuman.doctor_models",
<OPENHUMAN_ROOT>/src/core/jsonrpc_tests.rs:183:        "openhuman.config_get_runtime_flags",
<OPENHUMAN_ROOT>/src/core/jsonrpc_tests.rs:195:        "openhuman.autocomplete_status",
<OPENHUMAN_ROOT>/src/core/jsonrpc_tests.rs:205:    let err = invoke_method(default_state(), "openhuman.auth_store_session", json!({}))
<OPENHUMAN_ROOT>/src/core/jsonrpc_tests.rs:215:        "openhuman.service_status",
<OPENHUMAN_ROOT>/src/core/jsonrpc_tests.rs:228:    let result = invoke_method(default_state(), "openhuman.memory_init", json!({})).await;
<OPENHUMAN_ROOT>/src/core/jsonrpc_tests.rs:241:        "openhuman.memory_list_namespaces",
<OPENHUMAN_ROOT>/src/core/jsonrpc_tests.rs:253:        "openhuman.memory_query_namespace",
<OPENHUMAN_ROOT>/src/core/jsonrpc_tests.rs:265:        "openhuman.memory_recall_memories",
<OPENHUMAN_ROOT>/src/core/jsonrpc_tests.rs:277:        "openhuman.migrate_openclaw",
<OPENHUMAN_ROOT>/src/core/jsonrpc_tests.rs:289:        "openhuman.migrate_hermes",
<OPENHUMAN_ROOT>/src/core/jsonrpc_tests.rs:298:fn http_schema_dump_includes_openhuman_and_core_methods() {
<OPENHUMAN_ROOT>/src/core/jsonrpc_tests.rs:311:            .any(|m| m.method == "openhuman.health_snapshot"),
<OPENHUMAN_ROOT>/src/core/jsonrpc_tests.rs:312:        "schema dump should include migrated openhuman methods"
<OPENHUMAN_ROOT>/src/core/jsonrpc_tests.rs:318:            .any(|m| m.method == "openhuman.billing_get_current_plan"),
<OPENHUMAN_ROOT>/src/core/jsonrpc_tests.rs:325:            .any(|m| m.method == "openhuman.team_list_members"),
<OPENHUMAN_ROOT>/src/core/jsonrpc_tests.rs:334:        "openhuman.billing_get_current_plan",
<OPENHUMAN_ROOT>/src/core/jsonrpc_tests.rs:346:        "openhuman.billing_purchase_plan",
<OPENHUMAN_ROOT>/src/core/jsonrpc_tests.rs:356:    let err = invoke_method(default_state(), "openhuman.billing_top_up", json!({}))
<OPENHUMAN_ROOT>/src/core/jsonrpc_tests.rs:366:        "openhuman.billing_top_up",
<OPENHUMAN_ROOT>/src/core/jsonrpc_tests.rs:378:        "openhuman.billing_create_portal_session",
<OPENHUMAN_ROOT>/src/core/jsonrpc_tests.rs:388:    let err = invoke_method(default_state(), "openhuman.team_list_members", json!({}))
<OPENHUMAN_ROOT>/src/core/jsonrpc_tests.rs:398:        "openhuman.team_list_members",
<OPENHUMAN_ROOT>/src/core/jsonrpc_tests.rs:408:    let err = invoke_method(default_state(), "openhuman.team_create_invite", json!({}))
<OPENHUMAN_ROOT>/src/core/jsonrpc_tests.rs:418:        "openhuman.team_remove_member",
<OPENHUMAN_ROOT>/src/core/jsonrpc_tests.rs:430:        "openhuman.team_change_member_role",
<OPENHUMAN_ROOT>/src/core/jsonrpc_tests.rs:442:        "openhuman.billing_create_coinbase_charge",
<OPENHUMAN_ROOT>/src/core/jsonrpc_tests.rs:454:        "openhuman.billing_create_coinbase_charge",
<OPENHUMAN_ROOT>/src/core/jsonrpc_tests.rs:464:    let err = invoke_method(default_state(), "openhuman.team_list_invites", json!({}))
<OPENHUMAN_ROOT>/src/core/jsonrpc_tests.rs:474:        "openhuman.team_list_invites",
<OPENHUMAN_ROOT>/src/core/jsonrpc_tests.rs:484:    let err = invoke_method(default_state(), "openhuman.team_revoke_invite", json!({}))
<OPENHUMAN_ROOT>/src/core/jsonrpc_tests.rs:494:        "openhuman.team_revoke_invite",
<OPENHUMAN_ROOT>/src/core/jsonrpc_tests.rs:507:        "openhuman.billing_get_current_plan",
<OPENHUMAN_ROOT>/src/core/jsonrpc_tests.rs:508:        "openhuman.billing_purchase_plan",
<OPENHUMAN_ROOT>/src/core/jsonrpc_tests.rs:509:        "openhuman.billing_create_portal_session",
<OPENHUMAN_ROOT>/src/core/jsonrpc_tests.rs:510:        "openhuman.billing_top_up",
<OPENHUMAN_ROOT>/src/core/jsonrpc_tests.rs:511:        "openhuman.billing_create_coinbase_charge",
<OPENHUMAN_ROOT>/src/core/jsonrpc_tests.rs:512:        "openhuman.team_list_members",
<OPENHUMAN_ROOT>/src/core/jsonrpc_tests.rs:513:        "openhuman.team_create_invite",
<OPENHUMAN_ROOT>/src/core/jsonrpc_tests.rs:514:        "openhuman.team_list_invites",
<OPENHUMAN_ROOT>/src/core/jsonrpc_tests.rs:515:        "openhuman.team_revoke_invite",
<OPENHUMAN_ROOT>/src/core/jsonrpc_tests.rs:516:        "openhuman.team_remove_member",
<OPENHUMAN_ROOT>/src/core/jsonrpc_tests.rs:517:        "openhuman.team_change_member_role",
<OPENHUMAN_ROOT>/src/core/jsonrpc_tests.rs:581:    // Issue #2286: only OpenHuman backend path 401s (HTTP-method prefix) should
<OPENHUMAN_ROOT>/src/core/jsonrpc_tests.rs:647:fn is_session_expired_error_matches_openhuman_backend_path_401() {
<OPENHUMAN_ROOT>/src/core/jsonrpc_tests.rs:648:    // OpenHuman backend calls via authed_json use the format:
<OPENHUMAN_ROOT>/src/core/jsonrpc_tests.rs:696:    // that the user's OpenHuman app session expired.
<OPENHUMAN_ROOT>/src/core/jsonrpc_tests.rs:723:fn is_session_expired_error_matches_openhuman_session_expired_body() {
<OPENHUMAN_ROOT>/src/core/jsonrpc_tests.rs:727:        r#"OpenHuman API error (401 Unauthorized): {"success":false,"error":"Session expired. Please log in again."}"#
<OPENHUMAN_ROOT>/src/core/jsonrpc_tests.rs:752:    // card click would once again log the user out of OpenHuman.
<OPENHUMAN_ROOT>/src/core/jsonrpc_tests.rs:777:    // Regression guard for OPENHUMAN-TAURI-20: pre-#1467 cores rejected
<OPENHUMAN_ROOT>/src/core/jsonrpc_tests.rs:843:        "OPENHUMAN_WORKSPACE",
<OPENHUMAN_ROOT>/src/core/jsonrpc_tests.rs:850:        method: "openhuman.threads_generate_title".to_string(),
<OPENHUMAN_ROOT>/src/core/jsonrpc_tests.rs:864:        !message.contains("__OPENHUMAN_STRUCTURED_RPC_ERROR_V1__"),
<OPENHUMAN_ROOT>/src/core/jsonrpc_tests.rs:881:        "OPENHUMAN_WORKSPACE",
<OPENHUMAN_ROOT>/src/core/jsonrpc_tests.rs:918:        method: "openhuman.threads_message_append".to_string(),
<OPENHUMAN_ROOT>/src/core/jsonrpc_tests.rs:998:        "OPENHUMAN_WORKSPACE",
<OPENHUMAN_ROOT>/src/core/jsonrpc_tests.rs:1233:        "openhuman.health_snapshot",
<OPENHUMAN_ROOT>/src/core/jsonrpc_tests.rs:1244:    let err = invoke_method(default_state(), "openhuman.health_snapshot", json!("oops"))
<OPENHUMAN_ROOT>/src/core/jsonrpc_tests.rs:1254:    let result = invoke_method(default_state(), "openhuman.health_snapshot", json!(null)).await;
<OPENHUMAN_ROOT>/src/core/jsonrpc_tests.rs:1267:    let err = invoke_method(default_state(), "openhuman.totally_made_up_xyz", json!({}))
<OPENHUMAN_ROOT>/src/core/jsonrpc_tests.rs:1416:        crate::openhuman::wallet::WALLET_NOT_CONFIGURED_MESSAGE
<OPENHUMAN_ROOT>/src/core/jsonrpc_tests.rs:1426:        crate::openhuman::wallet::WALLET_NOT_CONFIGURED_MESSAGE,
<OPENHUMAN_ROOT>/Cargo.toml:2:name = "openhuman"
<OPENHUMAN_ROOT>/Cargo.toml:5:description = "OpenHuman core business logic and RPC server"
<OPENHUMAN_ROOT>/Cargo.toml:9:name = "openhuman-core"
<OPENHUMAN_ROOT>/Cargo.toml:37:name = "openhuman_core"
<OPENHUMAN_ROOT>/Cargo.toml:42:# to the vendored tiny.place submodule so OpenHuman can test SDK changes before
<OPENHUMAN_ROOT>/Cargo.toml:47:# `src/openhuman/tinyflows/` + the `flows::` domain. Pulls tinyagents 1.7 transitively
<OPENHUMAN_ROOT>/Cargo.toml:48:# (same version openhuman already uses — no conflict). Published on crates.io
<OPENHUMAN_ROOT>/Cargo.toml:51:# TinyJuice — host-agnostic TokenJuice compression engine. OpenHuman keeps
<OPENHUMAN_ROOT>/Cargo.toml:52:# config/RPC/tool/runtime adapters in `src/openhuman/tokenjuice/` and patches
<OPENHUMAN_ROOT>/Cargo.toml:57:# `.rag` workflow language. openhuman's agent engine + orchestration run on this
<OPENHUMAN_ROOT>/Cargo.toml:58:# crate's primitives via the adapter seam in `src/openhuman/tinyagents/` (issue
<OPENHUMAN_ROOT>/Cargo.toml:61:# We wire openhuman's own Provider/Tool, not the removed bundled openai client.
<OPENHUMAN_ROOT>/Cargo.toml:62:# The `sqlite` feature is enabled now that openhuman's direct rusqlite pin is
<OPENHUMAN_ROOT>/Cargo.toml:67:# the `rlm` language-workflow tool (`src/openhuman/rlm/`).
<OPENHUMAN_ROOT>/Cargo.toml:71:# below to `vendor/tinycortex`. OpenHuman's memory subsystem migrates onto this
<OPENHUMAN_ROOT>/Cargo.toml:72:# crate through the adapter seam in `src/openhuman/tinycortex/` (mirroring the
<OPENHUMAN_ROOT>/Cargo.toml:81:# the language-agnostic brace-depth heuristic. See src/openhuman/tokenjuice/compressors/code.rs.
<OPENHUMAN_ROOT>/Cargo.toml:181:# optimization); declared directly here so `openhuman` owns the
<OPENHUMAN_ROOT>/Cargo.toml:226:# only by `openhuman::scheduler_gate::signals` to decide when to throttle
<OPENHUMAN_ROOT>/Cargo.toml:266:# AppContainer / process-jail backend in `openhuman::cwd_jail`.
<OPENHUMAN_ROOT>/Cargo.toml:339:# Exposes the destructive `openhuman.test_reset` RPC. Off by default; the E2E
<OPENHUMAN_ROOT>/Cargo.toml:350:# See: https://github.com/tinyhumansai/openhuman/issues/273
<OPENHUMAN_ROOT>/Cargo.toml:354:# migration work can change the SDK source in-tree, test it against OpenHuman
<OPENHUMAN_ROOT>/Cargo.toml:360:# TinyAgents so integration work can test crate changes against OpenHuman before
<OPENHUMAN_ROOT>/Cargo.toml:397:#     debuginfo, so panics/backtraces in OpenHuman code still resolve to
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
<OPENHUMAN_ROOT>/package.json:26:    "lint:fix": "pnpm --filter openhuman-app lint:fix",
<OPENHUMAN_ROOT>/package.json:29:    "tauri": "pnpm --filter openhuman-app tauri",
<OPENHUMAN_ROOT>/package.json:30:    "test": "pnpm --filter openhuman-app test",
<OPENHUMAN_ROOT>/package.json:31:    "test:coverage": "pnpm --filter openhuman-app test:coverage",
<OPENHUMAN_ROOT>/package.json:32:    "test:rust": "pnpm --filter openhuman-app test:rust",
<OPENHUMAN_ROOT>/package.json:36:    "test:e2e": "pnpm --filter openhuman-app test:e2e:all",
<OPENHUMAN_ROOT>/package.json:37:    "test:e2e:flows": "pnpm --filter openhuman-app test:e2e:all:flows",
<OPENHUMAN_ROOT>/package.json:54:    "test:install-ps1": "pwsh -NoProfile -File scripts/tests/OpenHumanWindowsInstall.Tests.ps1",
<OPENHUMAN_ROOT>/package.json:55:    "rust:check": "pnpm --filter openhuman-app rust:check",
<OPENHUMAN_ROOT>/package.json:56:    "typecheck": "pnpm --filter openhuman-app compile",
<OPENHUMAN_ROOT>/package.json:58:    "tauri:ios:dev": "pnpm --filter openhuman-app tauri:ios:dev",
<OPENHUMAN_ROOT>/package.json:59:    "tauri:ios:build": "pnpm --filter openhuman-app tauri:ios:build",
<OPENHUMAN_ROOT>/package.json:61:    "tauri:android:dev": "pnpm --filter openhuman-app tauri:android:dev",
<OPENHUMAN_ROOT>/package.json:62:    "tauri:android:build": "pnpm --filter openhuman-app tauri:android:build",
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
<OPENHUMAN_ROOT>/docker-compose.yml:48:      - openhuman-workspace:/home/openhuman/.openhuman
<OPENHUMAN_ROOT>/docker-compose.yml:49:    mem_limit: ${OPENHUMAN_CORE_MEM_LIMIT:-4g}
<OPENHUMAN_ROOT>/docker-compose.yml:50:    cpus: ${OPENHUMAN_CORE_CPUS:-2.0}
<OPENHUMAN_ROOT>/docker-compose.yml:59:  openhuman-workspace:
<OPENHUMAN_ROOT>/docker-compose.yml:60:    name: openhuman-workspace
<OPENHUMAN_ROOT>/e2e/docker-compose.yml:6:#   - same image: ghcr.io/tinyhumansai/openhuman_ci:latest
<OPENHUMAN_ROOT>/e2e/docker-compose.yml:14:#     bash -lc "pnpm install --frozen-lockfile && pnpm --filter openhuman-app test:e2e:build"
<OPENHUMAN_ROOT>/e2e/docker-compose.yml:33:    image: ${OPENHUMAN_CI_IMAGE:-ghcr.io/tinyhumansai/openhuman_ci:latest}
<OPENHUMAN_ROOT>/e2e/docker-compose.yml:88:    image: ${OPENHUMAN_CI_IMAGE:-ghcr.io/tinyhumansai/openhuman_ci:latest}
<OPENHUMAN_ROOT>/src/core/types.rs:86:    /// The name of the method to be invoked (e.g., `openhuman.memory_doc_put`).
<OPENHUMAN_ROOT>/src/core/types.rs:140:    /// The current version of the OpenHuman core binary, usually from `CARGO_PKG_VERSION`.
<OPENHUMAN_ROOT>/src/core/types.rs:154:/// - [`HostKind::Cli`] — `openhuman-core` standalone binary spawned by an
<OPENHUMAN_ROOT>/src/core/types.rs:163:/// `OPENHUMAN_DOCKER=1`). Tauri-shell callers MUST pass `TauriShell`
<OPENHUMAN_ROOT>/src/core/types.rs:179:        if std::env::var("OPENHUMAN_DOCKER")
<OPENHUMAN_ROOT>/src/core/types.rs:194:    /// gate to decide whether the `OPENHUMAN_APPROVAL_GATE=0` env override
<OPENHUMAN_ROOT>/src/core/types.rs:213:/// Takes the host kind and whether an `OPENHUMAN_APPROVAL_GATE=0` env
<OPENHUMAN_ROOT>/src/core/types.rs:388:        // Operator sets OPENHUMAN_APPROVAL_GATE=0 inside a Tauri-shell
<OPENHUMAN_ROOT>/src/core/auth.rs:16://!    [`init_rpc_token`] reads `OPENHUMAN_CORE_TOKEN` from the environment.
<OPENHUMAN_ROOT>/src/core/auth.rs:56://!   `openhuman::inference::http::EXTERNAL_OPENAI_COMPAT_PROVIDER`.
<OPENHUMAN_ROOT>/src/core/auth.rs:71:use crate::openhuman::config::Config;
<OPENHUMAN_ROOT>/src/core/auth.rs:72:use crate::openhuman::credentials::AuthService;
<OPENHUMAN_ROOT>/src/core/auth.rs:73:use crate::openhuman::inference::http::EXTERNAL_OPENAI_COMPAT_PROVIDER;
<OPENHUMAN_ROOT>/src/core/auth.rs:99:    // AgentBox marketplace surface — see `openhuman::agentbox::http`.
<OPENHUMAN_ROOT>/src/core/auth.rs:100:    // Mounted only when `OPENHUMAN_AGENTBOX_MODE=1`; the public-path entry is
<OPENHUMAN_ROOT>/src/core/auth.rs:149:/// `OPENHUMAN_CORE_TOKEN` remains the canonical configuration surface for
<OPENHUMAN_ROOT>/src/core/auth.rs:152:/// file, or a developer running `openhuman-core serve` from a shell with the
<OPENHUMAN_ROOT>/src/core/auth.rs:160:pub const CORE_TOKEN_ENV_VAR: &str = "OPENHUMAN_CORE_TOKEN";
<OPENHUMAN_ROOT>/src/core/auth.rs:167:/// `OPENHUMAN_CORE_TOKEN`. This function is the bootstrap path for
<OPENHUMAN_ROOT>/src/core/auth.rs:171:/// `OPENHUMAN_CORE_TOKEN` is set in the process environment (typically by
<OPENHUMAN_ROOT>/src/core/auth.rs:196:    // OPENHUMAN_CORE_TOKEN. Used by Docker / cloud / systemd / a developer
<OPENHUMAN_ROOT>/src/core/auth.rs:197:    // running `openhuman-core serve` from a pre-configured shell. Desktop
<OPENHUMAN_ROOT>/src/core/auth.rs:225:/// without round-tripping through `OPENHUMAN_CORE_TOKEN` in the process
<OPENHUMAN_ROOT>/src/core/auth.rs:605:        // by `OPENHUMAN_AGENTBOX_MODE` at router-build time).
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
<OPENHUMAN_ROOT>/plan.md:91:| ✅ | `src/openhuman/agent/prompts/mod_tests.rs::grounding_contract_requires_exact_numeric_evidence` | Pins 5 verbatim prose substrings of the grounding contract — breaks on any copywriting pass. | Behavioral guarantee ("contract appended on every build path") already covered by the marker-based test; convert this to a single explicitly-labeled wording-lock, or assert stable structural markers. |
<OPENHUMAN_ROOT>/plan.md:92:| ✅ | `src/openhuman/agent/prompts/mod_tests.rs::identity_section_creates_missing_workspace_files` | Also string-matches SOUL.md brand-voice prose (`"Don't validate FUD"`). | Split: (a) files created + seeded from the checked-in template (compare against template file content); (b) a narrow, labeled brand-voice lock if the phrase must stay pinned. |
<OPENHUMAN_ROOT>/plan.md:94:| ✅ | `src/openhuman/hooks/../useDaemonLifecycle.test.ts` (`app/src/hooks/__tests__/`) | Pins exact `console.log` strings as an effect-rerun proxy. Listener-count assertions are legit — keep them. | Drop the log-text pinning; keep listener/startDaemon observable assertions. |
<OPENHUMAN_ROOT>/plan.md:95:| ✅ | `src/openhuman/provider_surfaces/schemas.rs::all_schemas_returns_two` / `all_controllers_returns_two` | Magic-number count breaks on any legitimate 3rd controller. | Replace with `schemas().len() == controllers().len()` parity + presence of a known op (`list_queue`). Standardize this as a shared `assert_schema_controller_parity()` helper — the `== N` pattern repeats across ~15 domains. |
<OPENHUMAN_ROOT>/plan.md:201:   Windows-install test (`OpenHumanWindowsInstall.Tests.ps1`). The mock server's socket-auth
<OPENHUMAN_ROOT>/Dockerfile:2:# OpenHuman Core — multi-stage Docker build
<OPENHUMAN_ROOT>/Dockerfile:3:# Produces a minimal image running the `openhuman-core` binary (JSON-RPC server).
<OPENHUMAN_ROOT>/Dockerfile:5:# Build:   docker build -t openhuman-core .
<OPENHUMAN_ROOT>/Dockerfile:6:# Run:     docker run -p 7788:7788 --env-file .env openhuman-core
<OPENHUMAN_ROOT>/Dockerfile:57:    cargo build --profile "${CARGO_PROFILE}" --bin openhuman-core 2>/dev/null || true && \
<OPENHUMAN_ROOT>/Dockerfile:64:    cargo build --profile "${CARGO_PROFILE}" --bin openhuman-core && \
<OPENHUMAN_ROOT>/Dockerfile:65:    cp "target/${CARGO_PROFILE}/openhuman-core" /tmp/openhuman-core
<OPENHUMAN_ROOT>/Dockerfile:88:RUN groupadd --gid 10001 openhuman \
<OPENHUMAN_ROOT>/Dockerfile:89: && useradd --uid 10001 --gid 10001 --create-home --shell /bin/bash openhuman
<OPENHUMAN_ROOT>/Dockerfile:94:ENV HOME=/home/openhuman
<OPENHUMAN_ROOT>/Dockerfile:95:RUN mkdir -p /home/openhuman/.openhuman \
<OPENHUMAN_ROOT>/Dockerfile:96: && chown -R openhuman:openhuman /home/openhuman
<OPENHUMAN_ROOT>/Dockerfile:99:COPY --from=builder /tmp/openhuman-core /usr/local/bin/openhuman-core
<OPENHUMAN_ROOT>/Dockerfile:112:# gosu to drop to the openhuman user before starting the binary.
<OPENHUMAN_ROOT>/Dockerfile:116:ENV OPENHUMAN_WORKSPACE=/home/openhuman/.openhuman
<OPENHUMAN_ROOT>/Dockerfile:118:ENV OPENHUMAN_CORE_HOST=0.0.0.0
<OPENHUMAN_ROOT>/Dockerfile:119:ENV OPENHUMAN_CORE_PORT=7788
<OPENHUMAN_ROOT>/Dockerfile:123:ENV OPENHUMAN_AGENTBOX_MODE=0
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
<OPENHUMAN_ROOT>/fastlane/metadata/en-US/description.txt:25:- OpenHuman desktop app
<OPENHUMAN_ROOT>/fastlane/metadata/en-US/description.txt:29:OpenHuman is in active development. Feedback is welcome through the OpenHuman support and community channels.
<OPENHUMAN_ROOT>/e2e/docker-entrypoint.sh:46:# Run the provided command (default: yarn workspace openhuman-app test:e2e:all)
<OPENHUMAN_ROOT>/src/core/all.rs:1://! Registry and dispatch logic for all OpenHuman controllers.
<OPENHUMAN_ROOT>/src/core/all.rs:43:    /// Returns the canonical RPC method name for this controller (e.g., `openhuman.memory_doc_put`).
<OPENHUMAN_ROOT>/src/core/all.rs:92:            handler: crate::openhuman::voice::cli::run_standalone_subcommand,
<OPENHUMAN_ROOT>/src/core/all.rs:108:    controllers.extend(crate::openhuman::about_app::all_about_app_registered_controllers());
<OPENHUMAN_ROOT>/src/core/all.rs:110:    controllers.extend(crate::openhuman::agentbox::all_agentbox_registered_controllers());
<OPENHUMAN_ROOT>/src/core/all.rs:112:    controllers.extend(crate::openhuman::app_state::all_app_state_registered_controllers());
<OPENHUMAN_ROOT>/src/core/all.rs:114:    controllers.extend(crate::openhuman::audio_toolkit::all_audio_toolkit_registered_controllers());
<OPENHUMAN_ROOT>/src/core/all.rs:116:    controllers.extend(crate::openhuman::composio::all_composio_registered_controllers());
<OPENHUMAN_ROOT>/src/core/all.rs:119:        .extend(crate::openhuman::recall_calendar::all_recall_calendar_registered_controllers());
<OPENHUMAN_ROOT>/src/core/all.rs:121:    controllers.extend(crate::openhuman::cron::all_cron_registered_controllers());
<OPENHUMAN_ROOT>/src/core/all.rs:123:    controllers.extend(crate::openhuman::flows::all_flows_registered_controllers());
<OPENHUMAN_ROOT>/src/core/all.rs:125:    controllers.extend(crate::openhuman::task_sources::all_task_sources_registered_controllers());
<OPENHUMAN_ROOT>/src/core/all.rs:126:    controllers.extend(crate::openhuman::dashboard::all_dashboard_registered_controllers());
<OPENHUMAN_ROOT>/src/core/all.rs:128:    controllers.extend(crate::openhuman::mcp_registry::all_mcp_registry_registered_controllers());
<OPENHUMAN_ROOT>/src/core/all.rs:131:    controllers.extend(crate::openhuman::webview_apis::all_webview_apis_registered_controllers());
<OPENHUMAN_ROOT>/src/core/all.rs:133:    controllers.extend(crate::openhuman::agent::all_agent_registered_controllers());
<OPENHUMAN_ROOT>/src/core/all.rs:137:        .extend(crate::openhuman::tinyagents::replay::all_agent_replay_registered_controllers());
<OPENHUMAN_ROOT>/src/core/all.rs:139:    controllers.extend(crate::openhuman::profiles::all_profiles_registered_controllers());
<OPENHUMAN_ROOT>/src/core/all.rs:142:        .extend(crate::openhuman::agent_registry::all_agent_registry_registered_controllers());
<OPENHUMAN_ROOT>/src/core/all.rs:145:        .extend(crate::openhuman::agent_experience::all_agent_experience_registered_controllers());
<OPENHUMAN_ROOT>/src/core/all.rs:147:    controllers.extend(crate::openhuman::health::all_health_registered_controllers());
<OPENHUMAN_ROOT>/src/core/all.rs:149:    controllers.extend(crate::openhuman::harness_init::all_harness_init_registered_controllers());
<OPENHUMAN_ROOT>/src/core/all.rs:151:    controllers.extend(crate::openhuman::doctor::all_doctor_registered_controllers());
<OPENHUMAN_ROOT>/src/core/all.rs:153:    controllers.extend(crate::openhuman::encryption::all_encryption_registered_controllers());
<OPENHUMAN_ROOT>/src/core/all.rs:156:        .extend(crate::openhuman::keyring_consent::all_keyring_consent_registered_controllers());
<OPENHUMAN_ROOT>/src/core/all.rs:158:    controllers.extend(crate::openhuman::security::all_security_registered_controllers());
<OPENHUMAN_ROOT>/src/core/all.rs:160:    controllers.extend(crate::openhuman::approval::all_approval_registered_controllers());
<OPENHUMAN_ROOT>/src/core/all.rs:162:    controllers.extend(crate::openhuman::plan_review::all_plan_review_registered_controllers());
<OPENHUMAN_ROOT>/src/core/all.rs:164:    controllers.extend(crate::openhuman::artifacts::all_artifacts_registered_controllers());
<OPENHUMAN_ROOT>/src/core/all.rs:166:    controllers.extend(crate::openhuman::heartbeat::all_heartbeat_registered_controllers());
<OPENHUMAN_ROOT>/src/core/all.rs:168:    controllers.extend(crate::openhuman::http_host::all_http_host_registered_controllers());
<OPENHUMAN_ROOT>/src/core/all.rs:170:    controllers.extend(crate::openhuman::cost::all_cost_registered_controllers());
<OPENHUMAN_ROOT>/src/core/all.rs:172:    controllers.extend(crate::openhuman::x402::all_x402_registered_controllers());
<OPENHUMAN_ROOT>/src/core/all.rs:174:    controllers.extend(crate::openhuman::autocomplete::all_autocomplete_registered_controllers());
<OPENHUMAN_ROOT>/src/core/all.rs:177:        crate::openhuman::channels::providers::web::all_web_channel_registered_controllers(),
<OPENHUMAN_ROOT>/src/core/all.rs:180:        .extend(crate::openhuman::channels::controllers::all_channels_registered_controllers());
<OPENHUMAN_ROOT>/src/core/all.rs:182:    controllers.extend(crate::openhuman::config::all_config_registered_controllers());
<OPENHUMAN_ROOT>/src/core/all.rs:184:    controllers.extend(crate::openhuman::connectivity::all_connectivity_registered_controllers());
<OPENHUMAN_ROOT>/src/core/all.rs:186:    controllers.extend(crate::openhuman::credentials::all_credentials_registered_controllers());
<OPENHUMAN_ROOT>/src/core/all.rs:188:    controllers.extend(crate::openhuman::service::all_service_registered_controllers());
<OPENHUMAN_ROOT>/src/core/all.rs:190:    controllers.extend(crate::openhuman::migration::all_migration_registered_controllers());
<OPENHUMAN_ROOT>/src/core/all.rs:193:        .extend(crate::openhuman::council_registry::all_council_registry_registered_controllers());
<OPENHUMAN_ROOT>/src/core/all.rs:195:    controllers.extend(crate::openhuman::model_council::all_model_council_registered_controllers());
<OPENHUMAN_ROOT>/src/core/all.rs:197:    controllers.extend(crate::openhuman::monitor::all_monitor_registered_controllers());
<OPENHUMAN_ROOT>/src/core/all.rs:200:    controllers.extend(crate::openhuman::inference::all_inference_registered_controllers());
<OPENHUMAN_ROOT>/src/core/all.rs:201:    controllers.extend(crate::openhuman::inference::all_local_inference_registered_controllers());
<OPENHUMAN_ROOT>/src/core/all.rs:203:    controllers.extend(crate::openhuman::embeddings::all_embeddings_registered_controllers());
<OPENHUMAN_ROOT>/src/core/all.rs:205:    controllers.extend(crate::openhuman::people::all_people_registered_controllers());
<OPENHUMAN_ROOT>/src/core/all.rs:208:        crate::openhuman::screen_intelligence::all_screen_intelligence_registered_controllers(),
<OPENHUMAN_ROOT>/src/core/all.rs:211:    controllers.extend(crate::openhuman::sandbox::all_sandbox_registered_controllers());
<OPENHUMAN_ROOT>/src/core/all.rs:213:    controllers.extend(crate::openhuman::socket::all_socket_registered_controllers());
<OPENHUMAN_ROOT>/src/core/all.rs:215:    controllers.extend(crate::openhuman::javascript::all_javascript_registered_controllers());
<OPENHUMAN_ROOT>/src/core/all.rs:217:    controllers.extend(crate::openhuman::workflows::all_workflows_registered_controllers());
<OPENHUMAN_ROOT>/src/core/all.rs:219:    controllers.extend(crate::openhuman::skill_runtime::all_skill_runtime_registered_controllers());
<OPENHUMAN_ROOT>/src/core/all.rs:222:        .extend(crate::openhuman::skill_registry::all_skill_registry_registered_controllers());
<OPENHUMAN_ROOT>/src/core/all.rs:224:    controllers.extend(crate::openhuman::workspace::all_workspace_registered_controllers());
<OPENHUMAN_ROOT>/src/core/all.rs:226:    controllers.extend(crate::openhuman::tools::all_tools_registered_controllers());
<OPENHUMAN_ROOT>/src/core/all.rs:228:    controllers.extend(crate::openhuman::tool_registry::all_tool_registry_registered_controllers());
<OPENHUMAN_ROOT>/src/core/all.rs:230:    controllers.extend(crate::openhuman::memory::all_memory_registered_controllers());
<OPENHUMAN_ROOT>/src/core/all.rs:232:    controllers.extend(crate::openhuman::memory_goals::all_memory_goals_registered_controllers());
<OPENHUMAN_ROOT>/src/core/all.rs:234:    controllers.extend(crate::openhuman::thread_goals::all_thread_goals_registered_controllers());
