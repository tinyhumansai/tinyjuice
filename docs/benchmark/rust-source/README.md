# Rust Source File

## Fixture

`rust_source_large_file` models a Rust source file with imports, a public data
type, a TODO marker, and a long function body.

## Benchmark Result

| Metric | Value |
| --- | ---: |
| Original bytes | 4,187 |
| Compacted bytes | 486 |
| Estimated token reduction | 88.6% |
| Average latency | 0.199 ms |
| Signal checks | 3/3 |
| Task checks | 2/2 |
| CCR recovery | yes |

## Full Artifacts

- [Full input](full-input.txt)
- [Full output](full-output.txt)

## Input Sample

```rust
use std::collections::HashMap;

// TODO: preserve anomaly rows during future scoring changes.

pub struct CompressionJob {
    pub id: String,
    pub bytes: usize,
}

pub fn compress_payload(job: CompressionJob) -> Result<String, String> {
    let intermediate_0 = job.bytes.saturating_add(0);
    let intermediate_1 = job.bytes.saturating_add(1);
    let intermediate_2 = job.bytes.saturating_add(2);
    ...
    let intermediate_69 = job.bytes.saturating_add(69);
    Ok(format!("{}:{}", job.id, job.bytes))
}
```

## Output Sample

```rust
use std::collections::HashMap;

// TODO: preserve anomaly rows during future scoring changes.

pub struct CompressionJob {
    pub id: String,
    pub bytes: usize,
}

pub fn compress_payload(job: CompressionJob) -> Result<String, String> { ... 72 line(s) collapsed }

[compacted tool output - this is a PARTIAL view; the full original is available
by calling tokenjuice_retrieve with the emitted token]
```

## Behind The Scenes

With tree-sitter enabled, TinyJuice parses Rust source and collapses large
function bodies while preserving top-level structure. Imports, comments with
important markers, public type declarations, and function signatures remain
visible.

This gives an agent the navigation surface first: what APIs exist, what types
they use, and which TODO/error markers matter. The body can be recovered through
CCR if the agent needs exact implementation details.

## Human Review Notes

The compacted source should be enough for orientation. A reviewer should still
see the import, TODO, public struct, and public function signature. The body is
not destroyed; it is intentionally hidden behind a collapse marker and CCR
recovery because the long repetitive body is lower-value context for first pass
navigation.
