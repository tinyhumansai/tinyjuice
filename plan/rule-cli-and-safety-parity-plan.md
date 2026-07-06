# Rule CLI And Safety Parity Plan

## Goal

Bring TinyJuice to TokenJuice product parity where it matters for safe
integration: rule coverage, fixture validation, machine protocol, CLI entry
points, and safe shell policy.

## P0: Rule Inventory Parity

Current state:

- TinyJuice vendors 101 built-in rules.
- The reference spec says upstream TokenJuice has 130 non-fixture rules.
- TinyJuice also has Rust/OpenHuman-specific rules.

Plan:

- Add a rule sync script in a later implementation slice.
- Preserve the flattened filename convention under `src/vendor/rules`.
- Generate a manifest with upstream commit, rule id, source path, local path,
  and any schema compatibility notes.
- Handle licensing in the sync: upstream TokenJuice is MIT while TinyJuice is
  GPL-3.0-only. Imported rules and fixtures need attribution and license
  preservation in the manifest; prefer mechanical import with a recorded
  source commit over hand-edited copies.
- Preserve TinyJuice-specific rules.
- Add tests that assert expected rule ids are present and `generic/fallback`
  sorts last.

Acceptance:

- Every imported rule has either a passing fixture or an explicit skipped
  reason.
- Duplicate ids are reported during verification.
- Project rules still override user rules, which still override built-ins.

## P0: Safe Shell Policy

Current state:

- Exact file reads are only guarded in the generic rule reducer.
- Content-router extension hints can still trigger code or JSON compression.

Plan:

- Add command identity helpers for shell commands and tool arguments.
- Strip leading `cd` prefixes where safe.
- Detect mixed sequential commands.
- Split unquoted pipes.
- Classify read-only inventory commands: `find`, `fd`, `ls`, `tree`,
  `rg --files`, `git ls-files`.
- Detect unsafe actions such as `find -exec`, `fd --exec`, redirects, and
  mutation-looking subcommands.
- Add host policy modes:
  - `compact-all`
  - `skip-all`
  - `skip-file-content`
  - `allow-safe-inventory`

Acceptance:

- `cat file.rs`, `sed -n 1,200p file.rs`, and `jq . file.json` stay raw by
  default.
- `find . -type f | sort | head` may compact.
- `rg --files` may compact.
- `find . -exec cat {} \;` stays raw.
- Mixed shell sequences stay raw unless explicitly allowed.

Status: implemented in the reducer and router policy paths. The default
`allow-safe-inventory` policy keeps exact file-content reads raw, rejects mixed
shell sequences and unsafe inventory actions, and allows safe inventory
pipelines such as `find . -type f | sort | head`. The rule fixture suite now
covers that safe-inventory path through the reducer.

## P0: Reduce-Json Protocol

Plan:

- Add request types for direct input and envelope input.
- Support `classifier`, `maxInlineChars`, `raw`, `noOmit`, `store`,
  `storeDir`, `cwd`, `trace`, and `recordStats` where meaningful.
- Return stable JSON with inline text, preview, stats, classification, metadata,
  trace, and optional CCR/artifact refs.
- Reject malformed payloads with structured errors.
- Never log raw content on parse errors.

Acceptance:

- Golden tests cover direct payload, envelope payload, malformed input, raw
  mode, max-inline clamping, and invalid classifier.
- Protocol docs exist before OpenHuman or non-Rust hosts depend on it.

Status: partially implemented. The library now exposes `ReduceJsonRequest`,
`ReduceJsonEnvelope`, `ReduceJsonResponse`, `ReduceJsonError`,
`reduce_json_str()`, and `reduce_json_request()`. The protocol accepts direct
and envelope payloads, supports the compatibility option fields, rejects
malformed JSON and NUL bytes with structured errors, and returns stable
serde-compatible response fields with optional metadata-only trace. The current
library contract is documented in `docs/reduce-json-protocol.md`. CLI wiring,
CCR/artifact refs, and durable stats recording remain to be implemented.

## P1: CLI Surface

Minimal order:

1. `tinyjuice reduce`
2. `tinyjuice reduce-json`
3. `tinyjuice verify --rules --fixtures`
4. `tinyjuice wrap`
5. `tinyjuice ls` and `tinyjuice cat <id>`
6. `tinyjuice stats`
7. `tinyjuice discover`
8. `tinyjuice doctor`
9. host installers

Implementation guidance:

- Put CLI behind a binary target or companion crate.
- Keep installer concerns out of the core library.
- Exit non-zero on invalid input.
- Pass through raw content on reducer failure unless the CLI command explicitly
  asks for validation.

Status: partially implemented. The crate now builds a dependency-free
`tinyjuice` binary with `reduce`, `reduce-json`, `verify`, `discover`, `wrap`,
`ls`, `cat`, `stats`, and `doctor` commands.
`reduce` accepts stdin or one file plus command metadata flags and prints
reduced inline text. `reduce-json` accepts the documented protocol payload from
stdin or one file, prints response JSON, and exits non-zero with structured
error JSON on invalid payloads. `verify --rules --fixtures` reports rule and
fixture diagnostics and exits non-zero for hard verification failures.
`discover` accepts a JSON array or NDJSON `ToolExecutionInput` stream and emits
metadata-only fallback family counts. `wrap` runs a command after `--`, reduces
captured stdout/stderr with exit-code metadata, and exits with the wrapped
command's status. `ls`, `cat`, and `stats` operate on an explicit CCR disk
tier via `--store-dir`; `cat` supports line and byte ranges, while `stats`
reports metadata-only token counts and byte totals. `doctor` emits structured
health JSON with `ok`, `warn`, `broken`, and `disabled` checks for built-in
rules, optional fixture verification, optional explicit CCR disk store
inspection, and host-aware `codex`, `openhuman`, and aggregate `hooks` targets.
`install codex` and `uninstall codex` maintain a TinyJuice-managed Codex
instruction block idempotently while preserving unrelated text. OpenHuman and
other host install/uninstall mutation remains a future CLI slice.

## P1: Rule Validation And Discovery

Plan:

- Add `verify_rules()` that checks built-in, user, and project roots.
- Report invalid JSON, schema errors, invalid regex, duplicate ids, and
  shadowed rules.
- Add fixture verification for rule examples.
- Add discovery reports for frequent `generic/fallback` outputs.

Acceptance:

- Validation is deterministic and does not panic on bad user files.
- Discovery reports command families and counts without raw tool output.

Status: implemented for the library surface. `verify_rules()` now reports parse
errors, invalid regex patterns, duplicate rule ids, and shadowed lower-priority
rules across the same builtin/user/project roots as `load_rules()`, without
changing the lenient runtime loader. Current diagnostics show the 101 built-ins
parse without duplicate ids and include 11 counter regexes that Rust `regex`
drops because they use lookahead. `verify_rule_fixtures()` walks
`*.fixture.json` examples, runs them through compiled rules, and reports pass
counts, parse errors, and hash-only output mismatches.
`discover_fallback_outputs()` groups command families that still classify to
`generic/fallback` while omitting raw tool output.

## What Not To Do

- Do not implement host installers before `doctor` can verify them.
- Do not silently compact CI logs by default.
- Do not turn every shell output into generic head/tail truncation.
- Do not store raw artifacts unless explicitly requested.
