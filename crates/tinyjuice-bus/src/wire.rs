//! The request and response envelopes that exist only on the wire.
//!
//! [`types`](crate::types) holds values the `tinyjuice` library also uses.
//! These do not exist in it: they are the shapes the interface wraps those
//! values in, and they lived as private structs inside the module adapter,
//! where a host had no way to reach them and re-declared them instead.

use serde::{Deserialize, Serialize};

use crate::types::CompressOptions;

/// The one-shot configuration a host installs before its first compression.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InstallRequest {
    /// Router and compressor knobs, built by the host from its own config.
    pub options: CompressOptions,
    /// Ceiling on how many originals the CCR cache retains.
    pub max_cache_entries: usize,
    /// Ceiling on the bytes those originals may occupy.
    pub max_cache_bytes: usize,
    /// How long a retrievable original stays retrievable. `None` keeps the
    /// module's own default.
    pub ccr_ttl_secs: Option<u64>,
    /// Where the disk tier writes, when the host wants one. `None` keeps the
    /// cache in memory only.
    pub disk_tier_root: Option<String>,
}

/// What `Compact` answers with.
///
/// Distinct from [`CompressedOutput`](crate::types::CompressedOutput) because
/// the compaction path reports token counts the compression path does not, and
/// flattens `content_kind` and `compressor` to their string spellings.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CompactResponse {
    /// The compacted text, with any retrieval footer already appended.
    pub text: String,
    /// Byte length of the input.
    pub original_bytes: usize,
    /// Byte length of `text`.
    pub compacted_bytes: usize,
    /// Which rule fired, or the empty string when none did.
    pub rule_id: String,
    /// Whether anything actually changed.
    pub applied: bool,
    /// The detected content kind, as its wire spelling.
    pub content_kind: String,
    /// The compressor that produced `text`, as its wire spelling.
    pub compressor: String,
    /// Estimated tokens in the input.
    pub original_tokens: u64,
    /// Estimated tokens in `text`.
    pub compacted_tokens: u64,
}

/// What a [`RetrieveRange`] is measured in.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum RangeUnit {
    /// Byte offsets into the stored original.
    Bytes,
    /// Zero-based line numbers.
    Lines,
}

/// A half-open slice of a stored original.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RetrieveRange {
    /// Inclusive start, in [`unit`](Self::unit)s.
    pub start: usize,
    /// Exclusive end, in [`unit`](Self::unit)s.
    pub end: usize,
    /// What `start` and `end` count.
    pub unit: RangeUnit,
}

/// What the module is currently holding.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CacheStats {
    /// Number of retained originals.
    pub entries: usize,
    /// Bytes those originals occupy.
    pub bytes: usize,
}
