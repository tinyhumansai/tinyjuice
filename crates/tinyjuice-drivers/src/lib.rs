//! TinyJuice on the host's `tinystoragedrivers` ports.
//!
//! [`BlobCcrStore`] is the CCR (compress-cache-retrieve) store's durable tier:
//! originals live as blobs under [`CCR_PREFIX`], each with the configured
//! lifetime, so expiry is the backend's concern rather than a directory sweep.
//! The backend (`SQLite`, `MongoDB`, files, memory) and the tenant scope are the
//! host's choice, made when it builds the [`BlobStore`] handle. Nothing here
//! names a database.
//!
//! [`tinyjuice::cache::CcrStore`] is synchronous, so the store owns a
//! [`Blocking`] bridge that runs the port's futures on a dedicated runtime
//! thread. That works from inside or outside a tokio runtime.
//!
//! [`BlobStore`]: tinystoragedrivers_core::BlobStore
//! [`Blocking`]: tinystoragedrivers_core::Blocking

mod blob_store;

pub use blob_store::{BlobCcrStore, CCR_PREFIX};
pub use tinyjuice_bus::storage::storage_schema;

#[cfg(test)]
#[path = "faulty_tests.rs"]
#[allow(clippy::unwrap_used, clippy::expect_used)]
mod faulty;
