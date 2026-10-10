use super::*;
use serde_json::json;

fn tool_over(
    lookup: impl Fn(String) -> Result<Option<String>, String> + Send + Sync + 'static,
) -> RetrieveToolOutputTool {
    RetrieveToolOutputTool::new(Arc::new(move |hash| {
        let reply = lookup(hash);
        Box::pin(async move { reply })
    }))
}

#[tokio::test]
async fn a_stored_original_is_handed_back() {
    let tool = tool_over(|hash| {
        assert_eq!(hash, "a1b2c3d4e5f6");
        Ok(Some("ORIGINAL PAYLOAD".to_string()))
    });
    let res = tool
        .execute(json!({ "hash": "  a1b2c3d4e5f6  " }))
        .await
        .unwrap();
    assert!(!res.is_error);
    assert_eq!(res.output(), "ORIGINAL PAYLOAD");
}

#[tokio::test]
async fn a_missing_or_blank_hash_is_an_error_without_a_lookup() {
    let tool = tool_over(|_| panic!("no lookup without a hash"));
    assert!(tool.execute(json!({})).await.unwrap().is_error);
    assert!(
        tool.execute(json!({ "hash": "  " }))
            .await
            .unwrap()
            .is_error
    );
}

#[tokio::test]
async fn a_lookup_failure_is_reported_verbatim() {
    let tool = tool_over(|_| Err("module offline".to_string()));
    let res = tool
        .execute(json!({ "hash": "deadbeefcafe" }))
        .await
        .unwrap();
    assert!(res.is_error);
    assert_eq!(res.output(), "module offline");
}

#[tokio::test]
async fn a_cache_miss_does_not_tell_the_model_to_re_run() {
    // The loop this guards against: a miss that says "re-run the tool" makes
    // the agent regenerate the same oversized result, which is compacted and
    // evicted again — an unbounded compact→retrieve→re-run loop.
    let tool = tool_over(|_| Ok(None));
    let msg = tool
        .execute(json!({ "hash": "deadbeefcafe" }))
        .await
        .unwrap()
        .output();
    let lowered = msg.to_lowercase();
    assert!(
        lowered.contains("do not re-run"),
        "a miss must explicitly discourage re-running: {msg}"
    );
    // Scan rather than match one phrase: every mention of re-running has to be
    // the negated one.
    let mut cursor = 0;
    while let Some(found) = lowered[cursor..].find("re-run") {
        let at = cursor + found;
        assert!(
            lowered[..at].trim_end().ends_with("do not"),
            "every mention of re-running must be negated, found a bare one at {at}: {msg}"
        );
        cursor = at + "re-run".len();
    }
    assert!(
        msg.contains("compacted summary"),
        "a miss must point the model at the summary it already has: {msg}"
    );
}

#[test]
fn the_tool_identity_is_stable() {
    let tool = tool_over(|_| Ok(None));
    assert_eq!(tool.name(), "retrieve_tool_output");
    assert_eq!(tool.permission_level(), PermissionLevel::ReadOnly);
    assert_eq!(tool.parameters_schema()["required"], json!(["hash"]));
}
