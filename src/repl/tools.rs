//! `tinytools::Tool` adapters for the REPL ops. Enabled by the `tinytools` feature.
//!
//! Every tool takes a `handle` (the CCR token from a compressed output's footer)
//! plus its own arguments. All are read-only, concurrency-safe and size-capped.

use std::sync::Arc;

use async_trait::async_trait;
use serde_json::{Value, json};
use tinytools::{PermissionLevel, Tool, ToolResult};

use super::{ReplError, ReplLimits, ReplOp, run_op};
use crate::cache::store::CcrStore;

struct ReplTool {
    name: &'static str,
    op: &'static str,
    description: &'static str,
    extra: Value,
    required: &'static [&'static str],
    store: Arc<dyn CcrStore>,
    limits: ReplLimits,
}

#[async_trait]
impl Tool for ReplTool {
    fn name(&self) -> &str {
        self.name
    }

    fn description(&self) -> &str {
        self.description
    }

    fn parameters_schema(&self) -> Value {
        let mut props =
            json!({ "handle": { "type": "string", "description": "handle from footer" } });
        if let (Some(p), Some(e)) = (props.as_object_mut(), self.extra.as_object()) {
            p.extend(e.clone());
        }
        let mut required = vec!["handle"];
        required.extend_from_slice(self.required);
        json!({ "type": "object", "properties": props, "required": required })
    }

    async fn execute(&self, args: Value) -> anyhow::Result<ToolResult> {
        let Some(handle) = args.get("handle").and_then(Value::as_str) else {
            return Ok(ToolResult::error("missing required argument: handle"));
        };
        let mut tagged = args.clone();
        let Some(obj) = tagged.as_object_mut() else {
            return Ok(ToolResult::error("arguments must be an object"));
        };
        obj.insert("op".into(), Value::String(self.op.into()));
        let op: ReplOp = match serde_json::from_value(tagged) {
            Ok(op) => op,
            Err(e) => return Ok(ToolResult::error(format!("invalid arguments: {e}"))),
        };
        match run_op(self.store.as_ref(), handle, &op, &self.limits) {
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
    let t = |name, op, description, extra: Value, required| -> Box<dyn Tool> {
        Box::new(ReplTool {
            name,
            op,
            description,
            extra,
            required,
            store: store.clone(),
            limits,
        })
    };
    vec![
        t(
            "juice_find",
            "find",
            "Query a stored output by mode: text (substring), grep/regex (regex; regex returns capture groups), rank (BM25), sed (`-n 10,20p`, `s/a/b/`), awk (`-F, '$2>5{print $1}'`), jq (JSON filter).",
            json!({
                "query": { "type": "string" },
                "mode": { "type": "string", "enum": ["text", "grep", "regex", "rank", "sed", "awk", "jq"] },
                "ignore_case": { "type": "boolean" },
                "context": { "type": "integer" },
                "top_k": { "type": "integer" },
                "scope": { "type": "string", "description": "python slice, e.g. [:-100]" },
                "unit": { "type": "string", "enum": ["lines", "chars"] }
            }),
            &["query"],
        ),
        t(
            "juice_extract",
            "extract",
            "List links or headings from an HTML or Markdown output.",
            json!({
                "what": { "type": "string", "enum": ["links", "headings"] },
                "scope": { "type": "string" },
                "unit": { "type": "string", "enum": ["lines", "chars"] }
            }),
            &["what"],
        ),
        t(
            "juice_summarize",
            "summarize",
            "Model-free summary: size, outline or JSON shape, then head/tail or, with a hint, the parts most relevant to it.",
            json!({
                "hint": { "type": "string", "description": "what you need from it" },
                "max_chars": { "type": "integer" },
                "scope": { "type": "string" },
                "unit": { "type": "string", "enum": ["lines", "chars"] }
            }),
            &[],
        ),
    ]
}
