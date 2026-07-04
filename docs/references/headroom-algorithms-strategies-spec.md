# Headroom Algorithms and Strategies Spec

## Purpose

This spec documents the compression algorithms and product strategies found in
`chopratejas/headroom` and maps them to TinyJuice concepts. It is a design
reference only. It does not claim TinyJuice savings until benchmark fixtures
exist in this repository.

Source baseline:

- Repository: `https://github.com/chopratejas/headroom`
- Inspected commit: `e8151f059b4a9ba3fa43c7c67a7d310af08c1f3d`
- Local comparison point: TinyJuice working tree on `2026-07-04`

## Current TinyJuice Overlap

TinyJuice already carries several Headroom-inspired ideas:

- content-kind routing for JSON, diffs, logs, search output, HTML, code, and
  plain text
- CCR offload and retrieval markers for lossy reductions
- a JSON array table compressor with error and numeric-outlier preservation
- unified-diff context collapsing and noisy lockfile summarization
- signal-preserving log compression
- optional tree-sitter code compression behind a feature boundary
- optional ML text compression through the existing OpenHuman runtime boundary

The biggest Headroom deltas are not single compressors. They are the formal
contracts around when a transform is allowed to mutate data, how expensive
offload paths are gated, and how provider cache boundaries are protected.

## Canonical Pipeline

Headroom's Rust core defines a two-mechanism compression pipeline:

- `ReformatTransform`: packs content more densely without requiring retrieval.
  Examples include JSON minification and log template compaction.
- `OffloadTransform`: removes bytes from the prompt, stores the original in CCR,
  and emits a retrieval marker. Offload success requires a cache key that
  resolves in the provided store.

The orchestrator runs reformats serially and runs each offload's bloat
estimator in parallel with the reformat phase. It then applies offloads only
when the estimator clears a threshold, or when reformats did not shrink enough
and the estimator still reports useful bloat.

TinyJuice should treat this as a contract upgrade:

- make "reformat" and "CCR-backed offload" explicit public concepts
- require offload transforms to prove recoverability before returning output
- record applied steps and cache keys in the compression report
- use domain bloat estimation instead of trying every compressor eagerly

## Bloat Estimation

Headroom uses domain-specific bloat scores instead of a generic byte-ratio
guess:

- Log bloat combines repetition with priority dilution.
- Diff bloat measures context-line dominance over change lines.
- Search bloat measures match clustering across files.
- JSON array bloat estimates row-array shape and saturation before invoking the
  heavier SmartCrusher path.
- Diff-noise bloat counts bytes inside droppable lockfile or whitespace-only
  hunks.

TinyJuice can adopt this without changing compressor output format. Each
compressor can expose a cheap `estimate_bloat(&str) -> f32` and the router can
use it to avoid slow or risky paths.

## SmartCrusher

Headroom's SmartCrusher is more than table rendering. It has four relevant
layers:

- Statistical analysis of JSON arrays:
  - field type, unique count, constant fields, numeric min/max/mean/variance
  - temporal field detection
  - change-point detection for numeric fields
  - pattern detection for time series, logs, search results, and generic rows
  - crushability analysis before row dropping
- Planning:
  - anchor selection
  - strategy-specific preservation for time series, clusters, top-N rows, and
    smart samples
  - error keyword and structural-outlier constraints
  - query-anchor and relevance scoring
  - optional learned preserve fields
- Tabular compaction:
  - schema is the union of keys, ordered by frequency then name
  - sparse fields are represented explicitly
  - heterogeneous arrays can be bucketed by discriminator fields
  - uniform nested objects can be flattened into dotted columns
  - stringified JSON can be parsed and represented as nested structure
  - opaque blobs can become CCR references
- Formatting:
  - render compact table or bucket IR
  - keep enough metadata to explain omitted rows

TinyJuice currently has a simpler table compressor with head/tail retention,
error preservation, and numeric outlier preservation. The next useful ingestion
is not a full port in one slice. It should start with analyzer and planner
interfaces so strategies can grow behind the existing `JsonCompressor`.

## Adaptive Sizing

Headroom's adaptive sizer computes how many items to keep by detecting
information saturation:

1. Fast path:
   - keep all items for small arrays
   - cluster near-duplicates with SimHash
2. Standard path:
   - build a cumulative unique-bigram coverage curve
   - run a Kneedle-style knee detector
   - use the knee as the natural keep count
3. Validation path:
   - compare zlib redundancy between the full set and retained subset
   - increase the keep count when the subset looks too redundant

This is a strong fit for TinyJuice's JSON compressor because it avoids fixed
head/tail constants such as `HEAD_ROWS` and `TAIL_ROWS`. TinyJuice should gate
it behind deterministic tests before making it the default.

## Anchor Selection

Headroom chooses positional anchors based on data pattern and query:

- search results are front-heavy
- logs are back-heavy
- time series keep both ends
- generic arrays spread anchors across the collection
- "latest", "recent", "current", and similar query words shift toward the
  back
- "first", "oldest", and similar query words shift toward the front
- middle anchors can be selected by information density
- duplicate rows are skipped by stable hashing

TinyJuice can use this to replace blind head/tail row preservation. The same
anchor machinery can also help search-output and log-output compressors.

## TextCrusher

Headroom's deterministic prose compressor is extractive:

- split text into sentence and line segments
- score each segment by recency, BM25 query relevance, and structural salience
- treat error words, digits, all-caps identifiers, and dotted identifiers as
  salient
- suppress near-duplicates through word shingles
- keep selected segments in original order up to a target ratio

This is safer than abstractive summarization because output consists of
verbatim segments. It is a good TinyJuice fallback for plain text when ML
compression is disabled or unavailable.

## Hybrid Relevance

Headroom combines BM25 and embedding relevance with adaptive weighting:

- exact identifiers, UUIDs, numeric IDs, hostnames, and emails increase the BM25
  weight
- when embeddings are unavailable, BM25 stays available with a small boost for
  matched terms
- batch scoring lets compressors rank rows or segments cheaply

TinyJuice can start with BM25-only relevance and leave embedding support behind
an explicit feature or adapter boundary. This matches the repository boundary
against adding heavyweight runtime dependencies to the core crate.

## Diff Noise

Headroom separates ordinary diff compression from a dedicated offload that
drops low-value diff regions:

- lockfile hunks
- generated bundle hunks
- whitespace-only hunks

It keeps file headers and emits a concise dropped-hunk reason, while stashing
the original diff in CCR. TinyJuice already summarizes lockfile hunks inside
`DiffCompressor`; a separate `DiffNoise` offload would make the policy tunable
and testable.

## Log Template Reformat

Headroom has a lossless log-template reformat that groups repeated log lines by
template and records variants. This differs from TinyJuice's current
signal-based log compressor:

- log-template reformat should be reconstructible without CCR
- signal compression is an offload because it drops low-priority lines and needs
  CCR

TinyJuice should add a `LogTemplate` reformat before the lossy signal path so
repetitive logs can shrink without retrieval.

## Tag Protection

Headroom protects workflow tags before ML compression:

- scan once through the input
- distinguish HTML tags from custom workflow tags
- replace custom-tag spans with placeholders before ML compression
- restore exact tags after compression
- handle nested tags, self-closing tags, and placeholder collisions

This addresses a real failure mode for OpenHuman-style prompts: ML compressors
may delete tags that downstream code parses as structure. TinyJuice should add
tag protection around the `ml_text` path before enabling broad plain-text
compression.

## Cache Alignment and Live Zone

Headroom's cache safety strategy is strict:

- cache-aligner detection warns about volatile system-prompt content but does
  not rewrite the prompt
- provider cache markers define a frozen prefix
- compression happens only in the live zone after the frozen prefix
- bytes outside rewritten blocks are copied from the original request rather
  than reserialized
- provider-specific walkers are used because Anthropic, OpenAI Chat, OpenAI
  Responses, Gemini, and Bedrock have different request shapes

TinyJuice is primarily a library, not a provider proxy, but OpenHuman adapters
should still expose a live-zone contract. This is the highest-safety ingestion
from Headroom because it prevents compression from invalidating prompt caches or
mutating user-visible history.

## CCR Backends

Headroom models CCR as a pluggable `CcrStore` with in-memory, SQLite, and Redis
backends. TinyJuice currently has a process-global memory store plus optional
disk tier. The Headroom contract suggests a cleaner split:

- in-memory store for tests and short sessions
- SQLite store for persistent local sessions
- Redis or remote store only behind an explicit feature or adapter
- fixed key format and marker format
- TTL and capacity policies per backend
- no silent fallback when a requested backend fails to initialize

TinyJuice should keep its current sensitive-data posture: no raw prompt or
context logging, and disk persistence must be explicit.

## Compression Policy

Headroom centralizes compression policy by auth mode and request economics:

- live-zone-only behavior
- cache aligner enablement
- volatile-content thresholds
- maximum lossy ratio
- read-only learning mode for requests where stability matters
- net-cost formula for mutating cached prefixes

TinyJuice can generalize this as a `CompressionPolicy` independent of auth mode.
OpenHuman adapters can map their runtime context onto the policy without adding
OpenHuman runtime dependencies to the crate core.

## Benchmarks and Safety Fixtures

Headroom has broad benchmark and adversarial test coverage. TinyJuice should not
import the claims, but it should copy the test strategy:

- CCR round-trip tests for every offload
- deterministic parity fixtures for table rendering and diff handling
- adversarial fixtures for tag protection and malformed input
- cache-boundary byte-fidelity tests for provider adapters
- latency budgets for bloat estimators
- quality fixtures that assert preservation of errors, IDs, changed lines,
  query matches, and structural outliers

