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

`README.md` now reflects that deterministic reducers, content-aware compressors,
and CCR recovery plumbing exist. Keep future public API contract changes
documented there or under `docs/` as implementation plans turn into code.

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

### Lossy Safety Is Partly Type-Checked

`CompressOutput::lossy` is still the compatibility flag for existing
compressors, and `route()` remains responsible for checking CCR availability.
The new `pipeline` module adds the first typed split:
`ReformatTransform` emits ordinary `TransformOutput`, while `OffloadOutput`
can only be built from a retained `CcrPutResult`. Existing compressors still
need gradual conversion into those traits before lossy safety is fully enforced
by construction.

### CCR Store Is Now Injectable

The process-global store remains for compatibility, but `CcrStore`,
`GlobalCcrStore`, and `MemoryCcrStore` now exist. `compress_content_with_store`
and `route_with_store` let tests and host adapters exercise CCR behavior
against an isolated store instead of the global cache. Remaining work is to
thread explicit stores through more OpenHuman call sites instead of relying on
the global compatibility path.

### Pipeline Reports Are Available But Compatibility-Based

`compress_content_with_store_report` and `route_with_store_report` now return a
redacted `PipelineReport` with byte counts, cheap bloat estimate, applied step,
CCR token ids, lossy flag, and skip reason. Because current compressors still
run through the compatibility router, the applied step is reported as
`compat_router` until compressors are converted into typed transforms.

### Missing Machine Protocol

The Rust crate has compatible types, but no stable `reduce-json` protocol or
CLI. That blocks non-Rust hosts and makes OpenHuman integration harder to test
from fixtures.

### Safe-Inventory Policy Implemented

The code now has a host-selectable `ShellCompactionPolicy` with the richer
reference behavior: safe inventory pipelines may compact, exact content reads
stay raw, mixed shell sequences stay raw, and unsafe actions such as
`find -exec` stay raw. The legacy `is_file_content_inspection_command()` helper
remains as a compatibility wrapper for older call sites.

### Footer Placement Requires A Host Contract

`CompressedOutput` now exposes both the compatibility `text` field and separate
`body`/`recovery_footer` fields. OpenHuman's tinyagents middleware composes
those fields by truncating only the body and reattaching the footer after its
per-tool char cap and byte-cap backstop. Other host integrations still need to
follow the same contract whenever they apply post-compaction caps.

### The Rule Catalog Depends On Arguments

`compact_output_with_policy` still forwards `arguments: None, exit_code: None`
for minimal call sites, but OpenHuman's tinyagents middleware now captures tool
arguments in `before_tool` and calls the full policy adapter after the tool
returns. That activates command rules, extension/query hints, and exit-code
handling on the production tinyagents path. Future host adapters should avoid
the minimal wrapper unless they genuinely lack tool metadata.

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
