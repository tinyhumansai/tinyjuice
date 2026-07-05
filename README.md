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

The important part: compacted output stays recoverable. When TinyJuice shows a
partial view, it stores the exact original behind a retrieval token instead of
silently throwing data away.

## Quick Setup

Install the CLI:

```sh
cargo install tinyjuice --locked
```

Run one hook installer:

| Logo | Client | Command |
| --- | --- | --- |
| <img width="48px" src="https://raw.githubusercontent.com/vincentkoc/tokenjuice/main/docs/client-openai.jpg" alt="Codex" /> | [Codex CLI](https://github.com/openai/codex) | `tinyjuice install codex` |
| <img width="48px" src="https://raw.githubusercontent.com/vincentkoc/tokenjuice/main/docs/client-claude.jpg" alt="Claude Code" /> | [Claude Code](https://docs.anthropic.com/en/docs/claude-code) | `tinyjuice install claude-code` |

Custom paths, development installs, recovery, and tuning live in
[docs/agent-hooks/README.md](docs/agent-hooks/README.md).

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

## Benchmark Snapshot

The checked-in benchmark corpus uses 10 real snapshots per category and verifies
inline accuracy plus CCR recovery for lossy compactions.

| Category | Cases | Avg est. token reduction | Avg latency |
| --- | ---: | ---: | ---: |
| JSON SmartCrusher | 10 | 58.2% | 0.362 ms |
| Test failure logs | 10 | 14.1% | 0.050 ms |
| Service and Docker logs | 10 | 86.3% | 0.177 ms |
| Search results | 10 | 39.8% | 0.494 ms |
| Unified diffs | 10 | 66.7% | 0.174 ms |
| HTML, RSS, and page snapshots | 10 | 75.4% | 0.187 ms |
| Rust source | 10 | 51.9% | 0.796 ms |
| Plain text with ML off | 10 | 0.0% | 0.000 ms |

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
