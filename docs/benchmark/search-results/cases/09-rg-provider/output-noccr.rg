[search: 500 match(es) across 20 file(s) · top 5-12 per file (adaptive) · full set via retrieve footer]
<OPENHUMAN_ROOT>/src/api/jwt.rs:7:pub use crate::openhuman::credentials::{APP_SESSION_PROVIDER, DEFAULT_AUTH_PROFILE_NAME};
<OPENHUMAN_ROOT>/src/api/rest_tests.rs:330:        provider,
<OPENHUMAN_ROOT>/src/api/rest_tests.rs:336:    assert_eq!(provider, "telegram");
<OPENHUMAN_ROOT>/src/api/rest_tests.rs:339:    // Discord path — proves the helper is provider-agnostic.
<OPENHUMAN_ROOT>/src/api/rest_tests.rs:351:        provider,
<OPENHUMAN_ROOT>/src/api/rest_tests.rs:357:    assert_eq!(provider, "discord");
<OPENHUMAN_ROOT>/src/api/rest_tests.rs:573:        provider: "telegram".to_string(),
<OPENHUMAN_ROOT>/src/api/rest_tests.rs:616:    // 404 on a non-`/channels/<provider>/messages/<id>` path should NOT be
[+3 more match(es) in <OPENHUMAN_ROOT>/src/api/rest_tests.rs]
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
[+34 more match(es) in <OPENHUMAN_ROOT>/src/api/config.rs]
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
[+12 more match(es) in <OPENHUMAN_ROOT>/src/api/rest.rs]
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
<OPENHUMAN_ROOT>/src/main.rs:155:            // lives at the call sites (`openhuman::inference::provider::ops::api_error`
[+3 more match(es) in <OPENHUMAN_ROOT>/src/main.rs]
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
<OPENHUMAN_ROOT>/src/bin/inference_probe.rs:200:        .context("provider.chat() failed")?;
[+1 more match(es) in <OPENHUMAN_ROOT>/src/bin/inference_probe.rs]
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
<OPENHUMAN_ROOT>/src/bin/slack_backfill.rs:356:                    let err = r.error.as_deref().unwrap_or("provider failure");
[+9 more match(es) in <OPENHUMAN_ROOT>/src/bin/slack_backfill.rs]
<OPENHUMAN_ROOT>/src/bin/harness_subagent_audit.rs:4://! provider/backend credentials. It records only sanitized progress metadata:
<OPENHUMAN_ROOT>/src/core/jsonrpc_tests.rs:679:fn is_session_expired_error_does_not_match_byo_key_provider_401() {
<OPENHUMAN_ROOT>/src/core/jsonrpc_tests.rs:680:    // BYO-key provider 401 should not clear the user session.
<OPENHUMAN_ROOT>/src/core/jsonrpc_tests.rs:712:    // it was too broad and caught Discord/OAuth provider token errors. It is
<OPENHUMAN_ROOT>/src/core/runtime.rs:4://! hundreds of tool specs + the nested provider/tool loop), and delegating
<OPENHUMAN_ROOT>/src/core/event_bus/events_tests.rs:134:                provider: "test-provider".into(),
<OPENHUMAN_ROOT>/src/core/event_bus/events_tests.rs:418:                provider: "slack".into(),
<OPENHUMAN_ROOT>/src/core/event_bus/events_tests.rs:426:                provider: "slack".into(),
<OPENHUMAN_ROOT>/src/core/event_bus/events_tests.rs:500:            DomainEvent::ProviderApiKeyRejected {
<OPENHUMAN_ROOT>/src/core/event_bus/events_tests.rs:501:                provider: "openrouter".into(),
<OPENHUMAN_ROOT>/src/core/jsonrpc.rs:148:                // Downstream call (backend_api / integrations / provider) already
<OPENHUMAN_ROOT>/src/core/jsonrpc.rs:156:                // query params, or pasted-through provider error text that
<OPENHUMAN_ROOT>/src/core/jsonrpc.rs:159:                let redacted = crate::openhuman::inference::provider::ops::sanitize_api_error(
<OPENHUMAN_ROOT>/src/core/jsonrpc.rs:237:    // the UI. Generic downstream/provider 401s must stay recoverable errors;
<OPENHUMAN_ROOT>/src/core/jsonrpc.rs:240:        let sanitized_reason = crate::openhuman::inference::provider::ops::sanitize_api_error(msg);
<OPENHUMAN_ROOT>/src/core/jsonrpc.rs:247:            // pasted-through provider replies. `sanitize_api_error` runs
<OPENHUMAN_ROOT>/src/core/jsonrpc.rs:280:/// downstream provider 401s (Discord bot token failures, BYO-key OpenAI /
<OPENHUMAN_ROOT>/src/core/jsonrpc.rs:288:/// - **Provider / downstream 401s** (`api_error` in
<OPENHUMAN_ROOT>/src/core/jsonrpc.rs:290:///   `"{ProviderName} API error (401 Unauthorized): {body}"` or
<OPENHUMAN_ROOT>/src/core/jsonrpc.rs:302:/// - Provider-prefixed 401s (`"Discord API error: ..."`, `"OpenAI API error ..."`)
<OPENHUMAN_ROOT>/src/core/jsonrpc.rs:319:    // The HTTP-method prefix distinguishes these from provider-prefixed errors.
<OPENHUMAN_ROOT>/src/core/jsonrpc.rs:342:/// at the `invoke_method` call site so provider auth failures are visible
[+15 more match(es) in <OPENHUMAN_ROOT>/src/core/jsonrpc.rs]
<OPENHUMAN_ROOT>/src/core/legacy_aliases.rs:207:        "openhuman.providers_list_models",
<OPENHUMAN_ROOT>/src/core/cli.rs:18:/// prompts, provider requests, and sub-agent tool loops.
<OPENHUMAN_ROOT>/src/core/cli.rs:290:    // hundreds of tool specs + the nested provider/tool loop), and delegating
<OPENHUMAN_ROOT>/src/core/subconscious_cli.rs:123:                eprintln!("[subconscious] session token found — provider available");
<OPENHUMAN_ROOT>/src/core/subconscious_cli.rs:126:                eprintln!("[subconscious] WARNING: no session token — cloud provider will fail");
<OPENHUMAN_ROOT>/src/core/subconscious_cli.rs:134:        // Check provider availability
<OPENHUMAN_ROOT>/src/core/subconscious_cli.rs:136:            crate::openhuman::subconscious::provider::subconscious_provider_unavailable_reason(
<OPENHUMAN_ROOT>/src/core/subconscious_cli.rs:140:            eprintln!("[subconscious] provider unavailable: {reason}");
[+5 more match(es) in <OPENHUMAN_ROOT>/src/core/subconscious_cli.rs]
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
[+1 more match(es) in <OPENHUMAN_ROOT>/plan.md]
<OPENHUMAN_ROOT>/src/core/agent_cli.rs:4://! This is intentionally scoped to *debugging*: no execution, no provider
<OPENHUMAN_ROOT>/src/core/socketio.rs:123:    /// `"provider"` | `"openhuman_budget"` | `"agent_loop"`
<OPENHUMAN_ROOT>/src/core/socketio.rs:137:    /// Provider name extracted from `"<provider> API error (...)"`
<OPENHUMAN_ROOT>/src/core/socketio.rs:138:    /// envelopes. `None` for non-provider errors (OpenHuman budget cap,
<OPENHUMAN_ROOT>/src/core/socketio.rs:139:    /// agent loop) and for transport failures without a provider prefix.
<OPENHUMAN_ROOT>/src/core/socketio.rs:141:    pub error_provider: Option<String>,
<OPENHUMAN_ROOT>/src/core/socketio.rs:142:    /// `Some(false)` once the reliable-provider chain has exhausted
<OPENHUMAN_ROOT>/src/core/socketio.rs:183:    /// Provider-assigned tool call id that groups `tool_args_delta`
[+6 more match(es) in <OPENHUMAN_ROOT>/src/core/socketio.rs]
<OPENHUMAN_ROOT>/src/core/observability.rs:34:/// (`openhuman::inference::provider::ops::should_report_provider_http_failure`) and the
<OPENHUMAN_ROOT>/src/core/observability.rs:35:/// `before_send` filter (`is_transient_provider_http_failure`). Update here
<OPENHUMAN_ROOT>/src/core/observability.rs:99:    /// HTTP layer (`providers::ops::api_error`) already demotes its own
<OPENHUMAN_ROOT>/src/core/observability.rs:196:    /// `AgentError::EmptyProviderResponse` + `AgentError::skips_sentry()`
<OPENHUMAN_ROOT>/src/core/observability.rs:210:    /// any future channel provider) whose error chain contains `"model
<OPENHUMAN_ROOT>/src/core/observability.rs:295:    /// [`crate::openhuman::inference::provider::backend_error_code_skips_sentry`].
<OPENHUMAN_ROOT>/src/core/observability.rs:299:    /// HTML page** instead of the provider's JSON error envelope — the
<OPENHUMAN_ROOT>/src/core/observability.rs:367:    if crate::openhuman::inference::provider::managed_error_skips_sentry(message) {
<OPENHUMAN_ROOT>/src/core/observability.rs:465:        return Some(ExpectedErrorKind::ProviderUserState);
<OPENHUMAN_ROOT>/src/core/observability.rs:484:    // Check `is_provider_user_state_message` BEFORE `is_backend_user_error_message`:
<OPENHUMAN_ROOT>/src/core/observability.rs:489:        return Some(ExpectedErrorKind::ProviderUserState);
<OPENHUMAN_ROOT>/src/core/observability.rs:493:    // (`provider::ops::api_error` / `chat_via_responses`) already demotes its
[+274 more match(es) in <OPENHUMAN_ROOT>/src/core/observability.rs]