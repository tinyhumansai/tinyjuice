# Plain Text Pass-Through

## Fixture

`plain_text_decline` models natural-language notes while ML text compression is
disabled.

## Benchmark Result

| Metric | Value |
| --- | ---: |
| Original bytes | 30,490 |
| Compacted bytes | 30,490 |
| Estimated token reduction | 0.0% |
| Average latency | 0.000 ms |
| Signal checks | 1/1 |
| Task checks | 1/1 |
| CCR recovery | n/a |

## Full Artifacts

- [Full input](full-input.txt)
- [Full output](full-output.txt)

## Input Sample

```text
Decision record 40: keep benchmark inputs deterministic, avoid raw context logging,
and compare compressed output against named signal checks before making any claims.

Decision record 41: keep benchmark inputs deterministic, avoid raw context logging,
and compare compressed output against named signal checks before making any claims.

Decision record 42: keep benchmark inputs deterministic, avoid raw context logging,
and compare compressed output against named signal checks before making any claims.
```

## Output Sample

```text
Decision record 40: keep benchmark inputs deterministic, avoid raw context logging,
and compare compressed output against named signal checks before making any claims.

Decision record 41: keep benchmark inputs deterministic, avoid raw context logging,
and compare compressed output against named signal checks before making any claims.

Decision record 42: keep benchmark inputs deterministic, avoid raw context logging,
and compare compressed output against named signal checks before making any claims.
```

## Behind The Scenes

TinyJuice detects plain text, but the ML text compressor is disabled by default.
There is no deterministic structural compressor for this input type, so the
router declines compression and returns the original unchanged.

This is intentional. A safe compression layer should know when not to act. The
benchmark checks that `Decision record 42` survives because the whole payload is
passed through.

## Human Review Notes

This report is the control case. A reviewer should expect no clever reduction:
the safest behavior is to return the text unchanged. This keeps TinyJuice from
pretending that arbitrary prose can be compressed deterministically when the ML
plain-text path is disabled.
