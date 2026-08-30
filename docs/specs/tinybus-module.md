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

## Wire shapes

All object fields use camelCase. `Install` accepts one object with `options`,
`maxCacheEntries`, `maxCacheBytes`, and the optional `ccrTtlSecs` and
`diskTierRoot`. `options` is a `CompressOptions` object; omitted option fields
take their defaults, so a future knob does not break an older host.

`Compress` accepts `(content, hint)`. The hint fields `mime`, `extension`,
`sourceTool`, `query`, and `explicit` are all optional. It returns
`CompressedOutput`: `text`, `contentKind`, `compressor`, `lossy`, `applied`,
`originalBytes`, `compactedBytes`, and optional `ccrToken`.

`Retrieve` accepts `(token, range)`, where `range` is optional. A range contains
`start`, `end`, and `unit` (`bytes` or `lines`). The response is either the
retrieved string or `null` when the token is not retained.
