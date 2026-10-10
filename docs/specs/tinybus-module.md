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
error. `ExtractHtml` takes one content string and returns `Result<String, HtmlError>`: Markdown on success or a structured input-size error. Publish a
new module release before hosts require these members, then pin that release's
artifact digest. Do not link a library fallback into the host.


### REPL wire vocabulary

`Query` takes a single request; the following request queries module storage
without returning the original to the host:

```json
{
  "target": {"kind": "handle", "token": "abc123"},
  "op": {"op": "find", "query": "ERROR", "mode": "grep", "context": 2}
}
```

Replace `target` with `{"kind":"content","content":"authorized artifact text"}`
to query supplied content. `find` supports `text` (default), `grep`, `regex`,
`rank`, `sed`, `awk`, and `jq`. `extract` takes `what` = `links` or `headings`;
`summarize` takes optional `hint` and `max_chars`. Every operation accepts
optional `scope` (a Python-style slice) and `unit` (`lines`, the default, or
`chars`). Optional find fields default to `ignore_case=false`, `context=0`,
and no `top_k`. Missing `scope`, `hint`, or `max_chars` means no supplied value.

Replies retain the legacy output tags `kind=lines|matches|search|links|headings|
values|text` inside `Ok`. Operation errors are in `Err`, tagged by `kind`:
`handle_not_found`, `input_too_large`, `invalid_pattern` (with `detail`),
`empty_query`, or `unsupported` (with `detail`). Hosts present cache misses
without re-running the original tool. Transport failures are separate bus errors.

Incoming `limits` may narrow the stock budgets, never expand them: 50 hits,
400 lines, 8,000 output characters, 240 characters per line, and 1 MiB of regex
compilation state. Supplied query content is limited to 10 MiB, matching the
host artifact file-read ceiling. HTML extraction accepts at most 8 MiB, matching
the web extractor input ceiling; excess input returns
`{"Err":{"kind":"input_too_large"}}` without parsing. A model summary obeys
both an explicit `max_chars` and the module's output cap. Without `max_chars`,
model summaries retain the output cap and deterministic overviews retain their
existing 2,000-character default.
Contract 1.1 modules do not provide the new members; contract 1.2 hosts must
require a compatible released artifact instead of calling a linked fallback.

`tinyjuice_bus::summary::SYSTEM_PROMPT` is the shared instruction text used by
the host's turn-bound summary callback. Import it through the contract when
building that callback; importing it does not execute a model. Its wording may
change between releases while the serialized callback request stays compatible.
`UnavailableReason` supplies content-free notices. Its disabled notice refers
to the configured summary scope, which may be a thread rather than a session.

Each `ReplLimits` field is a ceiling: `max_hits=50` selected matches,
`max_lines=400` range output and expanded grep lines, `max_output_chars=8000`
response characters, `max_line_chars=240` characters per returned line, and
`regex_size_limit=1048576` bytes of regex compilation state. Module requests may
narrow these values but cannot enlarge the module's stock ceilings. Grep keeps
matching lines before allocating remaining context slots. An explicit summary
`max_chars` also bounds a model-failure notice combined with its overview.
