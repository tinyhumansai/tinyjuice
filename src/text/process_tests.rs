use super::*;

// --- normalize_lines ---

#[test]
fn normalize_crlf() {
    assert_eq!(normalize_lines("a\r\nb"), vec!["a", "b"]);
}

#[test]
fn normalize_strips_trailing_space() {
    assert_eq!(normalize_lines("a   "), vec!["a"]);
}

// --- trim_empty_edges ---

#[test]
fn trim_edges_removes_blanks() {
    let lines: Vec<String> = ["", "a", "b", ""].iter().map(|s| s.to_string()).collect();
    assert_eq!(trim_empty_edges(&lines), vec!["a", "b"]);
}

#[test]
fn trim_edges_all_blank() {
    let lines: Vec<String> = ["", ""].iter().map(|s| s.to_string()).collect();
    assert!(trim_empty_edges(&lines).is_empty());
}

// --- dedupe_adjacent ---

#[test]
fn dedupe_keeps_non_adjacent() {
    let lines = ["a", "a", "b", "a"]
        .iter()
        .map(|s| s.to_string())
        .collect::<Vec<_>>();
    assert_eq!(dedupe_adjacent(&lines), vec!["a", "b", "a"]);
}

// --- head_tail ---

#[test]
fn head_tail_short_passthrough() {
    let lines: Vec<String> = (0..5).map(|i| format!("{}", i)).collect();
    assert_eq!(head_tail(&lines, 3, 3), lines);
}

#[test]
fn head_tail_omits_middle() {
    let lines: Vec<String> = (0..10).map(|i| format!("{}", i)).collect();
    let result = head_tail(&lines, 3, 3);
    assert_eq!(result.len(), 7); // 3 + marker + 3
    assert!(result[3].contains("4 lines omitted"));
}

// --- clamp_text ---

#[test]
fn clamp_text_passthrough_short() {
    assert_eq!(clamp_text("hi", 100), "hi");
}

#[test]
fn clamp_text_truncates() {
    let long_text = "a".repeat(2000);
    let clamped = clamp_text(&long_text, 100);
    assert!(count_text_chars(&clamped) <= 100 + count_text_chars(TRUNCATION_SUFFIX));
    assert!(clamped.ends_with("... truncated ..."));
}

// --- clamp_text_middle ---

#[test]
fn clamp_middle_passthrough_short() {
    assert_eq!(clamp_text_middle("hi", 100), "hi");
}

#[test]
fn clamp_middle_contains_marker() {
    let long_text = "a\n".repeat(200);
    let clamped = clamp_text_middle(&long_text, 50);
    assert!(
        clamped.contains("... omitted ..."),
        "missing marker in: {}",
        clamped
    );
}

// --- pluralize ---

#[test]
fn pluralize_regular() {
    assert_eq!(pluralize(2, "error"), "2 errors");
}

#[test]
fn pluralize_singular() {
    assert_eq!(pluralize(1, "error"), "1 error");
}

#[test]
fn pluralize_sibilant() {
    assert_eq!(pluralize(2, "match"), "2 matches");
}

#[test]
fn pluralize_y_ending() {
    assert_eq!(pluralize(2, "entry"), "2 entries");
}

#[test]
fn pluralize_already_ended() {
    assert_eq!(pluralize(3, "passed"), "3 passed");
}

#[test]
fn pluralize_failed_noun() {
    assert_eq!(pluralize(2, "failed"), "2 failed");
}

#[test]
fn pluralize_skipped_noun() {
    assert_eq!(pluralize(0, "skipped"), "0 skipped");
}

// --- trim_head_to_line_boundary edge cases ---

#[test]
fn clamp_text_no_newline_in_head() {
    // When there's no newline in the head portion, clamp_text still truncates
    // This exercises the "None" branch of trim_head_to_line_boundary
    let text = "a".repeat(200); // no newlines
    let clamped = clamp_text(&text, 50);
    assert!(clamped.ends_with("... truncated ..."));
}

#[test]
fn clamp_text_newline_at_early_position() {
    // Newline at position < len/2 → trim_head_to_line_boundary returns text as-is
    // (the newline is too early to use as a boundary)
    let text = "ab\n".to_owned() + &"x".repeat(200);
    let clamped = clamp_text(&text, 100);
    assert!(clamped.ends_with("... truncated ..."));
}

#[test]
fn clamp_middle_no_newline_in_tail() {
    // tail portion has no newline → trim_tail_to_line_boundary returns text as-is
    // This exercises the "None" branch of trim_tail_to_line_boundary
    let text = "line1\nline2\n".to_owned() + &"x".repeat(300);
    let clamped = clamp_text_middle(&text, 40);
    assert!(clamped.contains("... omitted ..."));
}

#[test]
fn clamp_middle_newline_at_late_position() {
    // Newline at position > len.div_ceil(2) → returns text as-is in trim_tail
    // Build tail where the first newline is very late
    let text = "line1\nline2\nline3\n".repeat(50);
    let clamped = clamp_text_middle(&text, 80);
    assert!(clamped.contains("... omitted ..."));
}

#[test]
fn clamp_middle_tail_newline_in_second_half() {
    // Force trim_tail_to_line_boundary to hit the "pos > len/2" branch:
    // The tail raw string must have its first newline past the midpoint.
    // We need a large body so the tail portion (30%) starts with many chars
    // before the first newline.
    // "xxxxxxxx\nyyyyyyy" where \n is at position > midpoint
    // Construct text with many lines; the last chunk has no early newline
    let many_lines: String = "head-line\n".repeat(100);
    // Tail segment ends with long non-newline text followed by newline at end
    let text = many_lines + &"z".repeat(200) + "\nlast";
    let clamped = clamp_text_middle(&text, 300);
    // Should produce output with the marker
    assert!(clamped.contains("... omitted ..."));
}

// --- head_tail edge cases ---

#[test]
fn head_tail_exact_boundary() {
    // lines.len() == head + tail → passthrough (not truncated)
    let lines: Vec<String> = (0..6).map(|i| format!("line{}", i)).collect();
    let result = head_tail(&lines, 3, 3);
    assert_eq!(result, lines, "exact head+tail should not truncate");
}

// --- dedupe_adjacent empty input ---

#[test]
fn dedupe_adjacent_empty() {
    assert!(dedupe_adjacent(&[]).is_empty());
}

// --- normalize_lines with no trailing whitespace ---

#[test]
fn normalize_lines_no_crlf() {
    let lines = normalize_lines("a\nb\nc");
    assert_eq!(lines, vec!["a", "b", "c"]);
}
