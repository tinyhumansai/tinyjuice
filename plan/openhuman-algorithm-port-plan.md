# OpenHuman Algorithm Port Plan

## Goal

Evaluate every algorithm in `docs/references/` and decide which get ported
into OpenHuman, in what order, and through which boundary. For each accepted
algorithm this plan names the TinyJuice core modules, the OpenHuman files to
add or change, the wiring, and the acceptance criteria — in enough detail that
an implementing agent can execute it without re-deriving the analysis.

Verified against the OpenHuman repo (`../openhuman-2`) on 2026-07-04. All
OpenHuman paths below exist today unless marked NEW.

## Ground Rules

- TinyJuice core stays independent of OpenHuman runtime types. Algorithms live
  in the crate; OpenHuman consumes them through `src/openhuman/tokenjuice/`
  (re-exports + `install_from_config`) or through host tools.
- Anything touching the filesystem, network, databases, or session storage is
  a host adapter or host tool in OpenHuman, never core TinyJuice.
- Every lossy transform must round-trip through CCR or decline. No percentage
  claims until fixture benchmarks exist.
- Each slice ends with a `vendor/tinyjuice` submodule bump in OpenHuman plus a
  compatibility pass (marker constants, tool docs, hook signatures).

## Verdict Summary

| Reference spec | Verdict | Priority | Boundary |
| --- | --- | --- | --- |
| headroom typed pipeline + CcrStore + bloat estimation | Port | P0 | TinyJuice core |
| shell-output-intercept / safe-inventory policy | Port | P0 | Core + `shell` tool metadata |
| hermes conversation compaction (deterministic subset) | Port | P0/P1 | Core helpers + tinyagents middlewares |
| savings-accounting | Port | P1 | Core + `tokenjuice/savings.rs` |
| web-extract-truncate-store | Port | P1 | Core reducer + `web_fetch` tool |
| ast-stub-read | Port | P1 | Core (exists behind feature) + `file_read` tool |
| headroom SmartCrusher / DiffNoise / log templates / TextCrusher | Port | P1/P2 | TinyJuice core only |
| ranked-search-read | Port (host-side) | P2 | NEW OpenHuman tool + core scoring |
| subagent-summary | Port (partial) | P2 | Core shapes + subagent_runner |
| hermes summary provider (LLM) + cache hints | Port (partial) | P2 | Core contracts + summarize.rs |
| batched-edit-validation | Defer | — | Would be NEW `edit_file` mode |
| sql-introspection-reduction | Defer | — | No SQL read-tool surface yet |
| turboquant-vector | Defer | — | Memory index concern, not context |
| tokenjuice reduce-json protocol + CLI + installers | Defer for OpenHuman | — | OpenHuman uses the crate directly |

The deferred verdicts and their unlock conditions are at the end.

---

## P0-1: Typed Pipeline, Injectable CcrStore, Bloat Estimation

Source specs: `headroom-improvement-ingestion-spec.md`,
`headroom-algorithms-strategies-spec.md`. Design detail already exists in
`plan/pipeline-and-ccr-plan.md`; this section only adds the OpenHuman wiring.

Why first: everything else that is lossy builds on the
`ReformatTransform`/`OffloadTransform` split, and the injectable `CcrStore`
is what lets OpenHuman-side tests run against an isolated store instead of
the process-global cache.

TinyJuice files (per pipeline-and-ccr-plan):

- `src/pipeline/mod.rs`, `src/pipeline/transform.rs`, `src/pipeline/report.rs`
  — present with the first transform/report primitives.
- `src/cache/store.rs` — `CcrStore`, `GlobalCcrStore`, and `MemoryCcrStore`
  are present; global wrappers are preserved.

OpenHuman files:

- Current `../openhuman-4` has a local `src/openhuman/tokenjuice/` copy rather
  than `vendor/tinyjuice`; mirror store/pipeline exports there until the crate
  dependency boundary is restored.
- `src/openhuman/tokenjuice/mod.rs` — re-export `CcrStore`, `PipelineReport`;
  `install_from_config` constructs the store explicitly instead of relying on
  global state, keeping the global as compatibility path.
- `src/openhuman/tokenjuice/schemas.rs` — surface `PipelineReport` fields
  (applied steps, skip reasons, CCR tokens present) in the debug RPC.

Acceptance:

- OpenHuman tests can construct an in-memory `CcrStore` and assert offload
  behavior without touching the global cache.
- Lossy output is unconstructable without a verified token (compile-time).
- No behavior change for existing compaction paths (fixture parity).

## P0-2: Fix The Hook, Then Safe Shell Policy

Source specs: `shell-output-intercept-spec.md`,
`tokenjuice-improvement-spec.md` (P0 Safety Policy Port). Prerequisite: the
Phase 1 call-site migration from `plan/openhuman-integration-plan.md` — the
rule catalog and command classification are dead until
`ToolOutputMiddleware::after_tool` (`src/openhuman/tinyagents/middleware.rs`
~line 787) passes tool arguments and exit codes via
`compact_tool_output_with_policy`.

TinyJuice files:

- `src/policy/shell.rs` (NEW) — command identity helpers: leading-`cd` strip,
  quote-aware tokenization, sequential-command detection, unquoted pipe split,
  inventory classification (`find`, `fd`, `ls`, `tree`, `rg --files`,
  `git ls-files`), unsafe-action detection (`-exec`, redirects, mutation
  subcommands), safe stdin filters (`head`, `tail`, `sort`, `uniq`, `wc`,
  `sed -n`).
- `src/policy/mod.rs` (NEW) — host policy enum: `compact-all`, `skip-all`,
  `skip-file-content`, `allow-safe-inventory`.
- `src/reduce.rs` — replace the narrow
  `is_file_content_inspection_command()` guard with the policy module.

OpenHuman files:

- `src/openhuman/tinyagents/middleware.rs` — the hook migration itself:
  build `arguments` from the tool result's recorded call arguments and pass
  `exit_code` (the `shell` tool at `src/openhuman/tools/impl/system/shell.rs`
  already returns exit status; thread it through `TaToolResult` metadata if
  not already present).
- `src/openhuman/config/schema/tokenjuice.rs` — add `shell_policy` key
  defaulting to `allow-safe-inventory`.
- `src/openhuman/tokenjuice/config_patch.rs` — partial-update support for the
  new key.

Acceptance:

- `cat file.rs`, `sed -n 1,200p file.rs`, `jq . file.json` through the
  OpenHuman `shell` tool stay raw by default.
- `find . -type f | sort | head` and `rg --files` compact.
- `find . -exec cat {} \;` and mixed `&&` sequences stay raw.
- Exit code and stderr presence preserved in compacted output.
- A shell result carrying argv reaches the rule reducer (end-to-end test in
  OpenHuman, not just crate-level).

## P0-3: Hermes Deterministic Conversation Primitives

Source spec: `hermes-compression-algorithms-spec.md`. Design detail exists in
`plan/conversation-compression-plan.md`; this section maps it onto OpenHuman's
already-existing conversation middlewares — this is an upgrade of live code,
not a new layer.

Existing OpenHuman seams:

- `src/openhuman/tinyagents/summarize.rs` — `ContextCompressionMiddleware`
  (live history summarization) and message trimming.
- `MicrocompactMiddleware` (registered in
  `src/openhuman/tinyagents/middleware.rs`) — clears older tool-result bodies,
  keeping the N most recent.
- `src/openhuman/agent/harness/session/transcript.rs` — conversation history.

Port order (deterministic subset only; LLM summary contract is P2):

1. TinyJuice `src/conversation/budget.rs` — `select_tail_by_budget`,
   threshold math (`effective_input_window = context_length −
   max_output_tokens`), protected head with decay after first compaction.
2. TinyJuice `src/conversation/boundary.rs` — tool-call/tool-result pairing by
   call id, boundary alignment (every retained tool result keeps its parent
   call), orphan sanitizer, latest-real-user and latest-visible-assistant
   anchors.
3. TinyJuice `src/conversation/tool_digest.rs` — dedup identical older tool
   results by stable hash, replace old large bodies with tool-specific digests
   (terminal: command/exit/lines; read: path/range/size; search:
   pattern/count/top evidence), keep recent tail verbatim.
4. TinyJuice `src/conversation/types.rs` — provider-neutral message shapes
   convertible from `tinyagents::harness::message::Message`.
5. `shrink_json_string_leaves()` for provider-safe tool-argument shrinking
   (parse first, shrink string leaves only, re-serialize valid JSON).

OpenHuman files:

- `src/openhuman/tinyagents/middleware.rs` — `MicrocompactMiddleware` swaps
  its clear-the-body behavior for `tool_digest` digests. Must not
  double-process: microcompact and the digest are one pass, not two.
- `src/openhuman/tinyagents/summarize.rs` — `ContextCompressionMiddleware`
  adopts `budget.rs` threshold math and `boundary.rs` alignment before its
  summarization step; conversion via `conversation/types.rs`.
- `src/openhuman/tinyagents/convert.rs` — message conversion helpers.

Acceptance:

- Retained tool results always have retained parent tool calls after
  compaction (property test over generated histories).
- Duplicate read-file outputs keep only the newest full copy.
- JSON tool arguments remain parseable after shrinking; non-JSON untouched.
- Compaction never drops the latest real user message or latest visible
  assistant reply.
- Digest output contains no raw secrets (redaction before persistence).

## P1-1: Savings Accounting Upgrade

Source spec: `savings-accounting-spec.md`. OpenHuman already persists savings
(`src/openhuman/tokenjuice/savings.rs`, dashboard-facing). The old reviewed
plan assumed OpenHuman's README still made a broad percentage-savings claim;
as of 2026-07-06 that claim is no longer present. This slice still lands early
because live estimates, measured usage, and future fixture benchmark results
must be labeled separately before any savings surface can be trusted.

TinyJuice files:

- `src/savings.rs` — extend records with accounting class (`counted`,
  `measured`, `estimated`), applied compressor, content kind, lossy flag, CCR
  token present, rule id, skip reason. Keep the existing recorder callback
  signature as a compatibility wrapper.

OpenHuman files:

- `src/openhuman/tokenjuice/savings.rs` — persist the new fields; feed
  measured usage from `inference::provider::UsageInfo` where a turn's real
  token counts are known, and label everything else estimated.
- Dashboard schema (`src/openhuman/tokenjuice/schemas.rs`) — expose the class
  labels so the UI can say "estimated" honestly.

Acceptance:

- No raw content in serialized records (existing invariant, re-asserted).
- Measured vs estimated is visible end-to-end (crate → adapter → RPC schema).
- Fixture benchmark results are reported separately from live stats.

Status: implemented. TinyJuice exposes class-labeled `SavingsRecord` values
with live vs fixture-benchmark source labels, byte metadata, lossy/CCR flags,
rule/skip metadata, and optional measured model usage. OpenHuman mirrors the
records, persists rollups by model, compressor, accounting class, record
source, and content kind, exposes those labels through `tokenjuice.savings_*`
controllers, and records provider `UsageInfo` as measured metadata-only usage.

## P1-2: Web Extract Truncate-Store

Source spec: `web-extract-truncate-store-spec.md`. OpenHuman has the host
surface: `src/openhuman/tools/impl/network/web_fetch.rs` (plus `curl.rs` and
`http_request.rs`). Today its output goes through the generic middleware chain;
large pages hit the 16 KiB byte-cap backstop, which is exactly the
unrecoverable head-truncation this spec replaces.

TinyJuice files:

- `src/compressors/web_extract.rs` (NEW) — accepts already-extracted
  markdown/text/HTML plus URL metadata; strips inline base64 image payloads
  (`![alt](data:image...)` → `[IMAGE: alt]`, real URLs kept); small pages
  returned whole; large pages: offload full text via `CcrStore`, return
  line-snapped head/tail (head ratio 0.75 of the char limit), omission marker
  and retrieval footer. The reducer gates footer emission through
  `OffloadOutput::from_retained_put`, so it cannot emit a footer without a
  retained original.
- `src/types.rs` — `WebExtractReduceInput { url, title, content, format,
  char_limit, metadata }` and batch shape with a combined inline budget.

Storage decision (resolves the spec's open question): model-facing recovery
goes through CCR like every other offload. OpenHuman's
`ToolResultArtifactStore` is not involved; the operator artifact store is a
CLI/standalone concern.

OpenHuman files:

- `src/openhuman/tools/impl/network/web_fetch.rs` — after extraction, call the
  reducer through `tokenjuice` re-exports and return its output as the tool
  result. The result then flows through the normal middleware chain; because
  it is already reduced it should sit under the caps (and the footer contract
  fix from the integration plan protects it if not).
- `src/openhuman/config/schema/tokenjuice.rs` — `web_extract` sub-block:
  `char_limit` (default 15000, clamp 2000..500000), `convert_base64_images`
  (default true).

Status: OpenHuman now has the shared reducer wired through both `web_fetch`
and `http_request`, and `[tokenjuice.web_extract]` is the canonical config
shape. Legacy flat `web_extract_*` keys, env vars, and settings patches remain
accepted and are normalized before live updates.

Acceptance:

- Base64 image bytes absent from output and metadata; real image URLs remain.
- Retrieval footer emitted only when the offload was retained.
- Full URLs with tokens/query secrets never logged (hash + host only).
- A 1 MB page returns head/tail plus footer; `tokenjuice_retrieve` returns
  the full stored text.

## P1-3: AST Stub Reads With Explicit Host Intent

Source specs: `ast-stub-read-spec.md`, roadmap detail in
`plan/content-compressor-roadmap.md`. The crate already has tree-sitter code
compression behind `tokenjuice-treesitter` (OpenHuman enables it by default in
`Cargo.toml`). Current TinyJuice/OpenHuman state has explicit stub modes and
host intent wired: exact reads remain byte-exact by default, while stub modes
must be requested explicitly.

TinyJuice files:

- `src/compressors/code.rs` — add `StubMode` (`SignaturesOnly`, `PublicApi`,
  `MatchedSymbols(Vec<String>)`, `ExpandAroundLines(Vec<Range>)`), elision
  metadata with line ranges, parse status in the report; lexical brace-aware
  fallback already exists.
- `src/types.rs` — `ContentHint` gains an explicit `read_intent` field
  (`Exact` | `Stub(StubMode)`), defaulting to `Exact`. The router treats
  `Exact` + code extension as passthrough for file-read sources; only `Stub`
  invokes the code compressor for reads.

OpenHuman files:

- `src/openhuman/tools/impl/filesystem/file_read.rs` — add an optional `mode`
  argument (`full` default, `stub`, `signatures`, `symbols: [..]`). The tool
  maps it to `read_intent` and calls the compressor directly for stub modes;
  `full` stays byte-exact and is marked exact so the middleware chain cannot
  compress it.
- Tool schema description must tell the model when to prefer stub mode
  (exploring unfamiliar files) and that `tokenjuice_retrieve`/re-read gets the
  full body.

Acceptance:

- `file_read` with no mode returns exact bytes even when the router would
  have compacted (regression test through the full middleware chain).
- Stub output preserves imports, exports, and public signatures; elided
  ranges are listed with line numbers.
- Parse failure reports fallback and never silently omits without markers.

## P1-4: Core Compressor Upgrades (SmartCrusher, DiffNoise, Log Templates, TextCrusher, BM25)

Source specs: both headroom specs. Design detail already exists in
`plan/content-compressor-roadmap.md`; nothing here needs OpenHuman-side code
beyond the submodule bump — these upgrade compaction quality for tool output
that already flows through the hook. Order within this slice (from the
ingestion spec): bloat estimators → DiffNoise + log-template reformat →
TextCrusher + shared BM25 → JSON SmartCrusher analyzer/planner → tag
protection around the ML callback.

One OpenHuman-relevant note: tag protection is the precondition for ever
enabling `ml_compression_enabled` (the Kompress bridge is already wired in
`src/openhuman/tokenjuice/mod.rs`). Until the tag-protection fixtures land,
that config stays default-off.

Acceptance: as specified per compressor in the roadmap plan; plus one
OpenHuman end-to-end fixture per compressor family driven through
`compact_tool_output_with_policy`.

Status: implemented. TinyJuice has host-hook e2e coverage through
`compact_tool_output_with_policy` for SmartCrusher, DiffNoise, search
thinning/BM25 query ranking, TextCrusher, HTML extraction, and generic command
fallback, each asserting recoverability where the path is lossy. OpenHuman
`../openhuman-4` mirrors that host-adapter coverage in
`src/openhuman/tokenjuice/tool_integration.rs`; commit `88a17f9f8` adds the
remaining HTML extraction and generic command fallback fixtures so each current
compressor family is exercised through the adapter path.

## P2-1: Ranked Search-Read Tool

Source spec: `ranked-search-read-spec.md`. This is a NEW host tool, not a
compressor: it touches the filesystem, so core TinyJuice only supplies
scoring. OpenHuman already has the pieces it collapses (`grep.rs`,
`glob_search.rs`, `file_read.rs` in `src/openhuman/tools/impl/filesystem/`).

TinyJuice files:

- `src/relevance/bm25.rs` (NEW, shared with TextCrusher and search
  compressor) — no external deps.
- `src/compressors/search.rs` — expose snippet-window merging and the rank
  score (symbol/path/density/import-export weights, generated/vendor
  penalties) as pure functions over host-provided match data.

OpenHuman files:

- `src/openhuman/tools/impl/filesystem/search_read.rs` — one tool:
  glob → grep → rank via TinyJuice scoring → bounded snippets with path/line
  metadata → omission report. Registered alongside the existing tools; the
  schema description positions it as the preferred first move over separate
  glob/grep/read calls.
- `src/openhuman/tools/impl/filesystem/mod.rs` — registration.
- `src/openhuman/tools/user_filter.rs` and
  `app/src/utils/toolDefinitions.ts` — include `search_read` in the file-read
  tool family so user tool preferences and frontend toggles do not hide the
  ranked read surface when "Read Files" is enabled.
- `app/src/utils/toolTimelineFormatting.ts` — render `search_read` as a
  known search/read timeline tool.

Acceptance:

- Exact symbol matches rank above path matches above density matches.
- Output is bounded (max files, snippets, bytes) with explicit omitted
  counts; vendor/generated paths deprioritized unless requested.
- The tool never returns whole files; follow-up reads use `file_read`.

Status: TinyJuice has pure ranking and snippet-window fixtures for exact
symbol/path/density ordering, vendor/generated penalties, bounded window
merging, and omitted counts. OpenHuman `../openhuman-4` already carried the
host `search_read` implementation and registration; commit `76399a6ce` wires it
into the Rust user-preference family, frontend tool catalog, and timeline
formatting so the registered tool is exposed consistently with `file_read`.

## P2-2: Subagent Summary Evidence Extraction

Source spec: `subagent-summary-spec.md`. OpenHuman already has
`src/openhuman/agent/harness/subagent_runner/` and a semantic
`PayloadSummarizer` (`src/openhuman/tinyagents/payload_summarizer.rs`). The
port is the deterministic layer: evidence extraction that does not depend on
a model, used as the fallback and as the scaffold the summarizer fills in.

TinyJuice files:

- `src/conversation/subagent.rs` (NEW) — transcript/event shapes, deterministic
  extraction of file paths, line numbers, tool names, and final conclusions;
  findings/evidence/uncertainty/omitted output with a byte budget.

OpenHuman files:

- `src/openhuman/agent/harness/subagent_runner/` — on subagent completion,
  run deterministic extraction first; hand the structured result to
  `PayloadSummarizer` as context rather than raw transcript; if the summarizer
  declines or fails (it has a circuit breaker), the deterministic summary is
  the result instead of the raw transcript.

Acceptance:

- Deterministic summary contains only verbatim evidence (no invented text).
- Summarizer failure path returns the deterministic summary, never the full
  transcript, and never loses the subagent's final answer.

Status: implemented. TinyJuice exposes provider-neutral subagent transcript
events, deterministic evidence extraction, bounded markdown rendering, omission
reports, and fixtures for long transcripts, failure evidence, contradictory
findings, and byte-budget truncation. OpenHuman `../openhuman-4` constructs
`SubagentRunOutcome::deterministic_summary` from the child run history, uses the
shared handoff renderer for completed and incomplete `spawn_subagent` /
`continue_subagent` results, preserves the final answer when it is not already
inside the deterministic summary, and falls back to the deterministic
checkpoint when the LLM checkpoint summary is empty or fails. The
`PayloadSummarizer` also supplies this deterministic scaffold for subagent-like
payloads before invoking its semantic summarizer.

## P2-3: Summary Contract And Prompt Cache Hints

Source spec: `hermes-compression-algorithms-spec.md` (P2 portions). Builds on
P0-3. TinyJuice adds `src/conversation/summary.rs` (SummaryProvider trait,
strict structured schema, deterministic fallback, failure policy:
auth/network failures abort and preserve messages) and `src/cache_hints.rs`
(static-prefix cache key = SHA-256 of instructions + NUL + sorted tool
schemas; Anthropic carrier hints only, no payload mutation) plus
`src/live_zone.rs` for frozen-prefix byte ranges, mutable replacement splicing,
and redacted volatile-value detection. OpenHuman's
`ContextCompressionMiddleware` implements `SummaryProvider` over its existing
summarization path and adopts the failure policy; cache hints feed the
provider layer (`src/openhuman/inference/provider/`) as routing hints only.

Acceptance: summary text cannot be mistaken for a fresh user turn (metadata
tag + end marker); repeated compactions update the previous summary instead
of nesting; tool order does not change the cache key; frozen prefix bytes are
never rewritten.

Status: implemented in core and mirrored in OpenHuman. TinyJuice exposes the
`SummaryProvider` contract, strict `StructuredSummary` shape, summary metadata
tag/end marker, deterministic fallback, summary upsert helpers, stable prompt
cache hints, and live-zone frozen-prefix splicing. OpenHuman
`../openhuman-4/src/openhuman/tokenjuice/` mirrors those modules, while
`src/openhuman/tinyagents/summarize.rs` uses TokenJuice boundary/tail planning,
the summary failure policy, deterministic trim fallback, the canonical summary
end marker, and duplicate-summary removal before rewriting model requests.
Cache hints and live-zone helpers are available through the mirrored
`tokenjuice` exports for provider/adaptor use.

---

## Deferred (with unlock conditions)

- **Batched edit + validation** (`batched-edit-validation-spec.md`): real
  value (fewer retry loops) but it is mutation tooling, not compression, and
  OpenHuman's `edit_file.rs`/`apply_patch.rs` already exist. Unlock: evidence
  from tool stats that edit-retry loops are a top token cost; then core
  defines match policies/report types and `edit_file` gains a batched mode.
- **SQL introspection reduction** (`sql-introspection-reduction-spec.md`):
  OpenHuman's only SQL tool today is `insert_sql_record.rs` (write-side).
  There is no schema-dump or query tool whose output needs reducing. Unlock:
  a SQL read/introspection tool lands in OpenHuman.
- **TurboQuant vectors** (`turboquant-vector-spec.md`): storage compression
  for embedding indexes, not prompt compression. OpenHuman does have an
  embedding surface (`.fastembed_cache`, memory tree), so this may eventually
  matter for memory-index size — but it must never be reported as token
  savings. Unlock: memory-index storage becomes a measured problem.
- **reduce-json protocol, CLI, installers**
  (`tokenjuice-improvement-spec.md` P0 protocol / P1 CLI,
  `tinyjuice-integration-spec.md`): OpenHuman consumes the crate directly, so
  none of this blocks OpenHuman. It stays on the TinyJuice product roadmap
  (`plan/rule-cli-and-safety-parity-plan.md`) for non-Rust hosts.

## Sequencing And Dependencies

```text
P0-1 pipeline/CcrStore ──┬─▶ P1-2 web extract (needs OffloadTransform)
                         ├─▶ P1-4 compressor upgrades (needs transforms + estimators)
                         └─▶ P1-3 stub reads (router intent honors typed path)
P0-2 hook fix + shell policy ─▶ everything flowing through after_tool
P0-3 conversation primitives ─▶ P2-2 subagent, P2-3 summary/cache hints
P1-1 savings accounting ─▶ resolves README claim conflict (independent)
P2-1 search-read tool (needs BM25 from P1-4, otherwise independent)
```

Every slice: crate change → crate tests/fixtures → submodule bump in
OpenHuman → adapter/tool change → OpenHuman end-to-end test → savings fields
verified. Small PRs per slice; no slice mixes core and host changes in one
commit.
