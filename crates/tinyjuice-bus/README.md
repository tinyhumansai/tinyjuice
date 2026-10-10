# tinyjuice-bus

The TinyBus wire contract for [TinyJuice](https://github.com/tinyhumansai/tinyjuice):
the interface names, the request and response types, and the contract version.

A host loads `tinyjuice-module` as a dynamic library and cannot import Rust
items from it. This crate is what supplies the call vocabulary instead. It is
`serde`, JSON Schema declarations, and error derives — no TinyBus, no async runtime, no compression code —
so hosts can declare tools and exchange typed requests without linking an engine.

The values here are **moved out of** the `tinyjuice` library rather than copied
from it: `tinyjuice::types` re-exports them, so there is one definition of each
and a host is looking at the same bytes the module validates against.


Contract 1.2 adds `Query(QueryRequest)` and `ExtractHtml(content)`. `Query`
accepts either a CCR handle or supplied content that the host has authorized,
plus a typed REPL operation, limits, and an optional summary callback ticket.
The response distinguishes expired handles and operation errors from transport
failures. `Repl(handle, op_json)` retains its existing two-argument form.

`repl` owns the serialized operation/result vocabulary; `tools` owns the stock
REPL and retrieval declarations and summary-focus schema. The implementation
crate re-exports these definitions for library compatibility. The summary
callback prompt and unavailable notices are shared in `summary`.
