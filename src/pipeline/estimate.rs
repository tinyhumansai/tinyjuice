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
    JsonSaturation,
    DiffContext,
    DiffNoise,
    LogRepetition,
    LogTemplates,
    SearchFanout,
    SearchClustering,
    HtmlMarkup,
    CodeBody,
    TextRepetition,
    LowSignal,
}

impl BloatReason {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::JsonRows => "json_rows",
            Self::JsonSaturation => "json_saturation",
            Self::DiffContext => "diff_context",
            Self::DiffNoise => "diff_noise",
            Self::LogRepetition => "log_repetition",
            Self::LogTemplates => "log_templates",
            Self::SearchFanout => "search_fanout",
            Self::SearchClustering => "search_clustering",
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
    if object_rows == 0 {
        return low();
    }

    let mut columns = std::collections::BTreeSet::new();
    let mut row_shapes = std::collections::HashSet::new();
    let mut row_hashes = std::collections::HashSet::new();
    let mut scalar_values_by_key: std::collections::HashMap<
        String,
        std::collections::HashSet<u64>,
    > = std::collections::HashMap::new();
    let mut present_cells = 0usize;

    for row in rows.iter().filter_map(|row| row.as_object()) {
        let mut shape = Vec::with_capacity(row.len());
        for (key, value) in row {
            columns.insert(key.clone());
            shape.push(key.as_str());
            present_cells += 1;
            if value.is_string() || value.is_boolean() || value.is_number() || value.is_null() {
                scalar_values_by_key
                    .entry(key.clone())
                    .or_default()
                    .insert(stable_value_hash(value));
            }
        }
        shape.sort_unstable();
        row_shapes.insert(shape.join("\0"));
        row_hashes.insert(stable_object_hash(row));
    }

    let total_cells = object_rows.saturating_mul(columns.len()).max(1);
    let sparse_cells = total_cells.saturating_sub(present_cells);
    let constant_fields = scalar_values_by_key
        .values()
        .filter(|values| values.len() == 1)
        .count();
    let duplicate_rows = object_rows.saturating_sub(row_hashes.len());
    let saturation_score = (score_ratio(duplicate_rows, object_rows) / 3)
        + (score_ratio(constant_fields, columns.len().max(1)) / 3)
        + (score_ratio(sparse_cells, total_cells) / 3);
    let row_pressure = score_ratio(object_rows.saturating_sub(2), rows.len()) / 2;
    let shape_pressure = score_ratio(row_shapes.len().saturating_sub(1), object_rows.max(1)) / 4;
    let score = (40 + row_pressure + saturation_score + shape_pressure).min(95);
    let reason = if saturation_score >= 20 || duplicate_rows >= object_rows / 4 {
        BloatReason::JsonSaturation
    } else {
        BloatReason::JsonRows
    };
    BloatEstimate { score, reason }
}

fn estimate_diff_bloat(content: &str) -> BloatEstimate {
    let mut context_bytes = 0usize;
    let mut changed_bytes = 0usize;
    let mut noisy_bytes = 0usize;
    let mut whitespace_only_bytes = 0usize;
    let mut current_file_is_noisy = false;
    let mut hunk_lines: Vec<&str> = Vec::new();

    let flush_hunk = |hunk_lines: &mut Vec<&str>, whitespace_only_bytes: &mut usize| {
        if hunk_is_whitespace_only(hunk_lines) {
            *whitespace_only_bytes += hunk_lines
                .iter()
                .filter(|line| is_changed_diff_line(line))
                .map(|line| line.len() + 1)
                .sum::<usize>();
        }
        hunk_lines.clear();
    };

    for line in content.lines().take(4_000) {
        if line.starts_with("diff --git") {
            flush_hunk(&mut hunk_lines, &mut whitespace_only_bytes);
            current_file_is_noisy = diff_path_is_noisy(line);
            continue;
        }
        if line.starts_with("@@") {
            flush_hunk(&mut hunk_lines, &mut whitespace_only_bytes);
            continue;
        }
        if line.starts_with("+++") || line.starts_with("---") {
            continue;
        }

        if is_changed_diff_line(line) {
            let bytes = line.len() + 1;
            changed_bytes += bytes;
            hunk_lines.push(line);
            if current_file_is_noisy {
                noisy_bytes += bytes;
            }
        } else if line.starts_with(' ') {
            let bytes = line.len() + 1;
            context_bytes += bytes;
            hunk_lines.push(line);
            if current_file_is_noisy {
                noisy_bytes += bytes;
            }
        } else if !line.trim().is_empty() && current_file_is_noisy {
            noisy_bytes += line.len() + 1;
        }
    }
    flush_hunk(&mut hunk_lines, &mut whitespace_only_bytes);

    let total = context_bytes + changed_bytes;
    if total == 0 {
        return low();
    }
    let droppable_noise = (noisy_bytes + whitespace_only_bytes).min(total);
    if droppable_noise > 0 {
        let score = score_ratio(droppable_noise, total).max(score_ratio(context_bytes, total) / 2);
        return BloatEstimate {
            score,
            reason: BloatReason::DiffNoise,
        };
    }
    BloatEstimate {
        score: score_ratio(context_bytes, total),
        reason: BloatReason::DiffContext,
    }
}

fn estimate_log_bloat(content: &str) -> BloatEstimate {
    let mut total = 0usize;
    let mut repeated = 0usize;
    let mut priority = 0usize;
    let mut previous = "";
    let mut templates: std::collections::HashMap<String, usize> = std::collections::HashMap::new();
    for line in content.lines().take(4_000) {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        total += 1;
        if log_line_has_priority(trimmed) {
            priority += 1;
        }
        if trimmed == previous {
            repeated += 1;
        }
        previous = trimmed;
        let template = log_template(trimmed);
        if template != trimmed {
            *templates.entry(template).or_default() += 1;
        }
    }
    if total == 0 {
        return low();
    }
    let template_repetition = templates
        .values()
        .filter(|count| **count >= 3)
        .map(|count| count.saturating_sub(1))
        .sum::<usize>();
    let priority_dilution = total.saturating_sub(priority).saturating_sub(80);
    let score = score_ratio(repeated + template_repetition + priority_dilution, total).min(95);
    if template_repetition >= 10 {
        return BloatEstimate {
            score,
            reason: BloatReason::LogTemplates,
        };
    }
    BloatEstimate {
        score,
        reason: BloatReason::LogRepetition,
    }
}

fn estimate_search_bloat(content: &str) -> BloatEstimate {
    let mut total = 0usize;
    let mut paths: std::collections::HashMap<&str, usize> = std::collections::HashMap::new();
    for line in content.lines().take(2_000) {
        let Some((path, _rest)) = line.split_once(':') else {
            continue;
        };
        if path.trim().is_empty() {
            continue;
        }
        total += 1;
        *paths.entry(path).or_default() += 1;
    }
    if total == 0 {
        return low();
    }
    let fanout = paths.len();
    let max_cluster = paths.values().copied().max().unwrap_or(0);
    if total >= 20 && score_ratio(max_cluster, total) >= 60 {
        return BloatEstimate {
            score: score_ratio(max_cluster, total).min(95),
            reason: BloatReason::SearchClustering,
        };
    }
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

fn stable_value_hash(value: &serde_json::Value) -> u64 {
    stable_hash(serde_json::to_string(value).unwrap_or_default().as_bytes())
}

fn stable_object_hash(object: &serde_json::Map<String, serde_json::Value>) -> u64 {
    let mut bytes = Vec::new();
    for (key, value) in object {
        bytes.extend_from_slice(key.as_bytes());
        bytes.push(0);
        bytes.extend_from_slice(serde_json::to_string(value).unwrap_or_default().as_bytes());
        bytes.push(0xff);
    }
    stable_hash(&bytes)
}

fn stable_hash(bytes: &[u8]) -> u64 {
    let mut hash = 0xcbf29ce484222325u64;
    for byte in bytes {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(0x100000001b3);
    }
    hash
}

fn is_changed_diff_line(line: &str) -> bool {
    (line.starts_with('+') && !line.starts_with("+++"))
        || (line.starts_with('-') && !line.starts_with("---"))
}

fn hunk_is_whitespace_only(lines: &[&str]) -> bool {
    let removed: Vec<String> = lines
        .iter()
        .filter_map(|line| line.strip_prefix('-'))
        .map(strip_ascii_whitespace)
        .collect();
    let added: Vec<String> = lines
        .iter()
        .filter_map(|line| line.strip_prefix('+'))
        .map(strip_ascii_whitespace)
        .collect();
    !removed.is_empty() && removed.len() == added.len() && removed == added
}

fn strip_ascii_whitespace(s: &str) -> String {
    s.chars().filter(|c| !c.is_ascii_whitespace()).collect()
}

fn diff_path_is_noisy(diff_git_line: &str) -> bool {
    let line = diff_git_line.to_ascii_lowercase();
    const LOCKFILE_PATTERNS: &[&str] = &[
        "cargo.lock",
        "package-lock.json",
        "pnpm-lock.yaml",
        "yarn.lock",
        "composer.lock",
        "poetry.lock",
        "gemfile.lock",
        "go.sum",
    ];
    const GENERATED_BUNDLE_PATTERNS: &[&str] = &[".min.js", ".min.css", ".map"];
    LOCKFILE_PATTERNS
        .iter()
        .chain(GENERATED_BUNDLE_PATTERNS)
        .any(|pattern| line.contains(pattern))
}

fn log_line_has_priority(line: &str) -> bool {
    const PRIORITY_MARKERS: &[&str] = &[
        "error",
        "failed",
        "failure",
        "panic",
        "exception",
        "warning",
        "warn",
        "fatal",
        "traceback",
    ];
    let lower = line.to_ascii_lowercase();
    PRIORITY_MARKERS.iter().any(|marker| lower.contains(marker))
}

fn log_template(line: &str) -> String {
    let mut out = String::with_capacity(line.len());
    let mut chars = line.chars().peekable();
    while let Some(ch) = chars.next() {
        if ch == '0' && chars.peek() == Some(&'x') {
            out.push_str("0x#");
            chars.next();
            while chars.peek().is_some_and(|next| next.is_ascii_hexdigit()) {
                chars.next();
            }
        } else if ch.is_ascii_digit() {
            out.push('#');
            while chars.peek().is_some_and(|next| next.is_ascii_digit()) {
                chars.next();
            }
        } else {
            out.push(ch);
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn json_rows_estimate_is_redacted() {
        let content = r#"[{"secret":"a"},{"secret":"b"},{"secret":"c"}]"#;
        let estimate = estimate_bloat(content, ContentKind::Json);
        assert!(matches!(
            estimate.reason,
            BloatReason::JsonRows | BloatReason::JsonSaturation
        ));
        assert!(estimate.score > 0);
        assert!(!estimate.reason.as_str().contains("secret"));
    }

    #[test]
    fn json_saturation_detects_duplicate_constant_rows() {
        let mut rows = Vec::new();
        for i in 0..80 {
            rows.push(format!(
                r#"{{"id":{},"status":"ok","region":"us","shape":"same"}}"#,
                i % 4
            ));
        }
        let content = format!("[{}]", rows.join(","));
        let estimate = estimate_bloat(&content, ContentKind::Json);

        assert_eq!(estimate.reason, BloatReason::JsonSaturation);
        assert!(estimate.score >= 60, "{estimate:?}");
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
    fn diff_noise_estimate_detects_lockfile_hunks() {
        let mut diff = String::from("diff --git a/Cargo.lock b/Cargo.lock\n");
        diff.push_str("@@ -1,20 +1,40 @@\n");
        for i in 0..40 {
            diff.push_str(&format!("+ dep version {i}\n"));
        }
        for i in 0..20 {
            diff.push_str(&format!("- old dep version {i}\n"));
        }

        let estimate = estimate_bloat(&diff, ContentKind::Diff);

        assert_eq!(estimate.reason, BloatReason::DiffNoise);
        assert!(estimate.score > 80, "{estimate:?}");
    }

    #[test]
    fn diff_noise_estimate_detects_generated_bundle_hunks() {
        let mut diff = String::from("diff --git a/dist/app.min.js b/dist/app.min.js\n");
        diff.push_str("@@ -1,40 +1,40 @@\n");
        for i in 0..40 {
            diff.push_str(&format!("+ minified chunk {i}\n"));
        }
        for i in 0..40 {
            diff.push_str(&format!("- previous minified chunk {i}\n"));
        }

        let estimate = estimate_bloat(&diff, ContentKind::Diff);

        assert_eq!(estimate.reason, BloatReason::DiffNoise);
        assert!(estimate.score > 80, "{estimate:?}");
    }

    #[test]
    fn diff_noise_estimate_detects_whitespace_only_hunks() {
        let diff = "\
diff --git a/src/lib.rs b/src/lib.rs
@@ -1,2 +1,2 @@
-fn main(){println!(\"hi\");}
+fn main() { println!(\"hi\"); }
 context
";

        let estimate = estimate_bloat(diff, ContentKind::Diff);

        assert_eq!(estimate.reason, BloatReason::DiffNoise);
        assert!(estimate.score > 0, "{estimate:?}");
    }

    #[test]
    fn log_template_estimate_detects_repeated_variants() {
        let mut log = String::new();
        for i in 0..80 {
            log.push_str(&format!(
                "2026-07-05T12:{:02}:00Z worker-{i} processed item id={}\n",
                i % 60,
                10_000 + i
            ));
        }

        let estimate = estimate_bloat(&log, ContentKind::Log);

        assert_eq!(estimate.reason, BloatReason::LogTemplates);
        assert!(estimate.score > 50, "{estimate:?}");
    }

    #[test]
    fn search_clustering_estimate_detects_single_file_fan_in() {
        let mut output = String::new();
        for i in 0..40 {
            output.push_str(&format!("src/lib.rs:{i}:needle match {i}\n"));
        }
        for i in 0..5 {
            output.push_str(&format!("src/other.rs:{i}:needle match {i}\n"));
        }

        let estimate = estimate_bloat(&output, ContentKind::Search);

        assert_eq!(estimate.reason, BloatReason::SearchClustering);
        assert!(estimate.score >= 80, "{estimate:?}");
    }

    #[test]
    fn plain_unique_text_is_low_signal() {
        let estimate = estimate_bloat("alpha\nbeta\ngamma", ContentKind::PlainText);
        assert_eq!(estimate.reason, BloatReason::LowSignal);
        assert_eq!(estimate.score, 0);
    }
}
