# Content Compressor Roadmap

## Goal

Improve existing compressors behind the current content-router model while
preserving recoverability, deterministic behavior, and OpenHuman-safe defaults.

## JSON SmartCrusher

Current behavior:

- Parses JSON array of objects.
- Renders a union-key table.
- Keeps all rows for small arrays.
- For large arrays, keeps fixed head/tail windows plus error rows and numeric
  outliers.
- Heterogeneous object arrays can render as smaller per-shape bucket tables,
  preserving original row indices and using CCR recovery when bucket rows are
  dropped.
- Full-present constant columns are hoisted into table metadata instead of
  repeated in every rendered row.
- Analyzer reports field frequencies, types, unique counts/ratios, constants,
  sparse fields, numeric stats, estimated table bytes, and rough estimated
  reduction.

Add:

- Planner for dynamic anchors, query matches, structural outliers, discriminator
  buckets, and duplicate clusters.
- Adaptive keep count using deterministic saturation/knee detection.
- Heterogeneous-array handling through buckets.
- Nested uniform object flattening into dotted columns.
- Stringified JSON parsing where safe.

Do not add:

- Non-deterministic learned planning in core.
- Claims about JSON savings before fixtures.

Acceptance:

- Existing JSON tests still pass.
- Fixtures cover homogeneous rows, sparse rows, heterogenous buckets, nested
  objects, stringified JSON, constants, outliers, errors, and duplicates.
- Omitted rows are recoverable through CCR.

## Diff Compression And DiffNoise

Current behavior:

- Keeps structural lines and changed lines.
- Collapses long unchanged context.
- Summarizes noisy lockfile/bundle hunks inside `DiffCompressor`.

Add:

- Separate `DiffNoise` offload transform.
- Configurable suffix list for lockfiles and generated bundles.
- Optional whitespace-only hunk dropping.
- Per-hunk omission reasons.
- Bloat estimate based on droppable body-byte share.

Do not add:

- Whitespace-only dropping by default without fixtures.
- Semantic diff interpretation in core.

Acceptance:

- Lockfile hunks drop with clear reason markers.
- Whitespace-only hunks drop only when paired plus/minus lines normalize equal.
- Semantic hunks are preserved.
- Original diff is retrievable.

## Log Compression

Current behavior:

- Command output uses the rule reducer.
- Non-command logs use signal preservation for errors, warnings, summaries, and
  stack traces.

Add:

- Lossless log-template reformat before signal offload.
- Reconstructible template blocks and variants.
- More command-rule parity for common build systems and CI outputs.
- GitHub Actions failing-step summaries through rule metadata.

Do not add:

- Signal compression for data that has no log signal.
- Raw log persistence unless CCR/artifact settings allow it.

Acceptance:

- Repetitive logs shrink without CCR.
- Logs with no useful template pass to signal compression or decline.
- A fixture can reconstruct original lines from template output.

## Code And AST Stub Reads

Current behavior:

- Tree-sitter path for Rust, TypeScript/JavaScript, and Python behind feature.
- Brace-depth heuristic fallback.
- Collapses function/method bodies.

Add:

- Explicit stub modes: signatures-only, public API, matched symbols, and
  expand-around-lines.
- Elision metadata with line ranges.
- Parse status in reports.
- Host intent that distinguishes exact read from stub read.
- Additional language parsers only behind features.

Do not add:

- Default stubbing of exact file reads.
- Claims that stub output is compilable.

Acceptance:

- Public declarations remain intact.
- Requested symbols can be expanded.
- Parse failure is reported and falls back safely.

## Search Compression

Current behavior:

- Parses grep/ripgrep-style `path:line:body`.
- Groups by file.
- Keeps top K matches per file.
- Scores query-term density or line salience.

Add:

- Shared BM25 scorer.
- Better query context propagation.
- Path and generated/vendor penalties for search-read adapter.
- Omitted match counts by file and global total.
- Snippet window merging for host-side search-read.

Do not add:

- Filesystem search in core compressor.
- Dropping retrieval footers for compressed search output.

Acceptance:

- Exact identifier matches survive.
- Query-aware output remains deterministic.
- Omitted counts are explicit.

## HTML And Web Text

Current behavior:

- Single-pass HTML-to-text extractor.
- Drops scripts, styles, head, noscript, and SVG.

Add:

- More complete entity support if fixtures show need.
- Web-extract reducer as an adapter-facing module for already extracted text.
- Base64 image stripping for web extraction outputs.

Do not add:

- URL fetching or scraping in core TinyJuice.
- DOM dependencies in default build unless a fixture justifies them.

Acceptance:

- HTML extraction never panics on malformed input.
- Web extract output preserves URL/title/error metadata from host.
- Full page recovery exists when a footer is emitted.

## Plain Text

Current behavior:

- Optional ML text compressor through host callback.
- ML disabled by default.
- Generic fallback only runs for command output.

Add:

- Deterministic extractive TextCrusher.
- Segment scoring by recency, BM25 query relevance, identifiers, numbers, and
  error/salience markers.
- Near-duplicate suppression with word shingles.
- Tag protection around ML compression before enabling broader use.

Do not add:

- Abstractive summarization in core plain-text compressor.
- Default ML text compression without tag-protection fixtures.

Acceptance:

- Output consists only of verbatim spans.
- Custom workflow tags survive ML compression byte-for-byte.
- Plain data without useful signal declines rather than blindly truncating.
