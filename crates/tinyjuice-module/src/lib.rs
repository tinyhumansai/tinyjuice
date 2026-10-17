//! Loadable TinyBus adapter for the TinyJuice compression engine.
//!
//! The published `tinyjuice` crate stays bus-agnostic. This private workspace
//! crate is the separately shipped native module that keeps the compression
//! engine and its dependency tree out of an embedding host.

mod service;
mod storage;

/// Entry points for hosts that link `TinyJuice` into their own executable.
#[cfg(feature = "static-link")]
pub mod linked {
    pub use crate::service::exports::{
        TINYBUS_MODULE_ABI_V1, tinybus_module_init_v1, tinybus_module_manifest_v1,
    };
}

#[cfg(feature = "static-link")]
pub use service::exports::linked_module;

pub use service::{BUS_NAME, OBJECT_PATH};

/// What the module stores. A host composes it into its schema registry and
/// derives the module's storage grant from it, so the grant never repeats the
/// blob prefix by hand.
pub use tinyjuice_drivers::storage_schema;
