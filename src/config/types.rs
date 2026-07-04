use serde::{Deserialize, Serialize};

use crate::{TinyJuiceError, TinyJuiceResult};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CompressionConfig {
    pub target_ratio: f32,
    pub preserve_system_instructions: bool,
}

impl CompressionConfig {
    pub fn validate(&self) -> TinyJuiceResult<()> {
        if self.target_ratio <= 0.0 || self.target_ratio > 1.0 {
            return Err(TinyJuiceError::InvalidTargetRatio);
        }

        Ok(())
    }
}

impl Default for CompressionConfig {
    fn default() -> Self {
        Self {
            target_ratio: 0.2,
            preserve_system_instructions: true,
        }
    }
}
