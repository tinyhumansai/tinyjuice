//! The `TinyJuice` module's bus identity and member names.

/// The well-known interface name the module claims on the bus.
pub const BUS_NAME: &str = "ai.tinyhumans.tinyjuice.Compression";

/// The object path the module serves its interface at.
pub const OBJECT_PATH: &str = "/ai/tinyhumans/tinyjuice/Compression";

/// The name a host claims to serve [`ml_host`] back to the module.
///
/// This direction is the unusual one: the ML plain-text compressor is the
/// host's, not the module's, so the module calls *out* to it. A host that does
/// not serve this name is not broken — the module falls back to a compressor
/// that needs no ML runtime.
pub const ML_HOST_NAME: &str = "ai.tinyhumans.tinyjuice.MlHost";

/// The object path a host serves [`ML_HOST_NAME`] at.
pub const ML_HOST_PATH: &str = "/ai/tinyhumans/tinyjuice/MlHost";

/// One constant per member of [`BUS_NAME`].
pub mod methods {
    /// Installs the host's configuration. Called once, before anything else.
    pub const INSTALL: &str = "Install";
    /// Reports the content kind the router would detect.
    pub const DETECT: &str = "Detect";
    /// Compresses content through the router.
    pub const COMPRESS: &str = "Compress";
    /// Compacts a tool result, reporting token counts.
    pub const COMPACT: &str = "Compact";
    /// Reads back an original the module offloaded.
    pub const RETRIEVE: &str = "Retrieve";
    /// Reports what the cache is holding.
    pub const CACHE_STATS: &str = "CacheStats";
}

/// One constant per member of [`ML_HOST_NAME`].
pub mod ml_host {
    /// Compresses plain text with the host's ML compressor, or declines.
    pub const COMPRESS: &str = "Compress";
}

/// Every member of [`BUS_NAME`], in the interface's declaration order.
pub const METHODS: &[&str] = &[
    methods::INSTALL,
    methods::DETECT,
    methods::COMPRESS,
    methods::COMPACT,
    methods::RETRIEVE,
    methods::CACHE_STATS,
];
