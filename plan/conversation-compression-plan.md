# Conversation Compression Plan

## Goal

Add optional conversation-level compaction primitives inspired by Hermes without
polluting the core content compressor or taking ownership of OpenHuman session
persistence.

## Boundary

Conversation compression is not the same as tool-output compression.

TinyJuice core can provide:

- provider-neutral message types
- token-budget helpers
- deterministic tool-result digests
- boundary and tool-pair safety helpers
- summary prompt and structured summary contracts
- deterministic fallback summary
- prompt-cache hints
- context usage breakdown shapes

OpenHuman or another host should own:

- session database mutation
- compaction leases
- provider-specific request rendering
- LLM summary provider choice
- measured usage data
- archive or child-session persistence

## Proposed Modules

```text
src/conversation/types.rs
src/conversation/budget.rs
src/conversation/tool_digest.rs
src/conversation/boundary.rs
src/conversation/summary.rs
src/cache_hints.rs
src/observability.rs
```

These modules should be feature-gated or kept dependency-light.

## P0: Deterministic Safety Primitives

Add pure helpers first:

- `shrink_json_string_leaves()`
- `select_tail_by_budget()`
- head protection and decaying head helpers
- tool-call/tool-result boundary alignment
- orphaned tool result sanitizer
- latest real user anchor
- latest visible assistant anchor
- manual partial split and rejoin helpers

Acceptance:

- JSON argument shrinking preserves parseability.
- Non-JSON argument text is not corrupted.
- Retained tool results have retained parent tool calls.
- Tail selection is deterministic and budget-aware.
- Partial compaction keeps the requested last N user exchanges.

## P1: Conversation Tool-Result Digest

Add deterministic old-tool-output digesting:

- pair assistant tool calls with tool results by call id
- deduplicate identical older tool results by stable hash
- replace old large terminal outputs with command, exit code, line count, and
  byte count
- replace old read-file results with path, line range, and size metadata
- replace old search results with pattern, match count, and top evidence
- keep recent tail tool results exact

Acceptance:

- Duplicate read-file outputs keep only the newest full copy.
- Old terminal output becomes a concise digest.
- Recent tail output remains verbatim.
- Sensitive keys and values are redacted before digest persistence.

## P1: Prompt Cache Hints

Add cache support without mutating provider payloads:

- `PromptCacheHint`
- stable static-prefix cache key from system prompt plus sorted tool schemas
- Anthropic-compatible carrier selection as hints only
- live-zone metadata shape for frozen prefixes and mutable blocks

Acceptance:

- Tool order does not change static-prefix cache key.
- Cache hints are reported separately from compression steps.
- Frozen byte ranges are left for adapters to splice byte-identically.

## P2: Summary Contract

Add optional summary interfaces:

```rust
pub trait SummaryProvider {
    fn summarize(&self, request: SummaryRequest)
        -> Result<StructuredSummary, SummaryError>;
}
```

Core support:

- summary prompt construction
- strict structured summary schema
- summary prefix and end marker
- metadata tagging so generated summaries are not treated as real user turns
- deterministic fallback summary from local anchors
- summary failure policy

Acceptance:

- Summary text cannot be mistaken for a fresh active user request.
- Auth and network summary failures preserve original messages.
- Deterministic fallback says it is local and may be incomplete.
- Repeated compactions update previous summaries instead of nesting them.

## P2: Context Breakdown

Add host-facing usage breakdown structs:

- system prompt
- tool definitions
- rules/context files
- skills
- MCP tools
- subagents
- memory
- conversation

Acceptance:

- Hosts can show why compression triggered without logging raw prompt text.
- Measured provider usage can override rough estimates in reports.

## What Not To Add

- Mandatory LLM summarization in core.
- Provider SDKs in TinyJuice.
- Session database writes.
- Compression leases implemented against OpenHuman storage inside core.
- Prompt cache mutation mixed into lossy compression APIs.

