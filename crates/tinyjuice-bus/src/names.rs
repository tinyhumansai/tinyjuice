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
    /// [`COMPACT`] with a [`CompactRequest`](crate::wire::CompactRequest):
    /// the tool's arguments, the caller's summary focus, and the context token
    /// the module hands back to the host when it asks for a summary.
    pub const COMPACT_WITH: &str = "CompactWith";
    /// Reads back an original the module offloaded.
    pub const RETRIEVE: &str = "Retrieve";
    /// Legacy JSON REPL query; retains its two positional arguments.
    pub const REPL: &str = "Repl";
    /// Typed REPL query over module storage or supplied content.
    pub const QUERY: &str = "Query";
    /// Convert HTML to Markdown inside the module.
    pub const EXTRACT_HTML: &str = "ExtractHtml";
    /// Reports what the cache is holding.
    pub const CACHE_STATS: &str = "CacheStats";
}

/// One constant per member of [`ML_HOST_NAME`].
pub mod ml_host {
    /// Compresses plain text with the host's ML compressor, or declines.
    pub const COMPRESS: &str = "Compress";
    /// Runs one tool-less model call for the module, or declines. Takes a
    /// [`GenerateRequest`](crate::wire::GenerateRequest) and answers
    /// `Option<String>`: `None` is the host saying it has no model for this.
    pub const GENERATE: &str = "Generate";
}

/// Every member of [`BUS_NAME`], in the interface's declaration order.
pub const METHODS: &[&str] = &[
    methods::INSTALL,
    methods::DETECT,
    methods::COMPRESS,
    methods::COMPACT,
    methods::COMPACT_WITH,
    methods::RETRIEVE,
    methods::REPL,
    methods::QUERY,
    methods::EXTRACT_HTML,
    methods::CACHE_STATS,
];
