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
tinyjuice = "0.2"
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

## SDK and Plugin Integration

TinyJuice now exposes two integration paths:

- Rust hosts use the crate SDK directly.
- Non-Rust plugins and harnesses call the `tinyjuice reduce-json` protocol.

The SDK accepts a host-neutral `ToolExecutionInput` with tool name, command,
argv, stdout/stderr or combined text, exit code, cwd, and metadata. The response
contains the inline text plus metadata about the applied content kind,
compressor, token estimate, byte counts, and CCR recovery token when one was
created. Do not log the request body from adapters; tool output may contain
prompts, credentials, or private context.

### Rust SDK

```rust
use tinyjuice::{
    AgentTokenjuiceCompression, TinyJuiceHost, TinyJuiceSdk, ToolExecutionInput,
};

async fn compact_for_harness(tool_output: String, exit_code: i32) {
    let sdk = TinyJuiceSdk::new(TinyJuiceHost::RustHarness)
        .with_profile(AgentTokenjuiceCompression::Full);

    let response = sdk
        .compress_tool_output(ToolExecutionInput {
            tool_name: "shell".to_string(),
            command: Some("cargo test".to_string()),
            argv: Some(vec!["cargo".to_string(), "test".to_string()]),
            combined_text: Some(tool_output),
            exit_code: Some(exit_code),
            ..Default::default()
        })
        .await;

    println!("{}", response.inline_text);
}
```

Use `TinyJuiceHost::OpenHuman` for OpenHuman adapters and
`TinyJuiceHost::RustHarness` for standalone Rust harnesses. Hosts should map
their own config into `CompressOptions`, choose the per-agent profile, and expose
CCR retrieval before enabling lossy compaction in production.

### JSON Protocol

Build the binary locally:

```sh
cargo build --release --bin tinyjuice
```

Send a full SDK request:

```json
{
  "host": "generic-json",
  "profile": "full",
  "input": {
    "toolName": "shell",
    "command": "cargo test",
    "argv": ["cargo", "test"],
    "combinedText": "large tool output...",
    "exitCode": 0,
    "metadata": {
      "source": "custom-harness"
    }
  },
  "options": {
    "minBytesToCompress": 512,
    "maxInlineChars": 1200,
    "ccrEnabled": true
  }
}
```

Run it through the protocol:

```sh
tinyjuice reduce-json payload.json
cat payload.json | tinyjuice reduce-json --host generic-json -
```

A bare `ToolExecutionInput` object is also accepted when the host, profile, and
options can stay at defaults.

### Codex and Claude Code Hooks

Build or install a `tinyjuice` binary, then merge a hook into the host config:

```sh
tinyjuice install codex
tinyjuice install claude-code
```

The Codex installer updates `~/.codex/hooks.json` with a `PostToolUse` hook for
`Bash` tool output. When TinyJuice compacts a large result, it emits
`hookSpecificOutput.additionalContext`, matching Codex's hook output model.

The Claude Code installer updates `~/.claude/settings.json` with a `PostToolUse`
hook for `Bash` tool output. When TinyJuice compacts a large result, it emits
`hookSpecificOutput.updatedToolOutput`, so Claude Code sees the compacted tool
result rather than the noisy original.

Both installers:

- preserve existing hooks and settings
- replace an older TinyJuice hook for the same host
- write a `.bak` file next to the edited JSON file
- expect `tinyjuice` to be on `PATH` unless `--binary` is supplied

Examples:

```sh
tinyjuice install codex --binary /usr/local/bin/tinyjuice
tinyjuice install claude-code --path ~/.claude/settings.json
```

The raw hook entrypoints are also available for custom installers:

```sh
tinyjuice codex-post-tool-use
tinyjuice claude-code-post-tool-use
```

Hook invocations use a disk-backed CCR store so recovery tokens survive the
short-lived hook process. By default the store lives under the user's cache
directory at `tinyjuice/ccr`; override it with:

```sh
export TINYJUICE_CCR_DIR=/path/to/tinyjuice-ccr
```

Recover a full original from a hook footer:

```sh
tinyjuice retrieve <token>
```

Useful hook tuning variables:

```sh
export TINYJUICE_MIN_BYTES_TO_COMPRESS=2048
export TINYJUICE_MAX_INLINE_CHARS=1200
export TINYJUICE_CCR_MIN_TOKENS=500
export TINYJUICE_CCR_ENABLED=true
```

Templates remain available for inspection or custom packaging:

```sh
tinyjuice hosts
tinyjuice host-template codex
tinyjuice host-template claude-code
tinyjuice host-template generic-json
```

## Local Development

```sh
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test
cargo run --example passthrough
cargo run --bin tinyjuice -- hosts
cargo run --bin tinyjuice -- host-template codex
```

Run hot-path benchmarks:

```sh
cargo bench
```

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
  sdk.rs             Host-neutral SDK and reduce-json request/response types
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
