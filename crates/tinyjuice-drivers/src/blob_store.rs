//! A [`CcrStore`] on the blob port.

use std::sync::Arc;
use std::time::Duration;

use tinyjuice::cache::store::{CcrPutResult, CcrStore, RangeUnit, range_from_original, short_hash};
use tinystoragedrivers_core::{BlobStore, Blocking, Capability, StorageError};

pub use tinyjuice_bus::storage::CCR_PREFIX;

const CONTENT_TYPE: &str = "text/plain; charset=utf-8";

/// CCR originals as expiring blobs, keyed by their content token.
///
/// Re-offloading identical content rewrites the same blob, which restarts its
/// lifetime (the same refresh the file tier gave by touching mtime). Tokens are
/// validated before they form a key, so an agent-supplied token can never
/// address anything outside [`CCR_PREFIX`].
#[derive(Clone)]
pub struct BlobCcrStore {
    blobs: Arc<dyn BlobStore>,
    bridge: Blocking,
    ttl: Option<Duration>,
}

impl std::fmt::Debug for BlobCcrStore {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BlobCcrStore")
            .field("ttl", &self.ttl)
            .finish_non_exhaustive()
    }
}

impl BlobCcrStore {
    /// A store on `blobs`. `ttl` is each original's lifetime from its write;
    /// `None` keeps originals until something deletes them.
    ///
    /// # Errors
    ///
    /// The bridge cannot start, or a `ttl` is set and the handle cannot expire
    /// blobs ([`Capability::BlobTtl`]).
    pub fn new(blobs: Arc<dyn BlobStore>, ttl: Option<Duration>) -> Result<Self, StorageError> {
        if ttl.is_some() && !blobs.capabilities().contains(Capability::BlobTtl) {
            return Err(StorageError::unsupported(
                Capability::BlobTtl,
                "the CCR store needs blob expiry for its ttl",
            ));
        }
        let bridge = Blocking::new()?;
        log::debug!("[tinyjuice_drivers] ccr blob store ready prefix={CCR_PREFIX} ttl={ttl:?}");
        Ok(Self { blobs, bridge, ttl })
    }

    fn key(token: &str) -> Option<String> {
        // `short_hash` output is the only shape ever stored.
        (token.len() == short_hash("").len() && token.bytes().all(|b| b.is_ascii_hexdigit()))
            .then(|| format!("{CCR_PREFIX}/{token}"))
    }

    /// Remove an original; reports whether it existed.
    pub fn delete(&self, token: &str) -> bool {
        let Some(key) = Self::key(token) else {
            return false;
        };
        let blobs = Arc::clone(&self.blobs);
        match self.bridge.run(async move { blobs.delete(&key).await }) {
            Ok(Ok(existed)) => existed,
            Ok(Err(e)) | Err(e) => {
                log::debug!("[tinyjuice_drivers] ccr delete failed: {}", e.kind());
                false
            }
        }
    }

    fn read(&self, token: &str) -> Option<String> {
        let key = Self::key(token)?;
        let blobs = Arc::clone(&self.blobs);
        match self.bridge.run(async move { blobs.get(&key).await }) {
            Ok(Ok(blob)) => String::from_utf8(blob?.bytes).ok(),
            Ok(Err(e)) | Err(e) => {
                log::debug!("[tinyjuice_drivers] ccr read failed: {}", e.kind());
                None
            }
        }
    }
}

impl CcrStore for BlobCcrStore {
    fn put(&self, content: &str) -> CcrPutResult {
        let token = short_hash(content);
        let Some(key) = Self::key(&token) else {
            return CcrPutResult::new(token, false);
        };
        if self.ttl.is_some_and(|ttl| ttl.is_zero()) {
            // A zero lifetime is expired on arrival; never advertise it.
            return CcrPutResult::new(token, false);
        }
        let blobs = Arc::clone(&self.blobs);
        let bytes = content.as_bytes().to_vec();
        let ttl = self.ttl;
        let result = self.bridge.run(async move {
            match ttl {
                Some(ttl) => {
                    blobs
                        .put_expiring(&key, bytes, Some(CONTENT_TYPE), ttl)
                        .await
                }
                None => blobs.put(&key, bytes, Some(CONTENT_TYPE)).await,
            }
        });
        let retained = match result {
            Ok(Ok(_)) => true,
            Ok(Err(e)) | Err(e) => {
                log::debug!("[tinyjuice_drivers] ccr write failed: {}", e.kind());
                false
            }
        };
        CcrPutResult::new(token, retained)
    }

    fn get(&self, token: &str) -> Option<String> {
        self.read(token)
    }

    fn get_range(&self, token: &str, start: usize, end: usize, unit: RangeUnit) -> Option<String> {
        let original = self.read(token)?;
        range_from_original(&original, start, end, unit)
    }
}

#[cfg(test)]
#[path = "blob_store_tests.rs"]
mod tests;
