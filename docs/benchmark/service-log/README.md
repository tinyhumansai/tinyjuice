# Docker Service Log

## Fixture

`docker_error_log` models a high-volume service log with thousands of routine
INFO lines, sparse upstream timeout errors, and sparse queue-depth warnings.

## Benchmark Result

| Metric | Value |
| --- | ---: |
| Original bytes | 295,945 |
| Compacted bytes | 563 |
| Estimated token reduction | 99.8% |
| Average latency | 1.110 ms |
| Signal checks | 3/3 |
| Task checks | 2/2 |
| CCR recovery | yes |

## Full Artifacts

- [Full input](full-input.txt)
- [Full output](full-output.txt)

## Input Sample

```text
2026-07-05T09:00:00Z INFO worker-0 handled request in 20ms
2026-07-05T09:01:00Z INFO worker-1 handled request in 21ms
2026-07-05T09:02:00Z INFO worker-2 handled request in 22ms
...
2026-07-05T09:17:12Z ERROR worker-7 request failed: upstream timeout request_id=req-977
...
2026-07-05T09:43:45Z warning: queue-depth high partition=3 depth=1903
...
2026-07-05T09:11:12Z ERROR worker-7 request failed: upstream timeout request_id=req-4311
```

## Output Sample

```text
2026-07-05T09:00:45Z warning: queue-depth high partition=0 depth=900
[... 976 line(s) omitted ...]
2026-07-05T09:17:12Z ERROR worker-7 request failed: upstream timeout request_id=req-977
[... 25 line(s) omitted ...]
2026-07-05T09:43:45Z warning: queue-depth high partition=3 depth=1903
[... omitted routine INFO lines ...]
2026-07-05T09:11:12Z ERROR worker-7 request failed: upstream timeout request_id=req-4311

[compacted tool output - this is a PARTIAL view; the full original is available
by calling tokenjuice_retrieve with the emitted token]
```

## Behind The Scenes

This is a non-command log blob, so TinyJuice uses signal-based log compression.
It classifies lines by severity and keeps errors, warnings, summaries, and stack
trace-like lines. Routine INFO lines are treated as low-value repeated noise.

The result is intentionally not a chronological miniature of the whole log. It
is an incident-focused view: the model sees the timeout worker, request IDs, and
warning class immediately, while the full raw log stays available in CCR.

## Human Review Notes

This report should be judged by incident usefulness, not by whether it preserves
normal request throughput. A reviewer should see the timeout worker, the warning
class, and the fact that routine INFO lines were omitted. The exact sequence of
INFO lines can be recovered through CCR if needed.
