//! Tool declarations shared by library adapters and module-only hosts.

use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

/// A tool's frozen public declaration and its corresponding REPL operation.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReplToolDeclaration {
    /// Existing product tool name.
    pub name: String,
    /// Operation tag inserted into the REPL request.
    pub op: String,
    /// Description presented to the model.
    pub description: String,
    /// JSON Schema for the tool's arguments.
    pub parameters: Value,
}

/// The stock REPL declarations, preserving their existing tool schemas.
#[must_use]
pub fn repl_tool_declarations() -> Vec<ReplToolDeclaration> {
    let t = |name: &str, op: &str, description: &str, extra: Value, required: &[&str]| {
        let mut props = json!({ "handle": { "type": "string", "description": "the handle named in the output's footer" } });
        if let (Some(p), Some(e)) = (props.as_object_mut(), extra.as_object()) {
            p.extend(e.clone());
        }
        let mut all_required = vec!["handle"];
        all_required.extend_from_slice(required);
        ReplToolDeclaration {
            name: name.into(),
            op: op.into(),
            description: description.into(),
            parameters: json!({ "type": "object", "properties": props, "required": all_required }),
        }
    };
    vec![
        t(
            "juice_find",
            "find",
            "Read or search part of a stored tool output by its handle, without loading all of it. mode `sed` reads exact lines (query `-n 120,200p`); `grep`/`regex` search (with `context` lines; regex returns capture groups); `text` substring; `rank` BM25; `awk` (`-F, '$2>5{print $1}'`); `jq` for JSON. Prefer this to juice_retrieve when you need only part of an output.",
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
            "HTML or Markdown outputs only: list their links or headings.",
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
            "Model-free overview of a stored output: size, outline or JSON shape, then head and tail; with `hint`, the parts most relevant to it. Use it to find where to read with juice_find.",
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

/// The argument's name on the wire.
pub const SUMMARY_FOCUS_ARG: &str = "summary_focus";

/// The schema for the optional `summary_focus` argument.
pub fn summary_focus_property() -> Value {
    json!({
        "type": "string",
        "description": "What you need from this result. A large result is summarized around \
                        this; the full output stays retrievable."
    })
}

/// Description of the existing retrieval tool.
pub const RETRIEVE_TOOL_DESCRIPTION: &str = "Retrieve the full, original text of a tool result that was compacted to \
         save context. When a tool output shows a marker like \
         `retrieve_tool_output(\"a1b2c3d4e5f6\")`, call this with that hash to get \
         the complete original back. Use it only when you actually need the dropped \
         detail — the compacted view is usually enough.";

/// Existing retrieval tool argument schema.
#[must_use]
pub fn retrieve_tool_parameters() -> Value {
    json!({
        "type": "object",
        "properties": {
            "hash": {
                "type": "string",
                "description": "The hash from a retrieve_tool_output(\"…\") marker."
            }
        },
        "required": ["hash"]
    })
}
