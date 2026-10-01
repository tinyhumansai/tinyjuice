use super::*;
use serde_json::json;

fn stringified_json_rows() -> String {
    let rows: Vec<_> = (0..80)
        .map(|i| {
            json!({
                "id": i,
                "status": "active",
                "metadata": json!({
                    "owner": format!("team-{i}"),
                    "flags": { "retry": i % 2 == 0 }
                })
                .to_string()
            })
        })
        .collect();
    serde_json::to_string_pretty(&rows).expect("rows serialize")
}

#[tokio::test]
async fn skips_short_output() {
    let (out, stats) = compact_tool_output_with_policy(
        "shell",
        None,
        "hello world",
        Some(0),
        AgentTokenjuiceCompression::Full,
    )
    .await;
    assert_eq!(out, "hello world");
    assert!(!stats.applied);
    assert_eq!(stats.original_bytes, 11);
}

#[tokio::test]
async fn compacts_long_git_status_via_argv() {
    let mut lines = vec!["On branch main".to_owned()];
    for i in 0..200 {
        lines.push(format!("\tmodified:   src/file_{i}.rs"));
    }
    let output = lines.join("\n");
    let args = json!({"command": "git status"});
    let (compacted, stats) = compact_tool_output_with_policy(
        "shell",
        Some(&args),
        &output,
        Some(0),
        AgentTokenjuiceCompression::Full,
    )
    .await;
    assert!(stats.applied, "expected compaction, got {:?}", stats);
    assert!(compacted.len() < output.len());
    assert!(
        compacted.contains("M: src/file_0.rs"),
        "git/status rule should rewrite modified paths: {compacted}"
    );
    assert!(
        !compacted.contains("On branch main"),
        "git/status rule should remove branch boilerplate: {compacted}"
    );
}

#[tokio::test]
async fn compacts_stringified_json_through_tool_adapter() {
    let output = stringified_json_rows();
    let args = json!({"path": "accounts.json"});

    let (compacted, stats) = compact_tool_output_with_policy(
        "web_fetch",
        Some(&args),
        &output,
        Some(0),
        AgentTokenjuiceCompression::Full,
    )
    .await;

    assert!(stats.applied, "expected SmartCrusher, got {:?}", stats);
    assert_eq!(stats.rule_id, "smartcrusher");
    assert!(compacted.contains("metadata.owner"), "{compacted}");
    assert!(compacted.contains("metadata.flags.retry"), "{compacted}");
    assert!(compacted.contains("team-7"), "{compacted}");
    assert!(compacted.contains("tinyjuice_retrieve"), "{compacted}");
    assert!(compacted.len() < output.len());
}

#[tokio::test]
async fn passes_through_incompressible_output() {
    let unique_lines: Vec<String> = (0..200)
        .map(|i| format!("unique-payload-chunk-{i}-{}", "x".repeat(30)))
        .collect();
    let output = unique_lines.join("\n");
    let (returned, stats) = compact_tool_output_with_policy(
        "unknown_tool",
        None,
        &output,
        Some(0),
        AgentTokenjuiceCompression::Full,
    )
    .await;
    if !stats.applied {
        assert_eq!(returned, output);
    }
}

#[tokio::test]
async fn disabled_flag_is_passthrough() {
    let big = "x".repeat(5000);
    assert_eq!(compact_output(big.clone(), "grep", false).await, big);
}

#[tokio::test]
async fn light_agent_profile_declines_lossy_ccr_compaction() {
    let mut lines = vec!["On branch main".to_owned()];
    for i in 0..200 {
        lines.push(format!("\tmodified:   src/file_{i}.rs"));
    }
    let output = lines.join("\n");
    let args = json!({"command": "git status"});
    let (returned, stats) = compact_tool_output_with_policy(
        "shell",
        Some(&args),
        &output,
        Some(0),
        AgentTokenjuiceCompression::Light,
    )
    .await;
    assert_eq!(returned, output);
    assert!(!stats.applied);
}

#[tokio::test]
async fn off_agent_profile_bypasses_router() {
    let big = "x".repeat(5000);
    let returned =
        compact_output_with_policy(big.clone(), "grep", true, AgentTokenjuiceCompression::Off)
            .await;
    assert_eq!(returned, big);
}

#[tokio::test]
async fn auto_agent_profile_requires_host_resolution() {
    let mut lines = vec!["On branch main".to_owned()];
    for i in 0..200 {
        lines.push(format!("\tmodified:   src/file_{i}.rs"));
    }
    let output = lines.join("\n");
    let args = json!({"command": "git status"});
    let (returned, stats) = compact_tool_output_with_policy(
        "shell",
        Some(&args),
        &output,
        Some(0),
        AgentTokenjuiceCompression::Auto,
    )
    .await;

    assert_eq!(returned, output);
    assert!(!stats.applied);
    assert_eq!(stats.rule_id, "none/agent-profile-auto-unresolved");
}

#[test]
fn extract_argv_handles_common_shapes() {
    let (cmd, argv) = extract_command_argv(Some(&json!({"command": "git status"})));
    assert_eq!(cmd.as_deref(), Some("git status"));
    assert_eq!(argv.unwrap(), vec!["git", "status"]);

    let (cmd, _) = extract_command_argv(Some(&json!({"command": "cargo", "args": ["test"]})));
    assert_eq!(cmd.as_deref(), Some("cargo test"));
}

#[test]
fn extract_extension_and_query() {
    assert_eq!(
        extract_extension(Some(&json!({"path": "src/lib.rs"}))).as_deref(),
        Some("rs")
    );
    assert_eq!(
        extract_query(Some(&json!({"pattern": "foo bar"}))).as_deref(),
        Some("foo bar")
    );
}

/// Turn the summary stage on in the global options. Safe alongside the
/// other tests here: none of them passes a context token, so the stage
/// still declines for them.
fn enable_llm_summary() {
    let mut opts = current_options();
    opts.llm_summary_enabled = true;
    opts.llm_summary_threshold_tokens = 10;
    configure(opts);
}

fn call<'a>(
    output: &'a str,
    profile: AgentTokenjuiceCompression,
    focus: Option<&'a str>,
) -> ToolOutputCall<'a> {
    ToolOutputCall {
        tool_name: "web_fetch",
        arguments: None,
        output,
        exit_code: None,
        profile,
        compaction_enabled: true,
        focus,
        context_token: Some("turn-7"),
        scope: Some("tool-integration"),
    }
}

#[tokio::test]
async fn the_full_profile_summarizes_with_the_callers_focus() {
    let _guard = crate::llm::callback_test_guard().await;
    enable_llm_summary();
    let seen = std::sync::Arc::new(std::sync::Mutex::new(Vec::<String>::new()));
    let sink = seen.clone();
    crate::llm::configure_callback(Some(std::sync::Arc::new(
        move |request: crate::llm::GenerateRequest| {
            sink.lock().unwrap().push(request.prompt);
            Box::pin(async { Ok(Some("focused note".to_string())) })
        },
    )));

    let output = "integration full profile ".repeat(60);
    let report = compact_tool_output(call(
        &output,
        AgentTokenjuiceCompression::Full,
        Some("the rate limits"),
    ))
    .await;
    assert!(report.text.starts_with("focused note"));
    assert_eq!(report.stats.rule_id, "llm_summary");
    assert!(report.stats.applied);
    assert!(report.notice.is_none());
    assert!(seen.lock().unwrap()[0].contains("Caller focus: the rate limits"));
    crate::llm::configure_callback(None);
}

#[tokio::test]
async fn the_light_profile_never_summarizes_unasked() {
    let _guard = crate::llm::callback_test_guard().await;
    enable_llm_summary();
    crate::llm::configure_callback(Some(std::sync::Arc::new(|_| {
        Box::pin(async { panic!("light without a context token must not reach the model") })
    })));
    let output = "integration light profile ".repeat(60);
    let report = compact_tool_output(ToolOutputCall {
        context_token: None,
        ..call(&output, AgentTokenjuiceCompression::Light, Some("anything"))
    })
    .await;
    assert_ne!(report.stats.rule_id, "llm_summary");
    crate::llm::configure_callback(None);
}

/// A host that binds a summary call to a light-profile result (OpenHuman's
/// orchestrator, whose `coding` model hint resolves to `Light`) gets its
/// summary, and the exact original stays retrievable from CCR.
#[tokio::test]
async fn the_light_profile_summarizes_when_the_host_asks() {
    let _guard = crate::llm::callback_test_guard().await;
    enable_llm_summary();
    let calls = std::sync::Arc::new(std::sync::atomic::AtomicUsize::new(0));
    let counter = calls.clone();
    crate::llm::configure_callback(Some(std::sync::Arc::new(move |_| {
        counter.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        Box::pin(async { Ok(Some("light note".to_string())) })
    })));
    let output = "integration light profile asked ".repeat(60);
    let report = compact_tool_output(ToolOutputCall {
        scope: Some("tool-integration-light-asked"),
        ..call(&output, AgentTokenjuiceCompression::Light, None)
    })
    .await;
    assert_eq!(calls.load(std::sync::atomic::Ordering::SeqCst), 1);
    assert!(report.text.starts_with("light note"), "{}", report.text);
    assert_eq!(report.stats.rule_id, "llm_summary");
    assert!(report.stats.applied);
    assert!(
        report
            .text
            .contains(crate::cache::marker::RETRIEVE_TOOL_NAME),
        "the exact original stays retrievable: {}",
        report.text
    );
    crate::llm::configure_callback(None);
}

#[tokio::test]
async fn a_failed_summary_falls_through_with_a_notice() {
    let _guard = crate::llm::callback_test_guard().await;
    enable_llm_summary();
    crate::llm::configure_callback(Some(std::sync::Arc::new(|_| {
        Box::pin(async { Err("offline".to_string()) })
    })));
    let output = "integration failed summary ".repeat(60);
    let report = compact_tool_output(ToolOutputCall {
        scope: Some("tool-integration-failure"),
        ..call(&output, AgentTokenjuiceCompression::Full, None)
    })
    .await;
    assert_ne!(report.stats.rule_id, "llm_summary");
    assert_eq!(
        report.notice,
        Some(crate::summarize::UnavailableReason::Failed.notice())
    );
    crate::llm::configure_callback(None);
}

#[tokio::test]
async fn a_summary_runs_even_with_compaction_disabled() {
    let _guard = crate::llm::callback_test_guard().await;
    enable_llm_summary();
    crate::llm::configure_callback(Some(std::sync::Arc::new(|_| {
        Box::pin(async { Ok(Some("note".to_string())) })
    })));
    let output = "integration compaction disabled ".repeat(60);
    let report = compact_tool_output(ToolOutputCall {
        compaction_enabled: false,
        ..call(&output, AgentTokenjuiceCompression::Full, None)
    })
    .await;
    assert_eq!(report.stats.rule_id, "llm_summary");

    crate::llm::configure_callback(None);
    let report = compact_tool_output(ToolOutputCall {
        compaction_enabled: false,
        scope: Some("tool-integration-disabled"),
        ..call(&output, AgentTokenjuiceCompression::Full, None)
    })
    .await;
    assert_eq!(report.stats.rule_id, "none/disabled");
    assert_eq!(report.text, output);
}

/// An explicit handle mode wins over the summary stage: an eligible output
/// gets a preview and handle without a model call.
#[tokio::test]
async fn handle_mode_is_honored_before_a_summary() {
    let _guard = crate::llm::callback_test_guard().await;
    enable_llm_summary();
    let mut opts = current_options();
    opts.repl_handle = true;
    opts.ccr_enabled = true;
    opts.ccr_min_tokens = 1;
    crate::llm::configure_callback(Some(std::sync::Arc::new(|_| {
        Box::pin(async { panic!("handle mode must not call the model") })
    })));
    let output = "integration handle mode wins ".repeat(400);
    let report = compact_tool_output_inner(
        ToolOutputCall {
            scope: Some("tool-integration-handle-first"),
            ..call(&output, AgentTokenjuiceCompression::Full, None)
        },
        Some(opts),
    )
    .await;
    crate::llm::configure_callback(None);
    assert_eq!(report.stats.rule_id, "repl", "{}", report.text);
    assert!(report.notice.is_none());
}
