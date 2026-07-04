# Web Extract Truncate-Store Spec

## Status

Design reference for web extraction output reduction. This is a specification
only; TinyJuice does not yet implement a web extraction adapter.

## Purpose

Long web pages can consume a model context window even after an extraction
backend has already removed boilerplate. TinyJuice should support a deterministic
path for these pages:

- return small extracted pages whole;
- replace inline base64 image blobs with placeholders;
- return a bounded head/tail window for large pages;
- store the full extracted text in a recoverable artifact;
- include an explicit retrieval footer in the model-facing text.

This is not semantic summarization. It does not ask an auxiliary model to decide
what is important. It is a reversible omission pattern for already-clean web
content.

## Source Notes

This spec is based on a merged Hermes Agent web extraction change and its public
web-search documentation:

- PR: `https://github.com/NousResearch/hermes-agent/pull/54843`
- Docs: `https://hermes-agent.nousresearch.com/docs/user-guide/features/web-search`

The public docs page still describes the older auxiliary-model summarization
path for long pages. The merged PR replaces that path with truncate-and-store.
TinyJuice should treat the merged code behavior as the newer design reference.

## Target Behavior

For each extracted page:

1. Accept provider-cleaned markdown or text from an upstream extractor.
2. Convert inline base64 image data to compact placeholders.
3. If the cleaned text length is within the configured character budget, return
   it whole.
4. If it exceeds the budget, store the full cleaned text and return a head/tail
   window plus a footer that tells the agent how to retrieve the omitted middle.
5. Preserve result metadata such as URL, title, error state, and truncation
   metrics without logging raw page content.

## Non-Goals

- Do not build a web scraper inside TinyJuice core.
- Do not replace extraction providers such as Firecrawl, Tavily, Exa, Parallel,
  browser snapshots, or host-owned fetch tools.
- Do not make compression percentage, speedup, or cost claims until TinyJuice has
  benchmark fixtures.
- Do not summarize page content with an LLM in the default path.
- Do not store raw web artifacts unless the host has enabled an artifact store or
  CCR-backed recovery mode.

## Inputs

```text
WebExtractReduceInput {
  url: String,
  title: Option<String>,
  content: String,
  format: WebExtractFormat,       // markdown | text | html
  provider: Option<String>,
  char_limit: Option<usize>,
  metadata: Map<String, Value>,
}
```

Batch adapters may send multiple page records:

```text
WebExtractBatchInput {
  pages: Vec<WebExtractReduceInput>,
  default_char_limit: usize,
  max_combined_inline_chars: usize,
  store_policy: StorePolicy,
}
```

## Configuration

Default config:

```toml
[tokenjuice.web_extract]
enabled = true
char_limit = 15000
min_char_limit = 2000
max_char_limit = 500000
head_ratio = 0.75
store_full_text = true
convert_base64_images = true
max_combined_inline_chars = 100000
```

Rules:

- `char_limit` is per page, not per batch.
- Values below `min_char_limit` clamp upward so the footer does not dominate the
  output.
- Values above `max_char_limit` clamp downward to prevent accidental context
  blowups.
- `max_combined_inline_chars` guards multi-URL extracts where each individual
  result is under the page limit but the batch is still too large.

## Algorithm

```text
reduce_web_extract(page, options):
  clean = normalize_provider_text(page.content)
  clean = replace_inline_base64_images(clean)

  limit = clamp(
    page.char_limit.or(options.char_limit),
    options.min_char_limit,
    options.max_char_limit
  )

  if len(clean) <= limit:
    return WholePage(clean)

  stored_ref = store_full_text(page.url, clean, options.store_policy)
  head_budget = floor(limit * options.head_ratio)
  tail_budget = limit - head_budget

  head = clean[0:head_budget]
  tail = clean[len(clean)-tail_budget:]
  head = snap_head_to_previous_line_boundary(head)
  tail = snap_tail_to_next_line_boundary(tail)

  return TruncatedPage {
    text: head + omission_marker + tail + retrieval_footer(stored_ref),
    ref: stored_ref,
    original_chars: len(clean),
    inline_chars: len(text),
  }
```

Line-boundary snapping should be best-effort:

- snap the head cut back to the last newline only when it does not discard most
  of the head budget;
- snap the tail cut forward to the next newline only when it does not discard
  most of the tail budget;
- fall back to raw character cuts for single-line content.

## Image Handling

Inline base64 images are token hazards and should never be returned as raw model
context.

Transformations:

```text
![alt](data:image/png;base64,AAAA...) -> [IMAGE: alt]
![](data:image/jpeg;base64,AAAA...)  -> [IMAGE]
(data:image/gif;base64,AAAA...)      -> [IMAGE]
data:image/gif;base64,AAAA...        -> [IMAGE]
```

Real `http://` and `https://` image links should remain intact so the host agent
can decide whether to inspect them with a vision or browser tool.

## Storage

TinyJuice should reuse the artifact-store design from
`tokenjuice-improvement-spec.md` rather than writing arbitrary filesystem paths
from core compressor code.

Stored full-text records should include:

```text
WebExtractArtifact {
  id: ArtifactId,
  source_url_hash: String,
  source_host: String,
  content_sha256: String,
  content_bytes: usize,
  content_chars: usize,
  format: WebExtractFormat,
  created_at: Timestamp,
  expires_at: Option<Timestamp>,
}
```

Storage requirements:

- Artifact ids must be generated by TinyJuice, not derived directly from URLs.
- URL hostnames may appear in display metadata, but paths and filenames should
  be hash-based or strictly sanitized.
- Disk storage must use private file permissions where the platform supports
  them.
- A failed store must degrade to a truncated result with a clear footer saying
  that full recovery is unavailable.
- Disk-backed retention should have size and TTL controls.

## Retrieval Footer

The model-facing footer should be explicit, stable, and host-adapter friendly.

Example:

```text
-------- [TRUNCATED] --------
Showing 11,250 chars (head) + 3,750 chars (tail) of 85,420 total clean characters.
Full text artifact: tj-web-01H...
To read the omitted middle: tinyjuice_retrieve id="tj-web-01H..." offset=<line> limit=<n>
-----------------------------
```

If the host exposes file-based retrieval instead of an artifact API, the adapter
may render a host-native call such as `read_file path="..." offset=<line>
limit=<n>`. TinyJuice core should prefer opaque artifact ids so it does not leak
local cache paths into model context by default.

Footer requirements:

- Include original size and displayed head/tail sizes.
- Include a retrieval handle only when storage succeeded.
- Include a concrete first `offset` when possible. The referenced PR used a
  placeholder offset; TinyJuice should improve this by counting the head's line
  count and pointing the first recovery call at the omitted region.
- Mark footer text so downstream reducers do not compact it away.

## Batch Handling

For multi-URL extraction:

1. Reduce each page independently.
2. Preserve per-page URL, title, error state, and artifact id.
3. Apply a combined inline budget after per-page reduction.
4. If the combined result is still too large, rank pages by host-provided order
   and shrink lower-priority pages to smaller windows before dropping footers.
5. Never drop a retrieval footer for a page whose middle was omitted.

The combined-budget guard avoids a case where `char_limit = 50000` across five
URLs produces a response that exceeds the host's tool-output cap and loses the
trailing retrieval instructions.

## TinyJuice Fit

This should be a host adapter plus compressor, not a provider integration:

```text
web extractor provider -> host web_extract tool -> TinyJuice web reducer
                        -> artifact store / CCR -> model-facing result
```

Candidate Rust layout:

```text
src/compressors/web_extract.rs
src/artifacts.rs
src/types.rs
docs/references/web-extract-truncate-store-spec.md
```

Adapter contract:

- OpenHuman or another host performs URL fetching and secret-safe URL validation.
- TinyJuice receives only extracted text and metadata.
- TinyJuice returns compacted page records and artifact references.
- The host exposes retrieval through `tokenjuice_retrieve`, `read_file`, or a
  host-native artifact read tool.

## Safety Rules

- Do not fetch URLs from TinyJuice core.
- Do not log raw extracted content.
- Do not log full URLs when they may contain tokens or signed query strings;
  store a hash and sanitized host display value instead.
- Preserve page-level errors and safety-block results.
- Preserve real image URLs but omit embedded image bytes.
- Decline lossy omission when storage is required and unavailable, unless the
  caller explicitly allows unrecoverable truncation.

## Acceptance Criteria

- Small extracted pages return byte-for-byte after base64 image conversion.
- Large pages return a head/tail window with a stable truncation footer.
- The omitted middle is present in the artifact store when a retrieval handle is
  emitted.
- Base64 image blobs are absent from output and stored metadata.
- Real markdown image URLs are preserved.
- Invalid `char_limit` values fall back to the default.
- Too-small `char_limit` values clamp to `min_char_limit`.
- Huge `char_limit` values clamp to `max_char_limit`.
- Multi-page output cannot exceed `max_combined_inline_chars` by silently
  truncating away retrieval footers.
- Store failures produce deterministic degraded output without panics.

## Open Questions

- Should web artifacts share the CCR store, the operator artifact store, or both?
- Should retrieval be line-based, byte-based, or both?
- Should HTML inputs route through the existing HTML compressor before the web
  reducer, or should web extraction always normalize to markdown/text first?
- Should the host be allowed to mark trusted provider output as already free of
  base64 image data and skip the image scan?
