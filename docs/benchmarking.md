# TinyJuice Benchmarking

TinyJuice has two benchmark surfaces:

- `cargo bench` runs Criterion hot-path benchmarks for router and rule-engine throughput.
- `cargo run --release --example compression_benchmark -- --iterations 20` runs a deterministic fixture suite and reports byte ratio, estimated token ratio, latency, compressor choice, inline accuracy, CCR recovery, and named gate failures.

The fixture suite emits metadata only. It does not print raw prompt, tool, or context payloads.

## Fixture Benchmark

Default Markdown report:

```sh
cargo run --release --example compression_benchmark -- --iterations 20
```

Machine-readable report:

```sh
cargo run --release --example compression_benchmark -- --iterations 20 --format json
```

Every fixture can carry two inline accuracy layers:

- Signal checks: structural facts that should survive compaction, such as error lines, changed diff lines, schema columns, or script removal.
- Task checks: small pinned questions that must be answerable from the compacted inline text, such as "Which test failed?" or "What config value changed?"

Lossy fixtures also verify recovery correctness by retrieving the CCR token and byte-comparing the result to the original input.

Current fixtures cover:

- JSON API inventories with rare anomaly rows.
- Cargo/test command output with a failure and panic details.
- High-volume service logs with sparse errors and warnings.
- Ripgrep-style search results.
- Unified diffs with long unchanged context.
- HTML status pages.
- Rust source files with long function bodies.
- Plain text, which should pass through while ML text compression is disabled.

Use the fixture runner for regression comparisons and candidate release notes. Only market reductions that also pass inline accuracy and CCR recovery gates. Do not turn one local run into a public production-corpus claim; claims need pinned corpora, quality gates, and hardware/runtime metadata.

## External Benchmarks To Cross-Reference

These are useful comparison targets, but they measure prompt/context compression for downstream LLM task quality rather than TinyJuice's recoverable tool-output compaction directly:

- [LLMLingua](https://arxiv.org/abs/2310.05736) evaluates prompt compression across GSM8K, BBH, ShareGPT, and Arxiv-March23.
- [LongLLMLingua](https://arxiv.org/abs/2310.06839) focuses on long-context prompt compression and reports NaturalQuestions, LooGLE, and long-context latency/cost evaluations.
- [Selective Context](https://aclanthology.org/2023.emnlp-main.391/) evaluates pruning redundant context on arXiv papers, news articles, and long conversations for summarization, question answering, and response generation.
- [LongBench](https://arxiv.org/abs/2308.14508) is a broader long-context benchmark that can be useful once TinyJuice adds task-level answer-quality evaluation.

For TinyJuice, the closest future cross-reference is not a single paper score. It is a paired report: compression metadata from this fixture runner plus downstream task accuracy on pinned tool-output corpora.
