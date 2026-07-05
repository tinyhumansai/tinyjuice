# Search Results

Real ripgrep result sets from an OpenHuman checkout. TinyJuice groups matches by file, keeps top hits, and records omitted match counts.

Each row links to the full raw input and the exact compacted output used by the benchmark.

## Cases

| Case | Input | Output | Original | Compacted | Est. token reduction | Avg latency | CCR |
| --- | --- | --- | ---: | ---: | ---: | ---: | --- |
| `09-rg-provider` | [input](cases/09-rg-provider/input.rg) | [output](cases/09-rg-provider/output.rg) | 58.0 KB | 7.2 KB | 87.7% | 0.127 ms | true |
| `05-rg-agent` | [input](cases/05-rg-agent/input.rg) | [output](cases/05-rg-agent/output.rg) | 63.2 KB | 12.7 KB | 79.9% | 0.136 ms | true |
| `04-rg-openhuman` | [input](cases/04-rg-openhuman/input.rg) | [output](cases/04-rg-openhuman/output.rg) | 76.8 KB | 16.7 KB | 77.5% | 0.167 ms | true |
| `06-rg-memory` | [input](cases/06-rg-memory/input.rg) | [output](cases/06-rg-memory/output.rg) | 70.0 KB | 28.5 KB | 59.8% | 0.145 ms | true |
| `10-rg-subconscious` | [input](cases/10-rg-subconscious/input.rg) | [output](cases/10-rg-subconscious/output.rg) | 80.2 KB | 49.0 KB | 39.1% | 0.156 ms | true |
| `08-rg-tinyplace` | [input](cases/08-rg-tinyplace/input.rg) | [output](cases/08-rg-tinyplace/output.rg) | 73.1 KB | 48.5 KB | 33.9% | 0.152 ms | true |
| `01-rg-tokenjuice` | [input](cases/01-rg-tokenjuice/input.rg) | [output](cases/01-rg-tokenjuice/output.rg) | 71.3 KB | 47.5 KB | 33.8% | 0.162 ms | true |
| `02-rg-compression` | [input](cases/02-rg-compression/input.rg) | [output](cases/02-rg-compression/output.rg) | 77.8 KB | 62.3 KB | 20.1% | 0.178 ms | true |
| `07-rg-workflow` | [input](cases/07-rg-workflow/input.rg) | [output](cases/07-rg-workflow/output.rg) | 269.9 KB | 229.0 KB | 15.1% | 0.336 ms | true |
| `03-rg-retrieve` | [input](cases/03-rg-retrieve/input.rg) | [output](cases/03-rg-retrieve/output.rg) | 1.9 MB | 1.9 MB | 0.8% | 1.644 ms | true |

## What TinyJuice Is Doing

Search results are parsed as file/line/body records. TinyJuice groups by file, keeps high-value matches per file, and tells the reader how many additional matches were hidden.

## Syntax-Aware Samples

### `09-rg-provider`

- [Full input](cases/09-rg-provider/input.rg)
- [Full output](cases/09-rg-provider/output.rg)

Input excerpt:

```text
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

```

Output excerpt:

```text
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

```

### `05-rg-agent`

- [Full input](cases/05-rg-agent/input.rg)
- [Full output](cases/05-rg-agent/output.rg)

Input excerpt:

```text
<OPENHUMAN_ROOT>/src/api/config.rs:14://! caused every `/auth/*`, `/agent-integrations/*`, and `/voice/*` request to
<OPENHUMAN_ROOT>/src/api/config.rs:103:/// so `/auth/*`, `/voice/*`, and `/agent-integrations/*` never accidentally
<OPENHUMAN_ROOT>/src/api/config.rs:460:/// (`/auth/me`, `/agent-integrations/…`) which then land on
<OPENHUMAN_ROOT>/src/api/config.rs:502:/// | `https://api.tinyhumans.ai/openai/v1/…`   | `/agent-integrations/foo` | `https://api.tinyhumans.ai/agent-integrations/foo`  ← path replaced   |
<OPENHUMAN_ROOT>/src/api/config.rs:678:             /agent-integrations/* requests don't 404 against your local LLM"
<OPENHUMAN_ROOT>/src/api/config.rs:800:        // /agent-integrations/* calls.
<OPENHUMAN_ROOT>/src/api/config.rs:804:                "/agent-integrations/composio/toolkits",
<OPENHUMAN_ROOT>/src/api/config.rs:806:            "https://api.tinyhumans.ai/agent-integrations/composio/toolkits"
<OPENHUMAN_ROOT>/src/api/config.rs:812:        let expected = "https://api.tinyhumans.ai/agent-integrations/composio/toolkits";
<OPENHUMAN_ROOT>/src/api/config.rs:816:                "/agent-integrations/composio/toolkits"
<OPENHUMAN_ROOT>/src/api/config.rs:823:                "/agent-integrations/composio/toolkits"
<OPENHUMAN_ROOT>/src/api/config.rs:834:                "/agent-integrations/composio/tools?toolkits=gmail"
<OPENHUMAN_ROOT>/src/api/config.rs:836:            "https://api.tinyhumans.ai/agent-integrations/composio/tools?toolkits=gmail"
<OPENHUMAN_ROOT>/src/api/config.rs:852:            api_url("http://localhost:1234/v1", "/agent-integrations/foo"),
<OPENHUMAN_ROOT>/src/api/config.rs:853:            "http://localhost:1234/agent-integrations/foo"
<OPENHUMAN_ROOT>/src/api/rest.rs:919:    /// Signals "the agent is typing…" on a channel that supports it
<OPENHUMAN_ROOT>/src/main.rs:111:            // the agent re-report routes through `TransientUpstreamHttp`, but the
<OPENHUMAN_ROOT>/src/main.rs:120:            // `agent::harness::session::runtime::run_single`,
<OPENHUMAN_ROOT>/src/main.rs:123:            // deterministic agent-state outcome surfaced to the user via
<OPENHUMAN_ROOT>/src/lib.rs:6://! - Domain-specific logic for the OpenHuman agent runtime.
<OPENHUMAN_ROOT>/src/bin/inference_probe.rs:7://! - `--mode harness` (default): build a real `Agent::from_config()`
<OPENHUMAN_ROOT>/src/bin/inference_probe.rs:24://!   RUST_LOG=info,openhuman_core::openhuman::agent=debug,openhuman_core::openhuman::inference=debug \
<OPENHUMAN_ROOT>/src/bin/inference_probe.rs:33:use openhuman_core::openhuman::agent::Agent;
<OPENHUMAN_ROOT>/src/bin/inference_probe.rs:74:        "[probe] config.agent.tool_dispatcher = {:?}",
<OPENHUMAN_ROOT>/src/bin/inference_probe.rs:75:        config.agent.tool_dispatcher
<OPENHUMAN_ROOT>/src/bin/inference_probe.rs:89:    let mut agent = Agent::from_config(config).context("Agent::from_config failed")?;
<OPENHUMAN_ROOT>/src/bin/inference_probe.rs:94:    agent.fetch_connected_integrations().await;
<OPENHUMAN_ROOT>/src/bin/inference_probe.rs:95:    let refreshed = agent.refresh_delegation_tools();
<OPENHUMAN_ROOT>/src/bin/inference_probe.rs:96:    let conn_count = agent.connected_integrations().len();
<OPENHUMAN_ROOT>/src/bin/inference_probe.rs:101:    eprintln!("[probe] visible tool count = {}", agent.tools().len());
<OPENHUMAN_ROOT>/src/bin/inference_probe.rs:102:    eprintln!("[probe] model = {}", agent.model_name());
<OPENHUMAN_ROOT>/src/bin/inference_probe.rs:104:    eprintln!("[probe] >>> agent.run_single() ...");
<OPENHUMAN_ROOT>/src/bin/inference_probe.rs:106:    let response = agent
<OPENHUMAN_ROOT>/src/bin/inference_probe.rs:109:        .context("agent.run_single failed")?;
<OPENHUMAN_ROOT>/src/bin/harness_subagent_audit.rs:1://! Live harness audit for reusable async sub-agent delegation.
<OPENHUMAN_ROOT>/src/bin/harness_subagent_audit.rs:12://! scripts/debug/harness-subagent-audit.sh --turns 2

```

Output excerpt:

```text
[search: 500 match(es) across 24 file(s) · top 5 per file · full set via retrieve footer]
<OPENHUMAN_ROOT>/src/api/config.rs:14://! caused every `/auth/*`, `/agent-integrations/*`, and `/voice/*` request to
<OPENHUMAN_ROOT>/src/api/config.rs:103:/// so `/auth/*`, `/voice/*`, and `/agent-integrations/*` never accidentally
<OPENHUMAN_ROOT>/src/api/config.rs:460:/// (`/auth/me`, `/agent-integrations/…`) which then land on
<OPENHUMAN_ROOT>/src/api/config.rs:502:/// | `https://api.tinyhumans.ai/openai/v1/…`   | `/agent-integrations/foo` | `https://api.tinyhumans.ai/agent-integrations/foo`  ← path replaced   |
<OPENHUMAN_ROOT>/src/api/config.rs:678:             /agent-integrations/* requests don't 404 against your local LLM"
[+10 more match(es) in <OPENHUMAN_ROOT>/src/api/config.rs]
<OPENHUMAN_ROOT>/src/api/rest.rs:919:    /// Signals "the agent is typing…" on a channel that supports it
<OPENHUMAN_ROOT>/src/main.rs:111:            // the agent re-report routes through `TransientUpstreamHttp`, but the
<OPENHUMAN_ROOT>/src/main.rs:120:            // `agent::harness::session::runtime::run_single`,
<OPENHUMAN_ROOT>/src/main.rs:123:            // deterministic agent-state outcome surfaced to the user via
<OPENHUMAN_ROOT>/src/lib.rs:6://! - Domain-specific logic for the OpenHuman agent runtime.
<OPENHUMAN_ROOT>/src/bin/inference_probe.rs:7://! - `--mode harness` (default): build a real `Agent::from_config()`
<OPENHUMAN_ROOT>/src/bin/inference_probe.rs:24://!   RUST_LOG=info,openhuman_core::openhuman::agent=debug,openhuman_core::openhuman::inference=debug \
<OPENHUMAN_ROOT>/src/bin/inference_probe.rs:33:use openhuman_core::openhuman::agent::Agent;
<OPENHUMAN_ROOT>/src/bin/inference_probe.rs:89:    let mut agent = Agent::from_config(config).context("Agent::from_config failed")?;
<OPENHUMAN_ROOT>/src/bin/inference_probe.rs:109:        .context("agent.run_single failed")?;
[+9 more match(es) in <OPENHUMAN_ROOT>/src/bin/inference_probe.rs]
<OPENHUMAN_ROOT>/src/bin/harness_subagent_audit.rs:27:    self, AuditSteerError, AuditSubagentSessionStore, DurableSubagentSession, DurableSubagentStatus,
<OPENHUMAN_ROOT>/src/bin/harness_subagent_audit.rs:91:    subagent_failed: Vec<SubagentFailedEvent>,
<OPENHUMAN_ROOT>/src/bin/harness_subagent_audit.rs:164:struct SubagentFailedEvent {
<OPENHUMAN_ROOT>/src/bin/harness_subagent_audit.rs:233:        eprintln!("[harness_subagent_audit] ERROR: {err:#}");
<OPENHUMAN_ROOT>/src/bin/harness_subagent_audit.rs:271:    let mut agent = Agent::from_config(&config).context("Agent::from_config failed")?;
[+165 more match(es) in <OPENHUMAN_ROOT>/src/bin/harness_subagent_audit.rs]
<OPENHUMAN_ROOT>/src/core/jsonrpc_tests.rs:93:/// semaphore, `GLOBAL_REGISTRY` agent.run_turn handler, `STARTED`
<OPENHUMAN_ROOT>/src/core/jsonrpc_tests.rs:697:    let msg = r#"[composio] list_connections failed: Backend returned 500 Internal Server Error for GET https://api.tinyhumans.ai/agent-integrations/composio/connections: 40...
<OPENHUMAN_ROOT>/src/core/event_bus/bus.rs:59:/// (e.g., an agent turn completed, a memory was stored).
<OPENHUMAN_ROOT>/src/core/event_bus/testing.rs:20://! [`crate::openhuman::agent::bus::mock_agent_run_turn`]) compose on top of
<OPENHUMAN_ROOT>/src/core/event_bus/testing.rs:61:/// [`crate::openhuman::agent::bus::use_real_agent_handler`] that need the
<OPENHUMAN_ROOT>/src/core/event_bus/testing.rs:62:/// real agent handler installed without racing against a stub-installing
<OPENHUMAN_ROOT>/src/core/event_bus/testing.rs:110:/// [`crate::openhuman::agent::bus::mock_agent_run_turn`]) should compose
<OPENHUMAN_ROOT>/CONTRIBUTING.md:7:For deeper architecture and subsystem references, use the GitBook under [`gitbooks/developing/`](gitbooks/developing/). For coding-agent and repository-specific implementation rules, se...
<OPENHUMAN_ROOT>/CONTRIBUTING.md:201:If you only changed docs in a normal local workflow, `pnpm format:check` is usually the only validation you need. AI-authored or remote-agent PRs must still fill in the AI Authored PR...
<OPENHUMAN_ROOT>/CONTRIBUTING.md:220:- For AI-authored or remote-agent PRs, also fill in the AI Authored PR Metadata section of the PR template.
<OPENHUMAN_ROOT>/CONTRIBUTING.md:249:├── AGENTS.md               # Coding-agent repo rules
<OPENHUMAN_ROOT>/CONTRIBUTING.md:303:If you are contributing through a coding agent or remote environment, include the metadata required by the PR template and the Codex PR checklist.

```

### `04-rg-openhuman`

- [Full input](cases/04-rg-openhuman/input.rg)
- [Full output](cases/04-rg-openhuman/output.rg)

Input excerpt:

```text
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

```

Output excerpt:

```text
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

```

### `06-rg-memory`

- [Full input](cases/06-rg-memory/input.rg)
- [Full output](cases/06-rg-memory/output.rg)

Input excerpt:

```text
<OPENHUMAN_ROOT>/src/main.rs:297:/// `src/openhuman/memory/safety/mod.rs`.
<OPENHUMAN_ROOT>/src/lib.rs:14:pub use openhuman::memory_store::{MemoryClient, MemoryState};
<OPENHUMAN_ROOT>/src/bin/gmail_backfill_3d.rs:1://! Backfill the last N days of Gmail into the memory-tree content store.
<OPENHUMAN_ROOT>/src/bin/gmail_backfill_3d.rs:5://! [`EmailThread`], ingests it through `ingest_page_into_memory_tree` (which
<OPENHUMAN_ROOT>/src/bin/gmail_backfill_3d.rs:39:use openhuman_core::openhuman::composio::providers::gmail::ingest::ingest_page_into_memory_tree;
<OPENHUMAN_ROOT>/src/bin/gmail_backfill_3d.rs:44:use openhuman_core::openhuman::memory_queue::drain_until_idle;
<OPENHUMAN_ROOT>/src/bin/gmail_backfill_3d.rs:45:use openhuman_core::openhuman::memory_store::chunks::store::{
<OPENHUMAN_ROOT>/src/bin/gmail_backfill_3d.rs:48:use openhuman_core::openhuman::memory_store::content::read::{
<OPENHUMAN_ROOT>/src/bin/gmail_backfill_3d.rs:55:    about = "Backfill last N days of Gmail into the memory-tree content store (.md files + SQLite)."
<OPENHUMAN_ROOT>/src/bin/gmail_backfill_3d.rs:123:        wipe_memory_tree_state(&config)?;
<OPENHUMAN_ROOT>/src/bin/gmail_backfill_3d.rs:172:    let content_root = config.memory_tree_content_root();
<OPENHUMAN_ROOT>/src/bin/gmail_backfill_3d.rs:244:            ingest_page_into_memory_tree(&config, &owner, None, &messages).await?;
<OPENHUMAN_ROOT>/src/bin/gmail_backfill_3d.rs:326:/// Wipe `<workspace>/memory_tree/chunks.db` (+ wal/shm) and
<OPENHUMAN_ROOT>/src/bin/gmail_backfill_3d.rs:331:fn wipe_memory_tree_state(config: &Config) -> Result<()> {
<OPENHUMAN_ROOT>/src/bin/gmail_backfill_3d.rs:332:    let mt_dir = config.workspace_dir.join("memory_tree");
<OPENHUMAN_ROOT>/src/bin/gmail_backfill_3d.rs:341:    let content_root = config.memory_tree_content_root();
<OPENHUMAN_ROOT>/src/bin/gmail_backfill_3d.rs:355:    let content_root = config.memory_tree_content_root();
<OPENHUMAN_ROOT>/src/bin/gmail_backfill_3d.rs:425:    let content_root = config.memory_tree_content_root();
<OPENHUMAN_ROOT>/src/bin/slack_backfill.rs:18://!   unconfigured — `memory/tree/ingest` soft-falls-back per call.
<OPENHUMAN_ROOT>/src/bin/slack_backfill.rs:24://! export OPENHUMAN_MEMORY_EMBED_ENDPOINT=http://localhost:11434
<OPENHUMAN_ROOT>/src/bin/slack_backfill.rs:25://! export OPENHUMAN_MEMORY_EMBED_MODEL=nomic-embed-text
<OPENHUMAN_ROOT>/src/bin/slack_backfill.rs:26://! export OPENHUMAN_MEMORY_EXTRACT_ENDPOINT=http://localhost:11434
<OPENHUMAN_ROOT>/src/bin/slack_backfill.rs:27://! export OPENHUMAN_MEMORY_EXTRACT_MODEL=qwen2.5:0.5b
<OPENHUMAN_ROOT>/src/bin/slack_backfill.rs:28://! export OPENHUMAN_MEMORY_SUMMARISE_ENDPOINT=http://localhost:11434
<OPENHUMAN_ROOT>/src/bin/slack_backfill.rs:29://! export OPENHUMAN_MEMORY_SUMMARISE_MODEL=llama3.1:8b
<OPENHUMAN_ROOT>/src/bin/slack_backfill.rs:30://! export RUST_LOG=info,openhuman_core::openhuman::composio::providers::slack=debug,openhuman_core::openhuman::memory=debug
<OPENHUMAN_ROOT>/src/bin/slack_backfill.rs:54:use openhuman_core::openhuman::memory;
<OPENHUMAN_ROOT>/src/bin/slack_backfill.rs:137:    /// touching the memory tree.
<OPENHUMAN_ROOT>/src/bin/slack_backfill.rs:151:    // memory-tree pipeline, the slack ingestion ops layer, …).
<OPENHUMAN_ROOT>/src/bin/slack_backfill.rs:174:    // `memory_tree.embedding_*`, `llm_extractor_*`, and
<OPENHUMAN_ROOT>/src/bin/slack_backfill.rs:187:    // Bootstrap the memory global so `SyncState` KV reads/writes work
<OPENHUMAN_ROOT>/src/bin/slack_backfill.rs:190:    memory::global::init(config.workspace_dir.clone())
<OPENHUMAN_ROOT>/src/bin/slack_backfill.rs:191:        .map_err(|e| anyhow::anyhow!("[slack_backfill] memory::global::init failed: {e}"))?;
<OPENHUMAN_ROOT>/src/bin/slack_backfill.rs:214:        use openhuman_core::openhuman::memory::ingest_pipeline::ingest_chat;
<OPENHUMAN_ROOT>/src/bin/slack_backfill.rs:215:        use openhuman_core::openhuman::memory_sync::canonicalize::chat::{ChatBatch, ChatMessage};
<OPENHUMAN_ROOT>/src/bin/slack_backfill.rs:302:        // memory tree or burning extra quota on retries.

```

Output excerpt:

```text
[search: 500 match(es) across 45 file(s) · top 5 per file · full set via retrieve footer]
<OPENHUMAN_ROOT>/src/main.rs:297:/// `src/openhuman/memory/safety/mod.rs`.
<OPENHUMAN_ROOT>/src/lib.rs:14:pub use openhuman::memory_store::{MemoryClient, MemoryState};
<OPENHUMAN_ROOT>/src/bin/gmail_backfill_3d.rs:1://! Backfill the last N days of Gmail into the memory-tree content store.
<OPENHUMAN_ROOT>/src/bin/gmail_backfill_3d.rs:5://! [`EmailThread`], ingests it through `ingest_page_into_memory_tree` (which
<OPENHUMAN_ROOT>/src/bin/gmail_backfill_3d.rs:39:use openhuman_core::openhuman::composio::providers::gmail::ingest::ingest_page_into_memory_tree;
<OPENHUMAN_ROOT>/src/bin/gmail_backfill_3d.rs:44:use openhuman_core::openhuman::memory_queue::drain_until_idle;
<OPENHUMAN_ROOT>/src/bin/gmail_backfill_3d.rs:45:use openhuman_core::openhuman::memory_store::chunks::store::{
[+11 more match(es) in <OPENHUMAN_ROOT>/src/bin/gmail_backfill_3d.rs]
<OPENHUMAN_ROOT>/src/bin/slack_backfill.rs:18://!   unconfigured — `memory/tree/ingest` soft-falls-back per call.
<OPENHUMAN_ROOT>/src/bin/slack_backfill.rs:24://! export OPENHUMAN_MEMORY_EMBED_ENDPOINT=http://localhost:11434
<OPENHUMAN_ROOT>/src/bin/slack_backfill.rs:25://! export OPENHUMAN_MEMORY_EMBED_MODEL=nomic-embed-text
<OPENHUMAN_ROOT>/src/bin/slack_backfill.rs:26://! export OPENHUMAN_MEMORY_EXTRACT_ENDPOINT=http://localhost:11434
<OPENHUMAN_ROOT>/src/bin/slack_backfill.rs:191:        .map_err(|e| anyhow::anyhow!("[slack_backfill] memory::global::init failed: {e}"))?;
[+21 more match(es) in <OPENHUMAN_ROOT>/src/bin/slack_backfill.rs]
<OPENHUMAN_ROOT>/src/bin/memory_tree_init_smoke.rs:1://! Manual stress smoke for the memory_tree schema-init race fix.
<OPENHUMAN_ROOT>/src/bin/memory_tree_init_smoke.rs:3://! Spins N concurrent threads racing into `memory::tree::store::with_connection`
<OPENHUMAN_ROOT>/src/bin/memory_tree_init_smoke.rs:15://!   cargo run --bin memory-tree-init-smoke -- 32
<OPENHUMAN_ROOT>/src/bin/memory_tree_init_smoke.rs:19://!   cargo run --bin memory-tree-init-smoke -- 32
<OPENHUMAN_ROOT>/src/bin/memory_tree_init_smoke.rs:33:use openhuman_core::openhuman::memory_store::chunks::store::with_connection;
[+1 more match(es) in <OPENHUMAN_ROOT>/src/bin/memory_tree_init_smoke.rs]
<OPENHUMAN_ROOT>/src/core/jsonrpc_tests.rs:89:/// `memory::tree::jobs::start` + `composio::start_periodic_sync` +
<OPENHUMAN_ROOT>/src/core/jsonrpc_tests.rs:224:async fn invoke_memory_init_accepts_empty_params() {
<OPENHUMAN_ROOT>/src/core/jsonrpc_tests.rs:228:    let result = invoke_method(default_state(), "openhuman.memory_init", json!({})).await;
<OPENHUMAN_ROOT>/src/core/jsonrpc_tests.rs:238:async fn invoke_memory_list_namespaces_rejects_unknown_param() {
<OPENHUMAN_ROOT>/src/core/jsonrpc_tests.rs:241:        "openhuman.memory_list_namespaces",
[+4 more match(es) in <OPENHUMAN_ROOT>/src/core/jsonrpc_tests.rs]
<OPENHUMAN_ROOT>/src/core/event_bus/bus.rs:59:/// (e.g., an agent turn completed, a memory was stored).
<OPENHUMAN_ROOT>/src/core/subconscious_cli.rs:113:        // Init memory client
<OPENHUMAN_ROOT>/src/core/subconscious_cli.rs:114:        let _ = crate::openhuman::memory::global::init(config.workspace_dir.clone());
<OPENHUMAN_ROOT>/src/core/subconscious_cli.rs:144:        // Create engine and run tick. The engine pulls its own memory_diff /
<OPENHUMAN_ROOT>/src/core/subconscious_cli.rs:145:        // context state from the workspace — no memory client to pass in.
<OPENHUMAN_ROOT>/src/core/subconscious_cli.rs:146:        let engine = crate::openhuman::subconscious::memory_instance(&config);
[+2 more match(es) in <OPENHUMAN_ROOT>/src/core/subconscious_cli.rs]
<OPENHUMAN_ROOT>/src/core/event_bind_tokens.rs:19://! This module owns only the in-memory store; the RPC handler that mints
<OPENHUMAN_ROOT>/src/core/event_bus/mod.rs:5://! modules (like memory, skills, and agents) to communicate without

```

### `10-rg-subconscious`

- [Full input](cases/10-rg-subconscious/input.rg)
- [Full output](cases/10-rg-subconscious/output.rg)

Input excerpt:

```text
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:150:    // ── Subconscious orchestrator ───────────────────────────────────────
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:151:    /// A subconscious trigger finished gate evaluation (promote or drop).
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:154:    SubconsciousTriggerProcessed {
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:1404:            Self::SubconsciousTriggerProcessed { .. } => "subconscious",
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:1463:            Self::SubconsciousTriggerProcessed { .. } => "SubconsciousTriggerProcessed",
<OPENHUMAN_ROOT>/README.md:70:- **[A subconscious](https://tinyhumans.gitbook.io/openhuman/features/subconscious)**: a background loop that diffs your world, advances your goals, and writes your morning briefing. Thinkin...
<OPENHUMAN_ROOT>/README.md:78:- **[A split brain, always on](https://tinyhumans.gitbook.io/openhuman/features/orchestration)**: a fast reflex agent triages inbound traffic while a deep reasoning core delegates to worker ...
<OPENHUMAN_ROOT>/AGENTS.md:178:Domains: `about_app`, `accessibility`, `agent`, `app_state`, `approval`, `autocomplete`, `billing`, `channels`, `composio`, `config`, `context`, `cost`, `credentials`, `cron`, `doctor`, `em...
<OPENHUMAN_ROOT>/src/core/cli.rs:80:        "subconscious" | "sub" => {
<OPENHUMAN_ROOT>/src/core/cli.rs:81:            crate::core::subconscious_cli::run_subconscious_command(&args[1..])
<OPENHUMAN_ROOT>/src/core/all.rs:282:    controllers.extend(crate::openhuman::subconscious::all_subconscious_registered_controllers());
<OPENHUMAN_ROOT>/src/core/all.rs:284:        crate::openhuman::subconscious_triggers::all_subconscious_triggers_registered_controllers(),
<OPENHUMAN_ROOT>/src/core/all.rs:461:    schemas.extend(crate::openhuman::subconscious::all_subconscious_controller_schemas());
<OPENHUMAN_ROOT>/src/core/all.rs:463:        crate::openhuman::subconscious_triggers::all_subconscious_triggers_controller_schemas(),
<OPENHUMAN_ROOT>/src/core/all.rs:615:            "Subconscious-orchestration read surface: chat windows (master/subconscious/per-session), message history, Master steering DMs, read state, and steering status.",
<OPENHUMAN_ROOT>/src/core/all.rs:636:        "subconscious" => Some("Periodic local-model background awareness loop."),
<OPENHUMAN_ROOT>/src/core/all.rs:637:        "subconscious_triggers" => {
<OPENHUMAN_ROOT>/src/core/observability.rs:238:    /// The subconscious engine's SQLite schema init couldn't open its database
<OPENHUMAN_ROOT>/src/core/observability.rs:243:    ///   `subconscious/` dir or DB file isn't writable/openable (permissions,
<OPENHUMAN_ROOT>/src/core/observability.rs:250:    /// `subconscious::store::apply_journal_mode`, which degrades WAL to a
<OPENHUMAN_ROOT>/src/core/observability.rs:256:    /// Anchored to the subconscious schema/open envelope plus the SQLite
<OPENHUMAN_ROOT>/src/core/observability.rs:260:    SubconsciousSchemaUnavailable,
<OPENHUMAN_ROOT>/src/core/observability.rs:594:    if is_subconscious_schema_unavailable_message(&lower) {
<OPENHUMAN_ROOT>/src/core/observability.rs:595:        return Some(ExpectedErrorKind::SubconsciousSchemaUnavailable);
<OPENHUMAN_ROOT>/src/core/observability.rs:794:/// Match subconscious-engine SQLite schema-init failures caused by the host
<OPENHUMAN_ROOT>/src/core/observability.rs:796:/// `SQLITE_IOERR_SHMMAP`). Anchored to the subconscious open/DDL envelope so it
<OPENHUMAN_ROOT>/src/core/observability.rs:801:/// See [`ExpectedErrorKind::SubconsciousSchemaUnavailable`].
<OPENHUMAN_ROOT>/src/core/observability.rs:802:fn is_subconscious_schema_unavailable_message(lower: &str) -> bool {
<OPENHUMAN_ROOT>/src/core/observability.rs:803:    let in_subconscious_envelope = lower.contains("subconscious schema ddl")
<OPENHUMAN_ROOT>/src/core/observability.rs:804:        || lower.contains("failed to open subconscious db");
<OPENHUMAN_ROOT>/src/core/observability.rs:805:    if !in_subconscious_envelope {
<OPENHUMAN_ROOT>/src/core/observability.rs:1527:    // truth with the subconscious circuit breaker) so the wording can't drift.
<OPENHUMAN_ROOT>/src/core/observability.rs:2197:        ExpectedErrorKind::SubconsciousSchemaUnavailable => {
<OPENHUMAN_ROOT>/src/core/observability.rs:2198:            // Host-filesystem condition: SQLite couldn't open the subconscious
<OPENHUMAN_ROOT>/src/core/observability.rs:2200:            // `subconscious::store` already prevents the shared-memory variant;
<OPENHUMAN_ROOT>/src/core/observability.rs:2207:            // subconscious DB path (home dir / username). Mirror the

```

Output excerpt:

```text
[search: 500 match(es) across 86 file(s) · top 5 per file · full set via retrieve footer]
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:150:    // ── Subconscious orchestrator ───────────────────────────────────────
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:151:    /// A subconscious trigger finished gate evaluation (promote or drop).
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:154:    SubconsciousTriggerProcessed {
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:1404:            Self::SubconsciousTriggerProcessed { .. } => "subconscious",
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:1463:            Self::SubconsciousTriggerProcessed { .. } => "SubconsciousTriggerProcessed",
<OPENHUMAN_ROOT>/README.md:70:- **[A subconscious](https://tinyhumans.gitbook.io/openhuman/features/subconscious)**: a background loop that diffs your world, advances your goals, and writes your morning briefing. Thinkin...
<OPENHUMAN_ROOT>/README.md:78:- **[A split brain, always on](https://tinyhumans.gitbook.io/openhuman/features/orchestration)**: a fast reflex agent triages inbound traffic while a deep reasoning core delegates to worker ...
<OPENHUMAN_ROOT>/AGENTS.md:178:Domains: `about_app`, `accessibility`, `agent`, `app_state`, `approval`, `autocomplete`, `billing`, `channels`, `composio`, `config`, `context`, `cost`, `credentials`, `cron`, `doctor`, `em...
<OPENHUMAN_ROOT>/src/core/cli.rs:80:        "subconscious" | "sub" => {
<OPENHUMAN_ROOT>/src/core/cli.rs:81:            crate::core::subconscious_cli::run_subconscious_command(&args[1..])
<OPENHUMAN_ROOT>/src/core/all.rs:282:    controllers.extend(crate::openhuman::subconscious::all_subconscious_registered_controllers());
<OPENHUMAN_ROOT>/src/core/all.rs:284:        crate::openhuman::subconscious_triggers::all_subconscious_triggers_registered_controllers(),
<OPENHUMAN_ROOT>/src/core/all.rs:461:    schemas.extend(crate::openhuman::subconscious::all_subconscious_controller_schemas());
<OPENHUMAN_ROOT>/src/core/all.rs:463:        crate::openhuman::subconscious_triggers::all_subconscious_triggers_controller_schemas(),
<OPENHUMAN_ROOT>/src/core/all.rs:615:            "Subconscious-orchestration read surface: chat windows (master/subconscious/per-session), message history, Master steering DMs, read state, and steering status.",
[+2 more match(es) in <OPENHUMAN_ROOT>/src/core/all.rs]
<OPENHUMAN_ROOT>/src/core/observability.rs:595:        return Some(ExpectedErrorKind::SubconsciousSchemaUnavailable);
<OPENHUMAN_ROOT>/src/core/observability.rs:794:/// Match subconscious-engine SQLite schema-init failures caused by the host
<OPENHUMAN_ROOT>/src/core/observability.rs:801:/// See [`ExpectedErrorKind::SubconsciousSchemaUnavailable`].
<OPENHUMAN_ROOT>/src/core/observability.rs:804:        || lower.contains("failed to open subconscious db");
<OPENHUMAN_ROOT>/src/core/observability.rs:2197:        ExpectedErrorKind::SubconsciousSchemaUnavailable => {
[+33 more match(es) in <OPENHUMAN_ROOT>/src/core/observability.rs]
<OPENHUMAN_ROOT>/docs/README.ko.md:68:- **[잠재의식(subconscious)](https://tinyhumans.gitbook.io/openhuman/features/subconscious)**: 당신의 세계의 변화를 비교 분석하고, 목표를 진전시키고, 아�...
<OPENHUMAN_ROOT>/src/core/subconscious_cli.rs:1://! `openhuman subconscious` — CLI for testing and debugging the subconscious loop.
<OPENHUMAN_ROOT>/src/core/subconscious_cli.rs:4://!   openhuman subconscious tick [--workspace <path>] [--mode simple|aggressive] [--verbose]
<OPENHUMAN_ROOT>/src/core/subconscious_cli.rs:5://!   openhuman subconscious status [--workspace <path>]
<OPENHUMAN_ROOT>/src/core/subconscious_cli.rs:126:                eprintln!("[subconscious] WARNING: no session token — cloud provider will fail");
<OPENHUMAN_ROOT>/src/core/subconscious_cli.rs:130:                eprintln!("[subconscious] WARNING: session token read failed: {e}");
[+29 more match(es) in <OPENHUMAN_ROOT>/src/core/subconscious_cli.rs]
<OPENHUMAN_ROOT>/src/core/jsonrpc.rs:2087:                    // Subconscious engine + heartbeat.
<OPENHUMAN_ROOT>/src/core/jsonrpc.rs:2089:                        log::info!("[subconscious] disabled by config (heartbeat.enabled = false)");
<OPENHUMAN_ROOT>/src/core/jsonrpc.rs:2091:                        match crate::openhuman::subconscious::registry::bootstrap_after_login()
<OPENHUMAN_ROOT>/src/core/jsonrpc.rs:2095:                                "[subconscious] bootstrapped on startup (existing session)"
<OPENHUMAN_ROOT>/src/core/jsonrpc.rs:2097:                            Err(e) => log::warn!("[subconscious] startup bootstrap failed: {e}"),
<OPENHUMAN_ROOT>/docs/README.zh-CN.md:68:- **[潜意识](https://tinyhumans.gitbook.io/openhuman/features/subconscious)**：一个后台循环，持续比对你的世界的变化、推进你的目标，并为你撰写晨...

```

### `08-rg-tinyplace`

- [Full input](cases/08-rg-tinyplace/input.rg)
- [Full output](cases/08-rg-tinyplace/output.rg)

Input excerpt:

```text
<OPENHUMAN_ROOT>/src/core/jsonrpc_tests.rs:1414:    // message so a wallet-less user's tinyplace RPC stays out of Sentry.
<OPENHUMAN_ROOT>/src/core/jsonrpc_tests.rs:1436:        "tinyplace signer init: bad seed"
<OPENHUMAN_ROOT>/src/core/socketio.rs:631:    let io_tinyplace = io.clone();
<OPENHUMAN_ROOT>/src/core/socketio.rs:722:    //     TinyPlaceOrchestrationTab targeted-refetches the affected chat live
<OPENHUMAN_ROOT>/src/core/socketio.rs:1221:    // 10. Tinyplace stream events → broadcast to all connected frontend sockets.
<OPENHUMAN_ROOT>/src/core/socketio.rs:1235:                        "[socketio] event_bus not initialised after {}s — tinyplace bridge giving up",
<OPENHUMAN_ROOT>/src/core/socketio.rs:1249:                        "[socketio] dropped {} event_bus events due to lag (tinyplace bridge)",
<OPENHUMAN_ROOT>/src/core/socketio.rs:1257:                crate::core::event_bus::DomainEvent::TinyPlaceStreamMessage {
<OPENHUMAN_ROOT>/src/core/socketio.rs:1268:                        "[socketio] broadcast tinyplace:stream_message stream_id={} kind={}",
<OPENHUMAN_ROOT>/src/core/socketio.rs:1272:                    let _ = io_tinyplace.emit("tinyplace:stream_message", &payload);
<OPENHUMAN_ROOT>/src/core/socketio.rs:1274:                crate::core::event_bus::DomainEvent::TinyPlaceStreamStatusChanged {
<OPENHUMAN_ROOT>/src/core/socketio.rs:1283:                        "[socketio] broadcast tinyplace:stream_status stream_id={} status={}",
<OPENHUMAN_ROOT>/src/core/socketio.rs:1287:                    let _ = io_tinyplace.emit("tinyplace:stream_status", &payload);
<OPENHUMAN_ROOT>/src/core/socketio.rs:1292:        log::debug!("[socketio] tinyplace stream bridge stopped");
<OPENHUMAN_ROOT>/src/core/jsonrpc.rs:107:                // A `tinyplace_*` RPC needs a wallet-derived signer but the user
<OPENHUMAN_ROOT>/src/core/jsonrpc.rs:385:/// Several `tinyplace_*` RPCs derive a signer seed from the wallet before they
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:1253:    /// A JSON message arrived on a tinyplace WebSocket stream.
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:1257:    TinyPlaceStreamMessage {
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:1262:        /// The raw JSON message from the tinyplace server.
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:1265:    /// A tinyplace WebSocket stream changed lifecycle status.
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:1267:    TinyPlaceStreamStatusChanged {
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:1441:            Self::TinyPlaceStreamMessage { .. } | Self::TinyPlaceStreamStatusChanged { .. } => {
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:1442:                "tinyplace"
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:1584:            Self::TinyPlaceStreamMessage { .. } => "TinyPlaceStreamMessage",
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:1585:            Self::TinyPlaceStreamStatusChanged { .. } => "TinyPlaceStreamStatusChanged",
<OPENHUMAN_ROOT>/README.md:79:- **[An agent economy](https://tinyhumans.gitbook.io/openhuman/features/tinyplace)**: a `@handle` on [tiny.place](https://tiny.place), Signal-encrypted agent-to-agent orchestration, x402 USD...
<OPENHUMAN_ROOT>/src/core/all.rs:360:    controllers.extend(crate::openhuman::tinyplace::all_tinyplace_registered_controllers());
<OPENHUMAN_ROOT>/src/core/all.rs:364:    // Orchestration read surface (stage 7): the TinyPlaceOrchestrationTab reads
<OPENHUMAN_ROOT>/src/core/all.rs:685:        "tinyplace" => Some(
<OPENHUMAN_ROOT>/plan.md:170:- **~20 RPC controller domains with zero E2E references** (`recall_calendar`, `tinyplace`,
<OPENHUMAN_ROOT>/plan.md:497:  real backend-facing surface: `recall_calendar`, `tinyplace`, `redirect_links`,
<OPENHUMAN_ROOT>/docs/README.ko.md:77:- **[에이전트 경제](https://tinyhumans.gitbook.io/openhuman/features/tinyplace)**: [tiny.place](https://tiny.place)의 `@handle`, Signal로 암호화된 에이전트 간 오케...
<OPENHUMAN_ROOT>/Cargo.toml:44:tinyplace = "2.0"
<OPENHUMAN_ROOT>/Cargo.toml:359:# TinyFlows, TinyCortex, TinyJuice, TinyChannels, and TinyPlace are vendored beside
<OPENHUMAN_ROOT>/Cargo.toml:366:tinyplace = { path = "vendor/tinyplace/sdk/rust" }
<OPENHUMAN_ROOT>/docs/README.zh-CN.md:77:- **[智能体经济](https://tinyhumans.gitbook.io/openhuman/features/tinyplace)**：在 [tiny.place](https://tiny.place) 上的 `@handle`、Signal 加密的智能体间编排、...

```

Output excerpt:

```text
[search: 500 match(es) across 88 file(s) · top 5 per file · full set via retrieve footer]
<OPENHUMAN_ROOT>/src/core/jsonrpc_tests.rs:1414:    // message so a wallet-less user's tinyplace RPC stays out of Sentry.
<OPENHUMAN_ROOT>/src/core/jsonrpc_tests.rs:1436:        "tinyplace signer init: bad seed"
<OPENHUMAN_ROOT>/src/core/socketio.rs:631:    let io_tinyplace = io.clone();
<OPENHUMAN_ROOT>/src/core/socketio.rs:722:    //     TinyPlaceOrchestrationTab targeted-refetches the affected chat live
<OPENHUMAN_ROOT>/src/core/socketio.rs:1221:    // 10. Tinyplace stream events → broadcast to all connected frontend sockets.
<OPENHUMAN_ROOT>/src/core/socketio.rs:1235:                        "[socketio] event_bus not initialised after {}s — tinyplace bridge giving up",
<OPENHUMAN_ROOT>/src/core/socketio.rs:1249:                        "[socketio] dropped {} event_bus events due to lag (tinyplace bridge)",
[+7 more match(es) in <OPENHUMAN_ROOT>/src/core/socketio.rs]
<OPENHUMAN_ROOT>/src/core/jsonrpc.rs:107:                // A `tinyplace_*` RPC needs a wallet-derived signer but the user
<OPENHUMAN_ROOT>/src/core/jsonrpc.rs:385:/// Several `tinyplace_*` RPCs derive a signer seed from the wallet before they
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:1253:    /// A JSON message arrived on a tinyplace WebSocket stream.
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:1257:    TinyPlaceStreamMessage {
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:1262:        /// The raw JSON message from the tinyplace server.
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:1265:    /// A tinyplace WebSocket stream changed lifecycle status.
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:1267:    TinyPlaceStreamStatusChanged {
[+4 more match(es) in <OPENHUMAN_ROOT>/src/core/event_bus/events.rs]
<OPENHUMAN_ROOT>/README.md:79:- **[An agent economy](https://tinyhumans.gitbook.io/openhuman/features/tinyplace)**: a `@handle` on [tiny.place](https://tiny.place), Signal-encrypted agent-to-agent orchestration, x402 USD...
<OPENHUMAN_ROOT>/src/core/all.rs:360:    controllers.extend(crate::openhuman::tinyplace::all_tinyplace_registered_controllers());
<OPENHUMAN_ROOT>/src/core/all.rs:364:    // Orchestration read surface (stage 7): the TinyPlaceOrchestrationTab reads
<OPENHUMAN_ROOT>/src/core/all.rs:685:        "tinyplace" => Some(
<OPENHUMAN_ROOT>/plan.md:170:- **~20 RPC controller domains with zero E2E references** (`recall_calendar`, `tinyplace`,
<OPENHUMAN_ROOT>/plan.md:497:  real backend-facing surface: `recall_calendar`, `tinyplace`, `redirect_links`,
<OPENHUMAN_ROOT>/docs/README.ko.md:77:- **[에이전트 경제](https://tinyhumans.gitbook.io/openhuman/features/tinyplace)**: [tiny.place](https://tiny.place)의 `@handle`, Signal로 암호화된 에이전트 간 오케...
<OPENHUMAN_ROOT>/Cargo.toml:44:tinyplace = "2.0"
<OPENHUMAN_ROOT>/Cargo.toml:359:# TinyFlows, TinyCortex, TinyJuice, TinyChannels, and TinyPlace are vendored beside
<OPENHUMAN_ROOT>/Cargo.toml:366:tinyplace = { path = "vendor/tinyplace/sdk/rust" }
<OPENHUMAN_ROOT>/docs/README.zh-CN.md:77:- **[智能体经济](https://tinyhumans.gitbook.io/openhuman/features/tinyplace)**：在 [tiny.place](https://tiny.place) 上的 `@handle`、Signal 加密的智能体间编排、...
<OPENHUMAN_ROOT>/docs/README.de.md:77:- **[Eine Agenten-Ökonomie](https://tinyhumans.gitbook.io/openhuman/features/tinyplace)**: ein `@handle` auf [tiny.place](https://tiny.place), Signal-verschlüsselte Agent-zu-Agent-...
<OPENHUMAN_ROOT>/docs/plans/subconscious-factory/phase-4-factory-registry-rpc.md:11:pub enum SubconsciousKind { Memory, TinyPlace }
<OPENHUMAN_ROOT>/docs/plans/subconscious-factory/phase-4-factory-registry-rpc.md:14:    pub fn id(self) -> &'static str;               // "memory" | "tinyplace"
<OPENHUMAN_ROOT>/docs/plans/subconscious-factory/phase-4-factory-registry-rpc.md:19:        // TinyPlace ⇐ orchestration.enabled                   (today's gate)
<OPENHUMAN_ROOT>/docs/plans/subconscious-factory/phase-4-factory-registry-rpc.md:62:cancel/abort semantics) — a slow memory tick must not delay a tinyplace
<OPENHUMAN_ROOT>/docs/plans/subconscious-factory/phase-4-factory-registry-rpc.md:73:  (each row gains `instance: "memory" | "tinyplace"`).
[+2 more match(es) in <OPENHUMAN_ROOT>/docs/plans/subconscious-factory/phase-4-factory-registry-rpc.md]
<OPENHUMAN_ROOT>/docs/plans/subconscious-factory/README.md:15:- **`tinyplace`** — the tiny.place orchestration world: harness-session

```

### `01-rg-tokenjuice`

- [Full input](cases/01-rg-tokenjuice/input.rg)
- [Full output](cases/01-rg-tokenjuice/output.rg)

Input excerpt:

```text
<OPENHUMAN_ROOT>/Cargo.toml:51:# TinyJuice — host-agnostic TokenJuice compression engine. OpenHuman keeps
<OPENHUMAN_ROOT>/Cargo.toml:52:# config/RPC/tool/runtime adapters in `src/openhuman/tokenjuice/` and patches
<OPENHUMAN_ROOT>/Cargo.toml:79:# TokenJuice code compressor — AST-aware signature extraction. Optional (C build)
<OPENHUMAN_ROOT>/Cargo.toml:80:# behind the default `tokenjuice-treesitter` feature; disabling it falls back to
<OPENHUMAN_ROOT>/Cargo.toml:81:# the language-agnostic brace-depth heuristic. See src/openhuman/tokenjuice/compressors/code.rs.
<OPENHUMAN_ROOT>/Cargo.toml:326:default = ["tokenjuice-treesitter"]
<OPENHUMAN_ROOT>/Cargo.toml:329:tokenjuice-treesitter = [
<OPENHUMAN_ROOT>/Cargo.toml:330:    "tinyjuice/tokenjuice-treesitter",
<OPENHUMAN_ROOT>/README.md:72:- **[TokenJuice](https://tinyhumans.gitbook.io/openhuman/features/token-compression)**: tool output compressed before it hits the model: same information, up to 80% fewer tokens. A brain thi...
<OPENHUMAN_ROOT>/README.md:145:| **Cost**               | ⚠️ Sub + add-ons  | ⚠️ BYO models     | ⚠️ BYO models     | ✅ One sub + TokenJuice                                                                  ...
<OPENHUMAN_ROOT>/docs/README.ko.md:70:- **[TokenJuice](https://tinyhumans.gitbook.io/openhuman/features/token-compression)**: 도구 출력은 모델에 닿기 전에 압축되어, 동일한 정보가 최대 80% 적은 �...
<OPENHUMAN_ROOT>/docs/README.ko.md:143:| **비용**           | ⚠️ 구독 + 애드온  | ⚠️ 모델 직접 제공 | ⚠️ 모델 직접 제공 | ✅ 단일 구독 + TokenJuice                                     ...
<OPENHUMAN_ROOT>/docs/README.ja-JP.md:70:- **[TokenJuice](https://tinyhumans.gitbook.io/openhuman/features/token-compression)**: ツール出力はモデルに届く前に圧縮され、同じ情報を最大 80% 少な�...
<OPENHUMAN_ROOT>/docs/README.ja-JP.md:143:| **コスト**                 | ⚠️ サブスク + アドオン | ⚠️ モデル持ち込み   | ⚠️ モデル持ち込み   | ✅ 1 つのサブスク + TokenJuice    ...
<OPENHUMAN_ROOT>/docs/tinyagents-port-plan.md:166:1. Delete transitional shims (`ToolAdapter` test-only wrapper, `subagent_graph.rs` no-op skeleton once the graph path is the real one, `retrieve_tool_output` vs tokenjuic...
<OPENHUMAN_ROOT>/docs/README.zh-CN.md:70:- **[TokenJuice](https://tinyhumans.gitbook.io/openhuman/features/token-compression)**：工具输出在触达模型之前先被压缩：信息不变，token 最多减少 80%。�...
<OPENHUMAN_ROOT>/docs/README.zh-CN.md:143:| **成本**       | ⚠️ 订阅 + 附加项 | ⚠️ 自带模型 | ⚠️ 自带模型  | ✅ 单一订阅 + TokenJuice                                                       ...
<OPENHUMAN_ROOT>/docs/README.de.md:70:- **[TokenJuice](https://tinyhumans.gitbook.io/openhuman/features/token-compression)**: Tool-Ausgaben werden komprimiert, bevor sie das Modell erreichen: dieselbe Information, bis zu...
<OPENHUMAN_ROOT>/docs/README.de.md:143:| **Kosten**             | ⚠️ Abo + Zusatzkosten | ⚠️ BYO-Modelle     | ⚠️ BYO-Modelle     | ✅ Ein Abo + TokenJuice                                                    ...
<OPENHUMAN_ROOT>/tests/agent_archivist_debug_round21_raw_coverage_e2e.rs:23:use openhuman_core::openhuman::tokenjuice::AgentTokenjuiceCompression;
<OPENHUMAN_ROOT>/tests/agent_archivist_debug_round21_raw_coverage_e2e.rs:262:        tokenjuice_compression: AgentTokenjuiceCompression::Auto,
<OPENHUMAN_ROOT>/docs/README.ur-pk.md:84:- **[TokenJuice](https://tinyhumans.gitbook.io/openhuman/features/token-compression)**: ٹول آؤٹ پٹ ماڈل تک پہنچنے سے پہلے کمپریس ہوتا ہے: و�...
<OPENHUMAN_ROOT>/docs/README.ur-pk.md:185:| **لاگت**           | ⚠️ سبسکرپشن + ایڈ آنز    | ⚠️ اپنے ماڈل        | ⚠️ اپنے ماڈل        | ✅ ایک سبسکرپشن + TokenJui...
<OPENHUMAN_ROOT>/gitbooks/README.md:23:* **An agent built for big data.** [Smart token compression (TokenJuice)](features/token-compression.md) compacts verbose tool output before it ever enters the model's context, so s...
<OPENHUMAN_ROOT>/vendor/tinyjuice/src/classify.rs:151:            "[tokenjuice] forced classification: rule='{}' family='{}'",
<OPENHUMAN_ROOT>/vendor/tinyjuice/src/classify.rs:170:            "[tokenjuice] no rule matched tool='{}' argv={:?} — using generic fallback",
<OPENHUMAN_ROOT>/vendor/tinyjuice/src/classify.rs:199:        "[tokenjuice] classified tool='{}' → rule='{}' family='{}' confidence={}",
<OPENHUMAN_ROOT>/gitbooks/developing/architecture/agent-harness.md:172:### TokenJuice - content-aware tool-output compaction (Stage 1a)
<OPENHUMAN_ROOT>/gitbooks/developing/architecture/agent-harness.md:174:Before a fresh tool result enters history (and ahead of the byte-budget backstop), it passes through the **TokenJuice content router** in the vendore...
<OPENHUMAN_ROOT>/gitbooks/developing/architecture/agent-harness.md:184:Every lossy compression offloads the original to the **CCR (Compress-Cache-Retrieve)** store behind a `⟦tj:<hash>⟧` marker, so compaction is effe...
<OPENHUMAN_ROOT>/vendor/tinyjuice/src/types.rs:1://! Core type definitions for the TokenJuice reduction engine.
<OPENHUMAN_ROOT>/vendor/tinyjuice/src/types.rs:304:/// Per-agent TokenJuice profile.
<OPENHUMAN_ROOT>/vendor/tinyjuice/src/types.rs:306:/// `Auto` is resolved by the agent definition layer. TokenJuice itself treats
<OPENHUMAN_ROOT>/vendor/tinyjuice/src/types.rs:307:/// `Auto` like `Full` so non-agent callers keep the global `[tokenjuice]`
<OPENHUMAN_ROOT>/vendor/tinyjuice/src/types.rs:311:pub enum AgentTokenjuiceCompression {
<OPENHUMAN_ROOT>/vendor/tinyjuice/src/types.rs:316:    /// Use the process-global TokenJuice configuration unchanged.

```

Output excerpt:

```text
[search: 500 match(es) across 108 file(s) · top 5 per file · full set via retrieve footer]
<OPENHUMAN_ROOT>/Cargo.toml:51:# TinyJuice — host-agnostic TokenJuice compression engine. OpenHuman keeps
<OPENHUMAN_ROOT>/Cargo.toml:52:# config/RPC/tool/runtime adapters in `src/openhuman/tokenjuice/` and patches
<OPENHUMAN_ROOT>/Cargo.toml:79:# TokenJuice code compressor — AST-aware signature extraction. Optional (C build)
<OPENHUMAN_ROOT>/Cargo.toml:80:# behind the default `tokenjuice-treesitter` feature; disabling it falls back to
<OPENHUMAN_ROOT>/Cargo.toml:81:# the language-agnostic brace-depth heuristic. See src/openhuman/tokenjuice/compressors/code.rs.
[+3 more match(es) in <OPENHUMAN_ROOT>/Cargo.toml]
<OPENHUMAN_ROOT>/README.md:72:- **[TokenJuice](https://tinyhumans.gitbook.io/openhuman/features/token-compression)**: tool output compressed before it hits the model: same information, up to 80% fewer tokens. A brain thi...
<OPENHUMAN_ROOT>/README.md:145:| **Cost**               | ⚠️ Sub + add-ons  | ⚠️ BYO models     | ⚠️ BYO models     | ✅ One sub + TokenJuice                                                                  ...
<OPENHUMAN_ROOT>/docs/README.ko.md:70:- **[TokenJuice](https://tinyhumans.gitbook.io/openhuman/features/token-compression)**: 도구 출력은 모델에 닿기 전에 압축되어, 동일한 정보가 최대 80% 적은 �...
<OPENHUMAN_ROOT>/docs/README.ko.md:143:| **비용**           | ⚠️ 구독 + 애드온  | ⚠️ 모델 직접 제공 | ⚠️ 모델 직접 제공 | ✅ 단일 구독 + TokenJuice                                     ...
<OPENHUMAN_ROOT>/docs/README.ja-JP.md:70:- **[TokenJuice](https://tinyhumans.gitbook.io/openhuman/features/token-compression)**: ツール出力はモデルに届く前に圧縮され、同じ情報を最大 80% 少な�...
<OPENHUMAN_ROOT>/docs/README.ja-JP.md:143:| **コスト**                 | ⚠️ サブスク + アドオン | ⚠️ モデル持ち込み   | ⚠️ モデル持ち込み   | ✅ 1 つのサブスク + TokenJuice    ...
<OPENHUMAN_ROOT>/docs/tinyagents-port-plan.md:166:1. Delete transitional shims (`ToolAdapter` test-only wrapper, `subagent_graph.rs` no-op skeleton once the graph path is the real one, `retrieve_tool_output` vs tokenjuic...
<OPENHUMAN_ROOT>/docs/README.zh-CN.md:70:- **[TokenJuice](https://tinyhumans.gitbook.io/openhuman/features/token-compression)**：工具输出在触达模型之前先被压缩：信息不变，token 最多减少 80%。�...
<OPENHUMAN_ROOT>/docs/README.zh-CN.md:143:| **成本**       | ⚠️ 订阅 + 附加项 | ⚠️ 自带模型 | ⚠️ 自带模型  | ✅ 单一订阅 + TokenJuice                                                       ...
<OPENHUMAN_ROOT>/docs/README.de.md:70:- **[TokenJuice](https://tinyhumans.gitbook.io/openhuman/features/token-compression)**: Tool-Ausgaben werden komprimiert, bevor sie das Modell erreichen: dieselbe Information, bis zu...
<OPENHUMAN_ROOT>/docs/README.de.md:143:| **Kosten**             | ⚠️ Abo + Zusatzkosten | ⚠️ BYO-Modelle     | ⚠️ BYO-Modelle     | ✅ Ein Abo + TokenJuice                                                    ...
<OPENHUMAN_ROOT>/tests/agent_archivist_debug_round21_raw_coverage_e2e.rs:23:use openhuman_core::openhuman::tokenjuice::AgentTokenjuiceCompression;
<OPENHUMAN_ROOT>/tests/agent_archivist_debug_round21_raw_coverage_e2e.rs:262:        tokenjuice_compression: AgentTokenjuiceCompression::Auto,
<OPENHUMAN_ROOT>/docs/README.ur-pk.md:84:- **[TokenJuice](https://tinyhumans.gitbook.io/openhuman/features/token-compression)**: ٹول آؤٹ پٹ ماڈل تک پہنچنے سے پہلے کمپریس ہوتا ہے: و�...
<OPENHUMAN_ROOT>/docs/README.ur-pk.md:185:| **لاگت**           | ⚠️ سبسکرپشن + ایڈ آنز    | ⚠️ اپنے ماڈل        | ⚠️ اپنے ماڈل        | ✅ ایک سبسکرپشن + TokenJui...
<OPENHUMAN_ROOT>/gitbooks/README.md:23:* **An agent built for big data.** [Smart token compression (TokenJuice)](features/token-compression.md) compacts verbose tool output before it ever enters the model's context, so s...
<OPENHUMAN_ROOT>/vendor/tinyjuice/src/classify.rs:151:            "[tokenjuice] forced classification: rule='{}' family='{}'",
<OPENHUMAN_ROOT>/vendor/tinyjuice/src/classify.rs:170:            "[tokenjuice] no rule matched tool='{}' argv={:?} — using generic fallback",
<OPENHUMAN_ROOT>/vendor/tinyjuice/src/classify.rs:199:        "[tokenjuice] classified tool='{}' → rule='{}' family='{}' confidence={}",
<OPENHUMAN_ROOT>/gitbooks/developing/architecture/agent-harness.md:172:### TokenJuice - content-aware tool-output compaction (Stage 1a)
<OPENHUMAN_ROOT>/gitbooks/developing/architecture/agent-harness.md:174:Before a fresh tool result enters history (and ahead of the byte-budget backstop), it passes through the **TokenJuice content router** in the vendore...
<OPENHUMAN_ROOT>/gitbooks/developing/architecture/agent-harness.md:184:Every lossy compression offloads the original to the **CCR (Compress-Cache-Retrieve)** store behind a `⟦tj:<hash>⟧` marker, so compaction is effe...
<OPENHUMAN_ROOT>/vendor/tinyjuice/src/types.rs:1://! Core type definitions for the TokenJuice reduction engine.
<OPENHUMAN_ROOT>/vendor/tinyjuice/src/types.rs:304:/// Per-agent TokenJuice profile.
<OPENHUMAN_ROOT>/vendor/tinyjuice/src/types.rs:306:/// `Auto` is resolved by the agent definition layer. TokenJuice itself treats
<OPENHUMAN_ROOT>/vendor/tinyjuice/src/types.rs:311:pub enum AgentTokenjuiceCompression {
<OPENHUMAN_ROOT>/vendor/tinyjuice/src/types.rs:324:impl AgentTokenjuiceCompression {
[+5 more match(es) in <OPENHUMAN_ROOT>/vendor/tinyjuice/src/types.rs]
<OPENHUMAN_ROOT>/vendor/tinyjuice/src/tool_integration.rs:23:use super::types::{AgentTokenjuiceCompression, CompressInput, CompressOptions, ContentHint};

```

### `02-rg-compression`

- [Full input](cases/02-rg-compression/input.rg)
- [Full output](cases/02-rg-compression/output.rg)

Input excerpt:

```text
<OPENHUMAN_ROOT>/src/core/event_bus/README.md:48:  compression, tool-exposure, and steering signals ride the TinyAgents
<OPENHUMAN_ROOT>/README.md:72:- **[TokenJuice](https://tinyhumans.gitbook.io/openhuman/features/token-compression)**: tool output compressed before it hits the model: same information, up to 80% fewer tokens. A brain thi...
<OPENHUMAN_ROOT>/Cargo.lock:261:name = "async-compression"
<OPENHUMAN_ROOT>/Cargo.lock:266: "compression-codecs",
<OPENHUMAN_ROOT>/Cargo.lock:267: "compression-core",
<OPENHUMAN_ROOT>/Cargo.lock:279: "async-compression",
<OPENHUMAN_ROOT>/Cargo.lock:1042:name = "compression-codecs"
<OPENHUMAN_ROOT>/Cargo.lock:1047: "compression-core",
<OPENHUMAN_ROOT>/Cargo.lock:1052:name = "compression-core"
<OPENHUMAN_ROOT>/docs/README.ko.md:70:- **[TokenJuice](https://tinyhumans.gitbook.io/openhuman/features/token-compression)**: 도구 출력은 모델에 닿기 전에 압축되어, 동일한 정보가 최대 80% 적은 �...
<OPENHUMAN_ROOT>/gitbooks/SUMMARY.md:44:* [Smart Token Compression](features/token-compression.md)
<OPENHUMAN_ROOT>/docs/README.zh-CN.md:70:- **[TokenJuice](https://tinyhumans.gitbook.io/openhuman/features/token-compression)**：工具输出在触达模型之前先被压缩：信息不变，token 最多减少 80%。�...
<OPENHUMAN_ROOT>/src/core/all.rs:651:            Some("Hierarchical time-based summarization tree for background knowledge compression.")
<OPENHUMAN_ROOT>/gitbooks/features/subconscious.md:196:* Long sessions stay bounded by **20:1 history compression** plus a rolling world-state diff with utilization-based eviction.
<OPENHUMAN_ROOT>/docs/README.de.md:70:- **[TokenJuice](https://tinyhumans.gitbook.io/openhuman/features/token-compression)**: Tool-Ausgaben werden komprimiert, bevor sie das Modell erreichen: dieselbe Information, bis zu...
<OPENHUMAN_ROOT>/gitbooks/features/privacy-and-security.md:65:Compression and locality together become the privacy architecture.
<OPENHUMAN_ROOT>/docs/README.ur-pk.md:84:- **[TokenJuice](https://tinyhumans.gitbook.io/openhuman/features/token-compression)**: ٹول آؤٹ پٹ ماڈل تک پہنچنے سے پہلے کمپریس ہوتا ہے: و�...
<OPENHUMAN_ROOT>/gitbooks/developing/architecture/agent-harness.md:145:**One engine, three entry points.** The loop lives in one place (the tinyagents `AgentHarness`, entered via `run_turn_via_tinyagents_shared` in `src/...
<OPENHUMAN_ROOT>/gitbooks/developing/architecture/agent-harness.md:164:* **Microcompact / autocompact** - when total history is creeping toward the context window, tinyagents middleware (message trimming + the compressio...
<OPENHUMAN_ROOT>/gitbooks/developing/architecture/agent-harness.md:184:Every lossy compression offloads the original to the **CCR (Compress-Cache-Retrieve)** store behind a `⟦tj:<hash>⟧` marker, so compaction is effe...
<OPENHUMAN_ROOT>/gitbooks/features/model-routing/README.md:40:| `hint:summarize` | A model good at compression | Memory tree summary builders |
<OPENHUMAN_ROOT>/gitbooks/features/model-routing/README.md:98:- [Smart Token Compression](../token-compression.md). what makes large reasoning calls affordable.
<OPENHUMAN_ROOT>/gitbooks/README.md:23:* **An agent built for big data.** [Smart token compression (TokenJuice)](features/token-compression.md) compacts verbose tool output before it ever enters the model's context, so s...
<OPENHUMAN_ROOT>/gitbooks/features/native-tools/web-scraper.md:31:* [Smart Token Compression](../token-compression.md) - what trims long pages before they hit the model.
<OPENHUMAN_ROOT>/gitbooks/features/billing-and-usage.md:100:## Cost & token compression
<OPENHUMAN_ROOT>/gitbooks/features/billing-and-usage.md:102:Because cost tracks **real token counts**, anything that shrinks the prompt directly lowers spend. OpenHuman's [TokenJuice token compression](token-compression....
<OPENHUMAN_ROOT>/gitbooks/features/billing-and-usage.md:108:- [Token compression (TokenJuice)](token-compression.md)
<OPENHUMAN_ROOT>/docs/README.ja-JP.md:70:- **[TokenJuice](https://tinyhumans.gitbook.io/openhuman/features/token-compression)**: ツール出力はモデルに届く前に圧縮され、同じ情報を最大 80% 少な�...
<OPENHUMAN_ROOT>/gitbooks/features/token-compression.md:3:  TokenJuice - a multi-stage compression router that compacts verbose tool
<OPENHUMAN_ROOT>/gitbooks/features/token-compression.md:8:# Smart Token Compression
<OPENHUMAN_ROOT>/gitbooks/features/token-compression.md:12:OpenHuman ships with **TokenJuice**, a compression router wired directly into the agent's tool-execution path. Before any tool result reaches a model, TokenJuice...
<OPENHUMAN_ROOT>/gitbooks/features/token-compression.md:53:3. **Compressor selection.** Each kind routes to a dedicated compressor, honoring per-kind toggles (`search_enabled`, `code_enabled`, `html_enabled`, `ml_compres...
<OPENHUMAN_ROOT>/gitbooks/features/token-compression.md:54:4. **Compression.** The compressor runs. If it declines or its output is no smaller than the input, TokenJuice falls back to the generic compressor or passes the...
<OPENHUMAN_ROOT>/gitbooks/features/token-compression.md:55:5. **CCR offload.** For **lossy** compressions where the original is large enough (`ccr_min_tokens`, default ~500 tokens), the full original is stowed in the **C...
<OPENHUMAN_ROOT>/gitbooks/features/token-compression.md:73:| **MlText**       | PlainText   | Opt-in ML salience compression (see below).                                                              |
<OPENHUMAN_ROOT>/gitbooks/features/token-compression.md:80:## ML compression (opt-in)

```

Output excerpt:

```text
[search: 500 match(es) across 158 file(s) · top 5 per file · full set via retrieve footer]
<OPENHUMAN_ROOT>/src/core/event_bus/README.md:48:  compression, tool-exposure, and steering signals ride the TinyAgents
<OPENHUMAN_ROOT>/README.md:72:- **[TokenJuice](https://tinyhumans.gitbook.io/openhuman/features/token-compression)**: tool output compressed before it hits the model: same information, up to 80% fewer tokens. A brain thi...
<OPENHUMAN_ROOT>/Cargo.lock:261:name = "async-compression"
<OPENHUMAN_ROOT>/Cargo.lock:266: "compression-codecs",
<OPENHUMAN_ROOT>/Cargo.lock:267: "compression-core",
<OPENHUMAN_ROOT>/Cargo.lock:279: "async-compression",
<OPENHUMAN_ROOT>/Cargo.lock:1042:name = "compression-codecs"
[+2 more match(es) in <OPENHUMAN_ROOT>/Cargo.lock]
<OPENHUMAN_ROOT>/docs/README.ko.md:70:- **[TokenJuice](https://tinyhumans.gitbook.io/openhuman/features/token-compression)**: 도구 출력은 모델에 닿기 전에 압축되어, 동일한 정보가 최대 80% 적은 �...
<OPENHUMAN_ROOT>/gitbooks/SUMMARY.md:44:* [Smart Token Compression](features/token-compression.md)
<OPENHUMAN_ROOT>/docs/README.zh-CN.md:70:- **[TokenJuice](https://tinyhumans.gitbook.io/openhuman/features/token-compression)**：工具输出在触达模型之前先被压缩：信息不变，token 最多减少 80%。�...
<OPENHUMAN_ROOT>/src/core/all.rs:651:            Some("Hierarchical time-based summarization tree for background knowledge compression.")
<OPENHUMAN_ROOT>/gitbooks/features/subconscious.md:196:* Long sessions stay bounded by **20:1 history compression** plus a rolling world-state diff with utilization-based eviction.
<OPENHUMAN_ROOT>/docs/README.de.md:70:- **[TokenJuice](https://tinyhumans.gitbook.io/openhuman/features/token-compression)**: Tool-Ausgaben werden komprimiert, bevor sie das Modell erreichen: dieselbe Information, bis zu...
<OPENHUMAN_ROOT>/gitbooks/features/privacy-and-security.md:65:Compression and locality together become the privacy architecture.
<OPENHUMAN_ROOT>/docs/README.ur-pk.md:84:- **[TokenJuice](https://tinyhumans.gitbook.io/openhuman/features/token-compression)**: ٹول آؤٹ پٹ ماڈل تک پہنچنے سے پہلے کمپریس ہوتا ہے: و�...
<OPENHUMAN_ROOT>/gitbooks/developing/architecture/agent-harness.md:145:**One engine, three entry points.** The loop lives in one place (the tinyagents `AgentHarness`, entered via `run_turn_via_tinyagents_shared` in `src/...
<OPENHUMAN_ROOT>/gitbooks/developing/architecture/agent-harness.md:164:* **Microcompact / autocompact** - when total history is creeping toward the context window, tinyagents middleware (message trimming + the compressio...
<OPENHUMAN_ROOT>/gitbooks/developing/architecture/agent-harness.md:184:Every lossy compression offloads the original to the **CCR (Compress-Cache-Retrieve)** store behind a `⟦tj:<hash>⟧` marker, so compaction is effe...
<OPENHUMAN_ROOT>/gitbooks/features/model-routing/README.md:40:| `hint:summarize` | A model good at compression | Memory tree summary builders |
<OPENHUMAN_ROOT>/gitbooks/features/model-routing/README.md:98:- [Smart Token Compression](../token-compression.md). what makes large reasoning calls affordable.
<OPENHUMAN_ROOT>/gitbooks/README.md:23:* **An agent built for big data.** [Smart token compression (TokenJuice)](features/token-compression.md) compacts verbose tool output before it ever enters the model's context, so s...
<OPENHUMAN_ROOT>/gitbooks/features/native-tools/web-scraper.md:31:* [Smart Token Compression](../token-compression.md) - what trims long pages before they hit the model.
<OPENHUMAN_ROOT>/gitbooks/features/billing-and-usage.md:100:## Cost & token compression
<OPENHUMAN_ROOT>/gitbooks/features/billing-and-usage.md:102:Because cost tracks **real token counts**, anything that shrinks the prompt directly lowers spend. OpenHuman's [TokenJuice token compression](token-compression....
<OPENHUMAN_ROOT>/gitbooks/features/billing-and-usage.md:108:- [Token compression (TokenJuice)](token-compression.md)
<OPENHUMAN_ROOT>/docs/README.ja-JP.md:70:- **[TokenJuice](https://tinyhumans.gitbook.io/openhuman/features/token-compression)**: ツール出力はモデルに届く前に圧縮され、同じ情報を最大 80% 少な�...
<OPENHUMAN_ROOT>/gitbooks/features/token-compression.md:3:  TokenJuice - a multi-stage compression router that compacts verbose tool
<OPENHUMAN_ROOT>/gitbooks/features/token-compression.md:12:OpenHuman ships with **TokenJuice**, a compression router wired directly into the agent's tool-execution path. Before any tool result reaches a model, TokenJuice...
<OPENHUMAN_ROOT>/gitbooks/features/token-compression.md:54:4. **Compression.** The compressor runs. If it declines or its output is no smaller than the input, TokenJuice falls back to the generic compressor or passes the...
<OPENHUMAN_ROOT>/gitbooks/features/token-compression.md:84:* **Off by default.** Enable with `ml_compression_enabled = true` in `[tokenjuice]`.
<OPENHUMAN_ROOT>/gitbooks/features/token-compression.md:93:Lossy compression would normally mean throwing data away. TokenJuice instead **offloads** the full original into the **Compress-Cache-Retrieve (CCR)** store and ...
[+6 more match(es) in <OPENHUMAN_ROOT>/gitbooks/features/token-compression.md]
<OPENHUMAN_ROOT>/gitbooks/developing/architecture/README.md:28:│ • TokenJuice compression │
<OPENHUMAN_ROOT>/gitbooks/developing/architecture/README.md:57:9. **Compress**. Tool output and large source data go through [TokenJuice](../../features/token-compression.md) before entering LLM context.

```

### `07-rg-workflow`

- [Full input](cases/07-rg-workflow/input.rg)
- [Full output](cases/07-rg-workflow/output.rg)

Input excerpt:

```text
<OPENHUMAN_ROOT>/src/main.rs:139:            // suppression lives at the `install_workflow_from_url_with_home`
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:460:    /// non-trigger node settles, so the Workflows UI can show a run advancing
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:477:    WorkflowLoaded { skill_id: String, runtime: String },
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:479:    WorkflowStopped { skill_id: String },
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:481:    WorkflowStartFailed { skill_id: String, error: String },
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:483:    WorkflowExecuted {
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:491:    /// The set of installed skills/workflows changed (install / uninstall /
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:495:    WorkflowsChanged { reason: String },
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:1335:            Self::WorkflowLoaded { .. }
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:1336:            | Self::WorkflowStopped { .. }
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:1337:            | Self::WorkflowStartFailed { .. }
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:1338:            | Self::WorkflowExecuted { .. }
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:1339:            | Self::WorkflowsChanged { .. } => "workflow",
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:1495:            Self::WorkflowLoaded { .. } => "WorkflowLoaded",
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:1496:            Self::WorkflowStopped { .. } => "WorkflowStopped",
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:1497:            Self::WorkflowStartFailed { .. } => "WorkflowStartFailed",
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:1498:            Self::WorkflowExecuted { .. } => "WorkflowExecuted",
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:1499:            Self::WorkflowsChanged { .. } => "WorkflowsChanged",
<OPENHUMAN_ROOT>/README.md:76:- **[Workflows](https://tinyhumans.gitbook.io/openhuman/features/workflows)**: the agent proposes the automation; you review it on a canvas and save. Durable, trigger-driven, approval-gated ...
<OPENHUMAN_ROOT>/README.md:125:## Workflows you can see
<OPENHUMAN_ROOT>/README.md:127:Heavily inspired by n8n and Zapier, [workflows](https://tinyhumans.gitbook.io/openhuman/features/workflows) bring the same visual, trigger-driven automation to your agent, except the agent ...
<OPENHUMAN_ROOT>/README.md:130: <img src="./gitbooks/.gitbook/assets/workflows.png" alt="OpenHuman workflow canvas">
<OPENHUMAN_ROOT>/README.md:133:> The agent proposes the workflow; you review it on a canvas and save it.
<OPENHUMAN_ROOT>/README.md:135:Saved workflows are durable and trigger-driven. They fire on schedules, webhooks, or channel events, survive restarts, and gate side effects behind approvals.
<OPENHUMAN_ROOT>/README.md:139:High-level comparison (products evolve, so verify against each vendor). OpenHuman is built to **minimize vendor sprawl**, keep **workflow knowledge on-device**, and give the agent a **persi...
<OPENHUMAN_ROOT>/README.md:150:| **Workflows**          | 🚫 None           | ⚠️ Scripts        | ⚠️ Scripts        | 🚀 Visual, durable, agent-proposed, approval-gated                                        ...
<OPENHUMAN_ROOT>/README.md:161:New contributor? Start with [`CONTRIBUTING.md`](./CONTRIBUTING.md) for the fork/PR workflow and local validation commands, or use the copy-paste AI-agent prompt in [`CONTRIBUTING-BEGINNERS....
<OPENHUMAN_ROOT>/e2e/docker-compose.yml:4:# This mirrors the `e2e-linux` job in `.github/workflows/e2e.yml` so any
<OPENHUMAN_ROOT>/plan.md:20:   most of `scripts/__tests__/`, and the Pester Windows-install test are invoked by no workflow.
<OPENHUMAN_ROOT>/plan.md:139:### P1 — core workflows
<OPENHUMAN_ROOT>/plan.md:197:**Holes found (verified by exhaustive grep of package.json + workflows):**
<OPENHUMAN_ROOT>/plan.md:240:- Legacy `e2e.yml` / `e2e-playwright.yml` / `test.yml` are now `workflow_dispatch`-only — the
<OPENHUMAN_ROOT>/plan.md:243:### Findings from §5 that are STILL open (re-verified by grep on the new workflows)
<OPENHUMAN_ROOT>/plan.md:247:| Orphaned harness self-tests (`scripts/mock-api/socket.{auth,transport}.test.mjs`, most of `scripts/__tests__/`, Pester `test:install-ps1`) | **Still orphaned.** No workflow references them ...
<OPENHUMAN_ROOT>/plan.md:251:| `check-domain-e2e-coverage.mjs` | Still not wired into any workflow (`check-coverage-matrix.mjs` runs in `pr-quality.yml`). |
<OPENHUMAN_ROOT>/plan.md:360:  ≥1 package.json script or workflow.

```

Output excerpt:

```text
[search: 500 match(es) across 101 file(s) · top 5 per file · full set via retrieve footer]
<OPENHUMAN_ROOT>/src/main.rs:139:            // suppression lives at the `install_workflow_from_url_with_home`
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:460:    /// non-trigger node settles, so the Workflows UI can show a run advancing
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:477:    WorkflowLoaded { skill_id: String, runtime: String },
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:481:    WorkflowStartFailed { skill_id: String, error: String },
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:1337:            | Self::WorkflowStartFailed { .. }
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:1497:            Self::WorkflowStartFailed { .. } => "WorkflowStartFailed",
[+12 more match(es) in <OPENHUMAN_ROOT>/src/core/event_bus/events.rs]
<OPENHUMAN_ROOT>/README.md:76:- **[Workflows](https://tinyhumans.gitbook.io/openhuman/features/workflows)**: the agent proposes the automation; you review it on a canvas and save. Durable, trigger-driven, approval-gated ...
<OPENHUMAN_ROOT>/README.md:125:## Workflows you can see
<OPENHUMAN_ROOT>/README.md:127:Heavily inspired by n8n and Zapier, [workflows](https://tinyhumans.gitbook.io/openhuman/features/workflows) bring the same visual, trigger-driven automation to your agent, except the agent ...
<OPENHUMAN_ROOT>/README.md:130: <img src="./gitbooks/.gitbook/assets/workflows.png" alt="OpenHuman workflow canvas">
<OPENHUMAN_ROOT>/README.md:133:> The agent proposes the workflow; you review it on a canvas and save it.
[+4 more match(es) in <OPENHUMAN_ROOT>/README.md]
<OPENHUMAN_ROOT>/e2e/docker-compose.yml:4:# This mirrors the `e2e-linux` job in `.github/workflows/e2e.yml` so any
<OPENHUMAN_ROOT>/plan.md:20:   most of `scripts/__tests__/`, and the Pester Windows-install test are invoked by no workflow.
<OPENHUMAN_ROOT>/plan.md:139:### P1 — core workflows
<OPENHUMAN_ROOT>/plan.md:197:**Holes found (verified by exhaustive grep of package.json + workflows):**
<OPENHUMAN_ROOT>/plan.md:240:- Legacy `e2e.yml` / `e2e-playwright.yml` / `test.yml` are now `workflow_dispatch`-only — the
<OPENHUMAN_ROOT>/plan.md:243:### Findings from §5 that are STILL open (re-verified by grep on the new workflows)
[+4 more match(es) in <OPENHUMAN_ROOT>/plan.md]
<OPENHUMAN_ROOT>/src/core/socketio.rs:1053:                // truth and the Workflows UI keeps a 2s poller as fallback, so
<OPENHUMAN_ROOT>/e2e/run-local.sh:5:# Mirrors `.github/workflows/e2e.yml` `e2e-linux` step-for-step inside
<OPENHUMAN_ROOT>/src/core/event_bus/events_tests.rs:213:        // Workflow
<OPENHUMAN_ROOT>/src/core/event_bus/events_tests.rs:215:            DomainEvent::WorkflowLoaded {
<OPENHUMAN_ROOT>/src/core/event_bus/events_tests.rs:219:            "workflow",
<OPENHUMAN_ROOT>/src/core/event_bus/events_tests.rs:222:            DomainEvent::WorkflowStopped {
<OPENHUMAN_ROOT>/src/core/event_bus/events_tests.rs:228:            DomainEvent::WorkflowStartFailed {
[+8 more match(es) in <OPENHUMAN_ROOT>/src/core/event_bus/events_tests.rs]
<OPENHUMAN_ROOT>/src/core/jsonrpc.rs:1858:                // Prune legacy bundled skills (dev-workflow / github-issue-crusher
<OPENHUMAN_ROOT>/src/core/jsonrpc.rs:1862:                crate::openhuman::workflows::registry::prune_legacy_default_workflows(
<OPENHUMAN_ROOT>/src/core/jsonrpc.rs:2263:        // workflows still dispatch when no realtime channel is configured or
<OPENHUMAN_ROOT>/src/core/jsonrpc.rs:2584:    // --- Triggered-workflow subscriber ---
<OPENHUMAN_ROOT>/src/core/jsonrpc.rs:2587:    // `OPENHUMAN_DISABLE_CHANNEL_LISTENERS=1`). Without this, any workflow
[+1 more match(es) in <OPENHUMAN_ROOT>/src/core/jsonrpc.rs]
<OPENHUMAN_ROOT>/e2e/docker-local-bootstrap.sh:8:# CI workflow runs (`.github/workflows/e2e.yml` → `e2e-linux`), but only

```

### `03-rg-retrieve`

- [Full input](cases/03-rg-retrieve/input.rg)
- [Full output](cases/03-rg-retrieve/output.rg)

Input excerpt:

```text
<OPENHUMAN_ROOT>/src/core/all.rs:296:    // TokenJuice content-router debug controllers (detect / compress / cache_stats / retrieve)
<OPENHUMAN_ROOT>/src/core/all.rs:708:/// Retrieves the schema for a specific RPC method.
<OPENHUMAN_ROOT>/vendor/tinycortex/src/lib.rs:6://! score and embed them, build summary trees, and retrieve explainable context.
<OPENHUMAN_ROOT>/scripts/tools-generator/discover-tools.js:80:      description: "Retrieve message history from a Telegram chat",
<OPENHUMAN_ROOT>/scripts/tools-generator/discover-tools.js:90:            description: "Number of messages to retrieve (max 100)",
<OPENHUMAN_ROOT>/tests/agent_retrieval_e2e.rs:150:/// (synthesised into a `delegate_retrieve_memory` tool), so the orchestrator
<OPENHUMAN_ROOT>/tests/agent_retrieval_e2e.rs:214:// ── Cross-chat retrieval: chat A seeds facts; retrieve from chat B ──────────
<OPENHUMAN_ROOT>/tests/agent_retrieval_e2e.rs:220:/// This is the core of "agent retrieves relevant context from other chats"
<OPENHUMAN_ROOT>/tests/agent_retrieval_e2e.rs:328:/// chunk so the orchestrator can cite the exact provenance of retrieved facts.
<OPENHUMAN_ROOT>/gitbooks/guides/privacy-sensitive-data.md:48:**Why local memory *is* the privacy design.** Most assistants trade privacy for context, because more context means more of your raw data uploaded. OpenHuman ...
<OPENHUMAN_ROOT>/gitbooks/guides/privacy-sensitive-data.md:82:* [ ] Are you comfortable that model turns send *retrieved snippets*, not your whole memory?
<OPENHUMAN_ROOT>/gitbooks/guides/privacy-sensitive-data.md:88:| "OpenHuman uploads my whole memory to answer." | It sends only what it retrieves for that specific turn. |
<OPENHUMAN_ROOT>/tests/memory_fast_retrieve_e2e.rs:1://! E2E tests for the deterministic E2GraphRAG retriever (`fast_retrieve`).
<OPENHUMAN_ROOT>/tests/memory_fast_retrieve_e2e.rs:17://!   cargo test --test memory_fast_retrieve_e2e
<OPENHUMAN_ROOT>/tests/memory_fast_retrieve_e2e.rs:19://!   bash scripts/test-rust-with-mock.sh --test memory_fast_retrieve_e2e
<OPENHUMAN_ROOT>/tests/memory_fast_retrieve_e2e.rs:27:use openhuman_core::openhuman::memory_tree::retrieval::{fast_retrieve, FastRetrieveOptions};
<OPENHUMAN_ROOT>/tests/memory_fast_retrieve_e2e.rs:77:    let resp = fast_retrieve(
<OPENHUMAN_ROOT>/tests/memory_fast_retrieve_e2e.rs:80:        FastRetrieveOptions::default(),
<OPENHUMAN_ROOT>/tests/memory_fast_retrieve_e2e.rs:83:    .expect("fast_retrieve should succeed");
<OPENHUMAN_ROOT>/tests/memory_fast_retrieve_e2e.rs:112:    let resp = fast_retrieve(
<OPENHUMAN_ROOT>/tests/memory_fast_retrieve_e2e.rs:115:        FastRetrieveOptions::default(),
<OPENHUMAN_ROOT>/tests/memory_fast_retrieve_e2e.rs:126:    let resp = fast_retrieve(&cfg, "anything at all", FastRetrieveOptions::default())
<OPENHUMAN_ROOT>/docs/TEST-COVERAGE-MATRIX.md:342:| 8.3.7 | Long-Source Exact Leaf Retrieval     | RU    | `src/openhuman/memory/tree/retrieval/benchmarks.rs::bench_long_source_retrieves_exact_leaf`     | 🟡     | Embe...
<OPENHUMAN_ROOT>/docs/tinyagents-port-plan.md:166:1. Delete transitional shims (`ToolAdapter` test-only wrapper, `subagent_graph.rs` no-op skeleton once the graph path is the real one, `retrieve_tool_output` vs tokenjuic...
<OPENHUMAN_ROOT>/docs/tinyagents-port-plan.md:212:- `retriever.rs:14-27` — crate `Retriever`/`InMemoryVectorStore` built but unused on the live path (dead-until-swap; out of scope here, note for the memory migration).
<OPENHUMAN_ROOT>/gitbooks/features/privacy-and-security.md:63:Because canonicalization, chunking, scoring and summary trees all run **inside your local Rust core**, your raw source data never leaves your machine. The onl...
<OPENHUMAN_ROOT>/gitbooks/features/token-compression.md:42:6. Append marker      ⟦tj:<hash>⟧ footer so the agent can retrieve the full original
<OPENHUMAN_ROOT>/gitbooks/features/token-compression.md:55:5. **CCR offload.** For **lossy** compressions where the original is large enough (`ccr_min_tokens`, default ~500 tokens), the full original is stowed in the **C...
<OPENHUMAN_ROOT>/gitbooks/features/token-compression.md:93:Lossy compression would normally mean throwing data away. TokenJuice instead **offloads** the full original into the **Compress-Cache-Retrieve (CCR)** store and ...
<OPENHUMAN_ROOT>/gitbooks/features/token-compression.md:97:* **The marker:** compacted output ends with a footer like `[compacted tool output — PARTIAL view; full original available via tokenjuice_retrieve with token "...
<OPENHUMAN_ROOT>/gitbooks/features/token-compression.md:98:* **Retrieval tool:** the agent calls the read-only **`tokenjuice_retrieve`** tool with that token (optionally a byte/line `range`) to pull back the full origina...
<OPENHUMAN_ROOT>/gitbooks/features/token-compression.md:134:* **RPC** (`openhuman.tokenjuice_*`): `detect`, `compress` (dry-run the pipeline), `settings_get` / `settings_update` (live partial patch), `cache_stats`, `retr...
<OPENHUMAN_ROOT>/gitbooks/features/token-compression.md:135:* **Agent tool:** `tokenjuice_retrieve` (read-only) recovers offloaded originals.
<OPENHUMAN_ROOT>/tests/json_rpc_e2e.rs:14385:    // --- retrieve them in a batch ---
<OPENHUMAN_ROOT>/tests/memory_sync_round23_raw_coverage_e2e.rs:183:        "SLACK_RETRIEVE_DETAILED_USER_INFORMATION" => {
<OPENHUMAN_ROOT>/tests/memory_sync_round23_raw_coverage_e2e.rs:310:            "SLACK_RETRIEVE_DETAILED_USER_INFORMATION",

```

Output excerpt:

```text
[search: 500 match(es) across 178 file(s) · top 5 per file · full set via retrieve footer]
<OPENHUMAN_ROOT>/src/core/all.rs:296:    // TokenJuice content-router debug controllers (detect / compress / cache_stats / retrieve)
<OPENHUMAN_ROOT>/src/core/all.rs:708:/// Retrieves the schema for a specific RPC method.
<OPENHUMAN_ROOT>/vendor/tinycortex/src/lib.rs:6://! score and embed them, build summary trees, and retrieve explainable context.
<OPENHUMAN_ROOT>/scripts/tools-generator/discover-tools.js:80:      description: "Retrieve message history from a Telegram chat",
<OPENHUMAN_ROOT>/scripts/tools-generator/discover-tools.js:90:            description: "Number of messages to retrieve (max 100)",
<OPENHUMAN_ROOT>/tests/agent_retrieval_e2e.rs:150:/// (synthesised into a `delegate_retrieve_memory` tool), so the orchestrator
<OPENHUMAN_ROOT>/tests/agent_retrieval_e2e.rs:214:// ── Cross-chat retrieval: chat A seeds facts; retrieve from chat B ──────────
<OPENHUMAN_ROOT>/tests/agent_retrieval_e2e.rs:220:/// This is the core of "agent retrieves relevant context from other chats"
<OPENHUMAN_ROOT>/tests/agent_retrieval_e2e.rs:328:/// chunk so the orchestrator can cite the exact provenance of retrieved facts.
<OPENHUMAN_ROOT>/gitbooks/guides/privacy-sensitive-data.md:48:**Why local memory *is* the privacy design.** Most assistants trade privacy for context, because more context means more of your raw data uploaded. OpenHuman ...
<OPENHUMAN_ROOT>/gitbooks/guides/privacy-sensitive-data.md:82:* [ ] Are you comfortable that model turns send *retrieved snippets*, not your whole memory?
<OPENHUMAN_ROOT>/gitbooks/guides/privacy-sensitive-data.md:88:| "OpenHuman uploads my whole memory to answer." | It sends only what it retrieves for that specific turn. |
<OPENHUMAN_ROOT>/tests/memory_fast_retrieve_e2e.rs:1://! E2E tests for the deterministic E2GraphRAG retriever (`fast_retrieve`).
<OPENHUMAN_ROOT>/tests/memory_fast_retrieve_e2e.rs:17://!   cargo test --test memory_fast_retrieve_e2e
<OPENHUMAN_ROOT>/tests/memory_fast_retrieve_e2e.rs:19://!   bash scripts/test-rust-with-mock.sh --test memory_fast_retrieve_e2e
<OPENHUMAN_ROOT>/tests/memory_fast_retrieve_e2e.rs:27:use openhuman_core::openhuman::memory_tree::retrieval::{fast_retrieve, FastRetrieveOptions};
<OPENHUMAN_ROOT>/tests/memory_fast_retrieve_e2e.rs:77:    let resp = fast_retrieve(
[+5 more match(es) in <OPENHUMAN_ROOT>/tests/memory_fast_retrieve_e2e.rs]
<OPENHUMAN_ROOT>/docs/TEST-COVERAGE-MATRIX.md:342:| 8.3.7 | Long-Source Exact Leaf Retrieval     | RU    | `src/openhuman/memory/tree/retrieval/benchmarks.rs::bench_long_source_retrieves_exact_leaf`     | 🟡     | Embe...
<OPENHUMAN_ROOT>/docs/tinyagents-port-plan.md:166:1. Delete transitional shims (`ToolAdapter` test-only wrapper, `subagent_graph.rs` no-op skeleton once the graph path is the real one, `retrieve_tool_output` vs tokenjuic...
<OPENHUMAN_ROOT>/docs/tinyagents-port-plan.md:212:- `retriever.rs:14-27` — crate `Retriever`/`InMemoryVectorStore` built but unused on the live path (dead-until-swap; out of scope here, note for the memory migration).
<OPENHUMAN_ROOT>/gitbooks/features/privacy-and-security.md:63:Because canonicalization, chunking, scoring and summary trees all run **inside your local Rust core**, your raw source data never leaves your machine. The onl...
<OPENHUMAN_ROOT>/gitbooks/features/token-compression.md:55:5. **CCR offload.** For **lossy** compressions where the original is large enough (`ccr_min_tokens`, default ~500 tokens), the full original is stowed in the **C...
<OPENHUMAN_ROOT>/gitbooks/features/token-compression.md:93:Lossy compression would normally mean throwing data away. TokenJuice instead **offloads** the full original into the **Compress-Cache-Retrieve (CCR)** store and ...
<OPENHUMAN_ROOT>/gitbooks/features/token-compression.md:97:* **The marker:** compacted output ends with a footer like `[compacted tool output — PARTIAL view; full original available via tokenjuice_retrieve with token "...
<OPENHUMAN_ROOT>/gitbooks/features/token-compression.md:98:* **Retrieval tool:** the agent calls the read-only **`tokenjuice_retrieve`** tool with that token (optionally a byte/line `range`) to pull back the full origina...
<OPENHUMAN_ROOT>/gitbooks/features/token-compression.md:135:* **Agent tool:** `tokenjuice_retrieve` (read-only) recovers offloaded originals.
[+2 more match(es) in <OPENHUMAN_ROOT>/gitbooks/features/token-compression.md]
<OPENHUMAN_ROOT>/tests/json_rpc_e2e.rs:14385:    // --- retrieve them in a batch ---
<OPENHUMAN_ROOT>/tests/memory_sync_round23_raw_coverage_e2e.rs:183:        "SLACK_RETRIEVE_DETAILED_USER_INFORMATION" => {
<OPENHUMAN_ROOT>/tests/memory_sync_round23_raw_coverage_e2e.rs:310:            "SLACK_RETRIEVE_DETAILED_USER_INFORMATION",
<OPENHUMAN_ROOT>/src/openhuman/channels/controllers/ops/discord.rs:154:/// Retrieve the stored Discord bot token from credentials.
<OPENHUMAN_ROOT>/gitbooks/features/obsidian-wiki/retrieval.md:75:`walk` and `smart_walk` both route through `fast_retrieve` (`src/openhuman/memory_tree/retrieval/fast.rs`), an **E2GraphRAG-style** algorithm that replaces...
<OPENHUMAN_ROOT>/gitbooks/features/obsidian-wiki/retrieval.md:83:Tunables (`FastRetrieveOptions`): `limit` (`k`, default 10, cap 100), `max_hops` (`h`, default 2, cap 4), and an optional `time_window_days` look-back on t...
<OPENHUMAN_ROOT>/tests/memory_sync_slack_bus_raw_coverage_e2e.rs:137:        "SLACK_RETRIEVE_DETAILED_USER_INFORMATION" => execute_envelope(json!({

```

