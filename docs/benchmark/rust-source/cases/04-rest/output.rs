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
pub fn flatten_authed_error(err: anyhow::Error) -> String { … 8 line(s) … ⟦tj:da698ccd8a30417b1e744eaa6c8dbc86⟧ }

/// Extract `(provider, message_id)` from a backend channel path of the
/// shape `…/channels/<provider>/messages/<id>`. Returns `None` for paths
/// that do not contain this four-segment subsequence.
///
/// Handles both the canonical four-segment form and paths with an arbitrary
/// base-path prefix (e.g. `/api/v1/channels/telegram/messages/1103`) via a
/// sliding window so that `BACKEND_URL` variants with path prefixes do not
/// silently fall through to `report_error` (OPENHUMAN-TAURI-R7).
fn parse_message_path(path: &str) -> Option<(&str, &str)> { … 14 line(s) … ⟦tj:e1ba36b9d80e4eea767369549a6be4c2⟧ }

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
fn backend_api_body_shape(body: &str) -> String { … 29 line(s) … ⟦tj:25e1fe0e8456169a00a40a7af4ed22e2⟧ }

/// A JSON key safe to echo into telemetry: a short ASCII identifier (the shape
/// of a schema field name). Anything else — non-ASCII, punctuation like `@`,
/// whitespace, or overlong — is treated as response-controlled data and excluded
/// so `body_shape` can never leak an email/UUID/free-text used as a key.
fn is_schema_like_key(key: &str) -> bool { … 8 line(s) … ⟦tj:f02e0ef286ecbbc1209c551d62fa845a⟧ }

fn sanitize_client_version(raw: &str) -> Option<String> { … 14 line(s) … ⟦tj:575295daac4c9b3d4b889be7758377cc⟧ }

fn build_backend_reqwest_client() -> Result<Client> { … 31 line(s) … ⟦tj:ebad9d86d21fbd345b90abc2577bc5bf⟧ }

fn parse_api_response_json(text: &str) -> Result<Value> { … 30 line(s) … ⟦tj:252ec75d708c945e3bfa5471d1ae893a⟧ }

fn user_id_from_object(obj: &serde_json::Map<String, Value>) -> Option<String> { … 11 line(s) … ⟦tj:dd49a8b5a840154743d78cacdd62a874⟧ }

/// Best-effort extraction of a user ID from an authenticated profile payload.
///
/// This function handles various envelope formats, including raw user objects
/// or those nested under `data` or `user` keys.
pub fn user_id_from_profile_payload(payload: &Value) -> Option<String> { … 16 line(s) … ⟦tj:62d5f9c928cfe963ee02d655490c88f0⟧ }

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
    pub fn new(api_base: &str) -> Result<Self> { … 12 line(s) … ⟦tj:aaddf236a710acb3932a84a99bcade0b⟧ }

    /// Borrow the underlying `reqwest::Client` for callers that need to
    /// drive a non-JSON request shape (e.g. `multipart/form-data` uploads
    /// for cloud STT) without re-implementing TLS/proxy plumbing.
    pub fn raw_client(&self) -> &Client {
        &self.client
    }

    /// Resolve a backend-relative path against the configured base URL.
    /// Mirrors what `authed_json` does internally so callers using
    /// `raw_client()` don't have to assemble URLs by hand.
    pub fn url_for(&self, path: &str) -> Result<Url> { … 5 line(s) … ⟦tj:00a5addc8c4559ff9055b5229c8668a4⟧ }

    /// Returns the URL for initiating a login flow for a specific provider.
    pub fn login_url(&self, provider: &str) -> Result<Url> { … 7 line(s) … ⟦tj:5aedd26225bbe65a8c00dff5d0b2b6d2⟧ }

    /// Initiates an OAuth connection flow for the current user and a specific provider.
    pub async fn connect(
        &self,
        provider: &str,
        bearer_jwt: &str,
        skill_id: Option<&str>,
        response_type: Option<&str>,
        encryption_mode: Option<&str>,
    ) -> Result<ConnectResponse> { … 46 line(s) … ⟦tj:9dc9ef7bb15dc308504a37c3c3080dcc⟧ }

    /// Fetches the current authenticated user profile using the provided JWT.
    pub async fn fetch_current_user(&self, bearer_jwt: &str) -> Result<Value> { … 17 line(s) … ⟦tj:b3f21ee589eb48124cfd1a9360249740⟧ }

    /// Exchanges a one-time login token (e.g. from Telegram) for a long-lived JWT.
    pub async fn consume_login_token(&self, login_token: &str) -> Result<String> { … 39 line(s) … ⟦tj:41cbd0ac227589fccd1722209a4d7a96⟧ }

    /// Validates that the provided session token is still active and accepted.
    pub async fn validate_session_token(&self, bearer_jwt: &str) -> Result<()> { … 4 line(s) … ⟦tj:f850c9d98f60ff16ceb47fd72c07a248⟧ }

    /// Creates a short-lived link token for connecting a specific communication channel.
    pub async fn create_channel_link_token(
        &self,
        channel: &str,
        bearer_jwt: &str,
    ) -> Result<Value> { … 26 line(s) … ⟦tj:d4330e34484e04b2245b1fcecef2ad9c⟧ }

    /// Generic authenticated JSON request helper for backend API routes.
    pub async fn authed_json(
        &self,
        bearer_jwt: &str,
        method: Method,
        path: &str,
        body: Option<Value>,
    ) -> Result<Value> { … 211 line(s) … ⟦tj:7e5df346187de9d1e342ab9c957d9081⟧ }

    /// Lists all active integrations for the current user.
    pub async fn list_integrations(&self, bearer_jwt: &str) -> Result<Vec<IntegrationSummary>> { … 25 line(s) … ⟦tj:b85029dae3250be552af7b5ba1eed31d⟧ }

    /// Fetches the decrypted OAuth tokens for a specific integration.
    ///
    /// This is a one-time handoff process. The encryption key must match the
    /// one used by the backend to encrypt the tokens.
    pub async fn fetch_integration_tokens_handoff(
        &self,
        integration_id: &str,
        bearer_jwt: &str,
        encryption_key: &str,
    ) -> Result<IntegrationTokensHandoff> { … 33 line(s) … ⟦tj:a134660949c59e8e4c486e23621b2895⟧ }

    /// Fetches the client key share for a specific integration.
    ///
    /// This is a one-time handoff; the key is deleted from the backend's
    /// temporary storage (Redis) after retrieval.
    pub async fn fetch_client_key(&self, integration_id: &str, bearer_jwt: &str) -> Result<String> { … 44 line(s) … ⟦tj:beeb0770add0293e7e7c8dde328cc4ab⟧ }

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
    ) -> Result<Value> { … 14 line(s) … ⟦tj:aeb257647fddbf4ac766f1edb57cea77⟧ }

    /// Deletes a message from a communication channel. Used to clean up
    /// ephemeral messages (e.g. thinking indicators) after the final
    /// response has been delivered.
    pub async fn send_channel_delete(
        &self,
        channel: &str,
        message_id: &str,
        bearer_jwt: &str,
    ) -> Result<Value> { … 14 line(s) … ⟦tj:6f0804714a20d92383924fe3d229e06b⟧ }

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
    ) -> Result<Value> { … 14 line(s) … ⟦tj:5b99cc3d7363d7a0e8a620ce23144d63⟧ }

    /// Updates an existing thread (e.g., closing or reopening it).
    pub async fn update_channel_thread(
        &self,
        channel: &str,
        bearer_jwt: &str,
        thread_id: &str,
        action: &str,
    ) -> Result<Value> { … 19 line(s) … ⟦tj:6e88a2298172ac07cc351ec7073be4c9⟧ }

    /// Lists threads in a communication channel, optionally filtering by status.
    pub async fn list_channel_threads(
        &self,
        channel: &str,
        bearer_jwt: &str,
        active_filter: Option<bool>,
    ) -> Result<Value> { … 14 line(s) … ⟦tj:cae2fee2152a60395cca8f7d1f28fa92⟧ }

    /// Revokes (deletes) an active integration.
    pub async fn revoke_integration(&self, integration_id: &str, bearer_jwt: &str) -> Result<()> { … 22 line(s) … ⟦tj:1464e692211475eaa6c05f68adfdce70⟧ }
}

/// AES-256-GCM decrypt compatible with backend `encryptMessageFromString` (IV 16 + tag 16 + ciphertext, base64).
pub fn decrypt_handoff_blob(b64_ciphertext: &str, key_str: &str) -> Result<String> { … 32 line(s) … ⟦tj:2403473aea18f7be5173ff249ee681ac⟧ }

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
fn key_bytes_from_string(key: &str) -> Result<Vec<u8>> { … 32 line(s) … ⟦tj:7b325fc299e0ddad7d13259ceb3a28c5⟧ }

#[cfg(test)]
#[path = "rest_tests.rs"]
mod key_bytes_from_string_tests;
[omitted blocks are individually retrievable: call tinyjuice_retrieve with the token inside an omission marker to expand just that block]

[compacted tool output — this is a PARTIAL view; the full original (48071 bytes) is available by calling tinyjuice_retrieve with token "96e2b866b8c25b3f06c574818d1a9745" (marker ⟦tj:96e2b866b8c25b3f06c574818d1a9745⟧)]