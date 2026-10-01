# Repository Guidelines

TinyJuice is a Rust crate for pluggable token compression in OpenHuman. Keep the
scaffold small until real compression strategies are ready.

## Development

- Use stable Rust with edition 2024 support.
- Run `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, and
  `cargo test` before opening a PR.
- Keep public API changes documented in `README.md` or `docs/`.
- Prefer small modules with `types.rs` for shared data and `mod_tests.rs` for
  module-local tests.

## Boundaries

- Do not add OpenHuman runtime dependencies to the core crate without an
  explicit feature or adapter boundary.
- Do not claim compression percentages until benchmark fixtures exist.
- Treat prompt and context input as sensitive data. Avoid logging raw content in
  library code.

## Tests live in `*_tests.rs` files

- Unit tests are never inline. Do not write a `#[cfg(test)] mod tests { ... }`
  block in a source file. Put the tests in a sibling `<module>_tests.rs`
  (`mod_tests.rs` beside a `mod.rs`, `lib_tests.rs` beside `lib.rs`) and declare
  it at the bottom of the module:

  ```rust
  #[cfg(test)]
  #[path = "foo_tests.rs"]
  mod tests;
  ```

- The test file starts with `use super::*;` and carries no `#[cfg(test)]` of its
  own. It is still a child module, so it reaches private items exactly as an
  inline module did.
- Name test files `<module>_tests.rs`; a second group for the same module is
  `<module>_<topic>_tests.rs`. Never `test.rs`, `tests.rs` or `<module>_test.rs`.
- Integration tests stay in the crate's `tests/` directory.
- OpenHuman's `scripts/externalize-inline-tests.mjs <repo-root> --write` moves
  inline test modules out mechanically; without `--write` it only reports.
- Existing `test.rs` and `<module>_test.rs` files predate this rule. Rename each
  to `<module>_tests.rs` (keep its `mod` name, add the `#[path]` attribute) the
  next time you touch it.
