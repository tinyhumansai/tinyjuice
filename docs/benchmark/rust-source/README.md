# Rust Source

Real OpenHuman Rust files. The source compressor keeps imports, signatures, and top-level structure while collapsing large bodies when useful.

Each row links to the full raw input and the exact compacted output used by the benchmark. Percentages are **token reduction: higher is better**; 0% means pass-through. `Algorithm` is the compressor-only reduction. `Pass 1` disables CCR (compressed with omission markers, no recovery footer). `Pass 2` is the final model-facing result with CCR enabled — it reads marginally *lower* than Pass 1 only because the recovery footer adds a few dozen bytes to the output.

## Cases

Every case links to the raw input, the exact model-facing output (with the CCR recovery footer), and a unified diff between the two.

| Case | Input | Output (after CCR) | Diff | Original | Algorithm | Pass 1: no CCR | Pass 2: with CCR | Avg latency | CCR |
| --- | --- | --- | --- | ---: | ---: | ---: | ---: | ---: | --- |
| `05-rest-tests` | [input](cases/05-rest-tests/input.rs) | [output](cases/05-rest-tests/output.rs) | [diff](cases/05-rest-tests/compression.diff) | 26.4 KB | 75.9% | 82.6% | 75.1% | 1.199 ms | true |
| `08-harness-subagent-audit` | [input](cases/08-harness-subagent-audit/input.rs) | [output](cases/08-harness-subagent-audit/output.rs) | [diff](cases/08-harness-subagent-audit/compression.diff) | 34.7 KB | 74.5% | 76.8% | 73.9% | 1.352 ms | true |
| `07-gmail-backfill-3d` | [input](cases/07-gmail-backfill-3d/input.rs) | [output](cases/07-gmail-backfill-3d/output.rs) | [diff](cases/07-gmail-backfill-3d/compression.diff) | 17.4 KB | 71.1% | 73.0% | 69.8% | 0.700 ms | true |
| `09-inference-probe` | [input](cases/09-inference-probe/input.rs) | [output](cases/09-inference-probe/output.rs) | [diff](cases/09-inference-probe/compression.diff) | 8.4 KB | 64.5% | 67.5% | 61.9% | 0.332 ms | true |
| `04-rest` | [input](cases/04-rest/input.rs) | [output](cases/04-rest/output.rs) | [diff](cases/04-rest/compression.diff) | 48.1 KB | 61.5% | 64.3% | 61.1% | 1.892 ms | true |
| `10-memory-tree-init-smoke` | [input](cases/10-memory-tree-init-smoke/input.rs) | [output](cases/10-memory-tree-init-smoke/output.rs) | [diff](cases/10-memory-tree-init-smoke/compression.diff) | 3.4 KB | 58.1% | 63.3% | 51.7% | 0.148 ms | true |
| `01-config` | [input](cases/01-config/input.rs) | [output](cases/01-config/output.rs) | [diff](cases/01-config/compression.diff) | 53.9 KB | 51.5% | 56.3% | 51.0% | 1.663 ms | true |
| `02-jwt` | [input](cases/02-jwt/input.rs) | [output](cases/02-jwt/output.rs) | [diff](cases/02-jwt/compression.diff) | 4.5 KB | 42.9% | 52.7% | 38.0% | 0.193 ms | true |
| `06-socket` | [input](cases/06-socket/input.rs) | [output](cases/06-socket/output.rs) | [diff](cases/06-socket/compression.diff) | 2.0 KB | 0.0% | 0.0% | 0.0% | 0.000 ms | n/a |
| `03-socket` | [input](cases/03-socket/input.rs) | [output](cases/03-socket/output.rs) | [diff](cases/03-socket/compression.diff) | 1.9 KB | 0.0% | 0.0% | 0.0% | 0.000 ms | n/a |

## What TinyJuice Is Doing

The code path keeps the navigation surface: imports, signatures, top-level items, and important comments. Large function bodies can be collapsed and recovered through CCR.

## Syntax-Aware Samples

### `05-rest-tests`

- [Full input](cases/05-rest-tests/input.rs)
- [Full output](cases/05-rest-tests/output.rs)
- [Input vs output diff](cases/05-rest-tests/compression.diff)

Input excerpt:

```rust
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
fn decodes_base64url_no_pad() {
    // A 32-byte key that, when base64url-encoded, contains both `-` and `_`.
    let raw = [
        0xff_u8, 0xfb, 0xef, 0x00, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xaa,
        0xbb, 0xcc, 0xdd, 0xee, 0xff, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a,
        0x0b, 0x0c, 0x0d,
    ];
    let url_key = URL_SAFE_NO_PAD.encode(raw);
    assert!(url_key.contains('-') || url_key.contains('_'));
    let decoded = key_bytes_from_string(&url_key).unwrap();
    assert_eq!(decoded, raw);
}

#[test]
fn decodes_standard_base64() {
    let raw = [0x41_u8; 32];
    let std_key = STANDARD.encode(raw);
    let decoded = key_bytes_from_string(&std_key).unwrap();
    assert_eq!(decoded, raw);
}

```

Output excerpt:

```rust
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

```

### `08-harness-subagent-audit`

- [Full input](cases/08-harness-subagent-audit/input.rs)
- [Full output](cases/08-harness-subagent-audit/output.rs)
- [Input vs output diff](cases/08-harness-subagent-audit/compression.diff)

Input excerpt:

```rust
//! Live harness audit for reusable async sub-agent delegation.
//!
//! This binary intentionally uses the user's real OpenHuman config and live
//! provider/backend credentials. It records only sanitized progress metadata:
//! tool names, task/session ids, statuses, character counts, and elapsed times.
//! It does not print prompts, tool arguments, assistant replies, transcripts,
//! credentials, or integration payloads.
//!
//! Typical usage:
//!
//! ` ` `sh
//! scripts/debug/harness-subagent-audit.sh --turns 2
//! ` ` `

use std::collections::BTreeSet;
use std::sync::{
    atomic::{AtomicBool, AtomicUsize, Ordering},
    Arc, Mutex,
};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use anyhow::{Context, Result};
use clap::Parser;
use openhuman_core::openhuman::agent::progress::AgentProgress;
use openhuman_core::openhuman::agent::Agent;
use openhuman_core::openhuman::agent_orchestration::harness_audit::{
    self, AuditSteerError, AuditSubagentSessionStore, DurableSubagentSession, DurableSubagentStatus,
};
use openhuman_core::openhuman::config::Config;
use serde::Serialize;
use tokio::sync::mpsc;

#[derive(Parser, Debug)]
#[command(name = "harness-subagent-audit")]
struct Args {
    /// Sub-agent archetype to request from the orchestrator.

```

Output excerpt:

```rust
//! Live harness audit for reusable async sub-agent delegation.
//!
//! This binary intentionally uses the user's real OpenHuman config and live
//! provider/backend credentials. It records only sanitized progress metadata:
//! tool names, task/session ids, statuses, character counts, and elapsed times.
//! It does not print prompts, tool arguments, assistant replies, transcripts,
//! credentials, or integration payloads.
//!
//! Typical usage:
//!
//! ` ` `sh
//! scripts/debug/harness-subagent-audit.sh --turns 2
//! ` ` `

use std::collections::BTreeSet;
use std::sync::{
    atomic::{AtomicBool, AtomicUsize, Ordering},
    Arc, Mutex,
};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use anyhow::{Context, Result};
use clap::Parser;
use openhuman_core::openhuman::agent::progress::AgentProgress;
use openhuman_core::openhuman::agent::Agent;
use openhuman_core::openhuman::agent_orchestration::harness_audit::{
    self, AuditSteerError, AuditSubagentSessionStore, DurableSubagentSession, DurableSubagentStatus,
};
use openhuman_core::openhuman::config::Config;
use serde::Serialize;
use tokio::sync::mpsc;

#[derive(Parser, Debug)]
#[command(name = "harness-subagent-audit")]
struct Args {
    /// Sub-agent archetype to request from the orchestrator.

```

### `07-gmail-backfill-3d`

- [Full input](cases/07-gmail-backfill-3d/input.rs)
- [Full output](cases/07-gmail-backfill-3d/output.rs)
- [Input vs output diff](cases/07-gmail-backfill-3d/compression.diff)

Input excerpt:

```rust
//! Backfill the last N days of Gmail into the memory-tree content store.
//!
//! Authenticates via Composio (JWT from `<workspace>/auth-profiles.json`),
//! fetches Gmail pages via `GMAIL_FETCH_EMAILS`, converts each thread into an
//! [`EmailThread`], ingests it through `ingest_page_into_memory_tree` (which
//! writes `.md` files via `content_store` and populates SQLite), then drains
//! the async worker pool until idle.
//!
//! After draining, the binary performs an integrity check: for every chunk
//! that has a `content_path` in SQLite, it verifies the on-disk SHA-256
//! matches the stored `content_sha256`.
//!
//! # Prerequisites
//!
//! - Signed-in openhuman session JWT in the same workspace the desktop app
//!   uses (stored at `<workspace>/auth-profiles.json`).
//! - Active Gmail connection on Composio for that user.
//!
//! # Usage
//!
//! ` ` `sh
//! cargo run --bin gmail-backfill-3d
//! cargo run --bin gmail-backfill-3d -- --days 7
//! cargo run --bin gmail-backfill-3d -- --days 14 --page-size 100
//! cargo run --bin gmail-backfill-3d -- --skip-drain
//! cargo run --bin gmail-backfill-3d -- --skip-verify
//! cargo run --bin gmail-backfill-3d -- --wipe
//! ` ` `
//!
//! Set `RUST_LOG=info` (or `debug`) for detailed output.

use anyhow::{Context, Result};
use clap::Parser;
use serde_json::{json, Value};

use openhuman_core::openhuman::composio::client::{

```

Output excerpt:

```rust
//! Backfill the last N days of Gmail into the memory-tree content store.
//!
//! Authenticates via Composio (JWT from `<workspace>/auth-profiles.json`),
//! fetches Gmail pages via `GMAIL_FETCH_EMAILS`, converts each thread into an
//! [`EmailThread`], ingests it through `ingest_page_into_memory_tree` (which
//! writes `.md` files via `content_store` and populates SQLite), then drains
//! the async worker pool until idle.
//!
//! After draining, the binary performs an integrity check: for every chunk
//! that has a `content_path` in SQLite, it verifies the on-disk SHA-256
//! matches the stored `content_sha256`.
//!
//! # Prerequisites
//!
//! - Signed-in openhuman session JWT in the same workspace the desktop app
//!   uses (stored at `<workspace>/auth-profiles.json`).
//! - Active Gmail connection on Composio for that user.
//!
//! # Usage
//!
//! ` ` `sh
//! cargo run --bin gmail-backfill-3d
//! cargo run --bin gmail-backfill-3d -- --days 7
//! cargo run --bin gmail-backfill-3d -- --days 14 --page-size 100
//! cargo run --bin gmail-backfill-3d -- --skip-drain
//! cargo run --bin gmail-backfill-3d -- --skip-verify
//! cargo run --bin gmail-backfill-3d -- --wipe
//! ` ` `
//!
//! Set `RUST_LOG=info` (or `debug`) for detailed output.

use anyhow::{Context, Result};
use clap::Parser;
use serde_json::{json, Value};

use openhuman_core::openhuman::composio::client::{

```

### `09-inference-probe`

- [Full input](cases/09-inference-probe/input.rs)
- [Full output](cases/09-inference-probe/output.rs)
- [Input vs output diff](cases/09-inference-probe/compression.diff)

Input excerpt:

```rust
//! Direct chat probe — exercise the full orchestrator harness end-to-end
//! with the live user config, run a single turn, and print whether the
//! harness actually emitted tool calls.
//!
//! Two modes:
//!
//! - `--mode harness` (default): build a real `Agent::from_config()`
//!   orchestrator and call `run_single("...")`. This is the production
//!   path — system prompt with tool catalog, Connected Integrations
//!   block, dispatcher selection, tool execution, all of it. Use this
//!   to verify whether tool calls fire for a given user prompt.
//!
//! - `--mode raw`: send a single hand-built request straight to the
//!   chat provider (no harness, no real tools). Useful to isolate
//!   "does the model itself follow P-format / native tool spec".
//!
//! # Usage
//!
//! ` ` `sh
//! # Drive a real orchestrator turn — needs BACKEND_URL set so the
//! # integrations client can fetch the user's Connected Integrations.
//! BACKEND_URL=https://staging-api.tinyhumans.ai \
//!   OPENHUMAN_APP_ENV=staging \
//!   RUST_LOG=info,openhuman_core::openhuman::agent=debug,openhuman_core::openhuman::inference=debug \
//!   cargo run --bin inference-probe -- \
//!     --mode harness --prompt "hey list my top 5 emails"
//!
//! # Raw provider call (no harness):
//! cargo run --bin inference-probe -- --mode raw --raw-mode pformat
//! ` ` `
use anyhow::{Context, Result};
use clap::Parser;
use openhuman_core::openhuman::agent::Agent;
use openhuman_core::openhuman::config::Config;
use openhuman_core::openhuman::inference::provider::create_chat_provider;
use openhuman_core::openhuman::inference::provider::traits::{ChatMessage, ChatRequest};

```

Output excerpt:

```rust
//! Direct chat probe — exercise the full orchestrator harness end-to-end
//! with the live user config, run a single turn, and print whether the
//! harness actually emitted tool calls.
//!
//! Two modes:
//!
//! - `--mode harness` (default): build a real `Agent::from_config()`
//!   orchestrator and call `run_single("...")`. This is the production
//!   path — system prompt with tool catalog, Connected Integrations
//!   block, dispatcher selection, tool execution, all of it. Use this
//!   to verify whether tool calls fire for a given user prompt.
//!
//! - `--mode raw`: send a single hand-built request straight to the
//!   chat provider (no harness, no real tools). Useful to isolate
//!   "does the model itself follow P-format / native tool spec".
//!
//! # Usage
//!
//! ` ` `sh
//! # Drive a real orchestrator turn — needs BACKEND_URL set so the
//! # integrations client can fetch the user's Connected Integrations.
//! BACKEND_URL=https://staging-api.tinyhumans.ai \
//!   OPENHUMAN_APP_ENV=staging \
//!   RUST_LOG=info,openhuman_core::openhuman::agent=debug,openhuman_core::openhuman::inference=debug \
//!   cargo run --bin inference-probe -- \
//!     --mode harness --prompt "hey list my top 5 emails"
//!
//! # Raw provider call (no harness):
//! cargo run --bin inference-probe -- --mode raw --raw-mode pformat
//! ` ` `
use anyhow::{Context, Result};
use clap::Parser;
use openhuman_core::openhuman::agent::Agent;
use openhuman_core::openhuman::config::Config;
use openhuman_core::openhuman::inference::provider::create_chat_provider;
use openhuman_core::openhuman::inference::provider::traits::{ChatMessage, ChatRequest};

```

### `04-rest`

- [Full input](cases/04-rest/input.rs)
- [Full output](cases/04-rest/output.rs)
- [Input vs output diff](cases/04-rest/compression.diff)

Input excerpt:

```rust
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

```

Output excerpt:

```rust
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

```

### `10-memory-tree-init-smoke`

- [Full input](cases/10-memory-tree-init-smoke/input.rs)
- [Full output](cases/10-memory-tree-init-smoke/output.rs)
- [Input vs output diff](cases/10-memory-tree-init-smoke/compression.diff)

Input excerpt:

```rust
//! Manual stress smoke for the memory_tree schema-init race fix.
//!
//! Spins N concurrent threads racing into `memory::tree::store::with_connection`
//! against a shared workspace. Pre-fix (without the mutex-gated init guard),
//! cold-start runs would surface SQLite codes 14 (CANTOPEN), 1546
//! (IOERR_TRUNCATE), or 4874 (IOERR_SHMMAP) on some threads. Post-fix,
//! all N threads must return Ok.
//!
//! # Usage
//!
//! ` ` `sh
//! # Fresh workspace (forces cold-start path)
//! rm -rf /tmp/mt-smoke
//! OPENHUMAN_WORKSPACE=/tmp/mt-smoke \
//!   cargo run --bin memory-tree-init-smoke -- 32
//!
//! # Re-run against warm DB (should also be Ok; exercises fast path)
//! OPENHUMAN_WORKSPACE=/tmp/mt-smoke \
//!   cargo run --bin memory-tree-init-smoke -- 32
//! ` ` `
//!
//! Arg is thread count (default 16, must be > 0). Higher = more contention.
//! Use `RUST_LOG=debug` to see per-worker results.
//!
//! Exit code: 0 if all threads Ok, 1 if any failed.

use std::path::PathBuf;
use std::process::ExitCode;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

use openhuman_core::openhuman::config::Config;
use openhuman_core::openhuman::memory_store::chunks::store::with_connection;

fn main() -> ExitCode {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info"))

```

Output excerpt:

```rust
//! Manual stress smoke for the memory_tree schema-init race fix.
//!
//! Spins N concurrent threads racing into `memory::tree::store::with_connection`
//! against a shared workspace. Pre-fix (without the mutex-gated init guard),
//! cold-start runs would surface SQLite codes 14 (CANTOPEN), 1546
//! (IOERR_TRUNCATE), or 4874 (IOERR_SHMMAP) on some threads. Post-fix,
//! all N threads must return Ok.
//!
//! # Usage
//!
//! ` ` `sh
//! # Fresh workspace (forces cold-start path)
//! rm -rf /tmp/mt-smoke
//! OPENHUMAN_WORKSPACE=/tmp/mt-smoke \
//!   cargo run --bin memory-tree-init-smoke -- 32
//!
//! # Re-run against warm DB (should also be Ok; exercises fast path)
//! OPENHUMAN_WORKSPACE=/tmp/mt-smoke \
//!   cargo run --bin memory-tree-init-smoke -- 32
//! ` ` `
//!
//! Arg is thread count (default 16, must be > 0). Higher = more contention.
//! Use `RUST_LOG=debug` to see per-worker results.
//!
//! Exit code: 0 if all threads Ok, 1 if any failed.

use std::path::PathBuf;
use std::process::ExitCode;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

use openhuman_core::openhuman::config::Config;
use openhuman_core::openhuman::memory_store::chunks::store::with_connection;

fn main() -> ExitCode { … 76 line(s) … ⟦tj:0bc2aa582a12a896dbb668373d73d51b⟧ }
[omitted blocks are individually retrievable: call tinyjuice_retrieve with the token inside an omission marker to expand just that block]

```

### `01-config`

- [Full input](cases/01-config/input.rs)
- [Full output](cases/01-config/output.rs)
- [Input vs output diff](cases/01-config/compression.diff)

Input excerpt:

```rust
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
//! ` ` `text
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
//! ` ` `
//!
//! ## Resolution order (both families)
//!
//! 1. Non-empty `config.api_url` / `config.inference_url` (user override).
//! 2. `BACKEND_URL` / `VITE_BACKEND_URL` runtime env (each checked

```

Output excerpt:

```rust
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
//! ` ` `text
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
//! ` ` `
//!
//! ## Resolution order (both families)
//!
//! 1. Non-empty `config.api_url` / `config.inference_url` (user override).
//! 2. `BACKEND_URL` / `VITE_BACKEND_URL` runtime env (each checked

```

### `02-jwt`

- [Full input](cases/02-jwt/input.rs)
- [Full output](cases/02-jwt/output.rs)
- [Input vs output diff](cases/02-jwt/compression.diff)

Input excerpt:

```rust
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
pub fn decode_jwt_payload(token: &str) -> Option<serde_json::Value> {
    // JWT = header.payload.signature (base64url, no padding). Only the payload
    // segment is needed.
    let payload_b64 = token.trim().split('.').nth(1)?;
    let bytes = base64::engine::general_purpose::URL_SAFE_NO_PAD
        .decode(payload_b64)
        .or_else(|_| base64::engine::general_purpose::URL_SAFE.decode(payload_b64))
        .ok()?;
    serde_json::from_slice(&bytes).ok()
}

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

```

Output excerpt:

```rust
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
pub fn decode_jwt_payload(token: &str) -> Option<serde_json::Value> { … 10 line(s) … ⟦tj:b821e28022db20530eaf2b6e8695b28c⟧ }

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
pub fn decode_jwt_exp(token: &str) -> Option<DateTime<Utc>> { … 8 line(s) … ⟦tj:902053cf712b0ec0319a8b4193f0b577⟧ }

#[cfg(test)]
mod tests {
    use super::*;


```

### `06-socket`

- [Full input](cases/06-socket/input.rs)
- [Full output](cases/06-socket/output.rs)
- [Input vs output diff](cases/06-socket/compression.diff)

Input excerpt:

```rust
//! Socket.IO (Engine.IO v4) WebSocket URL for the TinyHumans backend.

use url::Url;

/// Build a Socket.IO WebSocket URL from an HTTP(S) API base (e.g. `https://api.tinyhumans.ai`).
pub fn websocket_url(http_or_https_base: &str) -> String {
    let Ok(mut url) = Url::parse(http_or_https_base) else {
        return http_or_https_base.to_string();
    };

    let scheme = match url.scheme() {
        "https" => "wss",
        "http" => "ws",
        other => other,
    }
    .to_string();

    let _ = url.set_scheme(&scheme);

    // Ensure path ends with /socket.io/ and includes required query params
    url.set_path(&format!("{}/socket.io/", url.path().trim_end_matches('/')));
    url.set_query(Some("EIO=4&transport=websocket"));

    url.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn converts_https_to_wss() {
        let url = websocket_url("https://api.tinyhumans.ai");
        assert_eq!(
            url,
            "wss://api.tinyhumans.ai/socket.io/?EIO=4&transport=websocket"

```

Output excerpt:

```rust
//! Socket.IO (Engine.IO v4) WebSocket URL for the TinyHumans backend.

use url::Url;

/// Build a Socket.IO WebSocket URL from an HTTP(S) API base (e.g. `https://api.tinyhumans.ai`).
pub fn websocket_url(http_or_https_base: &str) -> String {
    let Ok(mut url) = Url::parse(http_or_https_base) else {
        return http_or_https_base.to_string();
    };

    let scheme = match url.scheme() {
        "https" => "wss",
        "http" => "ws",
        other => other,
    }
    .to_string();

    let _ = url.set_scheme(&scheme);

    // Ensure path ends with /socket.io/ and includes required query params
    url.set_path(&format!("{}/socket.io/", url.path().trim_end_matches('/')));
    url.set_query(Some("EIO=4&transport=websocket"));

    url.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn converts_https_to_wss() {
        let url = websocket_url("https://api.tinyhumans.ai");
        assert_eq!(
            url,
            "wss://api.tinyhumans.ai/socket.io/?EIO=4&transport=websocket"

```

### `03-socket`

- [Full input](cases/03-socket/input.rs)
- [Full output](cases/03-socket/output.rs)
- [Input vs output diff](cases/03-socket/compression.diff)

Input excerpt:

```rust
use serde::{Deserialize, Serialize};

/// Socket connection status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
#[derive(Default)]
pub enum ConnectionStatus {
    #[default]
    Disconnected,
    Connecting,
    Connected,
    Reconnecting,
    Error,
}

/// Socket connection state emitted to frontend
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SocketState {
    pub status: ConnectionStatus,
    pub socket_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

impl Default for SocketState {
    fn default() -> Self {
        Self {
            status: ConnectionStatus::Disconnected,
            socket_id: None,
            error: None,
        }
    }
}

/// Generic socket message wrapper
#[allow(dead_code)]

```

Output excerpt:

```rust
use serde::{Deserialize, Serialize};

/// Socket connection status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
#[derive(Default)]
pub enum ConnectionStatus {
    #[default]
    Disconnected,
    Connecting,
    Connected,
    Reconnecting,
    Error,
}

/// Socket connection state emitted to frontend
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SocketState {
    pub status: ConnectionStatus,
    pub socket_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

impl Default for SocketState {
    fn default() -> Self {
        Self {
            status: ConnectionStatus::Disconnected,
            socket_id: None,
            error: None,
        }
    }
}

/// Generic socket message wrapper
#[allow(dead_code)]

```

