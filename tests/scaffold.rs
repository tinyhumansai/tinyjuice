use tinyjuice::{CompressionConfig, CompressionInput, Compressor, PassthroughCompressor};

#[test]
fn public_scaffold_can_compress_through_trait() {
    let compressor: &dyn Compressor = &PassthroughCompressor;
    let output = compressor
        .compress(
            CompressionInput::new("public api smoke test"),
            &CompressionConfig::default(),
        )
        .expect("scaffold compressor should succeed");

    assert_eq!(output.text, "public api smoke test");
}
