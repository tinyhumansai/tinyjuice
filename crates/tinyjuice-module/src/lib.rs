//! Loadable TinyBus adapter for the TinyJuice compression engine.
//!
//! The published `tinyjuice` crate stays bus-agnostic. This private workspace
//! crate is the separately shipped native module that keeps the compression
//! engine and its dependency tree out of an embedding host.

mod service;

pub use service::{BUS_NAME, OBJECT_PATH};
