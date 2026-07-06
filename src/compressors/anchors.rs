//! Query-anchor extraction.
//!
//! Pulls deterministic, high-signal exact-match tokens out of a caller-supplied
//! query string so the content compressors can force-keep any row/hunk that
//! mentions one. These are the tokens a human would paste into a search box and
//! expect a *literal* hit on: identifiers (UUIDs), numeric IDs, quoted phrases,
//! hostnames and emails. Everything is hand-rolled (no regex on this path) to
//! stay consistent with the substring scanners in `signals.rs`.
//!
//! Anchors are returned lower-cased and de-duplicated; callers match them as
//! case-insensitive substrings over a serialized row/line.

use std::collections::BTreeSet;

/// Shortest bare token (hostname/email/uuid) worth anchoring on. Prevents noise
/// like `a.b` from forcing rows to stay.
const MIN_TOKEN_LEN: usize = 4;
/// A run of this many consecutive digits (or more) is a distinctive numeric id.
const MIN_DIGIT_RUN: usize = 4;

/// Extract exact-match anchor tokens from `query`.
///
/// Returns lower-cased, de-duplicated tokens: quoted phrases (verbatim, minus
/// the quotes), UUIDs, hostnames, emails, and 4+-digit numeric runs. An empty
/// or signal-free query yields an empty vector.
pub(crate) fn extract_anchors(query: &str) -> Vec<String> {
    let mut set: BTreeSet<String> = BTreeSet::new();

    // 1. Quoted phrases first — everything inside matching quotes is a literal
    //    the user asked for. Consume them so their inner tokens aren't also
    //    re-scanned as bare words.
    let residual = collect_quoted(query, &mut set);

    // 2. Whitespace tokens from what's left.
    for raw in residual.split_whitespace() {
        let tok = trim_punct(raw);
        if tok.is_empty() {
            continue;
        }
        if is_uuid(tok) {
            push(&mut set, tok);
            continue;
        }
        if is_email(tok) || is_hostname(tok) {
            if tok.len() >= MIN_TOKEN_LEN {
                push(&mut set, tok);
            }
            continue;
        }
        // 3. Distinctive numeric ids embedded anywhere in the token.
        for run in digit_runs(tok) {
            push(&mut set, &run);
        }
    }

    set.into_iter().collect()
}

/// Push a lower-cased anchor.
fn push(set: &mut BTreeSet<String>, tok: &str) {
    set.insert(tok.to_ascii_lowercase());
}

/// Extract `"..."` / `'...'` quoted spans into `set`, returning the query with
/// those spans blanked out so a second pass won't re-tokenize their contents.
fn collect_quoted(query: &str, set: &mut BTreeSet<String>) -> String {
    let mut out = String::with_capacity(query.len());
    let mut chars = query.char_indices().peekable();
    while let Some((_, c)) = chars.next() {
        if c == '"' || c == '\'' {
            let mut inner = String::new();
            let mut closed = false;
            for (_, d) in chars.by_ref() {
                if d == c {
                    closed = true;
                    break;
                }
                inner.push(d);
            }
            let trimmed = inner.trim();
            if closed && !trimmed.is_empty() {
                push(set, trimmed);
                out.push(' ');
            } else {
                // Unterminated quote: keep the text for bare-token scanning.
                out.push(' ');
                out.push_str(&inner);
            }
        } else {
            out.push(c);
        }
    }
    out
}

/// Strip leading/trailing punctuation that commonly wraps a token in prose.
fn trim_punct(tok: &str) -> &str {
    tok.trim_matches(|c: char| {
        !c.is_ascii_alphanumeric() && !matches!(c, '.' | '-' | '_' | '@' | ':' | '/')
    })
    .trim_matches(|c: char| matches!(c, ':' | '/'))
}

/// Canonical 8-4-4-4-12 hex UUID (case-insensitive).
fn is_uuid(tok: &str) -> bool {
    let b = tok.as_bytes();
    if b.len() != 36 {
        return false;
    }
    for (i, &c) in b.iter().enumerate() {
        match i {
            8 | 13 | 18 | 23 => {
                if c != b'-' {
                    return false;
                }
            }
            _ => {
                if !c.is_ascii_hexdigit() {
                    return false;
                }
            }
        }
    }
    true
}

/// `local@domain` where the domain looks like a hostname.
fn is_email(tok: &str) -> bool {
    let mut parts = tok.splitn(2, '@');
    let (Some(local), Some(domain)) = (parts.next(), parts.next()) else {
        return false;
    };
    !local.is_empty() && !local.contains('@') && is_hostname(domain)
}

/// Dotted labels with an alphabetic TLD-like last segment (`api.example.com`).
/// Rejects decimals (`3.14`) and version-ish all-numeric dotted tokens.
fn is_hostname(tok: &str) -> bool {
    if tok.contains('@') || tok.contains(' ') {
        return false;
    }
    let labels: Vec<&str> = tok.split('.').collect();
    if labels.len() < 2 {
        return false;
    }
    let mut saw_alpha = false;
    for (i, label) in labels.iter().enumerate() {
        if label.is_empty() {
            return false;
        }
        if !label
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || b == b'-')
        {
            return false;
        }
        if label.bytes().any(|b| b.is_ascii_alphabetic()) {
            saw_alpha = true;
        }
        // Last label (the TLD) must be all-alphabetic and ≥ 2 chars.
        if i == labels.len() - 1
            && (label.len() < 2 || !label.bytes().all(|b| b.is_ascii_alphabetic()))
        {
            return false;
        }
    }
    saw_alpha
}

/// Every maximal run of `MIN_DIGIT_RUN`+ consecutive ASCII digits in `tok`.
fn digit_runs(tok: &str) -> Vec<String> {
    let mut runs = Vec::new();
    let mut cur = String::new();
    for c in tok.chars() {
        if c.is_ascii_digit() {
            cur.push(c);
        } else {
            if cur.len() >= MIN_DIGIT_RUN {
                runs.push(std::mem::take(&mut cur));
            } else {
                cur.clear();
            }
        }
    }
    if cur.len() >= MIN_DIGIT_RUN {
        runs.push(cur);
    }
    runs
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extracts_uuid() {
        let a = extract_anchors("find migration 550e8400-e29b-41d4-a716-446655440000 please");
        assert!(a.contains(&"550e8400-e29b-41d4-a716-446655440000".to_string()));
    }

    #[test]
    fn extracts_quoted_phrase_verbatim() {
        let a = extract_anchors(r#"why did "CONFLICT_PACKAGES" appear"#);
        assert!(a.contains(&"conflict_packages".to_string()));
    }

    #[test]
    fn extracts_long_numeric_id() {
        let a = extract_anchors("show market 12345 and order 42");
        assert!(a.contains(&"12345".to_string()), "{a:?}");
        // 2-digit `42` is too short to anchor.
        assert!(!a.contains(&"42".to_string()), "{a:?}");
    }

    #[test]
    fn extracts_host_and_email() {
        let a = extract_anchors("errors from api.example.com for user bob@corp.io");
        assert!(a.contains(&"api.example.com".to_string()), "{a:?}");
        assert!(a.contains(&"bob@corp.io".to_string()), "{a:?}");
    }

    #[test]
    fn ignores_plain_words_and_decimals() {
        let a = extract_anchors("show me the latency 3.14 average value");
        assert!(a.is_empty(), "{a:?}");
    }

    #[test]
    fn empty_query_is_empty() {
        assert!(extract_anchors("").is_empty());
        assert!(extract_anchors("   ").is_empty());
    }
}
