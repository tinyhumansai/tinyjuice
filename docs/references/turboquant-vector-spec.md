# TurboQuant-Style Vector Compression Spec

## Status

Design reference for a possible TinyJuice vector-compression strategy. This is
not an implementation commitment, benchmark claim, or prompt compression claim.

This algorithm compresses fixed-width floating-point vectors for recall or
semantic-search indexes. Text must first be embedded into numeric vectors by a
separate embedding stage.

## Goals

- Compress vectors without model-specific training.
- Preserve approximate dot-product and cosine-similarity behavior.
- Support online ingestion.
- Use deterministic encoding for stable local indexes.
- Keep the encoded record simple enough for local storage.

## Inputs

```text
vector: Float32Array-like sequence
dim: number of vector dimensions
```

Recommended initial configuration:

```text
dim = 256
bits = 4
block_size = 32
use_qjl = false
polar_seed = 42
qjl_seed = 7919
```

## Record Format

```text
CompressedVector {
  config: CompressionConfig,
  polar: PolarPayload,
  qjl: Optional<QjlPayload>,
}

CompressionConfig {
  dim: u32,
  bits: u8,
  block_size: u16,
  use_qjl: bool,
  polar_seed: u64,
  qjl_seed: u64,
}

PolarPayload {
  packed_angles: bytes,
  magnitudes: Vec<f32>,
}

QjlPayload {
  sign_bits: bytes,
}
```

Byte serialization:

```text
u32le packed_angles_len
u32le magnitudes_byte_len
u32le qjl_sign_bits_len
bytes packed_angles
bytes magnitudes_as_f32le
bytes qjl_sign_bits
```

`qjl_sign_bits_len` is zero when `use_qjl = false`.

## Compression

1. Validate `len(vector) == dim`.
2. Optionally normalize if required by the metric.
3. Apply deterministic randomized rotation:
   - pad to the next power of two;
   - generate repeatable sign flips from `polar_seed`;
   - run a fast Walsh-Hadamard-style butterfly transform;
   - scale by `1 / sqrt(n)`;
   - repeat for the configured number of rounds.
4. Partition coordinates into `block_size` blocks.
5. For each block:
   - store a floating-point magnitude;
   - scale coordinates into a normalized range;
   - quantize each coordinate to `0..(2^bits - 1)`.
6. Bit-pack quantized coordinates into `packed_angles`.
7. If `use_qjl = true`, compute a Quantized Johnson-Lindenstrauss residual
   sketch and store 1-bit signs.

## Decompression

1. Validate serialized lengths against config.
2. Unpack quantized coordinate values.
3. Map values back into `[-1, 1]`:

```text
decoded = (quantized / (2^bits - 1)) * 2 - 1
```

4. Multiply decoded coordinates by block magnitude.
5. Use QJL residual signs, when enabled, for dot-product estimation rather than
   exact reconstruction.

## Operations

```text
compress(vector) -> CompressedVector
dequantize(compressed) -> Vec<f32>
dot_product(lhs, rhs) -> f32
cosine_similarity(lhs, rhs) -> f32
bytes_per_vector(config) -> usize
compression_ratio(config) -> f32
```

## Size Estimate

For `dim = 256`, `bits = 4`, `block_size = 32`:

```text
packed_angles = 256 * 4 bits = 128 bytes
magnitudes = ceil(256 / 32) * 4 bytes = 32 bytes
total = 160 bytes
```

Raw `f32` storage is:

```text
256 * 4 bytes = 1024 bytes
```

Structural ratio:

```text
1024 / 160 = 6.4x
1024 / 172 ~= 5.95x with 12-byte header
```

These are storage estimates only, not recall, latency, quality, or token-saving
claims.

## Embedding Boundary

Keep embedding separate:

```text
text/context -> embedding strategy -> vector compressor -> recall index
```

The compressor should not know whether vectors came from lexical, neural, local,
remote, or fixture-backed embeddings.

## Failure Modes

- vector length does not match `dim`;
- unsupported `bits`;
- zero or invalid `block_size`;
- corrupt serialized records;
- payload lengths do not match config;
- mixed configs in one index unless explicitly supported.

## Test Fixtures

- deterministic encoding for fixed seeds;
- round-trip serialization;
- dot-product error against raw vectors;
- cosine-similarity error against raw vectors;
- ranking stability on a small recall corpus;
- corrupt payload rejection;
- zero-vector behavior.
