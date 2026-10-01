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
