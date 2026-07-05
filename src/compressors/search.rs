//! Search-results compressor (relevance ranking).
//!
//! grep / ripgrep output is `path:line:body` matches. For very large result
//! sets the compressor groups matches by file, ranks them, keeps the top-K per
//! file plus a `[+N more in <file>]` tally, and (via the router) offloads the
//! full result set to CCR so the complete list is one `retrieve` away.
//!
//! Ranking: when the caller supplies a `query` in the [`ContentHint`], matches
//! are scored by query-term density in the body; otherwise by body length /
//! uniqueness (longer, more-distinctive lines first), with importance signals
//! (error/TODO) always boosted. Group/file order is preserved (first-seen).
//!
//! NOTE: this compressor is gated by `opts.search_enabled` in the router. It is
//! a behaviour change from the historical "never compact grep" stance — kept
//! lossless by the CCR offload — and is enabled by default per project decision.

use async_trait::async_trait;
use std::borrow::Cow;
use std::fmt::Write as _;

use super::Compressor;
use super::signals::line_score;
use crate::detect::parse_search_line;
use crate::relevance::tokenize;
use crate::types::LineRange;
use crate::types::{CompressInput, CompressOptions, CompressOutput, CompressorKind};

/// Only compress result sets with more than this many matching lines.
pub const MIN_MATCHES: usize = 40;
/// Matches kept per file before the "+N more" tally.
pub const TOP_K_PER_FILE: usize = 5;
/// Default line context on each side for ranked search-read snippets.
pub const DEFAULT_SNIPPET_CONTEXT: usize = 2;

pub struct SearchCompressor;

#[async_trait]
impl Compressor for SearchCompressor {
    fn kind(&self) -> CompressorKind {
        CompressorKind::Search
    }

    async fn compress(
        &self,
        input: &CompressInput<'_>,
        _opts: &CompressOptions,
    ) -> Option<CompressOutput> {
        compress(input.content, input.hint.query.as_deref())
    }
}

struct Match<'a> {
    line_no: u64,
    body: &'a str,
    score: f32,
    raw: &'a str,
}

/// Compress search output. `query` (when known) ranks matches by term density.
pub fn compress(content: &str, query: Option<&str>) -> Option<CompressOutput> {
    // Preserve any non-match preamble/summary lines (e.g. "80 match(es)") and
    // group match lines by file in first-seen order.
    let mut preamble: Vec<&str> = Vec::new();
    let mut files: Vec<(&str, Vec<Match<'_>>)> = Vec::new();
    let mut match_count = 0usize;

    let query_terms: Vec<String> = query
        .map(|q| {
            q.split_whitespace()
                .map(|t| t.to_ascii_lowercase())
                .filter(|t| t.len() >= 2)
                .collect()
        })
        .unwrap_or_default();

    for line in content.lines() {
        match parse_search_line(line) {
            Some((path, line_no, body)) => {
                match_count += 1;
                let score = score_match(body, &query_terms);
                let m = Match {
                    line_no,
                    body,
                    score,
                    raw: line,
                };
                if let Some((_, v)) = files.iter_mut().find(|(p, _)| *p == path) {
                    v.push(m);
                } else {
                    files.push((path, vec![m]));
                }
            }
            None => {
                if !line.trim().is_empty() && files.is_empty() {
                    // Only keep preamble that appears before any match.
                    preamble.push(line);
                }
            }
        }
    }

    if match_count < MIN_MATCHES || files.is_empty() {
        return None;
    }

    let mut out = String::with_capacity(content.len() / 2 + 64);
    for line in &preamble {
        let _ = writeln!(out, "{line}");
    }
    let _ = writeln!(
        out,
        "[search: {} match(es) across {} file(s) · top {} per file · full set via retrieve footer]",
        match_count,
        files.len(),
        TOP_K_PER_FILE
    );

    let mut omitted_total = 0usize;
    let mut files_with_omissions = 0usize;
    for (path, mut matches) in files {
        let total = matches.len();
        if total <= TOP_K_PER_FILE {
            // Keep all in original (line-number) order.
            matches.sort_by_key(|m| m.line_no);
            for m in &matches {
                let _ = writeln!(out, "{}", m.raw);
            }
            continue;
        }
        // Rank by score (desc), keep top-K, then re-sort kept by line number so
        // the output reads top-to-bottom within the file.
        matches.sort_by(|a, b| {
            b.score
                .partial_cmp(&a.score)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        let mut kept: Vec<&Match<'_>> = matches.iter().take(TOP_K_PER_FILE).collect();
        kept.sort_by_key(|m| m.line_no);
        for m in &kept {
            let _ = writeln!(out, "{}:{}:{}", path, m.line_no, m.body);
        }
        let _ = writeln!(
            out,
            "[+{} more match(es) in {path}]",
            total - TOP_K_PER_FILE
        );
        omitted_total += total - TOP_K_PER_FILE;
        files_with_omissions += 1;
    }
    if omitted_total > 0 {
        let _ = writeln!(
            out,
            "[search omitted: {omitted_total} match(es) not shown across {files_with_omissions} file(s)]"
        );
    }

    let out = out.trim_end().to_string();
    if out.len() >= content.len() {
        return None;
    }
    log::debug!(
        "[tokenjuice][search] {} matches -> {} bytes (from {} bytes)",
        match_count,
        out.len(),
        content.len()
    );
    Some(CompressOutput::lossy(out, CompressorKind::Search))
}

/// Score a match body. With query terms, density of those terms dominates;
/// otherwise distinctiveness (length) with an importance bump for error/TODO.
fn score_match(body: &str, query_terms: &[String]) -> f32 {
    let importance = line_score(body);
    if query_terms.is_empty() {
        // No query: favour longer, more-distinctive lines, plus importance.
        let len_score = (body.trim().len() as f32 / 80.0).min(1.0);
        return importance.max(0.2 + 0.8 * len_score);
    }
    let lower = body.to_ascii_lowercase();
    let hits = query_terms.iter().filter(|t| lower.contains(*t)).count();
    let density = hits as f32 / query_terms.len() as f32;
    importance.max(density)
}

/// Host-provided search-read query metadata. TinyJuice only scores already
/// discovered matches; filesystem traversal stays in the host.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct SearchReadQuery {
    pub literal: Option<String>,
    pub regex: Option<String>,
    pub symbols: Vec<String>,
    pub file_kinds: Vec<String>,
}

/// One candidate file already discovered by a host search adapter.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SearchReadCandidate<'a> {
    pub path: Cow<'a, str>,
    pub matched_lines: Vec<SearchReadLine<'a>>,
    pub imports: Vec<Cow<'a, str>>,
    pub exports: Vec<Cow<'a, str>>,
    pub generated: bool,
    pub vendor: bool,
    pub max_line: usize,
}

impl<'a> SearchReadCandidate<'a> {
    pub fn new(path: impl Into<Cow<'a, str>>) -> Self {
        Self {
            path: path.into(),
            matched_lines: Vec::new(),
            imports: Vec::new(),
            exports: Vec::new(),
            generated: false,
            vendor: false,
            max_line: 0,
        }
    }
}

/// One host-provided match line.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SearchReadLine<'a> {
    pub line_number: usize,
    pub text: Cow<'a, str>,
}

impl<'a> SearchReadLine<'a> {
    pub fn new(line_number: usize, text: impl Into<Cow<'a, str>>) -> Self {
        Self {
            line_number,
            text: text.into(),
        }
    }
}

/// Bounded snippet-window selection for a candidate file.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SnippetWindowSelection {
    pub windows: Vec<LineRange>,
    pub omitted_matches: usize,
}

/// Score a candidate file for a ranked search-read adapter. Exact symbol
/// matches dominate path matches, which dominate match density.
pub fn rank_search_read_candidate(
    candidate: &SearchReadCandidate<'_>,
    query: &SearchReadQuery,
) -> f32 {
    let exact_symbol_match = query
        .symbols
        .iter()
        .filter(|symbol| !symbol.trim().is_empty())
        .any(|symbol| candidate_has_exact_symbol(candidate, symbol));
    let path_match = query_terms(query)
        .iter()
        .any(|term| candidate.path.to_ascii_lowercase().contains(term));
    let regex_density = regex_match_density(candidate, query);
    let import_export_match = query
        .symbols
        .iter()
        .any(|symbol| import_export_contains(candidate, symbol));

    let generated_penalty = if candidate.generated || looks_generated_path(&candidate.path) {
        4.0
    } else {
        0.0
    };
    let vendor_penalty = if candidate.vendor || looks_vendor_path(&candidate.path) {
        3.0
    } else {
        0.0
    };

    (if exact_symbol_match { 8.0 } else { 0.0 })
        + (if path_match { 4.0 } else { 0.0 })
        + regex_density * 3.0
        + (if import_export_match { 2.0 } else { 0.0 })
        - generated_penalty
        - vendor_penalty
}

/// Select and merge snippet windows around match lines. The output is bounded
/// by `max_snippets`, with omitted match count made explicit.
pub fn select_snippet_windows(
    match_lines: &[usize],
    context: usize,
    max_line: usize,
    max_snippets: usize,
) -> SnippetWindowSelection {
    if max_snippets == 0 || match_lines.is_empty() || max_line == 0 {
        return SnippetWindowSelection {
            windows: Vec::new(),
            omitted_matches: match_lines.len(),
        };
    }

    let mut lines: Vec<usize> = match_lines
        .iter()
        .copied()
        .filter(|line| *line > 0)
        .collect();
    lines.sort_unstable();
    lines.dedup();

    let mut merged: Vec<LineRange> = Vec::new();
    for line in lines {
        let range = LineRange::new(
            line.saturating_sub(context).max(1),
            (line + context).min(max_line),
        );
        if let Some(last) = merged.last_mut()
            && range.start <= last.end.saturating_add(1)
        {
            last.end = last.end.max(range.end);
            continue;
        }
        merged.push(range);
    }

    let omitted_matches = if merged.len() > max_snippets {
        let kept_end = merged[max_snippets - 1].end;
        match_lines.iter().filter(|line| **line > kept_end).count()
    } else {
        0
    };
    merged.truncate(max_snippets);

    SnippetWindowSelection {
        windows: merged,
        omitted_matches,
    }
}

fn candidate_has_exact_symbol(candidate: &SearchReadCandidate<'_>, symbol: &str) -> bool {
    candidate
        .matched_lines
        .iter()
        .any(|line| contains_exact_token(&line.text, symbol))
        || candidate
            .exports
            .iter()
            .any(|export| export.as_ref() == symbol)
}

fn import_export_contains(candidate: &SearchReadCandidate<'_>, symbol: &str) -> bool {
    candidate
        .imports
        .iter()
        .chain(candidate.exports.iter())
        .any(|entry| entry.contains(symbol))
}

fn regex_match_density(candidate: &SearchReadCandidate<'_>, query: &SearchReadQuery) -> f32 {
    if candidate.matched_lines.is_empty() {
        return 0.0;
    }
    let terms = query_terms(query);
    if terms.is_empty() && query.regex.is_none() {
        return 0.0;
    }
    let hit_count = candidate
        .matched_lines
        .iter()
        .filter(|line| {
            let lower = line.text.to_ascii_lowercase();
            terms.iter().any(|term| lower.contains(term))
                || query
                    .regex
                    .as_ref()
                    .is_some_and(|regex| lower.contains(&regex.to_ascii_lowercase()))
        })
        .count();
    hit_count as f32 / candidate.matched_lines.len() as f32
}

fn query_terms(query: &SearchReadQuery) -> Vec<String> {
    query
        .literal
        .iter()
        .chain(query.symbols.iter())
        .flat_map(|value| tokenize(value))
        .collect()
}

fn contains_exact_token(text: &str, needle: &str) -> bool {
    tokenize(text)
        .iter()
        .any(|token| token == &needle.to_ascii_lowercase())
}

fn looks_vendor_path(path: &str) -> bool {
    let lower = path.to_ascii_lowercase();
    lower.contains("/node_modules/")
        || lower.contains("/vendor/")
        || lower.contains("/third_party/")
        || lower.starts_with("vendor/")
        || lower.starts_with("third_party/")
}

fn looks_generated_path(path: &str) -> bool {
    let lower = path.to_ascii_lowercase();
    lower.contains("/target/")
        || lower.contains("/dist/")
        || lower.contains("/build/")
        || lower.ends_with(".min.js")
        || lower.ends_with(".min.css")
        || lower.ends_with(".map")
        || lower.ends_with("package-lock.json")
        || lower.ends_with("cargo.lock")
        || lower.ends_with("go.sum")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn big_results() -> String {
        let mut s = String::from("120 match(es); scanned 3 file(s)\n");
        for i in 0..60 {
            let _ = writeln!(s, "src/a.rs:{i}:let value_{i} = compute_long_name_{i}();");
        }
        for i in 0..60 {
            let _ = writeln!(s, "src/b.rs:{i}:fn helper_function_number_{i}() {{}}");
        }
        s
    }

    #[test]
    fn keeps_top_k_per_file_and_tally() {
        let input = big_results();
        let out = compress(&input, None).expect("compresses").text;
        assert!(out.contains("more match(es) in src/a.rs"), "{out}");
        assert!(out.contains("more match(es) in src/b.rs"));
        assert!(
            out.contains("[search omitted: 110 match(es) not shown across 2 file(s)]"),
            "{out}"
        );
        // Preamble survives.
        assert!(out.contains("120 match(es)"));
        assert!(out.len() < input.len());
    }

    #[test]
    fn query_ranks_relevant_matches() {
        let mut s = String::new();
        for i in 0..50 {
            let body = if i == 7 {
                "the special needle token appears here".to_string()
            } else {
                format!("ordinary line content number {i}")
            };
            let _ = writeln!(s, "src/x.rs:{i}:{body}");
        }
        let out = compress(&s, Some("needle token")).expect("compresses").text;
        assert!(
            out.contains("special needle token"),
            "ranked-in match missing:\n{out}"
        );
    }

    #[test]
    fn small_result_set_passes_through() {
        let s = "a.rs:1:hit\nb.rs:2:hit\n";
        assert!(compress(s, None).is_none());
    }

    #[test]
    fn ranked_search_exact_symbol_beats_path_and_density() {
        let query = SearchReadQuery {
            literal: Some("payment".to_string()),
            symbols: vec!["settle_invoice".to_string()],
            ..Default::default()
        };
        let symbol = SearchReadCandidate {
            path: "src/billing/worker.rs".into(),
            matched_lines: vec![SearchReadLine::new(42, "fn settle_invoice() -> Result<()>")],
            max_line: 100,
            ..SearchReadCandidate::new("src/billing/worker.rs")
        };
        let path_only = SearchReadCandidate {
            path: "src/payment/readme.md".into(),
            matched_lines: vec![SearchReadLine::new(1, "general overview")],
            max_line: 10,
            ..SearchReadCandidate::new("src/payment/readme.md")
        };
        let density = SearchReadCandidate {
            path: "src/other.rs".into(),
            matched_lines: vec![
                SearchReadLine::new(10, "payment retry"),
                SearchReadLine::new(11, "payment queue"),
            ],
            max_line: 20,
            ..SearchReadCandidate::new("src/other.rs")
        };

        let symbol_score = rank_search_read_candidate(&symbol, &query);
        let path_score = rank_search_read_candidate(&path_only, &query);
        let density_score = rank_search_read_candidate(&density, &query);

        assert!(symbol_score > path_score, "{symbol_score} <= {path_score}");
        assert!(
            path_score > density_score,
            "{path_score} <= {density_score}"
        );
    }

    #[test]
    fn ranked_search_deprioritizes_vendor_and_generated_paths() {
        let query = SearchReadQuery {
            symbols: vec!["hydrateRoot".to_string()],
            ..Default::default()
        };
        let first_party = SearchReadCandidate {
            path: "src/app/root.tsx".into(),
            matched_lines: vec![SearchReadLine::new(8, "export function hydrateRoot() {}")],
            max_line: 20,
            ..SearchReadCandidate::new("src/app/root.tsx")
        };
        let vendor = SearchReadCandidate {
            path: "node_modules/pkg/root.tsx".into(),
            matched_lines: vec![SearchReadLine::new(8, "export function hydrateRoot() {}")],
            max_line: 20,
            vendor: true,
            ..SearchReadCandidate::new("node_modules/pkg/root.tsx")
        };

        assert!(
            rank_search_read_candidate(&first_party, &query)
                > rank_search_read_candidate(&vendor, &query)
        );
    }

    #[test]
    fn snippet_windows_merge_and_report_omitted_matches() {
        let selected = select_snippet_windows(&[10, 11, 20, 50], 2, 100, 2);

        assert_eq!(
            selected.windows,
            vec![LineRange::new(8, 13), LineRange::new(18, 22)]
        );
        assert_eq!(selected.omitted_matches, 1);
    }
}
