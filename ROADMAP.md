# TinyJuice Roadmap

TinyJuice is pre-1.0, but the core compression path is no longer blank
scaffolding. The router, several deterministic compressors, CCR recovery,
command-output rules, OpenHuman-style tool-output adapter, and analytics UI are
implemented. The near-term work is to harden those surfaces with fixtures,
quality gates, and host integration.

## Completed Foundations

- core compression trait, input model, output report, and error taxonomy
- content router with `CompressOptions` and `ContentHint`
- content-kind detection for JSON, code, logs, search, diffs, HTML, and text
- deterministic JSON, code, log, search, diff, HTML, and generic command paths
- CCR exact-original recovery with memory and optional disk tier
- built-in command reduction rules with user/project overlays
- OpenHuman-style tool-output adapter and per-agent profiles
- savings recorder and optional ML callback boundaries
- metadata-oriented analytics interface

## Next Milestones

1. Build benchmark fixtures for compression ratio, latency, retained facts, and
   regression safety.
2. Publish only fixture-backed savings claims; keep general percentage claims
   out of docs until then.
3. Harden OpenHuman integration around runtime config, recovery tool exposure,
   and dashboard attribution.
4. Expand source-code compression coverage and tree-sitter behavior tests.
5. Add deterministic plain-text extraction before relying on optional ML text
   compression.
6. Improve JSON planning for sparse, heterogeneous, nested, and query-aware
   arrays.
7. Add more fixture coverage for diff noise, repetitive logs, search ranking,
   and custom tags.

## Continuing Non-Goals

- no model-provider network calls in the core crate
- no OpenHuman runtime dependency without an explicit feature or adapter boundary
- no benchmark or quality claims without fixture evidence
- no raw prompt, context, or tool-output analytics
