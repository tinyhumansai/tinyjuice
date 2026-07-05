//! Host-facing, non-sensitive context usage breakdowns.
//!
//! These structs let an adapter explain why compression triggered without
//! handing TinyJuice raw prompt, tool, memory, or conversation text.

/// Provider-neutral context bucket categories.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ContextBucketKind {
    SystemPrompt,
    ToolDefinitions,
    RulesContextFiles,
    Skills,
    McpTools,
    Subagents,
    Memory,
    Conversation,
    Other,
}

impl ContextBucketKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::SystemPrompt => "system_prompt",
            Self::ToolDefinitions => "tool_definitions",
            Self::RulesContextFiles => "rules_context_files",
            Self::Skills => "skills",
            Self::McpTools => "mcp_tools",
            Self::Subagents => "subagents",
            Self::Memory => "memory",
            Self::Conversation => "conversation",
            Self::Other => "other",
        }
    }
}

/// One non-sensitive context usage bucket.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContextBucket {
    pub kind: ContextBucketKind,
    pub estimated_tokens: usize,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub measured_tokens: Option<usize>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub byte_count: Option<usize>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub item_count: Option<usize>,
    /// Stable prefix cost that compression should not be blamed for, such as
    /// the system prompt and static tool definitions.
    pub static_prefix: bool,
    /// Whether this bucket is a plausible target for host conversation/content
    /// compaction. Static provider costs should usually leave this false.
    pub compression_candidate: bool,
}

impl ContextBucket {
    pub fn estimated(kind: ContextBucketKind, estimated_tokens: usize) -> Self {
        Self {
            kind,
            estimated_tokens,
            measured_tokens: None,
            byte_count: None,
            item_count: None,
            static_prefix: false,
            compression_candidate: false,
        }
    }

    pub fn system_prompt(estimated_tokens: usize) -> Self {
        Self::estimated(ContextBucketKind::SystemPrompt, estimated_tokens).as_static_prefix()
    }

    pub fn tool_definitions(estimated_tokens: usize) -> Self {
        Self::estimated(ContextBucketKind::ToolDefinitions, estimated_tokens).as_static_prefix()
    }

    pub fn conversation(estimated_tokens: usize) -> Self {
        Self::estimated(ContextBucketKind::Conversation, estimated_tokens)
            .as_compression_candidate()
    }

    pub fn memory(estimated_tokens: usize) -> Self {
        Self::estimated(ContextBucketKind::Memory, estimated_tokens).as_compression_candidate()
    }

    pub fn with_measured_tokens(mut self, measured_tokens: usize) -> Self {
        self.measured_tokens = Some(measured_tokens);
        self
    }

    pub fn with_byte_count(mut self, byte_count: usize) -> Self {
        self.byte_count = Some(byte_count);
        self
    }

    pub fn with_item_count(mut self, item_count: usize) -> Self {
        self.item_count = Some(item_count);
        self
    }

    pub fn as_static_prefix(mut self) -> Self {
        self.static_prefix = true;
        self.compression_candidate = false;
        self
    }

    pub fn as_compression_candidate(mut self) -> Self {
        self.compression_candidate = true;
        self
    }

    pub fn effective_tokens(&self) -> usize {
        self.measured_tokens.unwrap_or(self.estimated_tokens)
    }
}

/// Non-sensitive context usage report for host UIs and logs.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContextBreakdown {
    pub categories: Vec<ContextBucket>,
    pub estimated_total_tokens: usize,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub measured_prompt_tokens: Option<usize>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub context_max_tokens: Option<usize>,
}

impl ContextBreakdown {
    pub fn new(categories: Vec<ContextBucket>) -> Self {
        let estimated_total_tokens = categories
            .iter()
            .map(|bucket| bucket.estimated_tokens)
            .sum();
        Self {
            categories,
            estimated_total_tokens,
            measured_prompt_tokens: None,
            context_max_tokens: None,
        }
    }

    pub fn empty() -> Self {
        Self::new(Vec::new())
    }

    pub fn add_bucket(&mut self, bucket: ContextBucket) {
        self.estimated_total_tokens += bucket.estimated_tokens;
        self.categories.push(bucket);
    }

    pub fn with_measured_prompt_tokens(mut self, measured_prompt_tokens: usize) -> Self {
        self.measured_prompt_tokens = Some(measured_prompt_tokens);
        self
    }

    pub fn with_context_max_tokens(mut self, context_max_tokens: usize) -> Self {
        self.context_max_tokens = Some(context_max_tokens);
        self
    }

    /// Total prompt tokens to display. Provider-measured usage wins over local
    /// estimates when the host can provide it.
    pub fn effective_prompt_tokens(&self) -> usize {
        self.measured_prompt_tokens.unwrap_or_else(|| {
            self.categories
                .iter()
                .map(ContextBucket::effective_tokens)
                .sum()
        })
    }

    /// Tokens that belong to stable prompt prefix material rather than
    /// compressible conversation/history.
    pub fn static_prefix_tokens(&self) -> usize {
        self.categories
            .iter()
            .filter(|bucket| bucket.static_prefix)
            .map(ContextBucket::effective_tokens)
            .sum()
    }

    /// Tokens in buckets the host marked as plausible compaction targets.
    pub fn compression_candidate_tokens(&self) -> usize {
        self.categories
            .iter()
            .filter(|bucket| bucket.compression_candidate)
            .map(ContextBucket::effective_tokens)
            .sum()
    }

    pub fn usage_ratio(&self) -> Option<f64> {
        let max = self.context_max_tokens?;
        if max == 0 {
            return None;
        }
        Some(self.effective_prompt_tokens() as f64 / max as f64)
    }

    pub fn bucket(&self, kind: ContextBucketKind) -> Option<&ContextBucket> {
        self.categories.iter().find(|bucket| bucket.kind == kind)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn breakdown_separates_static_prefix_from_compressible_context() {
        let breakdown = ContextBreakdown::new(vec![
            ContextBucket::system_prompt(100),
            ContextBucket::tool_definitions(200),
            ContextBucket::conversation(700),
            ContextBucket::memory(50),
        ])
        .with_context_max_tokens(2_000);

        assert_eq!(breakdown.estimated_total_tokens, 1_050);
        assert_eq!(breakdown.static_prefix_tokens(), 300);
        assert_eq!(breakdown.compression_candidate_tokens(), 750);
        assert_eq!(breakdown.effective_prompt_tokens(), 1_050);
        assert_eq!(breakdown.usage_ratio(), Some(0.525));
    }

    #[test]
    fn measured_prompt_tokens_override_rough_estimate() {
        let breakdown = ContextBreakdown::new(vec![
            ContextBucket::system_prompt(100),
            ContextBucket::conversation(400),
        ])
        .with_measured_prompt_tokens(800);

        assert_eq!(breakdown.estimated_total_tokens, 500);
        assert_eq!(breakdown.effective_prompt_tokens(), 800);
    }

    #[test]
    fn bucket_measured_tokens_override_bucket_estimate() {
        let breakdown = ContextBreakdown::new(vec![
            ContextBucket::system_prompt(100),
            ContextBucket::conversation(400).with_measured_tokens(250),
        ]);

        assert_eq!(breakdown.effective_prompt_tokens(), 350);
        assert_eq!(breakdown.compression_candidate_tokens(), 250);
    }

    #[test]
    fn serialized_breakdown_contains_no_raw_prompt_fields() {
        let breakdown = ContextBreakdown::new(vec![
            ContextBucket::estimated(ContextBucketKind::Skills, 42)
                .with_item_count(3)
                .with_byte_count(1_024),
        ]);
        let json = serde_json::to_string(&breakdown).unwrap();

        assert!(json.contains("skills"));
        assert!(!json.contains("content"));
        assert!(!json.contains("promptText"));
        assert!(!json.contains("raw"));
    }
}
