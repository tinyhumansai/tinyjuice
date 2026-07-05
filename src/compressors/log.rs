//! Build/test/lint log compressor.
//!
//! Two paths, chosen by whether the content is *command* output:
//!
//! - **Command output** (the [`CompressInput`] carries a derived command/argv):
//!   run the 100-rule reduction engine ([`reduce_execution_with_rules`]). This
//!   is TokenJuice's original behaviour — git/cargo/npm/docker-aware rules with
//!   failure preservation.
//! - **Non-command logs** (a blob detected as a log with no command context):
//!   the signal-based keep-failures/drop-noise compressor, a clean-room port of
//!   Headroom's `LogCompressor` (Apache-2.0).
//!
//! The signal path declines (returns `None`) when a blob has no error/warning/
//! summary/stack signal at all — that almost certainly isn't a log (a file
//! listing, CSV, generated data) and must not be head/tail truncated.

use async_trait::async_trait;
use std::collections::{BTreeSet, HashMap};
use std::fmt::Write as _;

use super::log_template;
use super::signals::{Severity, severity};
use super::{BLOCK_NOTE, Compressor, block_token};
use crate::reduce::reduce_execution_with_rules;
use crate::rules::cached_overlay_rules;
use crate::types::{
    CompressInput, CompressOptions, CompressOutput, CompressorKind, ReduceOptions,
    ToolExecutionInput,
};

/// Safety ceiling on distinct error templates kept (all distinct templates are
/// kept below this; the cap only guards a pathological blob).
pub const MAX_ERROR_TEMPLATES: usize = 40;
/// Distinct warning templates kept.
pub const MAX_WARNING_TEMPLATES: usize = 10;
pub const MAX_STACK_TRACES: usize = 3;
pub const STACK_TRACE_MAX_LINES: usize = 20;
pub const MAX_TOTAL_LINES: usize = 100;

/// Lines of causal context kept before/after each distinct error exemplar.
const CONTEXT_BEFORE: usize = 2;
const CONTEXT_AFTER: usize = 2;
/// Gaps of at most this many lines are emitted verbatim rather than replaced by
/// a marker + CCR token (the marker would outweigh the omitted text).
const MIN_GAP: usize = 2;
/// Only mine a dropped region for template summaries when it is at least this
/// many lines — small gaps aren't worth a summary line.
const MIN_REGION_FOR_TEMPLATES: usize = 6;

pub struct LogCompressor;

#[async_trait]
impl Compressor for LogCompressor {
    fn kind(&self) -> CompressorKind {
        CompressorKind::Log
    }

    async fn compress(
        &self,
        input: &CompressInput<'_>,
        opts: &CompressOptions,
    ) -> Option<CompressOutput> {
        let has_command =
            input.command.is_some() || input.argv.as_ref().is_some_and(|a| !a.is_empty());
        if has_command {
            compress_command(input, opts)
        } else {
            compress_signal(input.content, opts.ccr_enabled)
        }
    }
}

/// Command output → run the rule engine, tagged as [`CompressorKind::Log`].
fn compress_command(input: &CompressInput<'_>, opts: &CompressOptions) -> Option<CompressOutput> {
    run_rule_engine(input, opts, CompressorKind::Log)
}

/// The generic fallback path: same rule engine, tagged [`CompressorKind::Generic`].
/// Exposed for [`super::generic::GenericCompressor`] so the router can fall back
/// to head/tail summarisation of command output without re-implementing it.
pub fn compress_command_fallback(
    input: &CompressInput<'_>,
    opts: &CompressOptions,
) -> Option<CompressOutput> {
    run_rule_engine(input, opts, CompressorKind::Generic)
}

/// Run the 100-rule reduction engine over command output, reporting `kind`.
/// Returns `None` when reduction wouldn't shrink the payload.
fn run_rule_engine(
    input: &CompressInput<'_>,
    opts: &CompressOptions,
    kind: CompressorKind,
) -> Option<CompressOutput> {
    let exec = ToolExecutionInput {
        tool_name: input
            .hint
            .source_tool
            .clone()
            .unwrap_or_else(|| "shell".to_string()),
        command: input.command.clone(),
        argv: input.argv.clone(),
        stdout: Some(input.content.to_string()),
        exit_code: input.exit_code,
        ..Default::default()
    };
    let reduce_opts = ReduceOptions {
        max_inline_chars: opts.max_inline_chars,
        ..Default::default()
    };
    // Full three-layer overlay (builtin → user → project) so project rules in
    // `<cwd>/.tinyjuice/rules/` apply to the compression path, as documented.
    let rules = cached_overlay_rules(None);
    let result = reduce_execution_with_rules(exec, &rules, &reduce_opts);
    if result.inline_text.len() >= input.content.len() {
        return None;
    }
    let rule_label = result
        .classification
        .matched_reducer
        .unwrap_or(result.classification.family);
    log::debug!(
        "[tinyjuice][log] command rule={} kind={} {} -> {} bytes",
        rule_label,
        kind.as_str(),
        input.content.len(),
        result.inline_text.len()
    );
    Some(CompressOutput::lossy(result.inline_text, kind))
}

/// Signal-based log compression for non-command blobs detected as logs. With
/// `block_tokens`, each omitted gap is offloaded to CCR and its marker carries
/// a token that retrieves exactly those lines.
pub fn compress_signal(content: &str, block_tokens: bool) -> Option<CompressOutput> {
    let lines: Vec<&str> = content.lines().collect();
    if lines.len() <= MAX_TOTAL_LINES {
        return None;
    }

    let mut keep: BTreeSet<usize> = BTreeSet::new();
    // Per-kept-line suffixes (e.g. `×N` template counts) appended on emit.
    let mut annotations: HashMap<usize, String> = HashMap::new();

    for (i, line) in lines.iter().enumerate() {
        if is_summary_line(line) {
            keep.insert(i);
        }
    }

    // Precompute stack-frame membership so error classification can exclude
    // frames (a `panicking` frame must not become a keyword-derived error) and
    // the trace walk can reuse it.
    let stack_frame: Vec<bool> = lines.iter().map(|l| is_stack_frame(l)).collect();

    // Errors: keep one exemplar per distinct template with an `×N` count and a
    // small window of causal context; keep every distinct template up to a
    // safety ceiling (no flat first-few/last-few cap).
    collapse_by_template(
        &lines,
        |i| severity(lines[i]) == Severity::Error && !stack_frame[i],
        MAX_ERROR_TEMPLATES,
        true,
        &mut keep,
        &mut annotations,
    );

    // Warnings: exemplar + count per distinct template, fewer distinct kept.
    collapse_by_template(
        &lines,
        |i| severity(lines[i]) == Severity::Warning && !stack_frame[i],
        MAX_WARNING_TEMPLATES,
        false,
        &mut keep,
        &mut annotations,
    );

    let mut traces_kept = 0usize;
    let mut i = 0usize;
    while i < lines.len() && traces_kept < MAX_STACK_TRACES {
        if stack_frame[i] {
            let start = i;
            let mut taken = 0usize;
            while i < lines.len() && stack_frame[i] {
                if taken < STACK_TRACE_MAX_LINES {
                    keep.insert(i);
                    taken += 1;
                }
                i += 1;
            }
            if i > start {
                traces_kept += 1;
            }
        } else {
            i += 1;
        }
    }

    if keep.is_empty() {
        // Not a log (no signal) — never head/tail truncate legitimate data.
        return None;
    }

    let kept_vec: Vec<usize> = keep.iter().copied().collect();
    let kept_vec = if kept_vec.len() > MAX_TOTAL_LINES {
        select_first_last(&kept_vec, MAX_TOTAL_LINES)
    } else {
        kept_vec
    };
    let kept_set: BTreeSet<usize> = kept_vec.into_iter().collect();

    let mut out = String::with_capacity(content.len() / 2 + 64);
    let mut any_token = false;
    let mut prev: Option<usize> = None;
    for &i in &kept_set {
        let gap = match prev {
            Some(p) => Some((p + 1, i)),
            None if i > 0 => Some((0, i)),
            None => None,
        };
        if let Some((s, e)) = gap {
            any_token |= emit_gap(&mut out, &lines, s, e, block_tokens);
        }
        out.push_str(lines[i]);
        if let Some(a) = annotations.get(&i) {
            out.push_str(a);
        }
        out.push('\n');
        prev = Some(i);
    }
    if let Some(p) = prev
        && p + 1 < lines.len()
    {
        any_token |= emit_gap(&mut out, &lines, p + 1, lines.len(), block_tokens);
    }

    if any_token {
        out.push_str(BLOCK_NOTE);
        out.push('\n');
    }
    if out.len() >= content.len() {
        return None;
    }
    log::debug!(
        "[tinyjuice][log] signal kept {} of {} line(s)",
        kept_set.len(),
        lines.len(),
    );
    Some(CompressOutput::lossy(
        out.trim_end().to_string(),
        CompressorKind::Log,
    ))
}

/// Cluster the member lines (selected by `is_member`) by their dedupe template
/// key, keep one exemplar (the first occurrence) per distinct template, and
/// annotate it with `×N (first T1, last T2)` when the cluster has more than one
/// line. With `keep_context`, also retain up to [`CONTEXT_BEFORE`]/
/// [`CONTEXT_AFTER`] lines around the exemplar so causal context survives.
fn collapse_by_template(
    lines: &[&str],
    mut is_member: impl FnMut(usize) -> bool,
    max_templates: usize,
    keep_context: bool,
    keep: &mut BTreeSet<usize>,
    annotations: &mut HashMap<usize, String>,
) {
    let mut order: Vec<String> = Vec::new();
    let mut groups: HashMap<String, Vec<usize>> = HashMap::new();
    for (i, line) in lines.iter().enumerate() {
        if is_member(i) {
            let key = log_template::template_key(line);
            if !groups.contains_key(&key) {
                order.push(key.clone());
            }
            groups.entry(key).or_default().push(i);
        }
    }

    for key in order.into_iter().take(max_templates) {
        let idx = &groups[&key];
        let first = idx[0];
        let last = *idx.last().expect("non-empty group");
        keep.insert(first);
        if idx.len() > 1 {
            let ts = (
                log_template::parse_timestamp(lines[first]),
                log_template::parse_timestamp(lines[last]),
            );
            let ann = match ts {
                (Some(a), Some(b)) => format!("  [×{} first {a}, last {b}]", idx.len()),
                _ => format!("  [×{}]", idx.len()),
            };
            annotations.insert(first, ann);
        }
        if keep_context {
            for j in first.saturating_sub(CONTEXT_BEFORE)..first {
                keep.insert(j);
            }
            let ctx_end = (first + CONTEXT_AFTER).min(lines.len().saturating_sub(1));
            for j in (first + 1)..=ctx_end {
                keep.insert(j);
            }
        }
    }
}

/// Emit a dropped `[start, end)` gap. Gaps of at most [`MIN_GAP`] lines are
/// written verbatim (a marker + token would be larger than the omitted text).
/// Larger gaps get the standard omission marker — carrying a CCR token that
/// makes the region recoverable — optionally preceded by template summaries
/// when the region is dominated by a few repeating templates. Returns whether a
/// retrieval token was emitted.
fn emit_gap(
    out: &mut String,
    lines: &[&str],
    start: usize,
    end: usize,
    block_tokens: bool,
) -> bool {
    let len = end - start;
    if len <= MIN_GAP {
        for line in &lines[start..end] {
            let _ = writeln!(out, "{line}");
        }
        return false;
    }
    let region = &lines[start..end];
    if len >= MIN_REGION_FOR_TEMPLATES
        && let Some(summary) = log_template::summarize_region(region)
    {
        for s in summary {
            let _ = writeln!(out, "{s}");
        }
    }
    let token = block_token(&region.join("\n"), block_tokens);
    let _ = writeln!(out, "[... {len} line(s) omitted ...{token}]");
    !token.is_empty()
}

fn select_first_last(idx: &[usize], cap: usize) -> Vec<usize> {
    if idx.len() <= cap {
        return idx.to_vec();
    }
    if cap == 0 {
        return Vec::new();
    }
    let head = cap.div_ceil(2);
    let tail = cap - head;
    let mut out: std::collections::BTreeSet<usize> = std::collections::BTreeSet::new();
    for &i in idx.iter().take(head) {
        out.insert(i);
    }
    for &i in idx.iter().rev().take(tail) {
        out.insert(i);
    }
    out.into_iter().collect()
}

fn is_summary_line(line: &str) -> bool {
    let l = line.to_ascii_lowercase();
    let l = l.trim();
    l.starts_with("test result:")
        || l.starts_with("error: aborting")
        || l.contains(" passed")
        || l.contains(" failed")
        || l.contains("tests passed")
        || l.contains("tests failed")
        || l.contains("failures:")
        || (l.contains("warning") && l.contains("generated"))
        || l.starts_with("error: could not compile")
        || l.starts_with("build failed")
        || l.starts_with("build succeeded")
        || (l.contains("npm") && l.contains("err"))
}

fn is_stack_frame(line: &str) -> bool {
    let trimmed = line.trim_start();
    if trimmed.is_empty() {
        return false;
    }
    let indented = line.starts_with(' ') || line.starts_with('\t');
    // Rust backtrace frames: "  12: core::panicking::panic"
    let rust_frame = || {
        let digits = trimmed.chars().take_while(char::is_ascii_digit).count();
        digits > 0 && trimmed[digits..].starts_with(": ")
    };
    // Go goroutine frames: "\t/path/file.go:42 +0x1b"
    let go_frame = || trimmed.contains(".go:");
    indented
        && (trimmed.starts_with("at ")
            || trimmed.starts_with("File \"")
            || (trimmed.starts_with('#') && trimmed[1..].starts_with(|c: char| c.is_ascii_digit()))
            || rust_frame()
            || go_frame())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn noisy_log() -> String {
        let mut s = String::new();
        for i in 0..200 {
            let _ = writeln!(s, "   Compiling crate_{i} v0.1.0");
        }
        let _ = writeln!(s, "error[E0382]: borrow of moved value `x`");
        let _ = writeln!(s, "  --> src/main.rs:10:5");
        let _ = writeln!(s, "error: aborting due to previous error");
        let _ = writeln!(s, "test result: FAILED. 3 passed; 1 failed");
        s
    }

    #[test]
    fn signal_keeps_errors_and_summary_drops_noise() {
        let input = noisy_log();
        let out = compress_signal(&input, false).expect("compresses").text;
        assert!(out.contains("error[E0382]"), "{out}");
        assert!(out.contains("error: aborting"), "{out}");
        assert!(out.contains("test result: FAILED"), "{out}");
        assert!(out.len() < input.len());
        assert!(out.contains("omitted"));
    }

    #[test]
    fn recognizes_rust_and_go_stack_frames() {
        // Rust backtrace
        assert!(is_stack_frame("  12: core::panicking::panic"));
        assert!(is_stack_frame("   3: tinyjuice::compress::route"));
        // Go goroutine trace
        assert!(is_stack_frame("\t/srv/app/main.go:42 +0x1b"));
        // JS / Python / gdb still recognized
        assert!(is_stack_frame(
            "    at Object.<anonymous> (/app/index.js:1:1)"
        ));
        assert!(is_stack_frame(
            "  File \"/app/main.py\", line 3, in <module>"
        ));
        assert!(is_stack_frame("  #4 0x0000 in main ()"));
        // Non-frames
        assert!(!is_stack_frame("12: not indented so not a frame"));
        assert!(!is_stack_frame("    ordinary indented prose"));
    }

    #[test]
    fn rust_backtrace_frames_survive_signal_compression() {
        let mut s = String::new();
        for i in 0..200 {
            let _ = writeln!(s, "request handled id={i} status=200");
        }
        let _ = writeln!(s, "thread 'main' panicked at 'boom', src/main.rs:10:5");
        let _ = writeln!(s, "stack backtrace:");
        for i in 0..6 {
            let _ = writeln!(s, "  {i}: some_crate::module::function_{i}");
        }
        let out = compress_signal(&s, false).expect("compresses").text;
        assert!(out.contains("panicked"), "{out}");
        assert!(
            out.contains("some_crate::module::function_2"),
            "backtrace frames dropped:\n{out}"
        );
    }

    #[test]
    fn non_log_data_passes_through() {
        let mut s = String::new();
        for i in 0..400 {
            let _ = writeln!(s, "/var/data/file_{i:04}.bin\t{i}\trwxr-xr-x");
        }
        assert!(compress_signal(&s, false).is_none());
    }

    #[test]
    fn gap_token_retrieves_omitted_lines() {
        use crate::cache;
        let input = noisy_log();
        let out = compress_signal(&input, true).expect("compresses").text;
        let tokens = cache::parse_markers(&out);
        assert!(!tokens.is_empty(), "gap markers should carry tokens: {out}");
        let block = cache::retrieve(&tokens[0]).expect("stored");
        assert!(
            block.contains("Compiling crate_"),
            "gap block should hold the omitted noise: {block}"
        );
    }

    #[test]
    fn distinct_error_templates_all_kept_with_counts() {
        // 150 lines of noise, then three distinct error templates, each repeated.
        let mut s = String::new();
        for i in 0..150 {
            let _ = writeln!(s, "081109 20{i:04} 12 INFO worker: handled request {i} ok");
        }
        for i in 0..5 {
            let _ = writeln!(
                s,
                "081110 00{i:04} 12 ERROR disk: write to /dev/sda{i} failed"
            );
        }
        for i in 0..5 {
            let _ = writeln!(
                s,
                "081110 01{i:04} 12 ERROR net: connection to host_{i} refused"
            );
        }
        for i in 0..5 {
            let _ = writeln!(
                s,
                "081110 02{i:04} 12 ERROR auth: token for user_{i} expired"
            );
        }
        let out = compress_signal(&s, false).expect("compresses").text;
        assert!(out.contains("write to /dev/sda"), "disk error kept: {out}");
        assert!(out.contains("connection to host_"), "net error kept: {out}");
        assert!(out.contains("token for user_"), "auth error kept: {out}");
        // Each distinct template collapses to one exemplar with an ×5 count.
        assert_eq!(out.matches("[×5").count(), 3, "one ×5 per template: {out}");
    }

    #[test]
    fn burst_of_templated_errors_collapses_to_one_exemplar() {
        let mut s = String::new();
        for i in 0..120 {
            let _ = writeln!(s, "12:00:{:02} INFO idle tick {i}", i % 60);
        }
        for i in 0..20 {
            let _ = writeln!(
                s,
                "13:37:{:02} ERROR handler: request blk_{i} timed out after 30s",
                i % 60
            );
        }
        let out = compress_signal(&s, false).expect("compresses").text;
        // The 20-line burst collapses to a single exemplar annotated ×20 with
        // first/last timestamps. A couple of context lines around the exemplar
        // and a gap template summary may echo the phrase, but the 20 verbatim
        // copies are gone.
        assert!(out.contains("[×20 first 13:37:00, last 13:37:19]"), "{out}");
        assert!(
            out.matches("13:37:").count() < 20,
            "burst should not keep 20 verbatim copies: {out}"
        );
    }

    #[test]
    fn context_lines_kept_around_error() {
        let mut s = String::new();
        for i in 0..150 {
            let _ = writeln!(s, "12:00:{:02} INFO steady state {i}", i % 60);
        }
        let _ = writeln!(s, "13:00:00 INFO before-2 opening connection");
        let _ = writeln!(s, "13:00:01 INFO before-1 sending handshake");
        let _ = writeln!(s, "13:00:02 ERROR handshake rejected by peer");
        let _ = writeln!(s, "13:00:03 INFO after-1 closing socket");
        let _ = writeln!(s, "13:00:04 INFO after-2 cleaning up");
        for i in 0..50 {
            let _ = writeln!(s, "13:10:{:02} INFO steady state {i}", i % 60);
        }
        let out = compress_signal(&s, false).expect("compresses").text;
        assert!(out.contains("handshake rejected"), "error kept: {out}");
        assert!(
            out.contains("before-1 sending handshake"),
            "ctx-1 before: {out}"
        );
        assert!(
            out.contains("before-2 opening connection"),
            "ctx-2 before: {out}"
        );
        assert!(out.contains("after-1 closing socket"), "ctx-1 after: {out}");
        assert!(out.contains("after-2 cleaning up"), "ctx-2 after: {out}");
    }

    #[test]
    fn short_gap_emitted_verbatim() {
        // A 1-line gap between two kept errors must stay verbatim, not become a
        // marker + token that is larger than the omitted line.
        let mut s = String::new();
        for i in 0..150 {
            let _ = writeln!(s, "081109 20{i:04} 12 INFO noise line {i}");
        }
        let _ = writeln!(s, "081110 000001 12 ERROR alpha subsystem failed hard");
        let _ = writeln!(s, "081110 000002 12 INFO a single quiet interstitial line");
        let _ = writeln!(s, "081110 000003 12 ERROR beta subsystem failed hard");
        for i in 0..30 {
            let _ = writeln!(s, "081110 01{i:04} 12 INFO tail noise {i}");
        }
        let out = compress_signal(&s, true).expect("compresses").text;
        // The single interstitial line survives verbatim between the two errors,
        // with no omission marker separating them.
        assert!(
            out.contains("a single quiet interstitial line"),
            "1-line gap should be verbatim: {out}"
        );
        let alpha = out.find("alpha subsystem").expect("alpha kept");
        let beta = out.find("beta subsystem").expect("beta kept");
        let between = &out[alpha..beta];
        assert!(
            !between.contains("omitted"),
            "no marker for a 1-line gap: {between}"
        );
    }

    #[test]
    fn warn_exception_deduped_not_kept_verbatim() {
        // The HDFS pathology: many `WARN … Got exception …` lines. They must be
        // treated as warnings and collapsed to a single deduped exemplar, not
        // kept verbatim as distinct errors.
        let mut s = String::new();
        for i in 0..150 {
            let _ = writeln!(s, "081109 20{i:04} 12 INFO worker: processed block {i}");
        }
        for i in 0..30 {
            let _ = writeln!(
                s,
                "081109 21{i:04} {i} WARN dfs.DataNode$DataXceiver: 10.0.0.{i}:50010:Got exception while serving blk_-{i}00 to /10.1.0.{i}:"
            );
        }
        let out = compress_signal(&s, false).expect("compresses").text;
        assert_eq!(
            out.matches("Got exception while serving").count(),
            1,
            "WARN+exception lines must dedupe to one exemplar: {out}"
        );
    }

    #[test]
    fn round_trips_losslessly_with_ccr() {
        // Every omitted region is recoverable through its marker token.
        let mut s = String::new();
        for i in 0..300 {
            let _ = writeln!(
                s,
                "081109 20{i:04} 12 INFO dfs.DataNode: block blk_{i} stored ok"
            );
        }
        let _ = writeln!(s, "081110 000000 12 ERROR fatal corruption detected");
        let out = compress_signal(&s, true).expect("compresses");
        assert!(out.lossy, "signal compression is lossy");
        assert!(out.text.len() < s.len(), "must shrink");
        let tokens = crate::cache::parse_markers(&out.text);
        assert!(!tokens.is_empty(), "recovery tokens present: {}", out.text);
        for t in &tokens {
            assert!(crate::cache::retrieve(t).is_some(), "token {t} recoverable");
        }
    }
}
