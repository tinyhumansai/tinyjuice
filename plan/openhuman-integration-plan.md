# OpenHuman Integration Plan

## Goal

Integrate TinyJuice into OpenHuman as a shared Rust compression engine while
keeping the core crate independent of OpenHuman runtime types.

## Verified OpenHuman State (2026-07-04)

The integration is not greenfield. OpenHuman already vendors TinyJuice
(`vendor/tinyjuice` submodule pinned at `4b1a34f`, wired through
`[patch.crates-io]`) and ships it as the "TokenJuice" product feature. The
existing surface:

- Adapter module: `src/openhuman/tokenjuice/` re-exports the crate API and owns
  `install_from_config(&Config)`, called at startup and on live settings
  updates. It also wires the savings recorder and the ML compressor callback
  (Kompress bridge via `runtime_python_server`).
- Config: `[tokenjuice]` block in `src/openhuman/config/schema/tokenjuice.rs`
  plus per-agent `AgentTokenjuiceCompression` overrides and
  `tokenjuice/config_patch.rs` partial updates.
- Hook site: `ToolOutputMiddleware::after_tool` in
  `src/openhuman/tinyagents/middleware.rs` (~line 787). Per normal tool result
  the chain is: (1) `PayloadSummarizer` semantic summarization (orchestrator
  only), (2) TokenJuice compaction, (3) per-tool char cap, (4) shared 16 KiB
  byte-cap backstop with artifact persistence (`ToolResultArtifactStore`).
  `HandoffMiddleware` is registered after but runs first (the harness runs
  `after_tool` in reverse registration order) and can stash raw >50k-token
  payloads before any of this. Recovery tool results bypass this chain and are
  returned exactly.
- Recovery tools: both `retrieve_tool_output`
  (`src/openhuman/tools/impl/system/retrieve_tool_output.rs`, legacy) and
  `tokenjuice_retrieve` (`src/openhuman/tokenjuice/tools.rs`) are registered.
- `Auto` profile resolution is host-side in
  `agent/harness/definition.rs:487`: coding models resolve to `Light`,
  everything else to `Full`. TinyJuice now treats unresolved `Auto` as
  passthrough with `none/agent-profile-auto-unresolved`, so hosts must resolve
  it before calling the adapter.

### Known Integration Bugs (found in review)

1. **Hook migration status.** OpenHuman's tinyagents middleware now captures
   `TaToolCall` arguments in `before_tool`, parses rendered shell exit-code
   prefixes, and calls the full TokenJuice policy adapter after each tool
   result. Command rules, extension/query hints, and exit-code-aware log
   behavior are active on that production path. Keep the minimal wrapper only
   for call sites that truly lack metadata.
2. **Footer-truncation status.** `CompressedOutput` now exposes separate
   `body` and `recovery_footer` fields while keeping `text` as the
   compatibility body+footer output. OpenHuman's tinyagents middleware caps the
   body and reattaches the footer afterward. Other host-side caps still need
   the same audit.
3. **Recovery-tool passthrough status.** OpenHuman's tinyagents middleware now
   bypasses summarization, TokenJuice compaction, per-tool caps, and the shared
   byte cap for both `tokenjuice_retrieve` and legacy `retrieve_tool_output`.
   A host-path regression test covers both names with intentionally tiny caps.
4. **OpenHuman checkout shape.** The reviewed plan assumed a
   `vendor/tinyjuice` submodule, but this `../openhuman-4` checkout carries a
   local TokenJuice copy under `src/openhuman/tokenjuice/`. Footer and hook
   contract changes therefore need to be mirrored there until the dependency
   boundary is restored.

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
- ensuring recovery tool output is never compacted or host-capped
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

- Keep `ToolOutputMiddleware::after_tool` on the full policy adapter, passing
  captured tool JSON arguments and parsed exit code.
- Reconcile marker/tool-name constants with OpenHuman's tool docs. In the
  current `openhuman-4` checkout, mirror crate contract changes in the local
  `src/openhuman/tokenjuice/` copy rather than a `vendor/tinyjuice` submodule.
- Preserve the footer-aware cap contract: the host truncates
  `CompressedOutput::body` and reattaches `CompressedOutput::recovery_footer`.
- Verify credential scrubbing runs before `after_tool` compaction; if not,
  compaction can retain secrets in CCR (see Risks).
- Document the middleware ordering invariant: handoff observes raw output
  first, then summarizer, then TokenJuice, then caps. TokenJuice must tolerate
  receiving summarizer output rather than raw tool output.
- Add OpenHuman-side tests for `full`, `light`, `off`, and `auto` resolution
  (`auto` resolves in `definition.rs`, not in the crate). TinyJuice and the
  mirrored OpenHuman copy now enforce unresolved `Auto` as an explicit
  passthrough, covered by `auto_agent_profile_requires_host_resolution`.
- Add tests proving recovery-tool output bypasses compaction and host caps for
  both registered tool names.
- Add tests proving config reload updates cache and compressor settings.

Acceptance:

- `full` can return CCR-backed lossy compaction.
- `light` declines lossy compaction and still allows lossless reformat where
  available.
- `off` is exact passthrough.
- Recovery output is exact passthrough.
- A shell tool result carrying `command`/`argv` arguments reaches the rule
  reducer and compacts under a matching rule.
- An output that is compacted and then capped by the host still ends with a
  parseable recovery marker.

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
- Consolidate the duplicate recovery tools: OpenHuman registers both the
  legacy `retrieve_tool_output` and `tokenjuice_retrieve` with separate
  implementations. Keep one implementation, alias the legacy name during
  migration, and make sure footer text, tool schema docs, and
  `RECOVERY_TOOL_NAMES` agree on the canonical name.
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

OpenHuman already has conversation-level machinery in the tinyagents adapter:
`ContextCompressionMiddleware` (`src/openhuman/tinyagents/summarize.rs`, live
history summarization), `MessageTrimMiddleware`, and `MicrocompactMiddleware`
(clears older tool-result bodies). Live history reduction was deliberately
moved out of `ContextManager` into these middlewares (issue #4249). The
conversation plan is therefore an extraction-and-hardening exercise against
those seams, not a new layer.

Tasks:

- Slot TinyJuice's pure helpers (budgets, boundaries, tool digests, summary
  contracts) under the existing middlewares rather than adding a parallel
  adapter.
- Keep message, summary, lease, and persistence ownership in OpenHuman.
- Reconcile `MicrocompactMiddleware`'s body-clearing with the Hermes-style
  tool-result digest so the two do not double-process the same messages.

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
  off by default until tag protection and fixtures exist. Note the Kompress
  bridge is already wired in OpenHuman's `install_from_config`; verify its
  config default is off.
- OpenHuman's README already claims "up to 80% fewer tokens" for TokenJuice,
  which conflicts with this plan set's no-percentage-claims-until-fixtures
  constraint. Either land the fixture benchmark suite early enough to back the
  claim or soften the README wording.
- The host truncation caps (per-tool char cap, 16 KiB byte-cap backstop) and
  TinyJuice compaction currently compose blindly. Until the footer contract is
  fixed, any cap below the compacted size silently severs recoverability.
