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
) -> String { … 12 line(s) … ⟦tj:8ab7446b94ad83a883f5fc2eefb3700c⟧ }

/// Resolve the **chat/inference base URL** (used for inference routing only,
/// not for backend domain calls).
///
/// Prefer [`effective_backend_api_url`] for anything other than chat completions.
/// The two functions are intentionally separate — see the module-level doc.
pub fn effective_api_url(api_url: &Option<String>) -> String { … 8 line(s) … ⟦tj:3259d1b2cdb50e768517784dc15191f5⟧ }

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
pub fn effective_backend_api_url(api_url: &Option<String>) -> String { … 80 line(s) … ⟦tj:4e2dee5b380bdbeb9573b828cd17d56f⟧ }

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
pub fn looks_like_local_ai_endpoint(url: &str) -> bool { … 37 line(s) … ⟦tj:d28d6861d2e0f41ef5f84e44acf5c4fa⟧ }

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
pub fn looks_like_inference_provider_endpoint(url: &str) -> bool { … 38 line(s) … ⟦tj:1e28db1d697278585b21c20784e68f76⟧ }

/// Returns `true` when the URL's host is one of the known OpenHuman backends.
///
/// Used in [`effective_backend_api_url`] to short-circuit the local-AI check:
/// a user who set `api_url` to `https://api.tinyhumans.ai/openai/v1/chat/completions`
/// must still reach the real backend (not fall back to the default chain).
fn looks_like_openhuman_backend_endpoint(url: &str) -> bool { … 44 line(s) … ⟦tj:c8ad4fa73b0dd25f99294628364ab990⟧ }

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
pub(crate) fn normalize_backend_api_base_url(url: &str) -> String { … 24 line(s) … ⟦tj:c73bd9b828f964158ae68cd320349ba9⟧ }

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
pub fn api_url(base: &str, path: &str) -> String { … 15 line(s) … ⟦tj:d8b4209960f3ed1ab187959097f38565⟧ }

/// Last-resort URL join used when `url::Url::parse` rejects the base.
///
/// Guarantees a slash between `base` and `path` regardless of whether either
/// carries one, but does not otherwise validate the resulting string.
#[inline]
fn fallback_concat(base: &str, path: &str) -> String { … 8 line(s) … ⟦tj:fcd2f2f8c5b5600ec60826dd8207ff7f⟧ }

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
pub fn api_base_from_env() -> Option<String> { … 23 line(s) … ⟦tj:6a4c30ef3fd17e4c610ffcc7e6d0e134⟧ }

/// Resolve the app environment string (e.g. `"staging"`, `"production"`).
///
/// Resolution order mirrors [`api_base_from_env`]: runtime vars first, then
/// compile-time bakes, each key checked independently.
pub fn app_env_from_env() -> Option<String> { … 19 line(s) … ⟦tj:14092531d4677ee06232bb7390ec4b63⟧ }

/// Return `true` when `app_env` equals `"staging"` (case-insensitive).
pub fn is_staging_app_env(app_env: Option<&str>) -> bool {
    matches!(app_env.map(str::trim), Some(env) if env.eq_ignore_ascii_case("staging"))
}

/// Map an app environment string to its canonical API base URL constant.
pub fn default_api_base_url_for_env(app_env: Option<&str>) -> &'static str { … 7 line(s) … ⟦tj:cf20f6dda2061475dc21f25efef19123⟧ }

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
fn compile_time_app_env_values() -> [Option<&'static str>; 2] { … 6 line(s) … ⟦tj:bf1948239dd7134333026a11982ee5a8⟧ }

#[cfg(test)]
fn compile_time_app_env_values() -> [Option<&'static str>; 2] {
    [None, None]
}

// ─── Logging helpers ─────────────────────────────────────────────────────────

/// Redact username and password from a URL before writing it to a log.
///
/// Falls back to a scheme-prefixed parse for bare-host strings like
/// `localhost:1234` so those are still sanitised rather than returned verbatim.
pub(crate) fn redact_url_for_log(raw: &str) -> String { … 19 line(s) … ⟦tj:da0945bada4c33f3beac7a76a23e61f4⟧ }

/// Emit a single `warn!` log the **first time** the backend URL falls back
/// from a user-set local-AI endpoint. Uses `std::sync::Once` to suppress
/// subsequent emissions so the log is not spammed on every backend request.
fn warn_backend_url_fallback_once(local_url: &str) { … 12 line(s) … ⟦tj:902b7e14d8bb52c946af5094f9647eaf⟧ }

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
fn host_is_local(parsed: &url::Url) -> bool { … 13 line(s) … ⟦tj:83f7203ebedd486f433a649491eb2a35⟧ }

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

    fn env_lock() -> MutexGuard<'static, ()> { … 6 line(s) … ⟦tj:8ba64defed95449aceb77db7b39e0b54⟧ }

    /// RAII guard that captures the current values of the four backend env
    /// vars, removes them, and restores them on drop — even if the test panics.
    struct EnvSnapshot {
        vars: [(&'static str, Option<String>); 4],
    }

    impl EnvSnapshot {
        fn clear_backend_env() -> Self { … 13 line(s) … ⟦tj:d60db4e3d74826f4bdf83a8b86323230⟧ }
    }

    impl Drop for EnvSnapshot {
        fn drop(&mut self) { … 8 line(s) … ⟦tj:c003e9e9f4c370fa13bc0edeaa9336a9⟧ }
    }

    /// The URL that should be used as the backend base when no config override
    /// is present and the runtime env has been cleared for the test.
    fn fallback_backend_base_for_current_build() -> String { … 5 line(s) … ⟦tj:8f8c6f58d25c2b314b70f1c30fdfecf2⟧ }

    // ── api_url ───────────────────────────────────────────────────────────────

    #[test]
    fn api_url_empty_path_returns_normalized_base() { … 14 line(s) … ⟦tj:0f23dbe97485b8b6244f5d6d18b49a08⟧ }

    #[test]
    fn api_url_absolute_path_replaces_base_path() { … 11 line(s) … ⟦tj:106f3aa04e100454a379662410315ff4⟧ }

    #[test]
    fn api_url_clean_base_joins_cleanly() { … 17 line(s) … ⟦tj:398287afb71bb2821477492ec9b964fb⟧ }

    #[test]
    fn api_url_preserves_query_string_on_path() { … 9 line(s) … ⟦tj:824b61635e14ed03510636b69f7fdab9⟧ }

    #[test]
    fn api_url_unparseable_base_falls_back_to_concat() { … 4 line(s) … ⟦tj:3d3802c01ce41466a96f63744b5c0a13⟧ }

    #[test]
    fn api_url_with_lm_studio_base_joins_correctly() { … 9 line(s) … ⟦tj:47c41e45786a1336e79f19cffe91eeb5⟧ }

    #[test]
    fn api_url_multiple_trailing_slashes_on_base_are_stripped() { … 6 line(s) … ⟦tj:cb188473bde7636229f94f12dac5a755⟧ }

    #[test]
    fn api_url_relative_path_without_leading_slash_does_not_panic() { … 6 line(s) … ⟦tj:ba3efeb46565fa9cc8a00f4efffc8cc3⟧ }

    // ── normalize_api_base_url ────────────────────────────────────────────────

    #[test]
    fn normalize_strips_trailing_slashes_and_whitespace() { … 18 line(s) … ⟦tj:019a2b4696ca51cef26ddd99e9b3ff48⟧ }

    #[test]
    fn normalize_preserves_mid_path() { … 6 line(s) … ⟦tj:0906dd5a20471091d559454db25b9ab4⟧ }

    #[test]
    fn normalize_empty_string_returns_empty() {
        assert_eq!(normalize_api_base_url(""), "");
    }

    // ── normalize_backend_api_base_url ────────────────────────────────────────

    #[test]
    fn normalize_backend_strips_inference_path() { … 6 line(s) … ⟦tj:eaee81934ba27d392f9ec441495324b8⟧ }

    #[test]
    fn normalize_backend_handles_schemeless_input() { … 7 line(s) … ⟦tj:196d99b12519de97d257fbab25454f8f⟧ }

    #[test]
    fn normalize_backend_passes_through_clean_root() { … 6 line(s) … ⟦tj:6f379ca54c0dcf08c03e77621254dbfb⟧ }

    #[test]
    fn normalize_backend_empty_string_is_idempotent() {
        assert_eq!(normalize_backend_api_base_url(""), "");
    }

    // ── app / api env resolution ──────────────────────────────────────────────

    #[test]
    fn staging_env_resolves_to_staging_url() { … 7 line(s) … ⟦tj:ab160e68f0614ff942ee2eae4a48c194⟧ }

    #[test]
    fn non_staging_env_resolves_to_production_url() { … 8 line(s) … ⟦tj:2e12d141b9fdc98566970a08990e97f8⟧ }

    #[test]
    fn app_env_from_env_reads_runtime_var() { … 11 line(s) … ⟦tj:d889044ffbf3bbd4f3377c72107ee9ef⟧ }

    #[test]
    fn app_env_empty_primary_falls_through_to_secondary() { … 17 line(s) … ⟦tj:09886ccc8e10eb16050a054e9954b854⟧ }

    #[test]
    fn api_base_from_env_reads_runtime_var() { … 11 line(s) … ⟦tj:8f0d68f604087329726f6354e6694a16⟧ }

    #[test]
    fn api_base_empty_primary_falls_through_to_secondary() { … 17 line(s) … ⟦tj:e11b5f577644f91fa0e99f73f2107f05⟧ }

    // ── looks_like_local_ai_endpoint ─────────────────────────────────────────

    #[test]
    fn local_ai_matches_loopback_hosts() { … 9 line(s) … ⟦tj:7a9a6c97d4bbc35653ee5201a02c8ff6⟧ }

    #[test]
    fn local_ai_matches_chat_completions_path_on_any_host() { … 8 line(s) … ⟦tj:b0892d2c11284a1d605c4aa7a68ebe71⟧ }

    #[test]
    fn local_ai_rejects_bare_loopback_with_random_port() { … 6 line(s) … ⟦tj:9306ad9e9314dc1c56331daf4010482b⟧ }

    #[test]
    fn local_ai_matches_private_lan_hosts() { … 7 line(s) … ⟦tj:ed10a2b1e593b9088516afb8d10d212b⟧ }

    #[test]
    fn local_ai_rejects_real_backends() { … 11 line(s) … ⟦tj:a243ab1f89f89e2bbd711f3bcc943d49⟧ }

    #[test]
    fn local_ai_rejects_substring_path_false_positives() { … 12 line(s) … ⟦tj:a1077f8a8e4b9df6c5303d46d5189d13⟧ }

    #[test]
    fn local_ai_handles_garbage_input() { … 6 line(s) … ⟦tj:cc4f251731dd27bf2b7841a22834b913⟧ }

    #[test]
    fn local_ai_matches_lm_studio_default_port() { … 7 line(s) … ⟦tj:c9ebd8bb8363e2ef9a52b6acdb335d13⟧ }

    #[test]
    fn local_ai_matches_v1_subpath_on_loopback() { … 8 line(s) … ⟦tj:1a6cfb7ac28602b68f374f8254234086⟧ }

    // ── openhuman_backend detection ───────────────────────────────────────────

    #[test]
    fn openhuman_backend_detection_accepts_hosted_api_paths() { … 14 line(s) … ⟦tj:742f432d9884d4b393f6572d01cc214f⟧ }

    // ── effective_backend_api_url ─────────────────────────────────────────────

    #[test]
    fn backend_url_handles_llm_endpoint_overrides() { … 27 line(s) … ⟦tj:34d841fbf9165d4572e8c8666a4218a3⟧ }

    #[test]
    fn backend_url_falls_back_for_local_ai_override() { … 10 line(s) … ⟦tj:82a77ff97ef26c7bce6cf2ec029a7305⟧ }

    #[test]
    fn backend_url_falls_back_for_cloud_inference_base() { … 39 line(s) … ⟦tj:5175eb6d8be8c0e3d6ea645b04db9e02⟧ }

    #[test]
    fn backend_url_falls_back_to_env_when_override_is_local_ai() { … 12 line(s) … ⟦tj:47e9d663a0368440b917ff2e348186ac⟧ }

    #[test]
    fn backend_url_keeps_real_backend_override() { … 6 line(s) … ⟦tj:34debc34d6b7feb8026da9dc9ad84a5c⟧ }

    #[test]
    fn backend_url_without_override_matches_effective_api_url() { … 5 line(s) … ⟦tj:e68b71ae9962153114b66954ea6c57bd⟧ }

    // ── GH #4153: remote managed inference providers parked in `api_url` ──────

    #[test]
    fn inference_provider_matches_known_remote_hosts() { … 19 line(s) … ⟦tj:6aa056fcffc47e182aef0674e23a8b31⟧ }

    #[test]
    fn inference_provider_matches_bare_v1_base_on_unknown_host() { … 9 line(s) … ⟦tj:63ccbe02517e9a53e6eb43ad9b6811c9⟧ }

    #[test]
    fn inference_provider_excludes_openhuman_backend_and_plain_hosts() { … 17 line(s) … ⟦tj:10d2252a832a07f242d003d2af373b04⟧ }

    #[test]
    fn backend_url_falls_back_for_remote_inference_provider_override() { … 17 line(s) … ⟦tj:c709b02fb0aa46f225fb961036d35b6b⟧ }

    #[test]
    fn backend_url_falls_back_to_env_for_remote_inference_provider() { … 10 line(s) … ⟦tj:da6a97f4189c00219497f4cb1a68d483⟧ }

    #[test]
    fn backend_url_strips_inference_path_from_env() { … 14 line(s) … ⟦tj:95126b97148db58396af8cbfc6f2379e⟧ }
}
[collapsed bodies are individually retrievable: call tokenjuice_retrieve with the token inside a placeholder to expand just that body]

[compacted tool output — this is a PARTIAL view; the full original (53860 bytes) is available by calling tokenjuice_retrieve with token "628b8d203fa7110274d8705e54311625" (marker ⟦tj:628b8d203fa7110274d8705e54311625⟧)]