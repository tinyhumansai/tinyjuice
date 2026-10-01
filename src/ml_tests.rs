use super::*;

#[tokio::test]
async fn declines_when_no_callback_is_configured() {
    let _guard = callback_test_guard().await;
    configure_callback(None);

    let result = compress("plain text", &CompressOptions::default())
        .await
        .expect("missing callback is not an error");

    assert_eq!(result, None);
}

#[tokio::test]
async fn delegates_to_configured_callback_with_owned_options() {
    let _guard = callback_test_guard().await;
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
    let _guard = callback_test_guard().await;
    configure_callback(Some(Arc::new(|_, _| {
        Box::pin(async { Err("runtime offline".to_string()) })
    })));

    let error = compress("alpha beta gamma", &CompressOptions::default())
        .await
        .expect_err("callback errors should be returned to caller");

    assert_eq!(error, "runtime offline");
    configure_callback(None);
}
