# Implementation Prompt: TinyJuice → OpenHuman Algorithm Ports

You are implementing a reviewed, committed plan. Do not re-derive the
analysis — read the plan documents, pick up the next unfinished slice, and
execute it. Write code; the planning phase is over.

## Repos

- **TinyJuice** (this repo): Rust compression engine crate. Core work happens
  in `src/`. Plans in `plan/`, reference specs in `docs/references/`.
- **OpenHuman**: `../openhuman-4`. Rust + Tauri desktop agent product. It
  vendors this crate at `vendor/tinyjuice` (git submodule, wired via
  `[patch.crates-io]`) and exposes it as the "TokenJuice" feature through the
  adapter at `src/openhuman/tokenjuice/`.

## Read These First (in order)

1. `plan/openhuman-algorithm-port-plan.md` — the work list. Per-algorithm
   verdicts, priorities, exact file paths on both sides, acceptance criteria,
   dependency graph. This is your backlog.
2. `plan/openhuman-integration-plan.md` — integration phases and the
   "Verified OpenHuman State" + "Known Integration Bugs" sections.
3. `plan/pipeline-and-ccr-plan.md`, `plan/conversation-compression-plan.md`,
   `plan/content-compressor-roadmap.md` — design detail for the P0/P1 slices.
4. The reference spec in `docs/references/` named by whichever slice you are
   implementing.

## Goal

Work through the port plan in priority order. The first three slices, which
unblock everything else:

1. **P0-2a — Fix the OpenHuman hook** (highest value, smallest diff):
   `ToolOutputMiddleware::after_tool` in
   `../openhuman-4/src/openhuman/tinyagents/middleware.rs` (~line 787) calls
   `compact_output_with_policy(content, tool_name, enabled, profile)`, which
   forwards `arguments: None, exit_code: None`. This leaves the 100-rule
   command catalog, extension/query hints, and exit-code handling dead in
   production. Migrate the call site to
   `compact_tool_output_with_policy(tool_name, arguments, output, exit_code,
   profile)`, threading the tool's JSON arguments and exit code through
   `TaToolResult`/context as needed.
2. **P0-2b — Fix the footer-truncation bug**: TinyJuice appends its CCR
   recovery footer at the end of compacted output; OpenHuman's per-tool char
   cap and 16 KiB byte-cap backstop run *after* compaction and keep the head,
   silently severing the footer. Preferred fix: split body and footer into
   separate fields on `CompressedOutput` in this crate so hosts truncate the
   body and reattach the footer; then update the middleware to compose them.
3. **P0-1 — Typed pipeline + injectable CcrStore** per
   `plan/pipeline-and-ccr-plan.md`: `ReformatTransform` vs `OffloadTransform`
   (lossy output unconstructable without a verified CCR token), `CcrStore`
   trait with memory/disk impls and global-compat wrappers, `PipelineReport`.

Then continue: P0-2 safe shell policy, P0-3 Hermes conversation primitives,
P1-1 savings accounting, P1-2 web extract, P1-3 AST stub reads, P1-4
compressor upgrades, P2 slices — all specified in the port plan.

## Hard Constraints

- Core TinyJuice must not depend on OpenHuman runtime types. OpenHuman
  consumes the crate via `src/openhuman/tokenjuice/` re-exports and
  `install_from_config`, or via host tools.
- Lossy compaction must be recoverable through CCR or must decline. Never
  emit a lossy view without a retained original.
- Never log or persist raw prompt/tool/CCR content from library code. Reports
  use counts, hashes, ids, and redacted labels.
- Exact file reads stay byte-exact by default; stubbing requires explicit
  host intent.
- No compression-percentage claims anywhere until fixture benchmarks exist in
  this repo.
- Filesystem/network/database access belongs in OpenHuman adapters or tools,
  never in core TinyJuice.
- Keep existing public APIs compiling; add compatibility wrappers instead of
  breaking callers. `cache::offload_checked`/`retrieve`/`retrieve_range` stay
  until OpenHuman migrates.
- Recovery-tool output (`tokenjuice_retrieve`, legacy `retrieve_tool_output`)
  is never re-compacted.

## Workflow Requirements

- Work on a feature branch, never directly on `main`. Small, focused commits
  per coherent slice.
- One slice = crate change → crate tests/fixtures → `vendor/tinyjuice`
  submodule bump in OpenHuman → adapter/tool change → OpenHuman end-to-end
  test. Do not mix core and host changes in a single commit.
- Validation gates before every commit: `cargo fmt --check`,
  `cargo clippy --all-targets -- -D warnings`, `cargo test` (in whichever
  repo changed). OpenHuman changes must also build its crate.
- Each slice's acceptance criteria are listed in
  `plan/openhuman-algorithm-port-plan.md` — treat them as the definition of
  done and encode them as tests where possible.
- Update `README.md`/`docs/` when public API changes; the plan docs note the
  README still describes the crate as scaffolding, which is stale.

## Known Trip Hazards (verified 2026-07-04)

- The vendored submodule in OpenHuman (`4b1a34f`) is behind this repo's HEAD.
  Any footer/marker/hook change requires a submodule bump plus a pass over
  OpenHuman's marker parsing and tool docs
  (`src/openhuman/tools/impl/system/retrieve_tool_output.rs` still documents
  the legacy sentinel).
- OpenHuman runs `after_tool` middlewares in REVERSE registration order;
  registration order is load-bearing. The chain per tool result:
  HandoffMiddleware (sees raw first) → PayloadSummarizer → TokenJuice →
  per-tool char cap → byte-cap backstop. TokenJuice may receive summarizer
  output, not raw output.
- `AgentTokenjuiceCompression::Auto` is resolved host-side
  (`agent/harness/definition.rs:487`: coding models → Light, else Full).
  TinyJuice and the mirrored OpenHuman copy now treat unresolved `Auto` as
  passthrough with `none/agent-profile-auto-unresolved`; do not pass unresolved
  `Auto` into the TokenJuice adapter.
- Two recovery tool names are still registered in OpenHuman during migration:
  canonical `tokenjuice_retrieve` plus legacy `retrieve_tool_output`. The
  legacy tool now delegates to the canonical implementation; remaining Phase 3
  work is migration cleanup, not behavior divergence.
- `ml_compression_enabled` (Kompress bridge) stays default-off until tag
  protection fixtures exist.

## Reporting

At the end of each slice, summarize: what landed (files + tests), which
acceptance criteria are proven by which test, what was deferred, and any
plan assumption that turned out wrong (update the plan doc in the same PR
when reality disagrees with it).
