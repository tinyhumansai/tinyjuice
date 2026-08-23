# tinyjuice-bus

The TinyBus wire contract for [TinyJuice](https://github.com/tinyhumansai/tinyjuice):
the interface names, the request and response types, and the contract version.

A host loads `tinyjuice-module` as a dynamic library and cannot import Rust
items from it. This crate is what supplies the call vocabulary instead. It is
`serde` and nothing else — no TinyBus, no async runtime, no compression code —
so linking it costs a host almost nothing.

The values here are **moved out of** the `tinyjuice` library rather than copied
from it: `tinyjuice::types` re-exports them, so there is one definition of each
and a host is looking at the same bytes the module validates against.
