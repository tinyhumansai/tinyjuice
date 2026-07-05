# Benchmark Sample Reports

This folder contains human-reviewable before/after samples for each fixture in
the compression benchmark. The root benchmark table is intentionally compact;
these reports show what the model actually receives after TinyJuice routes and
compresses each input type.

The examples use deterministic generated fixtures, not production data. Long
inputs are excerpted so the docs stay readable. Lossy examples include a CCR
recovery footer in real benchmark output; the footer means the compacted text is
a partial inline view and the exact original is still retrievable by token.

## Reports

| Report | Full input | Full output |
| --- | --- | --- |
| [JSON SmartCrusher](json-smartcrusher/README.md) | [input](json-smartcrusher/full-input.txt) | [output](json-smartcrusher/full-output.txt) |
| [Cargo test failure log](cargo-test-log/README.md) | [input](cargo-test-log/full-input.txt) | [output](cargo-test-log/full-output.txt) |
| [Docker service log](service-log/README.md) | [input](service-log/full-input.txt) | [output](service-log/full-output.txt) |
| [Ripgrep search results](search-results/README.md) | [input](search-results/full-input.txt) | [output](search-results/full-output.txt) |
| [Unified diff](unified-diff/README.md) | [input](unified-diff/full-input.txt) | [output](unified-diff/full-output.txt) |
| [HTML status report](html-status-report/README.md) | [input](html-status-report/full-input.txt) | [output](html-status-report/full-output.txt) |
| [Rust source file](rust-source/README.md) | [input](rust-source/full-input.txt) | [output](rust-source/full-output.txt) |
| [Plain text pass-through](plain-text/README.md) | [input](plain-text/full-input.txt) | [output](plain-text/full-output.txt) |

## What To Look For

For each report, review whether the compacted output keeps the useful working
surface:

- the error, warning, anomaly, changed line, or public API signal is still
  visible;
- repeated low-value noise is collapsed or summarized;
- the output says when rows, lines, or context were omitted;
- lossy reports keep the exact original recoverable through CCR;
- pass-through reports stay unchanged when TinyJuice should not compress.
