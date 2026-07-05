# Cargo Test Failure Log

## Fixture

`cargo_test_failure` models noisy `cargo test --all-targets` output with many
passing tests and one failing test with panic details.

## Benchmark Result

| Metric | Value |
| --- | ---: |
| Original bytes | 22,001 |
| Compacted bytes | 1,408 |
| Estimated token reduction | 93.6% |
| Average latency | 0.667 ms |
| Signal checks | 3/3 |
| Task checks | 2/2 |
| CCR recovery | yes |

## Full Artifacts

- [Full input](full-input.txt)
- [Full output](full-output.txt)

## Input Sample

```text
test integration::case_000 ... ok stdout: generated 50 rows
test integration::case_001 ... ok stdout: generated 51 rows
test integration::case_002 ... ok stdout: generated 52 rows
...
---- tests::panics_on_empty_payload stdout ----
thread 'tests::panics_on_empty_payload' panicked at src/compress.rs:91:9:
empty payload should be rejected before compression
stack backtrace:
   0: tinyjuice::compress::route
   1: tinyjuice::tool_integration::compact_tool_output_with_policy

failures:
    tests::panics_on_empty_payload

test result: FAILED. 359 passed; 1 failed; 0 ignored; finished in 3.42s
```

## Output Sample

```text
[... passing test output omitted ...]
---- tests::panics_on_empty_payload stdout ----
thread 'tests::panics_on_empty_payload' panicked at src/compress.rs:91:9:
empty payload should be rejected before compression
stack backtrace:
   0: tinyjuice::compress::route
   1: tinyjuice::tool_integration::compact_tool_output_with_policy

failures:
    tests::panics_on_empty_payload

test result: FAILED. 359 passed; 1 failed; 0 ignored; finished in 3.42s

[compacted tool output - this is a PARTIAL view; the full original is available
by calling tokenjuice_retrieve with the emitted token]
```

## Behind The Scenes

Because the benchmark passes command context (`cargo test --all-targets`), the
log compressor runs the command-aware rule engine rather than generic truncation.
That rule family treats failures as the primary signal.

The reducer removes repetitive success lines, keeps the failing test name, keeps
the panic location and reason, keeps stack frames, and keeps the final test
summary. The compacted view should still answer the operator questions: which
test failed, where did it panic, and why?

The dropped passing-test noise is recoverable through CCR.

## Human Review Notes

The compacted output should still answer the two operational questions that
matter first: which test failed, and what panic message explains the failure.
The repeated passing cases are not useful for first-pass triage, so they should
be summarized rather than retained line by line.
