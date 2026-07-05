//! Unified-diff compressor.
//!
//! Clean-room port of Headroom's `DiffCompressor` (Apache-2.0). Keeps the
//! signal (changed lines, structural headers, hunk headers) and collapses long
//! runs of unchanged context to an anchor + marker. Lockfile/bundle hunks
//! collapse to a one-line `+A/-B` summary. The router offloads the original to
//! CCR, so the dropped context stays recoverable.

use async_trait::async_trait;
use std::fmt::Write as _;

use super::anchors::extract_anchors;
use super::{BLOCK_NOTE, Compressor, block_token};
use crate::text::ansi::strip_ansi;
use crate::types::{CompressInput, CompressOptions, CompressOutput, CompressorKind};

/// Context lines kept on each side of a changed run before collapsing.
pub const CONTEXT_ANCHOR: usize = 3;
/// A run of unchanged context longer than this collapses to a marker.
pub const CONTEXT_COLLAPSE_THRESHOLD: usize = 8;
/// Query words shorter than this are ignored as anchors (too noisy).
pub const MIN_QUERY_WORD: usize = 3;

pub struct DiffCompressor;

#[async_trait]
impl Compressor for DiffCompressor {
    fn kind(&self) -> CompressorKind {
        CompressorKind::Diff
    }

    async fn compress(
        &self,
        input: &CompressInput<'_>,
        opts: &CompressOptions,
    ) -> Option<CompressOutput> {
        compress(input.content, opts.ccr_enabled, input.hint.query.as_deref())
    }
}

/// Compress a unified diff. With `block_tokens`, each omitted block (collapsed
/// context run, summarized lockfile hunk, whitespace-only hunk) is offloaded to
/// CCR and its marker carries a token retrieving exactly that block. `query`
/// yields anchors/words that, when they appear in a context run, prevent that
/// run from being collapsed. Returns `None` when there's nothing structural to
/// work with or compression wouldn't shrink it.
pub fn compress(content: &str, block_tokens: bool, query: Option<&str>) -> Option<CompressOutput> {
    let stripped = strip_ansi(content);
    let lines: Vec<&str> = stripped.lines().collect();
    if lines.is_empty() {
        return None;
    }
    let keepers = query_keepers(query);

    let mut out = String::with_capacity(stripped.len() / 2 + 64);
    let mut i = 0usize;
    let mut current_file_is_noisy = false;
    let mut saw_hunk = false;
    let mut any_token = false;

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
            if line.starts_with("@@") {
                let _ = writeln!(out, "{line}");
                i += 1;
                let (added, removed, consumed) = summarize_hunk_body(&lines[i..]);
                let body = &lines[i..i + consumed];
                let has_keeper = lines_match_keeper(body, &keepers);

                if current_file_is_noisy {
                    let token = block_token(&body.join("\n"), block_tokens);
                    any_token |= !token.is_empty();
                    let _ = writeln!(
                        out,
                        "[... lockfile/bundle hunk: +{added}/-{removed} line(s) omitted ...{token}]"
                    );
                    i += consumed;
                    continue;
                }

                // Whitespace-only hunk: every -/+ pair is identical once all
                // whitespace is collapsed → semantically a no-op. Collapse it
                // like a lockfile hunk, unless a query anchor mentions it.
                if !has_keeper && is_whitespace_noop_hunk(body) {
                    let token = block_token(&body.join("\n"), block_tokens);
                    any_token |= !token.is_empty();
                    let _ = writeln!(
                        out,
                        "[... whitespace-only hunk: {consumed} line(s) omitted (formatting-only) ...{token}]"
                    );
                    i += consumed;
                    continue;
                }

                // Otherwise fall through: let the main loop process the body
                // (changed lines kept, long context runs collapsed).
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
            // A query anchor/word inside the run pins it — never collapse.
            if run.len() > CONTEXT_COLLAPSE_THRESHOLD && !lines_match_keeper(run, &keepers) {
                for l in &run[..CONTEXT_ANCHOR] {
                    let _ = writeln!(out, "{l}");
                }
                let omitted = run.len() - 2 * CONTEXT_ANCHOR;
                let token = block_token(
                    &run[CONTEXT_ANCHOR..run.len() - CONTEXT_ANCHOR].join("\n"),
                    block_tokens,
                );
                any_token |= !token.is_empty();
                let _ = writeln!(out, "[... {omitted} context line(s) omitted ...{token}]");
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
    if any_token {
        out.push_str(BLOCK_NOTE);
        out.push('\n');
    }
    if out.len() >= content.len() {
        return None;
    }
    log::debug!(
        "[tinyjuice][diff] {} -> {} bytes ({} input lines)",
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

/// Build the set of query-derived keeper tokens: exact-match anchors plus query
/// words longer than two chars. Lower-cased for case-insensitive matching.
fn query_keepers(query: Option<&str>) -> Vec<String> {
    let Some(q) = query else {
        return Vec::new();
    };
    let mut set: std::collections::BTreeSet<String> = extract_anchors(q).into_iter().collect();
    for w in q.split(|c: char| !c.is_alphanumeric() && c != '_') {
        if w.chars().count() >= MIN_QUERY_WORD {
            set.insert(w.to_ascii_lowercase());
        }
    }
    set.into_iter().collect()
}

/// True if any line (case-insensitively) contains any keeper token.
fn lines_match_keeper(lines: &[&str], keepers: &[String]) -> bool {
    if keepers.is_empty() {
        return false;
    }
    lines.iter().any(|line| {
        let lower = line.to_ascii_lowercase();
        keepers.iter().any(|k| lower.contains(k))
    })
}

/// True if `body` is a whitespace-only (formatting) hunk: it has at least one
/// changed line, an equal number of removals and additions, and every removal
/// pairs (in order) with an addition that is identical once all whitespace is
/// collapsed. Such a hunk carries no semantic change.
fn is_whitespace_noop_hunk(body: &[&str]) -> bool {
    let mut removes: Vec<String> = Vec::new();
    let mut adds: Vec<String> = Vec::new();
    for &line in body {
        if line.starts_with("+++") || line.starts_with("---") {
            return false;
        }
        if let Some(rest) = line.strip_prefix('+') {
            adds.push(collapse_ws(rest));
        } else if let Some(rest) = line.strip_prefix('-') {
            removes.push(collapse_ws(rest));
        }
        // Context / other lines don't affect the whitespace-only decision.
    }
    if removes.is_empty() || removes.len() != adds.len() {
        return false;
    }
    removes.iter().zip(&adds).all(|(r, a)| r == a)
}

/// Remove every whitespace character from `s` (used to compare lines modulo
/// indentation/spacing).
fn collapse_ws(s: &str) -> String {
    s.chars().filter(|c| !c.is_whitespace()).collect()
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
        let out = compress(&s, false, None).expect("compresses").text;
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
        let out = compress(&s, false, None).expect("compresses").text;
        assert!(out.contains("lockfile/bundle hunk"), "{out}");
        assert!(!out.contains("new dep entry 7"), "{out}");
        assert!(out.len() < s.len());
    }

    #[test]
    fn non_diff_returns_none() {
        assert!(compress("just some text\nno hunks here", false, None).is_none());
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
        let out = compress(&s, false, None).expect("compresses").text;
        assert!(out.contains("+export function newMap7()"), "{out}");
        assert!(!out.contains("lockfile/bundle hunk"), "{out}");
    }

    #[test]
    fn context_block_token_retrieves_omitted_lines() {
        use crate::cache;
        let mut s = String::from("diff --git a/x.rs b/x.rs\n@@ -1,40 +1,41 @@\n");
        for i in 0..20 {
            let _ = writeln!(s, " context line {i} unchanged here");
        }
        let _ = writeln!(s, "-old changed line");
        let _ = writeln!(s, "+new changed line");
        for i in 0..20 {
            let _ = writeln!(s, " more context {i} unchanged");
        }
        let out = compress(&s, true, None).expect("compresses").text;
        let tokens = cache::parse_markers(&out);
        assert!(
            !tokens.is_empty(),
            "context marker should carry a token: {out}"
        );
        let block = cache::retrieve(&tokens[0]).expect("stored");
        assert!(block.contains("context line 10 unchanged here"), "{block}");
        assert!(out.contains("individually retrievable"), "{out}");
    }

    #[test]
    fn whitespace_only_hunk_collapsed_and_recoverable() {
        use crate::cache;
        // Feature 9: every -/+ pair differs only in leading indentation → the
        // hunk is a formatting no-op and collapses to a one-line note, with the
        // full body recoverable via the per-block token.
        let mut s = String::from("diff --git a/src/x.rs b/src/x.rs\n@@ -1,40 +1,40 @@\n");
        for i in 0..20 {
            let _ = writeln!(s, "-    let value_{i} = compute();");
            let _ = writeln!(s, "+        let value_{i} = compute();");
        }
        let out = compress(&s, true, None).expect("compresses").text;
        assert!(out.contains("whitespace-only hunk"), "{out}");
        assert!(
            !out.contains("let value_5"),
            "body should be collapsed:\n{out}"
        );
        assert!(out.len() < s.len());

        let tokens = cache::parse_markers(&out);
        assert!(
            !tokens.is_empty(),
            "whitespace hunk should carry a token: {out}"
        );
        let block = cache::retrieve(&tokens[0]).expect("stored");
        assert!(block.contains("let value_5 = compute();"), "{block}");
    }

    #[test]
    fn real_change_hunk_not_treated_as_whitespace_noop() {
        // A hunk that genuinely changes a token must NOT collapse as whitespace.
        let mut s = String::from("diff --git a/src/x.rs b/src/x.rs\n@@ -1,3 +1,3 @@\n");
        let _ = writeln!(s, "-let total = old_value + 1;");
        let _ = writeln!(s, "+let total = new_value + 1;");
        for i in 0..20 {
            let _ = writeln!(s, " context tail {i}");
        }
        let out = compress(&s, false, None).expect("compresses").text;
        assert!(!out.contains("whitespace-only hunk"), "{out}");
        assert!(out.contains("+let total = new_value + 1;"), "{out}");
    }

    #[test]
    fn query_word_prevents_context_collapse() {
        // Feature 10: a long context run mentioning a query word is pinned; a
        // second file's plain context run still collapses so the output shrinks.
        let mut s = String::new();
        let _ = writeln!(s, "diff --git a/auth.rs b/auth.rs");
        let _ = writeln!(s, "@@ -1,25 +1,26 @@");
        for i in 0..20 {
            if i == 10 {
                let _ = writeln!(s, " let token = AUTHENTICATION_TOKEN;");
            } else {
                let _ = writeln!(s, " auth context {i}");
            }
        }
        let _ = writeln!(s, "-old auth line");
        let _ = writeln!(s, "+new auth line");
        let _ = writeln!(s, "diff --git a/other.rs b/other.rs");
        let _ = writeln!(s, "@@ -1,22 +1,23 @@");
        for i in 0..20 {
            let _ = writeln!(s, " other context {i}");
        }
        let _ = writeln!(s, "-old other line");
        let _ = writeln!(s, "+new other line");

        // With the query: the auth run is preserved verbatim.
        let with_q = compress(&s, false, Some("why is AUTHENTICATION_TOKEN failing"))
            .expect("compresses")
            .text;
        assert!(with_q.contains("AUTHENTICATION_TOKEN"), "{with_q}");
        assert!(
            with_q.contains("auth context 15"),
            "pinned run must survive:\n{with_q}"
        );
        // The unrelated other-file run still collapses, so output shrank.
        assert!(with_q.contains("context line(s) omitted"), "{with_q}");

        // Without the query: the same auth run collapses.
        let no_q = compress(&s, false, None).expect("compresses").text;
        assert!(
            !no_q.contains("auth context 15"),
            "run should collapse:\n{no_q}"
        );
    }
}
