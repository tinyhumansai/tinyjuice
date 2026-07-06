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
use crate::cache::CcrStore;
use crate::pipeline::{OffloadOutput, OffloadTransform, PipelineInput, estimate_bloat};
use crate::text::ansi::strip_ansi;
use crate::types::{CompressInput, CompressOptions, CompressOutput, CompressorKind, ContentKind};

/// Context lines kept on each side of a changed run before collapsing.
pub const CONTEXT_ANCHOR: usize = 3;
/// A run of unchanged context longer than this collapses to a marker.
pub const CONTEXT_COLLAPSE_THRESHOLD: usize = 8;

const DEFAULT_LOCKFILE_PATH_PATTERNS: &[&str] = &[
    "cargo.lock",
    "package-lock.json",
    "pnpm-lock.yaml",
    "yarn.lock",
    "composer.lock",
    "poetry.lock",
    "gemfile.lock",
    "go.sum",
];
const DEFAULT_GENERATED_BUNDLE_PATH_PATTERNS: &[&str] = &[".min.js", ".min.css", ".map"];

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
            noisy_path_substrings: DEFAULT_LOCKFILE_PATH_PATTERNS
                .iter()
                .map(|value| value.to_string())
                .chain(
                    DEFAULT_GENERATED_BUNDLE_PATH_PATTERNS
                        .iter()
                        .map(|value| value.to_string()),
                )
                .collect(),
            drop_whitespace_only_hunks: false,
        }
    }
}

/// Typed offload transform for low-value diff hunks.
pub struct DiffNoiseTransform {
    options: DiffNoiseOptions,
}

impl DiffNoiseTransform {
    pub fn new(options: DiffNoiseOptions) -> Self {
        Self { options }
    }

    pub fn options(&self) -> &DiffNoiseOptions {
        &self.options
    }
}

impl Default for DiffNoiseTransform {
    fn default() -> Self {
        Self::new(DiffNoiseOptions::default())
    }
}

impl OffloadTransform for DiffNoiseTransform {
    fn name(&self) -> &'static str {
        "diff_noise"
    }

    fn estimate_bloat(&self, input: &PipelineInput<'_>) -> f32 {
        if input.content_kind != ContentKind::Diff {
            return 0.0;
        }
        f32::from(estimate_bloat(input.content, input.content_kind).score) / 100.0
    }

    fn apply(&self, input: &PipelineInput<'_>, store: &dyn CcrStore) -> Option<OffloadOutput> {
        if input.content_kind != ContentKind::Diff {
            return None;
        }
        let compacted = compress_with_options(input.content, &self.options)?;
        OffloadOutput::from_retained_put(
            compacted.text,
            CompressorKind::Diff,
            store.put(input.original_content),
        )
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
    let mut current_file_noise_reason = None;
    let mut saw_hunk = false;

    while i < lines.len() {
        let line = lines[i];

        if line.starts_with("diff --git ") {
            current_file_noise_reason = diff_noise_reason(line, noise_options);
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
                let reason = if let Some(reason) = current_file_noise_reason {
                    Some(reason)
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

fn diff_noise_reason(diff_git_line: &str, options: &DiffNoiseOptions) -> Option<&'static str> {
    let l = diff_git_line.to_ascii_lowercase();
    let paths = diff_git_paths(diff_git_line);
    for pattern in &options.noisy_path_substrings {
        let pattern = pattern.to_ascii_lowercase();
        if pattern.is_empty()
            || !(l.contains(&pattern) || paths.iter().any(|path| path.ends_with(&pattern)))
        {
            continue;
        }
        if pattern_matches_any_default(&pattern, DEFAULT_LOCKFILE_PATH_PATTERNS) {
            return Some("lockfile");
        }
        if pattern_matches_any_default(&pattern, DEFAULT_GENERATED_BUNDLE_PATH_PATTERNS) {
            return Some("generated_bundle");
        }
        return Some("configured_noisy_path");
    }
    None
}

fn pattern_matches_any_default(pattern: &str, defaults: &[&str]) -> bool {
    defaults.contains(&pattern)
}

fn diff_git_paths(diff_git_line: &str) -> Vec<String> {
    diff_git_line
        .split_whitespace()
        .skip(2)
        .filter_map(|path| path.strip_prefix("a/").or_else(|| path.strip_prefix("b/")))
        .map(|path| path.to_ascii_lowercase())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cache::{CcrStore, MemoryCcrStore};
    use crate::types::ContentKind;

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
            out.contains("diff-noise hunk omitted reason=lockfile"),
            "{out}"
        );
        assert!(!out.contains("new dep entry 7"), "{out}");
        assert!(out.len() < s.len());
    }

    #[test]
    fn empty_noisy_path_patterns_disable_lockfile_omission() {
        let mut s = String::from("diff --git a/Cargo.lock b/Cargo.lock\n@@ -1,60 +1,80 @@\n");
        for i in 0..20 {
            let _ = writeln!(s, " context before {i}");
        }
        for i in 0..20 {
            let _ = writeln!(s, "+ new dep entry {i}");
        }
        for i in 0..20 {
            let _ = writeln!(s, "- old dep entry {i}");
        }
        for i in 0..20 {
            let _ = writeln!(s, " context after {i}");
        }
        let options = DiffNoiseOptions {
            noisy_path_substrings: Vec::new(),
            ..Default::default()
        };

        let out = compress_with_options(&s, &options)
            .expect("compresses")
            .text;

        assert!(!out.contains("diff-noise hunk omitted"), "{out}");
        assert!(out.contains("new dep entry 7"), "{out}");
    }

    #[test]
    fn summarizes_generated_bundle_hunk_with_reason() {
        let mut s =
            String::from("diff --git a/dist/app.min.js b/dist/app.min.js\n@@ -1,80 +1,80 @@\n");
        for i in 0..40 {
            let _ = writeln!(s, "+ minified chunk payload {i}");
        }
        for i in 0..40 {
            let _ = writeln!(s, "- previous minified payload {i}");
        }

        let out = compress(&s).expect("compresses").text;

        assert!(
            out.contains("diff-noise hunk omitted reason=generated_bundle"),
            "{out}"
        );
        assert!(!out.contains("minified chunk payload 7"), "{out}");
    }

    #[test]
    fn custom_noisy_path_patterns_get_configured_reason() {
        let mut s = String::from(
            "diff --git a/fixtures/snapshot.txt b/fixtures/snapshot.txt\n@@ -1,80 +1,80 @@\n",
        );
        for i in 0..40 {
            let _ = writeln!(s, "+ snapshot chunk payload {i}");
        }
        for i in 0..40 {
            let _ = writeln!(s, "- previous snapshot payload {i}");
        }
        let options = DiffNoiseOptions {
            noisy_path_substrings: vec!["snapshot.txt".to_string()],
            ..Default::default()
        };

        let out = compress_with_options(&s, &options)
            .expect("compresses")
            .text;

        assert!(
            out.contains("diff-noise hunk omitted reason=configured_noisy_path"),
            "{out}"
        );
        assert!(!out.contains("snapshot chunk payload 7"), "{out}");
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
    fn diff_noise_transform_requires_retained_ccr() {
        let mut s = String::from("diff --git a/Cargo.lock b/Cargo.lock\n@@ -1,80 +1,80 @@\n");
        for i in 0..80 {
            let _ = writeln!(s, "+ new dep entry {i}");
        }
        for i in 0..80 {
            let _ = writeln!(s, "- old dep entry {i}");
        }
        let input = PipelineInput {
            content: &s,
            original_content: &s,
            content_kind: ContentKind::Diff,
            original_bytes: s.len(),
        };
        let transform = DiffNoiseTransform::default();

        let rejecting_store = MemoryCcrStore::new(1, 1);
        assert!(transform.apply(&input, &rejecting_store).is_none());

        let store = MemoryCcrStore::default();
        let out = transform.apply(&input, &store).expect("retained offload");

        assert_eq!(out.kind(), CompressorKind::Diff);
        assert!(out.text().contains("reason=lockfile"), "{}", out.text());
        assert_eq!(store.get(out.token()).as_deref(), Some(s.as_str()));
    }

    #[test]
    fn diff_noise_transform_skips_non_diff_input() {
        let input = PipelineInput {
            content: "plain text",
            original_content: "plain text",
            content_kind: ContentKind::PlainText,
            original_bytes: "plain text".len(),
        };
        let transform = DiffNoiseTransform::default();
        let store = MemoryCcrStore::default();

        assert_eq!(transform.estimate_bloat(&input), 0.0);
        assert!(transform.apply(&input, &store).is_none());
    }

    #[test]
    fn non_diff_returns_none() {
        assert!(compress("just some text\nno hunks here").is_none());
    }
}
