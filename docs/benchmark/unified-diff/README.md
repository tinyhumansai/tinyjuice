# Unified Diff

## Fixture

`unified_diff_router_change` models a patch with long unchanged context around a
small configuration change.

## Benchmark Result

| Metric | Value |
| --- | ---: |
| Original bytes | 6,128 |
| Compacted bytes | 964 |
| Estimated token reduction | 84.3% |
| Average latency | 0.008 ms |
| Signal checks | 3/3 |
| Task checks | 1/1 |
| CCR recovery | yes |

## Full Artifacts

- [Full input](full-input.txt)
- [Full output](full-output.txt)

## Input Sample

```diff
diff --git a/src/config.rs b/src/config.rs
index 1111111..2222222 100644
--- a/src/config.rs
+++ b/src/config.rs
@@ -12,90 +12,90 @@ pub fn default_options() -> CompressOptions {
     unchanged_config_line_0: true,
     unchanged_config_line_1: true,
     unchanged_config_line_2: true,
     ...
-    ccr_enabled: false,
+    ccr_enabled: true,
     trailing_context_line_0: None,
     trailing_context_line_1: None,
     ...
```

## Output Sample

```diff
diff --git a/src/config.rs b/src/config.rs
index 1111111..2222222 100644
--- a/src/config.rs
+++ b/src/config.rs
@@ -12,90 +12,90 @@ pub fn default_options() -> CompressOptions {
     unchanged_config_line_0: true,
     unchanged_config_line_1: true,
     unchanged_config_line_2: true,
[... 74 context line(s) omitted ...]
-    ccr_enabled: false,
+    ccr_enabled: true,
     trailing_context_line_0: None,
     trailing_context_line_1: None,
     trailing_context_line_2: None,
[... 74 context line(s) omitted ...]

[compacted tool output - this is a PARTIAL view; the full original is available
by calling tokenjuice_retrieve with the emitted token]
```

## Behind The Scenes

The diff compressor keeps patch structure: file headers, index lines, hunk
headers, and changed lines. Long runs of unchanged context are collapsed to a
small anchor plus an omission marker.

That preserves what review agents usually need first: which file changed, which
hunk changed, and the exact added/removed lines. The surrounding context remains
available via CCR if the agent needs to inspect it.

## Human Review Notes

The changed lines must be exact. A reviewer should confirm that file headers,
hunk headers, and both the removed and added `ccr_enabled` lines are still
visible. The omitted context markers are acceptable because they describe what
was dropped and the full diff is recoverable.
