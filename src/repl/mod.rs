//! REPL-style inspection of a stored tool output.
//!
//! Instead of handing the host one compressed blob, the full original sits in a
//! [`CcrStore`] under a handle and the host drills in with small deterministic
//! ops (grep, find, regex, search, links, headings, summarize, slice). Nothing
//! here calls a model, and content and patterns are never logged.

pub mod awk;
#[cfg(feature = "jq")]
pub mod jq;
pub mod ops;
pub mod scope;
pub mod sed;
pub mod stats;
pub mod types;

#[cfg(feature = "tinytools")]
pub mod tools;

#[cfg(test)]
#[path = "mod_tests.rs"]
mod test;
#[cfg(test)]
mod test_stats;

use crate::cache::store::CcrStore;
pub use types::{
    ExtractKind, FindMode, Heading, Hit, Link, RegexMatch, ReplError, ReplLimits, ReplOp,
    ReplOutput, ScopeUnit, SearchHit,
};

const DEFAULT_TOP_K: usize = 5;
const DEFAULT_SUMMARY_CHARS: usize = 2_000;

/// Resolve `handle` in `store` and run `op` against the original.
pub fn run_op(
    store: &dyn CcrStore,
    handle: &str,
    op: &ReplOp,
    limits: &ReplLimits,
) -> Result<ReplOutput, ReplError> {
    let text = store.get(handle).ok_or(ReplError::HandleNotFound)?;
    run_on_text(&text, op, limits)
}

/// Run `op` directly on text (used when the caller already holds the original).
/// The op's `scope`, if any, narrows the text first; line numbers in the result
/// still refer to the original.
pub fn run_on_text(text: &str, op: &ReplOp, limits: &ReplLimits) -> Result<ReplOutput, ReplError> {
    let (scope, unit) = match op {
        ReplOp::Find { scope, unit, .. }
        | ReplOp::Extract { scope, unit, .. }
        | ReplOp::Summarize { scope, unit, .. } => (scope.as_deref(), *unit),
    };
    match scope {
        Some(spec) => {
            let (scoped, offset) = scope::apply(text, spec, unit)?;
            let mut out = run_scoped(scoped, op, limits)?;
            shift_lines(&mut out, offset);
            Ok(cap(out, limits))
        }
        None => Ok(cap(run_scoped(text, op, limits)?, limits)),
    }
}

fn shift_lines(out: &mut ReplOutput, by: usize) {
    if by == 0 {
        return;
    }
    match out {
        ReplOutput::Lines { hits, .. } => hits.iter_mut().for_each(|h| h.line += by),
        ReplOutput::Matches { matches, .. } => matches.iter_mut().for_each(|m| m.line += by),
        ReplOutput::Search { hits, .. } => hits.iter_mut().for_each(|h| {
            h.line_start += by;
            h.line_end += by;
        }),
        ReplOutput::Headings { headings, .. } => headings.iter_mut().for_each(|h| h.line += by),
        ReplOutput::Links { .. } | ReplOutput::Values { .. } | ReplOutput::Text { .. } => {}
    }
}

fn run_scoped(text: &str, op: &ReplOp, limits: &ReplLimits) -> Result<ReplOutput, ReplError> {
    let out = match op {
        ReplOp::Find {
            query,
            mode,
            ignore_case,
            context,
            top_k,
            ..
        } => match mode {
            FindMode::Text => {
                let (hits, truncated) = ops::find(text, query, limits)?;
                ReplOutput::Lines { hits, truncated }
            }
            FindMode::Grep => {
                let (hits, truncated) =
                    ops::grep(text, query, false, *ignore_case, *context, limits)?;
                ReplOutput::Lines { hits, truncated }
            }
            FindMode::Regex => {
                let (matches, truncated) = ops::regex_matches(text, query, limits)?;
                ReplOutput::Matches { matches, truncated }
            }
            FindMode::Rank => {
                let (hits, truncated) =
                    ops::search(text, query, top_k.unwrap_or(DEFAULT_TOP_K), limits)?;
                ReplOutput::Search { hits, truncated }
            }
            FindMode::Sed => {
                let (script, quiet) = split_flag(query, "-n");
                let (hits, truncated) = sed::run(text, script, quiet, limits)?;
                ReplOutput::Lines { hits, truncated }
            }
            FindMode::Awk => {
                let (hits, truncated) = awk::run(text, query, limits)?;
                ReplOutput::Lines { hits, truncated }
            }
            FindMode::Jq => run_jq(text, query, limits)?,
        },
        ReplOp::Extract {
            what: ExtractKind::Links,
            ..
        } => {
            let (links, truncated) = ops::extract_links(text, limits);
            ReplOutput::Links { links, truncated }
        }
        ReplOp::Extract {
            what: ExtractKind::Headings,
            ..
        } => {
            let (headings, truncated) = ops::extract_headings(text, limits);
            ReplOutput::Headings {
                headings,
                truncated,
            }
        }
        ReplOp::Summarize {
            max_chars, hint, ..
        } => {
            // Bound the working budget by the output cap, and clip the finished
            // summary to it: the size line and outline are written before budgeting.
            let budget = max_chars
                .unwrap_or(DEFAULT_SUMMARY_CHARS)
                .min(limits.max_output_chars);
            let mut summary = ops::summarize_with_hint(text, hint.as_deref(), budget, limits);
            if let Some((cut, _)) = summary.char_indices().nth(budget) {
                summary.truncate(cut);
            }
            ReplOutput::Text { text: summary }
        }
    };
    Ok(out)
}

/// Strip a leading `flag ` (sed's `-n`) from a script.
fn split_flag<'a>(script: &'a str, flag: &str) -> (&'a str, bool) {
    let t = script.trim_start();
    match t.strip_prefix(flag) {
        Some(rest) if rest.starts_with(char::is_whitespace) => (rest.trim_start(), true),
        _ => (script, false),
    }
}

#[cfg(feature = "jq")]
fn run_jq(text: &str, query: &str, limits: &ReplLimits) -> Result<ReplOutput, ReplError> {
    let (values, truncated) = jq::run(text, query, limits)?;
    Ok(ReplOutput::Values { values, truncated })
}

#[cfg(not(feature = "jq"))]
fn run_jq(_: &str, _: &str, _: &ReplLimits) -> Result<ReplOutput, ReplError> {
    Err(ReplError::Unsupported("jq"))
}

fn clip_chars(s: &str, max: usize) -> String {
    match s.char_indices().nth(max) {
        Some((cut, _)) => format!("{}…", &s[..cut]),
        None => s.to_string(),
    }
}

/// Drop trailing hits until the serialized result fits `max_output_chars`.
fn cap(mut out: ReplOutput, limits: &ReplLimits) -> ReplOutput {
    fn fit<T: serde::Serialize>(items: &mut Vec<T>, truncated: &mut usize, max: usize) {
        while items.len() > 1 && serde_json::to_string(items).map_or(0, |s| s.chars().count()) > max
        {
            items.pop();
            *truncated += 1;
        }
    }
    let max = limits.max_output_chars;
    match &mut out {
        ReplOutput::Lines { hits, truncated } => {
            fit(hits, truncated, max);
            for h in hits.iter_mut() {
                h.text = clip_chars(&h.text, max / 2);
            }
        }
        ReplOutput::Matches { matches, truncated } => {
            fit(matches, truncated, max);
            // Every capture is clipped already, but many captures can still
            // exceed the cap in one match.
            for m in matches.iter_mut() {
                m.text = clip_chars(&m.text, max / 4);
                let mut spent = 0usize;
                m.captures.retain(|c| {
                    spent = spent.saturating_add(c.as_ref().map_or(0, |s| s.chars().count()) + 4);
                    spent <= max / 2
                });
            }
        }
        ReplOutput::Search { hits, truncated } => {
            fit(hits, truncated, max);
            for h in hits.iter_mut() {
                h.text = clip_chars(&h.text, max / 2);
            }
        }
        ReplOutput::Values { values, truncated } => {
            fit(values, truncated, max);
            // One value can exceed the cap alone; clip it rather than keep it whole.
            for v in values.iter_mut() {
                *v = clip_chars(v, max);
            }
        }
        ReplOutput::Links { links, truncated } => {
            fit(links, truncated, max);
            for l in links.iter_mut() {
                l.text = clip_chars(&l.text, max / 4);
                l.href = clip_chars(&l.href, max / 2);
            }
        }
        ReplOutput::Headings {
            headings,
            truncated,
        } => {
            fit(headings, truncated, max);
            for h in headings.iter_mut() {
                h.text = clip_chars(&h.text, max / 2);
            }
        }
        ReplOutput::Text { text } => {
            if text.chars().count() > max {
                *text =
                    text.chars().take(max).collect::<String>() + "\n…[truncated; narrow the range]";
            }
        }
    }
    out
}

/// Tool names advertised in the handle footer.
pub const TOOL_NAMES: &[&str] = &["juice_find", "juice_extract", "juice_summarize"];

/// Write the full original to `<dir>/<token>.txt` (0600, atomic, idempotent) so an
/// agent can scrape it with its own program. Best-effort: `None` on any failure.
pub fn write_handle_file(
    dir: &std::path::Path,
    token: &str,
    content: &str,
) -> Option<std::path::PathBuf> {
    if token.is_empty() || !token.chars().all(|c| c.is_ascii_alphanumeric()) {
        return None;
    }
    std::fs::create_dir_all(dir).ok()?;
    let path = dir.join(format!("{token}.txt"));
    if path.is_file() {
        return Some(path);
    }
    let tmp = dir.join(format!(".{token}.{}.tmp", std::process::id()));
    let write = || -> std::io::Result<()> {
        use std::io::Write;
        let mut opts = std::fs::OpenOptions::new();
        opts.write(true).create(true).truncate(true);
        #[cfg(unix)]
        {
            use std::os::unix::fs::OpenOptionsExt;
            opts.mode(0o600);
        }
        opts.open(&tmp)?.write_all(content.as_bytes())?;
        std::fs::rename(&tmp, &path)
    };
    if write().is_err() {
        let _ = std::fs::remove_file(&tmp);
        return None;
    }
    Some(path)
}

/// Stats line, head snippet, preview and footer for a stored original. The footer
/// keeps the `tinyjuice_retrieve ... token "<hash>"` form so marker parsing still
/// finds the handle. Returns `(body, footer, stats)`; the body starts with the stats line.
pub fn handle_view(
    content: &str,
    token: &str,
    kind: crate::types::ContentKind,
    chars_per_token: f32,
    preview_chars: usize,
    file: Option<&std::path::Path>,
) -> (String, String, String) {
    let stats = stats::describe(content, kind, chars_per_token);
    let mut preview = ops::summarize(content, preview_chars, &ReplLimits::default());
    // The summary writes its size line and outline before budgeting, so bound
    // the whole preview here.
    if let Some((cut, _)) = preview.char_indices().nth(preview_chars) {
        preview.truncate(cut);
    }
    let body = format!(
        "[{stats}]\nhead (first {} chars):\n{}\n---\n{preview}",
        stats::HEAD_CHARS,
        stats::head(content),
    );
    let file_note = file
        .map(|p| format!(" Plain-text copy for scripts: {}.", p.display()))
        .unwrap_or_default();
    let footer = format!(
        "\n\n[full output is stored, not shown. Inspect it with {} using handle \"{token}\"; \
         or call {} with token \"{token}\" for the whole original.{file_note}]",
        TOOL_NAMES.join(" / "),
        crate::cache::marker::RETRIEVE_TOOL_NAME,
    );
    (body, footer, stats)
}
