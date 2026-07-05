//! Plain-text compressor.
//!
//! ML compression is tried first when enabled. When it is unavailable or
//! disabled, TextCrusher provides a deterministic extractive fallback: it keeps
//! verbatim salient/query-relevant spans and suppresses near-duplicates.

use async_trait::async_trait;
use std::collections::HashSet;

use super::Compressor;
use crate::relevance::Bm25Corpus;
use crate::types::{CompressInput, CompressOptions, CompressOutput, CompressorKind};

pub const MIN_SEGMENTS: usize = 12;
pub const TARGET_RATIO: f32 = 0.40;
const MIN_TARGET_CHARS: usize = 800;

pub struct TextCompressor;

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
        if opts.ml_text_enabled {
            match crate::ml::compress(input.content, opts).await {
                Ok(Some(text)) if text.len() < input.content.len() => {
                    return Some(CompressOutput::lossy(text, CompressorKind::MlText));
                }
                Ok(_) => {}
                Err(e) => {
                    log::debug!("[tokenjuice][ml] unavailable, falling back: {e:#}");
                }
            }
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
    use crate::types::CompressOptions;

    fn opts() -> CompressOptions {
        CompressOptions {
            min_bytes_to_compress: 64,
            ccr_min_tokens: 1,
            ..Default::default()
        }
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
