# Ripgrep Search Results

## Fixture

`ripgrep_many_matches` models a large ripgrep-style result set for the query
`tokenjuice recover compression`.

## Benchmark Result

| Metric | Value |
| --- | ---: |
| Original bytes | 11,910 |
| Compacted bytes | 2,945 |
| Estimated token reduction | 75.3% |
| Average latency | 0.034 ms |
| Signal checks | 2/2 |
| Task checks | 1/1 |
| CCR recovery | yes |

## Full Artifacts

- [Full input](full-input.txt)
- [Full output](full-output.txt)

## Input Sample

```text
150 matches across 6 files
src/compress.rs:10:ordinary tokenjuice mention number 0 in compression path
src/cache.rs:11:ordinary tokenjuice mention number 1 in compression path
src/tool_integration.rs:12:ordinary tokenjuice mention number 2 in compression path
...
src/cache.rs:83:recover exact original when tokenjuice compression emits a footer
...
docs/architecture.md:159:ordinary tokenjuice mention number 149 in compression path
```

## Output Sample

```text
150 matches across 6 files
[search: 150 match(es) across 6 file(s) - top 5 per file - full set via retrieve footer]
src/cache.rs:83:recover exact original when tokenjuice compression emits a footer
src/cache.rs:119:ordinary tokenjuice mention number 109 in compression path
[+20 more match(es) in src/cache.rs]
src/compress.rs:10:ordinary tokenjuice mention number 0 in compression path
[+20 more match(es) in src/compress.rs]
...

[compacted tool output - this is a PARTIAL view; the full original is available
by calling tokenjuice_retrieve with the emitted token]
```

## Behind The Scenes

TinyJuice parses `path:line:body` search matches, groups them by file, and
ranks matches inside each file. When the caller provides a query, lines with
more query-term density rank higher.

The output keeps the best matches per file and adds a per-file "more matches"
tally. That lets an agent decide which file to open next without spending
context on every low-value repeated hit.

The full search result set is lossy in the inline view but recoverable through
CCR.

## Human Review Notes

The output should help an agent choose a promising next file to open. The
important recovery-related hit is retained, and each file group reports how many
additional matches were omitted. That is the tradeoff: keep relevance and file
coverage, drop exhaustive repetition.
