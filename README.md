<h1 align="center">TinyJuice</h1>

<p align="center">
 <img src="https://github.com/tinyhumansai/tinyjuice/raw/main/docs/juice.png" />
</p>

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

The crate now includes deterministic reducers, content-aware compressors, and
CCR-backed recovery primitives. The scaffold remains intentionally small while
the OpenHuman adapter migration and typed pipeline work continue.

## Direction

TinyJuice supports and is growing toward:

- provider-neutral token input compression
- pluggable compression strategies
- configurable compression targets for lossless and recoverable lossy modes
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

## API Notes

`compress_content` and `route` return `CompressedOutput`. The `text` field is
the compatibility output ready to inline into model context. When CCR retained
an original, `body` contains the compacted body without the recovery footer and
`recovery_footer` contains the footer separately. Hosts that apply their own
post-compaction caps should truncate only `body` and then reattach
`recovery_footer` so recovery markers remain reachable.

`compress_content_with_store` and `route_with_store` accept an injectable
`CcrStore`. Their `_report` variants also return a redacted `PipelineReport`
with byte counts, a cheap bloat estimate, applied steps, skip reason, lossy
flag, and CCR token IDs. Existing `compress_content`, `route`, and `cache::*`
helpers still use the process-global `GlobalCcrStore` for compatibility. New
tests and host adapters can use `MemoryCcrStore` to assert CCR behavior without
touching the global cache.

Shell-producing hosts can use `apply_shell_compaction_policy` with
`ShellCompactionPolicy` before compacting command output. The default
OpenHuman-facing path uses `AllowSafeInventory`: exact file-content reads,
unsafe inventory actions, and mixed shell sequences stay raw, while repository
inventory output can still be summarized.

Conversation-level helpers under `conversation::*` expose deterministic Hermes
primitives without host runtime dependencies: token-budget tail selection,
tool-call/result boundary alignment, latest user/assistant anchors, JSON
string-leaf shrinking, and old tool-result digesting with sensitive metadata
redaction.

Run the local analytics interface:

```sh
cd interface
npm install
npm run dev
```

## Crate Layout

```text
src/
  cache/        CCR recovery stores and retrieval markers
  compressor/   Compression trait and input/output types
  compressors/  Content-aware compressor implementations
  config/       Compression target and policy configuration
  conversation/ Provider-neutral conversation compaction helpers
  pipeline/     Typed transform/report primitives
  policy/       Host compaction policy helpers
  reduce/       Rule-engine reducers and command normalization
  openhuman/    Placeholder OpenHuman integration boundary
  error.rs      Shared error type
interface/      Self-hostable analytics UI for compression run metadata
```

## Status

TinyJuice includes deterministic reducers, content-aware compressors, and CCR
recovery plumbing. The first typed-pipeline primitives, injectable CCR store,
report-producing router compatibility path, and cheap bloat estimators exist;
full transform conversion is still in progress. Some legacy scaffold modules
remain while the OpenHuman adapter migration continues.
