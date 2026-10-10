//! Pure, deterministic ops over a `&str`. No logging of content or patterns.

use regex::{Regex, RegexBuilder};

use super::types::{Heading, Hit, Link, RegexMatch, ReplError, ReplLimits, SearchHit};
use crate::compressors::html::html_to_markdown;
use crate::detect::kind::looks_like_html;
use crate::relevance::bm25::Bm25Corpus;

const SEARCH_WINDOW_LINES: usize = 6;

fn clip(line: &str, max: usize) -> String {
    if line.chars().count() <= max {
        return line.to_string();
    }
    let mut out: String = line.chars().take(max).collect();
    out.push('…');
    out
}

fn build_regex(pattern: &str, ignore_case: bool, limits: &ReplLimits) -> Result<Regex, ReplError> {
    RegexBuilder::new(pattern)
        .case_insensitive(ignore_case)
        .size_limit(limits.regex_size_limit)
        .dfa_size_limit(limits.regex_size_limit)
        .build()
        .map_err(|e| ReplError::InvalidPattern(e.to_string()))
}

pub fn grep(
    text: &str,
    pattern: &str,
    literal: bool,
    ignore_case: bool,
    context: usize,
    limits: &ReplLimits,
) -> Result<(Vec<Hit>, usize), ReplError> {
    if pattern.is_empty() {
        return Err(ReplError::EmptyQuery);
    }
    let source = if literal {
        regex::escape(pattern)
    } else {
        pattern.to_string()
    };
    let re = build_regex(&source, ignore_case, limits)?;
    // Bound context before selecting ranges, and allocate only the returned
    // lines. A caller-supplied context must not materialize the entire input.
    let context = context.min(limits.max_lines);
    let mut ranges: Vec<(usize, usize)> = Vec::new();
    let mut matching_lines = Vec::new();
    let mut matched = 0usize;
    let mut truncated = 0usize;
    for (i, line) in text.lines().enumerate() {
        if re.is_match(line) {
            if matched >= limits.max_hits {
                truncated += 1;
                continue;
            }
            matched += 1;
            matching_lines.push(i);
            let lo = i.saturating_sub(context);
            let hi = i.saturating_add(context);
            if let Some(last) = ranges.last_mut()
                && lo <= last.1.saturating_add(1)
            {
                last.1 = last.1.max(hi);
            } else {
                ranges.push((lo, hi));
            }
        }
    }
    let mut hits = Vec::new();
    let mut range = 0;
    let matching_lines = &matching_lines[..matching_lines.len().min(limits.max_lines)];
    let mut remaining_matches = matching_lines.len();
    for (i, line) in text.lines().enumerate() {
        while range < ranges.len() && i > ranges[range].1 {
            range += 1;
        }
        if range == ranges.len() {
            break;
        }
        if i < ranges[range].0 {
            continue;
        }
        let matching = matching_lines.binary_search(&i).is_ok();
        if matching {
            remaining_matches -= 1;
        }
        // Reserve slots for matching lines before filling them with context.
        if hits.len() + remaining_matches >= limits.max_lines && !matching {
            truncated += 1;
            continue;
        }
        if hits.len() >= limits.max_lines {
            truncated += 1;
            continue;
        }
        hits.push(Hit {
            line: i + 1,
            text: clip(line, limits.max_line_chars),
        });
    }
    Ok((hits, truncated))
}

pub fn find(text: &str, needle: &str, limits: &ReplLimits) -> Result<(Vec<Hit>, usize), ReplError> {
    if needle.is_empty() {
        return Err(ReplError::EmptyQuery);
    }
    grep(text, needle, true, true, 0, limits)
}

pub fn regex_matches(
    text: &str,
    pattern: &str,
    limits: &ReplLimits,
) -> Result<(Vec<RegexMatch>, usize), ReplError> {
    if pattern.is_empty() {
        return Err(ReplError::EmptyQuery);
    }
    let re = build_regex(pattern, false, limits)?;
    let mut out = Vec::new();
    let mut truncated = 0usize;
    for (i, line) in text.lines().enumerate() {
        for caps in re.captures_iter(line) {
            if out.len() >= limits.max_hits {
                truncated += 1;
                continue;
            }
            let whole = caps.get(0).map_or("", |m| m.as_str());
            out.push(RegexMatch {
                line: i + 1,
                text: clip(whole, limits.max_line_chars),
                captures: caps
                    .iter()
                    .skip(1)
                    .map(|m| m.map(|m| clip(m.as_str(), limits.max_line_chars)))
                    .collect(),
            });
        }
    }
    Ok((out, truncated))
}

/// BM25 over fixed windows of lines, best windows first.
pub fn search(
    text: &str,
    query: &str,
    top_k: usize,
    limits: &ReplLimits,
) -> Result<(Vec<SearchHit>, usize), ReplError> {
    if query.trim().is_empty() {
        return Err(ReplError::EmptyQuery);
    }
    let lines: Vec<&str> = text.lines().collect();
    let windows: Vec<(usize, usize)> = (0..lines.len())
        .step_by(SEARCH_WINDOW_LINES)
        .map(|s| (s, (s + SEARCH_WINDOW_LINES).min(lines.len())))
        .collect();
    let joined: Vec<String> = windows
        .iter()
        .map(|&(s, e)| lines[s..e].join("\n"))
        .collect();
    let corpus = Bm25Corpus::new(joined.iter().map(String::as_str));
    let mut scored: Vec<_> = corpus
        .score_all(query)
        .into_iter()
        .filter(|d| d.score > 0.0)
        .collect();
    scored.sort_by(|a, b| b.score.total_cmp(&a.score).then(a.index.cmp(&b.index)));
    let cap = top_k.min(limits.max_hits);
    let truncated = scored.len().saturating_sub(cap);
    let hits = scored
        .into_iter()
        .take(cap)
        .map(|d| {
            let (s, e) = windows[d.index];
            SearchHit {
                line_start: s + 1,
                line_end: e,
                score: d.score,
                text: clip(
                    &joined[d.index],
                    limits.max_line_chars * SEARCH_WINDOW_LINES,
                ),
            }
        })
        .collect();
    Ok((hits, truncated))
}

fn as_markdown(text: &str) -> std::borrow::Cow<'_, str> {
    if looks_like_html(text) {
        std::borrow::Cow::Owned(html_to_markdown(text))
    } else {
        std::borrow::Cow::Borrowed(text)
    }
}

/// Headings from Markdown, or from HTML via `html_to_markdown`. Fenced code is skipped.
/// Line numbers refer to the Markdown form for HTML input.
pub fn extract_headings(text: &str, limits: &ReplLimits) -> (Vec<Heading>, usize) {
    let md = as_markdown(text);
    let mut out = Vec::new();
    let mut truncated = 0usize;
    let mut fenced = false;
    for (i, line) in md.lines().enumerate() {
        let t = line.trim_start();
        if t.starts_with("```") || t.starts_with("~~~") {
            fenced = !fenced;
            continue;
        }
        if fenced {
            continue;
        }
        let level = t.chars().take_while(|&c| c == '#').count();
        if (1..=6).contains(&level) && t[level..].starts_with(' ') {
            if out.len() >= limits.max_hits {
                truncated += 1;
                continue;
            }
            out.push(Heading {
                level: level as u8,
                text: clip(
                    t[level..].trim().trim_end_matches('#').trim(),
                    limits.max_line_chars,
                ),
                line: i + 1,
            });
        }
    }
    (out, truncated)
}

/// Inline `[text](href)` links, deduplicated, in document order. HTML goes through
/// `html_to_markdown`, which already redacts sensitive query parameters.
pub fn extract_links(text: &str, limits: &ReplLimits) -> (Vec<Link>, usize) {
    let md = as_markdown(text);
    let bytes = md.as_bytes();
    let mut seen = std::collections::HashSet::new();
    let mut out = Vec::new();
    let mut truncated = 0usize;
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] != b'[' {
            i += 1;
            continue;
        }
        let Some(close) = md[i + 1..].find("](").map(|p| i + 1 + p) else {
            break;
        };
        let label = &md[i + 1..close];
        if label.contains('\n') || label.contains('[') {
            i += 1;
            continue;
        }
        let href_start = close + 2;
        let Some(end) = md[href_start..].find(')').map(|p| href_start + p) else {
            break;
        };
        let href = md[href_start..end].trim();
        if !href.is_empty()
            && !href.contains(char::is_whitespace)
            && seen.insert((label.to_string(), href.to_string()))
        {
            if out.len() >= limits.max_hits {
                truncated += 1;
            } else {
                out.push(Link {
                    text: clip(label, limits.max_line_chars),
                    href: href.to_string(),
                });
            }
        }
        i = end + 1;
    }
    (out, truncated)
}

/// Compact structural outline of a JSON value: keys, array lengths and the shape
/// of the first element. Leaf values are replaced by their type.
fn json_outline(value: &serde_json::Value, depth: usize, out: &mut String) {
    use serde_json::Value;
    match value {
        Value::Object(map) if depth == 0 => out.push_str(&format!("{{…{} keys}}", map.len())),
        Value::Object(map) => {
            out.push('{');
            for (i, (k, v)) in map.iter().take(12).enumerate() {
                if i > 0 {
                    out.push_str(", ");
                }
                out.push_str(k);
                out.push_str(": ");
                json_outline(v, depth - 1, out);
            }
            if map.len() > 12 {
                out.push_str(&format!(", …+{}", map.len() - 12));
            }
            out.push('}');
        }
        Value::Array(items) => {
            out.push_str(&format!("[{}×", items.len()));
            match items.first() {
                Some(first) if depth > 0 => json_outline(first, depth - 1, out),
                Some(_) => out.push('…'),
                None => {}
            }
            out.push(']');
        }
        Value::String(_) => out.push_str("str"),
        Value::Number(_) => out.push_str("num"),
        Value::Bool(_) => out.push_str("bool"),
        Value::Null => out.push_str("null"),
    }
}

fn json_summary(text: &str, max_chars: usize) -> Option<String> {
    let t = text.trim_start();
    if !(t.starts_with('{') || t.starts_with('[')) {
        return None;
    }
    let value: serde_json::Value = serde_json::from_str(text).ok()?;
    let mut shape = String::new();
    json_outline(&value, 6, &mut shape);
    if shape.chars().count() > max_chars {
        shape = shape.chars().take(max_chars).collect::<String>() + "…";
    }
    Some(format!(
        "{} bytes of JSON. Shape (arrays as [count×first-element]):\n{shape}\n",
        text.len()
    ))
}

/// Like [`summarize`], but a `hint` decides what the budget is spent on: the
/// size line and outline stay, then the lines most relevant to the hint (BM25 over
/// windows of lines) replace the head and tail.
pub fn summarize_with_hint(
    text: &str,
    hint: Option<&str>,
    max_chars: usize,
    limits: &ReplLimits,
) -> String {
    let Some(hint) = hint.map(str::trim).filter(|h| !h.is_empty()) else {
        return summarize(text, max_chars, limits);
    };
    if let Some(shape) = json_summary(text, max_chars) {
        return shape;
    }
    let lines = text.lines().count();
    let mut out = format!(
        "{} lines, {} bytes\nFocus: {}\n",
        lines,
        text.len(),
        clip(hint, limits.max_line_chars)
    );
    let (headings, _) = extract_headings(text, limits);
    for h in headings.iter().take(10) {
        out.push_str(&format!(
            "{}- {}\n",
            "  ".repeat(h.level.saturating_sub(1) as usize),
            h.text
        ));
    }
    let mut spent = out.chars().count();
    let mut wrote = false;
    if let Ok((hits, _)) = search(text, hint, limits.max_hits, limits) {
        for hit in hits {
            let block = format!(
                "[lines {}-{}]\n{}\n",
                hit.line_start, hit.line_end, hit.text
            );
            if spent + block.chars().count() > max_chars && wrote {
                break;
            }
            spent += block.chars().count();
            out.push_str(&block);
            wrote = true;
        }
    }
    if !wrote {
        out.push_str("(nothing matched the hint)\n");
        out.push_str(&summarize(text, max_chars.saturating_sub(spent), limits));
    }
    out
}

/// Model-free summary: size line, heading outline, then head and tail lines within budget.
pub fn summarize(text: &str, max_chars: usize, limits: &ReplLimits) -> String {
    if let Some(shape) = json_summary(text, max_chars) {
        return shape;
    }
    let lines: Vec<&str> = text.lines().collect();
    let mut out = format!("{} lines, {} bytes\n", lines.len(), text.len());
    let (headings, _) = extract_headings(text, limits);
    if !headings.is_empty() {
        out.push_str("Outline:\n");
        for h in headings.iter().take(20) {
            out.push_str(&format!(
                "{}- {}\n",
                "  ".repeat(h.level.saturating_sub(1) as usize),
                h.text
            ));
        }
    }
    let budget = max_chars.saturating_sub(out.chars().count());
    let half = budget / 2;
    let mut head = String::new();
    let mut head_n = 0;
    for l in lines.iter().filter(|l| !l.trim().is_empty()) {
        let l = clip(l, limits.max_line_chars);
        if head.chars().count() + l.chars().count() + 1 > half {
            break;
        }
        head.push_str(&l);
        head.push('\n');
        head_n += 1;
    }
    let mut tail: Vec<String> = Vec::new();
    let mut tail_len = 0;
    let body: Vec<&&str> = lines.iter().filter(|l| !l.trim().is_empty()).collect();
    for l in body[head_n.min(body.len())..].iter().rev() {
        let l = clip(l, limits.max_line_chars);
        if tail_len + l.chars().count() + 1 > half {
            break;
        }
        tail_len += l.chars().count() + 1;
        tail.push(l);
    }
    tail.reverse();
    out.push_str("Start:\n");
    out.push_str(&head);
    if !tail.is_empty() {
        out.push_str("…\nEnd:\n");
        out.push_str(&tail.join("\n"));
        out.push('\n');
    }
    out
}
