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
use once_cell::sync::Lazy;
use std::collections::HashSet;
use std::fmt::Write as _;

use super::Compressor;
use super::signals::{Severity, severity};
use crate::reduce::reduce_execution_with_rules;
use crate::rules::load_builtin_rules;
use crate::types::{
    CompiledRule, CompressInput, CompressOptions, CompressOutput, CompressorKind, ReduceOptions,
    ToolExecutionInput,
};

pub const MAX_ERRORS: usize = 10;
pub const MAX_WARNINGS: usize = 5;
pub const MAX_STACK_TRACES: usize = 3;
pub const STACK_TRACE_MAX_LINES: usize = 20;
pub const MAX_TOTAL_LINES: usize = 100;
pub const TEMPLATE_MIN_RUN: usize = 8;
pub const TEMPLATE_MIN_CONSTANT_TOKENS: usize = 3;

static BUILTIN_RULES: Lazy<Vec<CompiledRule>> = Lazy::new(load_builtin_rules);

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
            compress_templates(input.content).or_else(|| compress_signal(input.content))
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
    let result = reduce_execution_with_rules(exec, &BUILTIN_RULES, &reduce_opts);
    if result.inline_text.len() >= input.content.len() {
        return None;
    }
    let rule_label = result
        .classification
        .matched_reducer
        .unwrap_or(result.classification.family);
    log::debug!(
        "[tokenjuice][log] command rule={} kind={} {} -> {} bytes",
        rule_label,
        kind.as_str(),
        input.content.len(),
        result.inline_text.len()
    );
    Some(CompressOutput::lossy(result.inline_text, kind))
}

/// Signal-based log compression for non-command blobs detected as logs.
pub fn compress_signal(content: &str) -> Option<CompressOutput> {
    let lines: Vec<&str> = content.lines().collect();
    if lines.len() <= MAX_TOTAL_LINES {
        return None;
    }

    let mut keep: std::collections::BTreeSet<usize> = std::collections::BTreeSet::new();

    for (i, line) in lines.iter().enumerate() {
        if is_summary_line(line) {
            keep.insert(i);
        }
    }

    let error_idx: Vec<usize> = lines
        .iter()
        .enumerate()
        .filter(|(_, l)| severity(l) == Severity::Error)
        .map(|(i, _)| i)
        .collect();
    for &i in select_first_last(&error_idx, MAX_ERRORS).iter() {
        keep.insert(i);
    }

    let mut seen_warn: HashSet<String> = HashSet::new();
    let mut warn_kept = 0usize;
    for (i, line) in lines.iter().enumerate() {
        if warn_kept >= MAX_WARNINGS {
            break;
        }
        if severity(line) == Severity::Warning {
            let norm = normalize_for_dedupe(line);
            if seen_warn.insert(norm) {
                keep.insert(i);
                warn_kept += 1;
            }
        }
    }

    let mut traces_kept = 0usize;
    let mut i = 0usize;
    while i < lines.len() && traces_kept < MAX_STACK_TRACES {
        if is_stack_frame(lines[i]) {
            let start = i;
            let mut taken = 0usize;
            while i < lines.len() && is_stack_frame(lines[i]) {
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
    let kept_set: std::collections::BTreeSet<usize> = kept_vec.into_iter().collect();

    let mut out = String::with_capacity(content.len() / 2 + 64);
    let mut prev: Option<usize> = None;
    for &i in &kept_set {
        if let Some(p) = prev {
            let gap = i - p - 1;
            if gap > 0 {
                let _ = writeln!(out, "[... {gap} line(s) omitted ...]");
            }
        } else if i > 0 {
            let _ = writeln!(out, "[... {i} line(s) omitted ...]");
        }
        let _ = writeln!(out, "{}", lines[i]);
        prev = Some(i);
    }
    if let Some(p) = prev {
        let tail = lines.len().saturating_sub(p + 1);
        if tail > 0 {
            let _ = writeln!(out, "[... {tail} line(s) omitted ...]");
        }
    }

    if out.len() >= content.len() {
        return None;
    }
    log::debug!(
        "[tokenjuice][log] signal kept {} of {} line(s)",
        kept_set.len(),
        lines.len(),
    );
    Some(CompressOutput::lossy(
        out.trim_end().to_string(),
        CompressorKind::Log,
    ))
}

/// Lossless reformat for repetitive non-command logs. Consecutive runs of the
/// same line template are represented as one template plus captured variants.
pub fn compress_templates(content: &str) -> Option<CompressOutput> {
    let lines: Vec<&str> = content.lines().collect();
    if lines.len() < TEMPLATE_MIN_RUN {
        return None;
    }

    let mut out = String::with_capacity(content.len() / 2 + 128);
    let mut i = 0usize;
    let mut emitted_block = false;

    while i < lines.len() {
        let Some(first) = line_template(lines[i]) else {
            let _ = writeln!(out, "{}", lines[i]);
            i += 1;
            continue;
        };

        let start = i;
        let mut captures = vec![first.captures];
        i += 1;
        while i < lines.len() {
            let Some(next) = line_template(lines[i]) else {
                break;
            };
            if next.template != first.template || next.captures.len() != captures[0].len() {
                break;
            }
            captures.push(next.captures);
            i += 1;
        }

        if captures.len() >= TEMPLATE_MIN_RUN {
            write_template_block(&mut out, &first.template, &captures);
            emitted_block = true;
        } else {
            for line in &lines[start..i] {
                let _ = writeln!(out, "{line}");
            }
        }
    }

    let out = out.trim_end().to_string();
    if !emitted_block || out.len() >= content.len() {
        return None;
    }
    log::debug!(
        "[tokenjuice][log] template reformat {} -> {} bytes",
        content.len(),
        out.len(),
    );
    Some(CompressOutput::reformatted(out, CompressorKind::Log))
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct LineTemplate {
    template: String,
    captures: Vec<String>,
}

fn line_template(line: &str) -> Option<LineTemplate> {
    let mut template = String::with_capacity(line.len());
    let mut captures = Vec::new();
    let mut chars = line.char_indices().peekable();

    while let Some((idx, ch)) = chars.next() {
        if ch == '0' && chars.peek().is_some_and(|(_, next)| *next == 'x') {
            let start = idx;
            let mut end = idx + ch.len_utf8();
            if let Some((x_idx, x_ch)) = chars.next() {
                end = x_idx + x_ch.len_utf8();
            }
            while let Some((next_idx, next_ch)) = chars.peek().copied() {
                if next_ch.is_ascii_hexdigit() {
                    chars.next();
                    end = next_idx + next_ch.len_utf8();
                } else {
                    break;
                }
            }
            template.push_str("{}");
            captures.push(line[start..end].to_string());
        } else if ch.is_ascii_digit() {
            let start = idx;
            let mut end = idx + ch.len_utf8();
            while let Some((next_idx, next_ch)) = chars.peek().copied() {
                if next_ch.is_ascii_digit() {
                    chars.next();
                    end = next_idx + next_ch.len_utf8();
                } else {
                    break;
                }
            }
            template.push_str("{}");
            captures.push(line[start..end].to_string());
        } else {
            template.push(ch);
        }
    }

    if captures.len() < 2
        || template.trim().len() < 12
        || template_constant_tokens(&template) < TEMPLATE_MIN_CONSTANT_TOKENS
    {
        return None;
    }
    Some(LineTemplate { template, captures })
}

fn template_constant_tokens(template: &str) -> usize {
    template
        .split(|ch: char| !ch.is_ascii_alphanumeric())
        .filter(|token| !token.is_empty())
        .count()
}

fn write_template_block(out: &mut String, template: &str, captures: &[Vec<String>]) {
    let fields = captures.first().map_or(0, Vec::len);
    let _ = writeln!(
        out,
        "[TOKENJUICE LOG TEMPLATE run={} fields={fields}]",
        captures.len()
    );
    let _ = writeln!(out, "{template}");
    for row in captures {
        let rendered = row
            .iter()
            .map(|capture| escape_capture(capture))
            .collect::<Vec<_>>()
            .join("\t");
        let _ = writeln!(out, "{rendered}");
    }
    let _ = writeln!(out, "[/TOKENJUICE LOG TEMPLATE]");
}

fn escape_capture(value: &str) -> String {
    let mut out = String::with_capacity(value.len());
    for ch in value.chars() {
        match ch {
            '\\' => out.push_str("\\\\"),
            '\t' => out.push_str("\\t"),
            '\r' => out.push_str("\\r"),
            _ => out.push(ch),
        }
    }
    out
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
    indented
        && (trimmed.starts_with("at ")
            || trimmed.starts_with("File \"")
            || (trimmed.starts_with('#') && trimmed[1..].starts_with(|c: char| c.is_ascii_digit())))
}

fn normalize_for_dedupe(line: &str) -> String {
    line.chars()
        .filter(|c| !c.is_ascii_digit())
        .collect::<String>()
        .to_ascii_lowercase()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{CompressOptions, ContentHint, ContentKind};

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

    fn repetitive_template_log() -> String {
        let mut s = String::new();
        for i in 0..80 {
            let _ = writeln!(
                s,
                "2026-07-05T12:{:02}:00Z worker-{i} processed item id={} shard={}",
                i % 60,
                10_000 + i,
                i % 8
            );
        }
        s
    }

    fn reconstruct_template_reformat(output: &str) -> String {
        let mut out = String::new();
        let lines: Vec<&str> = output.lines().collect();
        let mut i = 0usize;
        while i < lines.len() {
            if lines[i].starts_with("[TOKENJUICE LOG TEMPLATE ") {
                let template = lines[i + 1];
                i += 2;
                while i < lines.len() && lines[i] != "[/TOKENJUICE LOG TEMPLATE]" {
                    let captures = lines[i]
                        .split('\t')
                        .map(unescape_capture_for_test)
                        .collect::<Vec<_>>();
                    let _ = writeln!(out, "{}", apply_template_for_test(template, &captures));
                    i += 1;
                }
                i += 1;
            } else {
                let _ = writeln!(out, "{}", lines[i]);
                i += 1;
            }
        }
        out
    }

    fn unescape_capture_for_test(value: &str) -> String {
        let mut out = String::new();
        let mut chars = value.chars();
        while let Some(ch) = chars.next() {
            if ch == '\\' {
                match chars.next() {
                    Some('\\') => out.push('\\'),
                    Some('t') => out.push('\t'),
                    Some('r') => out.push('\r'),
                    Some(other) => {
                        out.push('\\');
                        out.push(other);
                    }
                    None => out.push('\\'),
                }
            } else {
                out.push(ch);
            }
        }
        out
    }

    fn apply_template_for_test(template: &str, captures: &[String]) -> String {
        let mut out = String::new();
        let mut rest = template;
        for capture in captures {
            let Some((head, tail)) = rest.split_once("{}") else {
                break;
            };
            out.push_str(head);
            out.push_str(capture);
            rest = tail;
        }
        out.push_str(rest);
        out
    }

    #[test]
    fn template_reformat_is_lossless_and_smaller() {
        let input = repetitive_template_log();
        let out = compress_templates(&input).expect("template reformat");

        assert!(!out.lossy);
        assert!(out.text.contains("[TOKENJUICE LOG TEMPLATE run=80"));
        assert!(out.text.len() < input.len(), "{}", out.text);
        assert_eq!(
            reconstruct_template_reformat(&out.text).trim_end(),
            input.trim_end()
        );
    }

    #[tokio::test]
    async fn template_reformat_routes_without_ccr() {
        let input = repetitive_template_log();
        let hint = ContentHint {
            explicit: Some(ContentKind::Log),
            ..Default::default()
        };
        let opts = CompressOptions {
            min_bytes_to_compress: 64,
            ccr_min_tokens: usize::MAX,
            ..Default::default()
        };

        let out = crate::compress_content(&input, Some(hint), &opts).await;

        assert!(out.applied);
        assert!(!out.lossy);
        assert!(out.ccr_token.is_none());
        assert!(out.text.contains("[TOKENJUICE LOG TEMPLATE"));
    }

    #[test]
    fn template_reformat_declines_without_dynamic_fields() {
        let mut input = String::new();
        for _ in 0..40 {
            let _ = writeln!(input, "plain repeated line without dynamic fields");
        }

        assert!(compress_templates(&input).is_none());
    }

    #[test]
    fn template_reformat_declines_low_context_dynamic_lines() {
        let mut input = String::new();
        for i in 0..40 {
            let _ = writeln!(input, "id={} value={}", 10_000 + i, 20_000 + i);
        }

        assert!(compress_templates(&input).is_none());
    }

    #[test]
    fn signal_keeps_errors_and_summary_drops_noise() {
        let input = noisy_log();
        let out = compress_signal(&input).expect("compresses").text;
        assert!(out.contains("error[E0382]"), "{out}");
        assert!(out.contains("error: aborting"), "{out}");
        assert!(out.contains("test result: FAILED"), "{out}");
        assert!(out.len() < input.len());
        assert!(out.contains("omitted"));
    }

    #[test]
    fn non_log_data_passes_through() {
        let mut s = String::new();
        for i in 0..400 {
            let _ = writeln!(s, "/var/data/file_{i:04}.bin\t{i}\trwxr-xr-x");
        }
        assert!(compress_signal(&s).is_none());
    }
}
