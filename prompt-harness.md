# Harness Prompt: TinyJuice Test, Fixture, And Benchmark Work

You are working in the **TinyJuice** repository: a pure-Rust crate (edition
2024, no Python, no JS in the library — ignore `interface/`, it is a separate
self-hosted analytics UI). All verification is `cargo`-driven. The test,
fixture, and benchmark scaffolding already exists and is green; your job is to
extend it alongside implementation work, never to bolt tests on afterwards.

## Toolchain And Commands

- Build/test: stable Rust via `cargo`. No Python, no npm for the crate.
- `cargo test` — everything: 285+ unit tests (in `src/`, `*_tests.rs` and
  `#[cfg(test)]` modules), plus the integration binaries under `tests/`.
- `cargo test --test fixtures` — golden fixture runner only.
- `cargo test --test e2e_tool_output` — end-to-end hook tests only.
- `cargo bench` — criterion benchmarks (`benches/compression.rs`). For a
  quick smoke run:
  `cargo bench --bench compression -- --warm-up-time 0.3 --measurement-time 0.5 --sample-size 10`.
- Gates before every commit (all must be clean):
  `cargo fmt --check` && `cargo clippy --all-targets -- -D warnings` &&
  `cargo test`. If you add/remove features, also test
  `--no-default-features` (drops tree-sitter).

## Test Architecture (already in place)

```text
src/**/*_tests.rs, #[cfg(test)]   unit tests, colocated with the code
tests/common/mod.rs               shared helpers (fixture loader, config)
tests/fixtures.rs                 golden-output runner over tests/fixtures/
tests/fixtures/*.fixture.json     recorded tool executions + expected output
tests/e2e_tool_output.rs          full-path tests through the OpenHuman hook
tests/scaffold.rs                 legacy public-API smoke test (keep passing)
benches/compression.rs            criterion throughput benches (harness=false)
```

### Fixture contract

A fixture is one JSON file in `tests/fixtures/`, camelCase, shaped as:

```json
{
  "description": "what behavior this pins and why",
  "input": { "toolName": "bash", "argv": ["git", "status"], "stdout": "...", "exitCode": 0 },
  "expectedOutput": "exact reduced text (trailing whitespace ignored)"
}
```

`input` deserializes into `tinyjuice::types::ToolExecutionInput`; the runner
drives `reduce_execution_with_rules` with builtin rules and compares
`inline_text` exactly (modulo trailing whitespace). **Every new rule, every
rule bugfix, and every reducer behavior change gets a fixture** — that is the
cheap, reviewable regression mechanism. Name files
`<family>__<case>.fixture.json` (e.g. `git__status_modified.fixture.json`).

### E2E contract

`tests/e2e_tool_output.rs` exercises `compact_tool_output_with_policy` — the
exact entry point OpenHuman's middleware calls. It pins the plan's acceptance
criteria: `Off` and recovery-tool output are byte-exact passthrough, `Light`
never returns a lossy view or CCR marker, `Full` compacts recoverably with a
`cache::retrieve` roundtrip restoring the original. Extend this file when you
change router, profile, CCR, or footer behavior.

**Global-state rule:** `tinyjuice::configure` and the CCR cache are
process-global and tests run in parallel threads. Every e2e test must call
`common::install_test_config()` (installs identical defaults, so concurrent
installs cannot race into different states). Never install test-specific
`CompressOptions` in a parallel test; if a test truly needs different options,
pass them through APIs that take options as a parameter (`route`,
`compress_content`) instead of mutating global config.

### Benchmark contract

`benches/compression.rs` measures **throughput of hot paths** on
deterministic synthetic payloads (JSON rows, repetitive logs, plain-text
decline, rule reduction). It is NOT a compression-ratio benchmark — ratio and
savings claims require a separate fixture-backed report (see
`plan/openhuman-algorithm-port-plan.md`, savings accounting) and must not be
derived from these numbers. Add a bench case when you add a compressor or
touch the router hot path. Known baseline worth preserving: the router
declines plain text in ~70 µs at ~370 MiB/s, and `load_builtin_rules` costs
~11 ms per call (rule compilation is expensive — cache compiled rules, never
reload per invocation in hot paths).

## What To Work On

The backlog is `plan/openhuman-algorithm-port-plan.md` (priority-ordered,
with acceptance criteria per slice) supported by
`plan/openhuman-integration-plan.md` (verified OpenHuman state + known bugs)
and `prompt.md` (goals, constraints, trip hazards). Encode each slice's
acceptance criteria as tests in the layer that fits:

- rule/reducer behavior → fixture JSON
- router/profile/CCR/footer behavior → `tests/e2e_tool_output.rs`
- pure helpers (pipeline, conversation, policy modules) → colocated unit
  tests, property-style where the invariant allows (e.g. boundary alignment)
- performance-relevant paths → a criterion case

Immediate test-infrastructure gaps you may fill opportunistically:

- Only 3 fixtures exist for a 100-rule builtin catalog. When touching a rule
  family, add fixtures for it. A `verify --rules --fixtures` command is
  planned (`plan/rule-cli-and-safety-parity-plan.md`) — fixtures you add now
  feed it.
- The e2e suite does not yet cover: disk-tier CCR retrieval, ranged
  retrieval (`retrieve_range`), malformed-token rejection, or the
  footer-vs-truncation contract (that contract is P0-2b in `prompt.md`).
- No CI workflow exists in this repo. If asked to add one: GitHub Actions,
  stable toolchain, the three gates above, plus `cargo bench --no-run` to
  keep benches compiling.

## Hard Constraints (same as prompt.md, restated)

- Feature branch, never `main`; small focused commits.
- Core stays free of OpenHuman runtime types and heavy default deps.
- Lossy output must be CCR-recoverable or declined; recovery-tool output is
  never re-compacted; exact reads stay byte-exact by default.
- No raw prompt/tool/CCR content in logs, stats, or accounting records —
  fixtures under `tests/fixtures/` are the only place raw sample content
  lives, and samples must be synthetic.
- No compression-percentage claims anywhere until a fixture-backed benchmark
  report exists.
- Public API changes require doc updates (`README.md` or `docs/`).

## Reporting

After each slice: which tests prove which acceptance criteria, gate results
(`fmt`/`clippy`/`test` actual output, not "should pass"), bench deltas if a
hot path changed, and any plan-doc corrections discovered while implementing —
update the plan file in the same commit series.
