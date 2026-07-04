# Pipeline And CCR Plan

## Goal

Make TinyJuice's core safety contract explicit: lossless reformats can run
without storage, while lossy offloads must prove recoverability before returning
model-facing output.

## Why This Is Needed

The current router enforces this at runtime by checking `CompressOutput.lossy`
and CCR retention before appending a footer. That is good, but not strong enough
for future growth. New compressor authors can misunderstand the contract, call
lower-level functions directly, or return lossy text through the wrong path.

## Proposed Types

Add a small `pipeline` module:

```rust
pub trait ReformatTransform {
    fn name(&self) -> &'static str;
    fn applies_to(&self, input: &PipelineInput<'_>) -> bool;
    fn apply(&self, input: &PipelineInput<'_>) -> Option<TransformOutput>;
}

pub trait OffloadTransform {
    fn name(&self) -> &'static str;
    fn estimate_bloat(&self, input: &PipelineInput<'_>) -> f32;
    fn apply(
        &self,
        input: &PipelineInput<'_>,
        store: &dyn CcrStore,
    ) -> Option<OffloadOutput>;
}
```

`OffloadOutput` should include a verified CCR token. It should not be possible
to construct an offload output from public API without a token.

## CcrStore Trait

Introduce:

```rust
pub trait CcrStore: Send + Sync {
    fn put(&self, content: &str) -> CcrPutResult;
    fn get(&self, token: &str) -> Option<String>;
    fn get_range(&self, token: &str, start: usize, end: usize, unit: RangeUnit)
        -> Option<String>;
}
```

Default implementations:

- `MemoryCcrStore`
- `DiskCcrStore` or `MemoryWithDiskCcrStore`
- `GlobalCcrStore` compatibility wrapper over current `cache` functions

Keep existing `cache::offload_checked`, `cache::retrieve`, and
`cache::retrieve_range` as compatibility wrappers until callers migrate.

## Pipeline Report

Add a report shape that can be returned by new APIs and converted into existing
`CompressedOutput`:

```rust
PipelineReport {
  content_kind,
  original_bytes,
  compacted_bytes,
  applied_steps,
  skipped_steps,
  ccr_tokens,
  lossy,
  skip_reason,
}
```

Skip reasons must not include raw content.

## Migration Plan

### Step 1: Add Store Trait Without Rewiring Router

- Add `CcrStore` and wrappers around current global store.
- Move token validation into reusable store helpers.
- Add isolated tests using an in-memory store.

### Step 2: Add Pipeline Compatibility Wrapper

- Build a pipeline path that can run current compressors as compatibility
  transforms.
- Keep `compress_content()` and `route()` signatures intact.
- Assert current behavior is unchanged through existing fixtures.

### Step 3: Convert Existing Compressors Gradually

- JSON small-array table rendering becomes a reformat when no rows are omitted.
- JSON row dropping becomes an offload.
- HTML extraction is an offload.
- Diff context collapsing is an offload.
- Log template compression will be a reformat.
- Signal-log compression is an offload.
- Search thinning is an offload.
- Code stubbing is an offload unless the caller explicitly accepts a stub view.

### Step 4: Add Bloat Estimation

Add cheap estimators for:

- JSON array redundancy and row count
- diff context dominance and noisy-file share
- repeated log templates and signal density
- search match clustering
- HTML markup density
- plain-text segment redundancy

The router can use estimators to decide whether to try transforms and to report
why a transform was skipped.

## Acceptance Criteria

- Existing public APIs continue to compile.
- Lossy output cannot be emitted from the new offload path without a token.
- Reformat-only compression works with CCR disabled.
- Offload compression declines if CCR cannot retain the original.
- Recovery tool outputs still bypass compaction.
- Disk-backed retrieval rejects malformed tokens and path traversal attempts.
- Tests cover no-op, reformat-only, offload-only, mixed transforms, oversized
  CCR rejection, and disk-tier retrieval.

## What Not To Do

- Do not add Redis or SQLite stores in the first slice.
- Do not remove current global cache functions until OpenHuman migration is
  complete.
- Do not log raw content in pipeline reports.
- Do not make all compressors implement the new traits in one large PR.

