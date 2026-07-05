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
            current_file_is_noisy = is_noisy_path(line);
            let _ = writeln!(out, "{line}");
            i += 1;
            continue;
        }
        if is_structural(line) {
            saw_hunk |= line.starts_with("@@");
            if current_file_is_noisy && line.starts_with("@@") {
                let _ = writeln!(out, "{line}");
                i += 1;
                let (added, removed, consumed) = summarize_hunk_body(&lines[i..]);
                let _ = writeln!(
                    out,
                    "[... lockfile/bundle hunk: +{added}/-{removed} line(s) omitted ...]"
                );
                i += consumed;
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

fn summarize_hunk_body(lines: &[&str]) -> (usize, usize, usize) {
    let mut added = 0usize;
    let mut removed = 0usize;
    let mut n = 0usize;
    for &line in lines {
        if line.starts_with("@@") || line.starts_with("diff --git ") {
            break;
        }
        if line.starts_with('+') && !line.starts_with("+++") {
            added += 1;
        } else if line.starts_with('-') && !line.starts_with("---") {
            removed += 1;
        }
        n += 1;
    }
    (added, removed, n)
}

fn is_noisy_path(diff_git_line: &str) -> bool {
    // Match on the parsed file basenames, not the raw line: a substring
    // check would flag real source files like `user.mapper.ts` (".map") or
    // `heat.map.config.js` as bundle noise and drop their hunks.
    const NOISY_BASENAMES: &[&str] = &[
        "cargo.lock",
        "package-lock.json",
        "pnpm-lock.yaml",
        "yarn.lock",
        "composer.lock",
        "poetry.lock",
        "gemfile.lock",
        "go.sum",
    ];
    const NOISY_SUFFIXES: &[&str] = &[".min.js", ".min.css", ".map"];

    let Some(rest) = diff_git_line.strip_prefix("diff --git ") else {
        return false;
    };
    rest.split_whitespace().any(|tok| {
        let path = tok
            .strip_prefix("a/")
            .or_else(|| tok.strip_prefix("b/"))
            .unwrap_or(tok)
            .to_ascii_lowercase();
        let basename = path.rsplit('/').next().unwrap_or(path.as_str());
        NOISY_BASENAMES.contains(&basename) || NOISY_SUFFIXES.iter().any(|s| basename.ends_with(s))
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
        assert!(out.contains("lockfile/bundle hunk"), "{out}");
        assert!(!out.contains("new dep entry 7"), "{out}");
        assert!(out.len() < s.len());
    }

    #[test]
    fn non_diff_returns_none() {
        assert!(compress("just some text\nno hunks here").is_none());
    }

    #[test]
    fn source_files_with_map_in_name_are_not_noisy() {
        for line in [
            "diff --git a/src/user.mapper.ts b/src/user.mapper.ts",
            "diff --git a/heat.map.config.js b/heat.map.config.js",
            "diff --git a/src/routes/url.mapping.rs b/src/routes/url.mapping.rs",
            "diff --git a/minutes.md b/minutes.md",
        ] {
            assert!(!is_noisy_path(line), "misclassified as noisy: {line}");
        }
    }

    #[test]
    fn lockfiles_and_artifacts_are_noisy() {
        for line in [
            "diff --git a/Cargo.lock b/Cargo.lock",
            "diff --git a/vendor/pkg/go.sum b/vendor/pkg/go.sum",
            "diff --git a/dist/app.min.js b/dist/app.min.js",
            "diff --git a/dist/bundle.js.map b/dist/bundle.js.map",
            "diff --git a/web/package-lock.json b/web/package-lock.json",
        ] {
            assert!(is_noisy_path(line), "should be noisy: {line}");
        }
    }

    #[test]
    fn source_file_hunks_survive_even_with_mapper_name() {
        let mut s = String::from(
            "diff --git a/src/user.mapper.ts b/src/user.mapper.ts\n@@ -1,40 +1,42 @@\n",
        );
        for i in 0..20 {
            let _ = writeln!(s, " context line {i} unchanged here");
        }
        let _ = writeln!(s, "-export function oldMap() {{}}");
        let _ = writeln!(s, "+export function newMap7() {{}}");
        for i in 0..20 {
            let _ = writeln!(s, " more context {i} unchanged");
        }
        let out = compress(&s).expect("compresses").text;
        assert!(out.contains("+export function newMap7()"), "{out}");
        assert!(!out.contains("lockfile/bundle hunk"), "{out}");
    }
}
