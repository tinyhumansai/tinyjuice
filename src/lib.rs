//! Pluggable token compression for OpenHuman and other Rust hosts.
//!
//! TinyJuice owns the reusable TokenJuice compression engine. Hosts provide
//! configuration mapping, optional ML callbacks, savings attribution, and their
//! own RPC/tool surfaces.

pub mod cache;
pub mod classify;
pub mod compress;
pub mod compressor;
pub mod compressors;
pub mod config;
pub mod detect;
mod error;
pub mod ml;
pub mod openhuman;
pub mod reduce;
pub mod rules;
pub mod savings;
pub mod sdk;
pub mod text;
pub mod tokens;
pub mod tool_integration;
pub mod types;
mod util;

pub use compress::{compress_content, route};
pub use compressor::{
    CompressionInput, CompressionOutput, CompressionReport, Compressor, PassthroughCompressor,
};
pub use compressors::{compressor_for, generic_compressor};
pub use config::CompressionConfig;
pub use detect::detect_content_kind;
pub use error::{TinyJuiceError, TinyJuiceResult};
pub use reduce::reduce_execution_with_rules;
pub use rules::{LoadRuleOptions, load_builtin_rules, load_rules};
pub use sdk::{
    HostInstallSpec, SdkCompressOptions, SdkCompressionClassification, SdkCompressionRequest,
    SdkCompressionResponse, SdkCompressionStats, TinyJuiceHost, TinyJuiceSdk, arguments_value,
    compress_host_hook_payload, compress_request, host_hook_response, host_install_spec,
    host_install_specs, host_template, request_from_json_value,
};
pub use tool_integration::{
    CompactionStats, compact_output, compact_output_with_policy, compact_tool_output_with_policy,
    configure, current_options, install_config,
};
pub use types::{
    AgentTokenjuiceCompression, CompactResult, CompressInput, CompressOptions, CompressOutput,
    CompressedOutput, CompressorKind, ContentHint, ContentKind, ReduceOptions, ToolExecutionInput,
};

#[cfg(test)]
#[path = "text_tests.rs"]
mod text_tests;
