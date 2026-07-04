# Hermes Compression Algorithms and Techniques Spec

## Purpose

This spec documents the compression and prompt-size management techniques found
in `NousResearch/hermes-agent` and maps them to TinyJuice ingestion candidates.
It is a design reference only. It does not claim TinyJuice compression savings;
benchmarks must land before any percentage claims.

Source baseline:

- Repository: `https://github.com/NousResearch/hermes-agent`
- Inspected commit: `0d27d2ed147f5443bd111bf4cf3d295d9ec2917e`
- Local comparison point: TinyJuice working tree on `2026-07-04`

Primary Hermes source files inspected:

- `agent/context_compressor.py`
- `agent/conversation_compression.py`
- `agent/prompt_caching.py`
- `agent/transports/codex.py`
- `agent/context_breakdown.py`
- `agent/manual_compression_feedback.py`
- `hermes_cli/partial_compress.py`
- `trajectory_compressor.py`
- `acp_adapter/tools.py`
- `run_agent.py`
- `hermes_state.py`

## Executive Summary

Hermes does not primarily implement the same kind of content-aware reducer that
TinyJuice already has for JSON, diffs, logs, code, search output, HTML, and CCR
offload. Hermes' main production mechanism is conversation compaction:

1. detect when a conversation is near a model context threshold
2. cheaply prune old tool outputs
3. preserve a protected head and token-budgeted tail
4. summarize the middle turns with an auxiliary LLM
5. insert a strongly marked handoff summary
6. preserve or archive the original transcript at the session layer
7. keep prompt caching as a separate concern

The useful TinyJuice ingestion targets are therefore not "port Hermes as one
compressor." They are reusable algorithms and contracts around safe
conversation-level reduction:

- pre-LLM deterministic pruning
- token-budget tail selection
- active-turn and last-visible-reply anchoring
- tool-call/result pair integrity
- structured handoff summaries
- summary failure policy
- prompt-cache-aware static prefix keys
- manual partial compression boundaries
- provider-safe structured rendering and truncation
- context usage breakdowns for observability

## Current TinyJuice Overlap

TinyJuice already has core capabilities that Hermes lacks or handles at a
different layer:

- content-kind routing through `compress_content`
- CCR-backed lossy reduction and retrieval markers
- JSON table compression with head/tail, error-row, and outlier preservation
- diff context collapsing and lockfile summarization
- log signal preservation and TokenJuice rule support
- optional tree-sitter code compression
- optional ML text compression through an adapter boundary
- bounded in-memory and optional disk-backed CCR store

Hermes adds the strongest ideas around conversation history, provider request
validity, and operator-facing compaction lifecycle. TinyJuice should ingest
those as an optional conversation adapter layer rather than weakening its
content-specific router.

## Important Distinction: Compression vs Prompt Caching

Hermes keeps prompt caching separate from compression:

- `agent/context_compressor.py` and `agent/conversation_compression.py` mutate
  the message list by pruning and summarizing.
- `agent/prompt_caching.py` only injects Anthropic `cache_control` markers.
- `agent/transports/codex.py` computes a stable `prompt_cache_key` from static
  request prefix content.

TinyJuice should preserve this boundary:

- compression APIs may mutate text or return CCR markers
- prompt-cache APIs may describe stable/frozen prefix boundaries
- neither API should silently rewrite the other layer's state

This avoids mixing lossy data reduction with provider-specific cache routing.

## Hermes Production Compression Pipeline

Hermes' live compaction path can be represented as:

```text
messages + system prompt
  -> threshold check
  -> deterministic tool-output prune
  -> protected head selection
  -> token-budget protected tail selection
  -> boundary alignment around tool groups
  -> latest user and assistant anchoring
  -> middle-window summarization
  -> handoff summary insertion or merge
  -> tool-pair sanitizer
  -> historical media stripping
  -> session persistence, archive, or rotation
  -> post-compression accounting
```

TinyJuice should model this as a separate `conversation` profile:

```rust
pub struct ConversationCompressionInput {
    pub messages: Vec<ConversationMessage>,
    pub system_prompt: Option<String>,
    pub model_context_tokens: usize,
    pub output_reservation_tokens: Option<usize>,
    pub focus_topic: Option<String>,
    pub force: bool,
}

pub struct ConversationCompressionOutput {
    pub messages: Vec<ConversationMessage>,
    pub report: CompressionReport,
    pub boundary: Option<CompactionBoundary>,
    pub cache_hints: Vec<PromptCacheHint>,
}
```

This should live behind a feature or adapter boundary because it needs
conversation roles, provider request assumptions, and optional LLM summaries.
The core TinyJuice content compressors should remain independent.

## Threshold and Token Budgeting

Hermes computes a compaction threshold from the model context window, configured
threshold ratio, and output token reservation. The effective input window is:

```text
effective_input_window = context_length - max_output_tokens
threshold = max(effective_input_window * ratio, minimum_context_floor)
```

When the floor would equal or exceed the effective input window, Hermes triggers
around 85% of the effective window so smaller models can still compact before
provider rejection.

TinyJuice ingestion:

- Add a request-budget helper independent of any specific provider.
- Include output reservation in compression trigger decisions.
- Expose `threshold_tokens`, `estimated_prompt_tokens`, and
  `effective_input_window` in reports.
- Do not treat rough token estimates as exact tokenizer counts.
- Add tests for small context windows where a floor would otherwise block
  compaction.

Acceptance:

- A 64K context model with a 64K floor still produces a threshold below 64K.
- A configured output reservation reduces the input budget.
- The report distinguishes rough estimates from provider-reported usage.

## Deterministic Tool-Output Prepass

Before using an LLM, Hermes replaces old large tool results with informative
one-line summaries. It also deduplicates identical older tool outputs and
shrinks oversized tool-call arguments.

Hermes examples:

- terminal: command, exit code, line count
- read file: path, starting line, character count
- search: pattern, path, match count
- web extract: first URL and result size
- delegate task: goal preview and result size
- process tools: action and session id

This prepass is a strong TinyJuice fit because it is deterministic, cheap, and
does not need model calls.

Spec:

- Add a `ToolResultDigest` reducer for conversation messages.
- Pair assistant tool calls with later tool results by call id.
- Deduplicate identical tool-result bodies with a stable hash.
- Replace old duplicate bodies with a back-reference marker.
- Replace old non-duplicate bodies above a threshold with tool-specific digest
  strings.
- Keep recent tail tool results verbatim.
- Record each digest in the compression report.

The digest should not log or expose raw prompt/context content outside the
returned compressed message. It may include tool name, sanitized command/path
metadata, exit code, line count, match count, and byte count.

Acceptance:

- Repeated identical read-file outputs keep only the newest full copy.
- Old terminal output becomes a command/exit/line-count summary.
- Recent tail output remains exact.
- Tool arguments remain valid JSON after shrinking.
- Sensitive keys and values are redacted before persistence or external model
  calls.

## Provider-Safe Tool-Argument Shrinking

Hermes parses tool-call argument JSON before truncating long string leaves. This
avoids a common failure mode: slicing raw JSON produces invalid arguments, and
providers reject every future request until the broken history leaves context.

TinyJuice should adopt this as a reusable structured truncation utility:

```rust
pub fn shrink_json_string_leaves(
    value: &serde_json::Value,
    max_string_chars: usize,
) -> serde_json::Value
```

Rules:

- parse JSON first
- only shrink string leaves
- preserve object keys, booleans, numbers, arrays, paths, and ids
- serialize as valid JSON
- if the input is not JSON, return it unchanged unless a caller explicitly opts
  into raw text clamping

Acceptance:

- A large `write_file` argument stays syntactically valid JSON.
- Non-JSON tool arguments are not corrupted.
- Unicode content is preserved without unnecessary escaping.

## Multimodal and Historical Media Reduction

Hermes counts image content as a meaningful token budget even when text is
empty, strips older image parts after a newer image-bearing user message, and
can retry failed provider calls by shrinking base64 image payloads.

TinyJuice core should not add image processing dependencies by default. The
ingestion should be an adapter contract:

```rust
pub trait MediaReducer {
    fn estimate_media_tokens(&self, part: &MediaPart) -> usize;
    fn replace_historical_media(&self, messages: &[ConversationMessage])
        -> Vec<ConversationMessage>;
}
```

Spec:

- Count image parts with a conservative flat token estimate when no tokenizer is
  available.
- Preserve the newest image-bearing user turn.
- Replace older image parts with stable placeholder text.
- Keep binary resize/re-encode logic outside core TinyJuice, behind an adapter.

Acceptance:

- A message with five images cannot be treated as near-zero tokens.
- Old screenshots do not survive every compaction forever.
- The newest image-bearing user turn remains available.

## Protected Head and Decaying Head Preservation

Hermes always protects the system prompt and protects an initial head window on
the first compression. After the first compression, the early non-system head
decays to zero because those turns are already represented in the handoff
summary. This prevents old user instructions from becoming immortal.

TinyJuice ingestion:

- Add `protect_system_prompt: bool`.
- Add `protect_first_n_messages` for the first compaction only.
- Add `decay_head_after_first_compaction: bool`.
- Track compaction count in the report or caller-provided state.

Acceptance:

- First compaction may keep the opening task framing.
- Repeated compactions do not repeatedly copy the first user request.
- The system prompt remains protected unless the caller explicitly omits it.

## Tail Selection by Token Budget

Hermes protects recent context by token budget rather than fixed message count.
It walks backward from the end until a tail budget is reached, with a small
message-count floor and a soft ceiling to avoid pathological cuts inside a huge
message.

TinyJuice should expose this as reusable pure logic:

```rust
pub fn select_tail_by_budget(
    messages: &[ConversationMessage],
    head_end: usize,
    token_budget: usize,
    min_tail_messages: usize,
    soft_ceiling_ratio: f32,
) -> usize
```

Rules:

- walk backward from the end
- estimate each message including role overhead and tool-call envelope cost
- keep a bounded recent-message floor
- allow limited budget overshoot for oversized messages
- fall back to a useful middle window when the whole region fits the soft
  ceiling but compression was explicitly requested

Acceptance:

- A huge old tool result does not force preservation of all recent history.
- A short transcript does not rotate into a no-op compaction.
- The selected boundary is deterministic.

## Tool-Group Boundary Alignment

Hermes avoids splitting assistant tool calls from their tool results:

- if a compression-start boundary lands on a tool result, move it forward
- if a compression-end boundary lands after tool results, move it backward to
  include the parent assistant call
- after compaction, sanitize orphaned tool results and orphaned tool calls

TinyJuice should make this a first-class invariant for conversation compaction:

- Every retained `tool` result must have a retained assistant tool call.
- Every retained assistant tool call should have a retained tool result unless
  the provider format allows otherwise.
- Boundary movement must be monotonic and tested with parallel tool-call groups.

Acceptance:

- No compressed output contains a tool result whose call id is missing.
- No compressed output contains a tool-call-only assistant message that the
  target provider will reject.
- Sanitization happens after summary insertion and tail reattachment.

## Latest User and Assistant Anchors

Hermes explicitly anchors:

- the latest real user message, so the active task is never only inside a
  historical summary
- the latest visible assistant reply, so UIs do not replace the last answer with
  an opaque compaction block

It also skips internal compaction-summary messages when searching for the latest
real user turn.

TinyJuice ingestion:

- Add `anchor_latest_user: bool`.
- Add `anchor_latest_visible_assistant: bool`.
- Tag generated compaction summaries with metadata so they are not mistaken for
  real user messages.
- When anchoring would split a user/assistant pair, move the boundary to keep
  the pair together.

Acceptance:

- The most recent user ask remains outside the compressed middle.
- The last rendered assistant answer remains outside the compressed middle.
- Compaction summary messages are excluded from latest-user search.

## Structured Handoff Summary

Hermes uses a strict summary shape:

- historical task snapshot
- goal
- constraints and preferences
- completed actions
- active state
- historical in-progress state
- blockers
- key decisions
- resolved questions
- historical pending asks
- relevant files
- historical remaining work
- critical context

The handoff prefix repeatedly states that the summary is reference-only and
that the latest user message wins. Hermes also appends an explicit end marker
so weaker models do not treat quoted historical asks as fresh instructions.

TinyJuice should not put LLM summarization in the core crate by default. It
should define the transport-neutral summary contract and leave generation to an
adapter:

```rust
pub trait SummaryProvider {
    fn summarize(
        &self,
        request: SummaryRequest,
    ) -> Result<StructuredSummary, SummaryError>;
}
```

Core TinyJuice can still provide:

- summary prompt construction
- deterministic fallback summary construction
- summary insertion and metadata tagging
- redaction hooks
- boundary and report accounting

Acceptance:

- Summary text cannot be mistaken for an active user request.
- Summary output is redacted after generation, not only before generation.
- Generated summary messages carry internal metadata that callers can strip
  before provider calls if needed.
- Repeated compactions update the existing summary instead of nesting old
  summaries indefinitely.

## Iterative Summary Updates

Hermes stores the previous compaction summary and, on later compactions,
updates it with only the new turns. When a previous summary is found in the
message list after resume, Hermes rehydrates summary state from that message.
If no summary exists in the current session, stale cross-session summary state
is discarded.

TinyJuice ingestion:

- `CompactionState` should include an optional previous structured summary.
- Rehydration from message metadata should be explicit.
- Cross-session state must be cleared unless caller passes a lineage id.
- Old summary prefixes and end markers should be normalized before iterative
  update.

Acceptance:

- A resumed session with a previous summary can continue iterative compaction.
- A new session cannot accidentally inherit another session's summary.
- Historical summary wrappers are stripped before re-summarization.

## Deterministic Fallback Summary

When the LLM summarizer is unavailable, Hermes can build a deterministic
fallback from locally extractable anchors:

- recent user asks
- assistant tool calls
- tool-action digests
- relevant files and paths
- blocker/error snippets
- last dropped turns

TinyJuice should implement this before any LLM-backed summary provider. It gives
users a safe no-network fallback and provides tests for the summary insertion
contract.

Spec:

- Build a fallback from already-redacted message text.
- Extract path-like mentions and JSON fields named `path`, `workdir`,
  `file_path`, and `output_path`.
- Extract tool names and call ids.
- Preserve error/blocker snippets by keyword.
- Clamp total fallback size.
- Mark fallback summaries as lower confidence in the report.

Acceptance:

- If the summary provider is absent, deterministic fallback can still compact.
- Fallback text says it is locally generated and may be incomplete.
- Secrets are redacted before preservation.

## Summary Failure Policy

Hermes distinguishes summary failures:

- no provider configured: cooldown, optional fallback
- model not found / unavailable / timeout: try main model fallback when safe
- malformed JSON / proxy HTML / premature stream close: treat as transient
- auth or permission failure: abort compaction and preserve all messages
- network failure: abort compaction and preserve all messages
- repeated ineffective compression: back off

TinyJuice ingestion:

- Add `SummaryFailurePolicy`.
- Default to preserving data on auth and network failures.
- Only drop a middle window without an LLM summary if deterministic fallback is
  enabled and the caller accepts lossy summarization.
- Record failure reason, retry class, and whether any messages were dropped.
- Add cooldown state outside stateless core compression, owned by the adapter.

Acceptance:

- Auth failure never rotates into a degraded compressed session.
- Network failure preserves the original message list.
- Manual `force` can bypass transient cooldown.
- Repeated less-than-10-percent estimated savings backs off.

## In-Place vs Child-Session Compaction

Hermes supports two persistence modes:

- legacy child-session rotation: end the old session and create a continuation
- in-place compaction: keep one durable session id, soft-archive old rows, and
  insert compacted rows as the live transcript

TinyJuice should not own session databases, but it should expose enough
metadata for hosts to implement either mode safely:

```rust
pub enum PersistenceRecommendation {
    InPlaceArchive,
    ChildContinuation,
    NoPersistenceChange,
}
```

Compression reports should include:

- old message count
- new message count
- boundary start/end
- compaction count
- summary/fallback used
- whether original data is recoverable through CCR or host archive
- reason for abort or no-op

Acceptance:

- Hosts can distinguish "compaction succeeded in place" from "session id
  rotated."
- A no-op compression does not require session persistence changes.
- Reports do not include raw prompt/context content unless the caller explicitly
  asked for a preview.

## Compression Locks and Race Avoidance

Hermes uses a state-backed per-session compression lock with a TTL and refresher
so two agent paths do not compact the same session concurrently and create
orphaned continuations.

TinyJuice core should stay stateless, but host adapters need a lock contract:

```rust
pub trait CompactionLease {
    fn try_acquire(&self, session_id: &str) -> LeaseResult;
    fn refresh(&self, lease_id: &str) -> bool;
    fn release(&self, lease_id: &str);
}
```

Spec:

- The lock key is the pre-compaction session id.
- The lease has a TTL and bounded refresh failures.
- If lock acquisition fails, return the original messages unchanged with an
  explicit report status.
- Release only after persistence and post-compaction bookkeeping finish.

Acceptance:

- Concurrent compaction attempts do not both mutate the same session.
- A crashed compactor cannot block future compaction forever.
- Hosts can surface "compaction already in flight" without treating it as data
  loss.

## Manual Partial Compression

Hermes supports user-chosen boundaries such as "compress here" or "keep last N
exchanges." It splits history by the Nth most recent user turn so the preserved
tail starts at a legal role boundary, compresses the head through the normal
pipeline, then rejoins the verbatim tail.

TinyJuice ingestion:

- Add a pure `split_for_partial_compaction(history, keep_last_exchanges)` helper.
- Tail starts at a user message.
- If the split leaves no head, return a no-op or fall back to full compression.
- Add a rejoin helper that checks role alternation at the boundary.
- Add preview mode with counts and estimated tokens but no mutation.

Acceptance:

- `keep_last=2` preserves the last two user exchanges verbatim.
- Rejoined messages do not violate role alternation.
- Preview output includes head count, tail count, total count, and rough tokens.

## Prompt Cache Techniques

Hermes uses two cache strategies that TinyJuice can expose as hints.

### Anthropic Cache-Control Markers

Hermes' Anthropic helper:

- deep-copies messages
- applies up to four cache breakpoints
- marks the system prompt first
- then marks the last three non-system messages that can actually carry a
  provider-honored marker
- handles native Anthropic tool-result layout differently from envelope-style
  providers

TinyJuice should expose cache-boundary hints, not mutate provider payloads in
core:

```rust
pub struct PromptCacheHint {
    pub provider: String,
    pub message_index: usize,
    pub ttl: PromptCacheTtl,
    pub placement: CacheMarkerPlacement,
}
```

### Static Prefix Cache Key

Hermes' Codex transport computes `prompt_cache_key` as a SHA-256 digest of:

```text
instructions + NUL separator + sorted tool schemas
```

Session id is only a fallback when there is no static prefix. This keeps
recurring jobs warm even when each run has a timestamped session id.

TinyJuice ingestion:

- Add an optional `stable_prefix_cache_key(system_prompt, tools)` helper.
- Sort tool schemas by stable name/type.
- JSON serialize with sorted keys and compact separators.
- Prefix keys so they cannot be confused with session ids.
- Treat cache key as routing hint, never a correctness boundary.

Acceptance:

- Tool order does not change the cache key.
- A timestamped session id does not make identical static prefixes cache-cold.
- Cache hints are reported separately from compression steps.

## Context Usage Breakdown

Hermes computes a UI-oriented context breakdown:

- system prompt
- tool definitions
- rules/context files
- skills
- MCP tools
- subagent definitions
- memory
- conversation

TinyJuice should add a host-facing estimator API:

```rust
pub struct ContextBreakdown {
    pub categories: Vec<ContextBucket>,
    pub estimated_total_tokens: usize,
    pub measured_prompt_tokens: Option<usize>,
    pub context_max_tokens: Option<usize>,
}
```

This helps hosts explain why compression triggered and where savings came from.
It also prevents TinyJuice from being blamed for fixed prompt/tool-schema costs
it did not control.

Acceptance:

- The report separates conversation compression from static tool/schema cost.
- Hosts can render a compact usage bar without inspecting raw prompt text.
- Measured provider usage overrides rough estimate when available.

## ACP and Adapter-Level Structured Truncation

Hermes' ACP adapter formats tool results into compact Markdown instead of
dumping raw JSON:

- file search shows count, first N files, and a narrowing hint
- content search shows first N matches with short snippets
- read-file output is fenced to avoid Markdown table parsing
- process output keeps status, pid, exit code, and bounded stdout/stderr
- delegate output shows child status, duration, summary, error, and tool names
- generic structured values are rendered as bounded nested bullets

TinyJuice should separate model-facing compression from UI-facing rendering, but
the structured truncation primitives are reusable:

- nested JSON depth limit
- max fields/items limit
- priority key extraction
- content snippet limit
- structured failure detection
- fenced text that cannot be broken by backticks

Acceptance:

- Structured tool output can be rendered compactly without losing status/error.
- Deep or large JSON cannot blow a UI or model context budget.
- Raw credentials in known sensitive keys are redacted.

## Offline Trajectory Compression

Hermes includes `trajectory_compressor.py` for benchmark-style action
trajectories. Its strategy:

1. count tokens per turn with a tokenizer, falling back to char/4
2. skip if under target
3. protect first system/human/gpt/tool turns and last N turns
4. snap boundaries away from tool-response orphans
5. compress only as much middle context as needed to hit target
6. replace the compressed span with one summary turn
7. track aggregate metrics

This is less relevant to TinyJuice runtime compression but useful for benchmark
fixtures.

TinyJuice ingestion:

- Build benchmark fixtures around conversation trajectories.
- Test boundary snapping and target-token math with exact small examples.
- Track before/after tokens, turns removed, summary calls, errors, and
  still-over-limit outcomes.

Acceptance:

- Fixture compression can be reproduced deterministically with a stub summary
  provider.
- Metrics distinguish skipped-under-target from failed-to-fit.
- Tool-call/tool-result markers are not orphaned.

## Proposed TinyJuice Modules

### `conversation::types`

Add provider-neutral conversation structures:

- `ConversationMessage`
- `MessageRole`
- `ToolCall`
- `ToolResult`
- `MessageContent`
- `MediaPart`
- `CompactionBoundary`
- `CompactionState`

### `conversation::budget`

Pure helpers:

- rough message token estimator
- image/media token estimator hook
- effective input window
- threshold computation
- tail selection by token budget

### `conversation::tool_digest`

Deterministic prepass:

- tool-call map
- old tool-result digest
- duplicate result replacement
- JSON argument shrinker
- redaction hook

### `conversation::boundary`

Boundary safety:

- head protection
- decaying head
- tool-group alignment
- latest user anchor
- latest assistant anchor
- role alternation rejoin

### `conversation::summary`

Adapter-facing summary layer:

- summary prompt builder
- structured summary schema
- deterministic fallback summary
- summary prefix/end marker handling
- metadata tagging
- summary failure policy

### `cache_hints`

Prompt cache support:

- stable prefix cache key
- provider cache marker hints
- static-prefix/frozen-prefix metadata

### `observability`

Host-facing reports:

- context breakdown buckets
- compaction lifecycle status
- manual preview summary
- before/after rough token accounting

## Implementation Order

### P0: Deterministic Safety Primitives

Implement first because they do not require LLM calls:

- provider-safe JSON string-leaf shrinking
- tool-call/result pair sanitizer
- boundary alignment helpers
- tail selection by token budget
- latest-user and latest-assistant anchors
- manual partial split and rejoin helpers

### P1: Conversation Prepass

Add deterministic old-tool-output digesting:

- tool-specific digest renderers
- duplicate-result detection
- protected tail handling
- report step attribution
- redaction tests

### P1: Prompt Cache Hints

Add cache helpers without provider mutation:

- stable prefix cache key
- cache hint structures
- Anthropic-compatible "system plus last three carriers" selector
- docs showing that these are separate from compression

### P2: Summary Adapter Contract

Add the optional summarization layer:

- `SummaryProvider` trait
- summary prompt builder
- deterministic fallback summary
- summary failure policy
- compaction metadata tagging

No OpenHuman runtime or provider SDK dependency should enter core TinyJuice.

### P2: Host Integration Spec

Document how OpenHuman or other hosts should:

- acquire a compaction lease
- provide measured usage
- persist in-place archive or child continuation
- clear stale dedup caches after compaction
- surface warnings
- pass prompt-cache hints to provider adapters

### P3: Benchmark Trajectory Fixtures

Add fixtures before publishing savings claims:

- long tool-heavy conversation
- repeated read-file outputs
- huge terminal output
- multimodal image-heavy session
- partial compress "keep last N exchanges"
- auth/network summary failure
- repeated no-op compression anti-thrash

## Non-Goals

- Do not make LLM summarization mandatory for TinyJuice core.
- Do not add OpenHuman runtime dependencies to core TinyJuice.
- Do not merge prompt caching with lossy compression.
- Do not claim Hermes-style savings until TinyJuice has fixtures and metrics.
- Do not log raw prompt/context content from library code.
- Do not make session database rotation a TinyJuice responsibility.

## Test Matrix

Minimum tests before implementation is considered ready:

- JSON argument shrink preserves parseability.
- Non-JSON argument shrink is a no-op.
- Tool result without retained tool call is removed.
- Tool call without retained result is stripped or bridged according to target
  provider policy.
- Tail selector anchors latest real user message.
- Tail selector anchors latest visible assistant message.
- Summary metadata excludes generated summaries from latest-user search.
- Partial split keeps last N user exchanges.
- Partial rejoin avoids user/user and assistant/assistant adjacency.
- Static prefix cache key is stable under tool order changes.
- Compression failure policy aborts on auth and network failures.
- Deterministic fallback summary redacts sensitive content.
- Context breakdown never requires raw prompt logging.

## TinyJuice Fit

Hermes is most valuable to TinyJuice as a conversation-safety reference. The
core Rust reducer should continue to focus on content-aware compression and CCR.
The Hermes-derived work should enter as:

- pure boundary and budget helpers in core
- optional conversation message types
- optional host adapter traits for summary providers and leases
- prompt-cache hints that hosts can translate to provider payloads
- benchmark fixtures that prove correctness before savings claims

That keeps TinyJuice small and reusable while making it much safer to integrate
into long-running OpenHuman conversations.
