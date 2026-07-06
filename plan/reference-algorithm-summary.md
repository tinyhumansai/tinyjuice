# Reference Algorithm Summary

## TokenJuice

TokenJuice contributes the deterministic command-output reducer model:

- normalize shell/tool execution inputs
- classify commands against JSON rules
- apply line filters, counters, transforms, and head/tail summaries
- support built-in, user, and project rule overlays
- expose a `reduce-json` protocol for non-Rust hosts
- provide CLI commands, wrappers, validation, discovery, stats, installers, and
  doctor flows

TinyJuice already ports the reducer core and much of the rule model. The most
important remaining TokenJuice work is parity hardening: missing rules, fixture
verification, `reduce-json`, artifact metadata, and CLI product surface. Safe
shell inventory policy is now implemented in the Rust path.

## Headroom

Headroom contributes the strongest architectural contracts:

- distinguish lossless `ReformatTransform` from CCR-backed `OffloadTransform`
- estimate domain bloat before applying expensive or risky transforms
- protect provider cache boundaries through live zones
- keep frozen prefix bytes byte-identical
- expose pluggable CCR stores
- use adaptive JSON planning instead of fixed head/tail constants
- add deterministic TextCrusher and BM25 relevance
- protect custom workflow tags before ML compression

TinyJuice already has Headroom-inspired compressors for JSON, code, logs, diffs,
search, HTML, TextCrusher/BM25, pipeline reports, injectable CCR stores, and
the reformat/offload transform split. Remaining Headroom work is hardening with
fixtures and exposing the report model through stable product surfaces, not a
wholesale port.

## Hermes

Hermes is mainly a conversation-compaction reference, not a replacement for
TinyJuice's content router. Its useful algorithms are:

- threshold and token-budget helpers
- deterministic old-tool-output pruning
- protected head and token-budgeted tail selection
- tool-call/tool-result boundary alignment
- latest-user and latest-assistant anchoring
- provider-safe JSON argument shrinking
- structured handoff summaries
- summary failure policy
- prompt-cache hints and stable static-prefix cache keys
- live-zone byte-range contracts for frozen prefixes and mutable blocks
- context usage breakdowns

This should enter TinyJuice as an optional conversation adapter layer. It should
not be folded into `compress_content` or made mandatory for core compression.

## Web Extract Truncate-Store

This spec proposes a deterministic, recoverable path for long extracted web
pages:

- strip inline base64 image payloads
- return small extracted pages whole
- truncate large pages to a line-snapped head/tail window
- store full text in an artifact store or CCR-compatible retrieval layer
- preserve page metadata and retrieval footers

TinyJuice should not fetch URLs. The host should fetch and clean content, then
pass extracted text and metadata into a reducer.

## Ranked Search And Selective Read

This spec reduces exploratory tool calls by combining search, ranking, and
snippet selection:

- rank files by symbol matches, path matches, density, imports/exports, and
  generated/vendor penalties
- return bounded snippets with path and line metadata
- record omitted matches and file counts

Part of this exists in `src/compressors/search.rs`, but a full search-read
operation belongs in a host adapter because it touches the filesystem.

## AST Stub Reads

This spec reduces full-file reads by returning source structure:

- imports, exports, declarations, signatures, and selected comments
- elided function/method bodies with placeholders
- optional expansion around requested symbols or line ranges
- parser fallback reporting

TinyJuice has tree-sitter-backed code compression behind a feature plus explicit
stub modes, elision metadata, line numbers, parse-status reporting, and
OpenHuman host policy that keeps exact reads byte-exact by default.

## Batched Edit And Validation

This is not text compression. It reduces retry loops:

- group edits by file
- apply exact or carefully bounded fuzzy matches
- reject ambiguous matches
- validate changed files
- return compact validation evidence instead of full post-edit files

Core TinyJuice can define reports and validation result types. Filesystem
mutation should remain in host adapters.

## SQL Introspection Reduction

This spec replaces raw schema dumps with compact schema subgraphs:

- parse migrations or accept host-extracted schema data
- rank tables and columns against the query
- return connected schema facts, indexes, keys, and capped query rows
- report safe dialect rewrites

Database drivers should not enter core by default. The core can handle already
extracted schema IR and compact row rendering.

## Subagent Summaries

Subagent summaries reduce delegated exploration transcripts:

- prefer final conclusions when evidence is present
- preserve concrete findings, file paths, line numbers, and uncertainty
- drop dead-end exploration unless it explains uncertainty

This needs an adapter-level summary contract. Core should define data shapes and
deterministic evidence extraction, not choose or require a model.

## Savings Accounting

Savings accounting is infrastructure, not compression:

- distinguish counted, measured, and estimated savings
- track bytes, rough tokens, measured usage, costs, elapsed time, and calls
- avoid raw content in records

TinyJuice has class-labeled savings records for live and fixture-benchmark
sources. UI claims should still wait for fixture benchmark reports and measured
provider usage.

## TurboQuant Vector Compression

TurboQuant-style vector compression applies to embedding/index storage:

- quantize fixed-width float vectors
- preserve approximate dot product and cosine behavior
- keep embedding generation separate

This is not prompt compression. It should be deferred or isolated behind a
feature/module for memory and recall indexes.
