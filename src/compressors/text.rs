//! Plain-text compressor.
//!
//! ML compression is tried first when enabled. When it is unavailable or
//! disabled, TextCrusher provides a deterministic extractive fallback: it keeps
//! verbatim salient/query-relevant spans and suppresses near-duplicates.

use async_trait::async_trait;
use std::collections::HashSet;

use super::Compressor;
use crate::cache::CcrStore;
use crate::pipeline::{OffloadOutput, OffloadTransform, PipelineInput, estimate_bloat};
use crate::relevance::Bm25Corpus;
use crate::types::{CompressInput, CompressOptions, CompressOutput, CompressorKind, ContentKind};

pub const MIN_SEGMENTS: usize = 12;
pub const TARGET_RATIO: f32 = 0.40;
const MIN_TARGET_CHARS: usize = 800;

pub struct TextCompressor;

/// Typed offload transform for deterministic extractive TextCrusher.
#[derive(Debug, Clone)]
pub struct TextCrusherTransform {
    options: CompressOptions,
    query: Option<String>,
}

impl TextCrusherTransform {
    pub fn new(options: CompressOptions) -> Self {
        Self {
            options,
            query: None,
        }
    }

    pub fn with_query(mut self, query: impl Into<String>) -> Self {
        self.query = Some(query.into());
        self
    }

    pub fn options(&self) -> &CompressOptions {
        &self.options
    }

    pub fn query(&self) -> Option<&str> {
        self.query.as_deref()
    }
}

impl Default for TextCrusherTransform {
    fn default() -> Self {
        Self::new(CompressOptions::default())
    }
}

impl OffloadTransform for TextCrusherTransform {
    fn name(&self) -> &'static str {
        "textcrusher"
    }

    fn estimate_bloat(&self, input: &PipelineInput<'_>) -> f32 {
        if input.content_kind != ContentKind::PlainText {
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
        if input.content_kind != ContentKind::PlainText {
            return None;
        }
        let compacted = compress_textcrusher(input.content, self.query(), &self.options)?;
        OffloadOutput::from_retained_put(
            compacted.text,
            CompressorKind::TextCrusher,
            store.put(input.original_content),
        )
    }
}

#[async_trait]
impl Compressor for TextCompressor {
    fn kind(&self) -> CompressorKind {
        CompressorKind::TextCrusher
    }

    async fn compress(
        &self,
        input: &CompressInput<'_>,
        opts: &CompressOptions,
    ) -> Option<CompressOutput> {
        if let Some(out) = compress_ml_with_tag_protection(input.content, opts).await {
            return Some(out);
        }

        compress_textcrusher(input.content, input.hint.query.as_deref(), opts)
    }
}

#[derive(Debug, Clone)]
struct Segment {
    index: usize,
    text: String,
    terms: Vec<String>,
    score: f32,
    salience: f32,
}

pub async fn compress_ml_with_tag_protection(
    content: &str,
    opts: &CompressOptions,
) -> Option<CompressOutput> {
    if !opts.ml_text_enabled {
        return None;
    }

    let protected = protect_custom_tags(content);
    match crate::ml::compress(&protected.text, opts).await {
        Ok(Some(text)) => {
            let text = protected.restore(&text)?;
            if text.len() < content.len() {
                Some(CompressOutput::lossy(text, CompressorKind::MlText))
            } else {
                None
            }
        }
        Ok(_) => None,
        Err(e) => {
            log::debug!("[tokenjuice][ml] unavailable, falling back: {e:#}");
            None
        }
    }
}

pub fn compress_textcrusher(
    content: &str,
    query: Option<&str>,
    opts: &CompressOptions,
) -> Option<CompressOutput> {
    let mut segments = split_segments(content);
    if segments.len() < MIN_SEGMENTS {
        return None;
    }

    let duplicate_pressure = duplicate_pressure(&segments);
    let query = query.filter(|q| !q.trim().is_empty());
    let bm25 = query.map(|_| {
        Bm25Corpus::from_tokenized(
            segments
                .iter()
                .map(|segment| segment.terms.clone())
                .collect(),
        )
    });

    let segment_count = segments.len();
    for segment in &mut segments {
        segment.salience = salience_score(&segment.text);
        let recency = (segment.index + 1) as f32 / segment_count as f32;
        let query_score = match (query, bm25.as_ref()) {
            (Some(q), Some(corpus)) => corpus.score(q, segment.index).min(4.0) / 4.0,
            _ => 0.0,
        };
        segment.score = segment.salience * 2.0 + query_score * 2.5 + recency * 0.35;
    }

    let has_signal = query.is_some()
        || duplicate_pressure >= 4
        || segments.iter().any(|segment| segment.salience >= 0.5);
    if !has_signal {
        return None;
    }

    let ratio_target = (content.len() as f32 * TARGET_RATIO) as usize;
    let target_chars = opts
        .max_inline_chars
        .unwrap_or(ratio_target)
        .min(ratio_target.max(MIN_TARGET_CHARS))
        .min(content.len().saturating_sub(1));
    if target_chars == 0 {
        return None;
    }

    let mut ranked: Vec<usize> = (0..segments.len()).collect();
    ranked.sort_by(|&a, &b| {
        segments[b]
            .score
            .partial_cmp(&segments[a].score)
            .unwrap_or(std::cmp::Ordering::Equal)
            .then_with(|| a.cmp(&b))
    });

    let mut selected = Vec::new();
    let mut selected_terms: Vec<HashSet<String>> = Vec::new();
    let mut used_chars = 0usize;

    for index in ranked {
        let segment = &segments[index];
        if used_chars + segment.text.len() + 1 > target_chars && !selected.is_empty() {
            continue;
        }
        let terms: HashSet<String> = segment.terms.iter().cloned().collect();
        if selected_terms
            .iter()
            .any(|existing| jaccard(existing, &terms) >= 0.82)
        {
            continue;
        }
        used_chars += segment.text.len() + 1;
        selected.push(index);
        selected_terms.push(terms);
    }

    if selected.is_empty() || selected.len() == segments.len() {
        return None;
    }

    selected.sort_unstable();
    let out = selected
        .into_iter()
        .map(|index| segments[index].text.as_str())
        .collect::<Vec<_>>()
        .join("\n");

    if out.len() >= content.len() {
        return None;
    }
    Some(CompressOutput::lossy(out, CompressorKind::TextCrusher))
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ProtectedText {
    text: String,
    replacements: Vec<TagReplacement>,
}

impl ProtectedText {
    fn restore(&self, candidate: &str) -> Option<String> {
        for replacement in &self.replacements {
            if !candidate.contains(&replacement.placeholder) {
                return None;
            }
        }

        let mut restored = candidate.to_string();
        for replacement in &self.replacements {
            restored = restored.replace(&replacement.placeholder, &replacement.original);
        }
        Some(restored)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct TagReplacement {
    placeholder: String,
    original: String,
}

fn protect_custom_tags(content: &str) -> ProtectedText {
    let ranges = custom_tag_ranges(content);
    if ranges.is_empty() {
        return ProtectedText {
            text: content.to_string(),
            replacements: Vec::new(),
        };
    }

    let prefix = safe_placeholder_prefix(content);
    let mut text = String::with_capacity(content.len());
    let mut replacements = Vec::new();
    let mut cursor = 0usize;

    for (i, range) in ranges.into_iter().enumerate() {
        if range.start < cursor {
            continue;
        }
        text.push_str(&content[cursor..range.start]);
        let placeholder = format!("{prefix}{i}__");
        text.push_str(&placeholder);
        replacements.push(TagReplacement {
            placeholder,
            original: content[range.start..range.end].to_string(),
        });
        cursor = range.end;
    }
    text.push_str(&content[cursor..]);

    ProtectedText { text, replacements }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct ByteRange {
    start: usize,
    end: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct OpenTag {
    name: String,
    start: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TagKind {
    Open,
    Close,
    SelfClosing,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ParsedTag {
    range: ByteRange,
    name: String,
    kind: TagKind,
    custom: bool,
}

fn custom_tag_ranges(content: &str) -> Vec<ByteRange> {
    let mut ranges = Vec::new();
    let mut stack: Vec<OpenTag> = Vec::new();
    let mut cursor = 0usize;

    while let Some(relative) = content[cursor..].find('<') {
        let start = cursor + relative;
        let Some(tag) = parse_tag_at(content, start) else {
            cursor = start + 1;
            continue;
        };
        cursor = tag.range.end;

        if !tag.custom {
            continue;
        }

        match tag.kind {
            TagKind::SelfClosing => ranges.push(tag.range),
            TagKind::Open => stack.push(OpenTag {
                name: tag.name,
                start: tag.range.start,
            }),
            TagKind::Close => {
                if let Some(pos) = stack.iter().rposition(|open| open.name == tag.name) {
                    let open = stack.remove(pos);
                    stack.truncate(pos);
                    ranges.push(ByteRange {
                        start: open.start,
                        end: tag.range.end,
                    });
                }
            }
        }
    }

    ranges.sort_by(|a, b| a.start.cmp(&b.start).then_with(|| b.end.cmp(&a.end)));
    let mut non_overlapping = Vec::new();
    let mut covered_end = 0usize;
    for range in ranges {
        if range.start >= covered_end {
            covered_end = range.end;
            non_overlapping.push(range);
        }
    }
    non_overlapping
}

fn parse_tag_at(content: &str, start: usize) -> Option<ParsedTag> {
    let tail = content.get(start..)?;
    if !tail.starts_with('<')
        || tail.starts_with("<!--")
        || tail.starts_with("<!")
        || tail.starts_with("<?")
    {
        return None;
    }

    let end = start + tail.find('>')? + 1;
    let mut inner = content[start + 1..end - 1].trim();
    if inner.is_empty() {
        return None;
    }

    let closing = inner.starts_with('/');
    if closing {
        inner = inner[1..].trim_start();
    }
    let self_closing = !closing && inner.trim_end().ends_with('/');
    if self_closing {
        inner = inner.trim_end_matches('/').trim_end();
    }

    let name_end = inner
        .char_indices()
        .find_map(|(idx, ch)| (!is_tag_name_char(ch)).then_some(idx))
        .unwrap_or(inner.len());
    let name = &inner[..name_end];
    if !valid_tag_name(name) {
        return None;
    }

    let lower = name.to_ascii_lowercase();
    Some(ParsedTag {
        range: ByteRange { start, end },
        name: lower.clone(),
        kind: if closing {
            TagKind::Close
        } else if self_closing {
            TagKind::SelfClosing
        } else {
            TagKind::Open
        },
        custom: !is_html_tag(&lower),
    })
}

fn valid_tag_name(name: &str) -> bool {
    let mut chars = name.chars();
    chars
        .next()
        .is_some_and(|ch| ch.is_ascii_alphabetic() || ch == '_')
        && chars.all(is_tag_name_char)
}

fn is_tag_name_char(ch: char) -> bool {
    ch.is_ascii_alphanumeric() || ch == '_' || ch == '-' || ch == ':'
}

fn safe_placeholder_prefix(content: &str) -> String {
    for salt in 0..1024usize {
        let prefix = format!("__TOKENJUICE_TAG_{salt}_");
        if !content.contains(&prefix) {
            return prefix;
        }
    }
    "__TOKENJUICE_TAG_FALLBACK_".to_string()
}

fn is_html_tag(name: &str) -> bool {
    matches!(
        name,
        "a" | "abbr"
            | "address"
            | "area"
            | "article"
            | "aside"
            | "audio"
            | "b"
            | "base"
            | "bdi"
            | "bdo"
            | "blockquote"
            | "body"
            | "br"
            | "button"
            | "canvas"
            | "caption"
            | "cite"
            | "code"
            | "col"
            | "colgroup"
            | "data"
            | "datalist"
            | "dd"
            | "del"
            | "details"
            | "dfn"
            | "dialog"
            | "div"
            | "dl"
            | "dt"
            | "em"
            | "embed"
            | "fieldset"
            | "figcaption"
            | "figure"
            | "footer"
            | "form"
            | "h1"
            | "h2"
            | "h3"
            | "h4"
            | "h5"
            | "h6"
            | "head"
            | "header"
            | "hr"
            | "html"
            | "i"
            | "iframe"
            | "img"
            | "input"
            | "ins"
            | "kbd"
            | "label"
            | "legend"
            | "li"
            | "link"
            | "main"
            | "map"
            | "mark"
            | "menu"
            | "meta"
            | "meter"
            | "nav"
            | "noscript"
            | "object"
            | "ol"
            | "optgroup"
            | "option"
            | "output"
            | "p"
            | "picture"
            | "pre"
            | "progress"
            | "q"
            | "rp"
            | "rt"
            | "ruby"
            | "s"
            | "samp"
            | "script"
            | "section"
            | "select"
            | "slot"
            | "small"
            | "source"
            | "span"
            | "strong"
            | "style"
            | "sub"
            | "summary"
            | "sup"
            | "table"
            | "tbody"
            | "td"
            | "template"
            | "textarea"
            | "tfoot"
            | "th"
            | "thead"
            | "time"
            | "title"
            | "tr"
            | "track"
            | "u"
            | "ul"
            | "var"
            | "video"
            | "wbr"
    )
}

fn split_segments(content: &str) -> Vec<Segment> {
    let mut segments = Vec::new();
    for block in content.split("\n\n") {
        let trimmed = block.trim();
        if trimmed.is_empty() {
            continue;
        }
        if trimmed.len() <= 260 {
            push_segment(&mut segments, trimmed);
            continue;
        }
        for sentence in split_sentences(trimmed) {
            push_segment(&mut segments, sentence);
        }
    }
    segments
}

fn split_sentences(block: &str) -> Vec<&str> {
    let mut out = Vec::new();
    let mut start = 0usize;
    let mut iter = block.char_indices().peekable();
    while let Some((idx, ch)) = iter.next() {
        let sentence_end = matches!(ch, '.' | '!' | '?')
            && iter.peek().is_none_or(|(_, next)| next.is_whitespace());
        if !sentence_end {
            continue;
        }
        let end = idx + ch.len_utf8();
        let sentence = block[start..end].trim();
        if !sentence.is_empty() {
            out.push(sentence);
        }
        while let Some((next_idx, next_ch)) = iter.peek().copied() {
            if next_ch.is_whitespace() {
                iter.next();
                start = next_idx + next_ch.len_utf8();
            } else {
                break;
            }
        }
    }
    let tail = block[start..].trim();
    if !tail.is_empty() {
        out.push(tail);
    }
    out
}

fn push_segment(segments: &mut Vec<Segment>, text: &str) {
    let terms = crate::relevance::tokenize(text);
    if terms.len() < 3 {
        return;
    }
    segments.push(Segment {
        index: segments.len(),
        text: text.to_string(),
        terms,
        score: 0.0,
        salience: 0.0,
    });
}

fn salience_score(text: &str) -> f32 {
    let lower = text.to_ascii_lowercase();
    let mut score = 0.0f32;
    if lower.contains("error")
        || lower.contains("failed")
        || lower.contains("panic")
        || lower.contains("exception")
        || lower.contains("warning")
        || lower.contains("todo")
    {
        score += 1.0;
    }
    if text.chars().any(|ch| ch.is_ascii_digit()) {
        score += 0.35;
    }
    if text.split_whitespace().any(is_all_caps_identifier) {
        score += 0.5;
    }
    if text.split_whitespace().any(is_dotted_identifier) {
        score += 0.5;
    }
    score.min(2.0)
}

fn is_all_caps_identifier(token: &str) -> bool {
    let token = token.trim_matches(|ch: char| !ch.is_ascii_alphanumeric() && ch != '_');
    token.len() >= 4
        && token.chars().any(|ch| ch.is_ascii_alphabetic())
        && token
            .chars()
            .all(|ch| ch.is_ascii_uppercase() || ch.is_ascii_digit() || ch == '_')
}

fn is_dotted_identifier(token: &str) -> bool {
    let token = token.trim_matches(|ch: char| !ch.is_ascii_alphanumeric() && ch != '.');
    token.split('.').filter(|part| part.len() >= 2).count() >= 2
}

fn duplicate_pressure(segments: &[Segment]) -> usize {
    let mut seen: Vec<HashSet<String>> = Vec::new();
    let mut duplicates = 0usize;
    for segment in segments {
        let terms: HashSet<String> = segment.terms.iter().cloned().collect();
        if seen
            .iter()
            .any(|existing| jaccard(existing, &terms) >= 0.82)
        {
            duplicates += 1;
        } else {
            seen.push(terms);
        }
    }
    duplicates
}

fn jaccard(a: &HashSet<String>, b: &HashSet<String>) -> f32 {
    if a.is_empty() || b.is_empty() {
        return 0.0;
    }
    let intersection = a.intersection(b).count();
    let union = a.len() + b.len() - intersection;
    intersection as f32 / union as f32
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;

    use crate::cache::{CcrStore, MemoryCcrStore};
    use crate::pipeline::PipelineInput;
    use crate::types::CompressOptions;

    fn opts() -> CompressOptions {
        CompressOptions {
            min_bytes_to_compress: 64,
            ccr_min_tokens: 1,
            ..Default::default()
        }
    }

    fn ml_opts() -> CompressOptions {
        CompressOptions {
            ml_text_enabled: true,
            ..opts()
        }
    }

    #[test]
    fn protector_ignores_normal_html_tags() {
        let protected = protect_custom_tags("<div><span>hello</span><br /></div>");

        assert_eq!(protected.text, "<div><span>hello</span><br /></div>");
        assert!(protected.replacements.is_empty());
    }

    #[test]
    fn protector_handles_nested_and_self_closing_custom_tags() {
        let tag = "<workflow id=\"a\">\n<context-item value=\"1\" />\n</workflow>";
        let input = format!("before {tag} after");
        let protected = protect_custom_tags(&input);

        assert_eq!(protected.replacements.len(), 1);
        assert!(!protected.text.contains(tag));
        assert_eq!(
            protected.restore(&protected.text).as_deref(),
            Some(input.as_str())
        );
    }

    #[test]
    fn protector_restores_duplicate_blocks_independently() {
        let input = "<task>first</task>\nbody\n<task>second</task>";
        let protected = protect_custom_tags(input);

        assert_eq!(protected.replacements.len(), 2);
        assert_ne!(
            protected.replacements[0].placeholder,
            protected.replacements[1].placeholder
        );
        let candidate = format!(
            "{} then {}",
            protected.replacements[1].placeholder, protected.replacements[0].placeholder
        );
        let restored = protected.restore(&candidate).expect("restore");

        assert_eq!(restored, "<task>second</task> then <task>first</task>");
    }

    #[test]
    fn protector_chooses_uncollided_placeholder_prefix() {
        let input = "__TOKENJUICE_TAG_0_ already exists <custom-tag />";
        let protected = protect_custom_tags(input);

        assert_eq!(protected.replacements.len(), 1);
        assert!(
            protected.replacements[0]
                .placeholder
                .starts_with("__TOKENJUICE_TAG_1_"),
            "{protected:?}"
        );
        assert_eq!(protected.restore(&protected.text).as_deref(), Some(input));
    }

    #[test]
    fn malformed_custom_tag_passes_without_panic() {
        let input = "before <workflow><inner> missing close";
        let protected = protect_custom_tags(input);

        assert_eq!(protected.text, input);
        assert!(protected.replacements.is_empty());
    }

    #[tokio::test]
    async fn ml_path_restores_custom_tags_byte_for_byte() {
        let _guard = crate::ml::callback_test_guard().await;
        crate::ml::configure_callback(Some(Arc::new(|text, _opts| {
            Box::pin(async move {
                assert!(!text.contains("<workflow"));
                let placeholder = text
                    .split_whitespace()
                    .find(|part| part.contains("__TOKENJUICE_TAG_"))
                    .expect("placeholder")
                    .to_string();
                Ok(Some(format!("compressed {placeholder}")))
            })
        })));

        let tag = "<workflow id=\"alpha\">\n<context-item value=\"1\" />\n</workflow>";
        let input = format!("{tag}\n{}", "ordinary filler ".repeat(200));
        let out = compress_ml_with_tag_protection(&input, &ml_opts())
            .await
            .expect("ml output");
        crate::ml::configure_callback(None);

        assert_eq!(out.kind, CompressorKind::MlText);
        assert!(out.text.contains(tag), "{}", out.text);
    }

    #[tokio::test]
    async fn ml_path_declines_when_callback_drops_placeholder() {
        let _guard = crate::ml::callback_test_guard().await;
        crate::ml::configure_callback(Some(Arc::new(|_text, _opts| {
            Box::pin(async move { Ok(Some("compressed without protected marker".to_string())) })
        })));

        let input = format!("<workflow>keep me</workflow>\n{}", "filler ".repeat(200));
        let out = compress_ml_with_tag_protection(&input, &ml_opts()).await;
        crate::ml::configure_callback(None);

        assert!(out.is_none());
    }

    #[test]
    fn keeps_errors_and_identifiers_as_verbatim_spans() {
        let mut input = String::new();
        for i in 0..40 {
            input.push_str(&format!(
                "ordinary deployment progress line {i} with routine status information.\n\n"
            ));
        }
        input.push_str("ERROR sync.worker.v2 failed for REQUEST_ID 9F42 after retry 17.\n\n");
        for i in 40..80 {
            input.push_str(&format!(
                "ordinary deployment progress line {i} with routine status information.\n\n"
            ));
        }

        let out = compress_textcrusher(&input, Some("sync.worker.v2 REQUEST_ID"), &opts())
            .expect("compresses")
            .text;

        assert!(out.contains("ERROR sync.worker.v2 failed"), "{out}");
        for line in out.lines() {
            assert!(input.contains(line), "non-verbatim span: {line}");
        }
    }

    #[test]
    fn suppresses_near_duplicate_prose_deterministically() {
        let mut input = String::new();
        for i in 0..80 {
            input.push_str(&format!(
                "worker queue processed duplicate status message for tenant alpha batch {i}.\n\n"
            ));
        }

        let first = compress_textcrusher(&input, None, &opts())
            .expect("compresses")
            .text;
        let second = compress_textcrusher(&input, None, &opts())
            .expect("compresses")
            .text;

        assert_eq!(first, second);
        assert!(first.lines().count() < 20, "{first}");
        assert!(first.len() < input.len());
    }

    #[test]
    fn textcrusher_transform_requires_retained_ccr() {
        let mut input = String::new();
        for i in 0..40 {
            input.push_str(&format!(
                "ordinary deployment progress line {i} with routine status information.\n\n"
            ));
        }
        input.push_str("ERROR sync.worker.v2 failed for REQUEST_ID 9F42 after retry 17.\n\n");
        for i in 40..80 {
            input.push_str(&format!(
                "ordinary deployment progress line {i} with routine status information.\n\n"
            ));
        }
        let pipeline_input = PipelineInput {
            content: &input,
            original_content: &input,
            content_kind: ContentKind::PlainText,
            original_bytes: input.len(),
        };
        let transform = TextCrusherTransform::new(opts()).with_query("sync.worker.v2 REQUEST_ID");

        let rejecting_store = MemoryCcrStore::new(1, 1);
        assert!(transform.apply(&pipeline_input, &rejecting_store).is_none());

        let store = MemoryCcrStore::default();
        let out = transform
            .apply(&pipeline_input, &store)
            .expect("retained textcrusher output");

        assert_eq!(out.kind(), CompressorKind::TextCrusher);
        assert!(out.text().contains("ERROR sync.worker.v2 failed"));
        assert_eq!(store.get(out.token()).as_deref(), Some(input.as_str()));
    }

    #[test]
    fn textcrusher_transform_skips_non_plain_input() {
        let input = PipelineInput {
            content: "diff --git a/a b/a",
            original_content: "diff --git a/a b/a",
            content_kind: ContentKind::Diff,
            original_bytes: "diff --git a/a b/a".len(),
        };
        let transform = TextCrusherTransform::default();
        let store = MemoryCcrStore::default();

        assert_eq!(transform.estimate_bloat(&input), 0.0);
        assert!(transform.apply(&input, &store).is_none());
    }

    #[test]
    fn plain_unsalient_data_declines() {
        let sentences = [
            "soft morning light crossed the empty kitchen and settled on the table.",
            "a folded blanket rested beside the chair near the quiet window.",
            "the hallway carried a faint smell of cedar from the old cabinet.",
            "afternoon clouds gathered slowly above the roofs beyond the lane.",
            "a notebook remained open to a page of careful handwriting.",
            "the garden path curved around stones set deep in the soil.",
            "warm tea cooled beside a stack of letters tied with string.",
            "the room held a simple lamp and a shelf of travel books.",
            "a blue curtain moved lightly whenever the door opened.",
            "distant bells marked the hour across the sleeping square.",
            "the wooden floor showed pale marks from years of use.",
            "fresh linen hung across the rail in the narrow courtyard.",
            "a small map lay flat beneath a smooth glass paperweight.",
            "evening settled over the street with a gentle amber color.",
            "the desk drawer contained stamps envelopes and a fountain pen.",
            "quiet footsteps faded on the stair beyond the landing.",
            "the wall clock ticked steadily through the still apartment.",
            "a weathered basket sat near the hearth filled with kindling.",
            "the porch boards were dry after several days of sun.",
            "a clean bowl and spoon waited beside the folded napkin.",
        ];
        let mut input = String::new();
        for sentence in sentences {
            input.push_str(sentence);
            input.push_str("\n\n");
        }

        assert!(compress_textcrusher(&input, None, &opts()).is_none());
    }
}
