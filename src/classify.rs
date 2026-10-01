//! Rule classification: given a `ToolExecutionInput`, find the best-matching
//! `CompiledRule` and return a `ClassificationResult`.
//!
//! Port of `src/core/classify.ts` and the matching helpers from
//! `src/core/rules.ts`.

use crate::types::{ClassificationResult, CompiledRule, JsonRule, ToolExecutionInput};

// ---------------------------------------------------------------------------
// Matching helpers
// ---------------------------------------------------------------------------

/// True if every string in `expected` is present somewhere in `argv`.
fn includes_all(argv: &[String], expected: &[String]) -> bool {
    expected.iter().all(|part| argv.contains(part))
}

/// Test whether `rule` matches `input`.  Mirrors `matchesRule` in TS.
pub fn matches_rule(rule: &JsonRule, input: &ToolExecutionInput) -> bool {
    let argv = input.argv.as_deref().unwrap_or(&[]);
    // Fall back to a joined argv when `command` wasn't explicitly set so
    // `commandIncludes*` rules still match for argv-only callers.
    let command_fallback: String;
    let command: &str = match input.command.as_deref() {
        Some(c) => c,
        None => {
            command_fallback = argv.join(" ");
            &command_fallback
        }
    };
    let tool_name = &input.tool_name;

    // toolNames filter
    if let Some(tool_names) = &rule.r#match.tool_names
        && !tool_names.contains(tool_name)
    {
        return false;
    }

    // argv0 filter
    if let Some(argv0_list) = &rule.r#match.argv0 {
        let first = argv.first().map(String::as_str).unwrap_or("");
        if !argv0_list.iter().any(|s| s == first) {
            return false;
        }
    }

    // argvIncludes — all groups must match
    if let Some(groups) = &rule.r#match.argv_includes
        && !groups.iter().all(|group| includes_all(argv, group))
    {
        return false;
    }

    // argvIncludesAny — at least one group must match
    if let Some(groups) = &rule.r#match.argv_includes_any
        && !groups.iter().any(|group| includes_all(argv, group))
    {
        return false;
    }

    // commandIncludes — all substrings must appear in command
    if let Some(parts) = &rule.r#match.command_includes
        && !parts.iter().all(|part| command.contains(part.as_str()))
    {
        return false;
    }

    // commandIncludesAny — at least one substring must appear
    if let Some(parts) = &rule.r#match.command_includes_any
        && !parts.iter().any(|part| command.contains(part.as_str()))
    {
        return false;
    }

    true
}

// ---------------------------------------------------------------------------
// Scoring
// ---------------------------------------------------------------------------

/// Numeric specificity score for a rule — higher wins.
/// Mirrors `scoreRule` in TS.
fn score_rule(rule: &JsonRule) -> i64 {
    let priority = rule.priority.unwrap_or(0) as i64 * 1000;
    let argv0 = rule.r#match.argv0.as_ref().map(|v| v.len()).unwrap_or(0) as i64 * 100;
    let argv_includes = rule
        .r#match
        .argv_includes
        .as_ref()
        .map(|groups| groups.iter().map(|g| g.len()).sum::<usize>())
        .unwrap_or(0) as i64
        * 40;
    let argv_includes_any = rule
        .r#match
        .argv_includes_any
        .as_ref()
        .map(|groups| groups.iter().map(|g| g.len()).sum::<usize>())
        .unwrap_or(0) as i64
        * 35;
    let command_includes = rule
        .r#match
        .command_includes
        .as_ref()
        .map(|v| v.len())
        .unwrap_or(0) as i64
        * 25;
    let command_includes_any = rule
        .r#match
        .command_includes_any
        .as_ref()
        .map(|v| v.len())
        .unwrap_or(0) as i64
        * 20;
    let tool_names = rule
        .r#match
        .tool_names
        .as_ref()
        .map(|v| v.len())
        .unwrap_or(0) as i64
        * 10;

    priority
        + argv0
        + argv_includes
        + argv_includes_any
        + command_includes
        + command_includes_any
        + tool_names
}

// ---------------------------------------------------------------------------
// classify_execution
// ---------------------------------------------------------------------------

/// Classify `input` against the provided `rules` and return a
/// `ClassificationResult`.
///
/// If `forced_rule_id` is `Some`, that rule is used directly (if found).
pub fn classify_execution(
    input: &ToolExecutionInput,
    rules: &[CompiledRule],
    forced_rule_id: Option<&str>,
) -> ClassificationResult {
    // Forced classification
    if let Some(id) = forced_rule_id
        && let Some(rule) = rules.iter().find(|r| r.rule.id == id)
    {
        log::debug!(
            "[tinyjuice] forced classification: rule='{}' family='{}'",
            id,
            rule.rule.family
        );
        return ClassificationResult {
            family: rule.rule.family.clone(),
            confidence: 1.0,
            matched_reducer: Some(rule.rule.id.clone()),
        };
    }

    // Find all matching rules
    let matched: Vec<&CompiledRule> = rules
        .iter()
        .filter(|r| matches_rule(&r.rule, input))
        .collect();

    if matched.is_empty() {
        log::debug!(
            "[tinyjuice] no rule matched tool='{}' argv={:?} — using generic fallback",
            input.tool_name,
            input.argv
        );
        return ClassificationResult {
            family: "generic".to_owned(),
            confidence: 0.2,
            matched_reducer: None,
        };
    }

    // Only the best rule is used — a full sort (with scores recomputed inside
    // the comparator) is wasted work; a single min-by scan suffices.
    let best = matched
        .iter()
        .map(|r| (r, score_rule(&r.rule)))
        .min_by(|(a, sa), (b, sb)| sb.cmp(sa).then_with(|| a.rule.id.cmp(&b.rule.id)))
        .map(|(r, _)| *r)
        .expect("matched is non-empty");
    let confidence = if best.rule.id == "generic/fallback" {
        0.2
    } else {
        0.9
    };

    log::debug!(
        "[tinyjuice] classified tool='{}' → rule='{}' family='{}' confidence={}",
        input.tool_name,
        best.rule.id,
        best.rule.family,
        confidence
    );

    ClassificationResult {
        family: best.rule.family.clone(),
        confidence,
        matched_reducer: Some(best.rule.id.clone()),
    }
}

#[cfg(test)]
#[path = "classify_tests.rs"]
mod tests;
