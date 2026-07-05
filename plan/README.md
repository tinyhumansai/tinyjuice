# TinyJuice Implementation Plans

These plans are based on a source review of `src/` and the reference specs in
`docs/references/`. They are planning documents only.

TinyJuice is no longer just blank scaffolding. The current crate already has a
content router, deterministic reducers, per-kind compressors, CCR recovery,
rule loading, OpenHuman-facing tool-output hooks, savings callbacks, and a
large built-in command rule catalog. The implementation plan therefore focuses
on hardening and extending those boundaries rather than replacing them.

## Plan Set

- [Current State And Critique](current-state-and-critique.md)
- [Reference Algorithm Summary](reference-algorithm-summary.md)
- [OpenHuman Integration Plan](openhuman-integration-plan.md)
- [OpenHuman Algorithm Port Plan](openhuman-algorithm-port-plan.md)
- [Pipeline And CCR Plan](pipeline-and-ccr-plan.md)
- [Rule CLI And Safety Parity Plan](rule-cli-and-safety-parity-plan.md)
- [Content Compressor Roadmap](content-compressor-roadmap.md)
- [Conversation Compression Plan](conversation-compression-plan.md)
- [Adapter And Product Surfaces Plan](adapter-and-product-surfaces-plan.md)
- [Deferred And Rejected Work](deferred-and-rejected-work.md)

## Recommended Order

1. Stabilize current guarantees: exact-read bypasses, CCR recoverability,
   traceable skip reasons, and OpenHuman profile behavior.
2. Add the typed pipeline and injectable CCR store so lossy transforms cannot
   return output without proving recoverability.
3. Add safe-inventory command policy and `reduce-json` protocol parity.
4. Improve existing compressors with deterministic planners and fixtures.
5. Add OpenHuman conversation-level compression as a separate adapter layer.
6. Add artifact, web-extract, subagent, and SQL surfaces only after the protocol
   and storage boundaries are in place.

## Design Constraints To Preserve

- Core TinyJuice must stay independent of OpenHuman runtime internals.
- Public API changes must be documented in `README.md` or `docs/`.
- Lossy compaction should be recoverable through CCR or should decline.
- Exact prompt, file, and context input is sensitive. Library code must not log
  raw content.
- No compression-percentage claims should be made until fixture benchmarks
  exist in this repository.
- OpenHuman integration should be direct Rust crate use where possible, with a
  CLI protocol for non-Rust hosts.

