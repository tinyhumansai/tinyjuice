//! CCR (Compress-Cache-Retrieve) — original storage + retrieval markers.

pub mod marker;
pub mod store;

pub use marker::{
    LEGACY_RETRIEVE_TOOL_NAME, NEVER_COMPACT_TOOLS, RECOVERY_TOOL_NAMES, RETRIEVE_TOOL_NAME,
    format_marker, is_recovery_tool, parse_markers, recovery_footer,
};
pub use store::{
    CcrPutResult, CcrStore, DEFAULT_DISK_MAX_BYTES, DEFAULT_MAX_BYTES, DEFAULT_MAX_ENTRIES,
    GcStats, GlobalCcrStore, MemoryCcrStore, RangeUnit, configure, configure_disk_cap,
    disable_disk_tier, enable_disk_tier, gc_disk_dir, gc_disk_tier, offload, offload_checked,
    offload_checked_with_hash, retrieve, retrieve_range, short_hash, stats,
};
