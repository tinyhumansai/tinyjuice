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

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::sync::Mutex;

    static TEST_LOCK: Mutex<()> = Mutex::const_new(());

    #[tokio::test]
    async fn declines_when_no_callback_is_configured() {
        let _guard = TEST_LOCK.lock().await;
        configure_callback(None);

        let result = compress("plain text", &CompressOptions::default())
            .await
            .expect("missing callback is not an error");

        assert_eq!(result, None);
    }

    #[tokio::test]
    async fn delegates_to_configured_callback_with_owned_options() {
        let _guard = TEST_LOCK.lock().await;
        configure_callback(Some(Arc::new(|text, opts| {
            Box::pin(async move {
                assert_eq!(text, "alpha beta gamma");
                assert!(opts.ml_text_enabled);
                Ok(Some("alpha gamma".to_string()))
            })
        })));

        let opts = CompressOptions {
            ml_text_enabled: true,
            ..Default::default()
        };
        let result = compress("alpha beta gamma", &opts)
            .await
            .expect("callback result should propagate");

        assert_eq!(result.as_deref(), Some("alpha gamma"));
        configure_callback(None);
    }

    #[tokio::test]
    async fn propagates_callback_errors() {
        let _guard = TEST_LOCK.lock().await;
        configure_callback(Some(Arc::new(|_, _| {
            Box::pin(async { Err("runtime offline".to_string()) })
        })));

        let error = compress("alpha beta gamma", &CompressOptions::default())
            .await
            .expect_err("callback errors should be returned to caller");

        assert_eq!(error, "runtime offline");
        configure_callback(None);
    }
}
