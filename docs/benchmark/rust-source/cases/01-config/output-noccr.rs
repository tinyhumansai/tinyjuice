//! # API URL resolution & classification
//!
//! This module is the **single source of truth** for every URL the app uses to
//! reach either:
//!
//! * the **hosted backend** (auth, billing, integrations, voice, sockets, …), or
//! * the **LLM inference endpoint** (OpenAI-compatible chat completions).
//!
//! ## Why two separate URL families?
//!
//! Users can point `config.api_url` at a local model runner (Ollama, vLLM,
//! LM Studio). Those servers only speak `/v1/chat/completions` and 404 on
//! every other path. Naïvely reusing a single base URL for both families
//! caused every `/auth/*`, `/agent-integrations/*`, and `/voice/*` request to
//! 404 against the local runner — see Sentry cluster `OPENHUMAN-TAURI-51/-80/-7Z`.
//!
//! The fix is the [`effective_backend_api_url`] / [`effective_inference_url`]
//! split:
//!
//! ```text
//!                    config.api_url
//!                         │
//!              ┌──────────┴──────────┐
//!              │ looks_like_local_ai │
//!              └──────────┬──────────┘
//!                yes      │      no
//!         ┌───────────────┼────────────────────┐
//!         ▼               ▼                    ▼
//!  env / default   backend calls OK    inference calls OK
//!  (backend only)
//! ```
//!
//! ## Resolution order (both families)
//!
//! 1. Non-empty `config.api_url` / `config.inference_url` (user override).
//! 2. `BACKEND_URL` / `VITE_BACKEND_URL` runtime env (each checked
//!    independently so an empty primary does not shadow a valid secondary).
//! 3. Same keys baked in at compile time via `option_env!` (makes a
//!    distributed binary resolve to the correct environment without a shell).
//! 4. Environment-aware default: `staging` env → [`DEFAULT_STAGING_API_BASE_URL`],
//!    otherwise [`DEFAULT_API_BASE_URL`].

// ─── Public constants ────────────────────────────────────────────────────────

/// Production hosted-API root. Used as the final fallback for non-staging
/// builds when no override is configured.
pub const DEFAULT_API_BASE_URL: &str = "https://api.tinyhumans.ai";

/// Staging hosted-API root. Activated when `OPENHUMAN_APP_ENV=staging` (or
/// the Vite equivalent) is set at runtime or baked in at compile time.
pub const DEFAULT_STAGING_API_BASE_URL: &str = "https://staging-api.tinyhumans.ai";

/// Runtime env key used by the Tauri/core side to select the app environment.
pub const APP_ENV_VAR: &str = "OPENHUMAN_APP_ENV";

/// Runtime env key exposed to the Vite frontend bundle. Mirrors `APP_ENV_VAR`
/// so both the core sidecar and the renderer agree on the environment without
/// a separate IPC round-trip.
pub const VITE_APP_ENV_VAR: &str = "VITE_OPENHUMAN_APP_ENV";

/// The path the hosted backend appends to its root to expose the
/// OpenAI-compatible inference proxy. Joined onto [`effective_api_url`] when
/// the user has not configured a dedicated `inference_url`.
///
/// Having this as a named constant (rather than a string literal scattered
/// across call-sites) means a backend path rename shows up as a single diff.
pub const OPENHUMAN_INFERENCE_PATH: &str = "/openai/v1/chat/completions";

// ─── Known local-AI ports ────────────────────────────────────────────────────

/// Well-known ports used by local model runners.
///
/// Used by [`looks_like_local_ai_endpoint`] as a secondary signal when the
/// URL's host is loopback / private but the path alone is not conclusive
/// (e.g. `http://localhost:11434` — no path, but clearly Ollama).
///
/// | Port  | Runner        |
/// |-------|---------------|
/// | 11434 | Ollama        |
/// | 8000  | vLLM          |
/// | 8080  | common alt    |
/// | 1234  | LM Studio     |
/// | 8888  | Jupyter proxy |
const LOCAL_AI_PORTS: &[u16] = &[11434, 8000, 8080, 1234, 8888];

// ─── Effective URL resolvers ─────────────────────────────────────────────────

/// Resolve the URL for **LLM inference calls** (chat completions only).
///
/// # Resolution order
///
/// 1. `inference_url_override` — user explicitly pointed inference at a
///    custom OpenAI-compatible endpoint (e.g. `https://api.openai.com/v1/chat/completions`
///    or a local Ollama). Used as-is; no path stripping.
/// 2. [`effective_api_url`]`(api_url_override)` + [`OPENHUMAN_INFERENCE_PATH`] —
///    inference proxied through the hosted backend.
///
/// # Why the split matters
///
/// Without a dedicated `inference_url`, every inference call flows through the
/// hosted backend's OpenAI-compat proxy. When the user *does* set
/// `inference_url`, backend calls still go to [`effective_backend_api_url`] —
/// so `/auth/*`, `/voice/*`, and `/agent-integrations/*` never accidentally
/// hit `api.openai.com` or a local runner.
pub fn effective_inference_url(
    api_url_override: &Option<String>,
    inference_url_override: &Option<String>,
) -> String { … 12 line(s) … }

/// Resolve the **chat/inference base URL** (used for inference routing only,
/// not for backend domain calls).
///
/// Prefer [`effective_backend_api_url`] for anything other than chat completions.
/// The two functions are intentionally separate — see the module-level doc.
pub fn effective_api_url(api_url: &Option<String>) -> String {
    if let Some(u) = non_empty_str(api_url) {
        return normalize_api_base_url(u);
    }

    api_base_from_env()
        .unwrap_or_else(|| default_api_base_url_for_env(app_env_from_env().as_deref()).to_string())
}

/// Resolve the API base URL for **all hosted-backend calls**:
/// auth, billing, team, referral, webhooks, credentials, channels,
/// voice, sockets, app-state, integrations, core/jsonrpc, …
///
/// # Key difference from [`effective_api_url`]
///
/// The user override is **skipped** when it [`looks_like_local_ai_endpoint`]
/// **and** does not [`looks_like_openhuman_backend_endpoint`]. In that case
/// the function falls through to the env / default chain so backend requests
/// still reach the hosted API.
///
/// A one-shot `warn!` is emitted the first time the fallback fires so the
/// diagnostic is visible in sidecar logs without spamming on every request.
///
/// # Sentry context
///
/// `OPENHUMAN-TAURI-51 / -80 / -7Z` — Ollama users saw every integration
/// request 404 because `config.api_url` (set to the Ollama endpoint) was also
/// used as the integrations base.
pub fn effective_backend_api_url(api_url: &Option<String>) -> String {
    if let Some(u) = non_empty_str(api_url) {
        let is_local_ai = looks_like_local_ai_endpoint(u);
    { … 75 line(s) … }
        .unwrap_or_else(|| default_api_base_url_for_env(app_env_from_env().as_deref()).to_string())
}

// ─── URL classification ──────────────────────────────────────────────────────

/// Returns `true` when the URL appears to be a local / self-hosted model
/// runner rather than the hosted OpenHuman backend.
///
/// The heuristic is **intentionally tight** to avoid misclassifying:
/// * ad-hoc mock backends used in integration tests
///   (`http://127.0.0.1:<ephemeral-port>` with no path), and
/// * real custom backends that happen to include `/v1` as an API-version prefix.
///
/// # Classification logic
///
/// ```text
/// ┌─ path ends with /v1/chat/completions  ─────────────────────────────► TRUE
/// │  or /v1/completions (any host)
/// │
/// └─ host is loopback / private IP / localhost
///    AND (port ∈ LOCAL_AI_PORTS  OR  path starts with /v1/)  ──────────► TRUE
///
/// everything else  ────────────────────────────────────────────────────► FALSE
/// ```
///
/// Both path checks use `ends_with` (not `contains`) so a real backend whose
/// path merely *embeds* the segment (e.g. `/audit/v1/chat/completions-logs`)
/// is not misclassified.
///
/// A bare `/v1` path (e.g. `https://api.openai.com/v1`) intentionally does
/// NOT match — it is a legitimate API-version suffix used by many real
/// backends, and over-matching here would silently reroute paying users.
pub fn looks_like_local_ai_endpoint(url: &str) -> bool {
    let trimmed = url.trim();
    if trimmed.is_empty() {
    { … 32 line(s) … }
    port_is_llm || path_is_llm
}

/// Well-known managed inference-provider registrable domains. A `config.api_url`
/// pointed at one of these (or a subdomain) is a BYO chat/inference base — never
/// an OpenHuman control-plane backend — so backend calls must NOT route there.
///
/// Suffix-matched so `api.<provider>` / `<region>.<provider>` also classify.
/// Kept tight to genuinely managed inference hosts; an unknown custom backend
/// is still honored unless it carries the OpenAI-compatible `/v1` base shape
/// below. `tinyhumans.ai` is deliberately ABSENT — our own hosted backend is
/// recognised by [`looks_like_openhuman_backend_endpoint`] and must route.
const INFERENCE_PROVIDER_DOMAINS: &[&str] = &[
    "openrouter.ai",
    "openmodel.ai",
    "openai.com",
    "anthropic.com",
    "groq.com",
    "mistral.ai",
    "deepseek.com",
    "together.ai",
    "together.xyz",
    "perplexity.ai",
    "fireworks.ai",
    "deepinfra.com",
    "anyscale.com",
    "novita.ai",
    "hyperbolic.xyz",
    "x.ai",
    "googleapis.com",
    "cohere.ai",
    "cohere.com",
];

/// Returns `true` when the URL looks like a **remote managed inference
/// provider** base rather than the hosted OpenHuman backend.
///
/// Complements [`looks_like_local_ai_endpoint`] (which only catches *local*
/// model runners): together they let [`effective_backend_api_url`] fall back to
/// the canonical backend whenever `config.api_url` has been set to an inference
/// base instead of a control-plane base. See the misroute note in
/// `effective_backend_api_url` (GH #4153 / TAURI-RUST-BSF·8C·HDS·HW1·JJ5).
///
/// Two signals (either is sufficient):
/// 1. **Known provider host** — host equals or is a subdomain of a domain in
///    [`INFERENCE_PROVIDER_DOMAINS`].
/// 2. **OpenAI-compatible base path** — the path is exactly `/v1` or `/api/v1`
///    (trailing slash ignored). This is the canonical OpenAI-style base and is
///    never an OpenHuman control-plane base. A bare `/v1/chat/completions` is
///    already covered by [`looks_like_local_ai_endpoint`]'s path signal.
///
/// Our own hosted backend short-circuits to `false` so a user who set
/// `api_url` to `https://api.tinyhumans.ai/...` still reaches the backend.
pub fn looks_like_inference_provider_endpoint(url: &str) -> bool {
    let trimmed = url.trim();
    if trimmed.is_empty() {
    { … 33 line(s) … }
    false
}

/// Returns `true` when the URL's host is one of the known OpenHuman backends.
///
/// Used in [`effective_backend_api_url`] to short-circuit the local-AI check:
/// a user who set `api_url` to `https://api.tinyhumans.ai/openai/v1/chat/completions`
/// must still reach the real backend (not fall back to the default chain).
fn looks_like_openhuman_backend_endpoint(url: &str) -> bool {
    let trimmed = url.trim();
    let redacted = redact_url_for_log(trimmed);
    { … 39 line(s) … }
    is_openhuman
}

// ─── URL normalization helpers ───────────────────────────────────────────────

/// Trim whitespace and strip trailing slashes so all base URLs are in
/// canonical form before being joined with a path.
///
/// This is deliberately a cheap string operation (no URL parsing) so it can
/// be called on potentially-invalid strings without panicking.
pub fn normalize_api_base_url(url: &str) -> String {
    url.trim().trim_end_matches('/').to_string()
}

/// Like [`normalize_api_base_url`] but also **strips any inference-style path**
/// (e.g. `/openai/v1/chat/completions`) so the result is always a bare host
/// root suitable as a backend base.
///
/// # Why this exists
///
/// Users (and CI configs) sometimes set `BACKEND_URL` or `config.api_url` to
/// the full inference endpoint. Backend callers append domain-specific paths
/// (`/auth/me`, `/agent-integrations/…`) which then land on
/// `.../openai/v1/chat/completions/auth/me` — an obvious 404.
///
/// # Scheme-less fallback
///
/// `option_env!`-baked values occasionally omit the scheme
/// (e.g. `api.tinyhumans.ai/openai/v1/chat/completions`). We retry with an
/// `https://` prefix so the path can still be stripped before the value is
/// used as a base. Without this, a scheme-less inference path survived into
/// every backend call — Sentry `OPENHUMAN-TAURI-H6 / -HN`, issue #2075.
pub(crate) fn normalize_backend_api_base_url(url: &str) -> String {
    let normalized = normalize_api_base_url(url);
    if normalized.is_empty() {
    { … 19 line(s) … }
    parsed.to_string().trim_end_matches('/').to_string()
}

/// Safely join an API base URL with an absolute path.
///
/// # Behaviour
///
/// | `base`                                    | `path`                    | result                                                                 |
/// |-------------------------------------------|---------------------------|------------------------------------------------------------------------|
/// | `https://api.tinyhumans.ai`               | `/auth/me`                | `https://api.tinyhumans.ai/auth/me`                                   |
/// | `https://api.tinyhumans.ai/openai/v1/…`   | `/agent-integrations/foo` | `https://api.tinyhumans.ai/agent-integrations/foo`  ← path replaced   |
/// | `https://api.tinyhumans.ai`               | `""`                      | `https://api.tinyhumans.ai`                                           |
/// | `not a url`                               | `/x`                      | `not a url/x`  ← safe fallback concat                                 |
///
/// Paths **must start with `/`**. Relative paths (no leading slash) are
/// resolved per RFC 3986 — the base's last segment is dropped — which is
/// almost never what an API client wants.
pub fn api_url(base: &str, path: &str) -> String {
    let base = base.trim();

    { … 10 line(s) … }
    }
}

/// Last-resort URL join used when `url::Url::parse` rejects the base.
///
/// Guarantees a slash between `base` and `path` regardless of whether either
/// carries one, but does not otherwise validate the resulting string.
#[inline]
fn fallback_concat(base: &str, path: &str) -> String {
    let base = base.trim_end_matches('/');
    if path.starts_with('/') {
        format!("{base}{path}")
    } else {
        format!("{base}/{path}")
    }
}

// ─── Environment resolution ───────────────────────────────────────────────────

/// Resolve the hosted API base URL from the environment.
///
/// Checks `BACKEND_URL` then `VITE_BACKEND_URL` independently (runtime first,
/// then compile-time bakes). An empty string for the primary key does **not**
/// shadow a valid secondary key — this matters when a `.env` file sets
/// `BACKEND_URL=""` to disable the override while keeping `VITE_BACKEND_URL`
/// active for the renderer.
///
/// Returns `None` when neither key is set or both are empty.
pub fn api_base_from_env() -> Option<String> {
    // 1. Runtime — each key checked independently.
    for key in ["BACKEND_URL", "VITE_BACKEND_URL"] {
    { … 18 line(s) … }
    None
}

/// Resolve the app environment string (e.g. `"staging"`, `"production"`).
///
/// Resolution order mirrors [`api_base_from_env`]: runtime vars first, then
/// compile-time bakes, each key checked independently.
pub fn app_env_from_env() -> Option<String> {
    for key in [APP_ENV_VAR, VITE_APP_ENV_VAR] {
        if let Ok(v) = std::env::var(key) {
    { … 14 line(s) … }
    None
}

/// Return `true` when `app_env` equals `"staging"` (case-insensitive).
pub fn is_staging_app_env(app_env: Option<&str>) -> bool {
    matches!(app_env.map(str::trim), Some(env) if env.eq_ignore_ascii_case("staging"))
}

/// Map an app environment string to its canonical API base URL constant.
pub fn default_api_base_url_for_env(app_env: Option<&str>) -> &'static str {
    if is_staging_app_env(app_env) {
        DEFAULT_STAGING_API_BASE_URL
    } else {
        DEFAULT_API_BASE_URL
    }
}

// ─── Compile-time env accessors ───────────────────────────────────────────────

/// Values baked in by the build pipeline.
///
/// Stubbed to `[None, None]` in tests so that clearing runtime env vars
/// produces fully deterministic results regardless of what the CI baked in.
#[cfg(not(test))]
fn compile_time_api_base_env_values() -> [Option<&'static str>; 2] {
    [option_env!("BACKEND_URL"), option_env!("VITE_BACKEND_URL")]
}

#[cfg(test)]
fn compile_time_api_base_env_values() -> [Option<&'static str>; 2] {
    [None, None]
}

#[cfg(not(test))]
fn compile_time_app_env_values() -> [Option<&'static str>; 2] {
    [
        option_env!("OPENHUMAN_APP_ENV"),
        option_env!("VITE_OPENHUMAN_APP_ENV"),
    ]
}

#[cfg(test)]
fn compile_time_app_env_values() -> [Option<&'static str>; 2] {
    [None, None]
}

// ─── Logging helpers ─────────────────────────────────────────────────────────

/// Redact username and password from a URL before writing it to a log.
///
/// Falls back to a scheme-prefixed parse for bare-host strings like
/// `localhost:1234` so those are still sanitised rather than returned verbatim.
pub(crate) fn redact_url_for_log(raw: &str) -> String {
    let trimmed = raw.trim();

    { … 14 line(s) … }
    parsed.to_string().trim_end_matches('/').to_string()
}

/// Emit a single `warn!` log the **first time** the backend URL falls back
/// from a user-set local-AI endpoint. Uses `std::sync::Once` to suppress
/// subsequent emissions so the log is not spammed on every backend request.
fn warn_backend_url_fallback_once(local_url: &str) { … 12 line(s) … }

// ─── Private utilities ───────────────────────────────────────────────────────

/// Extract a trimmed, non-empty string reference from an `Option<String>`.
///
/// Centralises the `as_deref().map(str::trim).filter(|s| !s.is_empty())`
/// pattern that was repeated throughout the original code.
#[inline]
fn non_empty_str(s: &Option<String>) -> Option<&str> {
    s.as_deref().map(str::trim).filter(|s| !s.is_empty())
}

/// Returns `true` when the parsed URL's host is loopback, unspecified
/// (`0.0.0.0` / `[::]`), a private RFC 1918 IPv4 range, or `localhost`.
///
/// Using typed-host matching (via `url::Host` variants) rather than
/// `host_str()` string comparison ensures that IPv4-mapped IPv6 addresses
/// (`::ffff:127.0.0.1`), the bare IPv6 loopback (`::1`), and all three
/// IPv4 loopback forms classify correctly.
#[inline]
fn host_is_local(parsed: &url::Url) -> bool { … 13 line(s) … }

// ─── Tests ───────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use std::sync::{Mutex, MutexGuard, OnceLock};

    use super::*;

    // ── Test infrastructure ───────────────────────────────────────────────────

    /// Global mutex that serialises all env-mutating tests.
    /// `std::env` is process-global; without serialisation, parallel test
    /// threads race on `set_var` / `remove_var` and produce flaky failures.
    static ENV_LOCK: OnceLock<Mutex<()>> = OnceLock::new();

    fn env_lock() -> MutexGuard<'static, ()> {
        match ENV_LOCK.get_or_init(Mutex::default).lock() {
            Ok(g) => g,
            Err(p) => p.into_inner(), // recover from a poisoned lock
        }
    }

    /// RAII guard that captures the current values of the four backend env
    /// vars, removes them, and restores them on drop — even if the test panics.
    struct EnvSnapshot {
        vars: [(&'static str, Option<String>); 4],
    }

    impl EnvSnapshot {
        fn clear_backend_env() -> Self { … 13 line(s) … }
    }

    impl Drop for EnvSnapshot {
        fn drop(&mut self) {
            for (key, value) in &self.vars {
                match value {
                    Some(v) => std::env::set_var(key, v),
                    None => std::env::remove_var(key),
                }
            }
        }
    }

    /// The URL that should be used as the backend base when no config override
    /// is present and the runtime env has been cleared for the test.
    fn fallback_backend_base_for_current_build() -> String {
        api_base_from_env().unwrap_or_else(|| {
            default_api_base_url_for_env(app_env_from_env().as_deref()).to_string()
        })
    }

    // ── api_url ───────────────────────────────────────────────────────────────

    #[test]
    fn api_url_empty_path_returns_normalized_base() {
        assert_eq!(
            api_url("https://api.tinyhumans.ai", ""),
        { … 9 line(s) … }
        );
}

    #[test]
    fn api_url_absolute_path_replaces_base_path() { … 11 line(s) … }

    #[test]
    fn api_url_clean_base_joins_cleanly() {
        let expected = "https://api.tinyhumans.ai/agent-integrations/composio/toolkits";
        assert_eq!(
        { … 12 line(s) … }
        );
}

    #[test]
    fn api_url_preserves_query_string_on_path() {
        assert_eq!(
            api_url(
                "https://api.tinyhumans.ai",
                "/agent-integrations/composio/tools?toolkits=gmail"
            ),
            "https://api.tinyhumans.ai/agent-integrations/composio/tools?toolkits=gmail"
        );
    }

    #[test]
    fn api_url_unparseable_base_falls_back_to_concat() {
        assert_eq!(api_url("not a url", "/x"), "not a url/x");
        assert_eq!(api_url("not a url/", "/x"), "not a url/x");
    }

    #[test]
    fn api_url_with_lm_studio_base_joins_correctly() {
        // LM Studio URL must not reach effective_backend_api_url in practice
        // (it redirects), but api_url itself must not panic and the result
        // must use the correct host root.
        assert_eq!(
            api_url("http://localhost:1234/v1", "/agent-integrations/foo"),
            "http://localhost:1234/agent-integrations/foo"
        );
    }

    #[test]
    fn api_url_multiple_trailing_slashes_on_base_are_stripped() {
        assert_eq!(
            api_url("https://api.tinyhumans.ai///", "/v1/foo"),
            "https://api.tinyhumans.ai/v1/foo"
        );
    }

    #[test]
    fn api_url_relative_path_without_leading_slash_does_not_panic() {
        // Documented edge-case: relative paths are resolved RFC 3986-style
        // (last base segment dropped). The exact result depends on base
        // structure; we just pin the no-panic contract.
        assert!(!api_url("https://api.tinyhumans.ai", "relative").is_empty());
    }

    // ── normalize_api_base_url ────────────────────────────────────────────────

    #[test]
    fn normalize_strips_trailing_slashes_and_whitespace() {
        assert_eq!(
            normalize_api_base_url("https://api.tinyhumans.ai/"),
        { … 13 line(s) … }
        );
}

    #[test]
    fn normalize_preserves_mid_path() {
        assert_eq!(
            normalize_api_base_url("https://api.tinyhumans.ai/v2"),
            "https://api.tinyhumans.ai/v2"
        );
    }

    #[test]
    fn normalize_empty_string_returns_empty() {
        assert_eq!(normalize_api_base_url(""), "");
    }

    // ── normalize_backend_api_base_url ────────────────────────────────────────

    #[test]
    fn normalize_backend_strips_inference_path() {
        assert_eq!(
            normalize_backend_api_base_url("https://api.tinyhumans.ai/openai/v1/chat/completions"),
            "https://api.tinyhumans.ai"
        );
    }

    #[test]
    fn normalize_backend_handles_schemeless_input() {
        // Sentry OPENHUMAN-TAURI-H6 / issue #2075.
        assert_eq!(
            normalize_backend_api_base_url("api.tinyhumans.ai/openai/v1/chat/completions"),
            "https://api.tinyhumans.ai"
        );
    }

    #[test]
    fn normalize_backend_passes_through_clean_root() {
        assert_eq!(
            normalize_backend_api_base_url("https://api.tinyhumans.ai/"),
            "https://api.tinyhumans.ai"
        );
    }

    #[test]
    fn normalize_backend_empty_string_is_idempotent() {
        assert_eq!(normalize_backend_api_base_url(""), "");
    }

    // ── app / api env resolution ──────────────────────────────────────────────

    #[test]
    fn staging_env_resolves_to_staging_url() {
        assert_eq!(
            default_api_base_url_for_env(Some("staging")),
            DEFAULT_STAGING_API_BASE_URL
        );
        assert!(is_staging_app_env(Some("STAGING")));
    }

    #[test]
    fn non_staging_env_resolves_to_production_url() {
        assert_eq!(
            default_api_base_url_for_env(Some("production")),
            DEFAULT_API_BASE_URL
        );
        assert_eq!(default_api_base_url_for_env(None), DEFAULT_API_BASE_URL);
        assert!(!is_staging_app_env(Some("development")));
    }

    #[test]
    fn app_env_from_env_reads_runtime_var() { … 11 line(s) … }

    #[test]
    fn app_env_empty_primary_falls_through_to_secondary() {
        let _guard = env_lock();
        let prev_p = std::env::var(APP_ENV_VAR).ok();
        { … 12 line(s) … }
        assert_eq!(result.as_deref(), Some("staging"));
}

    #[test]
    fn api_base_from_env_reads_runtime_var() { … 11 line(s) … }

    #[test]
    fn api_base_empty_primary_falls_through_to_secondary() {
        let _guard = env_lock();
        let prev_p = std::env::var("BACKEND_URL").ok();
        { … 12 line(s) … }
        assert_eq!(result.as_deref(), Some("https://staging-api.tinyhumans.ai"));
}

    // ── looks_like_local_ai_endpoint ─────────────────────────────────────────

    #[test]
    fn local_ai_matches_loopback_hosts() {
        assert!(looks_like_local_ai_endpoint("http://127.0.0.1:11434/v1"));
        assert!(looks_like_local_ai_endpoint(
            "http://127.0.0.1:8080/v1/chat/completions"
        ));
        assert!(looks_like_local_ai_endpoint("http://localhost:11434/v1"));
        assert!(looks_like_local_ai_endpoint("http://[::1]:11434"));
        assert!(looks_like_local_ai_endpoint("http://0.0.0.0:11434/v1"));
    }

    #[test]
    fn local_ai_matches_chat_completions_path_on_any_host() {
        assert!(looks_like_local_ai_endpoint(
            "http://203.0.113.5:8080/v1/chat/completions"
        ));
        assert!(looks_like_local_ai_endpoint(
            "https://my-ollama.example/v1/completions"
        ));
    }

    #[test]
    fn local_ai_rejects_bare_loopback_with_random_port() {
        assert!(!looks_like_local_ai_endpoint("http://127.0.0.1:54321"));
        assert!(!looks_like_local_ai_endpoint("http://127.0.0.1:42000/"));
        assert!(!looks_like_local_ai_endpoint("http://localhost:33333"));
        assert!(!looks_like_local_ai_endpoint("http://[::1]:51234"));
    }

    #[test]
    fn local_ai_matches_private_lan_hosts() {
        assert!(looks_like_local_ai_endpoint(
            "http://192.168.1.100:11434/v1"
        ));
        assert!(looks_like_local_ai_endpoint("http://10.0.0.5:8080/v1"));
        assert!(looks_like_local_ai_endpoint("http://172.16.0.42:8000"));
    }

    #[test]
    fn local_ai_rejects_real_backends() { … 11 line(s) … }

    #[test]
    fn local_ai_rejects_substring_path_false_positives() { … 12 line(s) … }

    #[test]
    fn local_ai_handles_garbage_input() {
        assert!(!looks_like_local_ai_endpoint(""));
        assert!(!looks_like_local_ai_endpoint("   "));
        assert!(!looks_like_local_ai_endpoint("not a url"));
        assert!(!looks_like_local_ai_endpoint("/v1/chat/completions")); // relative — must not panic
    }

    #[test]
    fn local_ai_matches_lm_studio_default_port() {
        assert!(looks_like_local_ai_endpoint("http://localhost:1234"));
        assert!(looks_like_local_ai_endpoint("http://127.0.0.1:1234"));
        assert!(looks_like_local_ai_endpoint(
            "http://127.0.0.1:1234/v1/chat/completions"
        ));
    }

    #[test]
    fn local_ai_matches_v1_subpath_on_loopback() {
        assert!(looks_like_local_ai_endpoint(
            "http://localhost:11434/v1/models"
        ));
        assert!(looks_like_local_ai_endpoint(
            "http://127.0.0.1:8080/v1/embeddings"
        ));
    }

    // ── openhuman_backend detection ───────────────────────────────────────────

    #[test]
    fn openhuman_backend_detection_accepts_hosted_api_paths() {
        assert!(looks_like_openhuman_backend_endpoint(
            "https://api.tinyhumans.ai/openai/v1/chat/completions"
        { … 9 line(s) … }
        ));
}

    // ── effective_backend_api_url ─────────────────────────────────────────────

    #[test]
    fn backend_url_handles_llm_endpoint_overrides() {
        let _guard = env_lock();
        let _env = EnvSnapshot::clear_backend_env();
        { … 22 line(s) … }
        }
}

    #[test]
    fn backend_url_falls_back_for_local_ai_override() { … 10 line(s) … }

    #[test]
    fn backend_url_falls_back_for_cloud_inference_base() {
        // Regression: TAURI-RUST-HW1. A BYO user whose `api_url` is a public
        // cloud-inference provider's *canonical base* (no `/chat/completions`
        { … 34 line(s) … }
        );
}

    #[test]
    fn backend_url_falls_back_to_env_when_override_is_local_ai() { … 12 line(s) … }

    #[test]
    fn backend_url_keeps_real_backend_override() {
        assert_eq!(
            effective_backend_api_url(&Some("https://staging-api.tinyhumans.ai/".to_string())),
            "https://staging-api.tinyhumans.ai"
        );
    }

    #[test]
    fn backend_url_without_override_matches_effective_api_url() {
        let _guard = env_lock();
        let _env = EnvSnapshot::clear_backend_env();
        assert_eq!(effective_backend_api_url(&None), effective_api_url(&None));
    }

    // ── GH #4153: remote managed inference providers parked in `api_url` ──────

    #[test]
    fn inference_provider_matches_known_remote_hosts() {
        // The hosts the #4058 `host` tag pinned the `/teams/me/usage` flood to.
        assert!(looks_like_inference_provider_endpoint(
        { … 14 line(s) … }
        ));
}

    #[test]
    fn inference_provider_matches_bare_v1_base_on_unknown_host() {
        // An unknown OpenAI-compatible provider, recognised by its `/v1` base.
        assert!(looks_like_inference_provider_endpoint(
            "https://llm.unknown-provider.example/v1"
        ));
        assert!(looks_like_inference_provider_endpoint(
            "https://gw.example.test/api/v1/"
        ));
    }

    #[test]
    fn inference_provider_excludes_openhuman_backend_and_plain_hosts() {
        // Our own hosted backend is a backend, even though it serves inference.
        assert!(!looks_like_inference_provider_endpoint(
        { … 12 line(s) … }
        assert!(!looks_like_inference_provider_endpoint("not a url"));
}

    #[test]
    fn backend_url_falls_back_for_remote_inference_provider_override() {
        // The core of #4153: `config.api_url` set to a managed inference
        // provider must NOT be used as the control-plane base; backend calls
        { … 12 line(s) … }
        );
}

    #[test]
    fn backend_url_falls_back_to_env_for_remote_inference_provider() { … 10 line(s) … }

    #[test]
    fn backend_url_strips_inference_path_from_env() {
        // Regression: OPENHUMAN-TAURI-H6 / -HN, issue #2075.
        let _guard = env_lock();
        { … 9 line(s) … }
        );
}
}