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
fn decodes_base64url_no_pad() { … 12 line(s) … ⟦tj:ddf29b8d10b80e7e212d08535a344a6a⟧ }

#[test]
fn decodes_standard_base64() { … 6 line(s) … ⟦tj:39e7a405158b4a2bb93f1b93a288977b⟧ }

#[test]
fn decodes_raw_32_byte_key() { … 6 line(s) … ⟦tj:1f13f406f0fbc06a10064473ab59e0ea⟧ }

#[test]
fn trims_whitespace() { … 6 line(s) … ⟦tj:8893f6a3af7a5243d8c83bb6395b541f⟧ }

#[test]
fn rejects_wrong_length() { … 4 line(s) … ⟦tj:2a9bb9ec7d041f12b84985a6a9771a8c⟧ }

use super::user_id_from_profile_payload;

#[test]
fn extracts_id_from_root() { … 9 line(s) … ⟦tj:5773768f2eb30e66724da281b08aa272⟧ }

#[test]
fn extracts_id_from_data_nested() { … 6 line(s) … ⟦tj:e1ee621b90c91e62d6b73cf1cfdb9401⟧ }

#[test]
fn extracts_id_from_user_nested() { … 6 line(s) … ⟦tj:f621be448a674b2c9a7c33ebb8f0a1f2⟧ }

#[test]
fn extracts_id_from_data_user_nested() { … 8 line(s) … ⟦tj:a385a726076610d49ee904183dea7db6⟧ }

#[test]
fn ignores_whitespace_only_ids() { … 9 line(s) … ⟦tj:ba042d0abfbbef96787d3b0c2d6a327d⟧ }

#[test]
fn trims_extracted_ids() { … 6 line(s) … ⟦tj:12f6c492cf7dabbd943cb8e253e5f84f⟧ }

#[test]
fn rejects_non_string_ids() { … 8 line(s) … ⟦tj:599764046fe8a179f7dc5c065111d8de⟧ }

#[test]
fn returns_none_for_missing_ids() { … 6 line(s) … ⟦tj:2efcb5daa67eb02283c29dc68329a423⟧ }

#[test]
fn returns_none_for_non_object_payload() { … 4 line(s) … ⟦tj:915b4e228e4c73bcc9f9966a8b4122e2⟧ }

#[test]
fn sanitize_client_version_strips_invalid_chars_and_clamps_length() { … 6 line(s) … ⟦tj:682337378bb5ecf94632306e0709909e⟧ }

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

async fn spawn_header_capture_server() -> (String, CapturedHeaders) { … 33 line(s) … ⟦tj:82492fc04efaeb034f13e1b5c1142955⟧ }

#[tokio::test]
async fn backend_client_sends_x_core_version_on_auth_requests() { … 18 line(s) … ⟦tj:7fd443d2438553413484451b36aa1c88⟧ }

#[tokio::test]
async fn backend_client_sends_x_tauri_version_when_env_set() { … 23 line(s) … ⟦tj:00416ff5adbbbd1c2db3d0a14d3f46ef⟧ }

// Regression: OPENHUMAN-TAURI-8K / Sentry issue 7473650958.
// When config.api_url is a full LLM completions URL (e.g. /v1/chat/completions),
// Url::join used to produce wrong paths like /v1/chat/teams/me/usage instead of
// /teams/me/usage — BackendOAuthClient::new must strip the path to prevent this.
#[test]
fn new_strips_path_from_completions_url() { … 5 line(s) … ⟦tj:0e38abfeee34b87f3fcefa48174f316e⟧ }

#[test]
fn new_strips_path_from_openai_style_url() { … 6 line(s) … ⟦tj:c9ed61c0b95632b5363f319f3c601467⟧ }

#[test]
fn new_works_with_bare_origin() { … 5 line(s) … ⟦tj:f7c0e36766daa371912e7253506a59fe⟧ }

#[test]
fn new_works_with_trailing_slash() { … 5 line(s) … ⟦tj:905233d121b1818bda51aa1c26934c84⟧ }

#[tokio::test]
async fn backend_raw_client_inherits_x_core_version_default_header() { … 19 line(s) … ⟦tj:627ea2de77fe0533a0914c24a4a23d0e⟧ }

#[tokio::test]
async fn authed_json_surfaces_message_not_found_on_404() { … 61 line(s) … ⟦tj:717cd852142a7e7bc1a2e68b4b36d352⟧ }

#[tokio::test]
async fn authed_json_surfaces_unauthorized_on_401() { … 55 line(s) … ⟦tj:9fa166bbd27988efa5590850f7bc9ec4⟧ }

#[test]
fn backend_api_body_shape_emits_safe_keys_not_values() { … 11 line(s) … ⟦tj:0307fba2a39b92bd30702d313a608c9e⟧ }

#[test]
fn backend_api_body_shape_redacts_pii_and_nonidentifier_keys() { … 12 line(s) … ⟦tj:0a802ab7fd70b91488d1469613a6bbb6⟧ }

#[test]
fn backend_api_body_shape_classifies_non_object_bodies() { … 11 line(s) … ⟦tj:3707aef9fe263a20c412e512a0c3a2cb⟧ }

#[test]
fn backend_api_body_shape_bounds_long_safe_key_list() { … 27 line(s) … ⟦tj:4ca39f9332aabe3ca73374a625a69ad9⟧ }

#[tokio::test]
async fn authed_json_reports_non_channel_404_still_propagates() { … 34 line(s) … ⟦tj:bc8539adc3cd845315a9eda124c959a6⟧ }

#[test]
fn flatten_authed_error_maps_unauthorized_to_session_expired_sentinel() { … 26 line(s) … ⟦tj:da8b6b10540b061e477cf6bc7d774191⟧ }

#[test]
fn flatten_authed_error_preserves_non_unauthorized_chain() { … 14 line(s) … ⟦tj:2579fa79618571619321bcdee7d08b89⟧ }

#[test]
fn flatten_authed_error_does_not_swallow_message_not_found() { … 16 line(s) … ⟦tj:6835bb0f19ca341f86563586e73e545c⟧ }

#[tokio::test]
async fn authed_json_403_is_not_demoted_to_unauthorized() { … 27 line(s) … ⟦tj:d0ddd8c15f9c26a86863a41e769edd0b⟧ }

#[tokio::test]
async fn authed_json_404_outside_messages_path_still_reports() { … 26 line(s) … ⟦tj:953b3c5dad83a6bae71f32121e83ac3f⟧ }

// ── parse_message_path unit tests (TAURI-R7 regression guard) ───────────────

#[test]
fn parse_message_path_canonical_form() { … 6 line(s) … ⟦tj:d76df21a7b14af6685c2980799d0fba5⟧ }

#[test]
fn parse_message_path_discord_provider() { … 6 line(s) … ⟦tj:f7d463b38321e1e5fae6096dc18763e7⟧ }

#[test]
fn parse_message_path_base_path_prefix() { … 8 line(s) … ⟦tj:46688658aca17591cc79900dadec65fe⟧ }

#[test]
fn parse_message_path_double_prefix() { … 6 line(s) … ⟦tj:ac893d4aa885ec81e14c2db7e543c8df⟧ }

#[test]
fn parse_message_path_trailing_slash() { … 6 line(s) … ⟦tj:b5bf220278dc3b3e5e0e161528489bce⟧ }

#[test]
fn parse_message_path_percent_encoded_slug() { … 7 line(s) … ⟦tj:96437a052f103fcc83565f5c986f2bfa⟧ }

#[test]
fn parse_message_path_non_message_path_returns_none() { … 7 line(s) … ⟦tj:dad037a0b01041eba090f813c5d77cae⟧ }

// ── authed_json defense-in-depth: PATCH 404 with base-path prefix ───────────

#[tokio::test]
async fn authed_json_patch_404_with_base_path_prefix_does_not_report() { … 45 line(s) … ⟦tj:fb84c0b3ea60c51865609ea7f31485e0⟧ }
[collapsed bodies are individually retrievable: call tokenjuice_retrieve with the token inside a placeholder to expand just that body]

[compacted tool output — this is a PARTIAL view; the full original (26444 bytes) is available by calling tokenjuice_retrieve with token "95995e04f0b34dd67d20cf9ecfdfe4e3" (marker ⟦tj:95995e04f0b34dd67d20cf9ecfdfe4e3⟧)]