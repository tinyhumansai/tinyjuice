#[cfg(test)]
mod tests {
    use crate::{CompressionConfig, TinyJuiceError};

    #[test]
    fn default_config_targets_aggressive_compression() {
        let config = CompressionConfig::default();

        assert_eq!(config.target_ratio, 0.2);
        assert!(config.preserve_system_instructions);
    }

    #[test]
    fn invalid_target_ratio_is_rejected() {
        let config = CompressionConfig {
            target_ratio: 0.0,
            ..CompressionConfig::default()
        };

        assert_eq!(config.validate(), Err(TinyJuiceError::InvalidTargetRatio));
    }
}
