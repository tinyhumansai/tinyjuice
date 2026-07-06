use crate::reduce::reduce_execution_with_rules;
use crate::types::{
    ClassificationResult, CompactResult, CompiledRule, ReduceOptions, ReductionStats,
    ToolExecutionInput,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ReduceJsonRequest {
    Envelope(ReduceJsonEnvelope),
    Direct(ToolExecutionInput),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReduceJsonEnvelope {
    pub input: ToolExecutionInput,
    #[serde(default)]
    pub options: ReduceOptions,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReduceJsonResponse {
    pub inline_text: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub preview_text: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub facts: Option<std::collections::HashMap<String, usize>>,
    pub stats: ReductionStats,
    pub classification: ClassificationResult,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ReduceJsonMetadata>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub trace: Option<ReduceJsonTrace>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReduceJsonMetadata {
    pub no_omit_requested: bool,
    pub store_requested: bool,
    pub record_stats_requested: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReduceJsonTrace {
    pub tool_name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub argv0: Option<String>,
    pub raw_mode: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_inline_chars: Option<usize>,
    pub family: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub matched_reducer: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReduceJsonError {
    pub code: String,
    pub message: String,
}

impl ReduceJsonError {
    fn new(code: &'static str, message: impl Into<String>) -> Self {
        Self {
            code: code.to_owned(),
            message: message.into(),
        }
    }
}

pub type ReduceJsonResult<T> = Result<T, ReduceJsonError>;

pub fn reduce_json_str(json: &str, rules: &[CompiledRule]) -> ReduceJsonResult<ReduceJsonResponse> {
    if json.contains('\0') {
        return Err(ReduceJsonError::new(
            "nul_byte",
            "payload contains a NUL byte",
        ));
    }
    let request = serde_json::from_str::<ReduceJsonRequest>(json)
        .map_err(|error| ReduceJsonError::new("invalid_json", error.to_string()))?;
    reduce_json_request(request, rules)
}

pub fn reduce_json_request(
    request: ReduceJsonRequest,
    rules: &[CompiledRule],
) -> ReduceJsonResult<ReduceJsonResponse> {
    let (mut input, options) = match request {
        ReduceJsonRequest::Envelope(envelope) => (envelope.input, envelope.options),
        ReduceJsonRequest::Direct(input) => (input, ReduceOptions::default()),
    };

    if input.cwd.is_none() {
        input.cwd = options.cwd.clone();
    }

    let trace_requested = options.trace.unwrap_or(false);
    let metadata = metadata_for_options(&options);
    let trace_input = trace_requested.then(|| input.clone());
    let result = reduce_execution_with_rules(input, rules, &options);
    Ok(response_from_result(
        result,
        metadata,
        trace_input,
        &options,
    ))
}

fn response_from_result(
    result: CompactResult,
    metadata: Option<ReduceJsonMetadata>,
    trace_input: Option<ToolExecutionInput>,
    options: &ReduceOptions,
) -> ReduceJsonResponse {
    let trace = trace_input.map(|input| {
        let argv0 = trace_argv0(&input).map(str::to_owned);
        ReduceJsonTrace {
            tool_name: input.tool_name,
            argv0,
            raw_mode: options.raw.unwrap_or(false),
            max_inline_chars: options.max_inline_chars,
            family: result.classification.family.clone(),
            matched_reducer: result.classification.matched_reducer.clone(),
        }
    });

    ReduceJsonResponse {
        inline_text: result.inline_text,
        preview_text: result.preview_text,
        facts: result.facts,
        stats: result.stats,
        classification: result.classification,
        metadata,
        trace,
    }
}

fn metadata_for_options(options: &ReduceOptions) -> Option<ReduceJsonMetadata> {
    let metadata = ReduceJsonMetadata {
        no_omit_requested: options.no_omit.unwrap_or(false),
        store_requested: options.store.unwrap_or(false),
        record_stats_requested: options.record_stats.unwrap_or(false),
    };
    (metadata.no_omit_requested || metadata.store_requested || metadata.record_stats_requested)
        .then_some(metadata)
}

fn trace_argv0(input: &ToolExecutionInput) -> Option<&str> {
    input
        .argv
        .as_ref()
        .and_then(|argv| argv.first().map(String::as_str))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rules::load_builtin_rules;

    #[test]
    fn reduce_json_accepts_direct_payload() {
        let rules = load_builtin_rules();
        let response = reduce_json_str(
            r#"{
                "toolName": "bash",
                "argv": ["git", "status"],
                "stdout": "On branch main\n\nChanges not staged for commit:\n\tmodified:   src/lib.rs\n"
            }"#,
            &rules,
        )
        .expect("direct payload");

        assert_eq!(response.inline_text, "Changes not staged:\nM: src/lib.rs");
        assert_eq!(
            response.classification.matched_reducer.as_deref(),
            Some("git/status")
        );
        assert!(response.metadata.is_none());
        assert!(response.trace.is_none());
    }

    #[test]
    fn reduce_json_accepts_envelope_payload_with_options() {
        let rules = load_builtin_rules();
        let response = reduce_json_str(
            r#"{
                "input": {
                    "toolName": "bash",
                    "argv": ["some_tool"],
                    "stdout": "alpha\nbeta\ngamma\ndelta\nepsilon\nzeta"
                },
                "options": {
                    "maxInlineChars": 24,
                    "trace": true,
                    "recordStats": true
                }
            }"#,
            &rules,
        )
        .expect("envelope payload");

        assert!(response.inline_text.len() <= 24, "{response:#?}");
        assert_eq!(
            response.metadata,
            Some(ReduceJsonMetadata {
                no_omit_requested: false,
                store_requested: false,
                record_stats_requested: true,
            })
        );
        let trace = response.trace.expect("trace");
        assert_eq!(trace.tool_name, "bash");
        assert_eq!(trace.argv0.as_deref(), Some("some_tool"));
        assert_eq!(trace.max_inline_chars, Some(24));
        assert_eq!(trace.matched_reducer.as_deref(), Some("generic/fallback"));
    }

    #[test]
    fn reduce_json_raw_mode_returns_raw_text() {
        let rules = load_builtin_rules();
        let response = reduce_json_str(
            r#"{
                "input": {
                    "toolName": "bash",
                    "argv": ["git", "status"],
                    "stdout": "On branch main\n"
                },
                "options": { "raw": true }
            }"#,
            &rules,
        )
        .expect("raw payload");

        assert_eq!(response.inline_text, "On branch main\n");
    }

    #[test]
    fn reduce_json_invalid_classifier_falls_back_to_matching() {
        let rules = load_builtin_rules();
        let response = reduce_json_str(
            r#"{
                "input": {
                    "toolName": "bash",
                    "argv": ["git", "status"],
                    "stdout": "On branch main\n"
                },
                "options": { "classifier": "missing/rule" }
            }"#,
            &rules,
        )
        .expect("invalid classifier fallback");

        assert_eq!(
            response.classification.matched_reducer.as_deref(),
            Some("git/status")
        );
    }

    #[test]
    fn reduce_json_rejects_malformed_payload_without_raw_echo() {
        let rules = load_builtin_rules();
        let error = reduce_json_str(r#"{"toolName":"bash","stdout":"secret""#, &rules)
            .expect_err("invalid json");

        assert_eq!(error.code, "invalid_json");
        assert!(!error.message.contains("secret"));
    }

    #[test]
    fn reduce_json_rejects_nul_bytes() {
        let rules = load_builtin_rules();
        let error = reduce_json_str("{\"toolName\":\"bash\"}\0", &rules).expect_err("nul byte");

        assert_eq!(error.code, "nul_byte");
    }
}
