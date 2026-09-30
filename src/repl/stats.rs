//! One-line, model-free description of a stored output (shape, size, insights).
//!
//! The stats line reports structure only (key names, counts, sizes). Values are
//! never echoed there; the separate [`head`] snippet is the only raw content.

use std::collections::BTreeSet;

use serde_json::Value;

use super::ReplLimits;
use super::ops;
use crate::tokens::estimate_tokens_with;
use crate::types::ContentKind;

/// Hard cap on the rendered stats line (~100 tokens at 4 chars/token).
const MAX_CHARS: usize = 400;
const MAX_KEYS: usize = 8;
const KEY_CLIP: usize = 24;
/// Characters of the raw input shown as a head snippet.
pub const HEAD_CHARS: usize = 500;

fn clip(s: &str, max: usize) -> String {
    if s.chars().count() <= max {
        return s.to_string();
    }
    let mut out: String = s.chars().take(max.saturating_sub(1)).collect();
    out.push('…');
    out
}

/// First [`HEAD_CHARS`] characters of `text`, with an ellipsis when cut.
pub fn head(text: &str) -> String {
    clip(text, HEAD_CHARS)
}

fn tok(n: u64) -> String {
    if n >= 1_000 {
        format!("{:.1}k", n as f64 / 1_000.0)
    } else {
        n.to_string()
    }
}

fn depth(v: &Value) -> usize {
    match v {
        Value::Array(a) => 1 + a.iter().map(depth).max().unwrap_or(0),
        Value::Object(o) => 1 + o.values().map(depth).max().unwrap_or(0),
        _ => 0,
    }
}

fn type_name(v: &Value) -> &'static str {
    match v {
        Value::Null => "null",
        Value::Bool(_) => "bool",
        Value::Number(_) => "number",
        Value::String(_) => "string",
        Value::Array(_) => "array",
        Value::Object(_) => "object",
    }
}

fn key_label(k: &str, v: &Value) -> String {
    let k = clip(k, KEY_CLIP);
    match v {
        Value::Array(a) => format!("{k}[{}]", a.len()),
        Value::Object(o) => format!("{k}{{{}}}", o.len()),
        _ => k,
    }
}

fn json_shape(text: &str, cpt: f32) -> Option<String> {
    let v: Value = serde_json::from_str(text).ok()?;
    let d = depth(&v);
    let shape = match &v {
        Value::Object(o) => {
            let keys: Vec<String> = o
                .iter()
                .take(MAX_KEYS)
                .map(|(k, v)| key_label(k, v))
                .collect();
            let more = if o.len() > MAX_KEYS { ", …" } else { "" };
            let largest = o
                .iter()
                .max_by_key(|(_, v)| v.to_string().len())
                .map(|(k, v)| {
                    format!(
                        " · largest: {} ~{} tok",
                        clip(k, KEY_CLIP),
                        tok(estimate_tokens_with(&v.to_string(), cpt))
                    )
                })
                .unwrap_or_default();
            format!(
                "JSON object · {} keys ({}{more}) · depth {d}{largest}",
                o.len(),
                keys.join(", ")
            )
        }
        Value::Array(a) => {
            let types: BTreeSet<&str> = a.iter().map(type_name).collect();
            let elem = match types.len() {
                0 => "empty",
                1 => types.into_iter().next().unwrap_or("empty"),
                _ => "mixed",
            };
            let keys = match a.first() {
                Some(Value::Object(o)) => {
                    let ks: Vec<String> =
                        o.keys().take(MAX_KEYS).map(|k| clip(k, KEY_CLIP)).collect();
                    let more = if o.len() > MAX_KEYS { ", …" } else { "" };
                    format!(" · item keys: {}{more}", ks.join(", "))
                }
                _ => String::new(),
            };
            format!("JSON array · {} items of {elem}{keys} · depth {d}", a.len())
        }
        other => format!("JSON {}", type_name(other)),
    };
    Some(shape)
}

fn log_shape(text: &str) -> String {
    let (mut err, mut warn) = (0usize, 0usize);
    for l in text.lines() {
        let l = l.to_ascii_lowercase();
        if l.contains("error") || l.contains("fail") || l.contains("panic") {
            err += 1;
        } else if l.contains("warn") {
            warn += 1;
        }
    }
    format!("log · {err} error-like, {warn} warn-like lines")
}

fn search_shape(text: &str) -> String {
    let files: BTreeSet<&str> = text
        .lines()
        .filter_map(|l| l.split(':').next())
        .filter(|p| !p.is_empty())
        .collect();
    format!("search results · {} files", files.len())
}

fn diff_shape(text: &str) -> String {
    let files = text.lines().filter(|l| l.starts_with("diff --git")).count();
    let hunks = text.lines().filter(|l| l.starts_with("@@")).count();
    format!("diff · {files} files, {hunks} hunks")
}

fn doc_shape(text: &str, label: &str) -> String {
    let lim = ReplLimits {
        max_hits: usize::MAX,
        ..ReplLimits::default()
    };
    let (heads, more_h) = ops::extract_headings(text, &lim);
    let (links, more_l) = ops::extract_links(text, &lim);
    let title = heads
        .first()
        .map(|h| format!(" · title: {}", clip(&h.text, 40)))
        .unwrap_or_default();
    format!(
        "{label} · {} headings, {} links{title}",
        heads.len() + more_h,
        links.len() + more_l
    )
}

/// Render a compact description (<= ~100 tokens) of `text`.
pub fn describe(text: &str, kind: ContentKind, chars_per_token: f32) -> String {
    let shape = match kind {
        ContentKind::Json => json_shape(text, chars_per_token),
        ContentKind::Log => Some(log_shape(text)),
        ContentKind::Search => Some(search_shape(text)),
        ContentKind::Diff => Some(diff_shape(text)),
        ContentKind::Html => Some(doc_shape(text, "HTML")),
        ContentKind::PlainText | ContentKind::Code => {
            if text.lines().any(|l| l.starts_with('#')) {
                Some(doc_shape(text, "markdown"))
            } else {
                None
            }
        }
    }
    .unwrap_or_else(|| kind.as_str().to_string());
    let longest = text.lines().map(|l| l.chars().count()).max().unwrap_or(0);
    let line = format!(
        "{shape} · ~{} tok, {} bytes, {} lines (longest {longest})",
        tok(estimate_tokens_with(text, chars_per_token)),
        text.len(),
        text.lines().count(),
    );
    clip(&line, MAX_CHARS)
}
