//! Shared data for REPL-style output inspection.

use serde::{Deserialize, Serialize};

/// Unit of a Python-style scope slice.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ScopeUnit {
    /// Count lines (the default).
    #[default]
    Lines,
    /// Count Unicode characters.
    Chars,
}

/// Caps applied to every op so a query can never return the whole payload.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct ReplLimits {
    pub max_hits: usize,
    /// Cap for line-producing modes that select ranges (`sed`, `awk`).
    pub max_lines: usize,
    pub max_output_chars: usize,
    pub max_line_chars: usize,
    pub regex_size_limit: usize,
}

impl Default for ReplLimits {
    fn default() -> Self {
        Self {
            max_hits: 50,
            max_lines: 400,
            max_output_chars: 8_000,
            max_line_chars: 240,
            regex_size_limit: 1 << 20,
        }
    }
}

/// A located line. `line` is 1-based.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Hit {
    pub line: usize,
    pub text: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RegexMatch {
    pub line: usize,
    pub text: String,
    pub captures: Vec<Option<String>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SearchHit {
    pub line_start: usize,
    pub line_end: usize,
    pub score: f32,
    pub text: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Heading {
    pub level: u8,
    pub text: String,
    pub line: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Link {
    pub text: String,
    pub href: String,
}

/// How `find` reads its query.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FindMode {
    /// Case-insensitive substring.
    #[default]
    Text,
    /// Regex per line (optionally case-insensitive, with context lines).
    Grep,
    /// Regex per line, returning capture groups.
    Regex,
    /// BM25-ranked windows of lines.
    Rank,
    /// sed subset: `-n 10,20p`, `/re/d`, `s/a/b/g`.
    Sed,
    /// awk subset: `-F, '$3 > 5 { print $1, $NF }'`.
    Awk,
    /// jq filter over JSON output: `.items[] | select(.size > 10) | .name`.
    Jq,
}

/// What `extract` pulls out of HTML or Markdown.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExtractKind {
    Links,
    Headings,
}

/// An op over a stored original.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "op", rename_all = "snake_case")]
pub enum ReplOp {
    Find {
        query: String,
        #[serde(default)]
        mode: FindMode,
        #[serde(default)]
        ignore_case: bool,
        #[serde(default)]
        context: usize,
        #[serde(default)]
        top_k: Option<usize>,
        /// Python-style slice to search within, e.g. `[:-100]`.
        #[serde(default)]
        scope: Option<String>,
        #[serde(default)]
        unit: ScopeUnit,
    },
    Extract {
        what: ExtractKind,
        #[serde(default)]
        scope: Option<String>,
        #[serde(default)]
        unit: ScopeUnit,
    },
    Summarize {
        #[serde(default)]
        max_chars: Option<usize>,
        /// What the caller cares about; ranks which parts of the scope to keep.
        #[serde(default)]
        hint: Option<String>,
        #[serde(default)]
        scope: Option<String>,
        #[serde(default)]
        unit: ScopeUnit,
    },
}

/// Result of an op. `truncated` counts hits dropped by a cap.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum ReplOutput {
    Lines {
        hits: Vec<Hit>,
        truncated: usize,
    },
    Matches {
        matches: Vec<RegexMatch>,
        truncated: usize,
    },
    Search {
        hits: Vec<SearchHit>,
        truncated: usize,
    },
    Links {
        links: Vec<Link>,
        truncated: usize,
    },
    Headings {
        headings: Vec<Heading>,
        truncated: usize,
    },
    Values {
        values: Vec<String>,
        truncated: usize,
    },
    Text {
        text: String,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum ReplError {
    #[error("handle not found or expired; re-run the original tool")]
    HandleNotFound,
    #[error("invalid pattern: {0}")]
    InvalidPattern(String),
    #[error("empty query")]
    EmptyQuery,
    #[error("{0} support is not compiled in")]
    Unsupported(&'static str),
}
