<h1 align="center">TinyJuice</h1>

<p align="center">
 <a href="https://crates.io/crates/tinyjuice"><img src="https://img.shields.io/crates/v/tinyjuice.svg" alt="crates.io" /></a>
 <a href="https://docs.rs/tinyjuice"><img src="https://docs.rs/tinyjuice/badge.svg" alt="docs.rs" /></a>
 <a href="https://github.com/tinyhumansai/tinyjuice/actions/workflows/ci.yml"><img src="https://github.com/tinyhumansai/tinyjuice/actions/workflows/ci.yml/badge.svg" alt="CI" /></a>
 <a href="LICENSE"><img src="https://img.shields.io/badge/License-GPLv3-blue.svg" alt="License: GPL v3" /></a>
</p>

**TinyJuice is a pluggable token compression crate for OpenHuman.** The goal is
to provide a small Rust boundary where OpenHuman can route prompt, context, and
conversation inputs through interchangeable compression strategies before model
execution.

This repository is intentionally blank scaffolding right now. It defines the
crate shell, contribution workflow, CI/release scaffolding, and placeholder API
types. Compression algorithms, OpenHuman adapters, and benchmark claims have not
landed yet.

## Direction

TinyJuice is intended to support:

- provider-neutral token input compression
- pluggable compression strategies
- configurable compression targets, including aggressive 80-90% reduction modes
- inspectable compression reports for cost, quality, and safety review
- OpenHuman integration points that can be enabled without coupling the core
  crate to the full OpenHuman runtime

## Quick Start

Add TinyJuice to a Rust project once published:

```toml
[dependencies]
tinyjuice = "0.1"
```

For local development:

```sh
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo build --all-targets
cargo test
```

Run the placeholder example:

```sh
cargo run --example passthrough
```

## Crate Layout

```text
src/
  compressor/   Compression trait and input/output types
  config/       Compression target and policy configuration
  openhuman/    Placeholder OpenHuman integration boundary
  error.rs      Shared error type
```

## Status

TinyJuice is pre-implementation. The current `PassthroughCompressor` is a
scaffold implementation that returns input unchanged so downstream wiring can
compile while real strategies are designed and tested.
