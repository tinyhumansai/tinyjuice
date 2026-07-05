[search: 500 match(es) across 50 file(s) · top 5 per file · full set via retrieve footer]
<OPENHUMAN_ROOT>/src/api/jwt.rs:7:pub use crate::openhuman::credentials::{APP_SESSION_PROVIDER, DEFAULT_AUTH_PROFILE_NAME};
<OPENHUMAN_ROOT>/src/api/rest_tests.rs:330:        provider,
<OPENHUMAN_ROOT>/src/api/rest_tests.rs:336:    assert_eq!(provider, "telegram");
<OPENHUMAN_ROOT>/src/api/rest_tests.rs:339:    // Discord path — proves the helper is provider-agnostic.
<OPENHUMAN_ROOT>/src/api/rest_tests.rs:351:        provider,
<OPENHUMAN_ROOT>/src/api/rest_tests.rs:357:    assert_eq!(provider, "discord");
[+5 more match(es) in <OPENHUMAN_ROOT>/src/api/rest_tests.rs]
<OPENHUMAN_ROOT>/src/api/config.rs:157:        let is_inference_provider = looks_like_inference_provider_endpoint(u);
<OPENHUMAN_ROOT>/src/api/config.rs:167:            crate::openhuman::config::schema::cloud_providers::endpoint_host(u).is_some_and(|h| {
<OPENHUMAN_ROOT>/src/api/config.rs:168:                crate::openhuman::config::schema::cloud_providers::host_is_builtin_cloud_provider(
<OPENHUMAN_ROOT>/src/api/config.rs:176:            is_inference_provider,
<OPENHUMAN_ROOT>/src/api/config.rs:183:        // (local model runner OR remote managed provider), OR when it is one of
[+41 more match(es) in <OPENHUMAN_ROOT>/src/api/config.rs]
<OPENHUMAN_ROOT>/src/api/rest.rs:18:    /// user deletes the message on the provider side (Telegram, Discord,
<OPENHUMAN_ROOT>/src/api/rest.rs:23:    #[error("message not found on {provider}: {message_id}")]
<OPENHUMAN_ROOT>/src/api/rest.rs:25:        /// Channel provider segment (e.g. `"telegram"`, `"discord"`).
<OPENHUMAN_ROOT>/src/api/rest.rs:26:        provider: String,
<OPENHUMAN_ROOT>/src/api/rest.rs:27:        /// Provider-specific message id from the URL.
[+19 more match(es) in <OPENHUMAN_ROOT>/src/api/rest.rs]
<OPENHUMAN_ROOT>/src/main.rs:51:            // Defense-in-depth: drop transient-upstream provider failures that
<OPENHUMAN_ROOT>/src/main.rs:58:            // `openhuman::inference::provider::ops::should_report_provider_http_failure`
<OPENHUMAN_ROOT>/src/main.rs:61:            if openhuman_core::core::observability::is_transient_provider_http_failure(&event) {
<OPENHUMAN_ROOT>/src/main.rs:79:            // (domain=llm_provider, failure=transport) — flaky-network
<OPENHUMAN_ROOT>/src/main.rs:83:            if openhuman_core::core::observability::is_transient_provider_transport_failure(&event)
[+10 more match(es) in <OPENHUMAN_ROOT>/src/main.rs]
<OPENHUMAN_ROOT>/src/bin/gmail_backfill_3d.rs:39:use openhuman_core::openhuman::composio::providers::gmail::ingest::ingest_page_into_memory_tree;
<OPENHUMAN_ROOT>/src/bin/gmail_backfill_3d.rs:40:use openhuman_core::openhuman::composio::providers::registry::{
<OPENHUMAN_ROOT>/src/bin/gmail_backfill_3d.rs:41:    get_provider, init_default_providers,
<OPENHUMAN_ROOT>/src/bin/gmail_backfill_3d.rs:140:    init_default_providers();
<OPENHUMAN_ROOT>/src/bin/gmail_backfill_3d.rs:141:    let provider = get_provider("gmail").ok_or_else(|| {
[+2 more match(es) in <OPENHUMAN_ROOT>/src/bin/gmail_backfill_3d.rs]
<OPENHUMAN_ROOT>/src/bin/inference_probe.rs:14://!   chat provider (no harness, no real tools). Useful to isolate
<OPENHUMAN_ROOT>/src/bin/inference_probe.rs:28://! # Raw provider call (no harness):
<OPENHUMAN_ROOT>/src/bin/inference_probe.rs:35:use openhuman_core::openhuman::inference::provider::create_chat_provider;
<OPENHUMAN_ROOT>/src/bin/inference_probe.rs:121:        create_chat_provider(role, config).context("create_chat_provider failed")?;
<OPENHUMAN_ROOT>/src/bin/inference_probe.rs:200:        .context("provider.chat() failed")?;
[+8 more match(es) in <OPENHUMAN_ROOT>/src/bin/inference_probe.rs]
<OPENHUMAN_ROOT>/src/bin/slack_backfill.rs:2://! provider.
<OPENHUMAN_ROOT>/src/bin/slack_backfill.rs:5://! `SlackProvider::sync()` for each active Slack Composio connection —
<OPENHUMAN_ROOT>/src/bin/slack_backfill.rs:30://! export RUST_LOG=info,openhuman_core::openhuman::composio::providers::slack=debug,openhuman_core::openhuman::memory=debug
<OPENHUMAN_ROOT>/src/bin/slack_backfill.rs:45:use openhuman_core::openhuman::composio::providers::registry::{
<OPENHUMAN_ROOT>/src/bin/slack_backfill.rs:356:                    let err = r.error.as_deref().unwrap_or("provider failure");
[+16 more match(es) in <OPENHUMAN_ROOT>/src/bin/slack_backfill.rs]
<OPENHUMAN_ROOT>/src/bin/harness_subagent_audit.rs:4://! provider/backend credentials. It records only sanitized progress metadata:
<OPENHUMAN_ROOT>/AGENTS.md:148:**Provider chain** (`App.tsx`): `Sentry.ErrorBoundary` → `Redux Provider` → `PersistGate` → `BootCheckGate` → `CoreStateProvider` → `SocketProvider` → `ChatRuntimeProvider` → `HashRouter` → `CommandProvider` → `ServiceBlockingGate` → `AppShell`.
<OPENHUMAN_ROOT>/AGENTS.md:150:No `UserProvider`/`AIProvider`/`SkillProvider` — auth lives in `CoreStateProvider` via `fetchCoreAppSnapshot()` RPC.
<OPENHUMAN_ROOT>/AGENTS.md:152:**State** (`store/`): Redux Toolkit slices — `accounts`, `channelConnections`, `chatRuntime`, `coreMode`, `deepLinkAuth`, `mascot`, `notification`, `providerSurface`, `socket`, `thread`. Prefer Redux over ad-hoc `localStorage`.
<OPENHUMAN_ROOT>/AGENTS.md:164:Thin desktop host. Key modules: `core_process`, `core_rpc`, `cdp`, `cef_preflight`, `cef_profile`, `dictation_hotkeys`, `file_logging`, `mascot_native_window`, `screen_capture`, `window_state`, per-provider scanners (`discord_scanner`, `slack_scanner`, `telegram_scanner`, `whatsapp_scanner`, etc.), `meet_audio`/`meet_call`/`meet_video`, `fake_camera`, `webview_accounts`, `webview_apis`.
<OPENHUMAN_ROOT>/AGENTS.md:178:Domains: `about_app`, `accessibility`, `agent`, `app_state`, `approval`, `autocomplete`, `billing`, `channels`, `composio`, `config`, `context`, `cost`, `credentials`, `cron`, `doctor`, `embeddings`, `encryption`, `health`, `heartbeat`, `integrations`, `learning`, `local_ai`, `meet`, `meet_agent`, `memory`, `migration`, `node_runtime`, `notifications`, `overlay`, `people`, `prompt_injection`, `provider_surfaces`, `providers`, `redirect_links`, `referral`, `routing`, `scheduler_gate`, `screen_intelligence`, `security`, `service`, `skills`, `socket`, `subconscious`, `team`, `text_input`, `threads`, `tokenjuice`, `tool_timeout`, `tools`, `tree_summarizer`, `update`, `voice`, `wallet`, `webhooks`, `webview_accounts`, `webview_apis`, `webview_notifications`.
[+2 more match(es) in <OPENHUMAN_ROOT>/AGENTS.md]
<OPENHUMAN_ROOT>/plan.md:32:  frontend E2E (WDIO + Playwright), Rust unit (agent/memory; channels/providers/platform;
<OPENHUMAN_ROOT>/plan.md:55:| ✅ | `src/openhuman/routing/factory.rs` | `factory_constructs_without_panic_when_runtime_enabled`, `factory_llamacpp_provider_constructs_without_panic`, `factory_custom_openai_provider_constructs_without_panic`, `factory_lm_studio_provider_constructs_without_panic` | Skeptic traced the whole construction path — provably infallible (pure struct init), so the tests cannot fail. One test's comment claims to verify probe-URL selection but the body asserts nothing; fields are private so strengthening is blocked. |
<OPENHUMAN_ROOT>/plan.md:59:| ⚠️ | `src/openhuman/channels/providers/whatsapp_tests.rs` | 7 × `whatsapp_parse_<type>_message_skipped` | All hit the identical `type != "text" → continue` branch; collapse to one parameterized loop over the type strings. |
<OPENHUMAN_ROOT>/plan.md:79:| `src/openhuman/channels/providers/qq_tests.rs` | `test_name` | Sole coverage of `QQChannel::name()`, which keys routing (`routes.rs:345`) and the channel map (`runtime/startup.rs:701`). A rename would ship uncaught. |
<OPENHUMAN_ROOT>/plan.md:96:| ✅ | every `connector-*.spec.ts::composio_sync RPC routes to mock backend` | Name promises routing; body only asserts the session didn't crash (original assertion removed per inline comment). | Rename to what it checks, or move the real routing assertion to a native-provider connector where sync actually hits the mock. |
[+8 more match(es) in <OPENHUMAN_ROOT>/plan.md]
<OPENHUMAN_ROOT>/src/core/jsonrpc_tests.rs:679:fn is_session_expired_error_does_not_match_byo_key_provider_401() {
<OPENHUMAN_ROOT>/src/core/jsonrpc_tests.rs:680:    // BYO-key provider 401 should not clear the user session.
<OPENHUMAN_ROOT>/src/core/jsonrpc_tests.rs:712:    // it was too broad and caught Discord/OAuth provider token errors. It is
<OPENHUMAN_ROOT>/Cargo.lock:2938: "icu_provider 2.2.0",
<OPENHUMAN_ROOT>/Cargo.lock:2958: "icu_provider 2.2.0",
<OPENHUMAN_ROOT>/Cargo.lock:2970:name = "icu_provider"
<OPENHUMAN_ROOT>/Cargo.lock:2977: "icu_provider_macros",
<OPENHUMAN_ROOT>/Cargo.lock:2987:name = "icu_provider"
[+2 more match(es) in <OPENHUMAN_ROOT>/Cargo.lock]
<OPENHUMAN_ROOT>/e2e/docker-compose.yml:85:  # Rust inference provider E2E tests (wiremock-based, no live LLM needed).
<OPENHUMAN_ROOT>/src/core/socketio.rs:123:    /// `"provider"` | `"openhuman_budget"` | `"agent_loop"`
<OPENHUMAN_ROOT>/src/core/socketio.rs:137:    /// Provider name extracted from `"<provider> API error (...)"`
<OPENHUMAN_ROOT>/src/core/socketio.rs:138:    /// envelopes. `None` for non-provider errors (OpenHuman budget cap,
<OPENHUMAN_ROOT>/src/core/socketio.rs:139:    /// agent loop) and for transport failures without a provider prefix.
<OPENHUMAN_ROOT>/src/core/socketio.rs:141:    pub error_provider: Option<String>,
[+8 more match(es) in <OPENHUMAN_ROOT>/src/core/socketio.rs]
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
<OPENHUMAN_ROOT>/tests/observability_smoke.rs:14:    is_transient_integrations_failure, is_transient_provider_http_failure,
<OPENHUMAN_ROOT>/tests/observability_smoke.rs:61:            if is_transient_provider_http_failure(&event)
<OPENHUMAN_ROOT>/tests/observability_smoke.rs:242:        "permanent provider failures must reach Sentry"
<OPENHUMAN_ROOT>/tests/observability_smoke.rs:263:    // `failure=all_exhausted` event when every provider/model has been
<OPENHUMAN_ROOT>/tests/observability_smoke.rs:287:        "All providers/models failed. Attempts: openai API error (503 Service Unavailable); custom_openai API error (502 Bad Gateway)",
[+12 more match(es) in <OPENHUMAN_ROOT>/tests/observability_smoke.rs]
<OPENHUMAN_ROOT>/src/openhuman/provider_surfaces/schemas.rs:1://! Controller registry for `provider_surfaces`.
<OPENHUMAN_ROOT>/src/openhuman/provider_surfaces/schemas.rs:3://! The first cut exposes normalized provider event ingestion plus a queue
<OPENHUMAN_ROOT>/src/openhuman/provider_surfaces/schemas.rs:14:use super::types::ProviderEvent;
<OPENHUMAN_ROOT>/src/openhuman/provider_surfaces/schemas.rs:16:pub fn all_provider_surfaces_controller_schemas() -> Vec<ControllerSchema> {
<OPENHUMAN_ROOT>/src/openhuman/provider_surfaces/schemas.rs:20:pub fn all_provider_surfaces_registered_controllers() -> Vec<RegisteredController> {
[+16 more match(es) in <OPENHUMAN_ROOT>/src/openhuman/provider_surfaces/schemas.rs]
<OPENHUMAN_ROOT>/docs/tinyagents-full-migration-plan/C4-journal-progress-parity-plan.md:7:5** — deleting the `ProviderDelta` bridge and `progress_tracing`.
<OPENHUMAN_ROOT>/docs/tinyagents-full-migration-plan/C4-journal-progress-parity-plan.md:28:    `spawn_delta_forwarder` maps engine callbacks + `ProviderDelta` →
<OPENHUMAN_ROOT>/docs/tinyagents-full-migration-plan/C4-journal-progress-parity-plan.md:32:  (`channels/providers/web/progress_bridge.rs:157`) is a side-observer of the
<OPENHUMAN_ROOT>/docs/tinyagents-full-migration-plan/C4-journal-progress-parity-plan.md:54:  adapter's provider `UsageInfo` FIFO) to restore **charged USD**, cache-creation
<OPENHUMAN_ROOT>/docs/tinyagents-full-migration-plan/C4-journal-progress-parity-plan.md:73:2. **Usage accounting**: decide whether charged-USD/provider-cost belongs on a
[+3 more match(es) in <OPENHUMAN_ROOT>/docs/tinyagents-full-migration-plan/C4-journal-progress-parity-plan.md]
<OPENHUMAN_ROOT>/tests/agent_multimodal_public.rs:4:    prepare_messages_for_provider,
<OPENHUMAN_ROOT>/tests/agent_multimodal_public.rs:7:use openhuman_core::openhuman::inference::provider::ChatMessage;
<OPENHUMAN_ROOT>/tests/agent_multimodal_public.rs:50:    let prepared = prepare_messages_for_provider(
<OPENHUMAN_ROOT>/tests/agent_multimodal_public.rs:69:    let prepared = prepare_messages_for_provider(
<OPENHUMAN_ROOT>/tests/agent_multimodal_public.rs:88:    let err = prepare_messages_for_provider(
[+3 more match(es) in <OPENHUMAN_ROOT>/tests/agent_multimodal_public.rs]
<OPENHUMAN_ROOT>/gitbooks/guides/recover-failed-installation.md:69:| Sign-in stalls after the provider step; log mentions `openhuman://` scheme **not registered** (Windows) | The URL handler didn't register, or the install was moved after first launch | Follow the repair steps in [Troubleshooting Sign-In](../overview/troubleshooting-sign-in.md#windows-openhuman-handler-not-registered) |
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:85:    /// A sub-agent failed (max iterations, provider error, missing
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:216:    /// The configured embedding provider is unreachable or the requested model
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:241:    /// Published by `inference::provider::ops::http_error::
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:242:    /// log_byo_provider_auth_failure` (once per failure episode, via the
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:248:    /// inline provider-error notice. The `message` field is a pre-formatted,
[+23 more match(es) in <OPENHUMAN_ROOT>/src/core/event_bus/events.rs]
<OPENHUMAN_ROOT>/scripts/debug/harness-subagent-audit.sh:15:echo "[harness_subagent_audit] running live audit; requires configured provider/backend credentials" >&2
<OPENHUMAN_ROOT>/docs/tinycortex-migration-spec.md:100:- **Process glue:** `memory/global.rs` singleton + queue worker; `memory/source_scope.rs` task-locals; `memory/chat.rs`; embeddings provider wiring.
<OPENHUMAN_ROOT>/docs/tinycortex-migration-spec.md:113:`embeddings.rs` (`EmbeddingBackend`/`Embedder`), `chat.rs` (`ChatProvider`/`Summariser`×2/
<OPENHUMAN_ROOT>/tests/agent_turn_builder_leftovers_raw_coverage_e2e.rs:8:use openhuman_core::openhuman::inference::provider::{
<OPENHUMAN_ROOT>/tests/agent_turn_builder_leftovers_raw_coverage_e2e.rs:9:    ChatMessage, ChatRequest, ChatResponse, Provider, ToolCall, UsageInfo,
<OPENHUMAN_ROOT>/tests/agent_turn_builder_leftovers_raw_coverage_e2e.rs:59:struct ScriptedProvider {
<OPENHUMAN_ROOT>/tests/agent_turn_builder_leftovers_raw_coverage_e2e.rs:65:impl ScriptedProvider {
<OPENHUMAN_ROOT>/tests/agent_turn_builder_leftovers_raw_coverage_e2e.rs:80:impl Provider for ScriptedProvider {
[+12 more match(es) in <OPENHUMAN_ROOT>/tests/agent_turn_builder_leftovers_raw_coverage_e2e.rs]
<OPENHUMAN_ROOT>/src/openhuman/provider_surfaces/ops.rs:1://! Core operations for provider assistive surfaces.
<OPENHUMAN_ROOT>/src/openhuman/provider_surfaces/ops.rs:12:use super::types::{ProviderEvent, RespondQueueItem, RespondQueueListResponse};
<OPENHUMAN_ROOT>/src/openhuman/provider_surfaces/ops.rs:46:    request: ProviderEvent,
<OPENHUMAN_ROOT>/src/openhuman/provider_surfaces/ops.rs:49:        provider = %request.provider,
<OPENHUMAN_ROOT>/src/openhuman/provider_surfaces/ops.rs:54:        "[provider-surfaces] ingest_event"
[+5 more match(es) in <OPENHUMAN_ROOT>/src/openhuman/provider_surfaces/ops.rs]
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
[+12 more match(es) in <OPENHUMAN_ROOT>/src/core/all.rs]
<OPENHUMAN_ROOT>/tests/subconscious_fullstack_e2e.rs:2://! run against a **mocked LLM provider** — no network, no Ollama, no real
<OPENHUMAN_ROOT>/tests/subconscious_fullstack_e2e.rs:3://! model anywhere (cloud and local provider funnels are both overridden).
<OPENHUMAN_ROOT>/tests/subconscious_fullstack_e2e.rs:6://! SessionExecutor *above* the model), this test mocks at the *provider*
<OPENHUMAN_ROOT>/tests/subconscious_fullstack_e2e.rs:8://!   - `GatePass::evaluate` → `agent::triage::run_triage` → real provider
<OPENHUMAN_ROOT>/tests/subconscious_fullstack_e2e.rs:13://! The mock is installed via the factory's `test_provider_override` seam,
[+16 more match(es) in <OPENHUMAN_ROOT>/tests/subconscious_fullstack_e2e.rs]
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
[+19 more match(es) in <OPENHUMAN_ROOT>/src/openhuman/provider_surfaces/README.md]
<OPENHUMAN_ROOT>/src/core/runtime.rs:4://! hundreds of tool specs + the nested provider/tool loop), and delegating
<OPENHUMAN_ROOT>/scripts/debug/goals-live-cases.md:8:> configured provider/model + credentials in the target workspace.
<OPENHUMAN_ROOT>/scripts/debug/goals-live-cases.md:13:- For `reflect`: a workspace with provider creds (use your real workspace —
<OPENHUMAN_ROOT>/src/openhuman/provider_surfaces/store.rs:1://! Persistence for provider assistive surfaces.
<OPENHUMAN_ROOT>/src/openhuman/provider_surfaces/store.rs:3://! Follow-up work will add a SQLite-backed store for normalized provider
<OPENHUMAN_ROOT>/src/openhuman/provider_surfaces/store.rs:8:use crate::openhuman::provider_surfaces::types::{ProviderEvent, RespondQueueItem};
<OPENHUMAN_ROOT>/src/openhuman/provider_surfaces/store.rs:10:/// Soft cap on the in-memory respond queue to bound growth under provider
<OPENHUMAN_ROOT>/src/openhuman/provider_surfaces/store.rs:27:fn queue_item_id(event: &ProviderEvent) -> String {
[+3 more match(es) in <OPENHUMAN_ROOT>/src/openhuman/provider_surfaces/store.rs]
<OPENHUMAN_ROOT>/docs/TEST-COVERAGE-MATRIX.md:61:### 1.1 Multi-Provider Authentication
<OPENHUMAN_ROOT>/docs/TEST-COVERAGE-MATRIX.md:67:| 1.1.3 | Twitter (X) Login | WD    | `login-flow.spec.ts`                    | 🟡     | Generic OAuth path; assert provider tag in #968 |
<OPENHUMAN_ROOT>/docs/TEST-COVERAGE-MATRIX.md:75:| 1.2.2 | Multi-Provider Linking     | WD    | _missing_ — tracked #968                      | ❌     | Need spec linking 4 providers to one account |
<OPENHUMAN_ROOT>/docs/TEST-COVERAGE-MATRIX.md:219:| 4.4.10 | Provider Error Retry                    | RI    | `tests/agent_harness_e2e.rs`                                                                                                 | ✅     | First upstream 500 retried by ReliableProvider; second succeeds                                                 |
<OPENHUMAN_ROOT>/docs/TEST-COVERAGE-MATRIX.md:526:| 13.1.4 | Wallet Balances Panel               | VU         | `app/src/components/settings/panels/__tests__/WalletBalancesPanel.test.tsx`, `app/src/services/walletApi.test.ts`                                                           | ✅     | Loading/error/empty/loaded states; Retry + Refresh re-invocation; chain badges; truncated address; providerStatus chip                                                                                                                                                           |
[+10 more match(es) in <OPENHUMAN_ROOT>/docs/TEST-COVERAGE-MATRIX.md]
<OPENHUMAN_ROOT>/src/core/jsonrpc.rs:156:                // query params, or pasted-through provider error text that
<OPENHUMAN_ROOT>/src/core/jsonrpc.rs:159:                let redacted = crate::openhuman::inference::provider::ops::sanitize_api_error(
<OPENHUMAN_ROOT>/src/core/jsonrpc.rs:237:    // the UI. Generic downstream/provider 401s must stay recoverable errors;
<OPENHUMAN_ROOT>/src/core/jsonrpc.rs:240:        let sanitized_reason = crate::openhuman::inference::provider::ops::sanitize_api_error(msg);
<OPENHUMAN_ROOT>/src/core/jsonrpc.rs:247:            // pasted-through provider replies. `sanitize_api_error` runs
[+22 more match(es) in <OPENHUMAN_ROOT>/src/core/jsonrpc.rs]
<OPENHUMAN_ROOT>/scripts/debug/goals-live.mjs:44:  --isolated-workspace With --spawn-core, use a throwaway temp workspace (needs provider creds in env to enrich)
<OPENHUMAN_ROOT>/scripts/debug/goals-live.mjs:331:    console.log("  (no transcript usage recorded — provider may not have emitted usage)");
<OPENHUMAN_ROOT>/scripts/debug/goals-live.mjs:507:      "  (enrichment needs a configured provider/model + the goals_agent definition)",
<OPENHUMAN_ROOT>/tests/monitor_agent_e2e.rs:10:use openhuman_core::openhuman::inference::provider::thread_context::with_thread_id;
<OPENHUMAN_ROOT>/tests/monitor_agent_e2e.rs:11:use openhuman_core::openhuman::inference::provider::traits::ProviderCapabilities;
<OPENHUMAN_ROOT>/tests/monitor_agent_e2e.rs:12:use openhuman_core::openhuman::inference::provider::{
<OPENHUMAN_ROOT>/tests/monitor_agent_e2e.rs:13:    ChatMessage, ChatRequest, ChatResponse, Provider, ToolCall,
<OPENHUMAN_ROOT>/tests/monitor_agent_e2e.rs:33:enum ProviderStep {
[+38 more match(es) in <OPENHUMAN_ROOT>/tests/monitor_agent_e2e.rs]

[compacted tool output — this is a PARTIAL view; the full original (73216 bytes) is available by calling tokenjuice_retrieve with token "374a3c114df684cc8e841711a69825de" (marker ⟦tj:374a3c114df684cc8e841711a69825de⟧)]