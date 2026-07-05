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

TinyJuice is pre-1.0. The CLI, router, command-rule engine, CCR recovery store,
content detectors, native compressors, and OpenHuman-style adapter are in place;
public API names may still move as host integration hardens.
