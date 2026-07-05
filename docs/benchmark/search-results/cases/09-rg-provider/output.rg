[search: 500 match(es) across 13 file(s) · top 5 per file · full set via retrieve footer]
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
<OPENHUMAN_ROOT>/src/core/jsonrpc_tests.rs:679:fn is_session_expired_error_does_not_match_byo_key_provider_401() {
<OPENHUMAN_ROOT>/src/core/jsonrpc_tests.rs:680:    // BYO-key provider 401 should not clear the user session.
<OPENHUMAN_ROOT>/src/core/jsonrpc_tests.rs:712:    // it was too broad and caught Discord/OAuth provider token errors. It is
<OPENHUMAN_ROOT>/src/core/subconscious_cli.rs:123:                eprintln!("[subconscious] session token found — provider available");
<OPENHUMAN_ROOT>/src/core/subconscious_cli.rs:126:                eprintln!("[subconscious] WARNING: no session token — cloud provider will fail");
<OPENHUMAN_ROOT>/src/core/subconscious_cli.rs:134:        // Check provider availability
<OPENHUMAN_ROOT>/src/core/subconscious_cli.rs:136:            crate::openhuman::subconscious::provider::subconscious_provider_unavailable_reason(
<OPENHUMAN_ROOT>/src/core/subconscious_cli.rs:140:            eprintln!("[subconscious] provider unavailable: {reason}");
[+5 more match(es) in <OPENHUMAN_ROOT>/src/core/subconscious_cli.rs]
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:85:    /// A sub-agent failed (max iterations, provider error, missing
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:216:    /// The configured embedding provider is unreachable or the requested model
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:241:    /// Published by `inference::provider::ops::http_error::
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:242:    /// log_byo_provider_auth_failure` (once per failure episode, via the
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:248:    /// inline provider-error notice. The `message` field is a pre-formatted,
[+23 more match(es) in <OPENHUMAN_ROOT>/src/core/event_bus/events.rs]
<OPENHUMAN_ROOT>/src/core/observability.rs:34:/// (`openhuman::inference::provider::ops::should_report_provider_http_failure`) and the
<OPENHUMAN_ROOT>/src/core/observability.rs:35:/// `before_send` filter (`is_transient_provider_http_failure`). Update here
<OPENHUMAN_ROOT>/src/core/observability.rs:99:    /// HTTP layer (`providers::ops::api_error`) already demotes its own
<OPENHUMAN_ROOT>/src/core/observability.rs:196:    /// `AgentError::EmptyProviderResponse` + `AgentError::skips_sentry()`
<OPENHUMAN_ROOT>/src/core/observability.rs:210:    /// any future channel provider) whose error chain contains `"model
[+316 more match(es) in <OPENHUMAN_ROOT>/src/core/observability.rs]

[compacted tool output — this is a PARTIAL view; the full original (58021 bytes) is available by calling tokenjuice_retrieve with token "206543dc099b9c76c0690190597440c4" (marker ⟦tj:206543dc099b9c76c0690190597440c4⟧)]