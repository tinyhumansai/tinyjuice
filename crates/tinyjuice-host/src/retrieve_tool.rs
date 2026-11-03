//! `tinytools::Tool` adapter for retrieving a stored result through the host.

use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use async_trait::async_trait;
use serde_json::Value;
use tinyjuice_bus::tools::{RETRIEVE_TOOL_DESCRIPTION, retrieve_tool_parameters};
use tinyjuice_bus::wire::{RangeUnit, RetrieveRange};
use tinytools::{PermissionLevel, Tool, ToolResult};

/// Host lookup callback. `Ok(None)` means the token has been evicted or expired.
pub type RetrieveFn = Arc<
    dyn Fn(
            String,
            Option<RetrieveRange>,
        ) -> Pin<Box<dyn Future<Output = Result<Option<String>, String>> + Send>>
        + Send
        + Sync,
>;

/// The canonical `juice_retrieve` tool over a host-supplied lookup.
pub struct RetrieveToolOutputTool {
    retrieve: RetrieveFn,
}

impl RetrieveToolOutputTool {
    /// Build the tool over a host-owned token lookup.
    pub fn new(retrieve: RetrieveFn) -> Self {
        Self { retrieve }
    }
}

#[async_trait]
impl Tool for RetrieveToolOutputTool {
    fn name(&self) -> &str {
        tinyjuice_bus::tools::RETRIEVE
    }

    fn description(&self) -> &str {
        RETRIEVE_TOOL_DESCRIPTION
    }

    fn parameters_schema(&self) -> Value {
        retrieve_tool_parameters()
    }

    fn permission_level(&self) -> PermissionLevel {
        PermissionLevel::ReadOnly
    }

    fn is_concurrency_safe(&self, _args: &Value) -> bool {
        true
    }

    fn external_effect(&self) -> bool {
        false
    }

    async fn execute(&self, args: Value) -> anyhow::Result<ToolResult> {
        let token = args
            .get("token")
            .or_else(|| args.get("hash"))
            .and_then(Value::as_str)
            .map(str::trim)
            .filter(|value| !value.is_empty());
        let Some(token) = token else {
            return Ok(ToolResult::error(format!(
                "{}: missing required 'token' argument",
                tinyjuice_bus::tools::RETRIEVE
            )));
        };
        let range = match args
            .get("range")
            .filter(|value| value.is_object())
            .map(parse_range)
        {
            Some(Ok(range)) => Some(range),
            Some(Err(())) => {
                return Ok(ToolResult::error(format!(
                    "{}: range endpoint is too large",
                    tinyjuice_bus::tools::RETRIEVE
                )));
            }
            None => None,
        };

        match (self.retrieve)(token.to_owned(), range).await {
            Ok(Some(text)) => {
                log::debug!("[tinyjuice-host::retrieve] retrieved bytes={}", text.len());
                Ok(ToolResult::success(text))
            }
            Ok(None) => Ok(ToolResult::error(miss_message(token))),
            Err(error) => Ok(ToolResult::error(error)),
        }
    }
}

fn parse_range(range: &Value) -> Result<RetrieveRange, ()> {
    let start = match range.get("start").and_then(Value::as_u64) {
        Some(value) => usize_from_range_endpoint(value, usize::MAX as u64)?,
        None => 0,
    };
    let end = match range.get("end").and_then(Value::as_u64) {
        Some(value) => usize_from_range_endpoint(value, usize::MAX as u64)?,
        None => usize::MAX,
    };
    let unit = match range.get("unit").and_then(Value::as_str) {
        Some("bytes") => RangeUnit::Bytes,
        _ => RangeUnit::Lines,
    };
    Ok(RetrieveRange { start, end, unit })
}

fn usize_from_range_endpoint(value: u64, maximum: u64) -> Result<usize, ()> {
    if value > maximum {
        return Err(());
    }
    usize::try_from(value).map_err(|_| ())
}

fn miss_message(token: &str) -> String {
    format!(
        "{tool}: the full original for token '{token}' is no longer cached \
         (evicted, or from an earlier session). Do NOT re-run the same tool call to \
         regenerate it — that will produce the same oversized result and be compacted \
         again. Proceed using the compacted summary already shown above; only if a \
         specific missing detail is essential, retry with narrower arguments (a tighter \
         query, filter, or smaller limit) so the result is small enough to keep in full.",
        tool = tinyjuice_bus::tools::RETRIEVE
    )
}

#[cfg(test)]
#[path = "retrieve_tool_tests.rs"]
mod tests;
