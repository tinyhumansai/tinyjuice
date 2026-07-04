# Repository Guidelines

TinyJuice is a Rust crate for pluggable token compression in OpenHuman. Keep the
scaffold small until real compression strategies are ready.

## Development

- Use stable Rust with edition 2024 support.
- Run `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, and
  `cargo test` before opening a PR.
- Keep public API changes documented in `README.md` or `docs/`.
- Prefer small modules with `types.rs` for shared data and `test.rs` for
  module-local tests.

## Boundaries

- Do not add OpenHuman runtime dependencies to the core crate without an
  explicit feature or adapter boundary.
- Do not claim compression percentages until benchmark fixtures exist.
- Treat prompt and context input as sensitive data. Avoid logging raw content in
  library code.
