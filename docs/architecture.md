# Architecture

TinyJuice is organized around a small compression boundary:

1. callers provide `CompressionInput`
2. a `Compressor` strategy produces `CompressionOutput`
3. the output carries compressed text plus an inspectable `CompressionReport`

The core crate should remain independent of OpenHuman runtime internals. The
`openhuman` module is a placeholder for adapter types that can translate between
OpenHuman request context and TinyJuice inputs.
