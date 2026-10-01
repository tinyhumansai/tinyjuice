use super::*;

#[test]
fn strips_csi_colour() {
    assert_eq!(strip_ansi("\x1b[31mred\x1b[0m"), "red");
}

#[test]
fn strips_osc() {
    // OSC 8 hyperlink terminated with BEL
    assert_eq!(strip_ansi("\x1b]8;;http://x\x07link\x1b]8;;\x07"), "link");
}

#[test]
fn strips_incomplete_csi_at_end() {
    assert_eq!(strip_ansi("hello\x1b[1"), "hello");
}

#[test]
fn strips_csi_with_letter_terminator() {
    // ESC [ b — `[` starts a CSI sequence, `b` is the final byte → stripped
    assert_eq!(strip_ansi("a\x1b[b"), "a");
}

#[test]
fn strips_single_escape_fe_range() {
    // ESC N — falls in the @-_ range used by single-char escape sequences
    assert_eq!(strip_ansi("a\x1bNb"), "ab");
}

#[test]
fn passthrough_plain() {
    assert_eq!(strip_ansi("plain text"), "plain text");
}

#[test]
fn strips_lone_esc() {
    assert_eq!(strip_ansi("a\x1bb"), "ab");
}
