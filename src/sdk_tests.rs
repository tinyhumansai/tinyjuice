use super::*;
use serde_json::json;

#[tokio::test]
async fn sdk_compacts_command_output() {
    let mut lines = vec!["On branch main".to_string()];
    for index in 0..200 {
        lines.push(format!("\tmodified:   src/file_{index}.rs"));
    }
    let response = compress_request(SdkCompressionRequest {
        host: TinyJuiceHost::GenericJson,
        profile: AgentTokenjuiceCompression::Full,
        input: ToolExecutionInput {
            tool_name: "shell".to_string(),
            command: Some("git status".to_string()),
            combined_text: Some(lines.join("\n")),
            exit_code: Some(0),
            ..Default::default()
        },
        options: SdkCompressOptions {
            min_bytes_to_compress: Some(64),
            ..Default::default()
        },
    })
    .await;

    assert!(response.applied, "response: {response:?}");
    assert_eq!(response.host, TinyJuiceHost::GenericJson);
    assert_eq!(response.classification.content_kind, "plain_text");
    assert!(response.stats.reduced_chars < response.stats.raw_chars);
}

#[test]
fn parses_bare_tool_input_payload() {
    let request = request_from_json_value(json!({
        "toolName": "shell",
        "command": "cargo test",
        "combinedText": "ok"
    }))
    .expect("request");
    assert_eq!(request.input.tool_name, "shell");
    assert_eq!(request.input.command.as_deref(), Some("cargo test"));
}

#[test]
fn parses_codex_style_hook_payload() {
    let request = request_from_json_value(json!({
        "hook_event_name": "PostToolUse",
        "tool_name": "shell",
        "tool_input": {
            "command": "cargo test",
            "cwd": "/repo"
        },
        "tool_response": {
            "output": "test output",
            "exitCode": 0
        }
    }))
    .expect("request");
    assert_eq!(request.input.tool_name, "shell");
    assert_eq!(request.input.command.as_deref(), Some("cargo test"));
    assert_eq!(request.input.cwd.as_deref(), Some("/repo"));
    assert_eq!(request.input.combined_text.as_deref(), Some("test output"));
    assert_eq!(request.input.exit_code, Some(0));
}

#[test]
fn renders_hook_templates_as_json() {
    let codex = serde_json::from_str::<Value>(&host_template(TinyJuiceHost::Codex, "tinyjuice"))
        .expect("codex template");
    assert_eq!(
        codex["hooks"]["PostToolUse"][0]["matcher"].as_str(),
        Some("Bash")
    );
    assert!(
        codex["hooks"]["PostToolUse"][0]["hooks"][0]["command"]
            .as_str()
            .expect("command")
            .contains("codex-post-tool-use")
    );
    let codex_status = codex["hooks"]["PostToolUse"][0]["hooks"][0]["statusMessage"]
        .as_str()
        .expect("codex status");
    assert!(
        HOOK_STATUS_MESSAGES.contains(&codex_status),
        "unexpected status: {codex_status}"
    );

    let claude =
        serde_json::from_str::<Value>(&host_template(TinyJuiceHost::ClaudeCode, "tinyjuice"))
            .expect("claude template");
    assert!(
        claude["hooks"]["PostToolUse"][0]["hooks"][0]["command"]
            .as_str()
            .expect("command")
            .contains("claude-code-post-tool-use")
    );
    let claude_status = claude["hooks"]["PostToolUse"][0]["hooks"][0]["statusMessage"]
        .as_str()
        .expect("claude status");
    assert!(
        HOOK_STATUS_MESSAGES.contains(&claude_status),
        "unexpected status: {claude_status}"
    );
}

#[tokio::test]
async fn codex_hook_response_adds_context() {
    let mut lines = vec!["On branch main".to_string()];
    for index in 0..200 {
        lines.push(format!("\tmodified:   src/file_{index}.rs"));
    }
    let payload = json!({
        "hook_event_name": "PostToolUse",
        "tool_name": "Bash",
        "tool_input": {"command": "git status"},
        "tool_response": {"output": lines.join("\n"), "exitCode": 0}
    });
    let response = compress_host_hook_payload(
        TinyJuiceHost::Codex,
        &payload.to_string(),
        SdkCompressOptions {
            min_bytes_to_compress: Some(64),
            ..Default::default()
        },
    )
    .await
    .expect("hook response")
    .expect("compressed");
    assert_eq!(
        response["hookSpecificOutput"]["hookEventName"].as_str(),
        Some("PostToolUse")
    );
    assert!(
        response["hookSpecificOutput"]["additionalContext"]
            .as_str()
            .expect("context")
            .contains("M: src/file_")
    );
}

#[tokio::test]
async fn claude_hook_response_updates_tool_output() {
    let mut lines = vec!["On branch main".to_string()];
    for index in 0..200 {
        lines.push(format!("\tmodified:   src/file_{index}.rs"));
    }
    let payload = json!({
        "hook_event_name": "PostToolUse",
        "tool_name": "Bash",
        "tool_input": {"command": "git status"},
        "tool_response": {"output": lines.join("\n"), "exitCode": 0}
    });
    let response = compress_host_hook_payload(
        TinyJuiceHost::ClaudeCode,
        &payload.to_string(),
        SdkCompressOptions {
            min_bytes_to_compress: Some(64),
            ..Default::default()
        },
    )
    .await
    .expect("hook response")
    .expect("compressed");
    assert!(
        response["hookSpecificOutput"]["updatedToolOutput"]
            .as_str()
            .expect("output")
            .contains("M: src/file_")
    );
}

#[tokio::test]
async fn hook_response_skips_non_matching_event() {
    let payload = json!({
        "hook_event_name": "PreToolUse",
        "tool_name": "Bash",
        "tool_input": {"command": "git status"}
    });
    let response = compress_host_hook_payload(
        TinyJuiceHost::Codex,
        &payload.to_string(),
        SdkCompressOptions::default(),
    )
    .await
    .expect("hook response");
    assert!(response.is_none());
}

#[test]
fn host_ids_are_parseable() {
    for spec in host_install_specs() {
        let serialized = serde_json::to_value(spec.host).expect("host serializes");
        assert_eq!(serialized, Value::String(spec.host.id().to_string()));
        let parsed = TinyJuiceHost::from_str(spec.host.id()).expect("host id");
        assert_eq!(parsed, spec.host);
    }
}
