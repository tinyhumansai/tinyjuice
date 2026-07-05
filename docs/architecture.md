# Architecture

TinyJuice is organized around two public layers:

1. a small `Compressor` trait scaffold for simple strategy boundaries
2. the TokenJuice content router for real tool-output compaction

The router detects content kind, chooses a specialized compressor, declines when
compression is unsafe or not smaller, and stores exact originals in CCR when a
lossy view is emitted.

The core crate remains independent of OpenHuman runtime internals. OpenHuman and
other hosts install policy through `CompressOptions`, agent profiles, CCR
configuration, optional ML callbacks, and optional savings recorders.

TinyJuice should prefer preventing redundant context from entering the model
over lossy mid-task prompt compaction. Tool-output reducers, command rules,
batched operations, validation reports, and explicit recovery metadata are the
primary integration points.

## Main Technical Docs

- [Wiki home](../wiki/Home.md)
- [Capabilities](../wiki/Capabilities.md)
- [Router and Compressors](../wiki/Router-and-Compressors.md)
- [Rule Engine](../wiki/Rule-Engine.md)
- [CCR Recovery](../wiki/CCR-Recovery.md)
- [OpenHuman Integration](../wiki/OpenHuman-Integration.md)

## Design References

- [Reference specs](references/README.md)
