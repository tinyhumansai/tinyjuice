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

#[test]
fn anthropic_hints_do_not_label_a_user_message_as_system() {
    let hints = anthropic_cache_hints(&["user", "assistant", "tool", "assistant"]);

    assert!(
        hints
            .iter()
            .all(|hint| hint.placement != CacheMarkerPlacement::SystemPrompt)
    );
    assert_eq!(hints.len(), 3);
    assert_eq!(hints[0].message_index, Some(1));
}
