# OpenHuman Integration Plan

## Goal

Integrate TinyJuice into OpenHuman as a shared Rust compression engine while
keeping the core crate independent of OpenHuman runtime types.

The first OpenHuman integration should use the existing post-tool hook:

```rust
compact_tool_output_with_policy(
    tool_name,
    arguments.as_ref(),
    output,
    exit_code,
    agent_tokenjuice_profile,
).await
```

## Current Fit

The current crate already exposes most of the needed integration primitives:

- `install_config()` for runtime option installation
- `compact_tool_output_with_policy()` for post-tool compaction
- `AgentTokenjuiceCompression` profiles
- `tokenjuice_retrieve` marker and recovery-tool bypass constants
- ML callback and savings callback hooks
- `CompressOptions` independent of OpenHuman config schema

## Required OpenHuman Responsibilities

OpenHuman should own:

- mapping app config into `CompressOptions`
- mapping agent defaults into `auto`, `full`, `light`, or `off`
- calling `install_config()` at startup and config reload
- passing tool arguments and exit codes into TinyJuice
- exposing `tokenjuice_retrieve(token, range?)`
- ensuring recovery tool output is never compacted
- recording stats without raw content
- deciding exact-read/stub-read policy
- wiring optional ML compression through the existing callback

## Core TinyJuice Responsibilities

TinyJuice core should own:

- deterministic classification and compression
- CCR token generation and validation
- recovery footer formatting and parsing
- per-kind compressor behavior
- rule loading and rule validation
- adapter-neutral request/response types
- safe skip decisions and skip reason reports

## Integration Phases

### Phase 1: Stabilize Tool Output Hook

Tasks:

- Confirm all OpenHuman tool outputs pass through a single post-tool hook after
  credential scrubbing.
- Install current `CompressOptions` through `install_config()`.
- Add OpenHuman-side tests for `full`, `light`, `off`, and `auto` resolution.
- Add tests proving recovery-tool output bypasses compaction.
- Add tests proving config reload updates cache and compressor settings.

Acceptance:

- `full` can return CCR-backed lossy compaction.
- `light` declines lossy compaction and still allows lossless reformat where
  available.
- `off` is exact passthrough.
- Recovery output is exact passthrough.

### Phase 2: Exact Read And Safe Inventory Policy

Tasks:

- Add host metadata that distinguishes exact file reads from exploratory
  inventory and search results.
- Keep exact file reads raw by default.
- Allow explicit AST/stub reads through a separate adapter intent.
- Allow safe inventory commands such as `find . -type f | sort | head`.
- Decline mixed shell sequences and unsafe command forms.

Acceptance:

- `cat src/lib.rs` remains exact unless the host explicitly asks for a stub.
- `rg --files` and `find . -type f | sort | head` may compact.
- `find . -exec cat {} \;` remains raw.
- Exit code and stderr are preserved.

### Phase 3: Recovery UX

Tasks:

- Expose `tokenjuice_retrieve` in every agent profile that may see a footer.
- Support full and ranged retrieval by lines or bytes.
- Return clear not-found output for expired or evicted CCR entries.
- Consider UI affordances for "retrieve full original" without requiring the
  model to infer the token.

Acceptance:

- Every emitted footer references a retrievable token at emission time.
- Ranged retrieval handles UTF-8 safely.
- Not-found cases do not panic or leak local paths.

### Phase 4: Stats And Cost Integration

Tasks:

- Extend stats with applied compressor, original bytes, compacted bytes, lossy
  flag, CCR token present, content kind, and skip reason.
- Feed measured model usage from OpenHuman when available.
- Label savings as measured, counted, or estimated.
- Avoid raw prompt/tool content in logs and persistent stats.

Acceptance:

- OpenHuman can show recent compactions without displaying raw content.
- Cost estimates are not presented as measured facts.
- Fixture benchmark results are separate from live estimates.

### Phase 5: Conversation Layer

Tasks:

- Add an optional conversation compression adapter after the tool-output hook is
  stable.
- Keep message, summary, lease, and persistence ownership in OpenHuman.
- Use TinyJuice pure helpers for budgets, boundaries, tool digests, and summary
  contracts.

Acceptance:

- Conversation compaction does not require core TinyJuice to know OpenHuman
  session database types.
- Summary failures preserve original messages unless deterministic fallback is
  explicitly allowed.

## OpenHuman Integration Risks

- Applying code compression to exact reads would damage trust. Default should be
  raw exact reads.
- Compaction before credential scrubbing could retain secrets in CCR. The hook
  should run after scrubbing, or OpenHuman should explicitly mark sensitive
  outputs as raw.
- CCR disk tier stores original text. It should be opt-in, bounded, and rooted
  in an OpenHuman-controlled workspace path.
- ML text compression can remove workflow tags or instructions. It should stay
  off by default until tag protection and fixtures exist.

