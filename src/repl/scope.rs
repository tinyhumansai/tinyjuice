//! Python-style slice scoping: `[:]`, `[10:50]`, `[:-100]`, `[-200:]`.
//!
//! A host uses it to narrow what an op looks at, by lines (default) or characters.

use serde::{Deserialize, Serialize};

use super::types::ReplError;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ScopeUnit {
    #[default]
    Lines,
    Chars,
}

fn bad(msg: &str) -> ReplError {
    ReplError::InvalidPattern(format!("scope: {msg}"))
}

fn bound(s: &str) -> Result<Option<i64>, ReplError> {
    let s = s.trim();
    if s.is_empty() {
        return Ok(None);
    }
    s.parse()
        .map(Some)
        .map_err(|_| bad("bounds must be integers, like [10:-5]"))
}

/// Resolve a slice against `len` with Python semantics: negatives count from the
/// end, out-of-range bounds clamp, and a reversed range is empty.
pub fn resolve(spec: &str, len: usize) -> Result<(usize, usize), ReplError> {
    let inner = spec.trim().trim_start_matches('[').trim_end_matches(']');
    let (lo, hi) = inner
        .split_once(':')
        .ok_or_else(|| bad("expected [start:end]"))?;
    if hi.contains(':') {
        return Err(bad("steps are not supported"));
    }
    let n = len as i64;
    let fix = |b: Option<i64>, default: i64| match b {
        None => default,
        Some(v) if v < 0 => (n + v).max(0),
        Some(v) => v.min(n),
    };
    let start = fix(bound(lo)?, 0);
    let end = fix(bound(hi)?, n);
    Ok((start as usize, end.max(start) as usize))
}

/// The scoped text and the number of input lines before it, so line numbers in
/// results can be reported against the original.
pub fn apply<'a>(
    text: &'a str,
    spec: &str,
    unit: ScopeUnit,
) -> Result<(&'a str, usize), ReplError> {
    match unit {
        ScopeUnit::Lines => {
            let starts: Vec<usize> = std::iter::once(0)
                .chain(text.match_indices('\n').map(|(i, _)| i + 1))
                .filter(|&i| i < text.len())
                .collect();
            let (a, b) = resolve(spec, starts.len())?;
            if a >= b {
                return Ok(("", a));
            }
            let end = starts.get(b).copied().unwrap_or(text.len());
            Ok((&text[starts[a]..end], a))
        }
        ScopeUnit::Chars => {
            let offsets: Vec<usize> = text.char_indices().map(|(i, _)| i).collect();
            let (a, b) = resolve(spec, offsets.len())?;
            if a >= b {
                return Ok(("", 0));
            }
            let start = offsets[a];
            let end = offsets.get(b).copied().unwrap_or(text.len());
            Ok((&text[start..end], text[..start].matches('\n').count()))
        }
    }
}
