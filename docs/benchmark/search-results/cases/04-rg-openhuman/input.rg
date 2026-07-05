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
<OPENHUMAN_ROOT>/SECURITY.md:5:We provide security updates for the following versions of OpenHuman:
<OPENHUMAN_ROOT>/SECURITY.md:13:We recommend always running the [latest release](https://github.com/tinyhumansai/openhuman/releases/latest). OpenHuman is in early beta; older versions may not receive patches.
<OPENHUMAN_ROOT>/SECURITY.md:22:2. Email the maintainers with a clear description of the issue, steps to reproduce, and impact. You can reach us via the contact details listed in the [OpenHuman organization](https://github.com/openhumanxyz) or repository.
<OPENHUMAN_ROOT>/SECURITY.md:41:Out-of-scope for this process: general bugs, feature requests, and issues in third-party services we integrate with (e.g., Telegram, Notion) unless they are specific to how OpenHuman uses them.
<OPENHUMAN_ROOT>/SECURITY.md:53:Thank you for helping keep OpenHuman and its users safe.
<OPENHUMAN_ROOT>/AGENTS.md:1:# OpenHuman
<OPENHUMAN_ROOT>/AGENTS.md:13:| **`app/`**              | pnpm workspace `openhuman-app`: Vite + React (`app/src/`), Tauri desktop host (`app/src-tauri/`), Vitest tests                |
<OPENHUMAN_ROOT>/AGENTS.md:14:| **`src/`** (root)       | Rust lib crate `openhuman` + `openhuman-core` CLI binary (`src/main.rs`) — `src/core/` (transport), `src/openhuman/*` domains |
<OPENHUMAN_ROOT>/AGENTS.md:15:| **`Cargo.toml`** (root) | Core crate; `cargo build --bin openhuman-core`. Also `slack-backfill` and `gmail-backfill-3d` in `src/bin/`.                  |
<OPENHUMAN_ROOT>/AGENTS.md:18:Commands assume **repo root**. Root `package.json` is `openhuman-repo` (private, pnpm-enforced).
<OPENHUMAN_ROOT>/AGENTS.md:25:- **Core runs in-process** as a tokio task (sidecar removed PR #1061). Lifecycle: `core_process::CoreProcessHandle` in `app/src-tauri/src/core_process.rs`. Frontend RPC → `http://127.0.0.1:<port>/rpc` with per-launch hex bearer handed in-memory via `run_server_embedded_with_ready(rpc_token: Some(_))`. Renderer reads bearer via `core_rpc_token` Tauri command. `OPENHUMAN_CORE_TOKEN` still honoured for CLI/docker/cloud. Set `OPENHUMAN_CORE_REUSE_EXISTING=1` for external core debugging.
<OPENHUMAN_ROOT>/AGENTS.md:36:Connects to desktop core via `ConnectionProfile` transport strategies in `app/src/services/transport/`: `LanHttpTransport`, `TunnelTransport` (E2E encrypted XChaCha20-Poly1305), `CloudHttpTransport`. Key paths: PTT plugin `packages/tauri-plugin-ptt/`, iOS screens `app/src/pages/ios/`, devices domain `src/openhuman/devices/`, tunnel crypto `app/src/lib/tunnel/`. Build: `pnpm tauri:ios:dev` (stock `@tauri-apps/cli`, not vendored CEF). Backend dep: `tinyhumansai/backend#709`.
<OPENHUMAN_ROOT>/AGENTS.md:53:cargo build --manifest-path Cargo.toml --bin openhuman-core
<OPENHUMAN_ROOT>/AGENTS.md:97:- **Rust config**: TOML `Config` struct (`src/openhuman/config/schema/types.rs`) with env overrides (`load.rs`).
<OPENHUMAN_ROOT>/AGENTS.md:101:The `[autonomy]` block (`src/openhuman/config/schema/autonomy.rs`) drives `SecurityPolicy` (`src/openhuman/security/policy.rs`). Tiers: `readonly` / `supervised` / `full` × `workspace_only` × `trusted_roots` × `allow_tool_install`. Edit via `config.update_autonomy_settings` RPC or Settings → Agent access.
<OPENHUMAN_ROOT>/AGENTS.md:103:**Two path roots** (`src/openhuman/config/schema/types.rs`):
<OPENHUMAN_ROOT>/AGENTS.md:105:- **`action_dir`** — agent's read/write root. Acting tools resolve relative paths here. Default: `~/OpenHuman/projects` (`OPENHUMAN_ACTION_DIR`).
<OPENHUMAN_ROOT>/AGENTS.md:106:- **`workspace_dir`** — internal state (`~/.openhuman/users/<id>/workspace`). Agent tools **cannot** write here — enforced by `is_workspace_internal_path` fail-closed regardless of tier/trusted_roots.
<OPENHUMAN_ROOT>/AGENTS.md:110:**Approval gate** ON by default (opt out: `OPENHUMAN_APPROVAL_GATE=0`). Parks interactive chat turns only; background/cron allowed through. Frontend surfaces via `ApprovalRequestCard`. 10-min TTL → Deny.
<OPENHUMAN_ROOT>/AGENTS.md:135:- `e2e-run-spec.sh` creates/cleans temp `OPENHUMAN_WORKSPACE` by default.
<OPENHUMAN_ROOT>/AGENTS.md:158:**AI config**: bundled prompts in `src/openhuman/agent/prompts/` (also via `tauri.conf.json` resources). Loaders in `app/src/lib/ai/` with `?raw` imports.
<OPENHUMAN_ROOT>/AGENTS.md:166:IPC commands: `greet`, `write_ai_config_file`, `ai_get_config`, `ai_refresh_config`, `core_rpc_relay`, `core_rpc_token`, `start_core_process`, `restart_core_process`, window commands, `openhuman_*` daemon helpers.
<OPENHUMAN_ROOT>/AGENTS.md:176:### Domain layout (`src/openhuman/`)
<OPENHUMAN_ROOT>/AGENTS.md:180:**Skills runtime removed**: QuickJS gone. `src/openhuman/skills/` is metadata-only now.
<OPENHUMAN_ROOT>/AGENTS.md:184:- New functionality → dedicated subdirectory (`openhuman/<domain>/mod.rs` + siblings). No new root-level `*.rs` files.
<OPENHUMAN_ROOT>/AGENTS.md:185:- **Tool ownership**: domain tools live in that domain's `tools.rs`, re-exported via `src/openhuman/tools/mod.rs`. Only cross-cutting families stay in `tools/impl/`.
<OPENHUMAN_ROOT>/AGENTS.md:261:1. **Specify** — ground in existing domains, controller patterns, JSON-RPC naming (`openhuman.<namespace>_<function>`).
<OPENHUMAN_ROOT>/AGENTS.md:268:Update `src/openhuman/about_app/` when adding/removing/renaming user-facing features. Define E2E scenarios up front covering happy paths, failures, auth gates.
<OPENHUMAN_ROOT>/AGENTS.md:277:origin    git@github.com:<your-username>/openhuman.git  (push here)
<OPENHUMAN_ROOT>/AGENTS.md:278:upstream  git@github.com:tinyhumansai/openhuman.git     (fetch-only)
<OPENHUMAN_ROOT>/AGENTS.md:282:- Issues and PRs on upstream `tinyhumansai/openhuman`.
<OPENHUMAN_ROOT>/AGENTS.md:293:- **Windows deep links**: `openhuman://` registered via `tauri-plugin-deep-link::register_all`. Check in `app/src-tauri/src/deep_link_registration_check.rs`.
<OPENHUMAN_ROOT>/AGENTS.md:294:- **Core standalone debugging**: `./target/debug/openhuman-core serve` (token at `{workspace}/core.token`). Public endpoints: `GET /health`, `GET /schema`, `GET /events`.
<OPENHUMAN_ROOT>/INSTALL.md:1:# Installing OpenHuman
<OPENHUMAN_ROOT>/INSTALL.md:3:Download installers from [tinyhumans.ai/openhuman](https://tinyhumans.ai/openhuman?utm_source=github&utm_medium=readme) or from the [GitHub Releases](https://github.com/tinyhumansai/openhuman/releases/latest) page. For terminal installs, the native package paths below are preferred because they use your OS package manager or native installer where available.
<OPENHUMAN_ROOT>/INSTALL.md:13:brew install openhuman
<OPENHUMAN_ROOT>/INSTALL.md:19:# Download OpenHuman_<version>_amd64.deb or OpenHuman_<version>_arm64.deb
<OPENHUMAN_ROOT>/INSTALL.md:20:# from https://github.com/tinyhumansai/openhuman/releases/latest, then:
<OPENHUMAN_ROOT>/INSTALL.md:22:sudo apt-get install -y --no-install-recommends ./OpenHuman_*_amd64.deb
<OPENHUMAN_ROOT>/INSTALL.md:25:**Linux (Arch, AUR):** the [`openhuman-bin` AUR recipe](./packages/arch/openhuman-bin/) is in the repo. Once published, Arch users can install it with `yay -S openhuman-bin`.
<OPENHUMAN_ROOT>/INSTALL.md:27:**Windows:** download the signed `.msi` from the [latest release](https://github.com/tinyhumansai/openhuman/releases/latest) and run it.
<OPENHUMAN_ROOT>/INSTALL.md:29:**Manual `.dmg` / `.deb` / `.AppImage` / `.msi`:** grab the installer for your platform directly from the [latest release page](https://github.com/tinyhumansai/openhuman/releases/latest).
<OPENHUMAN_ROOT>/INSTALL.md:31:> **Linux:** the AppImage can crash on launch under Wayland, miss host system libraries such as `libgbm.so.1`, or fail on Arch-based distros with `sharun: Interpreter not found!`. See [#2463](https://github.com/tinyhumansai/openhuman/issues/2463) for the cause and env-var workarounds. The `.deb` package above avoids those failure modes on Debian/Ubuntu by letting apt resolve runtime dependencies.
<OPENHUMAN_ROOT>/INSTALL.md:39:curl -fsSL https://raw.githubusercontent.com/tinyhumansai/openhuman/main/scripts/install.sh | bash
<OPENHUMAN_ROOT>/INSTALL.md:42:irm https://raw.githubusercontent.com/tinyhumansai/openhuman/main/scripts/install.ps1 | iex
<OPENHUMAN_ROOT>/INSTALL.md:45:On Debian/Ubuntu, `install.sh` resolves the latest release `.deb` first and installs it with `apt-get` so runtime dependencies are handled by apt. Set `OPENHUMAN_INSTALLER_LINUX_PACKAGE=appimage` to force the AppImage path.
<OPENHUMAN_ROOT>/INSTALL.md:49:A separately signed script-install path is not currently available. Issue [#2620](https://github.com/tinyhumansai/openhuman/issues/2620) is closed after the native package paths were promoted, but current release assets do not include `install.sh.asc` / `install.ps1.asc` for pre-execution script verification. Treat the script install path as unverified and prefer the native package options above when possible.
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
<OPENHUMAN_ROOT>/Cargo.lock:4343:name = "openhuman"
<OPENHUMAN_ROOT>/README.md:1:<h1 align="center">OpenHuman</h1>
<OPENHUMAN_ROOT>/README.md:9:		<img src="https://trendshift.io/api/badge/repositories/23680" alt="tinyhumansai%2Fopenhuman | Trendshift" style="width: 250px; height: 55px;" width="250" height="55"/>
<OPENHUMAN_ROOT>/README.md:11:	<a href="https://www.producthunt.com/products/openhuman?embed=true&amp;utm_source=badge-top-post-badge&amp;utm_medium=badge&amp;utm_campaign=badge-openhuman" target="_blank" rel="noopener noreferrer">
<OPENHUMAN_ROOT>/README.md:12:		<img alt="OpenHuman - An open source AI harness built with the human in mind | Product Hunt" width="250" height="54" src="https://api.producthunt.com/widgets/embed-image/v1/top-post-badge.svg?post_id=1136902&amp;theme=light&amp;period=daily&amp;t=1778916022823">
<OPENHUMAN_ROOT>/README.md:14:		<a href="https://www.producthunt.com/products/openhuman?embed=true&amp;utm_source=badge-top-post-badge&amp;utm_medium=badge&amp;utm_campaign=badge-openhuman" target="_blank" rel="noopener noreferrer">
<OPENHUMAN_ROOT>/README.md:15:			<img alt="OpenHuman - An open source AI harness built with the human in mind | Product Hunt" width="250" height="54" src="https://api.producthunt.com/widgets/embed-image/v1/top-post-badge.svg?post_id=1136902&amp;theme=light&amp;period=weekly&amp;t=1779351403565">
<OPENHUMAN_ROOT>/README.md:19: <a href="https://www.producthunt.com/products/openhuman?embed=true&amp;utm_source=badge-top-post-topic-badge&amp;utm_medium=badge&amp;utm_campaign=badge-openhuman" target="_blank" rel="noopener noreferrer">
<OPENHUMAN_ROOT>/README.md:20:  <img alt="OpenHuman - An open source AI harness built with the human in mind | Product Hunt" width="250" height="54" src="https://api.producthunt.com/widgets/embed-image/v1/top-post-topic-badge.svg?post_id=1136902&amp;theme=light&amp;period=weekly&amp;topic_id=268&amp;t=1779351808756">
<OPENHUMAN_ROOT>/README.md:22:  <a href="https://www.producthunt.com/products/openhuman?embed=true&amp;utm_source=badge-top-post-topic-badge&amp;utm_medium=badge&amp;utm_campaign=badge-openhuman" target="_blank" rel="noopener noreferrer">
<OPENHUMAN_ROOT>/README.md:23:   <img alt="OpenHuman - An open source AI harness built with the human in mind | Product Hunt" width="250" height="54" src="https://api.producthunt.com/widgets/embed-image/v1/top-post-topic-badge.svg?post_id=1136902&amp;theme=light&amp;period=weekly&amp;topic_id=46&amp;t=1779351808756">
<OPENHUMAN_ROOT>/README.md:28: <strong>OpenHuman is your personal AI super intelligence: a brain that remembers everything, a fantastic orchestrator, a deep researcher. Local-first, simple, powerful.</strong>
<OPENHUMAN_ROOT>/README.md:35: <a href="https://tinyhumans.gitbook.io/openhuman/">Docs</a> •
<OPENHUMAN_ROOT>/README.md:45: <a href="https://github.com/tinyhumansai/openhuman/releases/latest"><img src="https://img.shields.io/github/v/release/tinyhumansai/openhuman?label=latest" alt="Latest Release" /></a>
<OPENHUMAN_ROOT>/README.md:46: <a href="https://github.com/tinyhumansai/openhuman/stargazers"><img src="https://img.shields.io/github/stars/tinyhumansai/openhuman?style=flat" alt="GitHub Stars" /></a>
<OPENHUMAN_ROOT>/README.md:47: <a href="./LICENSE"><img src="https://img.shields.io/github/license/tinyhumansai/openhuman" alt="License" /></a>
<OPENHUMAN_ROOT>/README.md:52:> OpenHuman is not AGI. But it is a meaningful architectural step closer, with better memory, better orchestration, and better tooling.
<OPENHUMAN_ROOT>/README.md:54:> 🎉 Within one week of launch, OpenHuman became the number one trending repository on GitHub for nine days in a row.
<OPENHUMAN_ROOT>/README.md:58:Download installers from [tinyhumans.ai/openhuman](https://tinyhumans.ai/openhuman?utm_source=github&utm_medium=readme) or from the [GitHub Releases](https://github.com/tinyhumansai/openhuman/releases/latest) page.
<OPENHUMAN_ROOT>/README.md:62:# What is OpenHuman?
<OPENHUMAN_ROOT>/README.md:64:OpenHuman is three things most assistants aren't: **a brain** that builds a persistent, local memory of your world; **a fantastic orchestrator** that runs fleets of agents on durable graphs; and **a deep researcher** that sweeps your data and the web before you finish asking. Every bullet links to the deeper writeup in the [docs](https://tinyhumans.gitbook.io/openhuman/).
<OPENHUMAN_ROOT>/README.md:68:- **[Memory Tree](https://tinyhumans.gitbook.io/openhuman/features/memory-tree) + [Obsidian Wiki](https://tinyhumans.gitbook.io/openhuman/features/obsidian-wiki)**: your data compressed into scored Markdown trees in SQLite on your machine, mirrored as an [Obsidian vault](https://x.com/karpathy/status/2039805659525644595) you can open and edit. No vector-soup black box.
<OPENHUMAN_ROOT>/README.md:69:- **[100+ OAuth integrations, 5,000+ MCP servers, 90,000+ Skills](https://tinyhumans.gitbook.io/openhuman/features/integrations)**: one click into Gmail, Notion, GitHub, Slack and the rest of your stack. [Auto-fetch](https://tinyhumans.gitbook.io/openhuman/features/obsidian-wiki/auto-fetch) feeds the brain every 20 minutes, so it has tomorrow's context this morning.
<OPENHUMAN_ROOT>/README.md:70:- **[A subconscious](https://tinyhumans.gitbook.io/openhuman/features/subconscious)**: a background loop that diffs your world, advances your goals, and writes your morning briefing. Thinking continues after you stop typing.
<OPENHUMAN_ROOT>/README.md:71:- **[Goals & Todos](https://tinyhumans.gitbook.io/openhuman/features/goals-and-todos)**: long-term goals, durable per-thread goals, and a shared kanban board per conversation.
<OPENHUMAN_ROOT>/README.md:72:- **[TokenJuice](https://tinyhumans.gitbook.io/openhuman/features/token-compression)**: tool output compressed before it hits the model: same information, up to 80% fewer tokens. A brain this big would be unaffordable without it.
<OPENHUMAN_ROOT>/README.md:76:- **[Workflows](https://tinyhumans.gitbook.io/openhuman/features/workflows)**: the agent proposes the automation; you review it on a canvas and save. Durable, trigger-driven, approval-gated runs on open-source [tinyflows](https://github.com/tinyhumansai/tinyflows).
<OPENHUMAN_ROOT>/README.md:77:- **[A harness that finishes the job](https://tinyhumans.gitbook.io/openhuman/developing/architecture/agent-harness)**: checkpointed graph runs on open-source [tinyagents](https://github.com/tinyhumansai/tinyagents). Stuck agents get steered, halted ones return a root cause, and every run replays with real per-call costs.
<OPENHUMAN_ROOT>/README.md:78:- **[A split brain, always on](https://tinyhumans.gitbook.io/openhuman/features/orchestration)**: a fast reflex agent triages inbound traffic while a deep reasoning core delegates to worker fleets, steered by the subconscious.
<OPENHUMAN_ROOT>/README.md:79:- **[An agent economy](https://tinyhumans.gitbook.io/openhuman/features/tinyplace)**: a `@handle` on [tiny.place](https://tiny.place), Signal-encrypted agent-to-agent orchestration, x402 USDC bounties and trading. Keys never touch disk.
<OPENHUMAN_ROOT>/README.md:83:- **[SuperContext](https://tinyhumans.gitbook.io/openhuman/features/super-context)**: a research scout sweeps your memory and files before the model reads your first message. No cold starts.
<OPENHUMAN_ROOT>/README.md:84:- **Batteries included**: web search, scraper, coder toolset, a real [browser](https://tinyhumans.gitbook.io/openhuman/features/native-tools/browser-and-computer), and [native voice](gitbooks/features/native-tools/voice.md) with in-process Whisper. [Model routing](https://tinyhumans.gitbook.io/openhuman/features/model-routing) picks the right LLM per workload on one subscription, with [local AI optional](https://tinyhumans.gitbook.io/openhuman/features/model-routing/local-ai).
<OPENHUMAN_ROOT>/README.md:85:- **[Meeting agents](https://tinyhumans.gitbook.io/openhuman/features/mascot/meeting-agents)**: joins **Meet, Zoom, Teams, and Webex** with a face and a voice. It auto-joins from your calendar, streams a live transcript, answers by name, and files a summary with action items.
<OPENHUMAN_ROOT>/README.md:86:- **[Image & video generation](https://tinyhumans.gitbook.io/openhuman/features/native-tools)**: Seedream/SeedEdit images and Seedance/Veo video, straight into your workspace on the same subscription.
<OPENHUMAN_ROOT>/README.md:87:- **[17 messaging channels](https://tinyhumans.gitbook.io/openhuman/features/channels)**: Telegram, Discord, Slack, WhatsApp, Signal, iMessage… plus **native email** (IMAP IDLE + SMTP). Your agent reaches you where you already are.
<OPENHUMAN_ROOT>/README.md:91:- **Simple, UI-first & Human**: install to working agent in a few clicks, with no config files and no terminal. And it has [a face](https://tinyhumans.gitbook.io/openhuman/features/mascot): a mascot that speaks, reacts, and remembers you.
<OPENHUMAN_ROOT>/README.md:92:- **[Privacy & security](https://tinyhumans.gitbook.io/openhuman/features/privacy-and-security)**: on-device encrypted data, approval gate, OS-keyring secrets, and opt-in sandboxing. There is also **[Privacy Mode](https://tinyhumans.gitbook.io/openhuman/features/privacy-mode)**: flip one switch and no inference leaves your machine, enforced in the Rust core.
<OPENHUMAN_ROOT>/README.md:93:- **[Themes & Theme Studio](https://tinyhumans.gitbook.io/openhuman/features/theming)**: five theme families plus a full visual editor, exportable as JSON.
<OPENHUMAN_ROOT>/README.md:97:OpenHuman is the first agent harness that gets to know you in minutes. Inspired by [Karpathy's LLM Knowledgebase](https://x.com/karpathy/status/2039805659525644595). Most agents start cold. Hermes learns by watching you work; OpenClaw waits for plugins to ferry context in. Either way, you spend days or weeks before the agent knows enough about your stack to be genuinely useful.
<OPENHUMAN_ROOT>/README.md:100: <img src="./gitbooks/.gitbook/assets/memory.png" alt="OpenHuman context-building diagram">
<OPENHUMAN_ROOT>/README.md:103:> OpenHuman summarizes and compresses all your documents, emails & chats; and creates a memory graph that lets your agent remember everything about you.
<OPENHUMAN_ROOT>/README.md:105:OpenHuman skips the wait. Connect your accounts, let [auto-fetch](https://tinyhumans.gitbook.io/openhuman/features/integrations/auto-fetch) pull data locally on a 20-minute loop, and then have [Memory Trees](https://tinyhumans.gitbook.io/openhuman/features/memory-tree) compress everything into Markdown files stored intelligently in a [Karpathy-style Obsidian wiki](https://tinyhumans.gitbook.io/openhuman/features/obsidian-wiki).
<OPENHUMAN_ROOT>/README.md:109:Already self-host [agentmemory](https://github.com/rohitg00/agentmemory) across other coding agents? OpenHuman ships an optional `Memory` backend that proxies to it. Set `memory.backend = "agentmemory"` in `config.toml` and the same durable store powers OpenHuman alongside Claude Code, Cursor, Codex, and OpenCode. See the [agentmemory backend](https://tinyhumans.gitbook.io/openhuman/features/obsidian-wiki/agentmemory-backend) page for setup.
<OPENHUMAN_ROOT>/README.md:113:Most agent harnesses run one agent in one loop. OpenHuman is an **[orchestrator](https://tinyhumans.gitbook.io/openhuman/features/orchestration)**:
<OPENHUMAN_ROOT>/README.md:116: <img src="./gitbooks/.gitbook/assets/orchestration.png" alt="OpenHuman orchestration diagram">
<OPENHUMAN_ROOT>/README.md:119:> Agent-to-agent messaging runs over Signal-protocol end-to-end encryption, so you can connect anything (Claude Code, Codex, OpenClaw, Hermes) and use OpenHuman to orchestrate all of your agents and tools.
<OPENHUMAN_ROOT>/README.md:127:Heavily inspired by n8n and Zapier, [workflows](https://tinyhumans.gitbook.io/openhuman/features/workflows) bring the same visual, trigger-driven automation to your agent, except the agent builds them for you. Ask for an automation and it proposes one: a [tinyflows](https://github.com/tinyhumansai/tinyflows) graph you review on a visual canvas before saving.
<OPENHUMAN_ROOT>/README.md:130: <img src="./gitbooks/.gitbook/assets/workflows.png" alt="OpenHuman workflow canvas">
<OPENHUMAN_ROOT>/README.md:137:## OpenHuman vs Other Agent Harnesses
<OPENHUMAN_ROOT>/README.md:139:High-level comparison (products evolve, so verify against each vendor). OpenHuman is built to **minimize vendor sprawl**, keep **workflow knowledge on-device**, and give the agent a **persistent memory** of your data, not only chat.
<OPENHUMAN_ROOT>/README.md:141:|                        | Claude Cowork     | OpenClaw          | Hermes Agent      | OpenHuman                                                                                                |
<OPENHUMAN_ROOT>/README.md:165:3. Use `pnpm dev` for web-only UI work, `pnpm --filter openhuman-app dev:app` for the desktop shell, and focused checks such as `pnpm typecheck`, `pnpm format:check`, and `cargo check -p openhuman --lib` before opening a PR.
<OPENHUMAN_ROOT>/README.md:167:Deeper docs: [Architecture](https://tinyhumans.gitbook.io/openhuman/developing/architecture) · [Getting Set Up](https://tinyhumans.gitbook.io/openhuman/developing/getting-set-up) · [Cloud Deploy](./gitbooks/features/cloud-deploy.md).
<OPENHUMAN_ROOT>/README.md:174: <a href="https://www.star-history.com/#tinyhumansai/openhuman&type=date&legend=top-left">
<OPENHUMAN_ROOT>/README.md:176: <source media="(prefers-color-scheme: dark)" srcset="https://api.star-history.com/svg?repos=tinyhumansai/openhuman&type=date&theme=dark&legend=top-left" />
<OPENHUMAN_ROOT>/README.md:177: <source media="(prefers-color-scheme: light)" srcset="https://api.star-history.com/svg?repos=tinyhumansai/openhuman&type=date&legend=top-left" />
<OPENHUMAN_ROOT>/README.md:178: <img alt="Star History Chart" src="https://api.star-history.com/svg?repos=tinyhumansai/openhuman&type=date&legend=top-left" />
<OPENHUMAN_ROOT>/README.md:187:<a href="https://github.com/tinyhumansai/openhuman/graphs/contributors">
<OPENHUMAN_ROOT>/README.md:188: <img src="https://contrib.rocks/image?repo=tinyhumansai/openhuman" alt="OpenHuman contributors" />
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
<OPENHUMAN_ROOT>/docs/README.ko.md:1:<h1 align="center">OpenHuman</h1>
<OPENHUMAN_ROOT>/docs/README.ko.md:9:		<img src="https://trendshift.io/api/badge/repositories/23680" alt="tinyhumansai%2Fopenhuman | Trendshift" style="width: 250px; height: 55px;" width="250" height="55"/>
<OPENHUMAN_ROOT>/docs/README.ko.md:11:	<a href="https://www.producthunt.com/products/openhuman?embed=true&amp;utm_source=badge-top-post-badge&amp;utm_medium=badge&amp;utm_campaign=badge-openhuman" target="_blank" rel="noopener noreferrer">
<OPENHUMAN_ROOT>/docs/README.ko.md:12:		<img alt="OpenHuman - An open source AI harness built with the human in mind | Product Hunt" width="250" height="54" src="https://api.producthunt.com/widgets/embed-image/v1/top-post-badge.svg?post_id=1136902&amp;theme=light&amp;period=daily&amp;t=1778916022823">
<OPENHUMAN_ROOT>/docs/README.ko.md:14:		<a href="https://www.producthunt.com/products/openhuman?embed=true&amp;utm_source=badge-top-post-badge&amp;utm_medium=badge&amp;utm_campaign=badge-openhuman" target="_blank" rel="noopener noreferrer">
<OPENHUMAN_ROOT>/docs/README.ko.md:15:			<img alt="OpenHuman - An open source AI harness built with the human in mind | Product Hunt" width="250" height="54" src="https://api.producthunt.com/widgets/embed-image/v1/top-post-badge.svg?post_id=1136902&amp;theme=light&amp;period=weekly&amp;t=1779351403565">
<OPENHUMAN_ROOT>/docs/README.ko.md:19: <a href="https://www.producthunt.com/products/openhuman?embed=true&amp;utm_source=badge-top-post-topic-badge&amp;utm_medium=badge&amp;utm_campaign=badge-openhuman" target="_blank" rel="noopener noreferrer">
<OPENHUMAN_ROOT>/docs/README.ko.md:20:  <img alt="OpenHuman - An open source AI harness built with the human in mind | Product Hunt" width="250" height="54" src="https://api.producthunt.com/widgets/embed-image/v1/top-post-topic-badge.svg?post_id=1136902&amp;theme=light&amp;period=weekly&amp;topic_id=268&amp;t=1779351808756">
<OPENHUMAN_ROOT>/docs/README.ko.md:22:  <a href="https://www.producthunt.com/products/openhuman?embed=true&amp;utm_source=badge-top-post-topic-badge&amp;utm_medium=badge&amp;utm_campaign=badge-openhuman" target="_blank" rel="noopener noreferrer">
<OPENHUMAN_ROOT>/docs/README.ko.md:23:   <img alt="OpenHuman - An open source AI harness built with the human in mind | Product Hunt" width="250" height="54" src="https://api.producthunt.com/widgets/embed-image/v1/top-post-topic-badge.svg?post_id=1136902&amp;theme=light&amp;period=weekly&amp;topic_id=46&amp;t=1779351808756">
<OPENHUMAN_ROOT>/docs/README.ko.md:28: <strong>OpenHuman은 당신의 개인용 AI 슈퍼 지능입니다: 모든 것을 기억하는 두뇌, 탁월한 오케스트레이터, 깊이 있는 리서처. 로컬 우선, 단순하고 강력합니다.</strong>
<OPENHUMAN_ROOT>/docs/README.ko.md:35: <a href="https://tinyhumans.gitbook.io/openhuman/">문서</a> •
<OPENHUMAN_ROOT>/docs/README.ko.md:45: <a href="https://github.com/tinyhumansai/openhuman/releases/latest"><img src="https://img.shields.io/github/v/release/tinyhumansai/openhuman?label=latest" alt="최신 릴리스" /></a>
<OPENHUMAN_ROOT>/docs/README.ko.md:46: <a href="https://github.com/tinyhumansai/openhuman/stargazers"><img src="https://img.shields.io/github/stars/tinyhumansai/openhuman?style=flat" alt="GitHub Stars" /></a>
<OPENHUMAN_ROOT>/docs/README.ko.md:47: <a href="../LICENSE"><img src="https://img.shields.io/github/license/tinyhumansai/openhuman" alt="라이선스" /></a>
<OPENHUMAN_ROOT>/docs/README.ko.md:52:> 🎉 출시 후 일주일 만에 OpenHuman은 9일 연속 GitHub 트렌딩 저장소 1위에 올랐습니다.
<OPENHUMAN_ROOT>/docs/README.ko.md:56:[tinyhumans.ai/openhuman](https://tinyhumans.ai/openhuman?utm_source=github&utm_medium=readme) 또는 [GitHub Releases](https://github.com/tinyhumansai/openhuman/releases/latest) 페이지에서 설치 프로그램을 다운로드하세요.
<OPENHUMAN_ROOT>/docs/README.ko.md:60:# OpenHuman이란 무엇인가요?
<OPENHUMAN_ROOT>/docs/README.ko.md:62:OpenHuman은 대부분의 어시스턴트가 갖지 못한 세 가지입니다: 당신의 세계에 대한 지속적인 로컬 메모리를 구축하는 **두뇌**, 내구성 있는 그래프 위에서 에이전트 함대를 운영하는 **탁월한 오케스트레이터**, 그리고 질문을 끝내기도 전에 당신의 데이터와 웹을 훑는 **깊이 있는 리서처**. 각 글머리 기호는 [문서](https://tinyhumans.gitbook.io/openhuman/)의 더 깊은 설명으로 연결됩니다.
<OPENHUMAN_ROOT>/docs/README.ko.md:66:- **[메모리 트리(Memory Tree)](https://tinyhumans.gitbook.io/openhuman/features/memory-tree) + [Obsidian 위키](https://tinyhumans.gitbook.io/openhuman/features/obsidian-wiki)**: 당신의 데이터는 점수가 매겨진 Markdown 트리로 압축되어 당신의 머신에 있는 SQLite에 저장되고, 열어서 직접 편집할 수 있는 [Obsidian 볼트](https://x.com/karpathy/status/2039805659525644595)로 미러링됩니다. 벡터 수프 같은 블랙박스가 아닙니다.
<OPENHUMAN_ROOT>/docs/README.ko.md:67:- **[100개 이상의 OAuth 통합, 5,000개 이상의 MCP 서버, 90,000개 이상의 Skills](https://tinyhumans.gitbook.io/openhuman/features/integrations)**: Gmail, Notion, GitHub, Slack 등 당신의 스택을 원클릭으로 연결하세요. [자동 가져오기(auto-fetch)](https://tinyhumans.gitbook.io/openhuman/features/obsidian-wiki/auto-fetch)가 20분마다 두뇌에 데이터를 공급합니다. 덕분에 오늘 아침에 이미 내일의 컨텍스트를 가지고 있습니다.
<OPENHUMAN_ROOT>/docs/README.ko.md:68:- **[잠재의식(subconscious)](https://tinyhumans.gitbook.io/openhuman/features/subconscious)**: 당신의 세계의 변화를 비교 분석하고, 목표를 진전시키고, 아침 브리핑을 작성하는 백그라운드 루프입니다. 타이핑을 멈춘 후에도 생각은 계속됩니다.
<OPENHUMAN_ROOT>/docs/README.ko.md:69:- **[목표 및 할 일(Goals & Todos)](https://tinyhumans.gitbook.io/openhuman/features/goals-and-todos)**: 장기 목표, 스레드별 지속 목표, 그리고 대화별 공유 칸반 보드를 제공합니다.
<OPENHUMAN_ROOT>/docs/README.ko.md:70:- **[TokenJuice](https://tinyhumans.gitbook.io/openhuman/features/token-compression)**: 도구 출력은 모델에 닿기 전에 압축되어, 동일한 정보가 최대 80% 적은 토큰으로 전달됩니다. 이것 없이는 이만큼 큰 두뇌를 감당할 수 없을 것입니다.
<OPENHUMAN_ROOT>/docs/README.ko.md:74:- **[Workflows](https://tinyhumans.gitbook.io/openhuman/features/workflows)**: 에이전트가 자동화를 제안하면 캔버스에서 검토하고 저장하면 됩니다. 내구성 있고, 트리거 기반이며, 승인 게이트를 거치는 실행이 오픈 소스 [tinyflows](https://github.com/tinyhumansai/tinyflows) 위에서 동작합니다.
<OPENHUMAN_ROOT>/docs/README.ko.md:75:- **[일을 끝까지 마무리하는 하네스](https://tinyhumans.gitbook.io/openhuman/developing/architecture/agent-harness)**: 오픈 소스 [tinyagents](https://github.com/tinyhumansai/tinyagents) 기반의 체크포인트 그래프 실행입니다. 막힌 에이전트는 방향을 조정받고, 중단된 에이전트는 근본 원인을 돌려주며, 모든 실행은 호출별 실제 비용과 함께 재생됩니다.
<OPENHUMAN_ROOT>/docs/README.ko.md:76:- **[상시 가동되는 분할 두뇌(split brain)](https://tinyhumans.gitbook.io/openhuman/features/orchestration)**: 빠른 반사 에이전트가 들어오는 트래픽을 분류하는 동안 깊은 추론 코어가 워커 함대에 작업을 위임하며, 잠재의식이 이를 조종합니다.
<OPENHUMAN_ROOT>/docs/README.ko.md:77:- **[에이전트 경제](https://tinyhumans.gitbook.io/openhuman/features/tinyplace)**: [tiny.place](https://tiny.place)의 `@handle`, Signal로 암호화된 에이전트 간 오케스트레이션, x402 USDC 바운티와 거래까지 제공합니다. 키는 디스크에 절대 닿지 않습니다.
<OPENHUMAN_ROOT>/docs/README.ko.md:81:- **[SuperContext](https://tinyhumans.gitbook.io/openhuman/features/super-context)**: 모델이 첫 메시지를 읽기 전에 리서치 스카우트가 당신의 메모리와 파일을 훑습니다. 콜드 스타트가 없습니다.
<OPENHUMAN_ROOT>/docs/README.ko.md:82:- **모든 것이 포함됨(Batteries included)**: 웹 검색, 스크레이퍼, 코더 툴셋, 실제 [브라우저](https://tinyhumans.gitbook.io/openhuman/features/native-tools/browser-and-computer), 인프로세스 Whisper를 갖춘 [네이티브 음성](../gitbooks/features/native-tools/voice.md), 그리고 워크로드별로 적합한 LLM을 선택하는 [모델 라우팅](https://tinyhumans.gitbook.io/openhuman/features/model-routing)까지. 하나의 구독으로, [로컬 AI는 선택 사항](https://tinyhumans.gitbook.io/openhuman/features/model-routing/local-ai)입니다.
<OPENHUMAN_ROOT>/docs/README.ko.md:83:- **[미팅 에이전트](https://tinyhumans.gitbook.io/openhuman/features/mascot/meeting-agents)**: 얼굴과 목소리를 가지고 **Meet, Zoom, Teams, Webex**에 참여합니다. 캘린더에서 자동으로 참여하고, 실시간 자막을 스트리밍하며, 이름이 불리면 대답하고, 요약과 액션 아이템을 정리합니다.
<OPENHUMAN_ROOT>/docs/README.ko.md:84:- **[이미지 및 비디오 생성](https://tinyhumans.gitbook.io/openhuman/features/native-tools)**: Seedream/SeedEdit 이미지와 Seedance/Veo 비디오가 동일한 구독으로 워크스페이스에 바로 생성됩니다.
<OPENHUMAN_ROOT>/docs/README.ko.md:85:- **[17개의 메시징 채널](https://tinyhumans.gitbook.io/openhuman/features/channels)**: Telegram, Discord, Slack, WhatsApp, Signal, iMessage… 그리고 **네이티브 이메일**(IMAP IDLE + SMTP)까지. 에이전트는 당신이 이미 있는 곳에서 당신에게 닿습니다.
<OPENHUMAN_ROOT>/docs/README.ko.md:89:- **단순함, UI 우선 및 인간 중심**: 설치 후 몇 번의 클릭만으로 작동하는 에이전트를 만날 수 있습니다. 설정 파일도, 터미널도 필요 없습니다. 그리고 [얼굴](https://tinyhumans.gitbook.io/openhuman/features/mascot)이 있습니다: 말하고, 반응하고, 당신을 기억하는 마스코트입니다.
<OPENHUMAN_ROOT>/docs/README.ko.md:90:- **[개인 정보 보호 및 보안](https://tinyhumans.gitbook.io/openhuman/features/privacy-and-security)**: 기기 내 암호화된 데이터, 승인 게이트, OS 키링 시크릿, 선택적 샌드박싱, 그리고 **[Privacy Mode](https://tinyhumans.gitbook.io/openhuman/features/privacy-mode)**: 스위치 하나로 어떤 추론도 당신의 머신을 떠나지 않으며, Rust 코어에서 강제됩니다.
<OPENHUMAN_ROOT>/docs/README.ko.md:91:- **[테마 및 Theme Studio](https://tinyhumans.gitbook.io/openhuman/features/theming)**: 5가지 테마 패밀리와 완전한 시각적 에디터, JSON으로 내보낼 수 있습니다.
<OPENHUMAN_ROOT>/docs/README.ko.md:95:OpenHuman은 몇 분 만에 당신을 알게 되는 최초의 에이전트 하네스입니다. [Karpathy의 LLM 지식 베이스](https://x.com/karpathy/status/2039805659525644595)에서 영감을 받았습니다. 대부분의 에이전트는 아무런 정보 없이 시작합니다. Hermes는 당신의 작업을 지켜보며 학습하고, OpenClaw는 플러그인이 컨텍스트를 가져오기를 기다립니다. 어느 쪽이든 에이전트가 당신의 스택에 대해 충분히 알고 정말 유용해지기까지는 며칠 또는 몇 주가 걸립니다.
<OPENHUMAN_ROOT>/docs/README.ko.md:98: <img src="../gitbooks/.gitbook/assets/memory.png" alt="OpenHuman 컨텍스트 구축 다이어그램">
<OPENHUMAN_ROOT>/docs/README.ko.md:101:> OpenHuman은 당신의 모든 문서, 이메일 및 채팅을 요약하고 압축합니다. 그리고 에이전트가 당신에 대한 모든 것을 기억할 수 있도록 메모리 그래프를 생성합니다.
<OPENHUMAN_ROOT>/docs/README.ko.md:103:OpenHuman은 기다림을 생략합니다. 계정을 연결하고, [자동 가져오기](https://tinyhumans.gitbook.io/openhuman/features/integrations/auto-fetch)가 20분 주기로 데이터를 로컬로 가져오게 한 다음, [메모리 트리](https://tinyhumans.gitbook.io/openhuman/features/memory-tree)가 모든 것을 [Karpathy 스타일의 Obsidian 위키](https://tinyhumans.gitbook.io/openhuman/features/obsidian-wiki)에 지능적으로 저장된 Markdown 파일로 압축하게 하세요.
<OPENHUMAN_ROOT>/docs/README.ko.md:107:이미 다른 코딩 에이전트에서 [agentmemory](https://github.com/rohitg00/agentmemory)를 자체 호스팅하고 있나요? OpenHuman은 이를 프록시하는 선택적 `Memory` 백엔드를 제공합니다. `config.toml`에서 `memory.backend = "agentmemory"`를 설정하면 동일한 내구성 있는 저장소가 Claude Code, Cursor, Codex, OpenCode와 함께 OpenHuman을 구동합니다. 설정 방법은 [agentmemory 백엔드](https://tinyhumans.gitbook.io/openhuman/features/obsidian-wiki/agentmemory-backend) 페이지를 참조하세요.
<OPENHUMAN_ROOT>/docs/README.ko.md:111:대부분의 에이전트 하네스는 하나의 루프에서 하나의 에이전트를 실행합니다. OpenHuman은 **[오케스트레이터](https://tinyhumans.gitbook.io/openhuman/features/orchestration)**입니다:
<OPENHUMAN_ROOT>/docs/README.ko.md:114: <img src="../gitbooks/.gitbook/assets/orchestration.png" alt="OpenHuman 오케스트레이션 다이어그램">
<OPENHUMAN_ROOT>/docs/README.ko.md:117:> 에이전트 간 메시징은 Signal 프로토콜 종단 간 암호화 위에서 동작하므로, Claude Code, Codex, OpenClaw, Hermes 등 무엇이든 연결하고 OpenHuman으로 모든 에이전트와 도구를 오케스트레이션할 수 있습니다.
<OPENHUMAN_ROOT>/docs/README.ko.md:125:n8n과 Zapier에서 깊은 영감을 받은 [워크플로우](https://tinyhumans.gitbook.io/openhuman/features/workflows)는 동일한 시각적, 트리거 기반 자동화를 에이전트에 가져옵니다. 다만 에이전트가 대신 만들어 준다는 점이 다릅니다. 자동화를 요청하면 에이전트가 하나를 제안합니다: 저장하기 전에 시각적 캔버스에서 검토하는 [tinyflows](https://github.com/tinyhumansai/tinyflows) 그래프입니다.
<OPENHUMAN_ROOT>/docs/README.ko.md:128: <img src="../gitbooks/.gitbook/assets/workflows.png" alt="OpenHuman 워크플로우 캔버스">
<OPENHUMAN_ROOT>/docs/README.ko.md:135:## OpenHuman vs 다른 에이전트 하네스
<OPENHUMAN_ROOT>/docs/README.ko.md:137:상위 수준 비교(제품은 진화하므로 각 벤더에 확인하세요). OpenHuman은 **벤더 분산화(sprawl)를 최소화**하고, **워크플로우 지식을 기기에 유지**하며, 채팅뿐만 아니라 당신의 데이터에 대한 **지속적인 기억**을 에이전트에게 제공하도록 구축되었습니다.
<OPENHUMAN_ROOT>/docs/README.ko.md:139:|                    | Claude Cowork     | OpenClaw          | Hermes Agent      | OpenHuman                                                                                            |
<OPENHUMAN_ROOT>/docs/README.ko.md:163:3. 웹 전용 UI 작업에는 `pnpm dev`를, 데스크톱 쉘에는 `pnpm --filter openhuman-app dev:app`을 사용하고, PR을 열기 전에 `pnpm typecheck`, `pnpm format:check`, `cargo check -p openhuman --lib`와 같은 집중 점검을 수행합니다.
<OPENHUMAN_ROOT>/docs/README.ko.md:165:상세 문서: [아키텍처](https://tinyhumans.gitbook.io/openhuman/developing/architecture) · [설정하기](https://tinyhumans.gitbook.io/openhuman/developing/getting-set-up) · [클라우드 배포](../gitbooks/features/cloud-deploy.md).
<OPENHUMAN_ROOT>/docs/README.ko.md:172: <a href="https://www.star-history.com/#tinyhumansai/openhuman&type=date&legend=top-left">
<OPENHUMAN_ROOT>/docs/README.ko.md:174: <source media="(prefers-color-scheme: dark)" srcset="https://api.star-history.com/svg?repos=tinyhumansai/openhuman&type=date&theme=dark&legend=top-left" />
<OPENHUMAN_ROOT>/docs/README.ko.md:175: <source media="(prefers-color-scheme: light)" srcset="https://api.star-history.com/svg?repos=tinyhumansai/openhuman&type=date&legend=top-left" />
<OPENHUMAN_ROOT>/docs/README.ko.md:176: <img alt="Star History Chart" src="https://api.star-history.com/svg?repos=tinyhumansai/openhuman&type=date&legend=top-left" />
<OPENHUMAN_ROOT>/docs/README.ko.md:185:<a href="https://github.com/tinyhumansai/openhuman/graphs/contributors">
<OPENHUMAN_ROOT>/docs/README.ko.md:186: <img src="https://contrib.rocks/image?repo=tinyhumansai/openhuman" alt="OpenHuman 기여자" />
<OPENHUMAN_ROOT>/docker-compose.yml:1:# OpenHuman Core — Docker Compose for self-hosted cloud deploy.
<OPENHUMAN_ROOT>/docker-compose.yml:3:# Brings up the headless Rust core (`openhuman-core`) on :7788, persists the
<OPENHUMAN_ROOT>/docker-compose.yml:9:#      OPENHUMAN_CORE_TOKEN; the latter is required for any client that calls
<OPENHUMAN_ROOT>/docker-compose.yml:15:# instead of building, replace `build:` with `image: ghcr.io/.../openhuman-core:<tag>`.
<OPENHUMAN_ROOT>/docker-compose.yml:18:  openhuman-core:
<OPENHUMAN_ROOT>/docker-compose.yml:22:    image: openhuman-core:local
<OPENHUMAN_ROOT>/docker-compose.yml:23:    container_name: openhuman-core
<OPENHUMAN_ROOT>/docker-compose.yml:33:      - "${OPENHUMAN_CORE_PORT:-7788}:7788"
