# TinyJuice native module

`tinyjuice-module` is the trusted TinyBus adapter that lets a host load the
compression engine without linking the engine or its dependencies into the host
binary.

It serves `ai.tinyhumans.tinyjuice.Compression` at
`/ai/tinyhumans/tinyjuice/Compression` with the following methods:

- `Install` applies router and CCR configuration.
- `Detect` classifies content.
- `Compress` routes one content blob through the configured engine.
- `Compact` is the tool-output hot path with an agent compression profile.
- `CompactWith` carries arguments, focus, and a turn-bound summary callback ticket.
- `Repl` retains the legacy `(handle, op_json)` interface.
- `Query` runs a typed, bounded REPL operation against module CCR storage or supplied content.
- `ExtractHtml` converts HTML to Markdown without host-side parsing.
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


Contract 1.2 keeps every existing method's argument count. `Query` takes one
`QueryRequest` (camelCase fields). Its target is tagged by `kind`: `handle`
with `token`, or `content` with authorized artifact `content`. REPL operation
and output fields preserve their existing snake_case wire representation.
`limits` defaults to the stock REPL caps. `contextToken` and `scope` retain the
existing turn-bound model callback contract; only summarize uses it. Supplied
content is queried without inserting an entry in CCR. Modules own query,
HTML, pattern, slice, and model-summary algorithms; hosts own artifact path
validation, approvals, credentials, and callback lifetime.

`Query` returns a serialized `Result<ReplOutput, QueryError>`; an expired handle
is `{"Err":{"kind":"handle_not_found"}}`. A transport failure remains a bus
error. `ExtractHtml` takes one content string and returns Markdown. Publish a
new module release before hosts require these members, then pin that release's
artifact digest. Do not link a library fallback into the host.
