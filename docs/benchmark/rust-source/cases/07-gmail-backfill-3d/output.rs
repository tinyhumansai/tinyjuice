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
//! ```sh
//! cargo run --bin gmail-backfill-3d
//! cargo run --bin gmail-backfill-3d -- --days 7
//! cargo run --bin gmail-backfill-3d -- --days 14 --page-size 100
//! cargo run --bin gmail-backfill-3d -- --skip-drain
//! cargo run --bin gmail-backfill-3d -- --skip-verify
//! cargo run --bin gmail-backfill-3d -- --wipe
//! ```
//!
//! Set `RUST_LOG=info` (or `debug`) for detailed output.

use anyhow::{Context, Result};
use clap::Parser;
use serde_json::{json, Value};

use openhuman_core::openhuman::composio::client::{
    create_composio_client, direct_execute, ComposioClientKind,
};
use openhuman_core::openhuman::composio::providers::gmail::ingest::ingest_page_into_memory_tree;
use openhuman_core::openhuman::composio::providers::registry::{
    get_provider, init_default_providers,
};
use openhuman_core::openhuman::config::Config;
use openhuman_core::openhuman::memory_queue::drain_until_idle;
use openhuman_core::openhuman::memory_store::chunks::store::{
    get_chunk_content_pointers, list_chunks, list_summaries_with_content_path, ListChunksQuery,
};
use openhuman_core::openhuman::memory_store::content::read::{
    verify_chunk_file, verify_summary_file, VerifyResult,
};

#[derive(Parser, Debug)]
#[command(
    name = "gmail-backfill-3d",
    about = "Backfill last N days of Gmail into the memory-tree content store (.md files + SQLite)."
)]
struct Cli {
    /// Lookback window in days. Default 3.
    #[arg(long, default_value_t = 3)]
    days: u32,

    /// Page size per `GMAIL_FETCH_EMAILS` call (1–500).
    #[arg(long, default_value_t = 50)]
    page_size: u32,

    /// Cap on pages we will request. Guards against runaway pagination.
    #[arg(long, default_value_t = 40)]
    max_pages: u32,

    /// Include SPAM and TRASH messages in the fetch.
    #[arg(long, default_value_t = false)]
    include_spam_trash: bool,

    /// Extra Gmail search query AND-ed with the default scope.
    #[arg(long)]
    query: Option<String>,

    /// Skip draining the async worker pool after ingest (useful for quick
    /// smoke-test of file writes only).
    #[arg(long, default_value_t = false)]
    skip_drain: bool,

    /// Skip the post-drain integrity check (SHA-256 file verification).
    #[arg(long, default_value_t = false)]
    skip_verify: bool,

    /// Override the owner string embedded in chunk metadata. Defaults to
    /// `"gmail-backfill"`.
    #[arg(long)]
    owner: Option<String>,

    /// Wipe `chunks.db` (+ wal/shm) AND `<content_root>/` before running.
    /// Useful after a chunker change that invalidates existing chunk IDs.
    #[arg(long, default_value_t = false)]
    wipe: bool,
}

#[tokio::main]
async fn main() -> Result<()> { … 226 line(s) … ⟦tj:f94621db810d452333e63e57c2d5d61e⟧ }

/// Wipe `<workspace>/memory_tree/chunks.db` (+ wal/shm) and
/// `<content_root>/` so the bin can re-run cleanly after a chunker
/// change that invalidates existing chunk IDs.
///
/// Logs each removed artifact at info; missing files are not an error.
fn wipe_memory_tree_state(config: &Config) -> Result<()> { … 18 line(s) … ⟦tj:25c2bd02a94650334fb2f36e5686e26b⟧ }

/// Read all chunks from SQLite and verify on-disk SHA-256 matches `content_sha256`.
///
/// Returns `(verified, mismatched, no_pointer, missing_file)`.
fn verify_all_chunk_files(config: &Config) -> Result<(usize, usize, usize, usize)> { … 65 line(s) … ⟦tj:1f4abc58cd26c839810bda5aba1f92c9⟧ }

/// Read all summary rows with a non-NULL `content_path` from SQLite and verify
/// the on-disk SHA-256 matches `content_sha256`.
///
/// Returns `(verified, mismatched, no_pointer, missing_file)`.
fn verify_all_summary_files(config: &Config) -> Result<(usize, usize, usize, usize)> { … 59 line(s) … ⟦tj:1e479fb2d47d722ae45efb9bfffcbd03⟧ }

/// Extract the `messages` array and `nextPageToken` from a Composio response.
fn extract_envelope(data: &Value) -> (Vec<Value>, Option<String>) { … 14 line(s) … ⟦tj:520d0f0d34eb988595c31d11c118cb02⟧ }
[collapsed bodies are individually retrievable: call tokenjuice_retrieve with the token inside a placeholder to expand just that body]

[compacted tool output — this is a PARTIAL view; the full original (17440 bytes) is available by calling tokenjuice_retrieve with token "de327d088c33663aff65f51429a1108e" (marker ⟦tj:de327d088c33663aff65f51429a1108e⟧)]