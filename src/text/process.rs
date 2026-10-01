//! Line-level text processing utilities.
//!
//! Port of the processing functions in `src/core/text.ts`.

use super::width::count_text_chars;
use unicode_segmentation::UnicodeSegmentation;

const TRUNCATION_SUFFIX: &str = "\n... truncated ...";
const MIDDLE_TRUNCATION_MARKER: &str = "\n... omitted ...\n";

// ---------------------------------------------------------------------------
// Line normalization
// ---------------------------------------------------------------------------

/// Split text into lines, normalising CRLF and stripping trailing whitespace
/// per line (mirrors `normalizeLines` in TS).
pub fn normalize_lines(text: &str) -> Vec<String> {
    text.replace("\r\n", "\n")
        .split('\n')
        .map(|line| line.trim_end().to_owned())
        .collect()
}

// ---------------------------------------------------------------------------
// Edge trimming
// ---------------------------------------------------------------------------

/// Remove empty lines from the start and end of a line slice.
pub fn trim_empty_edges(lines: &[String]) -> Vec<String> {
    let start = lines
        .iter()
        .position(|l| !l.trim().is_empty())
        .unwrap_or(lines.len());
    let end = lines
        .iter()
        .rposition(|l| !l.trim().is_empty())
        .map(|i| i + 1)
        .unwrap_or(0);
    if start >= end {
        return Vec::new();
    }
    lines[start..end].to_vec()
}

// ---------------------------------------------------------------------------
// Deduplication
// ---------------------------------------------------------------------------

/// Remove adjacent duplicate lines (keeps first occurrence).
pub fn dedupe_adjacent(lines: &[String]) -> Vec<String> {
    let mut out: Vec<String> = Vec::with_capacity(lines.len());
    for line in lines {
        if out.last().map(|l: &String| l.as_str()) != Some(line.as_str()) {
            out.push(line.clone());
        }
    }
    out
}

// ---------------------------------------------------------------------------
// Head / tail summarisation
// ---------------------------------------------------------------------------

/// Keep the first `head` lines, an omission marker, and the last `tail` lines.
/// If `lines.len() <= head + tail`, returns `lines` unchanged.
pub fn head_tail(lines: &[String], head: usize, tail: usize) -> Vec<String> {
    if lines.len() <= head + tail {
        return lines.to_vec();
    }
    let omitted = lines.len() - head - tail;
    let mut out = Vec::with_capacity(head + 1 + tail);
    out.extend_from_slice(&lines[..head]);
    out.push(format!("... {} lines omitted ...", omitted));
    out.extend_from_slice(&lines[lines.len() - tail..]);
    out
}

// ---------------------------------------------------------------------------
// Clamping
// ---------------------------------------------------------------------------

/// Trim `text` at the last newline that is at or before position 50% through
/// the text (mirrors `trimHeadToLineBoundary` in TS).
fn trim_head_to_line_boundary(text: &str) -> &str {
    let last_nl = text.rfind('\n');
    match last_nl {
        None => text,
        Some(pos) => {
            if pos < text.len() / 2 {
                text
            } else {
                &text[..pos]
            }
        }
    }
}

/// Trim `text` at the first newline that is at or after position 50% through
/// (mirrors `trimTailToLineBoundary` in TS).
fn trim_tail_to_line_boundary(text: &str) -> &str {
    let first_nl = text.find('\n');
    match first_nl {
        None => text,
        Some(pos) => {
            if pos > text.len().div_ceil(2) {
                text
            } else {
                &text[pos + 1..]
            }
        }
    }
}

/// Clamp `text` to at most `max_chars` grapheme clusters (tail-truncate).
pub fn clamp_text(text: &str, max_chars: usize) -> String {
    if count_text_chars(text) <= max_chars {
        return text.to_owned();
    }
    let suffix_chars = count_text_chars(TRUNCATION_SUFFIX);
    let body_chars = max_chars.saturating_sub(suffix_chars);
    let segs: Vec<&str> = text.graphemes(true).collect();
    let head: String = segs[..body_chars.min(segs.len())].concat();
    let head = trim_head_to_line_boundary(&head);
    format!("{}{}", head, TRUNCATION_SUFFIX)
}

/// Clamp `text` to at most `max_chars` grapheme clusters using middle-truncation.
/// Keeps 70% from the head and 30% from the tail.
pub fn clamp_text_middle(text: &str, max_chars: usize) -> String {
    if count_text_chars(text) <= max_chars {
        return text.to_owned();
    }
    let marker_chars = count_text_chars(MIDDLE_TRUNCATION_MARKER);
    let body_chars = max_chars.saturating_sub(marker_chars);
    let head_chars = (body_chars as f64 * 0.7).ceil() as usize;
    let tail_chars = body_chars.saturating_sub(head_chars);

    let segs: Vec<&str> = text.graphemes(true).collect();
    let total = segs.len();

    let head_raw: String = segs[..head_chars.min(total)].concat();
    let head = trim_head_to_line_boundary(&head_raw).to_owned();

    let tail_raw: String = segs[total.saturating_sub(tail_chars)..].concat();
    let tail = trim_tail_to_line_boundary(&tail_raw).to_owned();

    format!("{}{}{}", head, MIDDLE_TRUNCATION_MARKER, tail)
}

// ---------------------------------------------------------------------------
// Pluralize
// ---------------------------------------------------------------------------

/// English pluralization matching the upstream `pluralize` function exactly.
pub fn pluralize(count: usize, noun: &str) -> String {
    // If noun already ends in "passed", "failed", "skipped" — no change
    if noun.ends_with("passed") || noun.ends_with("failed") || noun.ends_with("skipped") {
        return format!("{} {}", count, noun);
    }
    if count == 1 {
        return format!("{} {}", count, noun);
    }
    if noun.ends_with('s')
        || noun.ends_with('x')
        || noun.ends_with('z')
        || noun.ends_with("sh")
        || noun.ends_with("ch")
    {
        return format!("{} {}es", count, noun);
    }
    // [^aeiou]y → -ies. Inspect the second-to-last *char* — indexing chars by
    // byte length breaks on non-ASCII counter names.
    let ends_consonant_y = noun.ends_with('y')
        && !matches!(
            noun.chars().rev().nth(1),
            None | Some('a' | 'e' | 'i' | 'o' | 'u')
        );
    if ends_consonant_y {
        let stem = &noun[..noun.len() - 1];
        return format!("{} {}ies", count, stem);
    }
    format!("{} {}s", count, noun)
}

#[cfg(test)]
#[path = "process_tests.rs"]
mod tests;
