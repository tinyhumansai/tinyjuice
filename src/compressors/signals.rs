//! Shared importance signals for the content-router compressors.
//!
//! A small, deterministic keyword registry + per-line scorer used by the
//! search, log, and JSON compressors to decide which lines/rows to keep when a
//! tool output is over budget. No ML, no regex on the hot path beyond simple
//! case-insensitive substring scans.
//!
//! Clean-room port of Headroom's `error_detection` priority signals
//! (Apache-2.0): error/fatal lines score highest, warnings next, importance
//! markers (security/TODO) a small bump, everything else baseline.

/// Keywords that mark a hard failure (case-insensitive substrings).
const ERROR_KEYWORDS: &[&str] = &[
    "error",
    "fatal",
    "panic",
    "panicked",
    "exception",
    "traceback",
    "failed",
    "failure",
    "segfault",
    "assertion",
    "abort",
    "[error]",
    "error:",
];

/// Keywords that mark a warning. Lower weight than errors.
const WARNING_KEYWORDS: &[&str] = &["warning", "warn:", "[warn]", "deprecated"];

/// Keywords that bump importance regardless of severity.
const IMPORTANCE_KEYWORDS: &[&str] = &[
    "security",
    "vulnerability",
    "critical",
    "todo",
    "fixme",
    "denied",
    "unauthorized",
    "forbidden",
];

/// Score weights. Higher = more likely to survive truncation.
pub const SCORE_ERROR: f32 = 1.0;
pub const SCORE_WARNING: f32 = 0.6;
pub const SCORE_IMPORTANCE: f32 = 0.4;
pub const SCORE_BASELINE: f32 = 0.1;

/// True if any error keyword appears in `text` (case-insensitive).
pub fn has_error_indicators(text: &str) -> bool {
    let lower = text.to_ascii_lowercase();
    ERROR_KEYWORDS.iter().any(|kw| lower.contains(kw))
}

/// Importance score for a single line in `[0.0, 1.0]`.
pub fn line_score(line: &str) -> f32 {
    let lower = line.to_ascii_lowercase();
    let mut score = SCORE_BASELINE;
    if ERROR_KEYWORDS.iter().any(|kw| lower.contains(kw)) {
        score = score.max(SCORE_ERROR);
    }
    if WARNING_KEYWORDS.iter().any(|kw| lower.contains(kw)) {
        score = score.max(SCORE_WARNING);
    }
    if IMPORTANCE_KEYWORDS.iter().any(|kw| lower.contains(kw)) {
        score = score.max(SCORE_IMPORTANCE);
    }
    score
}

/// Classify a line's severity for the log compressor's bucketing.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Severity {
    Error,
    Warning,
    Other,
}

/// Bucket a line into [`Severity`].
///
/// An explicit level token wins over keyword scanning: a standalone
/// `WARN`/`ERROR`/… field (bracketed, colon-suffixed, or `key=LEVEL`) pins the
/// severity so a `WARN … Got exception …` line stays a warning instead of being
/// dragged into the error bucket by the substring `exception`. Only when no
/// explicit level is present do we fall back to keyword substrings, and those
/// use ASCII word boundaries so identifiers like `error_count` don't match.
pub fn severity(line: &str) -> Severity {
    if let Some(sev) = explicit_level(line) {
        return sev;
    }
    if ERROR_KEYWORDS.iter().any(|kw| contains_keyword(line, kw)) {
        Severity::Error
    } else if WARNING_KEYWORDS.iter().any(|kw| contains_keyword(line, kw)) {
        Severity::Warning
    } else {
        Severity::Other
    }
}

/// Map a bare level token to its severity, or `None` when it isn't a level.
/// Case-sensitive: only the conventional all-uppercase forms and the common
/// lowercase `warn`/`error` log forms count, so a capitalised class name like
/// `Error` in prose isn't mistaken for a level field.
fn level_severity(tok: &str) -> Option<Severity> {
    match tok {
        "ERROR" | "ERR" | "FATAL" | "CRITICAL" | "CRIT" | "SEVERE" | "EMERG" | "ALERT"
        | "error" | "err" | "fatal" | "critical" | "crit" | "severe" => Some(Severity::Error),
        "WARN" | "WARNING" | "warn" | "warning" => Some(Severity::Warning),
        "INFO" | "DEBUG" | "TRACE" | "NOTICE" | "info" | "debug" | "trace" | "notice" => {
            Some(Severity::Other)
        }
        _ => None,
    }
}

/// Detect the first explicit level token in `line`, if any. Recognises bare
/// (`WARN`), bracketed (`[WARN]`), colon-suffixed (`WARN:`) and `key=LEVEL`
/// forms — the log-level field conventionally sits at the front of the line.
fn explicit_level(line: &str) -> Option<Severity> {
    for raw in line.split_whitespace() {
        // `key=LEVEL` → test the value after the last `=`.
        let candidate = raw.rsplit('=').next().unwrap_or(raw);
        let candidate = candidate.trim_matches(|c: char| {
            matches!(
                c,
                '[' | ']' | '(' | ')' | '{' | '}' | ':' | ',' | '<' | '>' | '"' | '\''
            )
        });
        if let Some(sev) = level_severity(candidate) {
            return Some(sev);
        }
    }
    None
}

/// True if `kw` (a lowercase keyword) appears in `line` at ASCII word
/// boundaries — the characters flanking the match must not be identifier
/// characters (alphanumeric or `_`), so `error_count`/`errors_total` don't
/// count as an `error`, while `error:`/`(error)` still do.
fn contains_keyword(line: &str, kw: &str) -> bool {
    let hay = line.to_ascii_lowercase();
    let bytes = hay.as_bytes();
    let mut from = 0;
    while let Some(rel) = hay[from..].find(kw) {
        let start = from + rel;
        let end = start + kw.len();
        let before_ok = start == 0 || !is_ident_byte(bytes[start - 1]);
        let after_ok = end >= bytes.len() || !is_ident_byte(bytes[end]);
        if before_ok && after_ok {
            return true;
        }
        from = start + 1;
    }
    false
}

fn is_ident_byte(b: u8) -> bool {
    b.is_ascii_alphanumeric() || b == b'_'
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn errors_score_highest() {
        assert_eq!(line_score("FATAL: connection refused"), SCORE_ERROR);
        assert_eq!(line_score("thread panicked at 'boom'"), SCORE_ERROR);
        assert!(line_score("error: mismatched types") >= SCORE_ERROR);
    }

    #[test]
    fn warnings_below_errors_above_baseline() {
        let w = line_score("warning: unused variable");
        assert!(w < SCORE_ERROR);
        assert!(w > SCORE_BASELINE);
    }

    #[test]
    fn plain_line_is_baseline() {
        assert_eq!(line_score("   Compiling foo v0.1.0"), SCORE_BASELINE);
    }

    #[test]
    fn severity_buckets() {
        assert_eq!(severity("error[E0382]: borrow"), Severity::Error);
        assert_eq!(severity("warning: deprecated"), Severity::Warning);
        assert_eq!(severity("running 12 tests"), Severity::Other);
    }

    #[test]
    fn explicit_level_wins_over_keyword_substring() {
        // `WARN` level pins the line as a warning even though it mentions
        // `exception` — the old substring scan wrongly bucketed this as Error.
        assert_eq!(
            severity(
                "081109 214043 2561 WARN dfs.DataNode: Got exception while serving blk_1 to /10.0.0.1:"
            ),
            Severity::Warning
        );
        assert_eq!(
            severity("[WARN] task failed, will retry"),
            Severity::Warning
        );
        assert_eq!(
            severity("INFO: request failed but retried ok"),
            Severity::Other
        );
        assert_eq!(
            severity("level=warn msg=\"exception caught\""),
            Severity::Warning
        );
    }

    #[test]
    fn explicit_error_level_still_error() {
        assert_eq!(severity("ERROR something broke"), Severity::Error);
        assert_eq!(severity("2024-01-01 FATAL boom"), Severity::Error);
        assert_eq!(severity("level=error msg=oops"), Severity::Error);
    }

    #[test]
    fn keyword_needs_word_boundary() {
        // `error_count` / `errors_total` are identifiers, not an error signal.
        assert_eq!(severity("metrics error_count=0 ok"), Severity::Other);
        assert_eq!(severity("stat errors_total 0"), Severity::Other);
        // A real error keyword at a boundary still classifies.
        assert_eq!(severity("fatal: repository not found"), Severity::Error);
        assert_eq!(
            severity("connection (error) while dialing"),
            Severity::Error
        );
    }

    #[test]
    fn has_error_indicators_detects() {
        assert!(has_error_indicators("test result: FAILED"));
        assert!(!has_error_indicators("all good, 12 passed"));
    }
}
