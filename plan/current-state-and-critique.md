# Current State And Critique

## What Exists Today

TinyJuice currently has two API layers:

- The original scaffold API in `compressor/` and `config/`, centered on
  `CompressionInput`, `CompressionOutput`, `CompressionReport`, and
  `PassthroughCompressor`.
- The newer TokenJuice-style engine, exported from `lib.rs`, centered on
  `CompressInput`, `CompressOptions`, `CompressedOutput`, `compress_content`,
  `route`, `compact_tool_output_with_policy`, and `reduce_execution_with_rules`.

The newer engine is the real implementation surface. It includes:

- content-kind detection for JSON, code, log, search, diff, HTML, and plain text
- per-kind compressors for JSON, code, log, search, diff, HTML, ML text, and
  generic command fallback
- a command-output rule reducer with built-in, user, and project rule overlays
- a built-in rule catalog under `src/vendor/rules`
- CCR recovery markers and a bounded in-memory plus optional disk-backed store
- OpenHuman-facing per-agent profiles: `auto`, `full`, `light`, and `off`
- host callback slots for ML compression and savings accounting

## Good Existing Decisions

The core routing model is sound. `route()` detects content, tries a specialized
compressor, falls back only for command output, declines non-shrinking output,
and uses CCR before returning lossy output.

The crate boundary is mostly clean. ML text compression is a host callback, and
OpenHuman config is mapped into plain `CompressOptions` through `install_config`
instead of importing runtime config types.

The CCR store has the right safety instincts: fixed-length hex token validation,
memory-first retrieval, optional disk tier, range retrieval, TTL support, and a
skip for recovery-tool output.

The rule system is worth preserving. Most shell and tool-output handling should
be added as rules or rule-engine features before adding one-off compressor code.

## Risks And Gaps

### README Drift

`README.md` still says the repository is intentionally blank scaffolding and
that real strategies have not landed. That is no longer true. The docs should
be updated after implementation plans turn into code, but this plan does not
edit README because the user requested plan files only.

### Duplicate Compressor Concepts

There are two `Compressor` traits: one in `compressor/types.rs` for the
scaffold API and one in `compressors/mod.rs` for the content router. That is
confusing for public users. The scaffold API can remain temporarily, but future
docs should make the router API canonical or wrap it behind a clearer public
facade.

### Exact File Reads Can Be Misrouted

The rule reducer explicitly avoids compacting exact file-content inspection
commands when they hit `generic/fallback`. The newer content router can still
compress content when an adapter passes an extension hint such as `rs`, `py`, or
`json`. That is good for explicit "stub this file" behavior, but risky for
normal exact reads. OpenHuman should provide a host policy that marks exact file
reads as raw unless the user or tool intentionally requests a stub/summary.

### Lossy Safety Is Runtime-Checked, Not Type-Checked

`CompressOutput::lossy` is currently a flag, and `route()` is responsible for
checking CCR availability. This works, but it is too easy for future code to
accidentally bypass `route()` or return lossy text without recoverability. The
Headroom-style typed pipeline should make offload-backed transforms structurally
different from lossless reformats.

### Global CCR Store Makes Tests And Policy Harder

The global store is convenient for integration, but it makes isolated tests,
adapter-specific policies, and future backends harder. An injectable `CcrStore`
trait should be introduced while preserving the current global functions as
compatibility wrappers.

### Missing Machine Protocol

The Rust crate has compatible types, but no stable `reduce-json` protocol or
CLI. That blocks non-Rust hosts and makes OpenHuman integration harder to test
from fixtures.

### Missing Safe-Inventory Policy

The code has a narrow `is_file_content_inspection_command()` guard. It does not
yet implement the richer policy from the references: safe inventory pipelines
may compact, exact content reads stay raw, mixed shell sequences stay raw, and
unsafe actions such as `find -exec` stay raw.

### Footer Placement Is A Fragile Host Contract

`route()` appends the recovery footer to the end of the compacted text and
returns a single string. Hosts that apply their own truncation caps after
compaction (OpenHuman applies a per-tool char cap and a 16 KiB byte-cap
backstop after the TokenJuice stage) keep the head and cut the tail, which
severs the footer and leaves an unrecoverable lossy view. `CompressedOutput`
should expose the footer (or the body/footer split) as separate fields so
hosts can truncate the body and reattach the marker. Until then, every host
integration must be audited for post-compaction truncation.

### The Rule Catalog Is Inert Without Arguments

`compact_output_with_policy` forwards `arguments: None, exit_code: None`, and
the rule reducer only fires for command output. OpenHuman currently calls this
minimal wrapper, so the 100-rule catalog, extension hints, query hints, and
exit-code handling are all dead at the only production call site. The plan's
Phase 1 must treat migrating that call site as the primary deliverable, not a
confirmation task.

### Limited Observability

Current stats expose bytes and compressor labels, and savings callbacks estimate
tokens. Missing pieces include skip reasons, applied-step traces, CCR retention
status, omission counts, and fixture-backed benchmark reports.

## What Should Be Added

- Type-level separation of reformat and offload transforms.
- Injectable CCR store trait with current memory/disk behavior preserved.
- Stable `reduce-json` request/response protocol and minimal CLI.
- Safe-inventory command policy before wider shell compaction.
- OpenHuman adapter tests for `full`, `light`, `off`, recovery tool bypass,
  exact file reads, and config reload.
- Deterministic benchmark and fixture suite before claims.
- Query context and BM25 scoring as a lightweight shared primitive.
- Better compressor metadata: omitted rows, omitted lines, skip reasons, and
  applied transforms.

## What Should Not Be Added Yet

- OpenHuman runtime dependencies in the core crate.
- Redis, SQLite, ONNX, database drivers, browser/web-fetch clients, or provider
  SDKs in the default build.
- LLM summarization as mandatory core behavior.
- Abstractive prose compression as the default fallback.
- Vector compression in the prompt-compression path.
- Host installers before `reduce-json`, validation, and doctorable policy are
  stable.
- Compression percentage marketing claims.

