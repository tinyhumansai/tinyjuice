<h1 align="center">TinyJuice</h1>

<p align="center">
 <img src="https://raw.githubusercontent.com/tinyhumansai/tinyjuice/refs/heads/main/docs/juice.png" />
</p>

<p align="center">
 <a href="https://crates.io/crates/tinyjuice"><img src="https://img.shields.io/crates/v/tinyjuice.svg" alt="crates.io" /></a>
 <a href="https://docs.rs/tinyjuice"><img src="https://docs.rs/tinyjuice/badge.svg" alt="docs.rs" /></a>
 <a href="https://github.com/tinyhumansai/tinyjuice/actions/workflows/ci.yml"><img src="https://github.com/tinyhumansai/tinyjuice/actions/workflows/ci.yml/badge.svg" alt="CI" /></a>
 <a href="LICENSE"><img src="https://img.shields.io/badge/License-GPLv3-blue.svg" alt="License: GPL v3" /></a>
</p>

**TinyJuice is a Rust token-compression engine for agent context.** It gives
OpenHuman and other Rust hosts a small, inspectable boundary for shrinking large
tool outputs before they enter a model context, while keeping exact originals
recoverable when a lossy view is shown.

Most agent systems pay the same context tax over and over: 5,000-line logs,
huge JSON lists, repeated grep output, lockfile diffs, rendered HTML, and full
source files all land in the model as raw text. TinyJuice routes those blobs by
content kind, applies a deterministic compressor tuned to the signal in that
kind, and reports what changed.

The result is not a magic "make prompts smaller" black box. It is a pluggable,
auditable compression layer with conservative pass-through behavior, recovery
markers for partial views, and host-owned policy for cost attribution.

## Why TinyJuice

- **Content-aware by default** - JSON, code, logs, search results, diffs, HTML,
  and plain text take different paths instead of one generic truncation rule.
- **Recoverable lossy views** - the CCR cache stores exact originals and appends
  a `tokenjuice_retrieve` footer whenever data is dropped.
- **Agent-profile policy** - hosts can run `full`, `light`, `off`, or runtime
  `auto` profiles per agent instead of using one global behavior.
- **Command-aware reduction** - built-in rules compact common shell, git, cargo,
  npm, docker, kubectl, database, cloud, lint, and test outputs.
- **OpenHuman-ready boundary** - the core crate avoids OpenHuman runtime
  dependencies; adapters install configuration, ML callbacks, and savings
  recorders from the host side.
- **No raw-content analytics requirement** - the dashboard consumes metadata,
  token and byte counts, latency, status, and strategy labels, not prompt text.

TinyJuice is designed for the work agents actually do: reading too much,
searching broadly, running noisy commands, and needing a compact but reversible
view that keeps failures, anomalies, changed hunks, signatures, and matching
lines visible.

## How It Works

```text
tool output / file / web payload
        |
        v
ContentHint + structural detection
        |
        v
JSON | Code | Log | Search | Diff | HTML | PlainText
        |
        v
specialized compressor or command-rule reducer
        |
        v
pass-through if unsafe, too small, disabled, or not smaller
        |
        v
CCR offload + retrieval footer when the view is lossy
```

The router is intentionally fail-soft. If it cannot shrink safely, it returns
the original bytes unchanged.

## Compression Surfaces

- **JSON SmartCrusher** - renders repeated object arrays as compact tables and
  keeps anomaly rows when large arrays are row-dropped.
- **Code compressor** - keeps imports, signatures, shallow structure, and
  important markers while collapsing deep bodies.
- **Log compressor** - preserves failures, warnings, summaries, stack traces,
  and command-rule outputs while dropping passing noise.
- **Search compressor** - groups grep/ripgrep output by file, ranks matches,
  and keeps top hits with per-file tallies.
- **Diff compressor** - keeps patch structure and changed lines, collapses long
  context and noisy lockfile/bundle hunks.
- **HTML compressor** - extracts readable text from rendered markup.
- **Plain-text ML slot** - optional host-provided callback for learned text
  compression; disabled by default.
- **Generic command fallback** - line-oriented head/tail reduction for command
  output when no specialized rule wins.

TinyJuice does not publish compression percentage claims yet. Throughput
benchmarks exist for hot paths, but ratio and quality claims require benchmark
fixtures that prove retained facts, latency, reversibility, and regression
safety.

## Quick Start

Add TinyJuice to a Rust project once published:

```toml
[dependencies]
tinyjuice = "0.1"
```

Use the small public trait scaffold when you want a simple strategy boundary:

```rust
use tinyjuice::{CompressionConfig, CompressionInput, Compressor, PassthroughCompressor};

fn main() -> Result<(), tinyjuice::TinyJuiceError> {
    let compressor = PassthroughCompressor;
    let output = compressor.compress(
        CompressionInput::new("Keep this text unchanged for now."),
        &CompressionConfig::default(),
    )?;

    assert_eq!(output.report.strategy, "passthrough");
    Ok(())
}
```

Use the content router for real tool-output compaction:

```rust
use tinyjuice::{CompressOptions, ContentHint, compress_content};

async fn compact_payload(big_payload: &str) {
    let hint = ContentHint {
        source_tool: Some("read_file".to_string()),
        extension: Some("json".to_string()),
        ..Default::default()
    };

    let result = compress_content(big_payload, Some(hint), &CompressOptions::default()).await;
    if result.applied {
        println!("{} -> {} bytes", result.original_bytes, result.compacted_bytes);
    }
}
```

OpenHuman-style tool output integration goes through:

```rust
use tinyjuice::{AgentTokenjuiceCompression, compact_tool_output_with_policy};

async fn compact_command_output(command_output: &str) {
    let (_text, _stats) = compact_tool_output_with_policy(
        "shell",
        Some(&serde_json::json!({ "command": "cargo test" })),
        command_output,
        Some(101),
        AgentTokenjuiceCompression::Full,
    ).await;
}
```

## Local Development

```sh
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test
cargo run --example passthrough
```

Run hot-path benchmarks:

```sh
cargo bench
```

Run fixture-driven compression benchmarks:

```sh
cargo run --release --example compression_benchmark -- --iterations 20
cargo run --release --example compression_benchmark -- --iterations 20 --format json
```

Fixture benchmark snapshot from `cargo run --release --example
compression_benchmark -- --iterations 20`:

| Use case | Compressor | Est. token reduction | Avg latency | CCR recovery |
| --- | --- | ---: | ---: | --- |
| JSON service inventory | SmartCrusher | 94.9% | 0.397 ms | yes |
| Cargo test failure log | Log | 93.6% | 0.667 ms | yes |
| Docker service log | Log | 99.8% | 1.110 ms | yes |
| Ripgrep search results | Search | 75.3% | 0.034 ms | yes |
| Unified diff | Diff | 84.3% | 0.008 ms | yes |
| HTML status report | HTML | 61.2% | 0.063 ms | yes |
| Rust source file | Code | 88.6% | 0.199 ms | yes |
| Plain text with ML off | None | 0.0% | 0.000 ms | n/a |

CCR recovery byte-compares the retrieved original for every lossy compaction.
These numbers are generated-fixture measurements, not production corpus claims.

See [docs/benchmarking.md](docs/benchmarking.md) for benchmark scope,
comparison targets, and reporting cautions. See [docs/benchmark](docs/benchmark)
for human-readable before/after sample reports and accuracy-check details.

Run the local analytics interface:

```sh
cd interface
npm install
npm run dev
```

The interface accepts metadata-oriented compression records. Do not feed raw
prompt, context, tool output, or credentials into analytics datasets.

## Crate Layout

```text
src/
  compress.rs        Universal content router
  compressors/       JSON, code, log, search, diff, HTML, ML, generic paths
  detect/            Content-kind hints and structural detection
  cache/             CCR offload, retrieval markers, memory/disk store
  rules/             Built-in + user + project command reduction rules
  reduce.rs          Rule-engine reduction pipeline
  tool_integration.rs OpenHuman-style tool-output adapter
  compressor/        Small public Compressor trait scaffold
  config/            Small public CompressionConfig scaffold
  openhuman/         Runtime-neutral OpenHuman adapter types
  savings.rs         Host-installed savings attribution hook
interface/           Self-hostable analytics UI
wiki/                Technical GitHub wiki source
docs/references/     Design references and candidate strategy specs
```

## Documentation

- [Wiki home](wiki/Home.md)
- [Quick Start](wiki/Quick-Start.md)
- [Capabilities](wiki/Capabilities.md)
- [Architecture](wiki/Architecture.md)
- [Router and Compressors](wiki/Router-and-Compressors.md)
- [Rule Engine](wiki/Rule-Engine.md)
- [CCR Recovery](wiki/CCR-Recovery.md)
- [OpenHuman Integration](wiki/OpenHuman-Integration.md)
- [Agent Guide](wiki/Agent-Guide.md)

## Status

TinyJuice is pre-1.0. The router, command-rule engine, CCR recovery store,
content detectors, several native compressors, the OpenHuman-style tool adapter,
and the analytics interface are implemented. Public API names may still move as
OpenHuman integration hardens.

The project boundary is deliberate: keep the core crate small, do not add
OpenHuman runtime dependencies without a feature or adapter boundary, and do not
claim compression percentages until benchmark fixtures exist.
