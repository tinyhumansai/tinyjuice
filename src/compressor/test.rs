#[cfg(test)]
mod tests {
    use crate::{
        CompressionConfig, CompressionInput, Compressor, PassthroughCompressor, TinyJuiceError,
    };

    #[test]
    fn passthrough_compressor_returns_input_unchanged() {
        let compressor = PassthroughCompressor;
        let input = CompressionInput::new("alpha beta gamma");

        let output = compressor
            .compress(input, &CompressionConfig::default())
            .expect("passthrough compression should succeed");

        assert_eq!(output.text, "alpha beta gamma");
        assert_eq!(output.report.original_bytes, output.report.compressed_bytes);
        assert_eq!(output.report.strategy, "passthrough");
    }

    #[test]
    fn empty_input_is_rejected() {
        let compressor = PassthroughCompressor;
        let input = CompressionInput::new("   ");

        let error = compressor
            .compress(input, &CompressionConfig::default())
            .expect_err("empty input should fail");

        assert_eq!(error, TinyJuiceError::EmptyInput);
    }
}
