//! Rule compilation: converts a `JsonRule` descriptor into a `CompiledRule`
//! with pre-built `regex::Regex` instances.
//!
//! Invalid regex patterns produce a non-fatal diagnostic log and are silently
//! dropped so a bad user rule does not crash the engine.

use crate::types::{
    CompiledCounter, CompiledOutputMatch, CompiledParts, CompiledRule, JsonRule, RuleOrigin,
};

// ---------------------------------------------------------------------------
// Regex helpers
// ---------------------------------------------------------------------------

/// Build regex flags ensuring `u` (Unicode) is always present.
///
/// Upstream uses `new RegExp(pattern, mergeRegexFlags(flags))` where `u` is
/// always prepended.  In Rust's `regex` crate there is no separate `u` flag —
/// Unicode is on by default — so we translate only `i` (case-insensitive) and
/// `m` (multiline).
fn build_regex(pattern: &str, flags: Option<&str>) -> Option<regex::Regex> {
    let case_insensitive = flags.map(|f| f.contains('i')).unwrap_or(false);
    let multiline = flags.map(|f| f.contains('m')).unwrap_or(false);

    // Build pattern with inline flags
    let prefix = match (case_insensitive, multiline) {
        (true, true) => "(?im)",
        (true, false) => "(?i)",
        (false, true) => "(?m)",
        (false, false) => "",
    };
    let full = format!("{}{}", prefix, pattern);

    match regex::Regex::new(&full) {
        Ok(re) => Some(re),
        Err(err) => {
            log::debug!(
                "[tinyjuice] rule compiler: invalid regex '{}' (flags={:?}): {}",
                pattern,
                flags,
                err
            );
            None
        }
    }
}

// ---------------------------------------------------------------------------
// compile_rule
// ---------------------------------------------------------------------------

/// Compile a `JsonRule` into a `CompiledRule`.
///
/// `path` is either a filesystem path or `"builtin:<id>"` for embedded rules.
pub fn compile_rule(rule: JsonRule, source: RuleOrigin, path: String) -> CompiledRule {
    log::debug!(
        "[tinyjuice] compiling rule '{}' from {:?} path={}",
        rule.id,
        source,
        path
    );

    let skip_patterns: Vec<regex::Regex> = rule
        .filters
        .as_ref()
        .and_then(|f| f.skip_patterns.as_ref())
        .map(|pats| pats.iter().filter_map(|p| build_regex(p, None)).collect())
        .unwrap_or_default();

    let keep_patterns: Vec<regex::Regex> = rule
        .filters
        .as_ref()
        .and_then(|f| f.keep_patterns.as_ref())
        .map(|pats| pats.iter().filter_map(|p| build_regex(p, None)).collect())
        .unwrap_or_default();

    let counters: Vec<CompiledCounter> = rule
        .counters
        .as_ref()
        .map(|counters| {
            counters
                .iter()
                .filter_map(|c| {
                    build_regex(&c.pattern, c.flags.as_deref()).map(|re| CompiledCounter {
                        name: c.name.clone(),
                        pattern: re,
                    })
                })
                .collect()
        })
        .unwrap_or_default();

    let output_matches: Vec<CompiledOutputMatch> = rule
        .match_output
        .as_ref()
        .map(|entries| {
            entries
                .iter()
                .filter_map(|entry| {
                    build_regex(&entry.pattern, entry.flags.as_deref()).map(|re| {
                        CompiledOutputMatch {
                            pattern: re,
                            message: entry.message.clone(),
                        }
                    })
                })
                .collect()
        })
        .unwrap_or_default();

    CompiledRule {
        compiled: CompiledParts {
            skip_patterns,
            keep_patterns,
            counters,
            output_matches,
        },
        rule,
        source,
        path,
    }
}

#[cfg(test)]
#[path = "compiler_tests.rs"]
mod tests;
