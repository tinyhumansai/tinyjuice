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

**TinyJuice is token compression for terminal-heavy agents.** It sits between
tool output and model context, turning noisy logs, diffs, JSON, search results,
HTML, and source files into compact views that keep the signal visible.

Agents waste context on the same junk over and over: passing test chatter,
duplicated JSON keys, huge Docker logs, repetitive grep hits, lockfile diffs,
and markup nobody needs to reason about. TinyJuice cuts that noise before it
hits the model.

The important part: nothing disappears silently. Every partial view marks what
was dropped, and with CCR enabled (the default) the exact original is stored
behind a retrieval token so it can be pulled back on demand. Hosts that want
the strict lossless-or-recoverable guarantee (e.g. coding agents on the
`light` profile) can require a recovery token for any lossy output.

## Highlights

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

## Quick Setup

Install the CLI:

```sh
cargo install tinyjuice --locked
```

Run the minimal reducer CLI:

```sh
cargo run -- reduce --tool-name bash --command "git status" status.txt
cargo run -- reduce-json payload.json
cargo run -- verify --rules --fixtures
cargo run -- discover executions.ndjson
cargo run -- wrap -- cargo test
cargo run -- ls --store-dir .tokenjuice/ccr
cargo run -- cat --store-dir .tokenjuice/ccr <token>
cargo run -- stats --store-dir .tokenjuice/ccr
cargo run -- doctor --store-dir .tokenjuice/ccr
cargo run -- doctor codex
cargo run -- doctor hooks
cargo run -- install codex --local target/debug/tinyjuice
cargo run -- uninstall codex
```

Run hot-path benchmarks:

Run one hook installer:

| Logo | Client | Install |
| --- | --- | --- |
| <img width="48px" src="https://raw.githubusercontent.com/vincentkoc/tokenjuice/main/docs/client-openai.jpg" alt="Codex" /> | [Codex CLI](https://github.com/openai/codex) | `tinyjuice install codex` |
| <img width="48px" src="https://raw.githubusercontent.com/vincentkoc/tokenjuice/main/docs/client-claude.jpg" alt="Claude Code" /> | [Claude Code](https://docs.anthropic.com/en/docs/claude-code) | `tinyjuice install claude-code` |

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

The CLI `ls`, `cat`, and `stats` commands operate only on an explicit CCR disk
tier via `--store-dir`; they do not imply access to another process's in-memory
cache. `cat` supports `--lines START:END` and `--bytes START:END` ranges.
`stats` reports metadata-only counts and byte totals without reading token
content.

The CLI `doctor` command emits structured health JSON using `ok`, `warn`,
`broken`, and `disabled` statuses. It verifies built-in rule health by default,
can include fixture checks with `--fixtures [dir]`, and can inspect an explicit
CCR disk tier with `--store-dir`.
Host-aware doctor targets (`codex`, `openhuman`, and aggregate `hooks`) expose
expected commands, detected commands when present, and one repair command
without mutating user configuration.
The first host mutation path is `install codex` / `uninstall codex`, which
maintains only a TinyJuice-managed instruction block under the Codex config
root and preserves unrelated text.

`run_typed_pipeline` is the typed-transform entry point for new reducers. It
runs lossless `ReformatTransform`s before CCR-backed `OffloadTransform`s and
keeps the true original content available to offload transforms even after an
intermediate reformat.

Shell-producing hosts can use the `route*_shell_policy` variants or call
`apply_shell_compaction_policy` with `ShellCompactionPolicy` before compacting
command output. The default OpenHuman-facing path uses `AllowSafeInventory`:
exact file-content reads, unsafe inventory actions, and mixed shell sequences
stay raw, while repository inventory output can still be summarized.

Conversation-level helpers under `conversation::*` expose deterministic Hermes
primitives without host runtime dependencies: token-budget tail selection,
tool-call/result boundary alignment, latest user/assistant anchors, JSON
string-leaf shrinking, partial split/rejoin helpers for compacting a middle
window, last-N user exchange retention helpers, and old tool-result digesting
with sensitive metadata redaction.

`savings::configure_record_recorder` installs a metadata-only savings recorder
that receives class-labeled `SavingsRecord` values (`counted`, `measured`, or
`estimated`) with a source label (`live` or `fixture_benchmark`), content kind,
compressor, byte/token counts, lossy/CCR flags, and redacted rule or skip
labels. Fixture benchmark records are meant to be reported separately from live
runtime stats. The older four-argument
`configure_recorder` callback remains as a compatibility wrapper.

`ContextBreakdown` and `ContextBucket` provide host-facing context usage
metadata for UI bars and compression diagnostics. Buckets separate static
prefix costs such as system prompts and tool definitions from compressible
conversation or memory history, and measured provider prompt usage can override
rough local estimates without storing raw prompt text.

`live_zone::*` exposes provider-neutral cache/live-zone contracts for hosts
that need to preserve frozen prompt bytes exactly. Hosts provide byte ranges;
TinyJuice validates mutable-block replacements, preserves the frozen prefix,
and can detect volatile cache-hostile values such as UUIDs, timestamps, JWTs,
and hex hashes with redacted findings only.

`reduce_json_str` and `reduce_json_request` expose the library form of the
`reduce-json` protocol. They accept direct `ToolExecutionInput` JSON or an
`{ "input": ..., "options": ... }` envelope, reject malformed payloads with
structured errors, and return stable serde-compatible response fields. See
[docs/reduce-json-protocol.md](docs/reduce-json-protocol.md) for the current
request and response contract.

`verify_rules` checks the same builtin/user/project rule layers as the loader
without changing the lenient load contract. It reports parse errors, invalid
regex patterns, duplicate rule IDs, and shadowed lower-priority rules so CLI
or host diagnostics can fail loudly while runtime loading remains compatible.
`verify_rule_fixtures` runs recorded `*.fixture.json` examples through compiled
rules and reports pass counts, parse errors, and hash-only output mismatches.
`discover_fallback_outputs` groups command families that still fall through to
`generic/fallback` without including raw tool output in the report.

Already-extracted web pages can be passed through `reduce_web_extract` or
`reduce_web_extract_with_store`. The reducer removes inline base64 image blobs,
preserves ordinary markdown image URLs, and stores omitted middle content in CCR
before emitting a head/tail truncation footer. If CCR cannot retain the full
cleaned page, the reducer returns the cleaned page without lossy truncation.

Source-code file reads are exact by default. Hosts that intentionally want a
structural view can call `stub_code` with a `StubMode`; the returned
`CodeStubOutput` includes symbols, elided line ranges, and whether tree-sitter
or the heuristic fallback produced the stub. `PublicApi` stubs keep imports and
public signatures while replacing private declarations with elision metadata;
matched-symbol and line-range expansion also work through the heuristic
fallback.

Run the local analytics interface:

Use `tinyjuice update <host>` to refresh an installed hook and
`tinyjuice uninstall <host>` to remove it.

Custom paths, development installs, recovery, and tuning live in
[docs/agent-hooks/README.md](docs/agent-hooks/README.md).
Interactive installs also ask whether to add optional TinyJuice support commit
attribution for agent-created commits.

## Why It Helps

- **More useful context** - failures, summaries, changed hunks, matching lines,
  signatures, and anomalies stay visible.
- **Less transcript waste** - repeated structure, boilerplate, setup chatter,
  and markup get collapsed.
- **Recoverable partial views** - exact originals can be pulled back when a
  compact view is not enough.
- **Agent-ready defaults** - command-aware reducers understand common shell,
  git, cargo, npm, Docker, kubectl, database, cloud, lint, and test output.
- **Host-owned policy** - OpenHuman and other runtimes decide when compression
  is full, light, off, or profile-driven.
- **Privacy-aware by design** - analytics can use metadata, byte counts,
  latency, status, and strategy labels without requiring raw prompt text.

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

## What It Compresses

| Surface | What stays visible |
| --- | --- |
| JSON | Tables, schema shape, anomaly rows |
| Logs | Errors, warnings, stack traces, summaries |
| Search results | Top matches, file grouping, match counts |
| Diffs | File headers, hunk headers, changed lines |
| Code | Imports, signatures, top-level structure |
| HTML | Readable page text without script and markup noise |
| Plain text | Pass-through unless a host enables an ML callback |

## How Much Does It Save?

The checked-in benchmark corpus is
<!-- bench:corpus -->15.4 MB of real content across 166 cases<!-- /bench:corpus --> —
real OpenHuman snapshots plus source files, algorithm implementations, and
logs fetched from public GitHub repositories (see the per-category
`ATTRIBUTION.md` for sources and licenses; refresh with
`scripts/benchmark/fetch-github-samples.sh`). Percentages are
**token reduction: higher is better**
(90% means the output shrank to a tenth of its size; 0% means it passed
through untouched). Two passes are measured:

- **Pass 1 — without CCR (lossless)**: only information-preserving output
  ships. Faithful reshapes — JSON minify/tables, HTML→readable text — still
  apply because nothing is lost. Anything that would *drop* detail (log lines,
  diff context, search matches, code bodies, sampled JSON rows) passes the
  original through untouched: without the recovery cache there is no way to get
  that detail back, so TinyJuice refuses to emit a view the caller can't
  recover. Pass 1 is lossless by construction.
- **Pass 2 — with CCR**: the recovery cache is on, so information-dropping
  compression is allowed — every dropped block is offloaded behind a retrieval
  token and the exact original is one call away. This is where logs, diffs,
  search, and source code actually compress.

Three shapes show up in the table. **Faithful reshapes** (HTML→text) compress
the same in both passes — Pass 2 reads marginally lower only because the
optional recovery footer adds a few dozen bytes. **Information-dropping**
categories (diffs, search, source code) are 0% in Pass 1 — lossless
pass-through — and only compress in Pass 2, where the drops are recoverable.
**Hybrids** compress losslessly in Pass 1 *and* further in Pass 2: JSON renders
the full markdown table in Pass 1 (every row and value — the "markdown trick")
then samples the long middle away behind retrieval tokens in Pass 2; logs
collapse runs of byte-identical lines to `line [×N]` in Pass 1 (so a
duplicate-heavy log like a request flood can shrink 99% even without the cache)
then drop low-signal lines in Pass 2.

`Applied` counts the cases where compression actually fired — the rest pass
through because they are too small or a shape the compressor declines.

<!-- bench:table -->
| Category | Cases | Applied | Pass 1: without CCR | Pass 2: with CCR | Avg latency |
| --- | ---: | ---: | ---: | ---: | ---: |
| [HTML, RSS, and page snapshots](docs/benchmark/html-status-report/README.md) | 10 | 10 | 77.0% | 75.6% | 0.184 ms |
| [JSON SmartCrusher](docs/benchmark/json-smartcrusher/README.md) | 10 | 4 | 17.7% | 35.3% | 1.337 ms |
| [Polyglot source and XML](docs/benchmark/polyglot-source/README.md) (TS/Py/C++/Go/Rust/XML) | 6 | 6 | 12.8% | 76.6% | 0.447 ms |
| [GitHub log files](docs/benchmark/github-logs/README.md) (loghub, Elastic, CrowdSec, lnav, fail2ban) | 33 | 26 | 5.9% | 60.8% | 4.805 ms |
| [GitHub source files](docs/benchmark/github-source/README.md) (13 languages, real repos + algorithms) | 47 | 43 | 3.4% | 30.8% | 0.633 ms |
| [Service logs and crash reports](docs/benchmark/service-log/README.md) | 10 | 10 | 0.0% | 85.9% | 1.360 ms |
| [Test failure logs](docs/benchmark/test-failure-log/README.md) | 10 | 10 | 0.0% | 69.6% | 0.086 ms |
| [Search results](docs/benchmark/search-results/README.md) | 10 | 10 | 0.0% | 31.5% | 0.905 ms |
| [Unified diffs](docs/benchmark/unified-diff/README.md) | 10 | 10 | 0.0% | 68.9% | 0.274 ms |
| [Rust source](docs/benchmark/rust-source/README.md) | 10 | 7 | 0.0% | 26.6% | 0.837 ms |
| [Plain text with ML off](docs/benchmark/plain-text/README.md) | 10 | 0 | 0.0% | 0.0% | 0.000 ms |

Across the whole corpus TinyJuice cut 15.4 MB of content down to
6.7 MB, and every case passes its accuracy gates: signal checks
(errors, changed lines, matches, class/function signatures survive), task
checks, structural invariants (no inflation, no encoding damage), and a
byte-exact CCR recovery compare.
<!-- /bench:table -->

Source-code numbers are deliberately lower than they used to be: the
compressor now keeps every class skeleton (fields, signatures, doc comments),
short and important bodies (`main`, constructors, error handling), and the
first/last lines of collapsed bodies instead of erasing whole classes behind
one marker. Log compression is likewise template-aware: repeated lines
collapse to one exemplar with a `×N (first…last)` count while every distinct
error survives with its surrounding context.

These are local real-snapshot corpus measurements, not production-wide claims.
See [docs/benchmark](docs/benchmark) and
[docs/benchmarking.md](docs/benchmarking.md) for the reproducible reports.

## For Developers

The technical docs live in the wiki:

- [SDK and Plugin Integration](wiki/SDK-and-Plugin-Integration.md)
- [Quick Start](wiki/Quick-Start.md)
- [Capabilities](wiki/Capabilities.md)
- [Architecture](wiki/Architecture.md)
- [Router and Compressors](wiki/Router-and-Compressors.md)
- [CCR Recovery](wiki/CCR-Recovery.md)
- [Rule Engine](wiki/Rule-Engine.md)
- [OpenHuman Integration](wiki/OpenHuman-Integration.md)
- [Development](wiki/Development.md)
- [Security and Privacy](wiki/Security-and-Privacy.md)

## Status

TinyJuice is pre-1.0. The router, command-rule engine, CCR recovery store,
content detectors, several native compressors, the OpenHuman-style tool adapter,
typed-pipeline primitives, injectable CCR store, report-producing router path,
savings metadata, deterministic conversation helpers, live-zone cache contracts,
and the analytics interface are implemented. Public API names may still move as OpenHuman
integration hardens.

The project boundary is deliberate: keep the core crate small, do not add
OpenHuman runtime dependencies without a feature or adapter boundary, and do not
claim compression percentages until benchmark fixtures exist.
