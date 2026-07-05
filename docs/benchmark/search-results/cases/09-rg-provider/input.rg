<OPENHUMAN_ROOT>/src/api/jwt.rs:7:pub use crate::openhuman::credentials::{APP_SESSION_PROVIDER, DEFAULT_AUTH_PROFILE_NAME};
<OPENHUMAN_ROOT>/src/api/rest_tests.rs:330:        provider,
<OPENHUMAN_ROOT>/src/api/rest_tests.rs:336:    assert_eq!(provider, "telegram");
<OPENHUMAN_ROOT>/src/api/rest_tests.rs:339:    // Discord path — proves the helper is provider-agnostic.
<OPENHUMAN_ROOT>/src/api/rest_tests.rs:351:        provider,
<OPENHUMAN_ROOT>/src/api/rest_tests.rs:357:    assert_eq!(provider, "discord");
<OPENHUMAN_ROOT>/src/api/rest_tests.rs:573:        provider: "telegram".to_string(),
<OPENHUMAN_ROOT>/src/api/rest_tests.rs:616:    // 404 on a non-`/channels/<provider>/messages/<id>` path should NOT be
<OPENHUMAN_ROOT>/src/api/rest_tests.rs:653:fn parse_message_path_discord_provider() {
<OPENHUMAN_ROOT>/src/api/rest_tests.rs:743:        provider,
<OPENHUMAN_ROOT>/src/api/rest_tests.rs:749:    assert_eq!(provider, "telegram");
<OPENHUMAN_ROOT>/src/api/config.rs:157:        let is_inference_provider = looks_like_inference_provider_endpoint(u);
<OPENHUMAN_ROOT>/src/api/config.rs:167:            crate::openhuman::config::schema::cloud_providers::endpoint_host(u).is_some_and(|h| {
<OPENHUMAN_ROOT>/src/api/config.rs:168:                crate::openhuman::config::schema::cloud_providers::host_is_builtin_cloud_provider(
<OPENHUMAN_ROOT>/src/api/config.rs:176:            is_inference_provider,
<OPENHUMAN_ROOT>/src/api/config.rs:183:        // (local model runner OR remote managed provider), OR when it is one of
<OPENHUMAN_ROOT>/src/api/config.rs:189:        // provider (`openrouter.ai`, `api.openmodel.ai`, …) was silently
<OPENHUMAN_ROOT>/src/api/config.rs:191:        // `/teams/*`, billing, referral — which the provider answers with a
<OPENHUMAN_ROOT>/src/api/config.rs:197:        // widens the same fallback to remote providers. Inference routing is
<OPENHUMAN_ROOT>/src/api/config.rs:201:        // `is_cloud_inference` (builtin cloud-provider host check, #4286) is
<OPENHUMAN_ROOT>/src/api/config.rs:202:        // kept as an additional signal: it and `is_inference_provider` use
<OPENHUMAN_ROOT>/src/api/config.rs:207:        if (!is_local_ai && !is_inference_provider && !is_cloud_inference) || is_openhuman {
<OPENHUMAN_ROOT>/src/api/config.rs:220:            is_inference_provider,
<OPENHUMAN_ROOT>/src/api/config.rs:222:            "[api/config] override classified as inference endpoint (managed provider or builtin cloud host) — falling back to backend default chain"
<OPENHUMAN_ROOT>/src/api/config.rs:302:/// Well-known managed inference-provider registrable domains. A `config.api_url`
<OPENHUMAN_ROOT>/src/api/config.rs:306:/// Suffix-matched so `api.<provider>` / `<region>.<provider>` also classify.
<OPENHUMAN_ROOT>/src/api/config.rs:311:const INFERENCE_PROVIDER_DOMAINS: &[&str] = &[
<OPENHUMAN_ROOT>/src/api/config.rs:334:/// provider** base rather than the hosted OpenHuman backend.
<OPENHUMAN_ROOT>/src/api/config.rs:343:/// 1. **Known provider host** — host equals or is a subdomain of a domain in
<OPENHUMAN_ROOT>/src/api/config.rs:344:///    [`INFERENCE_PROVIDER_DOMAINS`].
<OPENHUMAN_ROOT>/src/api/config.rs:352:pub fn looks_like_inference_provider_endpoint(url: &str) -> bool {
<OPENHUMAN_ROOT>/src/api/config.rs:368:    // ── Signal 1: known managed provider host (apex or subdomain) ──────────
<OPENHUMAN_ROOT>/src/api/config.rs:371:        if INFERENCE_PROVIDER_DOMAINS
<OPENHUMAN_ROOT>/src/api/config.rs:1183:        // cloud-inference provider's *canonical base* (no `/chat/completions`
<OPENHUMAN_ROOT>/src/api/config.rs:1250:    // ── GH #4153: remote managed inference providers parked in `api_url` ──────
<OPENHUMAN_ROOT>/src/api/config.rs:1253:    fn inference_provider_matches_known_remote_hosts() {
<OPENHUMAN_ROOT>/src/api/config.rs:1255:        assert!(looks_like_inference_provider_endpoint(
<OPENHUMAN_ROOT>/src/api/config.rs:1258:        assert!(looks_like_inference_provider_endpoint(
<OPENHUMAN_ROOT>/src/api/config.rs:1261:        // Other managed providers, apex and subdomain.
<OPENHUMAN_ROOT>/src/api/config.rs:1262:        assert!(looks_like_inference_provider_endpoint(
<OPENHUMAN_ROOT>/src/api/config.rs:1265:        assert!(looks_like_inference_provider_endpoint(
<OPENHUMAN_ROOT>/src/api/config.rs:1268:        assert!(looks_like_inference_provider_endpoint(
<OPENHUMAN_ROOT>/src/api/config.rs:1274:    fn inference_provider_matches_bare_v1_base_on_unknown_host() {
<OPENHUMAN_ROOT>/src/api/config.rs:1275:        // An unknown OpenAI-compatible provider, recognised by its `/v1` base.
<OPENHUMAN_ROOT>/src/api/config.rs:1276:        assert!(looks_like_inference_provider_endpoint(
<OPENHUMAN_ROOT>/src/api/config.rs:1277:            "https://llm.unknown-provider.example/v1"
<OPENHUMAN_ROOT>/src/api/config.rs:1279:        assert!(looks_like_inference_provider_endpoint(
<OPENHUMAN_ROOT>/src/api/config.rs:1285:    fn inference_provider_excludes_openhuman_backend_and_plain_hosts() {
<OPENHUMAN_ROOT>/src/api/config.rs:1287:        assert!(!looks_like_inference_provider_endpoint(
<OPENHUMAN_ROOT>/src/api/config.rs:1290:        assert!(!looks_like_inference_provider_endpoint(
<OPENHUMAN_ROOT>/src/api/config.rs:1293:        // A custom self-hosted OpenHuman backend (no provider host, no `/v1`
<OPENHUMAN_ROOT>/src/api/config.rs:1295:        assert!(!looks_like_inference_provider_endpoint(
<OPENHUMAN_ROOT>/src/api/config.rs:1299:        assert!(!looks_like_inference_provider_endpoint(""));
<OPENHUMAN_ROOT>/src/api/config.rs:1300:        assert!(!looks_like_inference_provider_endpoint("not a url"));
<OPENHUMAN_ROOT>/src/api/config.rs:1304:    fn backend_url_falls_back_for_remote_inference_provider_override() {
<OPENHUMAN_ROOT>/src/api/config.rs:1306:        // provider must NOT be used as the control-plane base; backend calls
<OPENHUMAN_ROOT>/src/api/config.rs:1323:    fn backend_url_falls_back_to_env_for_remote_inference_provider() {
<OPENHUMAN_ROOT>/src/api/rest.rs:18:    /// user deletes the message on the provider side (Telegram, Discord,
<OPENHUMAN_ROOT>/src/api/rest.rs:23:    #[error("message not found on {provider}: {message_id}")]
<OPENHUMAN_ROOT>/src/api/rest.rs:25:        /// Channel provider segment (e.g. `"telegram"`, `"discord"`).
<OPENHUMAN_ROOT>/src/api/rest.rs:26:        provider: String,
<OPENHUMAN_ROOT>/src/api/rest.rs:27:        /// Provider-specific message id from the URL.
<OPENHUMAN_ROOT>/src/api/rest.rs:74:/// Extract `(provider, message_id)` from a backend channel path of the
<OPENHUMAN_ROOT>/src/api/rest.rs:75:/// shape `…/channels/<provider>/messages/<id>`. Returns `None` for paths
<OPENHUMAN_ROOT>/src/api/rest.rs:314:    /// The name of the integration provider (e.g., "google", "slack").
<OPENHUMAN_ROOT>/src/api/rest.rs:315:    pub provider: String,
<OPENHUMAN_ROOT>/src/api/rest.rs:400:    /// Returns the URL for initiating a login flow for a specific provider.
<OPENHUMAN_ROOT>/src/api/rest.rs:401:    pub fn login_url(&self, provider: &str) -> Result<Url> {
<OPENHUMAN_ROOT>/src/api/rest.rs:402:        let p = provider.trim().trim_matches('/');
<OPENHUMAN_ROOT>/src/api/rest.rs:403:        anyhow::ensure!(!p.is_empty(), "provider is required");
<OPENHUMAN_ROOT>/src/api/rest.rs:409:    /// Initiates an OAuth connection flow for the current user and a specific provider.
<OPENHUMAN_ROOT>/src/api/rest.rs:412:        provider: &str,
<OPENHUMAN_ROOT>/src/api/rest.rs:418:        let p = provider.trim().trim_matches('/');
<OPENHUMAN_ROOT>/src/api/rest.rs:419:        anyhow::ensure!(!p.is_empty(), "provider is required");
<OPENHUMAN_ROOT>/src/api/rest.rs:660:            // 404 on `/channels/<provider>/messages/<id>` is an expected
<OPENHUMAN_ROOT>/src/api/rest.rs:661:            // state (user deleted the message provider-side, or backend
<OPENHUMAN_ROOT>/src/api/rest.rs:668:                if let Some((provider, message_id)) = parse_message_path(url.path()) {
<OPENHUMAN_ROOT>/src/api/rest.rs:672:                        provider = provider,
<OPENHUMAN_ROOT>/src/api/rest.rs:679:                        provider: provider.to_string(),
<OPENHUMAN_ROOT>/src/api/rest.rs:713:                && crate::openhuman::inference::provider::is_budget_exhausted_message(&text);
<OPENHUMAN_ROOT>/src/api/rest.rs:922:    /// is responsible for hitting the provider-native API.
<OPENHUMAN_ROOT>/src/main.rs:51:            // Defense-in-depth: drop transient-upstream provider failures that
<OPENHUMAN_ROOT>/src/main.rs:52:            // slipped past the call-site classifier. The reliable-provider
<OPENHUMAN_ROOT>/src/main.rs:54:            // fallback, and the aggregate "all providers exhausted" event
<OPENHUMAN_ROOT>/src/main.rs:58:            // `openhuman::inference::provider::ops::should_report_provider_http_failure`
<OPENHUMAN_ROOT>/src/main.rs:61:            if openhuman_core::core::observability::is_transient_provider_http_failure(&event) {
<OPENHUMAN_ROOT>/src/main.rs:64:            if openhuman_core::core::observability::is_all_transient_provider_exhaustion_event(
<OPENHUMAN_ROOT>/src/main.rs:79:            // (domain=llm_provider, failure=transport) — flaky-network
<OPENHUMAN_ROOT>/src/main.rs:83:            if openhuman_core::core::observability::is_transient_provider_transport_failure(&event)
<OPENHUMAN_ROOT>/src/main.rs:95:            // emit site demotes them, but the compatible provider reports the
<OPENHUMAN_ROOT>/src/main.rs:102:            // Drop provider monthly-quota exhausted events — third-party plan
<OPENHUMAN_ROOT>/src/main.rs:112:            // compatible provider can report the same `Internal Server Error
<OPENHUMAN_ROOT>/src/main.rs:122:            // `channels::providers::web::run_chat_task`. The cap is a
<OPENHUMAN_ROOT>/src/main.rs:146:            // is an expected state (provider-side delete or backend GC). Primary
<OPENHUMAN_ROOT>/src/main.rs:153:            // by llm_provider / backend_api, plus pre-flight "no session token
<OPENHUMAN_ROOT>/src/main.rs:155:            // lives at the call sites (`openhuman::inference::provider::ops::api_error`
<OPENHUMAN_ROOT>/src/bin/gmail_backfill_3d.rs:39:use openhuman_core::openhuman::composio::providers::gmail::ingest::ingest_page_into_memory_tree;
<OPENHUMAN_ROOT>/src/bin/gmail_backfill_3d.rs:40:use openhuman_core::openhuman::composio::providers::registry::{
<OPENHUMAN_ROOT>/src/bin/gmail_backfill_3d.rs:41:    get_provider, init_default_providers,
<OPENHUMAN_ROOT>/src/bin/gmail_backfill_3d.rs:140:    init_default_providers();
<OPENHUMAN_ROOT>/src/bin/gmail_backfill_3d.rs:141:    let provider = get_provider("gmail").ok_or_else(|| {
<OPENHUMAN_ROOT>/src/bin/gmail_backfill_3d.rs:142:        anyhow::anyhow!("GmailProvider not registered after init_default_providers")
<OPENHUMAN_ROOT>/src/bin/gmail_backfill_3d.rs:227:        provider.post_process_action_result("GMAIL_FETCH_EMAILS", Some(&args), &mut resp.data);
<OPENHUMAN_ROOT>/src/bin/inference_probe.rs:14://!   chat provider (no harness, no real tools). Useful to isolate
<OPENHUMAN_ROOT>/src/bin/inference_probe.rs:28://! # Raw provider call (no harness):
<OPENHUMAN_ROOT>/src/bin/inference_probe.rs:35:use openhuman_core::openhuman::inference::provider::create_chat_provider;
<OPENHUMAN_ROOT>/src/bin/inference_probe.rs:36:use openhuman_core::openhuman::inference::provider::traits::{ChatMessage, ChatRequest};
<OPENHUMAN_ROOT>/src/bin/inference_probe.rs:44:    /// "raw" — send a hand-built request directly to the chat provider.
<OPENHUMAN_ROOT>/src/bin/inference_probe.rs:48:    /// Provider role for `--mode raw`. Ignored in harness mode.
<OPENHUMAN_ROOT>/src/bin/inference_probe.rs:120:    let (provider, model_name) =
<OPENHUMAN_ROOT>/src/bin/inference_probe.rs:121:        create_chat_provider(role, config).context("create_chat_provider failed")?;
<OPENHUMAN_ROOT>/src/bin/inference_probe.rs:126:        "[probe] provider.supports_native_tools() = {}",
<OPENHUMAN_ROOT>/src/bin/inference_probe.rs:127:        provider.supports_native_tools()
<OPENHUMAN_ROOT>/src/bin/inference_probe.rs:195:    eprintln!("[probe] >>> raw provider.chat()...");
<OPENHUMAN_ROOT>/src/bin/inference_probe.rs:197:    let response = provider
<OPENHUMAN_ROOT>/src/bin/inference_probe.rs:200:        .context("provider.chat() failed")?;
<OPENHUMAN_ROOT>/src/bin/slack_backfill.rs:2://! provider.
<OPENHUMAN_ROOT>/src/bin/slack_backfill.rs:5://! `SlackProvider::sync()` for each active Slack Composio connection —
<OPENHUMAN_ROOT>/src/bin/slack_backfill.rs:30://! export RUST_LOG=info,openhuman_core::openhuman::composio::providers::slack=debug,openhuman_core::openhuman::memory=debug
<OPENHUMAN_ROOT>/src/bin/slack_backfill.rs:45:use openhuman_core::openhuman::composio::providers::registry::{
<OPENHUMAN_ROOT>/src/bin/slack_backfill.rs:46:    get_provider, init_default_providers,
<OPENHUMAN_ROOT>/src/bin/slack_backfill.rs:48:use openhuman_core::openhuman::composio::providers::slack::run_backfill_via_search;
<OPENHUMAN_ROOT>/src/bin/slack_backfill.rs:49:use openhuman_core::openhuman::composio::providers::{ProviderContext, SyncReason};
<OPENHUMAN_ROOT>/src/bin/slack_backfill.rs:86:    about = "Run SlackProvider::sync() once against the user's Composio-authorized Slack connection(s)."
<OPENHUMAN_ROOT>/src/bin/slack_backfill.rs:105:    /// only) before we consider rebuilding the provider around it.
<OPENHUMAN_ROOT>/src/bin/slack_backfill.rs:158:    // composio-side providers, including SlackProvider). Without this,
<OPENHUMAN_ROOT>/src/bin/slack_backfill.rs:188:    // from inside `SlackProvider::sync()`. `init` is idempotent and
<OPENHUMAN_ROOT>/src/bin/slack_backfill.rs:193:    // Register the default Composio providers (gmail, notion, slack).
<OPENHUMAN_ROOT>/src/bin/slack_backfill.rs:195:    init_default_providers();
<OPENHUMAN_ROOT>/src/bin/slack_backfill.rs:197:    let provider = get_provider("slack").ok_or_else(|| {
<OPENHUMAN_ROOT>/src/bin/slack_backfill.rs:198:        anyhow::anyhow!("SlackProvider not registered after init_default_providers")
<OPENHUMAN_ROOT>/src/bin/slack_backfill.rs:265:        // provider around SEARCH_MESSAGES (1 paginated call workspace-
<OPENHUMAN_ROOT>/src/bin/slack_backfill.rs:356:                    let err = r.error.as_deref().unwrap_or("provider failure");
<OPENHUMAN_ROOT>/src/bin/slack_backfill.rs:445:            // `ProviderContext` no longer caches a pre-baked client —
<OPENHUMAN_ROOT>/src/bin/slack_backfill.rs:449:            let ctx = ProviderContext {
<OPENHUMAN_ROOT>/src/bin/slack_backfill.rs:549:        let ctx = ProviderContext {
<OPENHUMAN_ROOT>/src/bin/slack_backfill.rs:557:        match provider.sync(&ctx, SyncReason::Manual).await {
<OPENHUMAN_ROOT>/src/bin/harness_subagent_audit.rs:4://! provider/backend credentials. It records only sanitized progress metadata:
<OPENHUMAN_ROOT>/AGENTS.md:148:**Provider chain** (`App.tsx`): `Sentry.ErrorBoundary` → `Redux Provider` → `PersistGate` → `BootCheckGate` → `CoreStateProvider` → `SocketProvider` → `ChatRuntimeProvider` → `HashRouter` → `CommandProvider` → `ServiceBlockingGate` → `AppShell`.
<OPENHUMAN_ROOT>/AGENTS.md:150:No `UserProvider`/`AIProvider`/`SkillProvider` — auth lives in `CoreStateProvider` via `fetchCoreAppSnapshot()` RPC.
<OPENHUMAN_ROOT>/AGENTS.md:152:**State** (`store/`): Redux Toolkit slices — `accounts`, `channelConnections`, `chatRuntime`, `coreMode`, `deepLinkAuth`, `mascot`, `notification`, `providerSurface`, `socket`, `thread`. Prefer Redux over ad-hoc `localStorage`.
<OPENHUMAN_ROOT>/AGENTS.md:164:Thin desktop host. Key modules: `core_process`, `core_rpc`, `cdp`, `cef_preflight`, `cef_profile`, `dictation_hotkeys`, `file_logging`, `mascot_native_window`, `screen_capture`, `window_state`, per-provider scanners (`discord_scanner`, `slack_scanner`, `telegram_scanner`, `whatsapp_scanner`, etc.), `meet_audio`/`meet_call`/`meet_video`, `fake_camera`, `webview_accounts`, `webview_apis`.
<OPENHUMAN_ROOT>/AGENTS.md:170:Embedded provider webviews **must not** grow new JS injection. No new `.js` under `webview_accounts/`, no new `build_init_script`/`RUNTIME_JS` blocks, no CDP `Page.addScriptToEvaluateOnNewDocument`. New behavior lives in CEF handlers, CDP from scanner modules, or Rust-side IPC hooks. Legacy injection (gmail, linkedin, google-meet) is grandfathered but should shrink. Audit new Tauri plugins for `js_init_script` calls.
<OPENHUMAN_ROOT>/AGENTS.md:178:Domains: `about_app`, `accessibility`, `agent`, `app_state`, `approval`, `autocomplete`, `billing`, `channels`, `composio`, `config`, `context`, `cost`, `credentials`, `cron`, `doctor`, `embeddings`, `encryption`, `health`, `heartbeat`, `integrations`, `learning`, `local_ai`, `meet`, `meet_agent`, `memory`, `migration`, `node_runtime`, `notifications`, `overlay`, `people`, `prompt_injection`, `provider_surfaces`, `providers`, `redirect_links`, `referral`, `routing`, `scheduler_gate`, `screen_intelligence`, `security`, `service`, `skills`, `socket`, `subconscious`, `team`, `text_input`, `threads`, `tokenjuice`, `tool_timeout`, `tools`, `tree_summarizer`, `update`, `voice`, `wallet`, `webhooks`, `webview_accounts`, `webview_apis`, `webview_notifications`.
<OPENHUMAN_ROOT>/AGENTS.md:242:- **Generated docs**: some architecture docs contain generated blocks marked `<!-- BEGIN/END GENERATED: … -->` sourced from code (today: the frontend provider chain in [`gitbooks/developing/architecture/frontend.md`](gitbooks/developing/architecture/frontend.md), from the `@generated-source:provider-chain` marker in `app/src/App.tsx`). Don't hand-edit between the markers — update the code source, then run `pnpm docs:generate`. CI (`pnpm docs:check`, the **Docs Drift** lane) fails on stale generated docs. Generator + tests: `scripts/generate-architecture-docs.mjs`.
<OPENHUMAN_ROOT>/plan.md:32:  frontend E2E (WDIO + Playwright), Rust unit (agent/memory; channels/providers/platform;
<OPENHUMAN_ROOT>/plan.md:55:| ✅ | `src/openhuman/routing/factory.rs` | `factory_constructs_without_panic_when_runtime_enabled`, `factory_llamacpp_provider_constructs_without_panic`, `factory_custom_openai_provider_constructs_without_panic`, `factory_lm_studio_provider_constructs_without_panic` | Skeptic traced the whole construction path — provably infallible (pure struct init), so the tests cannot fail. One test's comment claims to verify probe-URL selection but the body asserts nothing; fields are private so strengthening is blocked. |
<OPENHUMAN_ROOT>/plan.md:59:| ⚠️ | `src/openhuman/channels/providers/whatsapp_tests.rs` | 7 × `whatsapp_parse_<type>_message_skipped` | All hit the identical `type != "text" → continue` branch; collapse to one parameterized loop over the type strings. |
<OPENHUMAN_ROOT>/plan.md:79:| `src/openhuman/channels/providers/qq_tests.rs` | `test_name` | Sole coverage of `QQChannel::name()`, which keys routing (`routes.rs:345`) and the channel map (`runtime/startup.rs:701`). A rename would ship uncaught. |
<OPENHUMAN_ROOT>/plan.md:80:| `src/openhuman/provider_surfaces/schemas.rs` | `all_schemas_returns_two` etc. | Weak but the only guard that the registration lists are populated. **Improve** (see §3), don't delete. |
<OPENHUMAN_ROOT>/plan.md:95:| ✅ | `src/openhuman/provider_surfaces/schemas.rs::all_schemas_returns_two` / `all_controllers_returns_two` | Magic-number count breaks on any legitimate 3rd controller. | Replace with `schemas().len() == controllers().len()` parity + presence of a known op (`list_queue`). Standardize this as a shared `assert_schema_controller_parity()` helper — the `== N` pattern repeats across ~15 domains. |
<OPENHUMAN_ROOT>/plan.md:96:| ✅ | every `connector-*.spec.ts::composio_sync RPC routes to mock backend` | Name promises routing; body only asserts the session didn't crash (original assertion removed per inline comment). | Rename to what it checks, or move the real routing assertion to a native-provider connector where sync actually hits the mock. |
<OPENHUMAN_ROOT>/plan.md:179:- `socketSlice` / `channelConnectionsSlice` / `pttSlice` / `providerSurfaceSlice` reducer tests;
<OPENHUMAN_ROOT>/plan.md:300:   11 connector WDIO specs, 4 scanner-registry suites, ~4 allowlist tests × N channel providers,
<OPENHUMAN_ROOT>/plan.md:304:   declarations while *increasing* consistency (some copies have drifted, e.g. one provider's
<OPENHUMAN_ROOT>/plan.md:406:  identity split (template-compare + labeled brand-voice lock), provider_surfaces parity via
<OPENHUMAN_ROOT>/plan.md:428:  (socket/channelConnections/ptt/providerSurface) + TeamInvites. web3 tool layer has `web3_tests.rs`.
<OPENHUMAN_ROOT>/plan.md:498:  `desktop_companion`, `devices`, `announcements`, `provider_surfaces`, `people`,
<OPENHUMAN_ROOT>/src/core/jsonrpc_tests.rs:679:fn is_session_expired_error_does_not_match_byo_key_provider_401() {
<OPENHUMAN_ROOT>/src/core/jsonrpc_tests.rs:680:    // BYO-key provider 401 should not clear the user session.
<OPENHUMAN_ROOT>/src/core/jsonrpc_tests.rs:712:    // it was too broad and caught Discord/OAuth provider token errors. It is
<OPENHUMAN_ROOT>/Cargo.lock:2938: "icu_provider 2.2.0",
<OPENHUMAN_ROOT>/Cargo.lock:2958: "icu_provider 2.2.0",
<OPENHUMAN_ROOT>/Cargo.lock:2970:name = "icu_provider"
<OPENHUMAN_ROOT>/Cargo.lock:2977: "icu_provider_macros",
<OPENHUMAN_ROOT>/Cargo.lock:2987:name = "icu_provider"
<OPENHUMAN_ROOT>/Cargo.lock:3002:name = "icu_provider_macros"
<OPENHUMAN_ROOT>/Cargo.lock:3022: "icu_provider 1.5.0",
<OPENHUMAN_ROOT>/e2e/docker-compose.yml:85:  # Rust inference provider E2E tests (wiremock-based, no live LLM needed).
<OPENHUMAN_ROOT>/src/core/socketio.rs:123:    /// `"provider"` | `"openhuman_budget"` | `"agent_loop"`
<OPENHUMAN_ROOT>/src/core/socketio.rs:137:    /// Provider name extracted from `"<provider> API error (...)"`
<OPENHUMAN_ROOT>/src/core/socketio.rs:138:    /// envelopes. `None` for non-provider errors (OpenHuman budget cap,
<OPENHUMAN_ROOT>/src/core/socketio.rs:139:    /// agent loop) and for transport failures without a provider prefix.
<OPENHUMAN_ROOT>/src/core/socketio.rs:141:    pub error_provider: Option<String>,
<OPENHUMAN_ROOT>/src/core/socketio.rs:142:    /// `Some(false)` once the reliable-provider chain has exhausted
<OPENHUMAN_ROOT>/src/core/socketio.rs:183:    /// Provider-assigned tool call id that groups `tool_args_delta`
<OPENHUMAN_ROOT>/src/core/socketio.rs:503:                    match crate::openhuman::channels::providers::web::start_chat(
<OPENHUMAN_ROOT>/src/core/socketio.rs:512:                        crate::openhuman::channels::providers::web::ChatRequestMetadata::default(),
<OPENHUMAN_ROOT>/src/core/socketio.rs:554:                    let _ = crate::openhuman::channels::providers::web::cancel_chat(
<OPENHUMAN_ROOT>/src/core/socketio.rs:604:        let mut rx = crate::openhuman::channels::providers::web::subscribe_web_channel_events();
<OPENHUMAN_ROOT>/src/core/socketio.rs:967:                    provider,
<OPENHUMAN_ROOT>/src/core/socketio.rs:975:                        "provider": provider,
<OPENHUMAN_ROOT>/gitbooks/overview/troubleshooting-sign-in.md:46:On Windows the `openhuman://` URL scheme is registered to the running executable via `HKEY_CURRENT_USER\Software\Classes\openhuman\shell\open\command` at first launch. If that registration silently failed, or if the install was moved/copied after first launch, the browser cannot hand the OAuth callback back to the app, and sign-in stalls after the provider step (issue #2699).
<OPENHUMAN_ROOT>/gitbooks/overview/troubleshooting-sign-in.md:84:* The OAuth provider used.
<OPENHUMAN_ROOT>/src/core/auth.rs:56://!   `openhuman::inference::http::EXTERNAL_OPENAI_COMPAT_PROVIDER`.
<OPENHUMAN_ROOT>/src/core/auth.rs:73:use crate::openhuman::inference::http::EXTERNAL_OPENAI_COMPAT_PROVIDER;
<OPENHUMAN_ROOT>/src/core/auth.rs:382:    match auth.get_provider_bearer_token(EXTERNAL_OPENAI_COMPAT_PROVIDER, None) {
<OPENHUMAN_ROOT>/src/core/auth.rs:644:        auth.store_provider_token(
<OPENHUMAN_ROOT>/src/core/auth.rs:645:            EXTERNAL_OPENAI_COMPAT_PROVIDER,
<OPENHUMAN_ROOT>/gitbooks/legal/privacy-policy.md:62:## Service Providers
<OPENHUMAN_ROOT>/gitbooks/legal/privacy-policy.md:64:We may engage third-party service providers to support operation of the Service, such as infrastructure hosting or AI inference providers. These providers act only on our behalf, are subject to confidentiality obligations, and are restricted from using data for their own purposes.
<OPENHUMAN_ROOT>/src/core/event_bus/native_request.rs:7://! objects (`Arc<dyn Provider>`), streaming channels (`mpsc::Sender<T>`),
<OPENHUMAN_ROOT>/src/core/legacy_aliases.rs:207:        "openhuman.providers_list_models",
<OPENHUMAN_ROOT>/src/openhuman/provider_surfaces/rpc.rs:1://! RPC entry points for provider assistive surfaces.
<OPENHUMAN_ROOT>/src/openhuman/provider_surfaces/rpc.rs:3://! The first cut exposes normalized provider event ingestion plus a queue
<OPENHUMAN_ROOT>/tests/observability_smoke.rs:2://! transient-upstream provider, backend_api, integrations, and updater
<OPENHUMAN_ROOT>/tests/observability_smoke.rs:12:    is_all_transient_provider_exhaustion_event, is_budget_event, is_session_expired_event,
<OPENHUMAN_ROOT>/tests/observability_smoke.rs:14:    is_transient_integrations_failure, is_transient_provider_http_failure,
<OPENHUMAN_ROOT>/tests/observability_smoke.rs:61:            if is_transient_provider_http_failure(&event)
<OPENHUMAN_ROOT>/tests/observability_smoke.rs:62:                || is_all_transient_provider_exhaustion_event(&event)
<OPENHUMAN_ROOT>/tests/observability_smoke.rs:171:            ("domain", "llm_provider"),
<OPENHUMAN_ROOT>/tests/observability_smoke.rs:189:            ("domain", "llm_provider"),
<OPENHUMAN_ROOT>/tests/observability_smoke.rs:212:                ("domain", "llm_provider"),
<OPENHUMAN_ROOT>/tests/observability_smoke.rs:233:                ("domain", "llm_provider"),
<OPENHUMAN_ROOT>/tests/observability_smoke.rs:242:        "permanent provider failures must reach Sentry"
<OPENHUMAN_ROOT>/tests/observability_smoke.rs:263:    // `failure=all_exhausted` event when every provider/model has been
<OPENHUMAN_ROOT>/tests/observability_smoke.rs:267:        ("domain", "llm_provider"),
<OPENHUMAN_ROOT>/tests/observability_smoke.rs:283:            ("domain", "llm_provider"),
<OPENHUMAN_ROOT>/tests/observability_smoke.rs:287:        "All providers/models failed. Attempts: openai API error (503 Service Unavailable); custom_openai API error (502 Bad Gateway)",
<OPENHUMAN_ROOT>/tests/observability_smoke.rs:300:            ("domain", "llm_provider"),
<OPENHUMAN_ROOT>/tests/observability_smoke.rs:304:        "All providers/models failed. Attempts: openai API error (401 Unauthorized); custom_openai API error (503 Service Unavailable)",
<OPENHUMAN_ROOT>/tests/observability_smoke.rs:318:    let event = event_with_tags(&[("domain", "llm_provider"), ("failure", "non_2xx")]);
<OPENHUMAN_ROOT>/src/openhuman/provider_surfaces/schemas.rs:1://! Controller registry for `provider_surfaces`.
<OPENHUMAN_ROOT>/src/openhuman/provider_surfaces/schemas.rs:3://! The first cut exposes normalized provider event ingestion plus a queue
<OPENHUMAN_ROOT>/src/openhuman/provider_surfaces/schemas.rs:14:use super::types::ProviderEvent;
<OPENHUMAN_ROOT>/src/openhuman/provider_surfaces/schemas.rs:16:pub fn all_provider_surfaces_controller_schemas() -> Vec<ControllerSchema> {
<OPENHUMAN_ROOT>/src/openhuman/provider_surfaces/schemas.rs:20:pub fn all_provider_surfaces_registered_controllers() -> Vec<RegisteredController> {
<OPENHUMAN_ROOT>/src/openhuman/provider_surfaces/schemas.rs:36:            namespace: "provider_surfaces",
<OPENHUMAN_ROOT>/src/openhuman/provider_surfaces/schemas.rs:38:            description: "Ingest a normalized provider event into the local respond queue.",
<OPENHUMAN_ROOT>/src/openhuman/provider_surfaces/schemas.rs:40:                field("provider", TypeSchema::String, "Provider slug (e.g. linkedin, gmail)."),
<OPENHUMAN_ROOT>/src/openhuman/provider_surfaces/schemas.rs:41:                field("account_id", TypeSchema::String, "Provider account identifier."),
<OPENHUMAN_ROOT>/src/openhuman/provider_surfaces/schemas.rs:43:                field("entity_id", TypeSchema::String, "Stable provider entity identifier."),
<OPENHUMAN_ROOT>/src/openhuman/provider_surfaces/schemas.rs:50:                optional("deep_link", TypeSchema::String, "Provider deep link used to open the source surface."),
<OPENHUMAN_ROOT>/src/openhuman/provider_surfaces/schemas.rs:53:                    // ProviderEvent::requires_attention is #[serde(default)] so
<OPENHUMAN_ROOT>/src/openhuman/provider_surfaces/schemas.rs:63:                    comment: "Optional provider-specific raw payload for future debugging and enrichment.",
<OPENHUMAN_ROOT>/src/openhuman/provider_surfaces/schemas.rs:70:            namespace: "provider_surfaces",
<OPENHUMAN_ROOT>/src/openhuman/provider_surfaces/schemas.rs:72:            description: "List the local respond queue derived from provider events.",
<OPENHUMAN_ROOT>/src/openhuman/provider_surfaces/schemas.rs:77:            namespace: "provider_surfaces",
<OPENHUMAN_ROOT>/src/openhuman/provider_surfaces/schemas.rs:79:            description: "Unknown provider_surfaces controller.",
<OPENHUMAN_ROOT>/src/openhuman/provider_surfaces/schemas.rs:88:        let payload: ProviderEvent = parse_params(params)?;
<OPENHUMAN_ROOT>/src/openhuman/provider_surfaces/schemas.rs:140:            &all_provider_surfaces_controller_schemas(),
<OPENHUMAN_ROOT>/src/openhuman/provider_surfaces/schemas.rs:141:            &all_provider_surfaces_registered_controllers(),
<OPENHUMAN_ROOT>/src/openhuman/provider_surfaces/schemas.rs:150:        assert_eq!(schema.namespace, "provider_surfaces");
<OPENHUMAN_ROOT>/docs/tinyagents-full-migration-plan/C4-journal-progress-parity-plan.md:7:5** — deleting the `ProviderDelta` bridge and `progress_tracing`.
<OPENHUMAN_ROOT>/docs/tinyagents-full-migration-plan/C4-journal-progress-parity-plan.md:28:    `spawn_delta_forwarder` maps engine callbacks + `ProviderDelta` →
<OPENHUMAN_ROOT>/docs/tinyagents-full-migration-plan/C4-journal-progress-parity-plan.md:32:  (`channels/providers/web/progress_bridge.rs:157`) is a side-observer of the
<OPENHUMAN_ROOT>/docs/tinyagents-full-migration-plan/C4-journal-progress-parity-plan.md:54:  adapter's provider `UsageInfo` FIFO) to restore **charged USD**, cache-creation
<OPENHUMAN_ROOT>/docs/tinyagents-full-migration-plan/C4-journal-progress-parity-plan.md:73:2. **Usage accounting**: decide whether charged-USD/provider-cost belongs on a
<OPENHUMAN_ROOT>/docs/tinyagents-full-migration-plan/C4-journal-progress-parity-plan.md:74:   crate event (e.g. a `provider_cost`/`cache_creation` extension on
<OPENHUMAN_ROOT>/docs/tinyagents-full-migration-plan/C4-journal-progress-parity-plan.md:170:   the legacy `ProviderDelta` bridge in `session/tool_progress.rs` (253) and the
<OPENHUMAN_ROOT>/docs/tinyagents-full-migration-plan/C4-journal-progress-parity-plan.md:189:`channels/providers/web/progress_bridge.rs:157`. Crate foundation:
<OPENHUMAN_ROOT>/tests/agent_multimodal_public.rs:4:    prepare_messages_for_provider,
<OPENHUMAN_ROOT>/tests/agent_multimodal_public.rs:7:use openhuman_core::openhuman::inference::provider::ChatMessage;
<OPENHUMAN_ROOT>/tests/agent_multimodal_public.rs:50:    let prepared = prepare_messages_for_provider(
<OPENHUMAN_ROOT>/tests/agent_multimodal_public.rs:69:    let prepared = prepare_messages_for_provider(
<OPENHUMAN_ROOT>/tests/agent_multimodal_public.rs:88:    let err = prepare_messages_for_provider(
<OPENHUMAN_ROOT>/tests/agent_multimodal_public.rs:100:    let err = prepare_messages_for_provider(
<OPENHUMAN_ROOT>/tests/agent_multimodal_public.rs:110:    let err = prepare_messages_for_provider(
<OPENHUMAN_ROOT>/tests/agent_multimodal_public.rs:130:    let err = prepare_messages_for_provider(
<OPENHUMAN_ROOT>/gitbooks/guides/recover-failed-installation.md:69:| Sign-in stalls after the provider step; log mentions `openhuman://` scheme **not registered** (Windows) | The URL handler didn't register, or the install was moved after first launch | Follow the repair steps in [Troubleshooting Sign-In](../overview/troubleshooting-sign-in.md#windows-openhuman-handler-not-registered) |
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:85:    /// A sub-agent failed (max iterations, provider error, missing
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:216:    /// The configured embedding provider is unreachable or the requested model
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:224:        /// Short provider slug, e.g. `"ollama"`.
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:225:        provider: String,
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:229:        /// The provider that will serve embeddings for this session instead,
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:231:        fallback_provider: String,
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:238:    /// A BYO (bring-your-own-key) chat provider rejected the configured API
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:241:    /// Published by `inference::provider::ops::http_error::
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:242:    /// log_byo_provider_auth_failure` (once per failure episode, via the
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:248:    /// inline provider-error notice. The `message` field is a pre-formatted,
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:250:    ProviderApiKeyRejected {
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:251:        /// Provider slug, e.g. `"openrouter"`.
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:252:        provider: String,
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:283:    /// channel-provider ingest) — `connection_id` remains unchanged for
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:288:        provider: Option<String>,
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:367:        /// Provider-neutral envelope projected from the inbound channel message.
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:384:        /// Provider route selected for the LLM turn.
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:385:        provider: String,
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:733:        /// Provider-specific trigger payload.
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:795:        /// ran on the remote default provider.
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:858:        provider: String,
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:864:        provider: String,
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:1085:    /// `source` is a short slug (e.g. `"llm_provider.openhuman_backend"`,
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:1100:        provider: String,
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:1108:        provider: String,
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:1116:        provider: String,
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:1394:            Self::SessionExpired { .. } | Self::ProviderApiKeyRejected { .. } => "auth",
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:1562:            Self::ProviderApiKeyRejected { .. } => "ProviderApiKeyRejected",
<OPENHUMAN_ROOT>/scripts/debug/harness-subagent-audit.sh:15:echo "[harness_subagent_audit] running live audit; requires configured provider/backend credentials" >&2
<OPENHUMAN_ROOT>/docs/tinycortex-migration-spec.md:100:- **Process glue:** `memory/global.rs` singleton + queue worker; `memory/source_scope.rs` task-locals; `memory/chat.rs`; embeddings provider wiring.
<OPENHUMAN_ROOT>/docs/tinycortex-migration-spec.md:113:`embeddings.rs` (`EmbeddingBackend`/`Embedder`), `chat.rs` (`ChatProvider`/`Summariser`×2/
<OPENHUMAN_ROOT>/tests/agent_turn_builder_leftovers_raw_coverage_e2e.rs:8:use openhuman_core::openhuman::inference::provider::{
<OPENHUMAN_ROOT>/tests/agent_turn_builder_leftovers_raw_coverage_e2e.rs:9:    ChatMessage, ChatRequest, ChatResponse, Provider, ToolCall, UsageInfo,
<OPENHUMAN_ROOT>/tests/agent_turn_builder_leftovers_raw_coverage_e2e.rs:59:struct ScriptedProvider {
<OPENHUMAN_ROOT>/tests/agent_turn_builder_leftovers_raw_coverage_e2e.rs:65:impl ScriptedProvider {
<OPENHUMAN_ROOT>/tests/agent_turn_builder_leftovers_raw_coverage_e2e.rs:80:impl Provider for ScriptedProvider {
<OPENHUMAN_ROOT>/tests/agent_turn_builder_leftovers_raw_coverage_e2e.rs:83:    ) -> openhuman_core::openhuman::inference::provider::traits::ProviderCapabilities {
<OPENHUMAN_ROOT>/tests/agent_turn_builder_leftovers_raw_coverage_e2e.rs:84:        openhuman_core::openhuman::inference::provider::traits::ProviderCapabilities {
<OPENHUMAN_ROOT>/tests/agent_turn_builder_leftovers_raw_coverage_e2e.rs:367:    let provider = ScriptedProvider::new(
<OPENHUMAN_ROOT>/tests/agent_turn_builder_leftovers_raw_coverage_e2e.rs:376:        .provider_arc(provider.clone())
<OPENHUMAN_ROOT>/tests/agent_turn_builder_leftovers_raw_coverage_e2e.rs:413:    assert_eq!(provider.requests()[0].tool_names, vec!["round20_dup"]);
<OPENHUMAN_ROOT>/tests/agent_turn_builder_leftovers_raw_coverage_e2e.rs:414:    assert!(provider.requests()[1]
<OPENHUMAN_ROOT>/tests/agent_turn_builder_leftovers_raw_coverage_e2e.rs:428:    let provider = ScriptedProvider::new(
<OPENHUMAN_ROOT>/tests/agent_turn_builder_leftovers_raw_coverage_e2e.rs:437:        .provider_arc(provider)
<OPENHUMAN_ROOT>/tests/agent_turn_builder_leftovers_raw_coverage_e2e.rs:497:    let provider = ScriptedProvider::new(
<OPENHUMAN_ROOT>/tests/agent_turn_builder_leftovers_raw_coverage_e2e.rs:517:        .provider_arc(provider)
<OPENHUMAN_ROOT>/tests/agent_turn_builder_leftovers_raw_coverage_e2e.rs:562:    let empty_provider = ScriptedProvider::new(vec![text_response("   ", None)], false);
<OPENHUMAN_ROOT>/tests/agent_turn_builder_leftovers_raw_coverage_e2e.rs:564:        .provider_arc(empty_provider)
<OPENHUMAN_ROOT>/src/openhuman/provider_surfaces/ops.rs:1://! Core operations for provider assistive surfaces.
<OPENHUMAN_ROOT>/src/openhuman/provider_surfaces/ops.rs:12:use super::types::{ProviderEvent, RespondQueueItem, RespondQueueListResponse};
<OPENHUMAN_ROOT>/src/openhuman/provider_surfaces/ops.rs:46:    request: ProviderEvent,
<OPENHUMAN_ROOT>/src/openhuman/provider_surfaces/ops.rs:49:        provider = %request.provider,
<OPENHUMAN_ROOT>/src/openhuman/provider_surfaces/ops.rs:54:        "[provider-surfaces] ingest_event"
<OPENHUMAN_ROOT>/src/openhuman/provider_surfaces/ops.rs:65:    tracing::debug!(count, "[provider-surfaces] list_queue");
<OPENHUMAN_ROOT>/src/openhuman/provider_surfaces/ops.rs:81:    fn sample_event(entity_id: &str) -> ProviderEvent {
<OPENHUMAN_ROOT>/src/openhuman/provider_surfaces/ops.rs:82:        ProviderEvent {
<OPENHUMAN_ROOT>/src/openhuman/provider_surfaces/ops.rs:83:            provider: "linkedin".into(),
<OPENHUMAN_ROOT>/src/openhuman/provider_surfaces/ops.rs:111:        assert_eq!(first_result["provider"], "linkedin");
<OPENHUMAN_ROOT>/src/core/event_bus/events_tests.rs:134:                provider: "test-provider".into(),
<OPENHUMAN_ROOT>/src/core/event_bus/events_tests.rs:418:                provider: "slack".into(),
<OPENHUMAN_ROOT>/src/core/event_bus/events_tests.rs:426:                provider: "slack".into(),
<OPENHUMAN_ROOT>/src/core/event_bus/events_tests.rs:500:            DomainEvent::ProviderApiKeyRejected {
<OPENHUMAN_ROOT>/src/core/event_bus/events_tests.rs:501:                provider: "openrouter".into(),
<OPENHUMAN_ROOT>/gitbooks/guides/child-safe-companion.md:27:* Keep it local. Use a [local model](local-model.md) so conversations aren't sent to a cloud provider, and connect **no** personal integrations.
<OPENHUMAN_ROOT>/gitbooks/guides/child-safe-companion.md:57:* Turn on a [local model](local-model.md), then **route chat and reasoning at the local provider** so conversations run on-device. Enabling Local AI alone keeps only embeddings/memory local. Chat still uses the default cloud route until you point the chat/reasoning workloads at the local provider and confirm with a test message.
<OPENHUMAN_ROOT>/gitbooks/guides/child-safe-companion.md:75:* [ ] Inference is local (`ready`), so conversations aren't going to a cloud provider.
<OPENHUMAN_ROOT>/docs/tinycortex-memory-migration-plan.md:37:- LLM and embedding **compute** — abstracted behind `EmbeddingBackend`, `score::extract::ChatProvider`, `tree::summarise::Summariser` traits ("never makes a network call").
<OPENHUMAN_ROOT>/docs/tinycortex-memory-migration-plan.md:67:- **Process glue**: `memory/global.rs` singleton + background queue worker; `memory/source_scope.rs` tokio task-locals; `memory/chat.rs` (LLM adapter over `openhuman::inference`); embeddings provider wiring.
<OPENHUMAN_ROOT>/docs/tinycortex-memory-migration-plan.md:76:- `chat.rs` — `impl ChatProvider` + `impl Summariser` over `memory::chat::build_chat_provider` / `inference::provider`.
<OPENHUMAN_ROOT>/docs/tinycortex-memory-migration-plan.md:117:**W6 — Ingest.** `memory/ingest_pipeline.rs` + `memory/ingestion/` re-pointed to `tinycortex::ingest` + `score::extract` (LLM extraction via the seam's `ChatProvider`). The namespace document/graph store path stays host-side unless deliberately upstreamed (explicit decision in this workstream). `ingest_chat`/`ingest_document_with_scope` keep their signatures — 11 call sites (learning, agent harness, archivist) unchanged.
<OPENHUMAN_ROOT>/src/core/all.rs:177:        crate::openhuman::channels::providers::web::all_web_channel_registered_controllers(),
<OPENHUMAN_ROOT>/src/core/all.rs:198:    // Unified inference domain: text / vision / local runtime / cloud providers.
<OPENHUMAN_ROOT>/src/core/all.rs:199:    // (Formerly split across inference, local AI, and providers modules.)
<OPENHUMAN_ROOT>/src/core/all.rs:202:    // Embedding provider configuration and embed RPC.
<OPENHUMAN_ROOT>/src/core/all.rs:241:        crate::openhuman::composio::providers::slack::all_slack_memory_registered_controllers(),
<OPENHUMAN_ROOT>/src/core/all.rs:273:    // Local assistive surfaces over third-party provider apps
<OPENHUMAN_ROOT>/src/core/all.rs:275:        crate::openhuman::provider_surfaces::all_provider_surfaces_registered_controllers(),
<OPENHUMAN_ROOT>/src/core/all.rs:304:    // Integration notification ingest, triage, and per-provider settings
<OPENHUMAN_ROOT>/src/core/all.rs:410:        .extend(crate::openhuman::channels::providers::web::all_web_channel_controller_schemas());
<OPENHUMAN_ROOT>/src/core/all.rs:442:        crate::openhuman::composio::providers::slack::all_slack_memory_controller_schemas(),
<OPENHUMAN_ROOT>/src/core/all.rs:458:    schemas.extend(crate::openhuman::provider_surfaces::all_provider_surfaces_controller_schemas());
<OPENHUMAN_ROOT>/src/core/all.rs:479:    // Integration notification ingest, triage, and per-provider settings
<OPENHUMAN_ROOT>/src/core/all.rs:533:        "agentbox" => Some("AgentBox marketplace adapter status — mode flag and GMI MaaS provider wiring."),
<OPENHUMAN_ROOT>/src/core/all.rs:536:        "auth" => Some("Manage app session and provider credentials."),
<OPENHUMAN_ROOT>/src/core/all.rs:632:        "provider_surfaces" => Some(
<OPENHUMAN_ROOT>/src/core/all.rs:633:            "Local-first assistive surfaces for provider events, respond queues, and drafts.",
<OPENHUMAN_ROOT>/src/core/all.rs:661:             and per-provider routing settings.",
<OPENHUMAN_ROOT>/tests/subconscious_fullstack_e2e.rs:2://! run against a **mocked LLM provider** — no network, no Ollama, no real
<OPENHUMAN_ROOT>/tests/subconscious_fullstack_e2e.rs:3://! model anywhere (cloud and local provider funnels are both overridden).
<OPENHUMAN_ROOT>/tests/subconscious_fullstack_e2e.rs:6://! SessionExecutor *above* the model), this test mocks at the *provider*
<OPENHUMAN_ROOT>/tests/subconscious_fullstack_e2e.rs:8://!   - `GatePass::evaluate` → `agent::triage::run_triage` → real provider
<OPENHUMAN_ROOT>/tests/subconscious_fullstack_e2e.rs:13://! The mock is installed via the factory's `test_provider_override` seam,
<OPENHUMAN_ROOT>/tests/subconscious_fullstack_e2e.rs:14://! which both provider funnels consult first.
<OPENHUMAN_ROOT>/tests/subconscious_fullstack_e2e.rs:28:use openhuman_core::openhuman::inference::provider::factory::test_provider_override;
<OPENHUMAN_ROOT>/tests/subconscious_fullstack_e2e.rs:29:use openhuman_core::openhuman::inference::provider::traits::{
<OPENHUMAN_ROOT>/tests/subconscious_fullstack_e2e.rs:30:    ChatRequest, ChatResponse, ProviderCapabilities, ToolCall,
<OPENHUMAN_ROOT>/tests/subconscious_fullstack_e2e.rs:32:use openhuman_core::openhuman::inference::provider::Provider;
<OPENHUMAN_ROOT>/tests/subconscious_fullstack_e2e.rs:37:// Mock LLM provider — deterministic, content-routed, no network.
<OPENHUMAN_ROOT>/tests/subconscious_fullstack_e2e.rs:141:impl Provider for MockLlm {
<OPENHUMAN_ROOT>/tests/subconscious_fullstack_e2e.rs:142:    fn capabilities(&self) -> ProviderCapabilities {
<OPENHUMAN_ROOT>/tests/subconscious_fullstack_e2e.rs:143:        ProviderCapabilities {
<OPENHUMAN_ROOT>/tests/subconscious_fullstack_e2e.rs:178:// Hermetic harness: temp HOME + config + globals + installed mock provider.
<OPENHUMAN_ROOT>/tests/subconscious_fullstack_e2e.rs:182:/// process-global provider override.
<OPENHUMAN_ROOT>/tests/subconscious_fullstack_e2e.rs:218:    _install: test_provider_override::InstallGuard,
<OPENHUMAN_ROOT>/tests/subconscious_fullstack_e2e.rs:226:    // api_url is never dialed — the provider override intercepts creation
<OPENHUMAN_ROOT>/tests/subconscious_fullstack_e2e.rs:276:    // Install the mock LLM — both provider funnels consult this first.
<OPENHUMAN_ROOT>/tests/subconscious_fullstack_e2e.rs:277:    let install = test_provider_override::install(mock.clone());
<OPENHUMAN_ROOT>/tests/subconscious_fullstack_e2e.rs:396:// provider), whose output is merged by the orchestrator's follow-up turn.
<OPENHUMAN_ROOT>/src/openhuman/provider_surfaces/mod.rs:1://! Local assistive surfaces for third-party provider apps.
<OPENHUMAN_ROOT>/src/openhuman/provider_surfaces/mod.rs:4://! draft shelf, and provider-specific assistive actions that sit above
<OPENHUMAN_ROOT>/src/openhuman/provider_surfaces/mod.rs:17:    all_provider_surfaces_controller_schemas, all_provider_surfaces_registered_controllers,
<OPENHUMAN_ROOT>/vendor/tinyplace/CLAUDE.md:85:**Routing:** App Router under `website/app/`. Pages: `app/page.tsx` (home), `app/explore/` (layout + page; the explore sections — directory, profiles, messaging, events, marketplace, payments, ledger, reputation, leaderboards, stats, explorer, search — render as **internal tab views** inside the explore shell, not separate route files), `app/poker/`, `app/not-found.tsx`. Providers are injected via `app/providers.tsx` / `app/client-layout.tsx`.
<OPENHUMAN_ROOT>/vendor/tinyplace/CLAUDE.md:120:- **Solana / wallet code must be client-only.** The wallet adapter breaks under SSR, so providers are lazy/client-loaded (`app/providers.tsx`, `app/client-layout.tsx`, `ClientOnly`). Don't import wallet/`@solana/*` code into server-rendered module scope.
<OPENHUMAN_ROOT>/vendor/tinyplace/CLAUDE.md:130:- **`job_escrow`** — job lifecycle `Open → Delivered → Resolved`, with `Disputed`/`Refunded` branches, plus a server **controller** key that decides disputed outcomes (`resolve(award_provider)`). `fund` / `fund_for` deposit into the job's own token vault; `approve` / `resolve` / `refund` release funds directly to the provider/client minus rake (no rake on refund).
<OPENHUMAN_ROOT>/gitbooks/guides/move-to-new-pc.md:3:  Carry your OpenHuman persona, memory, workspace, and model/provider config to
<OPENHUMAN_ROOT>/gitbooks/guides/move-to-new-pc.md:38:| **Config** (models, providers, routing, autonomy) | `config.toml` | ✅ Yes |
<OPENHUMAN_ROOT>/gitbooks/guides/move-to-new-pc.md:85:* **Bring-your-own keys:** if you had entered your own provider API key, a Composio direct key, or similar **local** secrets, re-enter them on the new machine. Those are stored in the OS keychain and don't come across in the folder.
<OPENHUMAN_ROOT>/gitbooks/guides/move-to-new-pc.md:87:### 6. Re-check model / provider config
<OPENHUMAN_ROOT>/gitbooks/guides/move-to-new-pc.md:89:Your `config.toml` came along, so model routing and provider choices should already match. If you used a [local model](local-model.md), remember that **Ollama/LM Studio is separate software**. Install it on the new machine too, and let OpenHuman re-pull the model weights (they aren't in the data folder).
<OPENHUMAN_ROOT>/scripts/debug/harness-live-audit-cases.md:11:`--isolated-workspace` defaults to `--provider-mode openhuman-backend`. It writes a temporary backend config, seeds the temp auth store from `JWT_TOKEN`, starts `openhuman-core`, and removes the temp workspace by default. Only pass `--keep-workspace` when you need to inspect artifacts locally.
<OPENHUMAN_ROOT>/scripts/debug/harness-live-audit-cases.md:13:The optional `--provider-mode direct-openai` control path writes a temporary direct-provider config using `OPENAI_API_KEY` or `OPENAI_KEY`.
<OPENHUMAN_ROOT>/scripts/debug/harness-live-audit-cases.md:28:node scripts/debug/harness-subagent-rpc-audit.mjs --spawn-core --isolated-workspace --provider-mode direct-openai --model gpt-4.1-mini --scenario reuse-parent-comm
<OPENHUMAN_ROOT>/src/core/cli.rs:18:/// prompts, provider requests, and sub-agent tool loops.
<OPENHUMAN_ROOT>/src/core/cli.rs:290:    // hundreds of tool specs + the nested provider/tool loop), and delegating
<OPENHUMAN_ROOT>/tests/agent_retrieval_e2e.rs:9://! `agent_chat` requires a reachable provider (no provider connection
<OPENHUMAN_ROOT>/tests/agent_retrieval_e2e.rs:119:        provider: "gmail".into(),
<OPENHUMAN_ROOT>/tests/agent_retrieval_e2e.rs:345:            provider: "gmail".into(),
<OPENHUMAN_ROOT>/src/openhuman/provider_surfaces/README.md:1:# provider_surfaces
<OPENHUMAN_ROOT>/src/openhuman/provider_surfaces/README.md:3:Local assistive surfaces for third-party provider apps. This domain owns a normalized provider-event model and an in-memory **respond queue** that sits above embedded webviews (and future API-first integrations), so the assistant can surface actionable items (messages, mentions, etc.) from providers like LinkedIn/Gmail in a single local-first list. The current cut is an intentionally minimal scaffold: it exposes two RPC controllers, keeps state in process memory, and is wired into the controller registry ahead of behavioral/SQLite work.
<OPENHUMAN_ROOT>/src/openhuman/provider_surfaces/README.md:7:- Define a normalized inbound `ProviderEvent` contract (provider slug, account id, event kind, entity id, optional thread/title/snippet/sender/deep-link, timestamp, `requires_attention`, optional raw payload).
<OPENHUMAN_ROOT>/src/openhuman/provider_surfaces/README.md:8:- Ingest provider events and upsert them into a local respond queue keyed by `provider:account_id:event_kind:entity_id`.
<OPENHUMAN_ROOT>/src/openhuman/provider_surfaces/README.md:10:- Bound queue growth with a soft cap (500 items, oldest-from-tail dropped) under provider firehose volume.
<OPENHUMAN_ROOT>/src/openhuman/provider_surfaces/README.md:16:| `src/openhuman/provider_surfaces/mod.rs` | Export-only: declares submodules; re-exports `all_provider_surfaces_controller_schemas` / `all_provider_surfaces_registered_controllers`. |
<OPENHUMAN_ROOT>/src/openhuman/provider_surfaces/README.md:17:| `src/openhuman/provider_surfaces/types.rs` | Serde domain types: `ProviderEvent`, `RespondQueueItem`, `RespondQueueListResponse`. Snake_case contract shared by request and response. |
<OPENHUMAN_ROOT>/src/openhuman/provider_surfaces/README.md:18:| `src/openhuman/provider_surfaces/ops.rs` | Business logic / entry points: `ingest_event`, `list_queue`. Wrap results in `ApiEnvelope` + `RpcOutcome`. Contains the inline test suite. |
<OPENHUMAN_ROOT>/src/openhuman/provider_surfaces/README.md:19:| `src/openhuman/provider_surfaces/store.rs` | In-memory persistence: process-global `RESPOND_QUEUE` (`OnceLock<Mutex<Vec<…>>>`), `upsert_queue_item`, `list_queue_items`, `clear_queue` (test-only). |
<OPENHUMAN_ROOT>/src/openhuman/provider_surfaces/README.md:20:| `src/openhuman/provider_surfaces/schemas.rs` | Controller registry: `ControllerSchema`s + `handle_*` fns delegating to `ops.rs`. Inline schema tests. |
<OPENHUMAN_ROOT>/src/openhuman/provider_surfaces/README.md:21:| `src/openhuman/provider_surfaces/rpc.rs` | Docstring-only placeholder; no code. The handler delegation lives in `schemas.rs`, not here. |
<OPENHUMAN_ROOT>/src/openhuman/provider_surfaces/README.md:25:- `types::ProviderEvent` — inbound normalized provider event (`#[serde(deny_unknown_fields)]`).
<OPENHUMAN_ROOT>/src/openhuman/provider_surfaces/README.md:28:- `ops::ingest_event(ProviderEvent)` / `ops::list_queue(EmptyRequest)` — async handlers returning `RpcOutcome<ApiEnvelope<T>>`.
<OPENHUMAN_ROOT>/src/openhuman/provider_surfaces/README.md:30:- Re-exported from `mod.rs`: `all_provider_surfaces_controller_schemas`, `all_provider_surfaces_registered_controllers`.
<OPENHUMAN_ROOT>/src/openhuman/provider_surfaces/README.md:34:Namespace `provider_surfaces` (two controllers, registered via `src/core/all.rs`):
<OPENHUMAN_ROOT>/src/openhuman/provider_surfaces/README.md:38:| `provider_surfaces.ingest_event` | `provider`, `account_id`, `event_kind`, `entity_id`, `timestamp` (required); `thread_id`, `title`, `snippet`, `sender_name`, `sender_handle`, `deep_link` (optional); `requires_attention` (bool, defaults false), `raw_payload` (optional JSON) | Envelope containing the upserted `RespondQueueItem`. |
<OPENHUMAN_ROOT>/src/openhuman/provider_surfaces/README.md:39:| `provider_surfaces.list_queue` | none | Envelope containing `{ items, count }`. |
<OPENHUMAN_ROOT>/src/openhuman/provider_surfaces/README.md:41:`requires_attention` is `required: false` in the schema to match `ProviderEvent`'s `#[serde(default)]` so registry `validate_params` agrees with the struct.
<OPENHUMAN_ROOT>/src/openhuman/provider_surfaces/README.md:53:In-memory only. State lives in a process-global `RESPOND_QUEUE` (`static OnceLock<Mutex<Vec<RespondQueueItem>>>`) in `store.rs`, prepend-ordered (newest-first), soft-capped at `MAX_QUEUE_ITEMS = 500` (oldest dropped from the tail). Upsert dedupes by composite id `provider:account_id:event_kind:entity_id`. Module docstrings flag SQLite-backed persistence for normalized events, queue state, and local drafts as follow-up work — not yet present.
<OPENHUMAN_ROOT>/src/openhuman/provider_surfaces/README.md:61:- External crates: `serde` / `serde_json`, `uuid` (request ids), `tracing` (debug logging with `[provider-surfaces]` prefix).
<OPENHUMAN_ROOT>/src/openhuman/provider_surfaces/README.md:65:- `src/core/all.rs` — registers the controllers/schemas into the global registry (`all_provider_surfaces_registered_controllers`, `all_provider_surfaces_controller_schemas`, and a `"provider_surfaces"` dispatch arm).
<OPENHUMAN_ROOT>/src/openhuman/provider_surfaces/README.md:71:- **Scaffold, not finished domain.** Docstrings across `mod.rs`/`ops.rs`/`store.rs` explicitly call this an initial cut: state is in-memory, SQLite store + drafts + provider-specific assistive actions are deferred.
<OPENHUMAN_ROOT>/src/openhuman/provider_surfaces/README.md:73:- **Queue id is deterministic** (`provider:account_id:event_kind:entity_id`), so re-ingesting the same entity upserts (removes + re-prepends) rather than duplicating.
<OPENHUMAN_ROOT>/src/openhuman/provider_surfaces/README.md:75:- **Snake_case contract is intentional and shared** between request (`ProviderEvent`) and response (`RespondQueueItem`) so callers see one consistent shape.
<OPENHUMAN_ROOT>/src/core/runtime.rs:4://! hundreds of tool specs + the nested provider/tool loop), and delegating
<OPENHUMAN_ROOT>/scripts/debug/goals-live-cases.md:8:> configured provider/model + credentials in the target workspace.
<OPENHUMAN_ROOT>/scripts/debug/goals-live-cases.md:13:- For `reflect`: a workspace with provider creds (use your real workspace —
<OPENHUMAN_ROOT>/src/openhuman/provider_surfaces/store.rs:1://! Persistence for provider assistive surfaces.
<OPENHUMAN_ROOT>/src/openhuman/provider_surfaces/store.rs:3://! Follow-up work will add a SQLite-backed store for normalized provider
<OPENHUMAN_ROOT>/src/openhuman/provider_surfaces/store.rs:8:use crate::openhuman::provider_surfaces::types::{ProviderEvent, RespondQueueItem};
<OPENHUMAN_ROOT>/src/openhuman/provider_surfaces/store.rs:10:/// Soft cap on the in-memory respond queue to bound growth under provider
<OPENHUMAN_ROOT>/src/openhuman/provider_surfaces/store.rs:27:fn queue_item_id(event: &ProviderEvent) -> String {
<OPENHUMAN_ROOT>/src/openhuman/provider_surfaces/store.rs:30:        event.provider, event.account_id, event.event_kind, event.entity_id
<OPENHUMAN_ROOT>/src/openhuman/provider_surfaces/store.rs:34:pub fn upsert_queue_item(event: ProviderEvent) -> RespondQueueItem {
<OPENHUMAN_ROOT>/src/openhuman/provider_surfaces/store.rs:37:        provider: event.provider,
<OPENHUMAN_ROOT>/docs/TEST-COVERAGE-MATRIX.md:61:### 1.1 Multi-Provider Authentication
<OPENHUMAN_ROOT>/docs/TEST-COVERAGE-MATRIX.md:67:| 1.1.3 | Twitter (X) Login | WD    | `login-flow.spec.ts`                    | 🟡     | Generic OAuth path; assert provider tag in #968 |
<OPENHUMAN_ROOT>/docs/TEST-COVERAGE-MATRIX.md:75:| 1.2.2 | Multi-Provider Linking     | WD    | _missing_ — tracked #968                      | ❌     | Need spec linking 4 providers to one account |
<OPENHUMAN_ROOT>/docs/TEST-COVERAGE-MATRIX.md:164:| 3.3.3.4 | Provider Selection Persistence | RU+RI+VU | `src/openhuman/config/ops_tests.rs`, `tests/json_rpc_e2e.rs`, `app/src/utils/tauriCommands/config.test.ts` | ✅     | Covers `lm_studio` normalization and config round-trip |
<OPENHUMAN_ROOT>/docs/TEST-COVERAGE-MATRIX.md:184:| 4.2.3 | Streaming Responses                                                | RI       | `tests/json_rpc_e2e.rs`, `tests/agent_harness_e2e.rs`                                                                                                                                                                                                                | ✅     | `tests/agent_harness_e2e.rs` adds provider-level SSE tool-arg accumulation (chunked args reassembled + parsed) and engine-level delta forwarding (#3471)                                                                                                                                                                                                                                                                                                                                                                                    |
<OPENHUMAN_ROOT>/docs/TEST-COVERAGE-MATRIX.md:185:| 4.2.4 | Parallel inference (cross-thread + within-thread forked turns)     | RU+VU    | `src/openhuman/channels/providers/web_tests.rs`, `app/src/store/__tests__/chatRuntimeSlice.test.ts`, `app/src/providers/__tests__/ChatRuntimeProvider.test.tsx`                                                                                                      | 🟡     | Concurrent same-/cross-thread dispatch, cooperative `CancellationToken` teardown, and parallel-lane stream routing covered; dedicated WD E2E is a follow-up                                                                                                                                                                                                                                                                                                                                                                                 |
<OPENHUMAN_ROOT>/docs/TEST-COVERAGE-MATRIX.md:188:| 4.2.7 | Plan-mode review (Approve / Reject / Send-feedback before execute) | RU+RI+VU | `src/openhuman/plan_review/gate.rs`, `src/openhuman/plan_review/tool.rs`, `src/openhuman/plan_review/schemas.rs`, `tests/json_rpc_e2e.rs`, `app/src/pages/conversations/components/PlanReviewCard.test.tsx`, `app/src/pages/__tests__/Conversations.render.test.tsx` | ✅     | Interactive turns call `request_plan_review`, which parks the LIVE turn on the in-memory `PlanReviewGate` (oneshot) until the user decides; `plan_review_request` socket event drives `PlanReviewCard`, which resolves via `openhuman.plan_review_decide` (approve resumes-and-executes / reject resumes-and-stops / revise resumes-with-feedback). RU covers gate park/resolve/timeout + tool auto-approve + parking; RI covers the decide RPC; VU covers the card + provider wiring. WD E2E (agent-driven park flow) tracked as follow-up |
<OPENHUMAN_ROOT>/docs/TEST-COVERAGE-MATRIX.md:219:| 4.4.10 | Provider Error Retry                    | RI    | `tests/agent_harness_e2e.rs`                                                                                                 | ✅     | First upstream 500 retried by ReliableProvider; second succeeds                                                 |
<OPENHUMAN_ROOT>/docs/TEST-COVERAGE-MATRIX.md:287:| 6.3.3 | Steer lands in child history                                           | RU    | `src/openhuman/agent/harness/subagent_runner/ops_tests.rs::run_queue_steer_lands_in_subagent_history`                                                                                                                                                                                                                             | ✅     | End-to-end: a queued steer is drained by the child `run_turn_engine` and appears as a `[User steering message]` user turn in the provider request.                                                                                                                                           |
<OPENHUMAN_ROOT>/docs/TEST-COVERAGE-MATRIX.md:291:| 6.3.7 | Full-stack trigger pipeline with mocked LLM                            | RI    | `tests/subconscious_fullstack_e2e.rs` (feature `e2e-test-support`)                                                                                                                                                                                                                                                                | ✅     | Real `GatePass`+`LongLivedSession`+`Agent`+sub-agent run against a provider-layer mock (no network); promote/drop, persistence, real `spawn_subagent`.                                                                                                                                       |
<OPENHUMAN_ROOT>/docs/TEST-COVERAGE-MATRIX.md:293:| 6.3.9 | Vision sub-agent reads attached images                                 | RU    | `src/openhuman/agent_registry/agents/loader.rs::vision_agent_loads_on_vision_hint`, `src/openhuman/inference/provider/factory_tests.rs::vision_tier_is_vision_capable`, `src/openhuman/agent/harness/engine/core.rs::gate_tests`, `src/openhuman/agent/multimodal_tests.rs::extract_image_placeholders_pulls_att_tokens_in_order` | ✅     | Orchestrator (non-vision `chat-v1`) keeps the image as a placeholder, delegates to `vision_agent` on the `vision-v1` tier, which rehydrates the on-disk attachment and reads it. Engine gate prefers per-tier `current_model_vision`; turn placeholders forwarded into the sub-agent prompt. |
<OPENHUMAN_ROOT>/docs/TEST-COVERAGE-MATRIX.md:402:| 10.1.5 | Yuanbao Connection  | RU    | `src/openhuman/channels/providers/yuanbao/`, `src/openhuman/channels/controllers/ops.rs::tests::connect_yuanbao_*`, `src/openhuman/channels/runtime/startup.rs::yuanbao_secret_tests` | 🟡     | New API-key channel for Tencent Yuanbao. RU covers sign-token preflight (valid/invalid creds, env-override cluster routing), credentials store hydration (incl. stale app_key guard), and WS reconnect/shutdown. No WDIO spec yet — connect-flow UI is rendered via the generic `ChannelSetupModal` already exercised by other channel flow specs. |
<OPENHUMAN_ROOT>/docs/TEST-COVERAGE-MATRIX.md:526:| 13.1.4 | Wallet Balances Panel               | VU         | `app/src/components/settings/panels/__tests__/WalletBalancesPanel.test.tsx`, `app/src/services/walletApi.test.ts`                                                           | ✅     | Loading/error/empty/loaded states; Retry + Refresh re-invocation; chain badges; truncated address; providerStatus chip                                                                                                                                                           |
<OPENHUMAN_ROOT>/docs/TEST-COVERAGE-MATRIX.md:565:| 13.6.1 | Command Palette (⌘K / ⌘P)                          | VU+WD | `app/src/lib/commands/__tests__/globalActions.test.tsx`, `app/src/components/commands/__tests__/CommandProvider.test.tsx`, `app/test/e2e/specs/command-palette.spec.ts`         | ✅     | Opens via ⌘K, runs an action, lists seed nav actions, Esc closes                                                                                                    |
<OPENHUMAN_ROOT>/docs/TEST-COVERAGE-MATRIX.md:566:| 13.6.2 | Keyboard Shortcuts help directory (? / ⌘/)         | VU+WD | `app/src/components/shortcuts/__tests__/shortcutsView.test.tsx`, `app/src/components/commands/__tests__/CommandProvider.test.tsx`, `app/test/e2e/specs/command-palette.spec.ts` | ✅     | Registry-driven grouped list; opens via `?` and ⌘/ (mutually exclusive with palette); also reachable from the sidebar shortcut icon + Settings → Keyboard Shortcuts |
<OPENHUMAN_ROOT>/src/core/jsonrpc.rs:148:                // Downstream call (backend_api / integrations / provider) already
<OPENHUMAN_ROOT>/src/core/jsonrpc.rs:155:                // (backend / provider response) and can carry URL fragments,
<OPENHUMAN_ROOT>/src/core/jsonrpc.rs:156:                // query params, or pasted-through provider error text that
<OPENHUMAN_ROOT>/src/core/jsonrpc.rs:159:                let redacted = crate::openhuman::inference::provider::ops::sanitize_api_error(
<OPENHUMAN_ROOT>/src/core/jsonrpc.rs:237:    // the UI. Generic downstream/provider 401s must stay recoverable errors;
<OPENHUMAN_ROOT>/src/core/jsonrpc.rs:240:        let sanitized_reason = crate::openhuman::inference::provider::ops::sanitize_api_error(msg);
<OPENHUMAN_ROOT>/src/core/jsonrpc.rs:247:            // pasted-through provider replies. `sanitize_api_error` runs
<OPENHUMAN_ROOT>/src/core/jsonrpc.rs:280:/// downstream provider 401s (Discord bot token failures, BYO-key OpenAI /
<OPENHUMAN_ROOT>/src/core/jsonrpc.rs:288:/// - **Provider / downstream 401s** (`api_error` in
<OPENHUMAN_ROOT>/src/core/jsonrpc.rs:289:///   `src/openhuman/inference/provider/ops.rs`): formatted as
<OPENHUMAN_ROOT>/src/core/jsonrpc.rs:290:///   `"{ProviderName} API error (401 Unauthorized): {body}"` or
<OPENHUMAN_ROOT>/src/core/jsonrpc.rs:292:///   provider name, NOT an HTTP method verb.
<OPENHUMAN_ROOT>/src/core/jsonrpc.rs:302:/// - Provider-prefixed 401s (`"Discord API error: ..."`, `"OpenAI API error ..."`)
<OPENHUMAN_ROOT>/src/core/jsonrpc.rs:303:/// - `"invalid token"` — too broad; also matches Discord / OAuth provider tokens.
<OPENHUMAN_ROOT>/src/core/jsonrpc.rs:306:/// `inference/provider/ops.rs` lines 479–497) ALREADY publishes `SessionExpired`
<OPENHUMAN_ROOT>/src/core/jsonrpc.rs:319:    // The HTTP-method prefix distinguishes these from provider-prefixed errors.
<OPENHUMAN_ROOT>/src/core/jsonrpc.rs:340:/// either of which can come from BYO-key providers, Composio, channels, or
<OPENHUMAN_ROOT>/src/core/jsonrpc.rs:342:/// at the `invoke_method` call site so provider auth failures are visible
<OPENHUMAN_ROOT>/src/core/jsonrpc.rs:1129:        // exceeded" before anything reached the provider (issue #3205). The
<OPENHUMAN_ROOT>/src/core/jsonrpc.rs:1482:    let rx = crate::openhuman::channels::providers::web::subscribe_web_channel_events();
<OPENHUMAN_ROOT>/src/core/jsonrpc.rs:1755:    // AgentBox GMI MaaS provider bridge — no-op when env vars absent.
<OPENHUMAN_ROOT>/src/core/jsonrpc.rs:1760:    crate::openhuman::agentbox::register_gmi_provider_if_present();
<OPENHUMAN_ROOT>/src/core/jsonrpc.rs:1788:    // Initialize the global MemoryClient so composio providers
<OPENHUMAN_ROOT>/src/core/jsonrpc.rs:2523:    // surface (`openhuman.cost_get_dashboard`) and `record_provider_usage`
<OPENHUMAN_ROOT>/src/core/jsonrpc.rs:2655:    crate::openhuman::channels::providers::web::register_approval_surface_subscriber();
<OPENHUMAN_ROOT>/src/core/jsonrpc.rs:2675:        crate::openhuman::channels::providers::web::register_artifact_surface_subscriber();
<OPENHUMAN_ROOT>/src/core/jsonrpc.rs:2695:    crate::openhuman::channels::providers::web::register_artifact_surface_subscriber();
<OPENHUMAN_ROOT>/scripts/debug/goals-live.mjs:44:  --isolated-workspace With --spawn-core, use a throwaway temp workspace (needs provider creds in env to enrich)
<OPENHUMAN_ROOT>/scripts/debug/goals-live.mjs:331:    console.log("  (no transcript usage recorded — provider may not have emitted usage)");
<OPENHUMAN_ROOT>/scripts/debug/goals-live.mjs:507:      "  (enrichment needs a configured provider/model + the goals_agent definition)",
<OPENHUMAN_ROOT>/tests/monitor_agent_e2e.rs:10:use openhuman_core::openhuman::inference::provider::thread_context::with_thread_id;
<OPENHUMAN_ROOT>/tests/monitor_agent_e2e.rs:11:use openhuman_core::openhuman::inference::provider::traits::ProviderCapabilities;
<OPENHUMAN_ROOT>/tests/monitor_agent_e2e.rs:12:use openhuman_core::openhuman::inference::provider::{
<OPENHUMAN_ROOT>/tests/monitor_agent_e2e.rs:13:    ChatMessage, ChatRequest, ChatResponse, Provider, ToolCall,
<OPENHUMAN_ROOT>/tests/monitor_agent_e2e.rs:33:enum ProviderStep {
<OPENHUMAN_ROOT>/tests/monitor_agent_e2e.rs:45:struct ScriptedProvider {
<OPENHUMAN_ROOT>/tests/monitor_agent_e2e.rs:46:    steps: Mutex<VecDeque<ProviderStep>>,
<OPENHUMAN_ROOT>/tests/monitor_agent_e2e.rs:55:impl ScriptedProvider {
<OPENHUMAN_ROOT>/tests/monitor_agent_e2e.rs:56:    fn new(steps: Vec<ProviderStep>) -> Arc<Self> {
<OPENHUMAN_ROOT>/tests/monitor_agent_e2e.rs:69:impl Provider for ScriptedProvider {
<OPENHUMAN_ROOT>/tests/monitor_agent_e2e.rs:70:    fn capabilities(&self) -> ProviderCapabilities {
<OPENHUMAN_ROOT>/tests/monitor_agent_e2e.rs:71:        ProviderCapabilities {
<OPENHUMAN_ROOT>/tests/monitor_agent_e2e.rs:104:            Some(ProviderStep::Static(response)) => Ok(response),
<OPENHUMAN_ROOT>/tests/monitor_agent_e2e.rs:105:            Some(ProviderStep::Delayed(delay, response)) => {
<OPENHUMAN_ROOT>/tests/monitor_agent_e2e.rs:109:            Some(ProviderStep::FromHistory(factory)) => Ok(factory(&messages)),
<OPENHUMAN_ROOT>/tests/monitor_agent_e2e.rs:273:    provider: Arc<ScriptedProvider>,
<OPENHUMAN_ROOT>/tests/monitor_agent_e2e.rs:278:        .provider_arc(provider)
<OPENHUMAN_ROOT>/tests/monitor_agent_e2e.rs:305:    provider: Arc<ScriptedProvider>,
<OPENHUMAN_ROOT>/tests/monitor_agent_e2e.rs:311:        provider,
<OPENHUMAN_ROOT>/tests/monitor_agent_e2e.rs:329:    let provider = ScriptedProvider::new(vec![
<OPENHUMAN_ROOT>/tests/monitor_agent_e2e.rs:330:        ProviderStep::Static(tool_response(
<OPENHUMAN_ROOT>/tests/monitor_agent_e2e.rs:340:        ProviderStep::Delayed(
<OPENHUMAN_ROOT>/tests/monitor_agent_e2e.rs:344:        ProviderStep::FromHistory(Box::new(|messages| {
<OPENHUMAN_ROOT>/tests/monitor_agent_e2e.rs:358:    let answer = run_monitor_turn(&tmp, provider.clone(), AutonomyLevel::Supervised, 5).await;
<OPENHUMAN_ROOT>/tests/monitor_agent_e2e.rs:361:    let requests = provider.requests();
<OPENHUMAN_ROOT>/tests/monitor_agent_e2e.rs:375:    let provider = ScriptedProvider::new(vec![
<OPENHUMAN_ROOT>/tests/monitor_agent_e2e.rs:376:        ProviderStep::Static(tool_response(
<OPENHUMAN_ROOT>/tests/monitor_agent_e2e.rs:386:        ProviderStep::Delayed(
<OPENHUMAN_ROOT>/tests/monitor_agent_e2e.rs:390:        ProviderStep::FromHistory(Box::new(|messages| {
<OPENHUMAN_ROOT>/tests/monitor_agent_e2e.rs:398:        ProviderStep::FromHistory(Box::new(|messages| {
<OPENHUMAN_ROOT>/tests/monitor_agent_e2e.rs:405:    let answer = run_monitor_turn(&tmp, provider, AutonomyLevel::Supervised, 6).await;
<OPENHUMAN_ROOT>/tests/monitor_agent_e2e.rs:414:    let provider = ScriptedProvider::new(vec![
<OPENHUMAN_ROOT>/tests/monitor_agent_e2e.rs:415:        ProviderStep::Static(tool_response(
<OPENHUMAN_ROOT>/tests/monitor_agent_e2e.rs:425:        ProviderStep::FromHistory(Box::new(|messages| {
<OPENHUMAN_ROOT>/tests/monitor_agent_e2e.rs:433:        ProviderStep::FromHistory(Box::new(|messages| {
<OPENHUMAN_ROOT>/tests/monitor_agent_e2e.rs:443:    let answer = run_monitor_turn(&tmp, provider, AutonomyLevel::Supervised, 5).await;
<OPENHUMAN_ROOT>/tests/monitor_agent_e2e.rs:452:    let provider = ScriptedProvider::new(vec![
<OPENHUMAN_ROOT>/tests/monitor_agent_e2e.rs:453:        ProviderStep::Static(tool_response(
<OPENHUMAN_ROOT>/tests/monitor_agent_e2e.rs:463:        ProviderStep::Delayed(
<OPENHUMAN_ROOT>/tests/monitor_agent_e2e.rs:467:        ProviderStep::FromHistory(Box::new(|messages| {
<OPENHUMAN_ROOT>/tests/monitor_agent_e2e.rs:478:    let answer = run_monitor_turn(&tmp, provider, AutonomyLevel::Supervised, 5).await;
<OPENHUMAN_ROOT>/tests/monitor_agent_e2e.rs:487:    let provider = ScriptedProvider::new(vec![
<OPENHUMAN_ROOT>/tests/monitor_agent_e2e.rs:488:        ProviderStep::Static(tool_response(
