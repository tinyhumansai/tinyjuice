use super::{
    backend_api_body_shape, flatten_authed_error, key_bytes_from_string, parse_message_path,
    sanitize_client_version, BackendApiError, BackendOAuthClient, BACKEND_API_BODY_SHAPE_MAX_BYTES,
};
use axum::extract::State;
use axum::http::HeaderMap;
use axum::routing::{get, post};
use axum::{Json, Router};
use base64::engine::general_purpose::{STANDARD, URL_SAFE_NO_PAD};
use base64::Engine;
use reqwest::Method;
use serde_json::{json, Value};
use std::sync::{Arc, Mutex};
use tokio::net::TcpListener;

#[test]
fn decodes_base64url_no_pad() { … 12 line(s) … }

#[test]
fn decodes_standard_base64() {
    let raw = [0x41_u8; 32];
    let std_key = STANDARD.encode(raw);
    let decoded = key_bytes_from_string(&std_key).unwrap();
    assert_eq!(decoded, raw);
}

#[test]
fn decodes_raw_32_byte_key() {
    let raw = "abcdefghijklmnopqrstuvwxyz012345";
    assert_eq!(raw.len(), 32);
    let decoded = key_bytes_from_string(raw).unwrap();
    assert_eq!(decoded, raw.as_bytes());
}

#[test]
fn trims_whitespace() {
    let raw = [0x42_u8; 32];
    let url_key = format!("  {}\n", URL_SAFE_NO_PAD.encode(raw));
    let decoded = key_bytes_from_string(&url_key).unwrap();
    assert_eq!(decoded, raw);
}

#[test]
fn rejects_wrong_length() {
    let err = key_bytes_from_string("tooshort").unwrap_err();
    assert!(err.to_string().contains("must decode to 32 raw bytes"));
}

use super::user_id_from_profile_payload;

#[test]
fn extracts_id_from_root() {
    let payload1 = json!({ "id": "123" });
    let payload2 = json!({ "_id": "456" });
    let payload3 = json!({ "userId": "789" });

    assert_eq!(user_id_from_profile_payload(&payload1).unwrap(), "123");
    assert_eq!(user_id_from_profile_payload(&payload2).unwrap(), "456");
    assert_eq!(user_id_from_profile_payload(&payload3).unwrap(), "789");
}

#[test]
fn extracts_id_from_data_nested() {
    let payload = json!({
        "data": { "id": "abc" }
    });
    assert_eq!(user_id_from_profile_payload(&payload).unwrap(), "abc");
}

#[test]
fn extracts_id_from_user_nested() {
    let payload = json!({
        "user": { "id": "def" }
    });
    assert_eq!(user_id_from_profile_payload(&payload).unwrap(), "def");
}

#[test]
fn extracts_id_from_data_user_nested() {
    let payload = json!({
        "data": {
            "user": { "userId": "ghi" }
        }
    });
    assert_eq!(user_id_from_profile_payload(&payload).unwrap(), "ghi");
}

#[test]
fn ignores_whitespace_only_ids() {
    let payload = json!({
        "data": {
            "id": "   ",
            "_id": "real_id"
        }
    });
    assert_eq!(user_id_from_profile_payload(&payload).unwrap(), "real_id");
}

#[test]
fn trims_extracted_ids() {
    let payload = json!({
        "id": "  padded_id  "
    });
    assert_eq!(user_id_from_profile_payload(&payload).unwrap(), "padded_id");
}

#[test]
fn rejects_non_string_ids() {
    let payload = json!({
        "id": 123,
        "_id": ["not_a_string"],
        "userId": "valid_id"
    });
    assert_eq!(user_id_from_profile_payload(&payload).unwrap(), "valid_id");
}

#[test]
fn returns_none_for_missing_ids() {
    let payload = json!({
        "data": { "name": "alice" }
    });
    assert!(user_id_from_profile_payload(&payload).is_none());
}

#[test]
fn returns_none_for_non_object_payload() {
    let payload = json!("just a string");
    assert!(user_id_from_profile_payload(&payload).is_none());
}

#[test]
fn sanitize_client_version_strips_invalid_chars_and_clamps_length() {
    let raw = format!(" 1.2.3 (desktop)+build!?{} ", "a".repeat(80));
    let sanitized = sanitize_client_version(&raw).unwrap();
    assert_eq!(sanitized, format!("1.2.3desktop+build{}", "a".repeat(46)));
    assert_eq!(sanitized.len(), 64);
}

#[derive(Clone, Default)]
struct CapturedHeaders {
    entries: Arc<Mutex<Vec<HeaderMap>>>,
}

impl CapturedHeaders {
    fn push(&self, headers: &HeaderMap) {
        self.entries.lock().unwrap().push(headers.clone());
    }

    fn take(&self) -> Vec<HeaderMap> {
        self.entries.lock().unwrap().clone()
    }
}

async fn spawn_header_capture_server() -> (String, CapturedHeaders) {
    async fn capture_consume(
        State(captured): State<CapturedHeaders>,
    { … 28 line(s) … }
    (format!("http://{addr}"), captured)
}

#[tokio::test]
async fn backend_client_sends_x_core_version_on_auth_requests() {
    let (base_url, captured) = spawn_header_capture_server().await;
    let client = BackendOAuthClient::new(&base_url).unwrap();
    { … 13 line(s) … }
    );
}

#[tokio::test]
async fn backend_client_sends_x_tauri_version_when_env_set() {
    // Serialize against any concurrent test that also touches this env var.
    static ENV_LOCK: Mutex<()> = Mutex::new(());
    { … 18 line(s) … }
    assert!(request_headers.get("x-core-version").is_some());
}

// Regression: OPENHUMAN-TAURI-8K / Sentry issue 7473650958.
// When config.api_url is a full LLM completions URL (e.g. /v1/chat/completions),
// Url::join used to produce wrong paths like /v1/chat/teams/me/usage instead of
// /teams/me/usage — BackendOAuthClient::new must strip the path to prevent this.
#[test]
fn new_strips_path_from_completions_url() {
    let client = BackendOAuthClient::new("https://api.tinyhumans.ai/v1/chat/completions").unwrap();
    let url = client.url_for("/teams/me/usage").unwrap();
    assert_eq!(url.path(), "/teams/me/usage");
}

#[test]
fn new_strips_path_from_openai_style_url() {
    let client = BackendOAuthClient::new("https://api.openai.com/v1/chat/completions").unwrap();
    let url = client.url_for("/teams/me/usage").unwrap();
    assert_eq!(url.path(), "/teams/me/usage");
    assert_eq!(url.host_str(), Some("api.openai.com"));
}

#[test]
fn new_works_with_bare_origin() {
    let client = BackendOAuthClient::new("https://api.tinyhumans.ai").unwrap();
    let url = client.url_for("/teams/me/usage").unwrap();
    assert_eq!(url.path(), "/teams/me/usage");
}

#[test]
fn new_works_with_trailing_slash() {
    let client = BackendOAuthClient::new("https://api.tinyhumans.ai/").unwrap();
    let url = client.url_for("/teams/me/usage").unwrap();
    assert_eq!(url.path(), "/teams/me/usage");
}

#[tokio::test]
async fn backend_raw_client_inherits_x_core_version_default_header() {
    let (base_url, captured) = spawn_header_capture_server().await;
    let client = BackendOAuthClient::new(&base_url).unwrap();
    { … 14 line(s) … }
    );
}

#[tokio::test]
async fn authed_json_surfaces_message_not_found_on_404() {
    let app = Router::new()
        .route(
            "/channels/telegram/messages/1103",
            post(|| async { (axum::http::StatusCode::NOT_FOUND, "Not Found") }),
        )
        .route(
            "/channels/discord/messages/abc",
            post(|| async { (axum::http::StatusCode::NOT_FOUND, "Not Found") }),
        );
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    tokio::spawn(async move {
        axum::serve(listener, app).await.unwrap();
    });

    let base_url = format!("http://{addr}");
    let client = BackendOAuthClient::new(&base_url).unwrap();

    // Telegram path — matches OPENHUMAN-TAURI-2Y shape.
    let err = client
        .authed_json(
            "mock-jwt",
            Method::POST,
            "/channels/telegram/messages/1103",
            None,
        )
        .await
        .unwrap_err();
    let typed = err.downcast_ref::<BackendApiError>().unwrap();
    let BackendApiError::MessageNotFound {
        provider,
        message_id,
    } = typed
    else {
        panic!("expected MessageNotFound, got {typed:?}");
    };
    assert_eq!(provider, "telegram");
    assert_eq!(message_id, "1103");

    // Discord path — proves the helper is provider-agnostic.
    let err = client
        .authed_json(
            "mock-jwt",
            Method::POST,
            "/channels/discord/messages/abc",
            None,
        )
        .await
        .unwrap_err();
    let typed = err.downcast_ref::<BackendApiError>().unwrap();
    let BackendApiError::MessageNotFound {
        provider,
        message_id,
    } = typed
    else {
        panic!("expected MessageNotFound, got {typed:?}");
    };
    assert_eq!(provider, "discord");
    assert_eq!(message_id, "abc");
}

#[tokio::test]
async fn authed_json_surfaces_unauthorized_on_401() {
    // OPENHUMAN-TAURI-4K8: 401 on any authed backend endpoint must surface a
    // typed `BackendApiError::Unauthorized` and NOT funnel into `report_error`.
    // The mascot TTS path (`/openai/v1/audio/speech`) was the loudest reporter,
    // but the same shape fires on every authed endpoint once a session lapses,
    // so we cover two different paths/methods to prove the suppression is
    // status-driven, not path-keyed.
    let app = Router::new()
        .route(
            "/openai/v1/audio/speech",
            post(|| async { (axum::http::StatusCode::UNAUTHORIZED, "Unauthorized") }),
        )
        .route(
            "/referral/stats",
            get(|| async { (axum::http::StatusCode::UNAUTHORIZED, "Unauthorized") }),
        );
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    tokio::spawn(async move {
        axum::serve(listener, app).await.unwrap();
    });

    let base_url = format!("http://{addr}");
    let client = BackendOAuthClient::new(&base_url).unwrap();

    // Mascot TTS path — the original reporter.
    let err = client
        .authed_json(
            "mock-jwt",
            Method::POST,
            "/openai/v1/audio/speech",
            Some(json!({ "text": "hello" })),
        )
        .await
        .unwrap_err();
    let typed = err.downcast_ref::<BackendApiError>().unwrap();
    let BackendApiError::Unauthorized { method, path } = typed else {
        panic!("expected Unauthorized, got {typed:?}");
    };
    assert_eq!(method, "POST");
    assert_eq!(path, "/openai/v1/audio/speech");

    // Generic GET on a non-TTS path — proves the suppression is per-status,
    // not per-path. (Same root cause: expired/revoked backend session.)
    let err = client
        .authed_json("mock-jwt", Method::GET, "/referral/stats", None)
        .await
        .unwrap_err();
    let typed = err.downcast_ref::<BackendApiError>().unwrap();
    let BackendApiError::Unauthorized { method, path } = typed else {
        panic!("expected Unauthorized, got {typed:?}");
    };
    assert_eq!(method, "GET");
    assert_eq!(path, "/referral/stats");
}

#[test]
fn backend_api_body_shape_emits_safe_keys_not_values() { … 11 line(s) … }

#[test]
fn backend_api_body_shape_redacts_pii_and_nonidentifier_keys() { … 12 line(s) … }

#[test]
fn backend_api_body_shape_classifies_non_object_bodies() { … 11 line(s) … }

#[test]
fn backend_api_body_shape_bounds_long_safe_key_list() {
    // The `safe=[…]` list is truncated at BACKEND_API_BODY_SHAPE_MAX_BYTES = 120.
    // Surviving keys are ASCII identifiers (non-ASCII keys are redacted upstream),
    { … 22 line(s) … }
    );
}

#[tokio::test]
async fn authed_json_reports_non_channel_404_still_propagates() {
    // TAURI-RUST-8C: a GET 404 on a non-channel path (e.g. `/teams/me/usage`)
    // falls through to `report_error` (not a typed/suppressed state) — it must
    { … 29 line(s) … }
    );
}

#[test]
fn flatten_authed_error_maps_unauthorized_to_session_expired_sentinel() {
    // #3297: the typed `Unauthorized` (expected session-lapse 401) must flatten
    // onto a string that the JSON-RPC session-expiry classifiers recognise, so
    { … 21 line(s) … }
    );
}

#[test]
fn flatten_authed_error_preserves_non_unauthorized_chain() {
    // A non-Unauthorized failure (e.g. a transient network/timeout error) keeps
    // its full `{e:#}` anyhow chain and must NOT be demoted to session expiry —
    { … 9 line(s) … }
    );
}

#[test]
fn flatten_authed_error_does_not_swallow_message_not_found() {
    // `MessageNotFound` is a different expected state handled by its own callers
    // (channel streaming/delete paths downcast it); it must not be collapsed
    { … 11 line(s) … }
    );
}

#[tokio::test]
async fn authed_json_403_is_not_demoted_to_unauthorized() {
    // 403 (Forbidden) is a genuine authorization/permission problem — the
    // token authenticated but lacked scope. That IS a code/config bug we
    { … 22 line(s) … }
    );
}

#[tokio::test]
async fn authed_json_404_outside_messages_path_still_reports() {
    // 404 on a non-`/channels/<provider>/messages/<id>` path should NOT be
    // demoted to MessageNotFound — it's a real backend bug or routing
    { … 21 line(s) … }
    );
}

// ── parse_message_path unit tests (TAURI-R7 regression guard) ───────────────

#[test]
fn parse_message_path_canonical_form() {
    assert_eq!(
        parse_message_path("/channels/telegram/messages/1103"),
        Some(("telegram", "1103"))
    );
}

#[test]
fn parse_message_path_discord_provider() {
    assert_eq!(
        parse_message_path("/channels/discord/messages/abc"),
        Some(("discord", "abc"))
    );
}

#[test]
fn parse_message_path_base_path_prefix() {
    // TAURI-R7 root cause: BACKEND_URL with a path prefix adds segments,
    // breaking the strict 4-segment check. The sliding window must handle it.
    assert_eq!(
        parse_message_path("/api/v1/channels/telegram/messages/1103"),
        Some(("telegram", "1103"))
    );
}

#[test]
fn parse_message_path_double_prefix() {
    assert_eq!(
        parse_message_path("/v2/api/channels/discord/messages/abc"),
        Some(("discord", "abc"))
    );
}

#[test]
fn parse_message_path_trailing_slash() {
    assert_eq!(
        parse_message_path("/channels/telegram/messages/1103/"),
        Some(("telegram", "1103"))
    );
}

#[test]
fn parse_message_path_percent_encoded_slug() {
    // Channel slugs with percent-encoded characters must pass through verbatim.
    assert_eq!(
        parse_message_path("/channels/telegram%3Abot/messages/1103"),
        Some(("telegram%3Abot", "1103"))
    );
}

#[test]
fn parse_message_path_non_message_path_returns_none() {
    assert_eq!(parse_message_path("/channels/telegram/typing"), None);
    assert_eq!(parse_message_path("/channels/telegram"), None);
    assert_eq!(parse_message_path("/auth/profile"), None);
    assert_eq!(parse_message_path("/"), None);
    assert_eq!(parse_message_path(""), None);
}

// ── authed_json defense-in-depth: PATCH 404 with base-path prefix ───────────

#[tokio::test]
async fn authed_json_patch_404_with_base_path_prefix_does_not_report() {
    // Regression for TAURI-R7: if the resolved URL has a base-path prefix,
    // authed_json must still suppress the 404 (either via parse_message_path
    { … 40 line(s) … }
    assert_eq!(message_id, "9999");
}