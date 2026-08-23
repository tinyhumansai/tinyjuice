//! Every type that crosses `TinyJuice`'s `TinyBus` boundary, and the names of
//! the members that carry them.
//!
//! A host loads the `tinyjuice-module` dynamic library but cannot import Rust
//! items from that binary. This transport-free crate is the ordinary library
//! that supplies its call vocabulary: interface names, request and response
//! types, and the compatibility rule for that vocabulary.
//!
//! It deliberately contains no `TinyBus` transport, runtime, or compression
//! behaviour. Hosts own their connection and policies; `tinyjuice-module` owns
//! the adapter; and the root `tinyjuice` crate remains host-agnostic and
//! re-exports the shared values from here rather than defining a second copy.

pub mod names;
pub mod types;
pub mod version;
pub mod wire;

pub use names::{BUS_NAME, METHODS, ML_HOST_NAME, ML_HOST_PATH, OBJECT_PATH};
pub use types::{
    AgentTokenjuiceCompression, CompressOptions, CompressedOutput, CompressorKind, ContentHint,
    ContentKind,
};
pub use version::{is_compatible, CONTRACT_VERSION};
pub use wire::{CacheStats, CompactResponse, InstallRequest, RangeUnit, RetrieveRange};

#[cfg(test)]
mod test;
