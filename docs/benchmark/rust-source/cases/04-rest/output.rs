//! HTTP client for TinyHumans / AlphaHuman API routes (`/auth/...`, etc.).

use anyhow::{Context, Result};
use base64::Engine;
use reqwest::header::{HeaderMap, HeaderName, HeaderValue, AUTHORIZATION};
use reqwest::{Client, Method, Url};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::time::Duration;

use super::jwt::bearer_authorization_value;

/// Typed errors surfaced by `authed_json` for expected backend states that
/// callers should recover from in-flow rather than funnel into Sentry.
#[derive(Debug, thiserror::Error)]
pub enum BackendApiError {
    /// Edit / delete of a channel message returned 404. Happens when the
    /// user deletes the message on the provider side (Telegram, Discord,
    /// Slack, …) but our local `StreamingState` still has the id, or when
    /// the backend GC'd the relay row before we got around to editing it.
    /// Callers should clear stale state and skip the retry. Targets
    /// `OPENHUMAN-TAURI-2Y` (~454 events on `/channels/telegram/messages/<id>`).
    #[error("message not found on {provider}: {message_id}")]
    MessageNotFound {
        /// Channel provider segment (e.g. `"telegram"`, `"discord"`).
        provider: String,
        /// Provider-specific message id from the URL.
        message_id: String,
    },
    /// Backend rejected the bearer JWT with `401 Unauthorized`. This is an
    /// expected user-session state (token expired, revoked, rotated
    /// server-side) — not a code bug. Callers can route to a re-sign-in
    /// flow; the auth domain owns recovery. Targets `OPENHUMAN-TAURI-4K8`
    /// (12 events on `/openai/v1/audio/speech` mascot TTS, but the same
    /// shape fires on every authed endpoint once the session lapses).
    #[error("backend rejected session token on {method} {path}")]
    Unauthorized {
        /// HTTP method as a static string (`"GET"`, `"POST"`, …).
        method: String,
        /// Request path the 401 came back from (no query string).
        path: String,
    },
}

/// Flatten an `authed_json` error onto the JSON-RPC `String` channel.
///
/// `BackendApiError::Unauthorized` is an expected backend session-lapse 401
/// (token expired / revoked / rotated server-side), not a code bug — see the
/// variant docs above. Callers used to flatten it with `format!("{e:#}")` /
/// `e.to_string()`, producing `"backend rejected session token on {method}
/// {path}"`, which matches none of the JSON-RPC session-expiry classifiers
/// (`is_session_expired_error`, `is_session_expired_message`, the `before_send`
/// net), so every lapsed-session 401 leaked to Sentry — TAURI-RUST-8WY
/// (`/teams/me/usage`), TAURI-RUST-8WZ (`/payments/stripe/currentPlan`), and the
/// rest of the authed-endpoint family (#3297).
///
/// Mapping `Unauthorized` onto the existing `SESSION_EXPIRED` sentinel makes the
/// dispatcher (`core/jsonrpc.rs`) classify it as session expiry: it skips the
/// Sentry report AND publishes `DomainEvent::SessionExpired` so the auth domain
/// drives re-sign-in. This keys off the typed downcast — not the Display
/// wording — so it stays correct if the `#[error(...)]` text changes, consistent
/// with #2959's removal of brittle string-based suppression. Every other error
/// (including `MessageNotFound`) keeps its full `{e:#}` chain so genuine
/// failures still reach Sentry.
pub fn flatten_authed_error(err: anyhow::Error) -> String {
    match err.downcast_ref::<BackendApiError>() {
        Some(BackendApiError::Unauthorized { method, path }) => {
            format!("SESSION_EXPIRED: backend rejected session token on {method} {path}")
        }
        _ => format!("{err:#}"),
    }
}

/// Extract `(provider, message_id)` from a backend channel path of the
/// shape `…/channels/<provider>/messages/<id>`. Returns `None` for paths
/// that do not contain this four-segment subsequence.
///
/// Handles both the canonical four-segment form and paths with an arbitrary
/// base-path prefix (e.g. `/api/v1/channels/telegram/messages/1103`) via a
/// sliding window so that `BACKEND_URL` variants with path prefixes do not
/// silently fall through to `report_error` (OPENHUMAN-TAURI-R7).
fn parse_message_path(path: &str) -> Option<(&str, &str)> {
    let segments: Vec<&str> = path.split('/').filter(|s| !s.is_empty()).collect();
    // Fast path: exact four-segment canonical form /channels/<p>/messages/<id>
    { … 9 line(s) … ⟦tj:e1ba36b9d80e4eea767369549a6be4c2⟧ }
    None
}

const CLIENT_VERSION_HEADER_MAX_LEN: usize = 64;

/// Max bytes of the `body_shape` key-name list echoed into the `authed_json`
/// report. Bounded so a body with pathologically many keys can't bloat the
/// event; truncation is UTF-8-safe.
const BACKEND_API_BODY_SHAPE_MAX_BYTES: usize = 120;

/// PII-safe classification of a non-2xx response body for telemetry.
///
/// `report_error`'s message is written to the core/Tauri daily logs BEFORE any
/// Sentry `before_send` scrubbing, and that scrubber only catches a few
/// secret-shaped patterns — so the raw body must never be echoed (a non-2xx body
/// can carry emails / profile JSON / OAuth errors / nonstandard token fields).
/// We emit only the SHAPE: for a JSON object, the count of top-level keys plus
/// the sorted subset that look like schema field names; otherwise a coarse
/// label. Even key NAMES are response-controlled (a foreign backend could return
/// `{"jo@example.com": 1}`), so only keys matching a conservative ASCII-identifier
/// shape are echoed — everything else is counted as `redacted` and never logged.
/// The surviving names are enough to identify which backend/gateway produced a
/// response — the `TAURI-RUST-8C` case (a 91-byte body matching no route this
/// backend emits), where our canonical envelope is `{success,error,errorCode}`
/// and a foreign gateway/proxy is not.
fn backend_api_body_shape(body: &str) -> String {
    let trimmed = body.trim();
    if trimmed.is_empty() {
    { … 24 line(s) … ⟦tj:25e1fe0e8456169a00a40a7af4ed22e2⟧ }
    }
}

/// A JSON key safe to echo into telemetry: a short ASCII identifier (the shape
/// of a schema field name). Anything else — non-ASCII, punctuation like `@`,
/// whitespace, or overlong — is treated as response-controlled data and excluded
/// so `body_shape` can never leak an email/UUID/free-text used as a key.
fn is_schema_like_key(key: &str) -> bool {
    const MAX_KEY_LEN: usize = 40;
    !key.is_empty()
        && key.len() <= MAX_KEY_LEN
        && key
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || matches!(b, b'_' | b'-' | b'.'))
}

fn sanitize_client_version(raw: &str) -> Option<String> {
    let sanitized: String = raw
        .trim()
    { … 9 line(s) … ⟦tj:575295daac4c9b3d4b889be7758377cc⟧ }
    }
}

fn build_backend_reqwest_client() -> Result<Client> {
    let mut default_headers = HeaderMap::new();
    if let Some(version) = sanitize_client_version(env!("CARGO_PKG_VERSION")) {
    { … 26 line(s) … ⟦tj:ebad9d86d21fbd345b90abc2577bc5bf⟧ }
        .map_err(|e| anyhow::anyhow!("failed to build HTTP client: {e}"))
}

fn parse_api_response_json(text: &str) -> Result<Value> {
    let v: Value = serde_json::from_str(text).with_context(|| format!("parse API JSON: {text}"))?;
    let Some(obj) = v.as_object() else {
    { … 25 line(s) … ⟦tj:252ec75d708c945e3bfa5471d1ae893a⟧ }
    Ok(v)
}

fn user_id_from_object(obj: &serde_json::Map<String, Value>) -> Option<String> { … 11 line(s) … ⟦tj:dd49a8b5a840154743d78cacdd62a874⟧ }

/// Best-effort extraction of a user ID from an authenticated profile payload.
///
/// This function handles various envelope formats, including raw user objects
/// or those nested under `data` or `user` keys.
pub fn user_id_from_profile_payload(payload: &Value) -> Option<String> {
    let obj = payload.as_object()?;
    if let Some(data) = obj.get("data").and_then(|v| v.as_object()) {
    { … 11 line(s) … ⟦tj:62d5f9c928cfe963ee02d655490c88f0⟧ }
    })
}

/// Alias for [`user_id_from_profile_payload`] for semantic clarity in auth flows.
pub fn user_id_from_auth_me_payload(payload: &Value) -> Option<String> {
    user_id_from_profile_payload(payload)
}

/// JSON body returned by the backend when an OAuth connection process is initiated.
#[derive(Debug, Clone, Deserialize)]
pub struct ConnectResponse {
    /// The URL to redirect the user to for OAuth authorization.
    pub oauth_url: String,
    /// The state parameter used to prevent CSRF and correlate the callback.
    pub state: String,
}

#[derive(Debug, Clone, Deserialize)]
struct ConnectEnvelope {
    success: bool,
    #[serde(default, alias = "oauthUrl")]
    oauth_url: Option<String>,
    #[serde(default)]
    state: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
struct IntegrationsEnvelope {
    success: bool,
    data: IntegrationsData,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
struct IntegrationsData {
    integrations: Vec<IntegrationSummary>,
}

/// A summary of an active integration, as returned by the backend.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IntegrationSummary {
    /// Unique identifier for the integration.
    pub id: String,
    /// The name of the integration provider (e.g., "google", "slack").
    pub provider: String,
    /// RFC3339 timestamp of when the integration was created.
    pub created_at: String,
}

#[derive(Debug, Clone, Deserialize)]
struct TokensEnvelope {
    success: bool,
    data: TokensData,
}

#[derive(Debug, Clone, Deserialize)]
struct TokensData {
    encrypted: String,
}

#[derive(Debug, Clone, Deserialize)]
struct LoginTokenConsumeEnvelope {
    success: bool,
    data: LoginTokenConsumeData,
}

#[derive(Debug, Clone, Deserialize)]
struct LoginTokenConsumeData {
    jwt: String,
}

/// Decrypted OAuth token payload for handing off tokens to a local service or skill.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IntegrationTokensHandoff {
    /// The OAuth access token.
    pub access_token: String,
    /// The optional OAuth refresh token.
    #[serde(default)]
    pub refresh_token: Option<String>,
    /// RFC3339 timestamp of when the access token expires.
    pub expires_at: String,
}

/// A client for interacting with the TinyHumans / AlphaHuman backend API.
#[derive(Clone)]
pub struct BackendOAuthClient {
    client: Client,
    base: Url,
}

impl BackendOAuthClient {
    /// Creates a new `BackendOAuthClient` with the given API base URL.
    ///
    /// Any path, query, or fragment in `api_base` is stripped so that
    /// `Url::join` always resolves root-relative REST paths correctly.
    /// This guards against callers who pass a full LLM completions URL
    /// (e.g. `https://host/v1/chat/completions`) instead of just the origin:
    /// without stripping, `join("teams/me/usage")` would produce the wrong
    /// path `/v1/chat/teams/me/usage` via RFC 3986 relative resolution.
    pub fn new(api_base: &str) -> Result<Self> {
        let mut base = Url::parse(api_base.trim()).context("Invalid API base URL")?;
        anyhow::ensure!(
            matches!(base.scheme(), "http" | "https") && base.host_str().is_some(),
            "API base URL must be an absolute http(s) URL with host"
        );
        base.set_path("");
        base.set_query(None);
        base.set_fragment(None);
        let client = build_backend_reqwest_client()?;
        Ok(Self { client, base })
    }

    /// Borrow the underlying `reqwest::Client` for callers that need to
    /// drive a non-JSON request shape (e.g. `multipart/form-data` uploads
    /// for cloud STT) without re-implementing TLS/proxy plumbing.
    pub fn raw_client(&self) -> &Client {
        &self.client
    }

    /// Resolve a backend-relative path against the configured base URL.
    /// Mirrors what `authed_json` does internally so callers using
    /// `raw_client()` don't have to assemble URLs by hand.
    pub fn url_for(&self, path: &str) -> Result<Url> {
        self.base
            .join(path.trim_start_matches('/'))
            .with_context(|| format!("build URL for {path}"))
    }

    /// Returns the URL for initiating a login flow for a specific provider.
    pub fn login_url(&self, provider: &str) -> Result<Url> {
        let p = provider.trim().trim_matches('/');
        anyhow::ensure!(!p.is_empty(), "provider is required");
        self.base
            .join(&format!("auth/{p}/login"))
            .context("build login URL")
    }

    /// Initiates an OAuth connection flow for the current user and a specific provider.
    pub async fn connect(
        &self,
        provider: &str,
        bearer_jwt: &str,
        skill_id: Option<&str>,
        response_type: Option<&str>,
        encryption_mode: Option<&str>,
    ) -> Result<ConnectResponse> {
        let p = provider.trim().trim_matches('/');
        anyhow::ensure!(!p.is_empty(), "provider is required");
        { … 41 line(s) … ⟦tj:9dc9ef7bb15dc308504a37c3c3080dcc⟧ }
        Ok(ConnectResponse { oauth_url, state })
}

    /// Fetches the current authenticated user profile using the provided JWT.
    pub async fn fetch_current_user(&self, bearer_jwt: &str) -> Result<Value> {
        let url = self.base.join("auth/me").context("build /auth/me URL")?;
        let resp = self
        { … 12 line(s) … ⟦tj:b3f21ee589eb48124cfd1a9360249740⟧ }
        parse_api_response_json(&text)
}

    /// Exchanges a one-time login token (e.g. from Telegram) for a long-lived JWT.
    pub async fn consume_login_token(&self, login_token: &str) -> Result<String> {
        let token = login_token.trim();
        anyhow::ensure!(!token.is_empty(), "login token is required");
        { … 34 line(s) … ⟦tj:41cbd0ac227589fccd1722209a4d7a96⟧ }
        Ok(jwt)
}

    /// Validates that the provided session token is still active and accepted.
    pub async fn validate_session_token(&self, bearer_jwt: &str) -> Result<()> {
        let _ = self.fetch_current_user(bearer_jwt).await?;
        Ok(())
    }

    /// Creates a short-lived link token for connecting a specific communication channel.
    pub async fn create_channel_link_token(
        &self,
        channel: &str,
        bearer_jwt: &str,
    ) -> Result<Value> {
        let channel = channel.trim().trim_matches('/');
        anyhow::ensure!(!channel.is_empty(), "channel is required");
        { … 21 line(s) … ⟦tj:d4330e34484e04b2245b1fcecef2ad9c⟧ }
        parse_api_response_json(&text)
}

    /// Generic authenticated JSON request helper for backend API routes.
    pub async fn authed_json(
        &self,
        bearer_jwt: &str,
        method: Method,
        path: &str,
        body: Option<Value>,
    ) -> Result<Value> {
        let url = self
            .base
            .join(path.trim_start_matches('/'))
            .with_context(|| format!("build URL for {path}"))?;

        let mut request = self
            .client
            .request(method.clone(), url.clone())
            .header(AUTHORIZATION, bearer_authorization_value(bearer_jwt));

        if let Some(body) = body {
            request = request.json(&body);
        }

        let response = request.send().await.map_err(|e| {
            // Walk the error source chain so transient markers hidden in nested
            // causes (reqwest -> hyper -> rustls TLS EOF, etc.) still classify
            // correctly. The top-level `e.to_string()` often only carries the
            // outermost wrapper, e.g. "error sending request for url (...)".
            let mut error_message = e.to_string();
            let mut src: Option<&(dyn std::error::Error + 'static)> = std::error::Error::source(&e);
            while let Some(s) = src {
                error_message.push_str(" → ");
                error_message.push_str(&s.to_string());
                src = s.source();
            }
            if crate::core::observability::contains_transient_transport_phrase(&error_message) {
                tracing::warn!(
                    domain = "backend_api",
                    operation = "authed_json",
                    method = method.as_str(),
                    path = url.path(),
                    failure = "transport",
                    error = %error_message,
                    "[backend_api] transient transport failure on {} {}: {}",
                    method.as_str(),
                    url.path(),
                    error_message,
                );
            } else {
                crate::core::observability::report_error(
                    error_message.as_str(),
                    "backend_api",
                    "authed_json",
                    &[
                        ("method", method.as_str()),
                        ("path", url.path()),
                        ("failure", "transport"),
                    ],
                );
            }
            anyhow::Error::new(e).context(format!(
                "backend request {} {}",
                method.as_str(),
                url.path()
            ))
        })?;

        let status = response.status();
        let text = response.text().await.unwrap_or_default();
        if !status.is_success() {
            let status_code = status.as_u16();
            let status_str = status_code.to_string();

            // 401 on any authed backend endpoint is an expected user-session
            // state (token expired / revoked / rotated server-side), not a
            // code bug — every authed endpoint will see this once the session
            // lapses. Surface a typed `BackendApiError::Unauthorized` so the
            // auth domain can drive recovery, and skip `report_error` to
            // avoid Sentry noise. Targets `OPENHUMAN-TAURI-4K8` (mascot TTS
            // surfaced it first on `/openai/v1/audio/speech`, but the same
            // shape applies to every `authed_json` path).
            if status_code == 401 {
                tracing::info!(
                    domain = "backend_api",
                    operation = "authed_json",
                    method = method.as_str(),
                    path = url.path(),
                    status = status_code,
                    failure = "non_2xx",
                    "[backend_api] 401 on {} {} — session token rejected, surfacing typed error",
                    method.as_str(),
                    url.path(),
                );
                return Err(anyhow::Error::new(BackendApiError::Unauthorized {
                    method: method.as_str().to_string(),
                    path: url.path().to_string(),
                }));
            }

            // 404 on `/channels/<provider>/messages/<id>` is an expected
            // state (user deleted the message provider-side, or backend
            // GC'd the relay row) — not a code bug. Surface a typed
            // `BackendApiError::MessageNotFound` so callers (`bus.rs`
            // streaming/thinking/delete/final paths) can clear stale
            // ids and skip retry, without funneling the 404 into
            // `report_error`. Targets `OPENHUMAN-TAURI-2Y` (~454 events).
            if status_code == 404 {
                if let Some((provider, message_id)) = parse_message_path(url.path()) {
                    tracing::info!(
                        domain = "backend_api",
                        operation = "authed_json",
                        provider = provider,
                        message_id = message_id,
                        "[backend_api] message-not-found 404 on {} {} — surfacing typed error",
                        method.as_str(),
                        url.path(),
                    );
                    return Err(anyhow::Error::new(BackendApiError::MessageNotFound {
                        provider: provider.to_string(),
                        message_id: message_id.to_string(),
                    }));
                }
                // Defense-in-depth: PATCH/DELETE 404s on any channel-message path that
                // parse_message_path could not parse (e.g. exotic URL variant with extra
                // segments). Still an expected backend state — suppress the Sentry event
                // without propagating a typed error. Targets OPENHUMAN-TAURI-R7.
                if (method == Method::PATCH || method == Method::DELETE)
                    && url.path().contains("/channels/")
                    && url.path().contains("/messages/")
                {
                    tracing::debug!(
                        domain = "backend_api",
                        operation = "authed_json",
                        "[backend_api] channel-message 404 on {} {} — path not matched by \
                         parse_message_path, suppressing Sentry (TAURI-R7 defense-in-depth)",
                        method.as_str(),
                        url.path(),
                    );
                    anyhow::bail!(
                        "channel message not found (404) on {} {}",
                        method.as_str(),
                        url.path(),
                    );
                }
            }

            // These are transient infrastructure errors (proxy/CDN/backend
            // temporarily unavailable). They are not code bugs and callers already
            // implement retry/disable logic, so skip Sentry to avoid noise.
            let is_transient_infra =
                crate::core::observability::is_transient_http_status_code(status_code);
            let is_budget_exhausted = status_code == 400
                && crate::openhuman::inference::provider::is_budget_exhausted_message(&text);
            if is_budget_exhausted {
                tracing::info!(
                    method = method.as_str(),
                    path = url.path(),
                    status = status_code,
                    failure = "non_2xx",
                    kind = "budget",
                    "[backend_api] budget-exhausted 400 on {} {} — not reporting to Sentry",
                    method.as_str(),
                    url.path(),
                );
            } else if is_transient_infra {
                tracing::warn!(
                    domain = "backend_api",
                    operation = "authed_json",
                    method = method.as_str(),
                    path = url.path(),
                    status = status_code,
                    failure = "non_2xx",
                    "[backend_api] transient {status} on {} {} — not reporting to Sentry",
                    method.as_str(),
                    url.path(),
                );
            } else {
                // Enrich the report with the two fields triage needs to pin a
                // non-2xx's origin: the outbound `host` and a PII-safe `body_shape`
                // (top-level JSON key names only — never values; see
                // `backend_api_body_shape`). `report_error` previously logged only
                // `response_body_len`, leaving us blind when a client hits a
                // non-canonical backend (custom BACKEND_URL / proxy / foreign
                // host) — TAURI-RUST-8C: 12k `GET /teams/me/usage` 404s from one
                // user whose 91-byte body matched no route this backend emits,
                // un-diagnosable because neither host nor shape was captured.
                // `host_str()` carries no scheme/path/query/token. Telemetry only
                // — the error still propagates below (no suppression).
                let host = url.host_str().unwrap_or("");
                let body_shape = backend_api_body_shape(&text);
                crate::core::observability::report_error(
                    format!(
                        "{} {} failed ({status}); response_body_len={}; body_shape={}",
                        method.as_str(),
                        url.path(),
                        text.len(),
                        body_shape,
                    )
                    .as_str(),
                    "backend_api",
                    "authed_json",
                    &[
                        ("method", method.as_str()),
                        ("path", url.path()),
                        ("host", host),
                        ("status", status_str.as_str()),
                        ("failure", "non_2xx"),
                    ],
                );
            }
            anyhow::bail!(
                "{} {} failed ({status}): {text}",
                method.as_str(),
                url.path()
            );
        }

        parse_api_response_json(&text)
    }

    /// Lists all active integrations for the current user.
    pub async fn list_integrations(&self, bearer_jwt: &str) -> Result<Vec<IntegrationSummary>> {
        let url = self
            .base
        { … 20 line(s) … ⟦tj:b85029dae3250be552af7b5ba1eed31d⟧ }
        Ok(env.data.integrations)
}

    /// Fetches the decrypted OAuth tokens for a specific integration.
    ///
    /// This is a one-time handoff process. The encryption key must match the
    /// one used by the backend to encrypt the tokens.
    pub async fn fetch_integration_tokens_handoff(
        &self,
        integration_id: &str,
        bearer_jwt: &str,
        encryption_key: &str,
    ) -> Result<IntegrationTokensHandoff> {
        let id = integration_id.trim();
        anyhow::ensure!(
        { … 28 line(s) … ⟦tj:a134660949c59e8e4c486e23621b2895⟧ }
        serde_json::from_str(&plaintext).context("parse decrypted token JSON")
}

    /// Fetches the client key share for a specific integration.
    ///
    /// This is a one-time handoff; the key is deleted from the backend's
    /// temporary storage (Redis) after retrieval.
    pub async fn fetch_client_key(&self, integration_id: &str, bearer_jwt: &str) -> Result<String> {
        let id = integration_id.trim();
        anyhow::ensure!(
        { … 39 line(s) … ⟦tj:beeb0770add0293e7e7c8dde328cc4ab⟧ }
        Ok(client_key.to_string())
}

    /// Sends a message to a communication channel.
    pub async fn send_channel_message(
        &self,
        channel: &str,
        bearer_jwt: &str,
        message_body: Value,
    ) -> Result<Value> { … 12 line(s) … ⟦tj:dcc5e9f0eabd22a191c23f86ad5f532c⟧ }

    /// Signals "the agent is typing…" on a channel that supports it
    /// (Telegram's `sendChatAction`, Slack's typing event, …). The backend
    /// resolves the target chat from the channel integration metadata and
    /// is responsible for hitting the provider-native API.
    ///
    /// Telegram keeps the typing indicator alive for ~5 seconds per call,
    /// so callers should re-invoke every ~4 s for as long as the turn is
    /// in flight. Returns `Err` if the backend doesn't support typing for
    /// this channel — caller should swallow the error silently.
    pub async fn send_channel_typing(&self, channel: &str, bearer_jwt: &str) -> Result<Value> { … 12 line(s) … ⟦tj:6ea01fd4c9918ff55990eff1f6d1b0a1⟧ }

    /// Edits an existing channel message. Used by the progressive-edit
    /// streaming path (Telegram / Slack) to coalesce live deltas into a
    /// single evolving outbound message rather than spamming the chat
    /// with one bubble per token.
    ///
    /// `message_id` is the backend-returned id of the message that was
    /// first sent via [`Self::send_channel_message`]. Returns the
    /// updated message record, or an `Err` if the backend does not
    /// support editing for this channel (caller should fall back to
    /// atomic-final delivery).
    pub async fn send_channel_edit(
        &self,
        channel: &str,
        message_id: &str,
        bearer_jwt: &str,
        edit_body: Value,
    ) -> Result<Value> {
        let channel = channel.trim().trim_matches('/');
        anyhow::ensure!(!channel.is_empty(), "channel is required");
        { … 9 line(s) … ⟦tj:aeb257647fddbf4ac766f1edb57cea77⟧ }
        .await
}

    /// Deletes a message from a communication channel. Used to clean up
    /// ephemeral messages (e.g. thinking indicators) after the final
    /// response has been delivered.
    pub async fn send_channel_delete(
        &self,
        channel: &str,
        message_id: &str,
        bearer_jwt: &str,
    ) -> Result<Value> {
        let channel = channel.trim().trim_matches('/');
        anyhow::ensure!(!channel.is_empty(), "channel is required");
        { … 9 line(s) … ⟦tj:6f0804714a20d92383924fe3d229e06b⟧ }
        .await
}

    /// Sends a reaction (e.g. emoji) to a message in a channel.
    pub async fn send_channel_reaction(
        &self,
        channel: &str,
        bearer_jwt: &str,
        reaction_body: Value,
    ) -> Result<Value> { … 12 line(s) … ⟦tj:e8200dccb5c3a946c2e9659129bb300a⟧ }

    /// Creates a new thread in a communication channel.
    pub async fn create_channel_thread(
        &self,
        channel: &str,
        bearer_jwt: &str,
        title: &str,
    ) -> Result<Value> {
        let channel = channel.trim().trim_matches('/');
        anyhow::ensure!(!channel.is_empty(), "channel is required");
        { … 9 line(s) … ⟦tj:5b99cc3d7363d7a0e8a620ce23144d63⟧ }
        .await
}

    /// Updates an existing thread (e.g., closing or reopening it).
    pub async fn update_channel_thread(
        &self,
        channel: &str,
        bearer_jwt: &str,
        thread_id: &str,
        action: &str,
    ) -> Result<Value> {
        let channel = channel.trim().trim_matches('/');
        anyhow::ensure!(!channel.is_empty(), "channel is required");
        { … 14 line(s) … ⟦tj:6e88a2298172ac07cc351ec7073be4c9⟧ }
        .await
}

    /// Lists threads in a communication channel, optionally filtering by status.
    pub async fn list_channel_threads(
        &self,
        channel: &str,
        bearer_jwt: &str,
        active_filter: Option<bool>,
    ) -> Result<Value> {
        let channel = channel.trim().trim_matches('/');
        anyhow::ensure!(!channel.is_empty(), "channel is required");
        { … 9 line(s) … ⟦tj:cae2fee2152a60395cca8f7d1f28fa92⟧ }
        self.authed_json(bearer_jwt, Method::GET, &path, None).await
}

    /// Revokes (deletes) an active integration.
    pub async fn revoke_integration(&self, integration_id: &str, bearer_jwt: &str) -> Result<()> {
        let id = integration_id.trim();
        anyhow::ensure!(!id.is_empty(), "integration id is required");
        { … 17 line(s) … ⟦tj:1464e692211475eaa6c05f68adfdce70⟧ }
        Ok(())
}
}

/// AES-256-GCM decrypt compatible with backend `encryptMessageFromString` (IV 16 + tag 16 + ciphertext, base64).
pub fn decrypt_handoff_blob(b64_ciphertext: &str, key_str: &str) -> Result<String> {
    let key = key_bytes_from_string(key_str)?;
    let combined = base64::engine::general_purpose::STANDARD
    { … 27 line(s) … ⟦tj:2403473aea18f7be5173ff249ee681ac⟧ }
    String::from_utf8(plain).context("handoff plaintext is not UTF-8")
}

/// Decode the shared encryption key into 32 raw AES bytes.
///
/// Accepts, in order of preference:
/// 1. base64url without padding — the current backend format (e.g.
///    a 43-char alphanumeric string using `-` / `_`). This must be tried
///    BEFORE standard base64 because `-`/`_` are invalid in the standard
///    alphabet and would fail cleanly, whereas a standard-base64 string
///    never contains `-`/`_` so base64url_no_pad will still decode it
///    correctly as long as there's no padding.
/// 2. base64url with padding.
/// 3. Standard base64 with padding (legacy backend format).
/// 4. Standard base64 without padding.
/// 5. A raw 32-byte ASCII key (len == 32, used as-is).
///
/// NOTE: the key is only decoded locally for AES-GCM key material in
/// `decrypt_handoff_blob`. The key sent back to the backend (in the
/// `{ key: ... }` POST body of `fetch_integration_tokens_handoff`) is the
/// original string — never re-encoded — so base64url keys stay base64url
/// on the wire.
fn key_bytes_from_string(key: &str) -> Result<Vec<u8>> {
    let trimmed = key.trim();

    { … 27 line(s) … ⟦tj:7b325fc299e0ddad7d13259ceb3a28c5⟧ }
    );
}

#[cfg(test)]
#[path = "rest_tests.rs"]
mod key_bytes_from_string_tests;
[omitted blocks are individually retrievable: call tinyjuice_retrieve with the token inside an omission marker to expand just that block]

[PARTIAL view — full original (48071 bytes): call tinyjuice_retrieve with token "96e2b866b8c25b3f06c574818d1a9745"]