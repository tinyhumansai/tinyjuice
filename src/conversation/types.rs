use serde::{Deserialize, Serialize};
use serde_json::Value;

/// A regular chat message in a provider-neutral conversation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChatMessage {
    pub role: String,
    pub content: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Internal compaction summaries are not considered "real" user/assistant
    /// anchors when selecting protected conversation turns.
    #[serde(default)]
    pub is_compaction_summary: bool,
    /// UI-hidden assistant/system messages are skipped by visible-assistant
    /// anchoring.
    #[serde(default)]
    pub hidden: bool,
    /// Redacted metadata for host adapters. Core helpers never inspect raw host
    /// runtime types.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Value>,
}

impl ChatMessage {
    pub fn system(content: impl Into<String>) -> Self {
        Self::new("system", content)
    }

    pub fn user(content: impl Into<String>) -> Self {
        Self::new("user", content)
    }

    pub fn assistant(content: impl Into<String>) -> Self {
        Self::new("assistant", content)
    }

    pub fn tool(content: impl Into<String>) -> Self {
        Self::new("tool", content)
    }

    pub fn compaction_summary(role: impl Into<String>, content: impl Into<String>) -> Self {
        let mut msg = Self::new(role, content);
        msg.is_compaction_summary = true;
        msg
    }

    fn new(role: impl Into<String>, content: impl Into<String>) -> Self {
        Self {
            role: role.into(),
            content: content.into(),
            id: None,
            is_compaction_summary: false,
            hidden: false,
            metadata: None,
        }
    }
}

/// A tool call requested by an assistant turn.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ToolCall {
    pub id: String,
    pub name: String,
    /// Serialized JSON arguments when available. Non-JSON strings are preserved
    /// by shrink/redaction helpers.
    pub arguments: String,
}

impl ToolCall {
    pub fn new(
        id: impl Into<String>,
        name: impl Into<String>,
        arguments: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            arguments: arguments.into(),
        }
    }
}

/// A result returned for a tool call.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ToolResultMessage {
    pub tool_call_id: String,
    pub content: String,
}

impl ToolResultMessage {
    pub fn new(tool_call_id: impl Into<String>, content: impl Into<String>) -> Self {
        Self {
            tool_call_id: tool_call_id.into(),
            content: content.into(),
        }
    }
}

/// Provider-neutral conversation row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "camelCase")]
pub enum ConversationMessage {
    Chat(ChatMessage),
    AssistantToolCalls {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        text: Option<String>,
        tool_calls: Vec<ToolCall>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        metadata: Option<Value>,
    },
    ToolResults(Vec<ToolResultMessage>),
}

impl ConversationMessage {
    pub fn chat(role: impl Into<String>, content: impl Into<String>) -> Self {
        Self::Chat(ChatMessage::new(role, content))
    }

    pub fn system(content: impl Into<String>) -> Self {
        Self::Chat(ChatMessage::system(content))
    }

    pub fn user(content: impl Into<String>) -> Self {
        Self::Chat(ChatMessage::user(content))
    }

    pub fn assistant(content: impl Into<String>) -> Self {
        Self::Chat(ChatMessage::assistant(content))
    }

    pub fn assistant_tool_calls(tool_calls: Vec<ToolCall>) -> Self {
        Self::AssistantToolCalls {
            text: None,
            tool_calls,
            metadata: None,
        }
    }

    pub fn tool_results(results: Vec<ToolResultMessage>) -> Self {
        Self::ToolResults(results)
    }
}
