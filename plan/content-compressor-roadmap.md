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
- Planner preserves dynamic head/tail anchors, query-relevant matches,
  structural sparse rows, numeric outliers/change points, discriminator values,
  duplicate and near-duplicate representatives, information-dense rows, and
  deterministic spread anchors.
- Adaptive spread anchor counts use deterministic semantic-bigram saturation.
- Nested uniform objects flatten into dotted columns.
- Stringified JSON objects are parsed and flattened when they are small and
  valid.

Add:

- Broader fixture coverage for all SmartCrusher planner paths.

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
- Exposes `DiffNoiseTransform` as a typed offload transform that emits lossy
  diff views only with a retained CCR token.
- Supports configurable noisy path substrings, optional whitespace-only hunk
  dropping, per-hunk reason markers, and droppable-body bloat estimates.

Add:

- Broader fixture coverage for host-tuned noisy path policy.

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
- Repetitive non-command logs use a lossless, reconstructible template reformat
  before signal offload.
- `LogTemplateTransform` exposes the template path as a typed reformat that
  runs without CCR.
- `SignalLogTransform` exposes signal-preserving log compression as a typed
  offload that emits output only with a retained CCR token.

Add:

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
- Scores query relevance through the shared BM25 scorer, falling back to line
  salience without a query.
- `SearchTransform` exposes search-result thinning as a typed offload that
  emits output only with a retained CCR token.
- Ranked search-read helpers score exact symbols, paths, match density,
  imports/exports, generated/vendor penalties, omitted counts, and merged
  snippet windows over host-provided matches.

Add:

- Better query context propagation.
- Broader fixture coverage for search-read host adapters.

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
- `HtmlExtractTransform` exposes lossy HTML-to-text extraction as a typed
  offload that emits output only with a retained CCR token.
- Web-extract reducer accepts already extracted text/markdown/HTML, strips
  inline base64 image payloads, preserves metadata, and keeps recovery footers
  attached to truncated pages.

Add:

- More complete entity support if fixtures show need.
- Broader fixture coverage for web-extract host formats.

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
- Deterministic extractive TextCrusher keeps verbatim spans scored by recency,
  BM25 query relevance, identifiers, numbers, and error/salience markers.
- Near-duplicate suppression uses word-term overlap.
- `TextCrusherTransform` exposes the deterministic path as a typed offload
  that emits output only with a retained CCR token.
- Custom tag protection wraps ML compression so protected workflow tags must
  survive byte-for-byte.

Add:

- Broader fixture coverage for TextCrusher scoring and ML tag protection.

Do not add:

- Abstractive summarization in core plain-text compressor.
- Default ML text compression without tag-protection fixtures.

Acceptance:

- Output consists only of verbatim spans.
- Custom workflow tags survive ML compression byte-for-byte.
- Plain data without useful signal declines rather than blindly truncating.
