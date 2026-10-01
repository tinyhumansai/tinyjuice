use super::*;

fn request() -> GenerateRequest {
    GenerateRequest {
        context_token: "turn-1".into(),
        purpose: "tool_output_summary".into(),
        system: "system".into(),
        prompt: "prompt".into(),
        max_output_tokens: 16,
    }
}

#[tokio::test]
async fn declines_when_no_callback_is_configured() {
    let _guard = callback_test_guard().await;
    configure_callback(None);
    assert!(!has_callback());
    assert_eq!(generate(request()).await, Ok(None));
}

#[tokio::test]
async fn delegates_to_the_configured_callback() {
    let _guard = callback_test_guard().await;
    configure_callback(Some(Arc::new(|request: GenerateRequest| {
        Box::pin(async move {
            assert_eq!(request.context_token, "turn-1");
            Ok(Some(format!("reply to {}", request.prompt)))
        })
    })));
    assert_eq!(
        generate(request()).await.unwrap().as_deref(),
        Some("reply to prompt")
    );
    configure_callback(None);
}
