//! Source-code compressor — keep signatures, collapse bodies.
//!
//! Inspired by Headroom's `CodeCompressor`. The goal is to keep the structural
//! skeleton an agent needs to navigate a file — imports, type/function/class
//! signatures, top-level constants — while collapsing the deep bodies that
//! dominate byte count.
//!
//! This module currently ships the language-agnostic **brace-depth heuristic**:
//! lines at brace nesting depth 0–1 are kept; deeper bodies collapse to a
//! `{ … N lines … }` placeholder. Lines carrying error/TODO markers are always
//! kept. A higher-fidelity tree-sitter path (Rust/TS/Python) is layered on in a
//! follow-up slice and selected by language; until then every language uses the
//! heuristic. The router offloads the original to CCR for exact recovery.

use async_trait::async_trait;
use std::fmt::Write as _;

use super::Compressor;
use crate::cache;
use crate::types::{CompressInput, CompressOptions, CompressOutput, CompressorKind};

/// Bodies with more than this many collapsed lines get a placeholder; shorter
/// ones are kept verbatim (collapsing tiny bodies isn't worth the marker).
pub const MIN_BODY_LINES_TO_COLLAPSE: usize = 4;

/// Markers that force a line to be kept even inside a deep body.
const KEEP_MARKERS: &[&str] = &["TODO", "FIXME", "XXX", "error", "panic", "unsafe"];

pub struct CodeCompressor;

#[async_trait]
impl Compressor for CodeCompressor {
    fn kind(&self) -> CompressorKind {
        CompressorKind::Code
    }

    async fn compress(
        &self,
        input: &CompressInput<'_>,
        opts: &CompressOptions,
    ) -> Option<CompressOutput> {
        // Per-body retrieval tokens only make sense when the CCR store is on.
        let per_body_tokens = opts.ccr_enabled;
        // Prefer the AST path when a grammar matches the file's language; fall
        // back to the language-agnostic heuristic otherwise (or when the AST
        // path doesn't shrink the content).
        #[cfg(feature = "tinyjuice-treesitter")]
        if let Some(ext) = input.hint.extension.as_deref()
            && let Some(out) = treesitter::compress(input.content, ext, per_body_tokens)
        {
            return Some(out);
        }
        compress_heuristic(input.content, per_body_tokens)
    }
}

/// One-line note appended when any collapsed body carries its own token, so
/// the model knows the placeholders are individually expandable.
const PER_BODY_NOTE: &str = "\n[collapsed bodies are individually retrievable: call tinyjuice_retrieve with the token inside a placeholder to expand just that body]";

/// Offload a collapsed body to CCR and return the placeholder token text
/// (e.g. `⟦tj:abc123⟧`), or an empty string when tokens are disabled or the
/// body couldn't be retained.
fn body_token(body: &str, enabled: bool) -> String {
    if !enabled {
        return String::new();
    }
    let (token, retained) = cache::offload_checked(body);
    if retained {
        format!(" {}", cache::format_marker(&token))
    } else {
        String::new()
    }
}

/// Language-agnostic brace-depth compressor. Keeps lines at depth ≤ 1, collapses
/// deeper runs. With `per_body_tokens`, each collapsed body is offloaded to CCR
/// and its placeholder carries a token that retrieves exactly that body.
/// Returns `None` if it wouldn't shrink the content.
pub fn compress_heuristic(content: &str, per_body_tokens: bool) -> Option<CompressOutput> {
    let lines: Vec<&str> = content.lines().collect();
    if lines.len() < 12 {
        return None;
    }

    let mut out = String::with_capacity(content.len() / 2 + 64);
    let mut depth: i32 = 0;
    let mut collapsed: Vec<&str> = Vec::new();
    let mut any_token = false;

    let flush = |out: &mut String, collapsed: &mut Vec<&str>, any_token: &mut bool| {
        if collapsed.is_empty() {
            return;
        }
        if collapsed.len() >= MIN_BODY_LINES_TO_COLLAPSE {
            let token = body_token(&collapsed.join("\n"), per_body_tokens);
            *any_token |= !token.is_empty();
            let _ = writeln!(out, "    {{ … {} line(s) …{token} }}", collapsed.len());
        } else {
            for l in collapsed.iter() {
                let _ = writeln!(out, "{l}");
            }
        }
        collapsed.clear();
    };

    for line in &lines {
        let (opens, closes) = brace_delta(line);
        let start_depth = depth;
        // A line that carries signal is always kept, regardless of depth.
        let force_keep = KEEP_MARKERS.iter().any(|m| line.contains(m));

        // Keep top-level lines (depth 0) — imports, signatures, the line that
        // opens a block — and collapse the block body (depth ≥ 1). Short bodies
        // (e.g. small struct field lists) stay verbatim via the flush threshold,
        // so struct/enum fields survive while long function bodies collapse.
        if start_depth == 0 || force_keep {
            flush(&mut out, &mut collapsed, &mut any_token);
            let _ = writeln!(out, "{line}");
        } else {
            collapsed.push(line);
        }
        depth += opens - closes;
        if depth < 0 {
            depth = 0;
        }
    }
    flush(&mut out, &mut collapsed, &mut any_token);

    let mut out = out.trim_end().to_string();
    if any_token {
        out.push_str(PER_BODY_NOTE);
    }
    if out.len() >= content.len() {
        return None;
    }
    log::debug!(
        "[tinyjuice][code] heuristic {} -> {} bytes ({} lines)",
        content.len(),
        out.len(),
        lines.len()
    );
    Some(CompressOutput::lossy(out, CompressorKind::Code))
}

/// Count `{`/`}` on a line, ignoring those inside string/char literals and
/// line comments — a cheap approximation good enough for the depth heuristic.
fn brace_delta(line: &str) -> (i32, i32) {
    // A char literal ('x', '\n', '{') closes within a few characters; a Rust
    // lifetime ('a in generics) or a stray apostrophe never does. Treating
    // every ' as a string opener would flip the rest of a lifetime-carrying
    // line into phantom string mode and stop counting its braces.
    const CHAR_LITERAL_WINDOW: usize = 10;

    let mut opens = 0i32;
    let mut closes = 0i32;
    let mut in_str: Option<char> = None;
    let mut prev = '\0';
    let mut iter = line.char_indices().peekable();
    while let Some((i, c)) = iter.next() {
        match in_str {
            Some(q) => {
                if c == q && prev != '\\' {
                    in_str = None;
                }
            }
            None => match c {
                '"' | '`' => in_str = Some(c),
                '\'' => {
                    let rest = &line[i + c.len_utf8()..];
                    let close = rest
                        .char_indices()
                        .take_while(|(off, _)| *off <= CHAR_LITERAL_WINDOW)
                        .find(|&(_, ch)| ch == '\'')
                        .map(|(off, _)| off);
                    if let Some(off) = close {
                        // Real char literal: skip past it wholesale so a
                        // brace inside (e.g. '{') isn't counted.
                        let end = i + c.len_utf8() + off;
                        while iter.next_if(|&(j, _)| j <= end).is_some() {}
                    }
                    // No close nearby: a lifetime/apostrophe — ignore it.
                }
                '/' if iter.peek().map(|&(_, ch)| ch) == Some('/') => break, // line comment
                '#' => break,                                                // python/shell comment
                '{' => opens += 1,
                '}' => closes += 1,
                _ => {}
            },
        }
        prev = c;
    }
    (opens, closes)
}

/// AST-aware code compression via tree-sitter (Rust/TS/JS/Python). Keeps full
/// source but replaces function/method bodies longer than a threshold with a
/// `{ … N lines … }` (or `...` for Python) placeholder, preserving signatures,
/// imports, type declarations and struct/enum fields exactly.
#[cfg(feature = "tinyjuice-treesitter")]
mod treesitter {
    use super::{
        CompressOutput, CompressorKind, MIN_BODY_LINES_TO_COLLAPSE, PER_BODY_NOTE, body_token,
    };
    use tree_sitter::{Node, Parser};

    /// Pick the grammar for a file extension. Returns the language plus whether
    /// it is brace-delimited (vs. Python's indentation suite).
    fn language_for(ext: &str) -> Option<(tree_sitter::Language, bool)> {
        let ext = ext.to_ascii_lowercase();
        match ext.as_str() {
            "rs" => Some((tree_sitter_rust::LANGUAGE.into(), true)),
            "ts" | "mts" | "cts" => {
                Some((tree_sitter_typescript::LANGUAGE_TYPESCRIPT.into(), true))
            }
            "tsx" => Some((tree_sitter_typescript::LANGUAGE_TSX.into(), true)),
            "js" | "jsx" | "mjs" | "cjs" => {
                // The TypeScript grammar is a superset that parses JS too.
                Some((tree_sitter_typescript::LANGUAGE_TYPESCRIPT.into(), true))
            }
            "py" | "pyi" => Some((tree_sitter_python::LANGUAGE.into(), false)),
            _ => None,
        }
    }

    /// Node kinds whose `body` field is a collapsible function/method body.
    const BODY_PARENTS: &[&str] = &[
        "function_item",
        "function_declaration",
        "function_definition",
        "method_definition",
        "function",
        "arrow_function",
        "generator_function_declaration",
    ];

    pub fn compress(content: &str, ext: &str, per_body_tokens: bool) -> Option<CompressOutput> {
        let (language, braced) = language_for(ext)?;
        let mut parser = Parser::new();
        parser.set_language(&language).ok()?;
        let tree = parser.parse(content, None)?;
        let src = content.as_bytes();

        // Collect outermost collapsible body byte-ranges.
        let mut ranges: Vec<(usize, usize)> = Vec::new();
        collect_bodies(tree.root_node(), src, &mut ranges);
        // Sort and drop nested ranges (keep outermost only).
        ranges.sort_by_key(|r| r.0);
        let mut merged: Vec<(usize, usize)> = Vec::new();
        for r in ranges {
            if let Some(last) = merged.last()
                && r.0 < last.1
            {
                continue; // nested inside a body we're already collapsing
            }
            merged.push(r);
        }
        if merged.is_empty() {
            return None;
        }

        let mut out = String::with_capacity(content.len());
        let mut cursor = 0usize;
        let mut any_token = false;
        for (start, end) in merged {
            if start < cursor {
                continue;
            }
            out.push_str(&content[cursor..start]);
            let body = &content[start..end];
            let n_lines = body.lines().count();
            if n_lines < MIN_BODY_LINES_TO_COLLAPSE {
                out.push_str(body);
            } else {
                let token = body_token(body, per_body_tokens);
                any_token |= !token.is_empty();
                if braced {
                    out.push_str(&format!("{{ … {n_lines} line(s) …{token} }}"));
                } else {
                    // Python suite — keep an indented ellipsis so it still reads.
                    out.push_str(&format!("...  # {n_lines} line(s) collapsed{token}"));
                }
            }
            cursor = end;
        }
        out.push_str(&content[cursor..]);

        let mut out = out.trim_end().to_string();
        if any_token {
            out.push_str(PER_BODY_NOTE);
        }
        if out.len() >= content.len() {
            return None;
        }
        log::debug!(
            "[tinyjuice][code] tree-sitter ext={} {} -> {} bytes",
            ext,
            content.len(),
            out.len()
        );
        Some(CompressOutput::lossy(out, CompressorKind::Code))
    }

    /// Recursively collect the byte-ranges of function/method bodies.
    fn collect_bodies(node: Node, src: &[u8], out: &mut Vec<(usize, usize)>) {
        if BODY_PARENTS.contains(&node.kind())
            && let Some(body) = node.child_by_field_name("body")
        {
            out.push((body.start_byte(), body.end_byte()));
            // Don't descend into a collapsed body.
            let _ = src;
            return;
        }
        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            collect_bodies(child, src, out);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn keeps_signatures_collapses_bodies() {
        let mut src = String::from("use std::collections::HashMap;\n\n");
        src.push_str("pub fn process(items: &[i32]) -> i32 {\n");
        for i in 0..30 {
            src.push_str(&format!(
                "        let tmp_{i} = items.iter().sum::<i32>() + {i};\n"
            ));
        }
        src.push_str("        tmp_0\n}\n\n");
        src.push_str("struct Config {\n    name: String,\n    size: usize,\n}\n");
        let out = compress_heuristic(&src, false).expect("compresses");
        assert!(out.lossy);
        assert!(
            out.text.contains("pub fn process"),
            "signature kept:\n{}",
            out.text
        );
        assert!(out.text.contains("struct Config"));
        assert!(
            out.text.contains("line(s) …"),
            "body collapsed:\n{}",
            out.text
        );
        assert!(
            !out.text.contains("tmp_15"),
            "deep body should be collapsed"
        );
        assert!(out.text.len() < src.len());
    }

    #[test]
    fn short_file_passes_through() {
        let src = "fn a() {}\nfn b() {}\n";
        assert!(compress_heuristic(src, false).is_none());
    }

    #[cfg(feature = "tinyjuice-treesitter")]
    #[test]
    fn treesitter_collapses_rust_body_keeps_struct() {
        let mut src = String::from("use std::collections::HashMap;\n\n");
        src.push_str("pub fn process(items: &[i32]) -> i32 {\n");
        for i in 0..30 {
            src.push_str(&format!(
                "    let tmp_{i} = items.iter().sum::<i32>() + {i};\n"
            ));
        }
        src.push_str("    tmp_0\n}\n\n");
        src.push_str("pub struct Config {\n    pub name: String,\n    pub size: usize,\n}\n");
        let out = treesitter::compress(&src, "rs", false).expect("compresses");
        assert!(
            out.text.contains("pub fn process(items: &[i32]) -> i32"),
            "{}",
            out.text
        );
        // Struct fields preserved exactly (not a function body).
        assert!(out.text.contains("pub name: String"), "{}", out.text);
        assert!(out.text.contains("pub size: usize"));
        // Function body collapsed.
        assert!(out.text.contains("line(s) …"), "{}", out.text);
        assert!(!out.text.contains("tmp_15"));
        assert!(out.text.len() < src.len());
    }

    #[cfg(feature = "tinyjuice-treesitter")]
    #[test]
    fn treesitter_collapses_python_body() {
        let mut src = String::from("import os\n\ndef handler(event):\n");
        for i in 0..30 {
            src.push_str(&format!("    x_{i} = compute(event, {i})\n"));
        }
        src.push_str("    return x_0\n");
        let out = treesitter::compress(&src, "py", false).expect("compresses");
        assert!(out.text.contains("def handler(event):"), "{}", out.text);
        assert!(out.text.contains("collapsed"), "{}", out.text);
        assert!(!out.text.contains("x_15"));
    }

    #[test]
    fn brace_delta_handles_lifetimes_and_char_literals() {
        // Lifetimes must not open phantom string mode.
        assert_eq!(brace_delta("pub fn f<'a>(x: &'a str) {"), (1, 0));
        assert_eq!(brace_delta("impl<'de> Deserialize<'de> for X {"), (1, 0));
        assert_eq!(
            brace_delta("fn g<'a, 'b>(x: &'a str, y: &'b str) -> &'a str {"),
            (1, 0)
        );
        // Real char literals still shield their contents.
        assert_eq!(brace_delta("let c = '{';"), (0, 0));
        assert_eq!(brace_delta("if c == '}' { close(); }"), (1, 1));
        // Strings still shield braces.
        assert_eq!(brace_delta(r#"let s = "{}"; f(|| {"#), (1, 0));
    }

    #[test]
    fn lifetime_heavy_rust_keeps_signatures() {
        let mut src = String::from("use serde::Deserialize;\n\n");
        src.push_str("impl<'de> Deserialize<'de> for Config {\n");
        for i in 0..20 {
            src.push_str(&format!("        let field_{i} = map.next_value()?;\n"));
        }
        src.push_str("}\n\n");
        src.push_str("pub fn lookup<'a>(map: &'a Map, key: &str) -> Option<&'a str> {\n");
        for i in 0..20 {
            src.push_str(&format!("        let probe_{i} = map.get(key);\n"));
        }
        src.push_str("}\n\n");
        src.push_str("pub struct Cursor<'a> {\n    buf: &'a [u8],\n    pos: usize,\n}\n");
        let out = compress_heuristic(&src, false).expect("compresses");
        // Both signatures after the first lifetime-carrying line must survive
        // at top level — depth drift used to collapse them.
        assert!(out.text.contains("pub fn lookup<'a>"), "{}", out.text);
        assert!(out.text.contains("pub struct Cursor<'a>"), "{}", out.text);
        assert!(
            !out.text.contains("probe_15"),
            "body collapsed: {}",
            out.text
        );
    }

    #[test]
    fn per_body_tokens_retrieve_exactly_the_collapsed_body() {
        let mut src = String::from("use std::io;\n\n");
        src.push_str("pub fn alpha() -> i32 {\n");
        for i in 0..10 {
            src.push_str(&format!("        let a_{i} = compute({i});\n"));
        }
        src.push_str("        a_0\n}\n\n");
        src.push_str("pub fn beta() -> i32 {\n");
        for i in 0..10 {
            src.push_str(&format!("        let b_{i} = compute({i});\n"));
        }
        src.push_str("        b_0\n}\n");

        let out = compress_heuristic(&src, true).expect("compresses");
        let tokens = cache::parse_markers(&out.text);
        assert_eq!(
            tokens.len(),
            2,
            "one token per collapsed body: {}",
            out.text
        );
        assert!(
            out.text.contains("individually retrievable"),
            "{}",
            out.text
        );

        // Each token retrieves exactly its own body.
        let first = cache::retrieve(&tokens[0]).expect("stored");
        assert!(first.contains("let a_5 = compute(5);"), "{first}");
        assert!(!first.contains("b_5"), "bodies must not mix: {first}");
        let second = cache::retrieve(&tokens[1]).expect("stored");
        assert!(second.contains("let b_5 = compute(5);"), "{second}");
    }

    #[test]
    fn no_tokens_when_disabled() {
        let mut src = String::from("use std::io;\n\n");
        src.push_str("pub fn gamma() -> i32 {\n");
        for i in 0..12 {
            src.push_str(&format!("        let g_{i} = compute({i});\n"));
        }
        src.push_str("        g_0\n}\n");
        let out = compress_heuristic(&src, false).expect("compresses");
        assert!(cache::parse_markers(&out.text).is_empty(), "{}", out.text);
        assert!(!out.text.contains("individually retrievable"));
    }

    #[cfg(feature = "tinyjuice-treesitter")]
    #[test]
    fn treesitter_per_body_tokens_round_trip() {
        let mut src = String::from("pub fn delta(items: &[i32]) -> i32 {\n");
        for i in 0..12 {
            src.push_str(&format!(
                "    let d_{i} = items.iter().sum::<i32>() + {i};\n"
            ));
        }
        src.push_str("    d_0\n}\n");
        let out = treesitter::compress(&src, "rs", true).expect("compresses");
        let tokens = cache::parse_markers(&out.text);
        assert_eq!(tokens.len(), 1, "{}", out.text);
        let body = cache::retrieve(&tokens[0]).expect("stored");
        assert!(body.contains("let d_7"), "{body}");
    }

    #[test]
    fn keeps_marker_lines_in_body() {
        let mut src = String::from("fn f() {\n");
        for i in 0..20 {
            if i == 10 {
                src.push_str("        // TODO: handle the edge case here\n");
            } else {
                src.push_str(&format!("        do_thing({i});\n"));
            }
        }
        src.push_str("}\n");
        let out = compress_heuristic(&src, false).expect("compresses");
        assert!(out.text.contains("TODO"), "marker line kept:\n{}", out.text);
    }
}
