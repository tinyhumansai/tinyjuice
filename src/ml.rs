//! Optional ML compressor callback slot.
//!
//! TinyJuice stays host-agnostic: it never starts a Python server or knows
//! about OpenHuman config. A host may install an async callback that returns a
//! shorter plain-text view, and TinyJuice will otherwise decline the ML path.

use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, OnceLock, RwLock};

use crate::types::CompressOptions;

pub type MlCompressFuture = Pin<Box<dyn Future<Output = Result<Option<String>, String>> + Send>>;
pub type MlCompressCallback =
    dyn Fn(String, CompressOptions) -> MlCompressFuture + Send + Sync + 'static;

fn callback_cell() -> &'static RwLock<Option<Arc<MlCompressCallback>>> {
    static CALLBACK: OnceLock<RwLock<Option<Arc<MlCompressCallback>>>> = OnceLock::new();
    CALLBACK.get_or_init(|| RwLock::new(None))
}

/// Install or replace the host-provided ML compressor callback.
pub fn configure_callback(callback: Option<Arc<MlCompressCallback>>) {
    *callback_cell().write().unwrap_or_else(|p| p.into_inner()) = callback;
}

/// Compress text through the host callback, if one has been configured.
pub async fn compress(text: &str, opts: &CompressOptions) -> Result<Option<String>, String> {
    let callback = callback_cell()
        .read()
        .unwrap_or_else(|p| p.into_inner())
        .clone();
    let Some(callback) = callback else {
        return Ok(None);
    };
    callback(text.to_string(), opts.clone()).await
}
