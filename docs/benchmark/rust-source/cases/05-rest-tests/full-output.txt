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
fn decodes_standard_base64() { … 6 line(s) … }

#[test]
fn decodes_raw_32_byte_key() { … 6 line(s) … }

#[test]
fn trims_whitespace() { … 6 line(s) … }

#[test]
fn rejects_wrong_length() { … 4 line(s) … }

use super::user_id_from_profile_payload;

#[test]
fn extracts_id_from_root() { … 9 line(s) … }

#[test]
fn extracts_id_from_data_nested() { … 6 line(s) … }

#[test]
fn extracts_id_from_user_nested() { … 6 line(s) … }

#[test]
fn extracts_id_from_data_user_nested() { … 8 line(s) … }

#[test]
fn ignores_whitespace_only_ids() { … 9 line(s) … }

#[test]
fn trims_extracted_ids() { … 6 line(s) … }

#[test]
fn rejects_non_string_ids() { … 8 line(s) … }

#[test]
fn returns_none_for_missing_ids() { … 6 line(s) … }

#[test]
fn returns_none_for_non_object_payload() { … 4 line(s) … }

#[test]
fn sanitize_client_version_strips_invalid_chars_and_clamps_length() { … 6 line(s) … }

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

async fn spawn_header_capture_server() -> (String, CapturedHeaders) { … 33 line(s) … }

#[tokio::test]
async fn backend_client_sends_x_core_version_on_auth_requests() { … 18 line(s) … }

#[tokio::test]
async fn backend_client_sends_x_tauri_version_when_env_set() { … 23 line(s) … }

// Regression: OPENHUMAN-TAURI-8K / Sentry issue 7473650958.
// When config.api_url is a full LLM completions URL (e.g. /v1/chat/completions),
// Url::join used to produce wrong paths like /v1/chat/teams/me/usage instead of
// /teams/me/usage — BackendOAuthClient::new must strip the path to prevent this.
#[test]
fn new_strips_path_from_completions_url() { … 5 line(s) … }

#[test]
fn new_strips_path_from_openai_style_url() { … 6 line(s) … }

#[test]
fn new_works_with_bare_origin() { … 5 line(s) … }

#[test]
fn new_works_with_trailing_slash() { … 5 line(s) … }

#[tokio::test]
async fn backend_raw_client_inherits_x_core_version_default_header() { … 19 line(s) … }

#[tokio::test]
async fn authed_json_surfaces_message_not_found_on_404() { … 61 line(s) … }

#[tokio::test]
async fn authed_json_surfaces_unauthorized_on_401() { … 55 line(s) … }

#[test]
fn backend_api_body_shape_emits_safe_keys_not_values() { … 11 line(s) … }

#[test]
fn backend_api_body_shape_redacts_pii_and_nonidentifier_keys() { … 12 line(s) … }

#[test]
fn backend_api_body_shape_classifies_non_object_bodies() { … 11 line(s) … }

#[test]
fn backend_api_body_shape_bounds_long_safe_key_list() { … 27 line(s) … }

#[tokio::test]
async fn authed_json_reports_non_channel_404_still_propagates() { … 34 line(s) … }

#[test]
fn flatten_authed_error_maps_unauthorized_to_session_expired_sentinel() { … 26 line(s) … }

#[test]
fn flatten_authed_error_preserves_non_unauthorized_chain() { … 14 line(s) … }

#[test]
fn flatten_authed_error_does_not_swallow_message_not_found() { … 16 line(s) … }

#[tokio::test]
async fn authed_json_403_is_not_demoted_to_unauthorized() { … 27 line(s) … }

#[tokio::test]
async fn authed_json_404_outside_messages_path_still_reports() { … 26 line(s) … }

// ── parse_message_path unit tests (TAURI-R7 regression guard) ───────────────

#[test]
fn parse_message_path_canonical_form() { … 6 line(s) … }

#[test]
fn parse_message_path_discord_provider() { … 6 line(s) … }

#[test]
fn parse_message_path_base_path_prefix() { … 8 line(s) … }

#[test]
fn parse_message_path_double_prefix() { … 6 line(s) … }

#[test]
fn parse_message_path_trailing_slash() { … 6 line(s) … }

#[test]
fn parse_message_path_percent_encoded_slug() { … 7 line(s) … }

#[test]
fn parse_message_path_non_message_path_returns_none() { … 7 line(s) … }

// ── authed_json defense-in-depth: PATCH 404 with base-path prefix ───────────

#[tokio::test]
async fn authed_json_patch_404_with_base_path_prefix_does_not_report() { … 45 line(s) … }

[compacted tool output — this is a PARTIAL view; the full original (26444 bytes) is available by calling tokenjuice_retrieve with token "95995e04f0b34dd67d20cf9ecfdfe4e3" (marker ⟦tj:95995e04f0b34dd67d20cf9ecfdfe4e3⟧)]