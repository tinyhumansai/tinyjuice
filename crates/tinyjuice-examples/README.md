# tinyjuice-examples

Live agent evaluations of TinyJuice's REPL tools on a
[tinyagents](https://github.com/tinyhumansai/tinyagents) harness. **Not part of the
release**: this crate is its own Cargo workspace (the root `Cargo.toml` excludes it), so
`cargo build` and `cargo test` at the repository root never compile or lock tinyagents.

tinyagents is vendored as the `vendor/tinyagents` submodule (with its own nested
`vendor/tinytools` and `vendor/tinyinference`). Initialise it with
`git submodule update --init vendor/tinyagents && git -C vendor/tinyagents submodule update --init vendor/tinytools vendor/tinyinference`.

```sh
OPENROUTER_API_KEY=... cargo run --release \
  --manifest-path crates/tinyjuice-examples/Cargo.toml -- \
  [--arm raw|summary|summary_fast|repl]... [--scenario NAME]... [--reps N] \
  [--model ID] [--summarizer-model ID]
```

Each run gives an agent one payload tool, with its output routed through TinyJuice per arm,
plus the `juice_*` tools for the `repl` arms. Payloads are synthetic and deterministic, each
with a planted fact, and the row reports whether the answer contained it. Arms:

| Arm | Tool output seen by the model |
| --- | --- |
| `raw` | untouched |
| `summary` | LLM summary, 3000 output tokens, no timeout (the old behaviour) |
| `summary_fast` | LLM summary, 1000 tokens, 8 s timeout, then preview plus handle |
| `repl` | preview plus handle, inspected with `juice_*`; no summarizer model |

Each run uses its own summary scope. TinyJuice's summary cache is process-global and keyed
by scope, tool, focus and payload, so a shared scope would serve later runs from earlier ones.
