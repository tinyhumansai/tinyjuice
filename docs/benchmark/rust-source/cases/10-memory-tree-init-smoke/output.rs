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
//! ```sh
//! # Fresh workspace (forces cold-start path)
//! rm -rf /tmp/mt-smoke
//! OPENHUMAN_WORKSPACE=/tmp/mt-smoke \
//!   cargo run --bin memory-tree-init-smoke -- 32
//!
//! # Re-run against warm DB (should also be Ok; exercises fast path)
//! OPENHUMAN_WORKSPACE=/tmp/mt-smoke \
//!   cargo run --bin memory-tree-init-smoke -- 32
//! ```
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
[collapsed bodies are individually retrievable: call tokenjuice_retrieve with the token inside a placeholder to expand just that body]

[compacted tool output — this is a PARTIAL view; the full original (3396 bytes) is available by calling tokenjuice_retrieve with token "3d481ca113a31ec0f373a8b46b8c8b56" (marker ⟦tj:3d481ca113a31ec0f373a8b46b8c8b56⟧)]