use crate::reduce::reduce_execution_with_rules;
use crate::savings::{self, SavingsRecord};
use crate::types::{
    ClassificationResult, CompactResult, CompiledRule, CompressorKind, ContentKind, ReduceOptions,
    ReductionStats, ToolExecutionInput,
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
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ccr: Option<ReduceJsonCcrRef>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReduceJsonCcrRef {
    pub token: String,
    pub original_chars: usize,
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
    if options.record_stats.unwrap_or(false) {
        record_stats(&result);
    }

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

    let metadata = metadata.map(|mut metadata| {
        if let Some(token) = result.ccr_token.clone() {
            metadata.ccr = Some(ReduceJsonCcrRef {
                token,
                original_chars: result.stats.raw_chars,
            });
        }
        metadata
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
        ccr: None,
    };
    (metadata.no_omit_requested || metadata.store_requested || metadata.record_stats_requested)
        .then_some(metadata)
}

fn record_stats(result: &CompactResult) {
    let mut record = SavingsRecord::estimated_compaction(
        ContentKind::Log,
        CompressorKind::Generic,
        estimated_tokens_from_chars(result.stats.raw_chars),
        estimated_tokens_from_chars(result.stats.reduced_chars),
    )
    .with_bytes(
        result.stats.raw_chars as u64,
        result.stats.reduced_chars as u64,
    )
    .with_lossy(result.stats.reduced_chars < result.stats.raw_chars)
    .with_ccr_token_present(result.ccr_token.is_some());

    if let Some(rule_id) = result.classification.matched_reducer.as_deref() {
        record = record.with_rule_id(rule_id);
    }

    savings::record_event(record);
}

fn estimated_tokens_from_chars(chars: usize) -> u64 {
    if chars == 0 {
        0
    } else {
        ((chars as f64) / crate::tokens::CHARS_PER_TOKEN)
            .ceil()
            .max(1.0) as u64
    }
}

fn trace_argv0(input: &ToolExecutionInput) -> Option<&str> {
    input
        .argv
        .as_ref()
        .and_then(|argv| argv.first().map(String::as_str))
}

#[cfg(test)]
#[path = "protocol_tests.rs"]
mod tests;
