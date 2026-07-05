use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use sha2::{Digest, Sha256};

const STATIC_PREFIX_CACHE_KEY_PREFIX: &str = "tj-static-prefix-sha256:";

/// Provider-neutral prompt cache time-to-live hint.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum PromptCacheTtl {
    Ephemeral,
    OneHour,
}

/// Provider-neutral location where an adapter may place a cache marker.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum CacheMarkerPlacement {
    SystemPrompt,
    Message,
    ToolSchema,
}

/// Cache marker routing hint. Core TinyJuice never mutates provider payloads.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PromptCacheHint {
    pub provider: String,
    pub message_index: Option<usize>,
    pub ttl: PromptCacheTtl,
    pub placement: CacheMarkerPlacement,
}

/// Static tool schema material used when deriving a stable prefix cache key.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ToolSchemaForCache {
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    pub schema: Value,
}

impl ToolSchemaForCache {
    pub fn new(name: impl Into<String>, schema: Value) -> Self {
        Self {
            name: name.into(),
            kind: None,
            schema,
        }
    }

    pub fn with_kind(mut self, kind: impl Into<String>) -> Self {
        self.kind = Some(kind.into());
        self
    }
}

/// Static-prefix routing metadata. `frozen_prefix_bytes` is an exact copy of
/// the caller-provided prefix bytes so adapters can splice it byte-identically.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StaticPrefixCacheHint {
    pub cache_key: String,
    pub frozen_prefix_bytes: Vec<u8>,
    pub tool_count: usize,
}

/// Derive a stable cache key from static instructions plus sorted tool schemas.
pub fn stable_prefix_cache_key(instructions: &str, tools: &[ToolSchemaForCache]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(instructions.as_bytes());
    hasher.update([0]);
    for tool in sorted_tool_schemas(tools) {
        hasher.update(canonical_tool_schema(&tool).as_bytes());
        hasher.update([0]);
    }
    format!(
        "{STATIC_PREFIX_CACHE_KEY_PREFIX}{}",
        hex::encode(hasher.finalize())
    )
}

/// Build static-prefix metadata without rewriting the frozen prefix bytes.
pub fn static_prefix_cache_hint(
    instructions: &str,
    tools: &[ToolSchemaForCache],
) -> StaticPrefixCacheHint {
    StaticPrefixCacheHint {
        cache_key: stable_prefix_cache_key(instructions, tools),
        frozen_prefix_bytes: instructions.as_bytes().to_vec(),
        tool_count: tools.len(),
    }
}

/// Select Anthropic-style cache marker carriers as hints only: system prompt,
/// then the last three non-system messages.
pub fn anthropic_cache_hints(message_roles: &[impl AsRef<str>]) -> Vec<PromptCacheHint> {
    let mut hints = Vec::new();
    if !message_roles.is_empty() {
        hints.push(PromptCacheHint {
            provider: "anthropic".to_owned(),
            message_index: Some(0),
            ttl: PromptCacheTtl::Ephemeral,
            placement: CacheMarkerPlacement::SystemPrompt,
        });
    }

    let mut carriers: Vec<usize> = message_roles
        .iter()
        .enumerate()
        .filter_map(|(idx, role)| (role.as_ref() != "system").then_some(idx))
        .collect();
    carriers.truncate(carriers.len().saturating_sub(3));
    let last_three_start = carriers.len();

    for idx in message_roles
        .iter()
        .enumerate()
        .filter_map(|(idx, role)| (role.as_ref() != "system").then_some(idx))
        .skip(last_three_start)
    {
        if hints.len() >= 4 {
            break;
        }
        hints.push(PromptCacheHint {
            provider: "anthropic".to_owned(),
            message_index: Some(idx),
            ttl: PromptCacheTtl::Ephemeral,
            placement: CacheMarkerPlacement::Message,
        });
    }

    hints
}

fn sorted_tool_schemas(tools: &[ToolSchemaForCache]) -> Vec<ToolSchemaForCache> {
    let mut sorted = tools.to_vec();
    sorted.sort_by(|a, b| {
        a.name
            .cmp(&b.name)
            .then_with(|| a.kind.cmp(&b.kind))
            .then_with(|| canonical_json(&a.schema).cmp(&canonical_json(&b.schema)))
    });
    sorted
}

fn canonical_tool_schema(tool: &ToolSchemaForCache) -> String {
    let mut map = Map::new();
    map.insert("kind".to_owned(), canonicalize_json(&tool.kind));
    map.insert("name".to_owned(), canonicalize_json(&tool.name));
    map.insert("schema".to_owned(), canonicalize_json(&tool.schema));
    canonical_json(&Value::Object(map))
}

fn canonicalize_json<T: Serialize>(value: &T) -> Value {
    canonicalize_value(serde_json::to_value(value).unwrap_or(Value::Null))
}

fn canonicalize_value(value: Value) -> Value {
    match value {
        Value::Array(values) => Value::Array(values.into_iter().map(canonicalize_value).collect()),
        Value::Object(map) => {
            let mut entries: Vec<_> = map.into_iter().collect();
            entries.sort_by(|a, b| a.0.cmp(&b.0));
            Value::Object(
                entries
                    .into_iter()
                    .map(|(key, value)| (key, canonicalize_value(value)))
                    .collect(),
            )
        }
        other => other,
    }
}

fn canonical_json(value: &Value) -> String {
    serde_json::to_string(&canonicalize_value(value.clone())).unwrap_or_else(|_| "null".to_owned())
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn tool_order_does_not_change_static_prefix_cache_key() {
        let tools = vec![
            ToolSchemaForCache::new("read_file", json!({"input": {"path": "string"}})),
            ToolSchemaForCache::new("search", json!({"input": {"query": "string"}})),
        ];
        let reversed = tools.iter().cloned().rev().collect::<Vec<_>>();

        assert_eq!(
            stable_prefix_cache_key("instructions", &tools),
            stable_prefix_cache_key("instructions", &reversed)
        );
    }

    #[test]
    fn canonical_json_key_order_does_not_change_cache_key() {
        let left = ToolSchemaForCache::new("tool", json!({"b": 2, "a": {"d": 4, "c": 3}}));
        let right = ToolSchemaForCache::new("tool", json!({"a": {"c": 3, "d": 4}, "b": 2}));

        assert_eq!(
            stable_prefix_cache_key("instructions", &[left]),
            stable_prefix_cache_key("instructions", &[right])
        );
    }

    #[test]
    fn frozen_prefix_bytes_are_preserved_exactly() {
        let instructions = "system prompt\nkeep bytes";
        let hint = static_prefix_cache_hint(instructions, &[]);

        assert_eq!(hint.frozen_prefix_bytes, instructions.as_bytes());
        assert!(hint.cache_key.starts_with(STATIC_PREFIX_CACHE_KEY_PREFIX));
    }

    #[test]
    fn anthropic_hints_mark_system_and_last_three_carriers() {
        let hints = anthropic_cache_hints(&["system", "user", "assistant", "tool", "assistant"]);

        assert_eq!(hints.len(), 4);
        assert_eq!(hints[0].placement, CacheMarkerPlacement::SystemPrompt);
        assert_eq!(hints[1].message_index, Some(2));
        assert_eq!(hints[3].message_index, Some(4));
    }
}
