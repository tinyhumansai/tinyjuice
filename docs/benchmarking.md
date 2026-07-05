# TinyJuice Benchmarking

TinyJuice has two benchmark surfaces:

- `cargo bench` runs Criterion hot-path benchmarks for router and rule-engine throughput.
- `cargo run --release --example compression_benchmark -- --iterations 20` runs the real-snapshot corpus and reports byte ratio, estimated token ratio, latency, compressor choice, inline accuracy, CCR recovery, and named gate failures.

By default, the fixture suite emits metadata only. It does not print raw prompt,
tool, or context payloads unless `--dump-samples` is explicitly requested.
Human-readable before/after samples live under [`docs/benchmark`](benchmark/README.md).

## Corpus Benchmark

Refresh the real sample corpus and Markdown reports:

```sh
scripts/benchmark/update-real-samples.sh
cargo run --release --example compression_benchmark -- --iterations 20 --format json > /tmp/tinyjuice-corpus-benchmark.json
scripts/benchmark/render-reports.sh /tmp/tinyjuice-corpus-benchmark.json
```

The updater writes 10 cases per category under `docs/benchmark/<category>/cases/`.
It uses the adjacent OpenHuman checkout, live OpenHuman Docker logs when
available, public RSS/page snapshots when reachable, and checked-in OpenHuman
artifacts as fallbacks.

Default Markdown report:

```sh
cargo run --release --example compression_benchmark -- --iterations 20
```

Machine-readable report:

```sh
cargo run --release --example compression_benchmark -- --iterations 20 --format json
```

Write the full input and output artifacts used by the reports:

```sh
cargo run --release --example compression_benchmark -- --dump-samples docs/benchmark
```

Every case can carry two inline accuracy layers:

- Signal checks: structural facts that should survive compaction, such as error lines, changed diff lines, schema columns, or script removal.
- Task checks: small pinned questions that must be answerable from the compacted inline text, such as "Which test failed?" or "What config value changed?"

Lossy fixtures also verify recovery correctness by retrieving the CCR token and byte-comparing the result to the original input.

Current categories cover:

- JSON tool-catalog slices.
- Vitest command output.
- Runtime and Docker-style service logs.
- Ripgrep result sets.
- Unified diffs.
- RSS feeds, noisy HTML pages, forum pages, and coverage HTML.
- Rust source files.
- Plain text, which should pass through while ML text compression is disabled.

Use the corpus runner for regression comparisons and candidate release notes. Only market reductions that also pass inline accuracy and CCR recovery gates. Do not turn one local run into a public production-wide claim; claims need pinned corpora, quality gates, and hardware/runtime metadata.

## External Benchmarks To Cross-Reference

These are useful comparison targets, but they measure prompt/context compression for downstream LLM task quality rather than TinyJuice's recoverable tool-output compaction directly:

- [LLMLingua](https://arxiv.org/abs/2310.05736) evaluates prompt compression across GSM8K, BBH, ShareGPT, and Arxiv-March23.
- [LongLLMLingua](https://arxiv.org/abs/2310.06839) focuses on long-context prompt compression and reports NaturalQuestions, LooGLE, and long-context latency/cost evaluations.
- [Selective Context](https://aclanthology.org/2023.emnlp-main.391/) evaluates pruning redundant context on arXiv papers, news articles, and long conversations for summarization, question answering, and response generation.
- [LongBench](https://arxiv.org/abs/2308.14508) is a broader long-context benchmark that can be useful once TinyJuice adds task-level answer-quality evaluation.

For TinyJuice, the closest future cross-reference is not a single paper score. It is a paired report: compression metadata from this fixture runner plus downstream task accuracy on pinned tool-output corpora.
