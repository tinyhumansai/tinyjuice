# TinyJuice Roadmap

TinyJuice is starting as a blank Rust scaffold. The near-term work is to prove
the public boundaries before adding compression behavior.

## Milestones

1. Define the core compression trait, input model, output report, and error
   taxonomy.
2. Add token counting abstractions that can be supplied by OpenHuman or model
   provider adapters.
3. Implement the first deterministic compression strategy with fixture-backed
   tests.
4. Add OpenHuman adapter crates or feature-gated modules once the integration
   boundary is stable.
5. Build benchmark fixtures for compression ratio, latency, retained facts, and
   regression safety.
6. Document when aggressive 80-90% token reduction is appropriate and when it
   should be rejected.

## Non-Goals For The Initial Scaffold

- no production compression algorithm
- no model-provider network calls
- no OpenHuman runtime dependency
- no benchmark or quality claims
