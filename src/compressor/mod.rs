#[cfg(test)]
#[path = "mod_tests.rs"]
mod tests;
mod types;

pub use types::{
    CompressionInput, CompressionOutput, CompressionReport, Compressor, PassthroughCompressor,
};
