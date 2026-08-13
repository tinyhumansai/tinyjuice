# TinyJuice native module

`tinyjuice-module` is the trusted TinyBus adapter that lets a host load the
compression engine without linking the engine or its dependencies into the host
binary.

It serves `ai.tinyhumans.tinyjuice.Compression` at
`/ai/tinyhumans/tinyjuice/Compression` with six methods:

- `Install` applies router and CCR configuration.
- `Detect` classifies content.
- `Compress` routes one content blob through the configured engine.
- `Compact` is the tool-output hot path with an agent compression profile.
- `Retrieve` fetches a CCR original, optionally by range.
- `CacheStats` reports CCR occupancy.

The optional `ai.tinyhumans.tinyjuice.MlHost` callback remains host-owned. It
keeps Python/runtime provisioning and application configuration out of the
module while allowing the engine's ML compressor to call back over the same
in-process bus.
