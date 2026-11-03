use super::*;
use serde_json::json;
use std::sync::Mutex;

#[test]
fn range_endpoints_that_do_not_fit_usize_are_rejected() {
    assert!(usize_from_range_endpoint(4_294_967_296, u32::MAX as u64).is_err());
}

#[derive(Debug)]
struct LookupCall {
    token: String,
    range: Option<RetrieveRange>,
}

fn tool_with(
    reply: Result<Option<String>, String>,
    calls: Arc<Mutex<Vec<LookupCall>>>,
) -> RetrieveToolOutputTool {
    RetrieveToolOutputTool::new(Arc::new(move |token, range| {
        calls.lock().unwrap().push(LookupCall { token, range });
        let reply = reply.clone();
        Box::pin(async move { reply })
    }))
}

#[tokio::test]
async fn canonical_token_and_optional_range_reach_the_host_lookup() {
    let calls = Arc::new(Mutex::new(Vec::new()));
    let tool = tool_with(Ok(Some("slice".into())), Arc::clone(&calls));
    let result = tool
        .execute(json!({"token":"  abc123  ","range":{"start":4,"end":9,"unit":"bytes"}}))
        .await
        .unwrap();
    assert!(!result.is_error);
    assert_eq!(result.output(), "slice");
    let calls = calls.lock().unwrap();
    assert_eq!(calls.len(), 1);
    assert_eq!(calls[0].token, "abc123");
    let range = calls[0].range.as_ref().unwrap();
    assert_eq!((range.start, range.end), (4, 9));
    assert_eq!(range.unit, RangeUnit::Bytes);
}

#[tokio::test]
async fn legacy_hash_is_accepted_and_invalid_range_fields_keep_shipped_defaults() {
    let calls = Arc::new(Mutex::new(Vec::new()));
    let tool = tool_with(Ok(Some("text".into())), Arc::clone(&calls));
    let _ = tool
        .execute(json!({"hash":"legacy","range":{"start":-1,"unit":"unknown"}}))
        .await
        .unwrap();
    let calls = calls.lock().unwrap();
    assert_eq!(calls[0].token, "legacy");
    let range = calls[0].range.as_ref().unwrap();
    assert_eq!((range.start, range.end), (0, usize::MAX));
    assert_eq!(range.unit, RangeUnit::Lines);
}

#[tokio::test]
async fn missing_token_does_not_call_the_lookup() {
    let calls = Arc::new(Mutex::new(Vec::new()));
    let tool = tool_with(Ok(None), Arc::clone(&calls));
    assert!(tool.execute(json!({})).await.unwrap().is_error);
    assert!(calls.lock().unwrap().is_empty());
}

#[tokio::test]
async fn cache_miss_explains_preview_recovery_without_inviting_a_rerun() {
    let calls = Arc::new(Mutex::new(Vec::new()));
    let tool = tool_with(Ok(None), calls);
    let message = tool
        .execute(json!({"token":"evicted"}))
        .await
        .unwrap()
        .output();
    assert!(message.contains("Do NOT re-run"));
    assert!(message.contains("compacted summary"));
}

#[tokio::test]
async fn host_lookup_errors_are_preserved() {
    let calls = Arc::new(Mutex::new(Vec::new()));
    let tool = tool_with(Err("module unavailable".into()), calls);
    let result = tool.execute(json!({"token":"t"})).await.unwrap();
    assert!(result.is_error);
    assert_eq!(result.output(), "module unavailable");
}

#[test]
fn tool_identity_and_schema_match_the_current_bus_declaration() {
    let calls = Arc::new(Mutex::new(Vec::new()));
    let tool = tool_with(Ok(None), calls);
    assert_eq!(tool.name(), tinyjuice_bus::tools::RETRIEVE);
    assert_eq!(tool.description(), RETRIEVE_TOOL_DESCRIPTION);
    assert_eq!(tool.parameters_schema(), retrieve_tool_parameters());
    assert_eq!(tool.permission_level(), PermissionLevel::ReadOnly);
    assert!(tool.is_concurrency_safe(&json!({})));
    assert!(!tool.external_effect());
}

#[derive(Default)]
struct CaptureLogger {
    records: Mutex<Vec<String>>,
}

impl log::Log for CaptureLogger {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        metadata.level() <= log::Level::Debug
    }

    fn log(&self, record: &log::Record) {
        if self.enabled(record.metadata()) {
            self.records.lock().unwrap().push(record.args().to_string());
        }
    }

    fn flush(&self) {}
}

static CAPTURE_LOGGER: std::sync::OnceLock<CaptureLogger> = std::sync::OnceLock::new();

/// Install the capturing debug logger once per test process.
fn capture_logger() -> &'static CaptureLogger {
    let logger = CAPTURE_LOGGER.get_or_init(CaptureLogger::default);
    let _ = log::set_logger(logger);
    log::set_max_level(log::LevelFilter::Debug);
    logger
}

fn captured_logs() -> Vec<String> {
    capture_logger().records.lock().unwrap().clone()
}

#[tokio::test]
async fn successful_retrieve_never_logs_the_lookup_token() {
    capture_logger().records.lock().unwrap().clear();
    let token = "redact-me-9f2c4a1b";
    let calls = Arc::new(Mutex::new(Vec::new()));
    let tool = tool_with(Ok(Some("slice".into())), Arc::clone(&calls));
    let result = tool.execute(json!({"token": token})).await.unwrap();
    assert!(!result.is_error);
    let leaked: Vec<String> = captured_logs()
        .into_iter()
        .filter(|line| line.contains(token))
        .collect();
    assert!(
        leaked.is_empty(),
        "lookup token leaked into logs: {leaked:?}"
    );
}
