use super::*;
use tinyjuice::types::CompressOptions;

#[tokio::test]
async fn service_compresses_and_retrieves_a_large_log() {
    let service = Compression;
    service
        .install(InstallRequest {
            options: CompressOptions {
                min_bytes_to_compress: 64,
                ccr_min_tokens: 16,
                ..CompressOptions::default()
            },
            max_cache_entries: 8,
            max_cache_bytes: 1024 * 1024,
            ccr_ttl_secs: None,
            disk_tier_root: None,
        })
        .await
        .expect("install should succeed");

    let content = (0..200)
        .map(|index| {
            if index == 137 {
                format!("ERROR request failed id={index}\n")
            } else {
                format!("INFO request completed id={index}\n")
            }
        })
        .collect::<String>();
    let output = service
        .compress(
            content.clone(),
            ContentHint {
                explicit: Some(tinyjuice::types::ContentKind::Log),
                ..ContentHint::default()
            },
        )
        .await
        .expect("compression should succeed");
    assert!(output.applied);
    let token = output
        .ccr_token
        .expect("lossy compression should be recoverable");
    assert_eq!(
        service
            .retrieve(token, None)
            .await
            .expect("retrieve should succeed"),
        Some(content)
    );
}

#[tokio::test]
async fn service_repl_greps_a_stored_output_and_reports_bad_input() {
    let service = Compression;
    use tinyjuice::cache::CcrStore;
    let token = tinyjuice::cache::GlobalCcrStore
        .put("alpha\nbeta needle\ngamma\n")
        .token()
        .to_string();
    let reply = service
        .repl(token.clone(), r#"{"op":"find","query":"needle"}"#.into())
        .await
        .expect("repl should reply");
    let value: serde_json::Value = serde_json::from_str(&reply).unwrap();
    assert_eq!(value["hits"][0]["line"], 2);
    let bad = service.repl(token, "{".into()).await.unwrap();
    assert!(bad.contains("error"));
    let gone = service
        .repl(
            "missing".into(),
            r#"{"op":"extract","what":"links"}"#.into(),
        )
        .await
        .unwrap();
    assert!(gone.contains("expired"));
}

#[test]
fn caller_limits_cannot_expand_any_module_ceiling() {
    use tinyjuice_bus::repl::ReplLimits;
    let unlimited = ReplLimits {
        max_hits: usize::MAX,
        max_lines: usize::MAX,
        max_output_chars: usize::MAX,
        max_line_chars: usize::MAX,
        regex_size_limit: usize::MAX,
    };
    assert_eq!(query_limits(unlimited), ReplLimits::default());
    let narrowed = ReplLimits {
        max_hits: 1,
        max_lines: 2,
        max_output_chars: 3,
        max_line_chars: 4,
        regex_size_limit: 5,
    };
    assert_eq!(query_limits(narrowed), narrowed);
}

#[tokio::test]
async fn oversized_supplied_inputs_are_rejected_before_execution() {
    use tinyjuice_bus::wire::*;
    let query = QueryRequest {
        target: QueryTarget::Content {
            content: "x".repeat(MAX_QUERY_CONTENT_BYTES + 1),
        },
        op: serde_json::from_value(serde_json::json!({"op":"find","query":""})).unwrap(),
        limits: Default::default(),
        context_token: None,
        scope: None,
    };
    assert_eq!(
        Compression.query(query).await.unwrap(),
        Err(QueryError::InputTooLarge)
    );
    assert_eq!(
        Compression
            .extract_html("x".repeat(MAX_HTML_INPUT_BYTES + 1))
            .await
            .unwrap(),
        Err(HtmlError::InputTooLarge)
    );
}
