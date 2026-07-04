# TokenJuice Improvement Ingestion Spec

## Purpose

This spec records what TinyJuice can ingest from the original
`vincentkoc/tokenjuice` codebase and what TinyJuice has already improved in the
Rust/OpenHuman port.

Source baseline:

- Original: `https://github.com/vincentkoc/tokenjuice`, commit
  `49bdcf1755833ff1e02e44e6e7fe91c0fb44c16e`, release `v0.8.1`.
- Local TinyJuice review point: commit
  `d8c95921a8b722ca4b551710e3ccf7f862bdbecc` plus uncommitted doc work in this
  checkout.

## Current Delta

The original TokenJuice repository is a TypeScript CLI and adapter suite. It
ships a deterministic reducer, `reduce-json` machine protocol, command wrapper,
artifact store, rule validator, rule fixtures, host installers, doctor commands,
and many host-specific integration docs.

TinyJuice has already moved the reusable engine into Rust and added
OpenHuman-specific behavior that upstream does not have:

- content-aware router for JSON, HTML, code, logs, search output, diffs, command
  output, and optional ML text compression
- CCR recovery markers and a bounded memory/disk store for reversible lossy
  compaction
- per-agent compression profiles (`auto`, `full`, `light`, `off`)
- OpenHuman-facing tool-output APIs and startup configuration hooks
- savings attribution and cost-dashboard hooks
- optional tree-sitter dependencies behind a feature boundary
- Rust-native rule loading with builtin, user, and project overlay layers

The main gap is product surface and parity hardening: TinyJuice has the reducer
core, but not the full CLI/install/doctor ecosystem from upstream.

## Ingest Priorities

### P0: Rule Inventory Parity

TinyJuice currently vendors 100 rule JSON files. Upstream has 130 non-fixture
rule files. Missing upstream rules:

```text
build/cmake
build/dotnet
build/go-build
build/gradle
build/maven
build/msbuild
build/pnpm-build
build/swift-build
build/xcodebuild
devops/helm
devops/pulumi
devops/terraform
devops/terragrunt
filesystem/fd
filesystem/git-ls-files
filesystem/rg-files
git/diff
git/worktree-list
install/npm-ci
openclaw/sessions-history
package/composer
package/fnm
package/npm-ls
service/pm2
task/env
task/mise
task/node
task/php
task/python
task/ruby
task/uv
tests/rspec
tests/swift-test
text/wc
```

TinyJuice has four Rust/OpenHuman-specific rules not present upstream:

```text
build/cargo-build
build/cargo-doc
lint/cargo-clippy
lint/cargo-fmt
```

Spec:

- Add a rule sync script that imports upstream rule files into
  `src/vendor/rules`, preserving TinyJuice-specific rules.
- Keep the flattened filename convention (`git__status.json`) unless the crate
  moves to nested embedded includes.
- Add a generated manifest with upstream commit, rule id, source path, and local
  file path.
- Add fixture parity tests for every imported rule before claiming savings.
- Treat rule JSON as deterministic behavior. Do not rewrite rules ad hoc during
  import except for schema compatibility fixes documented in the manifest.

Acceptance:

- `load_builtin_rules()` sees every expected rule id.
- Every imported upstream fixture either passes or has an explicit skipped
  reason.
- `generic/fallback` still sorts last.

### P0: Machine Protocol Parity

Upstream's most useful integration boundary is `reduce-json`: a stable JSON
contract for host adapters. TinyJuice already has compatible core types but does
not expose a complete CLI/protocol surface.

Spec:

- Define a Rust `ReduceJsonRequest` with both accepted shapes:
  - direct `ToolExecutionInput`
  - `{ "input": ToolExecutionInput, "options": ReduceOptions }`
- Expand `ReduceOptions` to match upstream protocol fields where they make
  sense:
  - `classifier`
  - `maxInlineChars`
  - `raw`
  - `noOmit`
  - `store`
  - `storeDir`
  - `cwd`
  - `trace`
  - `recordStats`
- Reject malformed JSON with structured errors and no raw content logging.
- Return stable JSON that includes inline text, stats, classification, optional
  preview, optional compaction metadata, optional trace, and optional artifact or
  CCR references.

Acceptance:

- Golden tests cover direct payload, envelope payload, malformed input, NUL byte
  rejection, raw mode, and max-inline clamping.
- The protocol is documented before adapters consume it.

### P0: Safety Policy Port

Upstream has a safe-inventory policy for host adapters: file-content reads stay
raw, repository inventory commands may compact, and mixed or unsafe shell
commands stay raw. TinyJuice currently has a simpler file-content check.

Spec:

- Port command identity and shell-safety helpers:
  - `stripLeadingCdPrefix`
  - shell tokenization with quote handling
  - sequential command detection
  - unquoted pipe splitting
  - inventory command classification for `find`, `fd`, `ls`, `tree`,
    `rg --files`, and `git ls-files`
  - unsafe action detection such as `find -exec` and `fd --exec`
  - safe stdin-only filters for `head`, `tail`, `sort`, `uniq`, `wc`, and safe
    `sed` inventory filters
- Make host adapters choose one policy:
  - `compact-all`
  - `skip-all`
  - `skip-file-content`
  - `allow-safe-inventory`
- Preserve explicit raw bypasses.

Acceptance:

- Exact file content reads pass through unchanged.
- `find . -type f | sort | head` may compact.
- `find . -exec cat {} \;`, mixed shell sequences, and unsafe pipelines pass
  through unchanged.

### P1: CLI Surface

Upstream ships a complete CLI with `reduce`, `reduce-json`, `wrap`, `install`,
`uninstall`, `doctor`, `verify`, `discover`, `stats`, `ls`, and `cat`.

TinyJuice should expose a Rust CLI in phases:

1. `tinyjuice reduce`
2. `tinyjuice reduce-json`
3. `tinyjuice wrap`
4. `tinyjuice verify --rules --fixtures`
5. `tinyjuice ls` and `tinyjuice cat <id>`
6. `tinyjuice stats`
7. `tinyjuice discover`
8. `tinyjuice doctor`
9. `tinyjuice install <host>` and `tinyjuice uninstall <host>`

Keep the core crate host-agnostic. CLI code should live behind a binary target or
a separate companion crate so library users do not inherit installer concerns.

Acceptance:

- CLI exits non-zero on invalid input and never panics on bad user payloads.
- `--raw` bypasses reducer compaction.
- `--no-omit` disables omission markers where supported.
- `--max-capture-bytes` and `--max-input-bytes` are explicit and documented.

### P1: Artifact Store and Stats

TinyJuice has CCR for recovery, but upstream also has an operator-facing
artifact store for raw output, metadata, discovery, and stats.

Spec:

- Add a file artifact store separate from CCR:
  - default: `~/.tinyjuice/artifacts`
  - override: `TINYJUICE_ARTIFACT_DIR` or `storeDir`
  - file mode `0600`, directory mode `0700`
  - id format with a fixed prefix and strict validation
- Store metadata-only records when `recordStats` is enabled without raw storage.
- Track source, command, exit code, raw chars, reduced chars, ratio,
  classification, capture truncation, and created time.
- Keep CCR recovery tokens and artifact ids separate. CCR is for reversible
  model-facing compaction; artifacts are for operator inspection.

Acceptance:

- Invalid artifact ids cannot escape the artifact directory.
- Stats can be computed without storing raw prompt/tool content.
- Raw artifact storage remains opt-in.

### P1: Structured Summaries and Metadata

Upstream includes specialized summaries for GitHub Actions failures, inspection
commands, whole JSON output, `gh` tables, git diffs, git status, and search
results. TinyJuice has some rewrites, but not the full metadata model.

Spec:

- Port compaction metadata for omitted lines, omitted chars, clamped head/tail,
  and authoritative footer hints.
- Port GitHub Actions failure summarization for generic fallback output.
- Port whole-JSON compaction as a fallback before generic head/tail.
- Complete `git/diff`, `search/rg`, and `cloud/gh` rewrite parity.
- Add `trace` output for reducer debugging.

Acceptance:

- Common GitHub Actions logs produce a failing-step summary instead of generic
  head/tail.
- Valid JSON output compacts structurally.
- Debug traces show normalized command, reducer command, matched reducer, and
  family without including sensitive raw content.

### P1: Rule Validation and Discovery

Upstream validates rule schemas, detects duplicate ids, reports shadowing, and
has fixture verification. TinyJuice should formalize this in Rust.

Spec:

- Add a Rust validator for `JsonRule`.
- Add `verify_rules()` with builtin, user, and project roots.
- Report:
  - invalid JSON
  - schema/type errors
  - invalid regex
  - duplicate ids within a layer
  - shadowed builtin/user rules
- Add a discovery report that surfaces frequent `generic/fallback` artifacts and
  suggests candidate rule families.

Acceptance:

- `cargo test` covers invalid regex, duplicate ids, shadow warnings, and
  project-over-user-over-builtin precedence.

### P2: Host Installer Framework

Upstream contains many host adapters. TinyJuice should not blindly port all of
them into the core crate, but it should ingest the framework pattern.

Spec:

- Create a host registry with these capabilities:
  - install
  - uninstall
  - doctor
  - runtime hook command, where applicable
  - local binary path override for release testing
- Shared helper modules:
  - hooks JSON editing
  - instruction-file marker insertion
  - pre-tool command wrapping
  - post-tool output replacement
  - hook command doctoring
  - idempotent backup/restore
- First-class hosts:
  - OpenHuman runtime
  - Codex
  - Claude Code
  - Cursor
  - OpenCode
  - pi
  - GitHub Copilot CLI / VS Code Copilot
- Beta hosts can be docs-only until tested.

Acceptance:

- Installers preserve unrelated config.
- Re-running install is idempotent.
- Doctor reports `ok`, `warn`, `broken`, or `disabled` plus one repair command.

### P2: Packaging

Upstream ships npm and release tooling. TinyJuice should use Rust-native
packaging, but the release surface should be similarly easy to consume.

Spec:

- Publish the crate and a `tinyjuice` binary.
- Provide shell installers for macOS/Linux.
- Add Homebrew formula generation after the binary stabilizes.
- Keep Node/npm wrappers optional and thin if needed for JS hosts.

Acceptance:

- Non-Rust hosts can call `tinyjuice reduce-json` without linking Rust.
- Rust hosts can depend on the crate directly.

## Existing TinyJuice Improvements to Preserve

Do not regress these local improvements while ingesting upstream behavior:

- The core crate must not depend on OpenHuman runtime internals.
- CCR retrieval must remain token-validated and path traversal safe.
- Disk tier disable must stop future disk writes immediately.
- Re-offloading identical content must refresh TTL.
- Recovery tool outputs must never be compacted again.
- Agent `light` profile must disable lossy CCR-backed compression.
- Public API changes need README or docs updates.
- Compression percentage claims remain forbidden until benchmark fixtures exist.

## Licensing Notes

The original TokenJuice repository is MIT licensed. TinyJuice is GPL-3.0-only.
Directly copied source, rules, fixtures, or docs need attribution and license
notice preservation. Prefer mechanical import with a source manifest over
hand-copying snippets into unrelated files.

## Rollout Plan

1. Land rule parity and fixture verification.
2. Land `reduce-json` request/response protocol.
3. Land safety policy helpers and host-policy tests.
4. Land artifact metadata and stats.
5. Land minimal CLI: `reduce`, `reduce-json`, `verify`.
6. Add `wrap`, `ls`, `cat`, `stats`, and `discover`.
7. Add host installer framework and first OpenHuman/Codex adapters.

Each phase should be independently shippable and should keep `cargo fmt`,
`cargo clippy --all-targets -- -D warnings`, and `cargo test` green.
