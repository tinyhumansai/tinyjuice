//! `juice_summarize` backed by the host's model, with the deterministic
//! overview as its fallback.

use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};

use super::*;
use crate::cache::store::MemoryCcrStore;
use crate::llm::{self, GenerateRequest};
use crate::types::CompressOptions;

fn lim() -> ReplLimits {
    ReplLimits::default()
}

/// A stored output unique to each test, so the process-wide summary cache
/// never answers for a call another test made.
fn stored(tag: &str) -> (MemoryCcrStore, String, String) {
    let store = MemoryCcrStore::default();
    let text: String = (0..40)
        .map(|i| format!("{tag} line {i}: the service answered ok\n"))
        .collect();
    let token = store.put(&text).token().to_string();
    (store, token, text)
}

fn model(scope: &str) -> ModelSummary {
    ModelSummary {
        options: CompressOptions {
            llm_summary_enabled: true,
            ..CompressOptions::default()
        },
        context_token: "turn-1".into(),
        scope: Some(scope.into()),
    }
}

fn summarize(hint: Option<&str>, scope: Option<&str>) -> ReplOp {
    ReplOp::Summarize {
        max_chars: None,
        hint: hint.map(Into::into),
        scope: scope.map(Into::into),
        unit: ScopeUnit::Lines,
    }
}

fn recording(reply: Result<Option<String>, String>) -> Arc<Mutex<Vec<GenerateRequest>>> {
    let seen = Arc::new(Mutex::new(Vec::new()));
    let sink = seen.clone();
    llm::configure_callback(Some(Arc::new(move |request: GenerateRequest| {
        sink.lock().unwrap().push(request);
        let reply = reply.clone();
        Box::pin(async move { reply })
    })));
    seen
}

fn text_of(out: ReplOutput) -> String {
    match out {
        ReplOutput::Text { text } => text,
        other => panic!("expected text, got {other:?}"),
    }
}

#[tokio::test]
async fn supplied_and_stored_model_fallback_obey_explicit_character_budget() {
    let _guard = llm::callback_test_guard().await;
    let (store, token, text) = stored("bounded-fallback");
    let _requests = recording(Err("fixture model failure".into()));
    let model = model("bounded-fallback");
    for budget in [0, 16, 32] {
        let op = ReplOp::Summarize {
            hint: None,
            scope: None,
            unit: ScopeUnit::Lines,
            max_chars: Some(budget),
        };
        let supplied = text_of(
            run_on_text_with_model(&text, &op, &lim(), Some(&model))
                .await
                .unwrap(),
        );
        let stored = text_of(
            run_op_with_model(&store, &token, &op, &lim(), Some(&model))
                .await
                .unwrap(),
        );
        assert!(
            supplied.chars().count() <= budget,
            "supplied fallback exceeded {budget}: {supplied}"
        );
        assert!(
            stored.chars().count() <= budget,
            "stored fallback exceeded {budget}: {stored}"
        );
    }
    llm::configure_callback(None);
}

#[tokio::test]
async fn summarize_with_a_model_makes_exactly_one_call() {
    let _guard = llm::callback_test_guard().await;
    let seen = recording(Ok(Some("model gist".into())));
    let (store, token, _) = stored("model-one-call");
    let out = run_op_with_model(
        &store,
        &token,
        &summarize(Some("the failures"), None),
        &lim(),
        Some(&model("model-one-call")),
    )
    .await
    .unwrap();
    assert_eq!(text_of(out), "model gist");
    let requests = seen.lock().unwrap();
    assert_eq!(requests.len(), 1);
    assert_eq!(requests[0].context_token, "turn-1");
    assert!(requests[0].prompt.contains("Caller focus: the failures"));
    drop(requests);
    llm::configure_callback(None);
}

#[tokio::test]
async fn summarize_with_a_model_reads_only_the_scope() {
    let _guard = llm::callback_test_guard().await;
    let seen = recording(Ok(Some("scoped gist".into())));
    let (store, token, _) = stored("model-scope");
    run_op_with_model(
        &store,
        &token,
        &summarize(None, Some("[3:5]")),
        &lim(),
        Some(&model("model-scope")),
    )
    .await
    .unwrap();
    let prompt = seen.lock().unwrap()[0].prompt.clone();
    assert!(prompt.contains("model-scope line 3:"));
    assert!(prompt.contains("model-scope line 4:"));
    assert!(!prompt.contains("model-scope line 5:"));
    assert!(!prompt.contains("model-scope line 0:"));
    llm::configure_callback(None);
}

#[tokio::test]
async fn a_failed_model_summary_falls_back_to_the_overview_with_a_note() {
    let _guard = llm::callback_test_guard().await;
    recording(Err("model offline".into()));
    let (store, token, text) = stored("model-failed");
    let op = summarize(None, None);
    let out = run_op_with_model(&store, &token, &op, &lim(), Some(&model("model-failed")))
        .await
        .unwrap();
    let overview = text_of(run_on_text(&text, &op, &lim()).unwrap());
    let out = text_of(out);
    assert!(out.starts_with("[model summary unavailable"), "{out}");
    assert!(out.ends_with(&overview));
    llm::configure_callback(None);
}

#[tokio::test(start_paused = true)]
async fn a_timed_out_model_summary_says_to_ask_again() {
    let _guard = llm::callback_test_guard().await;
    llm::configure_callback(Some(Arc::new(|_| {
        Box::pin(async {
            tokio::time::sleep(std::time::Duration::from_secs(60)).await;
            Ok(Some("late".to_string()))
        })
    })));
    let (store, token, _) = stored("model-timeout");
    let mut slow = model("model-timeout");
    slow.options.llm_summary_timeout_ms = 50;
    let out = run_op_with_model(&store, &token, &summarize(None, None), &lim(), Some(&slow))
        .await
        .unwrap();
    let out = text_of(out);
    assert!(out.contains("call juice_summarize again"), "{out}");
    llm::configure_callback(None);
}

#[tokio::test]
async fn without_a_model_or_for_other_ops_nothing_calls_it() {
    let _guard = llm::callback_test_guard().await;
    let calls = Arc::new(AtomicUsize::new(0));
    let counter = calls.clone();
    llm::configure_callback(Some(Arc::new(move |_| {
        counter.fetch_add(1, Ordering::SeqCst);
        Box::pin(async { Ok(Some("gist".to_string())) })
    })));
    let (store, token, _) = stored("model-none");
    let op = summarize(None, None);
    assert_eq!(
        run_op_with_model(&store, &token, &op, &lim(), None)
            .await
            .unwrap(),
        run_op(&store, &token, &op, &lim()).unwrap()
    );
    let find = ReplOp::Find {
        query: "line 3".into(),
        mode: FindMode::Text,
        ignore_case: false,
        context: 0,
        top_k: None,
        scope: None,
        unit: ScopeUnit::Lines,
    };
    assert_eq!(
        run_op_with_model(&store, &token, &find, &lim(), Some(&model("model-none")))
            .await
            .unwrap(),
        run_op(&store, &token, &find, &lim()).unwrap()
    );
    assert_eq!(
        run_op_with_model(&store, "nope", &op, &lim(), Some(&model("model-none"))).await,
        Err(ReplError::HandleNotFound)
    );
    assert_eq!(calls.load(Ordering::SeqCst), 0);
    llm::configure_callback(None);
}

#[cfg(feature = "tinytools")]
#[tokio::test]
async fn the_summarize_tool_uses_the_model_it_was_given() {
    let _guard = llm::callback_test_guard().await;
    let seen = recording(Ok(Some("tool gist".into())));
    let (store, token, _) = stored("model-tool");
    let tools =
        super::tools::repl_tools_with_model(Arc::new(store), lim(), Some(model("model-tool")));
    let tool = tools
        .iter()
        .find(|t| t.name() == "juice_summarize")
        .unwrap();
    let result = tool
        .execute(serde_json::json!({ "handle": token, "hint": "status" }))
        .await
        .unwrap();
    assert!(!result.is_error, "{result:?}");
    assert!(format!("{result:?}").contains("tool gist"));
    assert_eq!(seen.lock().unwrap().len(), 1);
    llm::configure_callback(None);
}

#[tokio::test]
async fn a_model_summary_honors_max_chars_for_supplied_content() {
    let _guard = llm::callback_test_guard().await;
    recording(Ok(Some("résumé ".repeat(100))));
    let (_, _, text) = stored("model-artifact-cap");
    let mut op = summarize(None, None);
    if let ReplOp::Summarize { max_chars, .. } = &mut op {
        *max_chars = Some(17);
    }
    let out = run_on_text_with_model(&text, &op, &lim(), Some(&model("model-artifact-cap")))
        .await
        .unwrap();
    assert_eq!(text_of(out).chars().count(), 17);
    llm::configure_callback(None);
}

#[tokio::test(start_paused = true)]
async fn a_supplied_content_timeout_does_not_invent_a_recovery_handle() {
    let _guard = llm::callback_test_guard().await;
    llm::configure_callback(Some(Arc::new(|_| Box::pin(std::future::pending()))));
    let (_, _, text) = stored("model-artifact-timeout");
    let mut slow = model("model-artifact-timeout");
    slow.options.llm_summary_timeout_ms = 50;
    let out = run_on_text_with_model(&text, &summarize(None, None), &lim(), Some(&slow))
        .await
        .unwrap();
    let out = text_of(out);
    assert!(out.contains("retry this query"), "{out}");
    assert!(!out.contains("same handle"), "{out}");
    llm::configure_callback(None);
}
