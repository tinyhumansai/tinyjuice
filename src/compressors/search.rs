//! Search-results compressor (relevance ranking).
//!
//! grep / ripgrep output is `path:line:body` matches. For very large result
//! sets the compressor groups matches by file, ranks them, keeps the top-K per
//! file plus a `[+N more in <file>]` tally, and (via the router) offloads the
//! full result set to CCR so the complete list is one `retrieve` away.
//!
//! Ranking: when the caller supplies a `query` in the [`ContentHint`], matches
//! are scored by the shared BM25/tokenizer path so regex punctuation and exact
//! identifiers carry through. Without a query, matches use body length /
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
use crate::cache::CcrStore;
use crate::detect::parse_search_line;
use crate::pipeline::{OffloadOutput, OffloadTransform, PipelineInput, estimate_bloat};
use crate::relevance::{Bm25Corpus, tokenize};
use crate::types::LineRange;
use crate::types::{CompressInput, CompressOptions, CompressOutput, CompressorKind, ContentKind};

/// Only compress result sets with more than this many matching lines.
pub const MIN_MATCHES: usize = 40;
/// Matches kept per file before the "+N more" tally.
pub const TOP_K_PER_FILE: usize = 5;
/// Default line context on each side for ranked search-read snippets.
pub const DEFAULT_SNIPPET_CONTEXT: usize = 2;

pub struct SearchCompressor;

/// Typed offload transform for lossy search-result thinning.
#[derive(Debug, Clone, Default)]
pub struct SearchTransform {
    query: Option<String>,
}

impl SearchTransform {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_query(mut self, query: impl Into<String>) -> Self {
        self.query = Some(query.into());
        self
    }

    pub fn query(&self) -> Option<&str> {
        self.query.as_deref()
    }
}

impl OffloadTransform for SearchTransform {
    fn name(&self) -> &'static str {
        "search"
    }

    fn estimate_bloat(&self, input: &PipelineInput<'_>) -> f32 {
        if input.content_kind != ContentKind::Search {
            return 0.0;
        }
        let score = f32::from(estimate_bloat(input.content, input.content_kind).score) / 100.0;
        if self
            .query
            .as_deref()
            .is_some_and(|query| !query.trim().is_empty())
        {
            score.max(0.1)
        } else {
            score
        }
    }

    fn apply(&self, input: &PipelineInput<'_>, store: &dyn CcrStore) -> Option<OffloadOutput> {
        if input.content_kind != ContentKind::Search {
            return None;
        }
        let compacted = compress(input.content, self.query())?;
        OffloadOutput::from_retained_put(
            compacted.text,
            CompressorKind::Search,
            store.put(input.original_content),
        )
    }
}

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

struct ParsedMatch<'a> {
    path: &'a str,
    line_no: u64,
    body: &'a str,
    raw: &'a str,
}

/// Compress search output. `query` (when known) ranks matches by shared BM25.
pub fn compress(content: &str, query: Option<&str>) -> Option<CompressOutput> {
    // Preserve any non-match preamble/summary lines (e.g. "80 match(es)") and
    // group match lines by file in first-seen order.
    let mut preamble: Vec<&str> = Vec::new();
    let mut parsed_matches: Vec<ParsedMatch<'_>> = Vec::new();

    for line in content.lines() {
        match parse_search_line(line) {
            Some((path, line_no, body)) => {
                parsed_matches.push(ParsedMatch {
                    path,
                    line_no,
                    body,
                    raw: line,
                });
            }
            None => {
                if !line.trim().is_empty() && parsed_matches.is_empty() {
                    // Only keep preamble that appears before any match.
                    preamble.push(line);
                }
            }
        }
    }

    let match_count = parsed_matches.len();
    if match_count < MIN_MATCHES || parsed_matches.is_empty() {
        return None;
    }

    let scores = score_matches(&parsed_matches, query);
    let mut files: Vec<(&str, Vec<Match<'_>>)> = Vec::new();
    for (parsed, score) in parsed_matches.iter().zip(scores) {
        let m = Match {
            line_no: parsed.line_no,
            body: parsed.body,
            score,
            raw: parsed.raw,
        };
        if let Some((_, v)) = files.iter_mut().find(|(p, _)| *p == parsed.path) {
            v.push(m);
        } else {
            files.push((parsed.path, vec![m]));
        }
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

/// Score search-result bodies. With query text, shared BM25 dominates so query
/// syntax and exact identifiers are interpreted the same way as search-read
/// candidate ranking. Otherwise, distinctiveness (length) plus importance
/// signals decides ranking.
fn score_matches(matches: &[ParsedMatch<'_>], query: Option<&str>) -> Vec<f32> {
    if let Some(query) = query
        && !tokenize(query).is_empty()
    {
        let corpus = Bm25Corpus::new(matches.iter().map(|m| m.body));
        return corpus
            .score_all(query)
            .into_iter()
            .map(|score| {
                let importance = line_score(matches[score.index].body);
                score.score.max(importance * 0.25)
            })
            .collect();
    }

    matches
        .iter()
        .map(|m| score_match_without_query(m.body))
        .collect()
}

fn score_match_without_query(body: &str) -> f32 {
    let importance = line_score(body);
    let len_score = (body.trim().len() as f32 / 80.0).min(1.0);
    importance.max(0.2 + 0.8 * len_score)
}

/// Host-provided search-read query metadata. TinyJuice only scores already
/// discovered matches; filesystem traversal stays in the host.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SearchReadQuery {
    pub literal: Option<String>,
    pub regex: Option<String>,
    pub symbols: Vec<String>,
    pub file_kinds: Vec<String>,
    pub penalize_vendor: bool,
    pub penalize_generated: bool,
}

impl Default for SearchReadQuery {
    fn default() -> Self {
        Self {
            literal: None,
            regex: None,
            symbols: Vec::new(),
            file_kinds: Vec::new(),
            penalize_vendor: true,
            penalize_generated: true,
        }
    }
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

    let generated_penalty = if query.penalize_generated
        && (candidate.generated || looks_generated_path(&candidate.path))
    {
        4.0
    } else {
        0.0
    };
    let vendor_penalty =
        if query.penalize_vendor && (candidate.vendor || looks_vendor_path(&candidate.path)) {
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
#[path = "search_tests.rs"]
mod tests;
