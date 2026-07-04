# TinyJuice Integration Spec

## Purpose

This spec defines how TinyJuice should integrate cleanly across OpenHuman and
adjacent TinyHumansAI codebases without copying reducer logic into every host.

The goal is one shared compression engine with thin adapters:

- Rust hosts call the `tinyjuice` crate.
- Non-Rust hosts call a stable `tinyjuice reduce-json` CLI protocol.
- Agent runtimes get the same safety policy, bypass behavior, recovery model,
  and observability everywhere.

## Integration Principles

- Tool semantics are never changed. TinyJuice observes output after execution or
  wraps execution only when a host lacks post-tool hooks.
- Raw prompt and context content is sensitive. Do not log raw content in library
  code, installer code, or doctor output.
- Exact file-content reads stay raw unless the caller explicitly opts in.
- Lossy compaction must be reversible through CCR or declined.
- Host adapters own host wiring. TinyJuice core owns classification, reduction,
  caching, metadata, and safety policy.
- Configuration is mapped at host boundaries. The core crate does not import
  OpenHuman runtime configuration types.

## Target Surfaces

### OpenHuman Desktop / Tauri Core

Use the Rust crate directly.

Primary hook:

- after tool output is captured and credentials are scrubbed
- before output is appended to conversation context
- before cost accounting finalizes prompt tokens

Recommended API:

```rust
compact_tool_output_with_policy(
    tool_name,
    arguments.as_ref(),
    output,
    exit_code,
    agent_tokenjuice_profile,
).await
```

Responsibilities:

- map OpenHuman config into `CompressOptions`
- call `install_config()` at startup and whenever runtime config changes
- expose a `tokenjuice_retrieve` tool for CCR recovery
- include compaction stats in internal telemetry without raw content
- surface per-agent `auto`, `full`, `light`, and `off` profiles

### OpenHuman Runtime Python Server

Use TinyJuice through a narrow boundary. Do not port reducer logic to Python.

Preferred options:

- call the `tinyjuice` binary with `reduce-json`
- or call a Rust-backed FFI/sidecar only if process startup cost becomes a
  measured problem

Responsibilities:

- pass tool name, command, argv, stdout/stderr/combined text, exit code, cwd, and
  metadata
- preserve raw bypass flags
- keep Kompress or ML backends as optional compressors behind feature/config
  gates

### Agent Harnesses and CLI Tool Loops

For Rust harnesses such as TinyAgents/RustAgents, depend on the crate directly.
For TypeScript, Python, or shell-based harnesses, call `tinyjuice reduce-json`.

Integration point:

- post-tool output replacement where possible
- pre-tool wrapping only when the host cannot rewrite output after execution

Required behavior:

- pass through non-shell tool outputs unless the adapter has enough metadata for
  safe content classification
- apply the safe-inventory policy for shell commands
- preserve host-native error and exit-code reporting

### CI and Developer Tooling

Use the CLI in explicit workflows, not hidden build steps.

Useful commands after the CLI exists:

```sh
tinyjuice reduce build.log
tinyjuice reduce-json payload.json
tinyjuice verify --rules --fixtures
tinyjuice stats --timezone utc
tinyjuice doctor
```

CI should validate TinyJuice behavior with fixtures. CI should not silently
compact normal job logs unless a job explicitly opts in.

## Stable Adapter Contract

### Request

Adapters send either a direct input object or an envelope:

```json
{
  "input": {
    "toolName": "exec",
    "command": "cargo test",
    "argv": ["cargo", "test"],
    "cwd": "/repo",
    "combinedText": "test output...",
    "exitCode": 0,
    "metadata": {
      "source": "openhuman"
    }
  },
  "options": {
    "maxInlineChars": 1200,
    "raw": false,
    "noOmit": false,
    "store": false,
    "recordStats": true,
    "cwd": "/repo"
  }
}
```

### Response

Adapters receive:

```json
{
  "inlineText": "compacted output",
  "previewText": "optional unclamped preview",
  "stats": {
    "rawChars": 10000,
    "reducedChars": 1200,
    "ratio": 0.12
  },
  "classification": {
    "family": "test-results",
    "confidence": 0.9,
    "matchedReducer": "tests/cargo-test"
  }
}
```

Optional fields may include compaction metadata, a debug trace, artifact refs, or
CCR refs.

### Error Handling

- Invalid adapter payloads return structured errors.
- Runtime hook parse failures pass through raw output.
- Missing TinyJuice binary or disabled hooks are adapter doctor issues, not tool
  execution failures.
- A compressor failure must degrade to raw output.

## Configuration Model

Hosts should expose one conceptual config block, even if their native config
format differs:

```toml
[tokenjuice]
enabled = true
profile = "auto"
min_bytes_to_compress = 512
max_inline_chars = 1200
safe_inventory = true
record_stats = true

[tokenjuice.ccr]
enabled = true
min_tokens = 1024
max_entries = 256
max_bytes = 67108864
ttl_secs = 86400
disk_tier = true

[tokenjuice.compressors]
search = true
code = true
html = true
json = true
ml_text = false
```

Host mappings:

- `auto`: host default. Coding agents should resolve to `light`; general agents
  may resolve to `full`.
- `full`: all enabled compressors and CCR-backed lossy compaction.
- `light`: only lossless or non-CCR reductions; no ML text compression.
- `off`: full passthrough.

## Recovery Contract

Lossy compaction must add a retrieval marker only after the original is retained
in CCR.

Requirements:

- recovery token format is fixed and validated
- retrieval first checks memory, then optional disk tier
- disk paths are never derived from unvalidated user text
- recovery tool output is never compacted again
- adapters expose retrieval as a normal host tool where supported

OpenHuman should provide:

- `tokenjuice_retrieve(token)`
- optional ranged retrieval by lines or bytes
- clear not-found behavior for expired entries

## Host Adapter Modes

### Post-Tool Compaction

Preferred mode. The host executes the command normally, then TinyJuice replaces
or annotates output.

Use for:

- OpenHuman tool loop
- Codex post-tool hooks
- Claude Code post-tool hooks
- CLI harnesses that expose post-run output

### Pre-Tool Wrapping

Fallback mode. The host command is rewritten to call:

```sh
tinyjuice wrap -- <original command>
```

Use only when the host cannot replace output after execution.

Requirements:

- preserve shell semantics
- preserve exit code
- support raw bypass
- ensure classifier normalization can recover the original command and argv

### Instruction-Only Integration

Some hosts only support rule/instruction files. For those, install a short
instruction that tells the agent when to call `tinyjuice reduce-json` or
`tinyjuice wrap`.

Instruction-only integrations are beta until they have a runtime test.

## Installer and Doctor Framework

TinyJuice should eventually provide:

```sh
tinyjuice install openhuman
tinyjuice install codex
tinyjuice install claude-code
tinyjuice install cursor
tinyjuice uninstall codex
tinyjuice doctor codex
tinyjuice doctor hooks
```

Installer requirements:

- preserve unrelated config keys
- replace only prior TinyJuice-managed entries
- write atomically where possible
- create backups for user-owned files
- support `--local` for testing an unreleased binary
- be idempotent

Doctor requirements:

- detect `ok`, `warn`, `broken`, and `disabled`
- report expected command and detected command
- check missing executable paths
- check known feature flags where the host has them
- return one repair command

## Codebase Rollout

### Phase 1: OpenHuman Core

- depend on the crate directly
- configure TinyJuice at startup
- compact post-tool output through `compact_tool_output_with_policy`
- expose CCR retrieval
- record stats without raw content
- add tests for `full`, `light`, and `off`

### Phase 2: Runtime Python Boundary

- define the JSON payload builder
- call `tinyjuice reduce-json`
- map errors to passthrough
- add fixtures for shell output, JSON output, failure output, and raw bypass

### Phase 3: Agent Harnesses

- add a shared adapter package or helper module per language
- route shell outputs through the safe-inventory policy
- keep non-shell outputs raw until metadata is reliable
- add per-agent profile defaults

### Phase 4: Host Installers

- implement Codex and OpenHuman installers first
- add aggregate `doctor hooks`
- port other upstream host adapters only after each has tests

### Phase 5: Observability and Governance

- ship stats and discover commands
- add fallback-frequency reports
- review high-fallback command families before adding new rules
- add benchmark fixtures before making compression percentage claims

## Validation Gates

Every integration PR should run:

```sh
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test
```

Adapter-specific gates:

- fixture tests for request/response JSON
- raw bypass tests
- unsafe shell passthrough tests
- exact file read passthrough tests
- CCR retrieval tests
- config reload tests
- no-raw-content logging tests where practical

Host installer gates:

- install idempotency
- uninstall idempotency
- doctor status matrix
- unrelated config preservation
- temp-home isolation so tests never read real user config

## Non-Goals

- Do not couple the core crate to OpenHuman runtime internals.
- Do not copy every upstream host adapter before the framework and tests exist.
- Do not silently compact exact source-file reads.
- Do not claim 80-90% savings without benchmark fixtures.
- Do not add OpenHuman-only dependencies to the core crate without a feature or
  adapter boundary.
