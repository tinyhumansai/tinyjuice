//! Lossless log template mining for the signal log compressor.
//!
//! Drain-inspired online template extraction, a clean-room reimplementation
//! modelled on Headroom's log-clustering approach (Apache-2.0) — no code copied,
//! only the idea: consecutive lines with the same token count are folded into a
//! per-position template where columns that vary become the wildcard `<*>`.
//!
//! Two collapse surfaces build on this:
//!
//! - [`template_key`] turns a line into a prefix-preserving dedupe key so the
//!   compressor can keep one exemplar per distinct error/warning template plus a
//!   `×N` count, instead of a flat first-few/last-few window.
//! - [`summarize_region`] mines a dropped INFO/DEBUG region and, when it is
//!   dominated by a handful of templates, returns human-readable summary lines
//!   (`[Template: … <*> …] (N lines, first <ts1>, last <ts2>)`) that ride
//!   alongside the omission marker — the marker still carries the CCR token that
//!   makes the region recoverable, so nothing is lost.

/// Minimum consecutive lines for a run to become a template.
const MIN_RUN: usize = 3;
/// A run's tokens must match the template at ≥ this fraction of positions.
const MATCH_RATIO: f32 = 0.4;
/// Wildcard placeholder rendered in a template display.
const WILDCARD: &str = "<*>";
/// Region summarisation only fires when this few distinct templates dominate.
const MAX_REGION_TEMPLATES: usize = 3;

/// A mined template run over some line slice.
#[derive(Debug, Clone)]
pub struct TemplateRun {
    /// Per-position tokens; `None` is a wildcard column.
    template: Vec<Option<String>>,
    /// Number of lines folded into this run.
    pub count: usize,
    /// First / last recognised timestamp across the run's lines.
    pub first_ts: Option<String>,
    pub last_ts: Option<String>,
    /// Half-open `[start, end)` range into the mined slice.
    pub start: usize,
    pub end: usize,
}

impl TemplateRun {
    /// Number of fixed (non-wildcard) columns.
    fn const_positions(&self) -> usize {
        self.template.iter().filter(|t| t.is_some()).count()
    }

    /// Number of wildcard columns.
    fn wildcards(&self) -> usize {
        self.template.iter().filter(|t| t.is_none()).count()
    }

    /// Render the template with `<*>` for wildcard columns.
    pub fn display(&self) -> String {
        self.template
            .iter()
            .map(|t| t.as_deref().unwrap_or(WILDCARD))
            .collect::<Vec<_>>()
            .join(" ")
    }

    /// A run is worth emitting when it is long enough, keeps at least two fixed
    /// columns, and has at least one wildcard (rejecting all-wildcard noise).
    fn is_emittable(&self) -> bool {
        self.count >= MIN_RUN && self.const_positions() >= 2 && self.wildcards() >= 1
    }
}

/// Drain-inspired online mining over `lines`: walk once, growing a run while
/// successive lines share the token count and match the running template at
/// ≥ [`MATCH_RATIO`] of positions; break the run otherwise. Returns only the
/// emittable runs, in line order.
pub fn mine(lines: &[&str]) -> Vec<TemplateRun> {
    let mut out: Vec<TemplateRun> = Vec::new();
    let mut cur: Option<TemplateRun> = None;

    for (i, line) in lines.iter().enumerate() {
        let tokens: Vec<&str> = line.split_whitespace().collect();
        let ts = parse_timestamp(line);

        match cur.as_mut() {
            Some(run) if run.template.len() == tokens.len() && matches(run, &tokens) => {
                merge(run, &tokens);
                run.count += 1;
                run.end = i + 1;
                if ts.is_some() {
                    run.last_ts = ts;
                }
            }
            _ => {
                if let Some(run) = cur.take()
                    && run.is_emittable()
                {
                    out.push(run);
                }
                cur = Some(TemplateRun {
                    template: tokens.iter().map(|t| Some((*t).to_string())).collect(),
                    count: 1,
                    first_ts: ts.clone(),
                    last_ts: ts,
                    start: i,
                    end: i + 1,
                });
            }
        }
    }
    if let Some(run) = cur.take()
        && run.is_emittable()
    {
        out.push(run);
    }
    out
}

/// Fraction of positions where `tokens` matches the run's template (wildcard
/// columns always count as a match) is ≥ [`MATCH_RATIO`].
fn matches(run: &TemplateRun, tokens: &[&str]) -> bool {
    if tokens.is_empty() {
        return false;
    }
    let hits = run
        .template
        .iter()
        .zip(tokens)
        .filter(|(t, tok)| match t {
            None => true,
            Some(v) => v == *tok,
        })
        .count();
    hits as f32 / tokens.len() as f32 >= MATCH_RATIO
}

/// Widen the template: any fixed column that disagrees with `tokens` becomes a
/// wildcard.
fn merge(run: &mut TemplateRun, tokens: &[&str]) {
    for (slot, tok) in run.template.iter_mut().zip(tokens) {
        if let Some(v) = slot
            && v != tok
        {
            *slot = None;
        }
    }
}

/// Summarise a dropped region. Returns one display line per template when the
/// region is dominated by ≤ [`MAX_REGION_TEMPLATES`] emittable templates that
/// together cover most of the region; otherwise `None` (the caller then emits a
/// plain omission marker). Purely descriptive — recovery still rides on the
/// marker's CCR token.
pub fn summarize_region(lines: &[&str]) -> Option<Vec<String>> {
    if lines.len() < MIN_RUN {
        return None;
    }
    let runs = mine(lines);
    if runs.is_empty() || runs.len() > MAX_REGION_TEMPLATES {
        return None;
    }
    let covered: usize = runs.iter().map(|r| r.count).sum();
    // Require the templates to explain at least 80% of the region.
    if covered * 5 < lines.len() * 4 {
        return None;
    }
    Some(runs.iter().map(render_summary).collect())
}

/// `[Template: … <*> …] (N lines, first <ts1>, last <ts2>)`.
fn render_summary(run: &TemplateRun) -> String {
    let mut s = format!("[Template: {}] ({} lines", run.display(), run.count);
    if let (Some(a), Some(b)) = (&run.first_ts, &run.last_ts) {
        s.push_str(&format!(", first {a}, last {b}"));
    }
    s.push(')');
    s
}

/// Prefix-preserving dedupe key: two lines share a key iff they are the same
/// template. The leading timestamp and any purely-numeric thread/pid fields are
/// stripped, the prefix up to the first `:`/`=` is kept verbatim (so
/// `HTTP 500 …` and `HTTP 404 …` stay distinct), and only the tail has its
/// variable tokens (numbers, hex, IPs, UUIDs) folded to placeholders.
pub fn template_key(line: &str) -> String {
    let mut rest = line.trim_start();

    // Drop a leading timestamp.
    if let Some(ts) = parse_timestamp(rest)
        && rest.starts_with(&ts)
    {
        rest = rest[ts.len()..].trim_start();
    }

    // Drop leading purely-numeric fields (thread ids / pids).
    loop {
        let tok = rest.split_whitespace().next().unwrap_or("");
        if !tok.is_empty() && tok.bytes().all(|b| b.is_ascii_digit()) {
            rest = rest[tok.len()..].trim_start();
        } else {
            break;
        }
    }

    // Split at the first `:` or `=` into a verbatim prefix and a normalised tail.
    let split = rest.find([':', '=']);
    let (prefix, tail) = match split {
        Some(idx) => (&rest[..idx], &rest[idx..]),
        None => ("", rest),
    };

    let mut key = prefix.to_ascii_lowercase();
    key.push('\u{1}'); // separator so prefix/tail can't collide
    key.push_str(&normalize_tail(tail));
    key
}

/// Fold a message tail to its template form: lowercase, collapse whitespace,
/// and replace variable tokens with `<ip>`/`<uuid>`/`<hex>`/`<n>` placeholders.
fn normalize_tail(tail: &str) -> String {
    tail.split_whitespace()
        .map(normalize_token)
        .collect::<Vec<_>>()
        .join(" ")
}

fn normalize_token(tok: &str) -> String {
    // Strip surrounding punctuation for classification, remember it to re-attach.
    let lead: String = tok
        .chars()
        .take_while(|c| !c.is_ascii_alphanumeric())
        .collect();
    let trail: String = tok
        .chars()
        .rev()
        .take_while(|c| !c.is_ascii_alphanumeric())
        .collect::<String>()
        .chars()
        .rev()
        .collect();
    let core = &tok[lead.len()..tok.len() - trail.len().min(tok.len() - lead.len())];
    if core.is_empty() {
        // Pure punctuation — nothing to normalise.
        return tok.to_ascii_lowercase();
    }

    let replaced = if is_ipv4(core) {
        "<ip>".to_string()
    } else if is_uuid(core) {
        "<uuid>".to_string()
    } else if is_hexish(core) {
        "<hex>".to_string()
    } else {
        replace_digit_runs(&core.to_ascii_lowercase())
    };
    format!(
        "{}{}{}",
        lead.to_ascii_lowercase(),
        replaced,
        trail.to_ascii_lowercase()
    )
}

/// Replace maximal ASCII-digit runs (with an optional leading `-`/`+` sign)
/// with `<n>`, so signed ids like `blk_-2918` and `blk_8376` share a template.
fn replace_digit_runs(s: &str) -> String {
    let chars: Vec<char> = s.chars().collect();
    let mut out = String::with_capacity(s.len());
    let mut i = 0;
    while i < chars.len() {
        let c = chars[i];
        let signed =
            matches!(c, '-' | '+') && i + 1 < chars.len() && chars[i + 1].is_ascii_digit();
        if c.is_ascii_digit() || signed {
            if signed {
                i += 1;
            }
            while i < chars.len() && chars[i].is_ascii_digit() {
                i += 1;
            }
            out.push_str("<n>");
        } else {
            out.push(c);
            i += 1;
        }
    }
    out
}

fn is_ipv4(s: &str) -> bool {
    // Accept `a.b.c.d` optionally followed by `:port`.
    let host = s.split(':').next().unwrap_or(s);
    let parts: Vec<&str> = host.split('.').collect();
    parts.len() == 4
        && parts
            .iter()
            .all(|p| !p.is_empty() && p.len() <= 3 && p.bytes().all(|b| b.is_ascii_digit()))
}

fn is_uuid(s: &str) -> bool {
    let b = s.as_bytes();
    b.len() == 36
        && b.iter().enumerate().all(|(i, &c)| {
            if matches!(i, 8 | 13 | 18 | 23) {
                c == b'-'
            } else {
                c.is_ascii_hexdigit()
            }
        })
}

fn is_hexish(s: &str) -> bool {
    let s = s.strip_prefix("0x").or_else(|| s.strip_prefix("0X")).unwrap_or(s);
    s.len() >= 8
        && s.bytes().any(|b| b.is_ascii_digit())
        && s.bytes().any(|b| b.is_ascii_alphabetic())
        && s.bytes().all(|b| b.is_ascii_hexdigit())
}

/// Recognise a leading/common timestamp shape and return the raw matched text
/// (no date maths). Handles `YYMMDD HHMMSS` (HDFS), ISO-8601, syslog
/// `Mon DD HH:MM:SS`, and a bare `HH:MM:SS(.mmm)?`.
pub fn parse_timestamp(line: &str) -> Option<String> {
    let toks: Vec<&str> = line.split_whitespace().collect();
    if toks.is_empty() {
        return None;
    }

    // HDFS `YYMMDD HHMMSS`: two 6-digit numeric tokens.
    if toks.len() >= 2 && is_digits(toks[0], 6) && is_digits(toks[1], 6) {
        return Some(format!("{} {}", toks[0], toks[1]));
    }

    // Syslog `Mon DD HH:MM:SS`.
    if toks.len() >= 3
        && is_month(toks[0])
        && toks[1].len() <= 2
        && toks[1].bytes().all(|b| b.is_ascii_digit())
        && is_clock(toks[2])
    {
        return Some(format!("{} {} {}", toks[0], toks[1], toks[2]));
    }

    // ISO-8601: a token carrying `YYYY-MM-DD` (optionally `THH:MM:SS…`).
    for tok in &toks {
        if is_iso8601(tok) {
            return Some((*tok).to_string());
        }
    }

    // Bare clock `HH:MM:SS(.mmm)?` anywhere in the leading tokens.
    for tok in toks.iter().take(4) {
        let t = tok.trim_matches(|c: char| matches!(c, '[' | ']' | '(' | ')'));
        if is_clock(t) {
            return Some(t.to_string());
        }
    }
    None
}

fn is_digits(s: &str, len: usize) -> bool {
    s.len() == len && s.bytes().all(|b| b.is_ascii_digit())
}

fn is_month(s: &str) -> bool {
    matches!(
        s,
        "Jan" | "Feb" | "Mar" | "Apr" | "May" | "Jun" | "Jul" | "Aug" | "Sep" | "Oct" | "Nov"
            | "Dec"
    )
}

/// `HH:MM:SS` with an optional `.fraction` suffix.
fn is_clock(s: &str) -> bool {
    let (clock, frac) = match s.split_once('.') {
        Some((a, b)) => (a, Some(b)),
        None => (s, None),
    };
    let parts: Vec<&str> = clock.split(':').collect();
    if parts.len() != 3 {
        return false;
    }
    if !parts
        .iter()
        .all(|p| p.len() == 2 && p.bytes().all(|b| b.is_ascii_digit()))
    {
        return false;
    }
    frac.is_none_or(|f| !f.is_empty() && f.bytes().all(|b| b.is_ascii_digit()))
}

/// A token that begins with `YYYY-MM-DD` (ISO date, optionally with a time).
fn is_iso8601(s: &str) -> bool {
    let b = s.as_bytes();
    b.len() >= 10
        && b[..4].iter().all(|c| c.is_ascii_digit())
        && b[4] == b'-'
        && b[5..7].iter().all(|c| c.is_ascii_digit())
        && b[7] == b'-'
        && b[8..10].iter().all(|c| c.is_ascii_digit())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_common_timestamp_shapes() {
        assert_eq!(
            parse_timestamp("081109 203615 148 INFO dfs.DataNode: x").as_deref(),
            Some("081109 203615")
        );
        assert_eq!(
            parse_timestamp("2024-01-02T10:11:12.500Z did a thing").as_deref(),
            Some("2024-01-02T10:11:12.500Z")
        );
        assert_eq!(
            parse_timestamp("Dec  fake").as_deref(),
            None,
            "single month token isn't a timestamp"
        );
        assert_eq!(
            parse_timestamp("Nov 10 04:41:03 host sshd: boom").as_deref(),
            Some("Nov 10 04:41:03")
        );
        assert_eq!(
            parse_timestamp("12:34:56.789 request served").as_deref(),
            Some("12:34:56.789")
        );
        assert_eq!(parse_timestamp("no timestamp at all here"), None);
    }

    #[test]
    fn template_key_folds_variable_tail_but_keeps_prefix() {
        let a = "081109 214043 2561 WARN dfs.DataNode: Got exception while serving blk_-2918118818249673980 to /10.251.90.64:";
        let b = "081109 214402 2677 WARN dfs.DataNode: Got exception while serving blk_8376667364205250596 to /10.251.91.159:";
        assert_eq!(template_key(a), template_key(b), "same template must collapse");
    }

    #[test]
    fn template_key_prefix_distinguishes_numeric_codes() {
        // Different status codes before the `:` must not merge.
        let a = template_key("HTTP 500 backend: request handling failed");
        let b = template_key("HTTP 404 backend: request handling failed");
        assert_ne!(a, b, "distinct prefixes must stay distinct");
    }

    #[test]
    fn mine_folds_a_run_into_one_wildcard_template() {
        let lines: Vec<String> = (0..20)
            .map(|i| format!("worker {i} processed batch ok"))
            .collect();
        let refs: Vec<&str> = lines.iter().map(String::as_str).collect();
        let runs = mine(&refs);
        assert_eq!(runs.len(), 1, "one run expected");
        assert_eq!(runs[0].count, 20);
        assert!(runs[0].display().contains("<*>"), "{}", runs[0].display());
    }

    #[test]
    fn mine_rejects_all_wildcard_and_short_runs() {
        // Every column varies → all-wildcard → rejected.
        let lines = ["a b c", "d e f", "g h i", "j k l"];
        assert!(mine(&lines).is_empty());
        // Fewer than MIN_RUN identical-shape lines → rejected.
        let short = ["x 1 y", "x 2 y"];
        assert!(mine(&short).is_empty());
    }

    #[test]
    fn summarize_region_reports_dominant_template() {
        let lines: Vec<String> = (0..30)
            .map(|i| format!("Compiling crate_{i} v0.1.0"))
            .collect();
        let refs: Vec<&str> = lines.iter().map(String::as_str).collect();
        let summary = summarize_region(&refs).expect("dominant template");
        assert_eq!(summary.len(), 1);
        assert!(summary[0].contains("[Template:"), "{}", summary[0]);
        assert!(summary[0].contains("30 lines"), "{}", summary[0]);
    }

    #[test]
    fn summarize_region_declines_when_diverse() {
        // Many distinct shapes → not dominated by a few templates.
        let lines: Vec<String> = (0..30)
            .map(|i| {
                let extra = "x ".repeat(i % 7);
                format!("{extra}line {i} end")
            })
            .collect();
        let refs: Vec<&str> = lines.iter().map(String::as_str).collect();
        assert!(summarize_region(&refs).is_none());
    }

    #[test]
    fn miner_never_inflates_via_region_summary() {
        // The summary lines must be shorter than the region they describe.
        let lines: Vec<String> = (0..40)
            .map(|i| format!("081109 20{:04} INFO dfs.DataNode: block blk_{i} stored ok", i))
            .collect();
        let refs: Vec<&str> = lines.iter().map(String::as_str).collect();
        let region_len: usize = lines.iter().map(|l| l.len() + 1).sum();
        if let Some(summary) = summarize_region(&refs) {
            let summary_len: usize = summary.iter().map(|l| l.len() + 1).sum();
            assert!(summary_len < region_len, "summary must not inflate");
        }
    }
}
