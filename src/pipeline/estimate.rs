use crate::types::ContentKind;

/// Redacted estimate of how much non-essential bulk a payload appears to carry.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BloatEstimate {
    /// Heuristic score in the range 0..=100.
    pub score: u8,
    /// Dominant signal behind the score. Never contains raw content.
    pub reason: BloatReason,
}

/// Categorical bloat reason.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BloatReason {
    JsonRows,
    DiffContext,
    LogRepetition,
    SearchFanout,
    HtmlMarkup,
    CodeBody,
    TextRepetition,
    LowSignal,
}

impl BloatReason {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::JsonRows => "json_rows",
            Self::DiffContext => "diff_context",
            Self::LogRepetition => "log_repetition",
            Self::SearchFanout => "search_fanout",
            Self::HtmlMarkup => "html_markup",
            Self::CodeBody => "code_body",
            Self::TextRepetition => "text_repetition",
            Self::LowSignal => "low_signal",
        }
    }
}

/// Estimate bloat without retaining or reporting raw content.
pub fn estimate_bloat(content: &str, kind: ContentKind) -> BloatEstimate {
    match kind {
        ContentKind::Json => estimate_json_bloat(content),
        ContentKind::Diff => estimate_diff_bloat(content),
        ContentKind::Log => estimate_log_bloat(content),
        ContentKind::Search => estimate_search_bloat(content),
        ContentKind::Html => estimate_html_bloat(content),
        ContentKind::Code => estimate_code_bloat(content),
        ContentKind::PlainText => estimate_text_bloat(content),
    }
}

fn score_ratio(numerator: usize, denominator: usize) -> u8 {
    if denominator == 0 {
        return 0;
    }
    ((numerator.min(denominator) * 100) / denominator) as u8
}

fn estimate_json_bloat(content: &str) -> BloatEstimate {
    let Ok(value) = serde_json::from_str::<serde_json::Value>(content.trim()) else {
        return low();
    };
    let serde_json::Value::Array(rows) = value else {
        return low();
    };
    if rows.len() < 2 {
        return low();
    }
    let object_rows = rows.iter().filter(|row| row.is_object()).count();
    let score = (40 + score_ratio(object_rows.saturating_sub(2), rows.len()) / 2).min(95);
    BloatEstimate {
        score,
        reason: BloatReason::JsonRows,
    }
}

fn estimate_diff_bloat(content: &str) -> BloatEstimate {
    let mut context = 0usize;
    let mut changed = 0usize;
    for line in content.lines().take(2_000) {
        if line.starts_with("@@")
            || line.starts_with("diff --git")
            || line.starts_with("+++")
            || line.starts_with("---")
        {
            continue;
        }
        if line.starts_with('+') || line.starts_with('-') {
            changed += 1;
        } else if line.starts_with(' ') || !line.trim().is_empty() {
            context += 1;
        }
    }
    let total = context + changed;
    if total == 0 {
        return low();
    }
    BloatEstimate {
        score: score_ratio(context, total),
        reason: BloatReason::DiffContext,
    }
}

fn estimate_log_bloat(content: &str) -> BloatEstimate {
    let mut total = 0usize;
    let mut repeated = 0usize;
    let mut previous = "";
    for line in content.lines().take(2_000) {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        total += 1;
        if trimmed == previous {
            repeated += 1;
        }
        previous = trimmed;
    }
    if total == 0 {
        return low();
    }
    BloatEstimate {
        score: score_ratio(repeated + total.saturating_sub(80), total),
        reason: BloatReason::LogRepetition,
    }
}

fn estimate_search_bloat(content: &str) -> BloatEstimate {
    let mut total = 0usize;
    let mut paths = std::collections::HashSet::new();
    for line in content.lines().take(2_000) {
        let Some((path, _rest)) = line.split_once(':') else {
            continue;
        };
        if path.trim().is_empty() {
            continue;
        }
        total += 1;
        paths.insert(path);
    }
    if total == 0 {
        return low();
    }
    let fanout = paths.len();
    BloatEstimate {
        score: (score_ratio(total.saturating_sub(20), total) / 2
            + score_ratio(fanout.saturating_sub(3), fanout.max(1)) / 2)
            .min(95),
        reason: BloatReason::SearchFanout,
    }
}

fn estimate_html_bloat(content: &str) -> BloatEstimate {
    let tag_bytes = content
        .bytes()
        .filter(|byte| matches!(byte, b'<' | b'>'))
        .count();
    BloatEstimate {
        score: score_ratio(tag_bytes * 10, content.len().max(1)).min(95),
        reason: BloatReason::HtmlMarkup,
    }
}

fn estimate_code_bloat(content: &str) -> BloatEstimate {
    let mut total = 0usize;
    let mut body_like = 0usize;
    for line in content.lines().take(2_000) {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        total += 1;
        if trimmed == "{" || trimmed == "}" || trimmed.ends_with(';') || trimmed.contains(" = ") {
            body_like += 1;
        }
    }
    if total == 0 {
        return low();
    }
    BloatEstimate {
        score: score_ratio(body_like, total),
        reason: BloatReason::CodeBody,
    }
}

fn estimate_text_bloat(content: &str) -> BloatEstimate {
    let mut total = 0usize;
    let mut repeated = 0usize;
    let mut seen = std::collections::HashSet::new();
    for line in content.lines().take(2_000) {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        total += 1;
        if !seen.insert(trimmed) {
            repeated += 1;
        }
    }
    if total == 0 || repeated == 0 {
        return low();
    }
    BloatEstimate {
        score: score_ratio(repeated, total),
        reason: BloatReason::TextRepetition,
    }
}

fn low() -> BloatEstimate {
    BloatEstimate {
        score: 0,
        reason: BloatReason::LowSignal,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn json_rows_estimate_is_redacted() {
        let content = r#"[{"secret":"a"},{"secret":"b"},{"secret":"c"}]"#;
        let estimate = estimate_bloat(content, ContentKind::Json);
        assert_eq!(estimate.reason, BloatReason::JsonRows);
        assert!(estimate.score > 0);
        assert!(!estimate.reason.as_str().contains("secret"));
    }

    #[test]
    fn diff_context_estimate_detects_context_heavy_payload() {
        let mut diff = String::from("diff --git a/a b/a\n@@ -1,8 +1,8 @@\n");
        for i in 0..80 {
            diff.push_str(&format!(" context line {i}\n"));
        }
        diff.push_str("-old\n+new\n");
        let estimate = estimate_bloat(&diff, ContentKind::Diff);
        assert_eq!(estimate.reason, BloatReason::DiffContext);
        assert!(estimate.score > 80);
    }

    #[test]
    fn plain_unique_text_is_low_signal() {
        let estimate = estimate_bloat("alpha\nbeta\ngamma", ContentKind::PlainText);
        assert_eq!(estimate.reason, BloatReason::LowSignal);
        assert_eq!(estimate.score, 0);
    }
}
