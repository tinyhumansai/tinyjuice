//! The scenario's payload tool, with its result routed through TinyJuice per arm.

use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Instant;

use async_trait::async_trait;
use tinyjuice::tool_integration::{ToolOutputCall, compact_tool_output};
use tinyjuice::types::AgentTokenjuiceCompression;
use tinytools::{Tool, ToolResult};

use crate::Arm;

pub struct PayloadTool {
    pub name: &'static str,
    pub description: &'static str,
    pub payload: String,
    pub arm: Arm,
    pub question: &'static str,
    /// Unique per run. The summary cache and failure breaker are keyed by it, so
    /// reusing one would serve a later run's summary from an earlier one.
    pub run_id: String,
    /// Milliseconds spent inside TinyJuice for this run, for the report.
    pub compaction_ms: Arc<AtomicU64>,
}

#[async_trait]
impl Tool for PayloadTool {
    fn name(&self) -> &str {
        self.name
    }

    fn description(&self) -> &str {
        self.description
    }

    fn parameters_schema(&self) -> serde_json::Value {
        serde_json::json!({ "type": "object", "properties": {} })
    }

    async fn execute(&self, _args: serde_json::Value) -> anyhow::Result<ToolResult> {
        if self.arm == Arm::Raw {
            return Ok(ToolResult::success(self.payload.clone()));
        }
        let started = Instant::now();
        let report = compact_tool_output(ToolOutputCall {
            tool_name: self.name,
            arguments: None,
            output: &self.payload,
            exit_code: None,
            profile: AgentTokenjuiceCompression::Full,
            compaction_enabled: true,
            focus: Some(self.question),
            context_token: Some("agent-eval"),
            scope: Some(&self.run_id),
        })
        .await;
        self.compaction_ms
            .fetch_add(started.elapsed().as_millis() as u64, Ordering::Relaxed);
        let text = match report.notice {
            Some(notice) => format!("{notice}\n{}", report.text),
            None => report.text,
        };
        Ok(ToolResult::success(text))
    }
}
