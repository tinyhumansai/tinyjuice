# Contributing To TinyJuice

Thanks for helping build TinyJuice. This project is early, so the best
contributions are small, explicit, tested, and easy to review.

## Development Setup

Install a stable Rust toolchain with Rust 2024 support, then run:

```sh
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo build --all-targets
cargo test
```

The bundled example should also run:

```sh
cargo run --example passthrough
```

## Project Philosophy

TinyJuice should make token compression explicit and inspectable. Prefer:

- small modules with narrow responsibilities
- typed state and typed errors
- deterministic compression behavior around model-facing context
- public APIs that are easy to test
- reports that make compression ratios and strategy choices visible
- examples that show concrete compression behavior rather than abstract promises

New module directories should keep shared type definitions in `types.rs` and
module-local unit tests in `test.rs`. Integration tests belong in `tests/`.

## Pull Request Checklist

Before opening a pull request:

- run `cargo fmt --check`
- run `cargo clippy --all-targets -- -D warnings`
- run `cargo build --all-targets`
- run `cargo test`
- add or update tests for behavior changes
- update docs when public APIs, architecture, or examples change
- keep the PR focused on one logical change

## Commit Style

Use concise imperative commit subjects, for example:

```text
Add compression target validation tests
Document OpenHuman adapter boundary
```

Avoid mixing formatting, refactors, and behavior changes unless they are
inseparable.

## Issue Triage

Good issues include:

- the TinyJuice version or commit
- the relevant module or API
- a minimal code example when behavior is surprising
- expected behavior
- actual behavior
- commands run locally

Feature requests should explain the agent workflow they unlock, the public API
shape they imply, and any safety, privacy, or quality concerns.

## Security

Do not report vulnerabilities through public issues. Use the process in
[SECURITY.md](SECURITY.md).
