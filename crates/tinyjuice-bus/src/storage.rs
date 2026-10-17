//! What TinyJuice stores, as a schema the host composes.
//!
//! The CCR store's originals are blobs under `juice/ccr/<token>`. The module
//! runs for one tenant at a time, so they live at [`ScopeClass::Module`], and a
//! host derives the module's storage grant from this schema rather than
//! spelling the prefix out a second time.
//!
//! Before schema v2 the same blobs lived under `tinyjuice_ccr/<token>`. One
//! blob step moves them; the bytes and content type are untouched. The steps
//! are pure: no I/O happens until the host's registry runs them.

use tinystoragedrivers_core::Result;
use tinystoragedrivers_core::schema::{BlobOutput, Owner, Schema, ScopeClass, Step};

/// Where the CCR store keeps originals, `juice/ccr/<token>` (no trailing slash).
pub const CCR_PREFIX: &str = "juice/ccr";

/// Where schema v1 kept the originals, `tinyjuice_ccr/<token>`.
const LEGACY_PREFIX: &str = "tinyjuice_ccr/";

/// The schema owner, and the first path segment of every blob key.
const OWNER: &str = "juice";

/// Everything TinyJuice stores, for the host's `Registry::compose`.
///
/// # Panics
///
/// Never in practice: the owner name is a constant that satisfies the owner
/// grammar, which the tests pin.
#[must_use]
pub fn storage_schema() -> Schema {
    let prefix = format!("{CCR_PREFIX}/");
    Schema::new(Owner::new(OWNER).expect("`juice` is a valid schema owner"))
        .blobs(&prefix, ScopeClass::Module)
        .step(ccr_prefix_step())
}

/// v1 to v2: `tinyjuice_ccr/<token>` becomes `juice/ccr/<token>`.
fn ccr_prefix_step() -> Step {
    Step::blobs("ccr_prefix_v2", "juice/ccr/", 1, 2, move_ccr_blob).source_prefix(LEGACY_PREFIX)
}

/// One blob of the v1 prefix, re-keyed. Bytes and content type are untouched.
fn move_ccr_blob(
    key: String,
    content_type: Option<String>,
    bytes: Vec<u8>,
) -> Result<Option<BlobOutput>> {
    // The runner lists by the legacy prefix, so every key carries it; a key
    // that somehow does not has nothing to move.
    let Some(rest) = key.strip_prefix(LEGACY_PREFIX) else {
        return Ok(None);
    };
    Ok(Some((format!("{CCR_PREFIX}/{rest}"), content_type, bytes)))
}

#[cfg(test)]
#[path = "storage_tests.rs"]
mod tests;
