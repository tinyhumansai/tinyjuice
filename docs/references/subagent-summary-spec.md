# Subagent Summary Spec

## Status

Design reference for compressing delegated exploration results before they return
to the main conversation.

## Goal

Allow a cheaper or read-only worker to inspect broad context, then return a
bounded summary instead of a full transcript.

## Inputs

```text
SubagentSummaryInput {
  task: String,
  transcript: Vec<SubagentEvent>,
  evidence_policy: EvidencePolicy,
  max_bytes: usize,
}

SubagentEvent {
  role: Role,
  content: String,
  tool_name: Option<String>,
  metadata: EventMetadata,
}
```

## Output

```text
SubagentSummaryOutput {
  conclusion: String,
  findings: Vec<Finding>,
  evidence: Vec<EvidenceRef>,
  open_questions: Vec<String>,
  omitted: OmissionReport,
}
```

## Algorithm

1. Segment transcript into task, tool calls, tool outputs, prose, and final
   answer.
2. Prefer the final answer when it includes evidence.
3. Extract concrete findings and file references.
4. Preserve short evidence snippets and line/path references.
5. Drop exploratory dead ends unless they explain uncertainty.
6. Enforce `max_bytes` with an omission report.

## Compression Contract

The summary must answer:

- what was found;
- where it was found;
- what remains uncertain;
- what action the main agent should take next.

## Safety Rules

- Do not invent evidence.
- Preserve file paths and line numbers when available.
- Mark inferences as inferences.
- Keep enough detail for verification before editing.

## TinyJuice Fit

Suggested modules:

```text
src/compressors/signals.rs
src/compressors/generic.rs
src/types.rs
```

The core crate should not choose models; it should only define how a delegated
transcript is reduced.

## Test Fixtures

- long transcript reduced to conclusion plus evidence;
- failed exploration keeps failure reason;
- contradictory findings remain as uncertainty;
- byte budget produces an omission report.
