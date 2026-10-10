//! `tinytools::Tool` adapters for the REPL ops. Enabled by the `tinytools` feature.
//!
//! Every tool takes a `handle` (the CCR token from a compressed output's footer)
//! plus its own arguments. All are read-only, concurrency-safe and size-capped.

use std::sync::Arc;

use async_trait::async_trait;
use serde_json::Value;
use tinyjuice_bus::tools::{ReplToolDeclaration, repl_tool_declarations};
use tinytools::{PermissionLevel, Tool, ToolResult};

use super::{ModelSummary, ReplError, ReplLimits, ReplOp, run_op_with_model};
use crate::cache::store::CcrStore;

struct ReplTool {
    declaration: ReplToolDeclaration,
    store: Arc<dyn CcrStore>,
    limits: ReplLimits,
    /// Set for `juice_summarize` when the host lends it a model.
    model: Option<ModelSummary>,
}

#[async_trait]
impl Tool for ReplTool {
    fn name(&self) -> &str {
        &self.declaration.name
    }

    fn description(&self) -> &str {
        &self.declaration.description
    }

    fn parameters_schema(&self) -> Value {
        self.declaration.parameters.clone()
    }

    async fn execute(&self, args: Value) -> anyhow::Result<ToolResult> {
        let Some(handle) = args.get("handle").and_then(Value::as_str) else {
            return Ok(ToolResult::error("missing required argument: handle"));
        };
        let mut tagged = args.clone();
        let Some(obj) = tagged.as_object_mut() else {
            return Ok(ToolResult::error("arguments must be an object"));
        };
        obj.insert("op".into(), Value::String(self.declaration.op.clone()));
        let op: ReplOp = match serde_json::from_value(tagged) {
            Ok(op) => op,
            Err(e) => return Ok(ToolResult::error(format!("invalid arguments: {e}"))),
        };
        match run_op_with_model(
            self.store.as_ref(),
            handle,
            &op,
            &self.limits,
            self.model.as_ref(),
        )
        .await
        {
            Ok(out) => Ok(ToolResult::json(serde_json::to_value(out)?)),
            Err(ReplError::HandleNotFound) => {
                Ok(ToolResult::failed(ReplError::HandleNotFound.to_string()))
            }
            Err(e) => Ok(ToolResult::error(e.to_string())),
        }
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

    fn max_result_size_chars(&self) -> Option<usize> {
        Some(self.limits.max_output_chars * 2)
    }
}

/// The REPL toolset over `store`. Hosts register these; TinyJuice does no dispatch.
pub fn repl_tools(store: Arc<dyn CcrStore>, limits: ReplLimits) -> Vec<Box<dyn Tool>> {
    repl_tools_with_model(store, limits, None)
}

/// [`repl_tools`], with `juice_summarize` written by the host's model when
/// `model` is set (see [`super::run_op_with_model`]).
pub fn repl_tools_with_model(
    store: Arc<dyn CcrStore>,
    limits: ReplLimits,
    model: Option<ModelSummary>,
) -> Vec<Box<dyn Tool>> {
    repl_tool_declarations()
        .into_iter()
        .map(|declaration| {
            let model = (declaration.op == "summarize")
                .then(|| model.clone())
                .flatten();
            Box::new(ReplTool {
                declaration,
                store: store.clone(),
                limits,
                model,
            }) as Box<dyn Tool>
        })
        .collect()
}
