//! ML plain-text compressor ("Kompress") — trait slot.
//!
//! Plain text has no structural skeleton to exploit, so high-quality
//! compression needs a learned model (Headroom uses ModernBERT token
//! classification to drop low-salience spans). That path runs the `kompress`
//! backend of the shared `runtime_python_server` and is **opt-in** behind the
//! `tokenjuice.ml_compression_enabled` config flag.
//!
//! This module is the [`Compressor`] slot; it delegates to
//! [`crate::ml`]. Whenever the flag is off or the Python
//! runtime is unavailable, `compress` declines so the router falls back to the
//! generic compressor — never an error in the agent loop.

use async_trait::async_trait;

use super::Compressor;
use crate::compressors::text::compress_ml_with_tag_protection;
use crate::types::{CompressInput, CompressOptions, CompressOutput, CompressorKind};

pub struct MlTextCompressor;

#[async_trait]
impl Compressor for MlTextCompressor {
    fn kind(&self) -> CompressorKind {
        CompressorKind::MlText
    }

    async fn compress(
        &self,
        input: &CompressInput<'_>,
        opts: &CompressOptions,
    ) -> Option<CompressOutput> {
        compress_ml_with_tag_protection(input.content, opts).await
    }
}
