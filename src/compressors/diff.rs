//! Unified-diff compressor.
//!
//! Clean-room port of Headroom's `DiffCompressor` (Apache-2.0). Keeps the
//! signal (changed lines, structural headers, hunk headers) and collapses long
//! runs of unchanged context to an anchor + marker. Lockfile/bundle hunks
//! collapse to a one-line `+A/-B` summary. The router offloads the original to
//! CCR, so the dropped context stays recoverable.

use async_trait::async_trait;
use std::fmt::Write as _;

use super::Compressor;
use crate::text::ansi::strip_ansi;
use crate::types::{CompressInput, CompressOptions, CompressOutput, CompressorKind};

/// Context lines kept on each side of a changed run before collapsing.
pub const CONTEXT_ANCHOR: usize = 3;
/// A run of unchanged context longer than this collapses to a marker.
pub const CONTEXT_COLLAPSE_THRESHOLD: usize = 8;

const DEFAULT_NOISY_PATH_SUBSTRINGS: &[&str] = &[
    "cargo.lock",
    "package-lock.json",
    "pnpm-lock.yaml",
    "yarn.lock",
    "composer.lock",
    "poetry.lock",
    "gemfile.lock",
    ".min.js",
    ".min.css",
    ".map",
    "go.sum",
];

/// Controls the low-value diff hunk offload policy.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DiffNoiseOptions {
    /// Case-insensitive path substrings that identify generated or lockfile
    /// hunks whose body can be represented by a reason marker.
    pub noisy_path_substrings: Vec<String>,
    /// Drop hunks whose added/removed lines are identical after ASCII
    /// whitespace stripping. Off by default until fixture coverage is broad.
    pub drop_whitespace_only_hunks: bool,
}

impl Default for DiffNoiseOptions {
    fn default() -> Self {
        Self {
            noisy_path_substrings: DEFAULT_NOISY_PATH_SUBSTRINGS
                .iter()
                .map(|value| value.to_string())
                .collect(),
            drop_whitespace_only_hunks: false,
        }
    }
}

pub struct DiffCompressor;

#[async_trait]
impl Compressor for DiffCompressor {
    fn kind(&self) -> CompressorKind {
        CompressorKind::Diff
    }

    async fn compress(
        &self,
        input: &CompressInput<'_>,
        _opts: &CompressOptions,
    ) -> Option<CompressOutput> {
        compress(input.content)
    }
}

/// Compress a unified diff. Returns `None` when there's nothing structural to
/// work with or compression wouldn't shrink it.
pub fn compress(content: &str) -> Option<CompressOutput> {
    compress_with_options(content, &DiffNoiseOptions::default())
}

/// Compress a unified diff with explicit diff-noise policy.
pub fn compress_with_options(
    content: &str,
    noise_options: &DiffNoiseOptions,
) -> Option<CompressOutput> {
    let stripped = strip_ansi(content);
    let lines: Vec<&str> = stripped.lines().collect();
    if lines.is_empty() {
        return None;
    }

    let mut out = String::with_capacity(stripped.len() / 2 + 64);
    let mut i = 0usize;
    let mut current_file_is_noisy = false;
    let mut saw_hunk = false;

    while i < lines.len() {
        let line = lines[i];

        if line.starts_with("diff --git ") {
            current_file_is_noisy = is_noisy_path(line, noise_options);
            let _ = writeln!(out, "{line}");
            i += 1;
            continue;
        }
        if is_structural(line) {
            saw_hunk |= line.starts_with("@@");
            if line.starts_with("@@") {
                let _ = writeln!(out, "{line}");
                let hunk_start = i + 1;
                let hunk_end = hunk_body_end(&lines, hunk_start);
                let hunk_body = &lines[hunk_start..hunk_end];
                let reason = if current_file_is_noisy {
                    Some("lockfile_or_bundle")
                } else if noise_options.drop_whitespace_only_hunks
                    && hunk_is_whitespace_only(hunk_body)
                {
                    Some("whitespace_only")
                } else {
                    None
                };
                if let Some(reason) = reason {
                    let summary = summarize_hunk_body(hunk_body);
                    let _ = writeln!(
                        out,
                        "[... diff-noise hunk omitted reason={reason} +{}/-{} context={} line(s) ...]",
                        summary.added, summary.removed, summary.context
                    );
                    i = hunk_end;
                    continue;
                }
                i += 1;
                continue;
            }
            let _ = writeln!(out, "{line}");
            i += 1;
            continue;
        }

        if is_context(line) {
            let start = i;
            while i < lines.len() && is_context(lines[i]) {
                i += 1;
            }
            let run = &lines[start..i];
            if run.len() > CONTEXT_COLLAPSE_THRESHOLD {
                for l in &run[..CONTEXT_ANCHOR] {
                    let _ = writeln!(out, "{l}");
                }
                let omitted = run.len() - 2 * CONTEXT_ANCHOR;
                let _ = writeln!(out, "[... {omitted} context line(s) omitted ...]");
                for l in &run[run.len() - CONTEXT_ANCHOR..] {
                    let _ = writeln!(out, "{l}");
                }
            } else {
                for l in run {
                    let _ = writeln!(out, "{l}");
                }
            }
            continue;
        }

        let _ = writeln!(out, "{line}");
        i += 1;
    }

    if !saw_hunk {
        return None;
    }
    if out.len() >= content.len() {
        return None;
    }
    log::debug!(
        "[tokenjuice][diff] {} -> {} bytes ({} input lines)",
        content.len(),
        out.len(),
        lines.len(),
    );
    Some(CompressOutput::lossy(
        out.trim_end().to_string(),
        CompressorKind::Diff,
    ))
}

fn is_structural(line: &str) -> bool {
    line.starts_with("@@")
        || line.starts_with("--- ")
        || line.starts_with("+++ ")
        || line.starts_with("index ")
        || line.starts_with("new file")
        || line.starts_with("deleted file")
        || line.starts_with("rename ")
        || line.starts_with("similarity ")
        || line.starts_with("Binary files")
}

fn is_context(line: &str) -> bool {
    line.starts_with(' ')
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
struct HunkSummary {
    added: usize,
    removed: usize,
    context: usize,
}

fn hunk_body_end(lines: &[&str], start: usize) -> usize {
    let mut i = start;
    while i < lines.len() && !lines[i].starts_with("@@") && !lines[i].starts_with("diff --git ") {
        i += 1;
    }
    i
}

fn summarize_hunk_body(lines: &[&str]) -> HunkSummary {
    let mut summary = HunkSummary::default();
    for &line in lines {
        if line.starts_with('+') && !line.starts_with("+++") {
            summary.added += 1;
        } else if line.starts_with('-') && !line.starts_with("---") {
            summary.removed += 1;
        } else if line.starts_with(' ') {
            summary.context += 1;
        }
    }
    summary
}

fn hunk_is_whitespace_only(lines: &[&str]) -> bool {
    let removed: Vec<String> = lines
        .iter()
        .filter_map(|line| line.strip_prefix('-'))
        .map(strip_ascii_whitespace)
        .collect();
    let added: Vec<String> = lines
        .iter()
        .filter_map(|line| line.strip_prefix('+'))
        .map(strip_ascii_whitespace)
        .collect();
    !removed.is_empty() && removed.len() == added.len() && removed == added
}

fn strip_ascii_whitespace(s: &str) -> String {
    s.chars().filter(|c| !c.is_ascii_whitespace()).collect()
}

fn is_noisy_path(diff_git_line: &str, options: &DiffNoiseOptions) -> bool {
    let l = diff_git_line.to_ascii_lowercase();
    options.noisy_path_substrings.iter().any(|p| {
        let p = p.to_ascii_lowercase();
        !p.is_empty() && l.contains(&p)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn keeps_changed_lines_collapses_context() {
        let mut s = String::from("diff --git a/x.rs b/x.rs\n@@ -1,40 +1,41 @@\n");
        for i in 0..20 {
            let _ = writeln!(s, " context line {i} unchanged here");
        }
        let _ = writeln!(s, "-old changed line");
        let _ = writeln!(s, "+new changed line");
        for i in 0..20 {
            let _ = writeln!(s, " more context {i} unchanged");
        }
        let out = compress(&s).expect("compresses").text;
        assert!(out.contains("-old changed line"), "{out}");
        assert!(out.contains("+new changed line"), "{out}");
        assert!(out.contains("context line(s) omitted"), "{out}");
        assert!(out.contains("@@ -1,40 +1,41 @@"));
        assert!(out.len() < s.len());
    }

    #[test]
    fn summarizes_lockfile_hunk() {
        let mut s = String::from("diff --git a/Cargo.lock b/Cargo.lock\n@@ -1,60 +1,80 @@\n");
        for i in 0..40 {
            let _ = writeln!(s, "+ new dep entry {i}");
        }
        for i in 0..20 {
            let _ = writeln!(s, "- old dep entry {i}");
        }
        let out = compress(&s).expect("compresses").text;
        assert!(
            out.contains("diff-noise hunk omitted reason=lockfile_or_bundle"),
            "{out}"
        );
        assert!(!out.contains("new dep entry 7"), "{out}");
        assert!(out.len() < s.len());
    }

    #[test]
    fn whitespace_only_hunks_are_preserved_by_default() {
        let mut s = String::from("diff --git a/src/lib.rs b/src/lib.rs\n@@ -1,40 +1,40 @@\n");
        for i in 0..20 {
            let _ = writeln!(s, " context before {i}");
        }
        let _ = writeln!(s, "-fn main(){{println!(\"hi\");}}");
        let _ = writeln!(s, "+fn main() {{ println!(\"hi\"); }}");
        for i in 0..20 {
            let _ = writeln!(s, " context after {i}");
        }
        let out = compress(&s).expect("compresses").text;

        assert!(out.contains("-fn main()"), "{out}");
        assert!(out.contains("+fn main()"), "{out}");
        assert!(!out.contains("reason=whitespace_only"), "{out}");
    }

    #[test]
    fn whitespace_only_hunks_drop_when_enabled() {
        let mut s = String::from("diff --git a/src/lib.rs b/src/lib.rs\n@@ -1,60 +1,60 @@\n");
        for i in 0..50 {
            let _ = writeln!(s, " context before {i}");
        }
        let _ = writeln!(s, "-fn main(){{println!(\"hi\");}}");
        let _ = writeln!(s, "+fn main() {{ println!(\"hi\"); }}");
        for i in 0..50 {
            let _ = writeln!(s, " context after {i}");
        }
        let options = DiffNoiseOptions {
            drop_whitespace_only_hunks: true,
            ..Default::default()
        };
        let out = compress_with_options(&s, &options)
            .expect("compresses")
            .text;

        assert!(
            out.contains("diff-noise hunk omitted reason=whitespace_only"),
            "{out}"
        );
        assert!(!out.contains("-fn main()"), "{out}");
        assert!(!out.contains("+fn main()"), "{out}");
    }

    #[test]
    fn semantic_hunks_survive_whitespace_noise_mode() {
        let mut s = String::from("diff --git a/src/lib.rs b/src/lib.rs\n@@ -1,40 +1,40 @@\n");
        for i in 0..20 {
            let _ = writeln!(s, " context before {i}");
        }
        let _ = writeln!(s, "-fn answer() -> i32 {{ 41 }}");
        let _ = writeln!(s, "+fn answer() -> i32 {{ 42 }}");
        for i in 0..20 {
            let _ = writeln!(s, " context after {i}");
        }
        let options = DiffNoiseOptions {
            drop_whitespace_only_hunks: true,
            ..Default::default()
        };
        let out = compress_with_options(&s, &options)
            .expect("compresses")
            .text;

        assert!(out.contains("41"), "{out}");
        assert!(out.contains("42"), "{out}");
        assert!(!out.contains("diff-noise hunk omitted"), "{out}");
    }

    #[test]
    fn non_diff_returns_none() {
        assert!(compress("just some text\nno hunks here").is_none());
    }
}
