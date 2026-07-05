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

## Quick Setup

Install the CLI:

```sh
cargo install tinyjuice --locked
```

Run one hook installer:

| Logo | Client | Install |
| --- | --- | --- |
| <img width="48px" src="https://raw.githubusercontent.com/vincentkoc/tokenjuice/main/docs/client-openai.jpg" alt="Codex" /> | [Codex CLI](https://github.com/openai/codex) | `tinyjuice install codex` |
| <img width="48px" src="https://raw.githubusercontent.com/vincentkoc/tokenjuice/main/docs/client-claude.jpg" alt="Claude Code" /> | [Claude Code](https://docs.anthropic.com/en/docs/claude-code) | `tinyjuice install claude-code` |

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

The checked-in benchmark corpus is 6.4 MB of real tool output — 10 real
snapshots per category. Percentages are **token reduction: higher is better**
(90% means the output shrank to a tenth of its size; 0% means it passed
through untouched). `Applied` counts the cases where compression actually
fired — the rest pass through because they are too small or a shape the
compressor declines.

| Category | Cases | Applied | Token reduction (mean) | Avg latency |
| --- | ---: | ---: | ---: | ---: |
| [Service and Docker logs](docs/benchmark/service-log/README.md) | 10 | 10 | 86.9% | 0.193 ms |
| [HTML, RSS, and page snapshots](docs/benchmark/html-status-report/README.md) | 10 | 10 | 77.5% | 0.184 ms |
| [Unified diffs](docs/benchmark/unified-diff/README.md) | 10 | 10 | 70.4% | 0.182 ms |
| [Rust source](docs/benchmark/rust-source/README.md) | 10 | 8 | 53.7% | 0.702 ms |
| [Search results](docs/benchmark/search-results/README.md) | 10 | 10 | 48.3% | 0.401 ms |
| [JSON SmartCrusher](docs/benchmark/json-smartcrusher/README.md) | 10 | 3 | 20.1% | 0.535 ms |
| [Test failure logs](docs/benchmark/test-failure-log/README.md) | 10 | 2 | 15.3% | 0.041 ms |
| [Plain text with ML off](docs/benchmark/plain-text/README.md) | 10 | 0 | 0.0% | 0.000 ms |

Across the whole corpus TinyJuice cut 6.4 MB of tool output down to 3.9 MB.
Enabling CCR recovery costs well under 1% of the savings — the retrieval
footer adds a few dozen bytes per compacted output, which is why "with CCR"
numbers in the detailed reports read marginally lower than the raw algorithm
numbers. Every case passes its accuracy gates: signal checks (errors, changed
lines, matches survive), task checks, and a byte-exact CCR recovery compare.

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

TinyJuice is pre-1.0. The CLI, router, command-rule engine, CCR recovery store,
content detectors, native compressors, and OpenHuman-style adapter are in place;
public API names may still move as host integration hardens.
