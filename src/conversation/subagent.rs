use serde::{Deserialize, Serialize};

/// Input for deterministic subagent transcript reduction.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubagentSummaryInput {
    pub task: String,
    #[serde(default)]
    pub transcript: Vec<SubagentEvent>,
    #[serde(default)]
    pub evidence_policy: EvidencePolicy,
    pub max_bytes: usize,
}

/// One provider-neutral event from a delegated subagent run.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubagentEvent {
    pub role: SubagentEventRole,
    pub content: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tool_name: Option<String>,
    #[serde(default)]
    pub metadata: EventMetadata,
}

impl SubagentEvent {
    pub fn user(content: impl Into<String>) -> Self {
        Self::new(SubagentEventRole::User, content)
    }

    pub fn assistant(content: impl Into<String>) -> Self {
        Self::new(SubagentEventRole::Assistant, content)
    }

    pub fn tool(tool_name: impl Into<String>, content: impl Into<String>) -> Self {
        let mut event = Self::new(SubagentEventRole::Tool, content);
        event.tool_name = Some(tool_name.into());
        event
    }

    fn new(role: SubagentEventRole, content: impl Into<String>) -> Self {
        Self {
            role,
            content: content.into(),
            tool_name: None,
            metadata: EventMetadata::default(),
        }
    }
}

/// Event role without host runtime types.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SubagentEventRole {
    System,
    User,
    Assistant,
    Tool,
}

/// Redacted host metadata for an event.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EventMetadata {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub line_start: Option<usize>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub line_end: Option<usize>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tool_call_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exit_code: Option<i32>,
    #[serde(default)]
    pub failed: bool,
}

/// Bounds for deterministic evidence extraction.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EvidencePolicy {
    pub max_findings: usize,
    pub max_evidence: usize,
    pub max_open_questions: usize,
    pub max_snippet_chars: usize,
}

impl Default for EvidencePolicy {
    fn default() -> Self {
        Self {
            max_findings: 8,
            max_evidence: 12,
            max_open_questions: 6,
            max_snippet_chars: 240,
        }
    }
}

/// Deterministic subagent summary.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubagentSummaryOutput {
    pub conclusion: String,
    #[serde(default)]
    pub findings: Vec<Finding>,
    #[serde(default)]
    pub evidence: Vec<EvidenceRef>,
    #[serde(default)]
    pub open_questions: Vec<String>,
    pub omitted: OmissionReport,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Finding {
    pub text: String,
    #[serde(default)]
    pub evidence_indices: Vec<usize>,
    #[serde(default)]
    pub inferred: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EvidenceRef {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub line_start: Option<usize>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub line_end: Option<usize>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tool_name: Option<String>,
    pub snippet: String,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OmissionReport {
    pub omitted_events: usize,
    pub omitted_findings: usize,
    pub omitted_evidence: usize,
    pub omitted_open_questions: usize,
    pub byte_truncated: bool,
}

/// Reduce a delegated transcript without calling a model or inventing text.
pub fn summarize_subagent_transcript(input: &SubagentSummaryInput) -> SubagentSummaryOutput {
    let policy = input.evidence_policy;
    let mut output = SubagentSummaryOutput {
        conclusion: extract_conclusion(input),
        findings: extract_findings(input, policy),
        evidence: extract_evidence(input, policy),
        open_questions: extract_open_questions(input, policy),
        omitted: OmissionReport::default(),
    };

    attach_evidence_indices(&mut output);
    apply_policy_bounds(&mut output, input, policy);
    enforce_byte_budget(&mut output, input.max_bytes);
    output
}

/// Render a summary for model-facing handoff.
pub fn format_subagent_summary_markdown(output: &SubagentSummaryOutput) -> String {
    let mut out = String::new();
    out.push_str("## Conclusion\n");
    out.push_str(if output.conclusion.trim().is_empty() {
        "No final conclusion found in the subagent transcript."
    } else {
        output.conclusion.trim()
    });
    out.push('\n');

    if !output.findings.is_empty() {
        out.push_str("\n## Findings\n");
        for finding in &output.findings {
            out.push_str("- ");
            out.push_str(finding.text.trim());
            if !finding.evidence_indices.is_empty() {
                let refs = finding
                    .evidence_indices
                    .iter()
                    .map(|index| format!("#{}", index + 1))
                    .collect::<Vec<_>>()
                    .join(", ");
                out.push_str(" [evidence: ");
                out.push_str(&refs);
                out.push(']');
            }
            if finding.inferred {
                out.push_str(" [inference]");
            }
            out.push('\n');
        }
    }

    if !output.evidence.is_empty() {
        out.push_str("\n## Evidence\n");
        for (index, evidence) in output.evidence.iter().enumerate() {
            out.push_str(&format!("{}. ", index + 1));
            if let Some(path) = &evidence.path {
                out.push_str(path);
                if let Some(start) = evidence.line_start {
                    out.push(':');
                    out.push_str(&start.to_string());
                    if evidence.line_end.is_some_and(|end| end != start) {
                        out.push('-');
                        out.push_str(&evidence.line_end.unwrap().to_string());
                    }
                }
                out.push_str(" — ");
            } else if let Some(tool) = &evidence.tool_name {
                out.push_str(tool);
                out.push_str(" — ");
            }
            out.push_str(evidence.snippet.trim());
            out.push('\n');
        }
    }

    if !output.open_questions.is_empty() {
        out.push_str("\n## Open Questions\n");
        for question in &output.open_questions {
            out.push_str("- ");
            out.push_str(question.trim());
            out.push('\n');
        }
    }

    if output.omitted != OmissionReport::default() {
        out.push_str("\n## Omitted\n");
        if output.omitted.omitted_events > 0 {
            out.push_str(&format!(
                "- {} transcript event(s)\n",
                output.omitted.omitted_events
            ));
        }
        if output.omitted.omitted_findings > 0 {
            out.push_str(&format!(
                "- {} finding(s)\n",
                output.omitted.omitted_findings
            ));
        }
        if output.omitted.omitted_evidence > 0 {
            out.push_str(&format!(
                "- {} evidence item(s)\n",
                output.omitted.omitted_evidence
            ));
        }
        if output.omitted.omitted_open_questions > 0 {
            out.push_str(&format!(
                "- {} open question(s)\n",
                output.omitted.omitted_open_questions
            ));
        }
        if output.omitted.byte_truncated {
            out.push_str("- byte budget truncation applied\n");
        }
    }

    out.trim_end().to_string()
}

fn extract_conclusion(input: &SubagentSummaryInput) -> String {
    input
        .transcript
        .iter()
        .rev()
        .find(|event| {
            event.role == SubagentEventRole::Assistant && !event.content.trim().is_empty()
        })
        .map(|event| clean_block(&event.content))
        .unwrap_or_default()
}

fn extract_findings(input: &SubagentSummaryInput, policy: EvidencePolicy) -> Vec<Finding> {
    let Some(final_event) = input
        .transcript
        .iter()
        .rev()
        .find(|event| event.role == SubagentEventRole::Assistant)
    else {
        return fallback_findings(input, policy);
    };

    let mut out = Vec::new();
    for line in final_event.content.lines() {
        let line = clean_line(line);
        if line.is_empty() || is_heading(&line) {
            continue;
        }
        if looks_like_finding(&line) {
            out.push(Finding {
                text: line,
                evidence_indices: Vec::new(),
                inferred: false,
            });
        }
    }
    if out.is_empty() {
        out.push(Finding {
            text: clean_block(&final_event.content),
            evidence_indices: Vec::new(),
            inferred: false,
        });
    }
    out.truncate(policy.max_findings);
    out
}

fn fallback_findings(input: &SubagentSummaryInput, policy: EvidencePolicy) -> Vec<Finding> {
    let mut out = Vec::new();
    for event in &input.transcript {
        if event.metadata.failed || event.metadata.exit_code.is_some_and(|code| code != 0) {
            out.push(Finding {
                text: failure_text(event),
                evidence_indices: Vec::new(),
                inferred: false,
            });
        }
        if out.len() >= policy.max_findings {
            break;
        }
    }
    out
}

fn extract_evidence(input: &SubagentSummaryInput, policy: EvidencePolicy) -> Vec<EvidenceRef> {
    let mut out = Vec::new();
    for event in &input.transcript {
        if let Some(evidence) = evidence_from_metadata(event, policy) {
            push_unique_evidence(&mut out, evidence);
        }
        for line in event.content.lines() {
            let line = clean_line(line);
            if line.is_empty() {
                continue;
            }
            if let Some((path, start, end)) = parse_path_line_ref(&line) {
                push_unique_evidence(
                    &mut out,
                    EvidenceRef {
                        path: Some(path),
                        line_start: Some(start),
                        line_end: end,
                        tool_name: event.tool_name.clone(),
                        snippet: shrink_chars(&line, policy.max_snippet_chars),
                    },
                );
            } else if event.metadata.failed
                || event.metadata.exit_code.is_some_and(|code| code != 0)
                || is_uncertainty(&line)
            {
                push_unique_evidence(
                    &mut out,
                    EvidenceRef {
                        path: event.metadata.path.clone(),
                        line_start: event.metadata.line_start,
                        line_end: event.metadata.line_end,
                        tool_name: event.tool_name.clone(),
                        snippet: shrink_chars(&line, policy.max_snippet_chars),
                    },
                );
            }
        }
    }
    out.truncate(policy.max_evidence);
    out
}

fn evidence_from_metadata(event: &SubagentEvent, policy: EvidencePolicy) -> Option<EvidenceRef> {
    let path = event.metadata.path.as_ref()?;
    let snippet = event
        .content
        .lines()
        .map(clean_line)
        .find(|line| !line.is_empty())
        .unwrap_or_default();
    Some(EvidenceRef {
        path: Some(path.clone()),
        line_start: event.metadata.line_start,
        line_end: event.metadata.line_end,
        tool_name: event.tool_name.clone(),
        snippet: shrink_chars(&snippet, policy.max_snippet_chars),
    })
}

fn extract_open_questions(input: &SubagentSummaryInput, policy: EvidencePolicy) -> Vec<String> {
    let mut out = Vec::new();
    for event in &input.transcript {
        for line in event.content.lines() {
            let line = clean_line(line);
            if line.is_empty() {
                continue;
            }
            if is_uncertainty(&line) || line.ends_with('?') {
                push_unique_string(&mut out, shrink_chars(&line, policy.max_snippet_chars));
            }
        }
    }
    out.truncate(policy.max_open_questions);
    out
}

fn attach_evidence_indices(output: &mut SubagentSummaryOutput) {
    for finding in &mut output.findings {
        for (index, evidence) in output.evidence.iter().enumerate() {
            let Some(path) = evidence.path.as_deref() else {
                continue;
            };
            if finding.text.contains(path) {
                finding.evidence_indices.push(index);
                continue;
            }
            if let Some(file_name) = path.rsplit('/').next()
                && !file_name.is_empty()
                && finding.text.contains(file_name)
            {
                finding.evidence_indices.push(index);
            }
        }
        finding.evidence_indices.sort_unstable();
        finding.evidence_indices.dedup();
    }
}

fn apply_policy_bounds(
    output: &mut SubagentSummaryOutput,
    input: &SubagentSummaryInput,
    policy: EvidencePolicy,
) {
    output.omitted.omitted_events = input
        .transcript
        .len()
        .saturating_sub(events_represented(output));
    if output.findings.len() > policy.max_findings {
        output.omitted.omitted_findings = output.findings.len() - policy.max_findings;
        output.findings.truncate(policy.max_findings);
    }
    if output.evidence.len() > policy.max_evidence {
        output.omitted.omitted_evidence = output.evidence.len() - policy.max_evidence;
        output.evidence.truncate(policy.max_evidence);
    }
    if output.open_questions.len() > policy.max_open_questions {
        output.omitted.omitted_open_questions =
            output.open_questions.len() - policy.max_open_questions;
        output.open_questions.truncate(policy.max_open_questions);
    }
}

fn events_represented(output: &SubagentSummaryOutput) -> usize {
    let mut count = 0;
    if !output.conclusion.trim().is_empty() {
        count += 1;
    }
    count + output.findings.len() + output.evidence.len() + output.open_questions.len()
}

fn enforce_byte_budget(output: &mut SubagentSummaryOutput, max_bytes: usize) {
    if max_bytes == 0 {
        output.conclusion.clear();
        output.findings.clear();
        output.evidence.clear();
        output.open_questions.clear();
        output.omitted.byte_truncated = true;
        return;
    }

    while format_subagent_summary_markdown(output).len() > max_bytes {
        if output.evidence.pop().is_some() {
            output.omitted.omitted_evidence += 1;
        } else if output.findings.pop().is_some() {
            output.omitted.omitted_findings += 1;
        } else if output.open_questions.pop().is_some() {
            output.omitted.omitted_open_questions += 1;
        } else {
            let keep = max_bytes.saturating_sub(96).max(24);
            output.conclusion =
                crate::util::utf8_safe_prefix_at_byte_boundary(&output.conclusion, keep)
                    .trim_end()
                    .to_string();
            output.omitted.byte_truncated = true;
            break;
        }
        output.omitted.byte_truncated = true;
    }
}

fn failure_text(event: &SubagentEvent) -> String {
    let mut text = String::new();
    if let Some(tool) = &event.tool_name {
        text.push_str(tool);
        text.push_str(": ");
    }
    if let Some(exit_code) = event.metadata.exit_code {
        text.push_str(&format!("exit code {exit_code}; "));
    }
    text.push_str(clean_block(&event.content).as_str());
    text
}

fn parse_path_line_ref(line: &str) -> Option<(String, usize, Option<usize>)> {
    for raw in line.split_whitespace() {
        let token = raw.trim_matches(|ch: char| {
            matches!(
                ch,
                '(' | ')' | '[' | ']' | '{' | '}' | ',' | ';' | '"' | '\'' | '`'
            )
        });
        let Some((path, line_part)) = token.rsplit_once(':') else {
            continue;
        };
        if !looks_like_path(path) {
            continue;
        }
        let (start, end) = parse_line_range(line_part)?;
        return Some((path.to_string(), start, end));
    }
    None
}

fn parse_line_range(value: &str) -> Option<(usize, Option<usize>)> {
    if let Some((start, end)) = value.split_once('-') {
        let start = start.parse().ok()?;
        let end = end.parse().ok();
        Some((start, end))
    } else {
        Some((value.parse().ok()?, None))
    }
}

fn looks_like_path(value: &str) -> bool {
    value.contains('/')
        || value.contains('\\')
        || value.rsplit_once('.').is_some_and(|(_, ext)| {
            !ext.is_empty() && ext.chars().all(|ch| ch.is_ascii_alphanumeric())
        })
}

fn looks_like_finding(line: &str) -> bool {
    line.starts_with("- ")
        || line.starts_with("* ")
        || line.starts_with("Finding:")
        || line.starts_with("Conclusion:")
        || line.contains(':')
        || parse_path_line_ref(line).is_some()
}

fn is_heading(line: &str) -> bool {
    line.starts_with('#')
        || matches!(
            line.trim_end_matches(':').to_ascii_lowercase().as_str(),
            "findings" | "evidence" | "open questions" | "conclusion" | "summary"
        )
}

fn is_uncertainty(line: &str) -> bool {
    let lower = line.to_ascii_lowercase();
    [
        "uncertain",
        "unknown",
        "not sure",
        "could not",
        "failed",
        "error",
        "contradict",
        "conflict",
        "missing",
        "unverified",
    ]
    .iter()
    .any(|needle| lower.contains(needle))
}

fn clean_block(value: &str) -> String {
    value
        .lines()
        .map(clean_line)
        .filter(|line| !line.is_empty())
        .collect::<Vec<_>>()
        .join("\n")
}

fn clean_line(value: &str) -> String {
    value
        .trim()
        .trim_start_matches("- ")
        .trim_start_matches("* ")
        .trim()
        .to_string()
}

fn shrink_chars(value: &str, max_chars: usize) -> String {
    if value.chars().count() <= max_chars {
        return value.to_string();
    }
    let mut out: String = value.chars().take(max_chars.saturating_sub(1)).collect();
    out.push_str("...");
    out
}

fn push_unique_evidence(out: &mut Vec<EvidenceRef>, evidence: EvidenceRef) {
    if out.iter().any(|existing| {
        existing.path == evidence.path
            && existing.line_start == evidence.line_start
            && existing.line_end == evidence.line_end
            && existing.snippet == evidence.snippet
    }) {
        return;
    }
    out.push(evidence);
}

fn push_unique_string(out: &mut Vec<String>, value: String) {
    if !out.iter().any(|existing| existing == &value) {
        out.push(value);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input(transcript: Vec<SubagentEvent>, max_bytes: usize) -> SubagentSummaryInput {
        SubagentSummaryInput {
            task: "inspect auth flow".to_string(),
            transcript,
            evidence_policy: EvidencePolicy::default(),
            max_bytes,
        }
    }

    #[test]
    fn long_transcript_reduces_to_conclusion_plus_evidence() {
        let transcript = vec![
            SubagentEvent::user("Find the auth handler."),
            SubagentEvent::tool("grep", "src/auth.rs:42: pub fn login() {}\nnoise"),
            SubagentEvent::assistant(
                "Findings:\n- login is defined in src/auth.rs:42\n\nConclusion: edit auth.rs",
            ),
        ];
        let output = summarize_subagent_transcript(&input(transcript, 4_000));

        assert!(output.conclusion.contains("login is defined"));
        assert_eq!(output.evidence.len(), 1);
        assert_eq!(output.evidence[0].path.as_deref(), Some("src/auth.rs"));
        assert_eq!(output.evidence[0].line_start, Some(42));
        assert_eq!(output.findings[0].evidence_indices, vec![0]);
    }

    #[test]
    fn failed_exploration_keeps_failure_reason() {
        let mut failed = SubagentEvent::tool("shell", "error: permission denied");
        failed.metadata.exit_code = Some(1);
        failed.metadata.failed = true;
        let output = summarize_subagent_transcript(&input(vec![failed], 4_000));

        assert!(output.findings[0].text.contains("exit code 1"));
        assert!(output.evidence[0].snippet.contains("permission denied"));
        assert!(output.open_questions[0].contains("error"));
    }

    #[test]
    fn contradictory_findings_remain_uncertainty() {
        let transcript = vec![
            SubagentEvent::assistant("src/a.rs:10 says enabled"),
            SubagentEvent::assistant("contradiction: src/b.rs:20 says disabled"),
        ];
        let output = summarize_subagent_transcript(&input(transcript, 4_000));

        assert!(
            output
                .open_questions
                .iter()
                .any(|question| question.contains("contradiction"))
        );
        assert_eq!(output.evidence.len(), 2);
    }

    #[test]
    fn byte_budget_produces_omission_report() {
        let transcript = vec![
            SubagentEvent::tool("grep", "src/a.rs:10: first\nsrc/b.rs:20: second"),
            SubagentEvent::assistant("Conclusion: src/a.rs:10 and src/b.rs:20 need review"),
        ];
        let output = summarize_subagent_transcript(&input(transcript, 120));
        let rendered = format_subagent_summary_markdown(&output);

        assert!(rendered.len() <= 120);
        assert!(output.omitted.byte_truncated);
        assert!(output.omitted.omitted_evidence > 0 || output.omitted.omitted_findings > 0);
    }
}
