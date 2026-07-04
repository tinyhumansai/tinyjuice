use tinyjuice::{CompressionConfig, CompressionInput, Compressor, PassthroughCompressor};

fn main() -> Result<(), tinyjuice::TinyJuiceError> {
    let compressor = PassthroughCompressor;
    let input = CompressionInput::new("Keep this text unchanged for now.");
    let output = compressor.compress(input, &CompressionConfig::default())?;

    println!("{}", output.text);
    Ok(())
}
