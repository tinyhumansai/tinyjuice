# Deferred And Rejected Work

## Deferred: TurboQuant Vector Compression

TurboQuant-style vector compression is useful for embedding indexes and recall
storage, not for immediate prompt compression. It should be deferred until
TinyJuice has a memory/indexing surface.

Conditions before implementation:

- clear embedding boundary
- fixed vector config types
- ranking-quality fixtures
- corrupt payload rejection tests
- feature gate or separate module

Do not present vector storage ratios as token savings.

## Deferred: Heavy ML And Embeddings

ML text compression and embedding relevance should stay optional.

Reasons:

- OpenHuman already has runtime boundaries for Python/ML work.
- The core crate should remain small and deterministic by default.
- ML compressors can damage custom workflow tags and instructions without tag
  protection.

Conditions before expansion:

- tag-protection fixtures
- no-raw-content logging audit
- clear timeout/fallback behavior
- model unavailable path tested

## Deferred: SQLite, Redis, Database Drivers, Provider SDKs

These are useful behind adapters or feature gates, but not in the default core.

Reasons:

- They increase build, runtime, and security surface.
- They can couple TinyJuice to host deployment choices.
- Current memory/disk CCR is enough for initial OpenHuman integration.

## Deferred: Host Installers

Installers are product surface, not core compression. They should wait until the
CLI, protocol, doctor, and validation framework are stable.

First installers should target OpenHuman and Codex only.

## Rejected For Core: Web Fetching

TinyJuice should not fetch URLs. Hosts should perform fetch, auth, robots/safety
checks, and extraction. TinyJuice can reduce already extracted content.

## Rejected For Core: Filesystem Mutation

Batched editing, fuzzy matching, and validation can reduce tool loops, but
actual mutation belongs to host tools. TinyJuice core can provide report types
and compact validation renderers.

## Rejected For Default Path: Abstractive Summarization

Abstractive summarization can be valuable for conversation compaction, but it
should not be a default compressor:

- it can invent facts
- it needs model/provider choices
- it has failure modes that should preserve data
- it crosses from compression into interpretation

Use deterministic fallback summaries first, then adapter-supplied summary
providers with strict failure policy.

## Rejected: Compression Percentage Claims

Do not claim 80 percent, 90 percent, or any broad savings percentage until this
repository has benchmark fixtures and reports. Live savings should be labeled as
measured, counted, or estimated.

## Rejected: Raw Content Logging

No implementation plan should introduce raw prompt, context, tool-output, CCR,
web page, SQL result, or subagent transcript logging in library code. Reports
should use counts, hashes, categories, ids, and redacted labels.

