//! Host-neutral SDK and plugin adapter surface.
//!
//! This module is the stable boundary for Rust harnesses and for non-Rust hosts
//! that call the `tinyjuice reduce-json` protocol. It keeps host integration
//! metadata, request/response JSON, and conservative profile policy close to
//! the core router without importing any host runtime dependencies.

use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use std::collections::HashMap;
use std::str::FromStr;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::compress::route;
use crate::tokens::estimate_tokens;
use crate::types::{
    AgentTokenjuiceCompression, CompressInput, CompressOptions, CompressedOutput, ContentHint,
    ContentKind, ToolExecutionInput,
};

const HOOK_STATUS_MESSAGES: &[&str] = &[
    "TinyJuice is juicing OpenHuman context",
    "TinyJuice is squeezing tool noise",
    "TinyJuice is distilling Bash signal",
    "TinyJuice is packing tiny context",
    "TinyJuice is trimming OpenHuman traces",
    "TinyJuice is polishing CCR memory",
    "TinyJuice is crushing token sludge",
    "TinyJuice is bottling compact context",
];

/// Supported host/plugin integration targets.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "kebab-case")]
pub enum TinyJuiceHost {
    /// Generic stdin/stdout JSON protocol for custom harnesses.
    #[default]
    GenericJson,
    /// OpenHuman or another Rust host calling the crate directly.
    #[serde(rename = "openhuman")]
    OpenHuman,
    /// OpenAI Codex-style hook/plugin integration.
    Codex,
    /// Claude Code-style hook/plugin integration.
    ClaudeCode,
    /// Rust agent harnesses such as TinyAgents/RustAgents.
    RustHarness,
}

impl TinyJuiceHost {
    pub fn id(self) -> &'static str {
        match self {
            Self::GenericJson => "generic-json",
            Self::OpenHuman => "openhuman",
            Self::Codex => "codex",
            Self::ClaudeCode => "claude-code",
            Self::RustHarness => "rust-harness",
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            Self::GenericJson => "Generic JSON harness",
            Self::OpenHuman => "OpenHuman",
            Self::Codex => "Codex",
            Self::ClaudeCode => "Claude Code",
            Self::RustHarness => "Rust harness",
        }
    }
}

impl FromStr for TinyJuiceHost {
    type Err = String;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "generic-json" | "generic" | "json" => Ok(Self::GenericJson),
            "openhuman" | "open-human" => Ok(Self::OpenHuman),
            "codex" => Ok(Self::Codex),
            "claude-code" | "claude" => Ok(Self::ClaudeCode),
            "rust-harness" | "rust" | "harness" => Ok(Self::RustHarness),
            other => Err(format!("unknown TinyJuice host '{other}'")),
        }
    }
}

/// Serializable compression options accepted by SDK requests.
///
/// Every field is optional so plugin/harness callers can send only the knobs
/// they own. Missing values inherit [`CompressOptions::default`].
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SdkCompressOptions {
    #[serde(default)]
    pub router_enabled: Option<bool>,
    #[serde(default)]
    pub ccr_enabled: Option<bool>,
    #[serde(default)]
    pub search_enabled: Option<bool>,
    #[serde(default)]
    pub code_enabled: Option<bool>,
    #[serde(default)]
    pub html_enabled: Option<bool>,
    #[serde(default)]
    pub ml_text_enabled: Option<bool>,
    #[serde(default)]
    pub min_bytes_to_compress: Option<usize>,
    #[serde(default)]
    pub min_bytes_to_compress_log: Option<usize>,
    #[serde(default)]
    pub ccr_min_tokens: Option<usize>,
    #[serde(default)]
    pub lossy_without_ccr: Option<bool>,
    #[serde(default)]
    pub max_inline_chars: Option<usize>,
    #[serde(default)]
    pub chars_per_token: Option<f32>,
}

impl SdkCompressOptions {
    pub fn into_compress_options(self) -> CompressOptions {
        let mut opts = CompressOptions::default();
        if let Some(value) = self.router_enabled {
            opts.router_enabled = value;
        }
        if let Some(value) = self.ccr_enabled {
            opts.ccr_enabled = value;
        }
        if let Some(value) = self.search_enabled {
            opts.search_enabled = value;
        }
        if let Some(value) = self.code_enabled {
            opts.code_enabled = value;
        }
        if let Some(value) = self.html_enabled {
            opts.html_enabled = value;
        }
        if let Some(value) = self.ml_text_enabled {
            opts.ml_text_enabled = value;
        }
        if let Some(value) = self.min_bytes_to_compress {
            opts.min_bytes_to_compress = value;
        }
        if let Some(value) = self.min_bytes_to_compress_log {
            opts.min_bytes_to_compress_log = value;
        }
        if let Some(value) = self.ccr_min_tokens {
            opts.ccr_min_tokens = value;
        }
        if let Some(value) = self.lossy_without_ccr {
            opts.lossy_without_ccr = value;
        }
        if let Some(value) = self.max_inline_chars {
            opts.max_inline_chars = Some(value);
        }
        if let Some(value) = self.chars_per_token {
            opts.chars_per_token = value;
        }
        opts
    }
}

/// Stable SDK request shape for tool-output compression.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SdkCompressionRequest {
    #[serde(default)]
    pub host: TinyJuiceHost,
    #[serde(default)]
    pub profile: AgentTokenjuiceCompression,
    #[serde(default)]
    pub input: ToolExecutionInput,
    #[serde(default)]
    pub options: SdkCompressOptions,
}

impl SdkCompressionRequest {
    pub fn new(input: ToolExecutionInput) -> Self {
        Self {
            input,
            ..Default::default()
        }
    }
}

/// SDK response stats. Character counts mirror the cross-language JSON
/// protocol, while byte/token counts are included for Rust hosts and dashboards.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SdkCompressionStats {
    pub raw_chars: usize,
    pub reduced_chars: usize,
    pub ratio: f64,
    pub original_bytes: usize,
    pub compacted_bytes: usize,
    pub original_tokens: usize,
    pub compacted_tokens: usize,
}

/// Classification/strategy metadata without raw content.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SdkCompressionClassification {
    pub family: String,
    pub content_kind: String,
    pub compressor: String,
    pub confidence: f64,
    #[serde(default)]
    pub matched_reducer: Option<String>,
}

/// Stable SDK response shape.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SdkCompressionResponse {
    pub inline_text: String,
    #[serde(default)]
    pub preview_text: Option<String>,
    pub stats: SdkCompressionStats,
    pub classification: SdkCompressionClassification,
    pub host: TinyJuiceHost,
    pub profile: AgentTokenjuiceCompression,
    pub applied: bool,
    pub lossy: bool,
    #[serde(default)]
    pub ccr_token: Option<String>,
}

/// Host integration metadata suitable for docs, installers, or plugin UIs.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HostInstallSpec {
    pub host: TinyJuiceHost,
    pub label: String,
    pub install_command: String,
    pub hook_file: String,
    pub hook_event: String,
    pub description: String,
    pub notes: Vec<String>,
}

/// A configured TinyJuice SDK client for direct Rust use.
#[derive(Debug, Clone)]
pub struct TinyJuiceSdk {
    host: TinyJuiceHost,
    profile: AgentTokenjuiceCompression,
    options: CompressOptions,
}

impl TinyJuiceSdk {
    pub fn new(host: TinyJuiceHost) -> Self {
        Self {
            host,
            profile: AgentTokenjuiceCompression::Full,
            options: CompressOptions::default(),
        }
    }

    pub fn with_profile(mut self, profile: AgentTokenjuiceCompression) -> Self {
        self.profile = profile;
        self
    }

    pub fn with_options(mut self, options: CompressOptions) -> Self {
        self.options = options;
        self
    }

    pub async fn compress_tool_output(&self, input: ToolExecutionInput) -> SdkCompressionResponse {
        compress_request(SdkCompressionRequest {
            host: self.host,
            profile: self.profile,
            input,
            options: SdkCompressOptions::from(self.options.clone()),
        })
        .await
    }
}

/// Compress one raw hook payload and return a host-native hook response.
///
/// Returns `None` when the payload is not a matching post-tool hook, has no
/// compressible output, or TinyJuice declines to rewrite the content.
pub async fn compress_host_hook_payload(
    host: TinyJuiceHost,
    raw_json: &str,
    options: SdkCompressOptions,
) -> Result<Option<Value>, serde_json::Error> {
    let value = serde_json::from_str(raw_json)?;
    if !is_supported_post_tool_payload(host, &value) {
        return Ok(None);
    }
    let mut request = request_from_json_value(value)?;
    request.host = host;
    request.options = options;
    let response = compress_request(request).await;
    Ok(host_hook_response(&response))
}

/// Build the JSON object a hook command should print for the host.
pub fn host_hook_response(response: &SdkCompressionResponse) -> Option<Value> {
    if !response.applied {
        return None;
    }
    let inline_text = decorate_hook_inline_text(response);
    match response.host {
        TinyJuiceHost::Codex => Some(serde_json::json!({
            "hookSpecificOutput": {
                "hookEventName": "PostToolUse",
                "additionalContext": inline_text
            }
        })),
        TinyJuiceHost::ClaudeCode => Some(serde_json::json!({
            "hookSpecificOutput": {
                "hookEventName": "PostToolUse",
                "updatedToolOutput": inline_text
            }
        })),
        TinyJuiceHost::GenericJson | TinyJuiceHost::OpenHuman | TinyJuiceHost::RustHarness => None,
    }
}

impl From<CompressOptions> for SdkCompressOptions {
    fn from(value: CompressOptions) -> Self {
        Self {
            router_enabled: Some(value.router_enabled),
            ccr_enabled: Some(value.ccr_enabled),
            search_enabled: Some(value.search_enabled),
            code_enabled: Some(value.code_enabled),
            html_enabled: Some(value.html_enabled),
            ml_text_enabled: Some(value.ml_text_enabled),
            min_bytes_to_compress: Some(value.min_bytes_to_compress),
            min_bytes_to_compress_log: Some(value.min_bytes_to_compress_log),
            ccr_min_tokens: Some(value.ccr_min_tokens),
            lossy_without_ccr: Some(value.lossy_without_ccr),
            max_inline_chars: value.max_inline_chars,
            chars_per_token: Some(value.chars_per_token),
        }
    }
}

/// Compress one SDK request.
pub async fn compress_request(request: SdkCompressionRequest) -> SdkCompressionResponse {
    let original_text = tool_output_text(&request.input);
    let mut opts = request.options.into_compress_options();

    match request.profile {
        AgentTokenjuiceCompression::Off => {
            return response_from_output(
                request.host,
                request.profile,
                &original_text,
                CompressedOutput::passthrough(original_text.clone(), ContentKind::PlainText),
            );
        }
        AgentTokenjuiceCompression::Light => {
            opts.ccr_enabled = false;
            opts.lossy_without_ccr = false;
            opts.ml_text_enabled = false;
        }
        AgentTokenjuiceCompression::Auto | AgentTokenjuiceCompression::Full => {}
    }

    let hint = hint_from_tool_input(&request.input);
    let route_input = CompressInput {
        content: &original_text,
        kind: ContentKind::PlainText,
        hint: &hint,
        exit_code: request.input.exit_code,
        command: request.input.command.clone(),
        argv: request.input.argv.clone(),
        original_bytes: original_text.len(),
    };
    let output = route(route_input, &opts).await;
    response_from_output(request.host, request.profile, &original_text, output)
}

/// Return install metadata for all first-class host surfaces.
pub fn host_install_specs() -> Vec<HostInstallSpec> {
    [
        TinyJuiceHost::OpenHuman,
        TinyJuiceHost::RustHarness,
        TinyJuiceHost::Codex,
        TinyJuiceHost::ClaudeCode,
        TinyJuiceHost::GenericJson,
    ]
    .into_iter()
    .map(host_install_spec)
    .collect()
}

/// Return install metadata for one host surface.
pub fn host_install_spec(host: TinyJuiceHost) -> HostInstallSpec {
    match host {
        TinyJuiceHost::OpenHuman => HostInstallSpec {
            host,
            label: host.label().to_string(),
            install_command: r#"cargo add tinyjuice"#.to_string(),
            hook_file: "host Rust tool-output adapter".to_string(),
            hook_event: "post-tool-output".to_string(),
            description: "Call the crate directly after tool output is captured and scrubbed."
                .to_string(),
            notes: vec![
                "Map host config into CompressOptions.".to_string(),
                "Expose a tinyjuice_retrieve tool before enabling lossy CCR views.".to_string(),
            ],
        },
        TinyJuiceHost::RustHarness => HostInstallSpec {
            host,
            label: host.label().to_string(),
            install_command: r#"cargo add tinyjuice"#.to_string(),
            hook_file: "harness post-tool hook".to_string(),
            hook_event: "post-tool-output".to_string(),
            description: "Use TinyJuiceSdk or compress_request inside the Rust harness loop."
                .to_string(),
            notes: vec![
                "Pass command, argv, exit code, and cwd when available.".to_string(),
                "Do not log raw tool output from adapter diagnostics.".to_string(),
            ],
        },
        TinyJuiceHost::Codex => HostInstallSpec {
            host,
            label: host.label().to_string(),
            install_command: r#"tinyjuice install codex"#.to_string(),
            hook_file: "~/.codex/hooks.json".to_string(),
            hook_event: "PostToolUse".to_string(),
            description: "Install a Codex PostToolUse hook for Bash output.".to_string(),
            notes: vec![
                "The installer merges with existing hooks and writes a backup.".to_string(),
                "Use `tinyjuice update codex` to refresh the hook after changing binaries."
                    .to_string(),
                "Use `tinyjuice uninstall codex` to remove only TinyJuice hooks.".to_string(),
                "The hook statusMessage rotates through small TinyJuice/OpenHuman working messages."
                    .to_string(),
                "The hook expects a tinyjuice binary on PATH.".to_string(),
            ],
        },
        TinyJuiceHost::ClaudeCode => HostInstallSpec {
            host,
            label: host.label().to_string(),
            install_command: r#"tinyjuice install claude-code"#.to_string(),
            hook_file: "~/.claude/settings.json".to_string(),
            hook_event: "PostToolUse".to_string(),
            description: "Install a Claude Code PostToolUse hook for Bash output.".to_string(),
            notes: vec![
                "The installer merges with existing settings and writes a backup.".to_string(),
                "Use `tinyjuice update claude-code` to refresh the hook after changing binaries."
                    .to_string(),
                "Use `tinyjuice uninstall claude-code` to remove only TinyJuice hooks.".to_string(),
                "The hook statusMessage rotates through small TinyJuice/OpenHuman working messages."
                    .to_string(),
                "Claude Code supports updatedToolOutput, so TinyJuice can replace noisy output."
                    .to_string(),
            ],
        },
        TinyJuiceHost::GenericJson => HostInstallSpec {
            host,
            label: host.label().to_string(),
            install_command: r#"tinyjuice reduce-json payload.json"#.to_string(),
            hook_file: "stdin/stdout JSON".to_string(),
            hook_event: "post-tool-output".to_string(),
            description: "Pipe a stable JSON request into TinyJuice from any harness.".to_string(),
            notes: vec![
                "Use combinedText for already-merged output, or stdout/stderr fields otherwise."
                    .to_string(),
                "The response never includes raw content except inlineText.".to_string(),
            ],
        },
    }
}

/// Render a starter hook/template for a host.
pub fn host_template(host: TinyJuiceHost, binary: &str) -> String {
    let status_message = hook_status_message();
    match host {
        TinyJuiceHost::Codex => serde_json::to_string_pretty(&serde_json::json!({
            "hooks": {
                "PostToolUse": [
                    {
                        "matcher": "Bash",
                        "hooks": [
                            {
                                "type": "command",
                                "command": format!("{binary} codex-post-tool-use"),
                                "statusMessage": status_message,
                                "timeout": 10
                            }
                        ]
                    }
                ]
            }
        }))
        .expect("template JSON serializes"),
        TinyJuiceHost::ClaudeCode => serde_json::to_string_pretty(&serde_json::json!({
            "hooks": {
                "PostToolUse": [
                    {
                        "matcher": "Bash",
                        "hooks": [
                            {
                                "type": "command",
                                "command": format!("{binary} claude-code-post-tool-use"),
                                "statusMessage": status_message,
                                "timeout": 10
                            }
                        ]
                    }
                ]
            }
        }))
        .expect("template JSON serializes"),
        TinyJuiceHost::OpenHuman | TinyJuiceHost::RustHarness => {
            format!(
                r#"use tinyjuice::{{AgentTokenjuiceCompression, TinyJuiceHost, TinyJuiceSdk, ToolExecutionInput}};

let sdk = TinyJuiceSdk::new(TinyJuiceHost::{variant})
    .with_profile(AgentTokenjuiceCompression::Full);
let response = sdk.compress_tool_output(ToolExecutionInput {{
    tool_name: "shell".to_string(),
    command: Some("cargo test".to_string()),
    combined_text: Some(tool_output),
    exit_code: Some(exit_code),
    ..Default::default()
}}).await;
"#,
                variant = match host {
                    TinyJuiceHost::OpenHuman => "OpenHuman",
                    TinyJuiceHost::RustHarness => "RustHarness",
                    _ => unreachable!(),
                }
            )
        }
        TinyJuiceHost::GenericJson => serde_json::to_string_pretty(&serde_json::json!({
            "host": "generic-json",
            "profile": "full",
            "input": {
                "toolName": "shell",
                "command": "cargo test",
                "argv": ["cargo", "test"],
                "combinedText": "tool output goes here",
                "exitCode": 0,
                "metadata": {
                    "source": "custom-harness"
                }
            },
            "options": {
                "minBytesToCompress": 512,
                "maxInlineChars": 1200,
                "ccrEnabled": true
            }
        }))
        .expect("template JSON serializes"),
    }
}

fn hook_status_message() -> &'static str {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_nanos())
        .unwrap_or_default();
    let index = (nanos % HOOK_STATUS_MESSAGES.len() as u128) as usize;
    HOOK_STATUS_MESSAGES[index]
}

fn response_from_output(
    host: TinyJuiceHost,
    profile: AgentTokenjuiceCompression,
    original_text: &str,
    output: CompressedOutput,
) -> SdkCompressionResponse {
    let raw_chars = original_text.chars().count();
    let reduced_chars = output.text.chars().count();
    let ratio = if raw_chars == 0 {
        1.0
    } else {
        reduced_chars as f64 / raw_chars as f64
    };
    let original_tokens = estimate_tokens(original_text) as usize;
    let compacted_tokens = estimate_tokens(&output.text) as usize;
    let family = output.content_kind.as_str().to_string();
    let compressor = output.compressor.as_str().to_string();
    SdkCompressionResponse {
        inline_text: output.text,
        preview_text: None,
        stats: SdkCompressionStats {
            raw_chars,
            reduced_chars,
            ratio,
            original_bytes: output.original_bytes,
            compacted_bytes: output.compacted_bytes,
            original_tokens,
            compacted_tokens,
        },
        classification: SdkCompressionClassification {
            family,
            content_kind: output.content_kind.as_str().to_string(),
            compressor: compressor.clone(),
            confidence: if output.applied { 1.0 } else { 0.0 },
            matched_reducer: output.applied.then_some(compressor),
        },
        host,
        profile,
        applied: output.applied,
        lossy: output.lossy,
        ccr_token: output.ccr_token,
    }
}

fn decorate_hook_inline_text(response: &SdkCompressionResponse) -> String {
    let Some(token) = &response.ccr_token else {
        return response.inline_text.clone();
    };
    format!(
        "{}\n\n[CLI recovery: run `tinyjuice retrieve {}` to print the full original.]",
        response.inline_text, token
    )
}

fn tool_output_text(input: &ToolExecutionInput) -> String {
    if let Some(text) = &input.combined_text {
        return text.clone();
    }
    match (&input.stdout, &input.stderr) {
        (Some(stdout), Some(stderr)) if !stderr.is_empty() => format!("{stdout}\n{stderr}"),
        (Some(stdout), _) => stdout.clone(),
        (_, Some(stderr)) => stderr.clone(),
        _ => String::new(),
    }
}

fn is_supported_post_tool_payload(host: TinyJuiceHost, value: &Value) -> bool {
    let event = string_field(value, &["hook_event_name", "hookEventName"]);
    if event.as_deref() != Some("PostToolUse") {
        return false;
    }
    let Some(tool_name) = string_field(value, &["tool_name", "toolName"]) else {
        return false;
    };
    match host {
        TinyJuiceHost::Codex | TinyJuiceHost::ClaudeCode => {
            matches!(tool_name.as_str(), "Bash" | "bash" | "shell" | "exec")
        }
        TinyJuiceHost::GenericJson | TinyJuiceHost::OpenHuman | TinyJuiceHost::RustHarness => true,
    }
}

fn hint_from_tool_input(input: &ToolExecutionInput) -> ContentHint {
    ContentHint {
        source_tool: (!input.tool_name.is_empty()).then(|| input.tool_name.clone()),
        extension: extension_from_args(input.args.as_ref()),
        query: query_from_args(input.args.as_ref()),
        ..Default::default()
    }
}

fn extension_from_args(args: Option<&HashMap<String, Value>>) -> Option<String> {
    let path = args.and_then(|map| {
        ["path", "file_path", "file", "filename"]
            .iter()
            .find_map(|key| map.get(*key).and_then(Value::as_str))
    })?;
    std::path::Path::new(path)
        .extension()
        .and_then(|value| value.to_str())
        .map(str::to_ascii_lowercase)
}

fn query_from_args(args: Option<&HashMap<String, Value>>) -> Option<String> {
    args.and_then(|map| {
        ["query", "pattern", "search", "q", "regex"]
            .iter()
            .find_map(|key| map.get(*key).and_then(Value::as_str))
            .map(str::to_string)
    })
}

/// Convert either the full SDK request shape or a bare ToolExecutionInput JSON
/// object into a request.
pub fn request_from_json_value(value: Value) -> Result<SdkCompressionRequest, serde_json::Error> {
    if value.get("input").is_some() {
        serde_json::from_value(value)
    } else if looks_like_host_hook_payload(&value) {
        Ok(request_from_host_hook_payload(value))
    } else {
        let input = serde_json::from_value(value)?;
        Ok(SdkCompressionRequest::new(input))
    }
}

fn looks_like_host_hook_payload(value: &Value) -> bool {
    value.get("tool_name").is_some()
        || value.get("tool_input").is_some()
        || value.get("tool_response").is_some()
        || value.get("toolInput").is_some()
        || value.get("toolResponse").is_some()
}

fn request_from_host_hook_payload(value: Value) -> SdkCompressionRequest {
    let tool_name = string_field(&value, &["tool_name", "toolName"]).unwrap_or_default();
    let tool_input = value
        .get("tool_input")
        .or_else(|| value.get("toolInput"))
        .cloned()
        .unwrap_or(Value::Null);
    let tool_response = value
        .get("tool_response")
        .or_else(|| value.get("toolResponse"))
        .cloned()
        .unwrap_or(Value::Null);

    let command = string_field(&tool_input, &["command", "cmd"]);
    let argv = array_string_field(&tool_input, &["argv"]);
    let args = object_as_hash_map(tool_input.as_object());
    let combined_text = response_text(&tool_response);
    let exit_code = i32_field(&value, &["exitCode", "exit_code"]).or_else(|| {
        i32_field(
            &tool_response,
            &["exitCode", "exit_code", "status", "statusCode"],
        )
    });
    let cwd = string_field(&value, &["cwd"])
        .or_else(|| string_field(&tool_input, &["cwd"]))
        .or_else(|| string_field(&tool_response, &["cwd"]));

    SdkCompressionRequest::new(ToolExecutionInput {
        tool_name,
        command,
        argv,
        args,
        cwd,
        combined_text,
        exit_code,
        ..Default::default()
    })
}

fn string_field(value: &Value, keys: &[&str]) -> Option<String> {
    keys.iter()
        .find_map(|key| value.get(*key).and_then(Value::as_str))
        .map(str::to_string)
}

fn i32_field(value: &Value, keys: &[&str]) -> Option<i32> {
    keys.iter()
        .find_map(|key| value.get(*key).and_then(Value::as_i64))
        .and_then(|value| i32::try_from(value).ok())
}

fn array_string_field(value: &Value, keys: &[&str]) -> Option<Vec<String>> {
    keys.iter()
        .find_map(|key| value.get(*key).and_then(Value::as_array))
        .map(|items| {
            items
                .iter()
                .filter_map(Value::as_str)
                .map(str::to_string)
                .collect::<Vec<_>>()
        })
        .filter(|items| !items.is_empty())
}

fn object_as_hash_map(object: Option<&Map<String, Value>>) -> Option<HashMap<String, Value>> {
    let map = object?;
    let converted = map
        .iter()
        .map(|(key, value)| (key.clone(), value.clone()))
        .collect::<HashMap<_, _>>();
    (!converted.is_empty()).then_some(converted)
}

fn response_text(value: &Value) -> Option<String> {
    if let Some(text) = value.as_str() {
        return Some(text.to_string());
    }
    string_field(
        value,
        &[
            "combinedText",
            "combined_text",
            "output",
            "text",
            "content",
            "stdout",
            "stderr",
        ],
    )
    .or_else(|| {
        if value.is_null() {
            None
        } else {
            Some(value.to_string())
        }
    })
}

/// Build a JSON object with command/argv fields suitable for hosts that still
/// use [`crate::compact_tool_output_with_policy`].
pub fn arguments_value(input: &ToolExecutionInput) -> Option<Value> {
    let mut map = Map::new();
    if let Some(args) = &input.args {
        for (key, value) in args {
            map.insert(key.clone(), value.clone());
        }
    }
    if let Some(command) = &input.command {
        map.insert("command".to_string(), Value::String(command.clone()));
    }
    if let Some(argv) = &input.argv {
        map.insert(
            "argv".to_string(),
            Value::Array(argv.iter().cloned().map(Value::String).collect()),
        );
    }
    (!map.is_empty()).then_some(Value::Object(map))
}

#[cfg(test)]
mod tests {
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
        let codex =
            serde_json::from_str::<Value>(&host_template(TinyJuiceHost::Codex, "tinyjuice"))
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
}
