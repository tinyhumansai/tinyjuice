//! Session JWT load and `Authorization` helpers for the TinyHumans API.

use base64::Engine;
use chrono::{DateTime, Utc};

pub use crate::openhuman::credentials::session_support::get_session_token;
pub use crate::openhuman::credentials::{APP_SESSION_PROVIDER, DEFAULT_AUTH_PROFILE_NAME};

/// Value for `Authorization: Bearer …` (matches backend expectations).
pub fn bearer_authorization_value(token: &str) -> String {
    format!("Bearer {}", token.trim())
}

/// Best-effort decode of a JWT payload without verifying the signature.
pub fn decode_jwt_payload(token: &str) -> Option<serde_json::Value> { … 10 line(s) … }

/// Best-effort decode of a JWT's `exp` (expiry) claim into a UTC timestamp.
///
/// The backend app-session token is a JWT but is stored bare — the client
/// historically recorded `expires_at: None` and so blindly sent requests with a
/// token it could have known was dead, generating doomed 401s (Sentry
/// TAURI-RUST-8WY `/teams/me/usage`, 8WZ `/payments/stripe/currentPlan`; #3297).
/// Decoding `exp` at store time lets `require_live_session_token` reject an
/// expired token locally instead of round-tripping to a guaranteed 401.
///
/// This does NOT verify the signature — the client only needs to *read* `exp`;
/// the backend stays the authority on validity (a token revoked before its `exp`
/// still 401s, caught by the `flatten_authed_error` net). Returns `None` for any
/// non-JWT / malformed / `exp`-less token, in which case expiry tracking
/// degrades to the previous behaviour (no local precheck).
pub fn decode_jwt_exp(token: &str) -> Option<DateTime<Utc>> { … 8 line(s) … }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bearer_authorization_value() { … 22 line(s) … }

    fn jwt_with_payload(payload_json: &str) -> String { … 5 line(s) … }

    #[test]
    fn decode_jwt_exp_reads_integer_exp() { … 7 line(s) … }

    #[test]
    fn decode_jwt_exp_reads_float_exp() { … 7 line(s) … }

    #[test]
    fn decode_jwt_exp_none_when_exp_absent() { … 4 line(s) … }

    #[test]
    fn decode_jwt_exp_none_for_non_jwt_or_garbage() { … 7 line(s) … }
}

[compacted tool output — this is a PARTIAL view; the full original (4490 bytes) is available by calling tokenjuice_retrieve with token "72a9fa15745dfefa792a0adf04d13794" (marker ⟦tj:72a9fa15745dfefa792a0adf04d13794⟧)]