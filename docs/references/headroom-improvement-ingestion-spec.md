# Headroom Improvement Ingestion Spec

## Purpose

This spec turns the Headroom deep dive into an implementation roadmap for
TinyJuice. It focuses on improvements that can be ingested without violating
the crate boundaries in `AGENTS.md`.

Source baseline:

- Repository: `https://github.com/chopratejas/headroom`
- Inspected commit: `e8151f059b4a9ba3fa43c7c67a7d310af08c1f3d`
- Companion algorithm reference:
  `docs/references/headroom-algorithms-strategies-spec.md`

## Non-Goals

- Do not vendor Headroom wholesale.
- Do not add OpenHuman runtime dependencies to the TinyJuice core crate.
- Do not add Redis, SQLite, ONNX, or embedding dependencies to the default
  build without explicit features.
- Do not claim Headroom's public compression percentages for TinyJuice.
- Do not log raw prompt, context, tool output, or CCR payloads.

## P0: Typed Reformat and Offload Pipeline

TinyJuice should split compressor outputs into two explicit mechanisms:

- `Reformat`: shrinks bytes while preserving all information in the prompt.
- `Offload`: removes bytes from the prompt and stores the original behind CCR.

Current TinyJuice code represents this through `CompressOutput::reformatted`
and `CompressOutput::lossy`, but the router still treats compressors as one
class of strategy. A Headroom-style pipeline would make the contract type-level.

Spec:

- Add `ReformatTransform` and `OffloadTransform` traits under a new small module,
  likely `src/pipeline/`.
- Require `OffloadTransform::apply` to take a CCR store and return a required
  cache token.
- Add `estimate_bloat(&self, content: &str) -> f32` to offloads.
- Return a `PipelineReport` with:
  - final text
  - applied steps
  - original and compacted byte counts
  - CCR tokens emitted
  - skip reasons without raw content
- Keep the existing `compress_content` API as the compatibility wrapper.

Acceptance:

- A lossy/offload transform cannot compile without returning a cache token.
- Reformat-only compression works when CCR is disabled.
- Offload compression declines when the CCR store cannot retain the original.
- Existing public APIs keep working.
- Tests cover no-op, reformat-only, offload-only, and mixed pipelines.

## P0: Live-Zone Adapter Contract

Headroom's strongest safety improvement is provider cache protection. TinyJuice
should expose the contract even if OpenHuman owns provider-specific walkers.

Spec:

- Define a provider-neutral `LiveZone` data structure:
  - frozen prefix range or block count
  - mutable block ranges
  - provider identifier
  - source of the freeze decision
- Define an adapter helper that compresses only supplied mutable blocks.
- Require adapter callers to splice replacements without reserializing frozen
  bytes when they operate on provider JSON requests.
- Add a detector-only cache-alignment helper for volatile values in frozen
  prompts:
  - UUIDs
  - ISO-like timestamps
  - JWT-shaped values
  - MD5, SHA1, and SHA256 hex hashes
- Detection must emit redacted labels and samples only.

Acceptance:

- Frozen bytes are byte-identical before and after compression in adapter
  fixtures.
- Volatile detector returns findings without modifying input.
- Tests include malformed JSON, string content, block content, and no-marker
  requests.

## P0: CCR Store Interface

TinyJuice has a useful CCR store today, but the global store makes it hard to
test policies and future backends independently.

Spec:

- Introduce a `CcrStore` trait:
  - `put(hash, payload)`
  - `get(hash)`
  - optional `len`
- Provide default implementations:
  - existing bounded in-memory store
  - existing disk tier adapted behind the trait
- Keep backend construction outside the hot compression path.
- Make requested backend initialization fail loudly instead of silently falling
  back, except for the existing best-effort compatibility path.
- Keep token validation before filesystem access.

Acceptance:

- Existing `cache::offload_checked` and `cache::retrieve` behavior remains
  compatible.
- New pipeline tests can inject an isolated in-memory store.
- Disk-backed retrieval rejects malformed tokens and path traversal attempts.
- TTL refresh behavior remains covered.

## P1: JSON SmartCrusher Upgrade

TinyJuice should evolve `src/compressors/json.rs` from fixed table rendering
into a staged analyzer and planner.

Spec:

- Add an analyzer that computes:
  - key frequencies
  - field types
  - unique ratios
  - constant fields
  - numeric statistics
  - change points
  - estimated reduction
- Add a planner that chooses row retention from:
  - dynamic anchors
  - error rows
  - numeric outliers
  - structural outliers
  - query matches when query context is available
- Add adaptive keep-count sizing:
  - SimHash duplicate clustering
  - unique-bigram saturation curve
  - knee detection
  - zlib redundancy validation
- Add heterogeneous-array handling:
  - schema by key frequency
  - sparse rows
  - discriminator buckets when clean
  - nested-uniform flattening
  - stringified JSON parsing
  - opaque blob CCR references

Acceptance:

- Existing JSON compressor tests still pass.
- New fixtures cover homogeneous rows, sparse rows, heterogeneous buckets,
  nested objects, stringified JSON, constants, outliers, errors, and duplicate
  rows.
- Row dropping is deterministic.
- Any omitted row is recoverable through CCR.

## P1: DiffNoise Offload

TinyJuice currently summarizes lockfile and bundle hunks inside
`DiffCompressor`. Headroom's dedicated `DiffNoise` offload makes this policy
separate and tunable.

Spec:

- Add `DiffNoise` as an offload transform for `ContentKind::Diff`.
- Estimate bloat as the fraction of diff-body bytes inside droppable sections.
- Drop:
  - configured lockfile suffixes
  - generated bundle suffixes
  - whitespace-only hunks when enabled
- Keep file headers and hunk headers.
- Append a CCR marker for the full original diff.

Acceptance:

- Lockfile hunks drop with a reason marker.
- Whitespace-only hunks drop only when every paired plus/minus line matches
  after ASCII whitespace stripping.
- Semantic hunks are preserved.
- The original diff is retrievable.

## P1: Log Template Reformat

TinyJuice should add a lossless reformat before the current signal-based log
offload.

Spec:

- Mine repeated log templates with configurable:
  - minimum line count
  - minimum consecutive run
  - similarity threshold
  - minimum constant tokens
- Emit template blocks plus variant data sufficient to reconstruct original
  lines.
- Run before the lossy signal compressor.
- Do not use CCR for the reformat path.

Acceptance:

- Repetitive logs shrink without CCR.
- Logs with no useful template pass through to the existing signal compressor.
- A fixture can reconstruct the original line sequence from the reformat output.

## P1: Extractive TextCrusher

ML text compression should not be the only useful plain-text path. Add a
deterministic extractive compressor.

Spec:

- Split text into sentence and line segments.
- Score by:
  - recency
  - BM25 relevance to optional query context
  - salience markers such as errors, numbers, all-caps identifiers, and dotted
    identifiers
- Suppress near-duplicates using word shingles.
- Keep selected segments in original order.
- Mark output as offload-backed when any segments are omitted.

Acceptance:

- Output contains only verbatim text spans from the input.
- Error and identifier fixtures are preserved.
- Near-duplicate prose collapses deterministically.
- With no query context, recency and salience still produce stable output.

## P1: Tag Protection for ML Compression

Before expanding ML compression, TinyJuice should protect custom workflow tags.

Spec:

- Add a single-pass custom tag protector around `MlTextCompressor`.
- Do not protect normal HTML tags.
- Protect custom tag blocks, nested blocks, and self-closing tags.
- Detect placeholder collisions and choose a safe salted prefix.
- Restore exact original tag text after ML compression.

Acceptance:

- Custom tags survive ML compression byte-for-byte.
- HTML content still routes through HTML handling.
- Duplicate custom-tag blocks restore independently.
- Malformed tags pass through without panics.

## P1: Query-Aware Relevance

Several Headroom strategies improve when the compressor knows what the user is
asking.

Spec:

- Add optional `query_context` to `ContentHint` or `CompressOptions`.
- Implement BM25 scoring in core with no external dependency.
- Use relevance in:
  - JSON row planning
  - search-output thinning
  - TextCrusher segment scoring
- Leave embedding relevance behind a feature or adapter.

Acceptance:

- Existing callers do not need to provide a query.
- Query-aware output is deterministic.
- Exact identifiers receive enough score to survive compression.

## P2: Policy Object

Headroom centralizes mode-specific decisions in a compression policy. TinyJuice
should implement a provider-neutral variant.

Spec:

- Add `CompressionPolicy` with fields such as:
  - live-zone-only
  - max offload ratio
  - volatile detector threshold
  - CCR required
  - ML compression allowed
  - learning/read-only mode for future adaptive rules
- Let OpenHuman adapters map runtime context into this policy.
- Keep the core crate free of OpenHuman runtime dependencies.

Acceptance:

- Policy defaults match current TinyJuice behavior.
- Tests show conservative policy disables ML and aggressive offload.
- Policy decisions are reported without raw content.

## P2: Observability and Benchmarks

Headroom has extensive tests and metrics around savings, latency, and
adversarial safety. TinyJuice should add its own local fixture suite before
advertising reductions.

Spec:

- Add fixture groups:
  - JSON arrays
  - diffs
  - logs
  - prose
  - tagged workflow text
  - provider live-zone payloads
- Track:
  - original bytes and estimated tokens
  - compacted bytes and estimated tokens
  - strategy steps
  - CCR retrieval availability
  - skip reasons
  - latency per transform
- Keep raw fixture inputs in test data only, not runtime logs.

Acceptance:

- `cargo test` covers every fixture.
- Benchmarks run separately from normal tests.
- Documentation phrases savings as fixture results only, never as a general
  claim.

## Suggested Implementation Order

1. Add the typed pipeline and isolated CCR store interface.
2. Port live-zone/cache-alignment contracts at the adapter boundary.
3. Add bloat estimation to existing JSON, diff, log, and search compressors.
4. Add `DiffNoise` and log-template transforms.
5. Add TextCrusher and BM25 relevance.
6. Upgrade JSON SmartCrusher with adaptive sizing and anchors.
7. Add tag protection around ML compression.
8. Add benchmark fixtures and publish only fixture-backed numbers.

