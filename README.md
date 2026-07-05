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

- **JSON SmartCrusher** - renders repeated object arrays as compact tables,
  flattens safe nested cells, and keeps query-relevant, query-direction,
  anomaly, numeric change-point, information-dense, duplicate/near-duplicate
  cluster, and saturation/knee-based spread-anchor rows when large arrays are
  row-dropped.
- **Code compressor** - keeps imports, signatures, shallow structure, and
  important markers while collapsing deep bodies.
- **Log compressor** - preserves failures, warnings, summaries, stack traces,
  command-rule outputs, and reconstructible high-context template runs while
  dropping passing noise.
- **Search compressor** - groups grep/ripgrep output by file, ranks matches
  with shared BM25 query context, and keeps top hits with per-file and global
  omitted-match tallies.
- **Diff compressor** - keeps patch structure and changed lines, collapses long
  context, and marks omitted lockfile, generated-bundle, or configured noisy
  hunks with explicit reasons.
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

Host agent layers should resolve `AgentTokenjuiceCompression::Auto` to `Full`,
`Light`, or `Off` before calling TokenJuice. Passing unresolved `Auto` to the
tool-output adapter leaves the output unchanged and reports
`none/agent-profile-auto-unresolved`.

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

Shell-producing hosts can use the `route*_shell_policy` variants or call
`apply_shell_compaction_policy` with `ShellCompactionPolicy` before compacting
command output. The default OpenHuman-facing path uses `AllowSafeInventory`:
exact file-content reads, unsafe inventory actions, and mixed shell sequences
stay raw, while repository inventory output can still be summarized.

Conversation-level helpers under `conversation::*` expose deterministic Hermes
primitives without host runtime dependencies: token-budget tail selection,
tool-call/result boundary alignment, latest user/assistant anchors, JSON
string-leaf shrinking, and old tool-result digesting with sensitive metadata
redaction.

`savings::configure_record_recorder` installs a metadata-only savings recorder
that receives class-labeled `SavingsRecord` values (`counted`, `measured`, or
`estimated`) with content kind, compressor, byte/token counts, lossy/CCR flags,
and redacted rule or skip labels. The older four-argument
`configure_recorder` callback remains as a compatibility wrapper.

`ContextBreakdown` and `ContextBucket` provide host-facing context usage
metadata for UI bars and compression diagnostics. Buckets separate static
prefix costs such as system prompts and tool definitions from compressible
conversation or memory history, and measured provider prompt usage can override
rough local estimates without storing raw prompt text.

Already-extracted web pages can be passed through `reduce_web_extract` or
`reduce_web_extract_with_store`. The reducer removes inline base64 image blobs,
preserves ordinary markdown image URLs, and stores omitted middle content in CCR
before emitting a head/tail truncation footer. If CCR cannot retain the full
cleaned page, the reducer returns the cleaned page without lossy truncation.

Source-code file reads are exact by default. Hosts that intentionally want a
structural view can call `stub_code` with a `StubMode`; the returned
`CodeStubOutput` includes symbols, elided line ranges, and whether tree-sitter
or the heuristic fallback produced the stub.

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
  cache/        CCR recovery stores and retrieval markers
  compressor/   Compression trait and input/output types
  compressors/  Content-aware compressor implementations
  config/       Compression target and policy configuration
  conversation/ Provider-neutral conversation compaction helpers
  detect/       Content-kind hints and structural detection
  observability.rs Non-sensitive context usage breakdowns
  pipeline/     Typed transform/report primitives
  policy/       Host compaction policy helpers
  reduce/       Rule-engine reducers and command normalization
  rules/        Built-in, user, and project command reduction rules
  savings.rs    Host-installed savings attribution hook
  tool_integration.rs OpenHuman-style tool-output adapter
  openhuman/    Placeholder OpenHuman integration boundary
  error.rs      Shared error type
interface/      Self-hostable analytics UI for compression run metadata
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
typed-pipeline primitives, injectable CCR store, report-producing router path,
savings metadata, deterministic conversation helpers, and the analytics
interface are implemented. Public API names may still move as OpenHuman
integration hardens.

The project boundary is deliberate: keep the core crate small, do not add
OpenHuman runtime dependencies without a feature or adapter boundary, and do not
claim compression percentages until benchmark fixtures exist.
