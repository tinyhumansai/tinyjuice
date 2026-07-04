//! Pluggable token compression scaffolding for OpenHuman.
//!
//! TinyJuice currently exposes placeholder types and a passthrough compressor so
//! integration code can compile while real compression strategies are designed.

pub mod compressor;
pub mod config;
mod error;
pub mod openhuman;

pub use compressor::{
    CompressionInput, CompressionOutput, CompressionReport, Compressor, PassthroughCompressor,
};
pub use config::CompressionConfig;
pub use error::{TinyJuiceError, TinyJuiceResult};
