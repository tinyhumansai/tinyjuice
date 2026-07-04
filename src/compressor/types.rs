use serde::{Deserialize, Serialize};

use crate::{CompressionConfig, TinyJuiceError, TinyJuiceResult};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CompressionInput {
    pub text: String,
}

impl CompressionInput {
    pub fn new(text: impl Into<String>) -> Self {
        Self { text: text.into() }
    }

    pub fn validate(&self) -> TinyJuiceResult<()> {
        if self.text.trim().is_empty() {
            return Err(TinyJuiceError::EmptyInput);
        }

        Ok(())
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CompressionOutput {
    pub text: String,
    pub report: CompressionReport,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CompressionReport {
    pub original_bytes: usize,
    pub compressed_bytes: usize,
    pub strategy: String,
}

pub trait Compressor {
    fn name(&self) -> &'static str;

    fn compress(
        &self,
        input: CompressionInput,
        config: &CompressionConfig,
    ) -> TinyJuiceResult<CompressionOutput>;
}

#[derive(Clone, Debug, Default)]
pub struct PassthroughCompressor;

impl Compressor for PassthroughCompressor {
    fn name(&self) -> &'static str {
        "passthrough"
    }

    fn compress(
        &self,
        input: CompressionInput,
        config: &CompressionConfig,
    ) -> TinyJuiceResult<CompressionOutput> {
        input.validate()?;
        config.validate()?;

        let original_bytes = input.text.len();

        Ok(CompressionOutput {
            report: CompressionReport {
                original_bytes,
                compressed_bytes: original_bytes,
                strategy: self.name().to_string(),
            },
            text: input.text,
        })
    }
}
