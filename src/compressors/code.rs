//! Source-code compressor — keep the declaration skeleton, collapse bodies.
//!
//! Inspired by Headroom's `CodeCompressor`. The goal is to keep the structural
//! skeleton an agent needs to navigate a file — imports, type/function/class
//! signatures, top-level constants, and (crucially) the member signatures of
//! containers — while collapsing the deep executable bodies that dominate byte
//! count.
//!
//! This module ships the language-agnostic **declaration-skeleton heuristic**:
//! it walks the file tracking a stack of open blocks, remembering for each
//! whether it is a *container* (class/struct/impl/trait/interface/enum/
//! namespace/module/extern) or a *function/other* block. A line is kept when it
//! is at top level, when it is a block-opening signature at any depth, or when
//! it is a member sitting directly inside a container (field, const, method
//! signature, doc comment). Free-function bodies collapse to a
//! `{ … N line(s) … }` placeholder; large bodies keep a head/tail skeleton so a
//! few real lines survive around the elision. A higher-fidelity tree-sitter path
//! (Rust/TS/JS/Python) is layered on and selected by language; until then every
//! language uses the heuristic. The router offloads each omitted block to CCR
//! for exact recovery.

use async_trait::async_trait;
use std::fmt::Write as _;

use super::{BLOCK_NOTE, Compressor, block_token};
use crate::types::{CompressInput, CompressOptions, CompressOutput, CompressorKind};

/// A body must have at least this many *interior* lines (not counting the brace
/// lines) before it is worth replacing with a placeholder. Small bodies stay
/// verbatim — the marker (~45 bytes) can be longer than the code it replaces.
pub const MIN_BODY_LINES_TO_COLLAPSE: usize = 8;

/// Once a collapsed body has at least this many interior lines, keep a skeleton
/// (first two + last one interior lines) around the elided middle instead of a
/// single marker, so the reader still sees how the body opens and returns.
const SKELETON_MIN_INTERIOR: usize = 12;

/// A collapse must save at least this many bytes over the placeholder marker,
/// otherwise it is a no-op inflation and the body is kept verbatim.
const MIN_BYTE_SAVINGS: usize = 16;

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
        // A query token in a function name or body pins that body open so the
        // caller can still read the code they were looking for.
        let query = input.hint.query.as_deref();
        // Prefer the AST path when a grammar matches the file's language; fall
        // back to the language-agnostic heuristic otherwise (or when the AST
        // path doesn't shrink the content).
        #[cfg(feature = "tinyjuice-treesitter")]
        if let Some(ext) = input.hint.extension.as_deref()
            && let Some(out) = treesitter::compress(input.content, ext, per_body_tokens, query)
        {
            return Some(out);
        }
        compress_heuristic(input.content, per_body_tokens, query)
    }
}

/// Language-agnostic declaration-skeleton compressor. Keeps top-level lines,
/// block-opening signatures at any depth, and members sitting directly inside a
/// container; collapses executable bodies. With `per_body_tokens`, each
/// collapsed body is offloaded to CCR and its placeholder carries a token that
/// retrieves exactly that body. `query` (when set) pins any body whose name or
/// text mentions a query token. Returns `None` if it wouldn't shrink the content.
pub fn compress_heuristic(
    content: &str,
    per_body_tokens: bool,
    query: Option<&str>,
) -> Option<CompressOutput> {
    let lines: Vec<&str> = content.lines().collect();
    if lines.len() < 12 {
        return None;
    }

    let mut out = String::with_capacity(content.len() / 2 + 64);
    // Each entry records whether the open block is a container (true) or a
    // function/other block (false). Stack length is the brace-nesting depth.
    let mut stack: Vec<bool> = Vec::new();
    // Classification for the next `{` that opens on its own line (next-line
    // brace style, e.g. PHP/C#): set from the preceding declaration line.
    let mut pending_container: Option<bool> = None;
    let mut collapsed: Vec<&str> = Vec::new();
    // The kept line that opened the body currently accumulating in `collapsed`.
    let mut body_sig: Option<&str> = None;
    let mut any_token = false;
    let mut in_block_comment = false;

    for line in &lines {
        let was_in_comment = in_block_comment;
        let (opens, closes) = brace_delta(line, &mut in_block_comment);
        let t = line.trim();
        let start_depth = stack.len();
        let enclosing_container = *stack.last().unwrap_or(&false);

        // Blank lines: at container/top level they separate members and are
        // kept; inside a body they stay glued to the body so it collapses as one.
        if t.is_empty() && !was_in_comment {
            if start_depth == 0 || enclosing_container {
                emit_body(
                    &mut out,
                    &collapsed,
                    per_body_tokens,
                    &mut any_token,
                    body_sig,
                    query,
                );
                collapsed.clear();
                let _ = writeln!(out, "{line}");
            } else {
                collapsed.push(line);
            }
            continue;
        }

        let decl_opener = !was_in_comment && opens > closes && is_decl_opener_line(t);
        let keep = start_depth == 0 || decl_opener || (enclosing_container && is_member_line(t));

        if keep {
            emit_body(
                &mut out,
                &collapsed,
                per_body_tokens,
                &mut any_token,
                body_sig,
                query,
            );
            collapsed.clear();
            let _ = writeln!(out, "{line}");
            // This kept line is the signature for whatever body follows.
            body_sig = Some(line);
        } else {
            collapsed.push(line);
        }

        // Update the block stack. Signature/declaration lines that carry no
        // brace arm `pending_container` for the `{` that opens next line.
        if was_in_comment {
            // Depth is tracked by brace_delta's comment state; nothing to push.
        } else if opens > closes {
            let n = opens - closes;
            let is_cont = if is_decl_opener_line(t) {
                is_container_line(t)
            } else {
                pending_container.unwrap_or(false)
            };
            for k in 0..n {
                stack.push(k == 0 && is_cont);
            }
            pending_container = None;
        } else if closes > opens {
            for _ in 0..(closes - opens) {
                stack.pop();
            }
            pending_container = None;
        } else if opens == 0 {
            // No braces on this line: it may be a declaration whose body brace
            // lands on the next line.
            if is_container_line(t) {
                pending_container = Some(true);
            } else if is_signature_line(t) {
                pending_container = Some(false);
            }
        } else {
            // Balanced braces (one-liner block): depth unchanged.
            pending_container = None;
        }
    }
    emit_body(
        &mut out,
        &collapsed,
        per_body_tokens,
        &mut any_token,
        body_sig,
        query,
    );

    let mut out = out.trim_end().to_string();
    if any_token {
        out.push_str(BLOCK_NOTE);
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

/// Emit an accumulated body run: verbatim when it is short, worth less than the
/// marker, or pinned by `keep_body`; otherwise a placeholder. Large bodies keep
/// a head/tail skeleton around the elided middle. `sig` is the signature line
/// that opened the body (used to read its name for the keep heuristics).
fn emit_body(
    out: &mut String,
    collapsed: &[&str],
    per_body_tokens: bool,
    any_token: &mut bool,
    sig: Option<&str>,
    query: Option<&str>,
) {
    if collapsed.is_empty() {
        return;
    }

    // Interior = the run minus a leading bare `{` and a trailing `}` line.
    let lo = usize::from(brace_open_only(collapsed[0].trim()));
    let mut hi = collapsed.len();
    if hi > lo && brace_close_only(collapsed[hi - 1].trim()) {
        hi -= 1;
    }
    let interior = &collapsed[lo..hi];
    let interior_count = interior.len();

    let name = sig.and_then(fn_name);
    let body_text = collapsed.join("\n");

    let collapse = interior_count >= MIN_BODY_LINES_TO_COLLAPSE
        && !keep_body(name.as_deref(), &body_text, interior_count, query);

    if collapse {
        let indent = leading_ws(interior.first().copied().unwrap_or(collapsed[0]));
        // The token recovers the FULL original body regardless of skeleton.
        let token = block_token(&body_text, per_body_tokens);

        if interior_count >= SKELETON_MIN_INTERIOR {
            let elided = interior_count - 3;
            let marker = format!("{indent}{{ … {elided} line(s) …{token} }}");
            let removed: usize = interior[2..interior_count - 1]
                .iter()
                .map(|l| l.len() + 1)
                .sum();
            if removed > marker.len() + MIN_BYTE_SAVINGS {
                *any_token |= !token.is_empty();
                let _ = writeln!(out, "{}", interior[0]);
                let _ = writeln!(out, "{}", interior[1]);
                let _ = writeln!(out, "{marker}");
                let _ = writeln!(out, "{}", interior[interior_count - 1]);
                return;
            }
        } else {
            let marker = format!("{indent}{{ … {} line(s) …{token} }}", collapsed.len());
            if body_text.len() > marker.len() + MIN_BYTE_SAVINGS {
                *any_token |= !token.is_empty();
                let _ = writeln!(out, "{marker}");
                return;
            }
        }
    }

    for l in collapsed {
        let _ = writeln!(out, "{l}");
    }
}

/// Leading-whitespace prefix of a line (for aligning placeholder markers).
fn leading_ws(s: &str) -> &str {
    &s[..s.len() - s.trim_start().len()]
}

/// A line that is nothing but an opening brace (next-line brace style).
fn brace_open_only(t: &str) -> bool {
    t == "{"
}

/// A line that is only closing punctuation: `}`, `};`, `},`, `})`, `});` …
fn brace_close_only(t: &str) -> bool {
    t.starts_with('}')
        && t.chars()
            .all(|c| matches!(c, '}' | ';' | ',' | ')' | ' ' | '\t'))
}

/// Keywords that introduce a *container* whose members we keep visible.
const CONTAINER_KW: &[&str] = &[
    "class",
    "struct",
    "impl",
    "trait",
    "interface",
    "enum",
    "namespace",
    "module",
    "mod",
    "record",
    "union",
    "protocol",
    "object",
    "extension",
];

/// Keywords that introduce a function/method body.
const FN_KW: &[&str] = &["fn", "func", "def", "function", "fun", "sub", "method"];

/// Leading modifiers/visibility keywords to skip before the "kind" token.
const MODIFIER_KW: &[&str] = &[
    "pub",
    "public",
    "private",
    "protected",
    "internal",
    "static",
    "final",
    "abstract",
    "override",
    "async",
    "export",
    "default",
    "virtual",
    "inline",
    "unsafe",
    "const",
    "extern",
    "sealed",
    "partial",
    "open",
    "suspend",
    "explicit",
    "readonly",
    "friend",
    "data",
    "mut",
];

/// Control-flow keywords whose `{` opens a block that is NOT a declaration.
const CONTROL_KW: &[&str] = &[
    "if",
    "else",
    "for",
    "while",
    "switch",
    "do",
    "try",
    "catch",
    "finally",
    "match",
    "loop",
    "foreach",
    "using",
    "lock",
    "fixed",
    "synchronized",
    "with",
    "when",
    "return",
    "elif",
    "except",
];

/// The token identifying a declaration's kind — the first word after any
/// leading visibility/modifier keywords. Trailing `{` and generics are ignored.
fn kind_token(t: &str) -> Option<&str> {
    let mut toks = t.split(|c: char| c.is_whitespace() || c == '(' || c == '<' || c == '{');
    for tok in toks.by_ref() {
        let w = tok.trim_end_matches(':');
        if w.is_empty() {
            continue;
        }
        if MODIFIER_KW.contains(&w) {
            continue;
        }
        return Some(w);
    }
    None
}

/// True when the line introduces a container (its members should stay visible).
fn is_container_line(t: &str) -> bool {
    if t.starts_with("extern") {
        return true;
    }
    kind_token(t).is_some_and(|w| CONTAINER_KW.contains(&w))
}

/// True when a line that ENDS with `{` is a declaration opener (a container or
/// a function/method signature) rather than a control-flow block.
fn is_decl_opener_line(t: &str) -> bool {
    let tr = t.trim_end();
    let head = match tr.strip_suffix('{') {
        Some(h) => h.trim_end(),
        None => return false,
    };
    if head.is_empty() {
        return false; // bare `{`
    }
    if is_container_line(head) {
        return true;
    }
    match kind_token(head) {
        Some(w) if CONTROL_KW.contains(&w) => false,
        Some(w) if FN_KW.contains(&w) => true,
        // `type name(params)` — Java/C#/C++/PHP method signature.
        _ => head.contains('(') && head.contains(')'),
    }
}

/// True when a line (carrying no brace) looks like a function/method signature
/// whose body brace lands on the next line.
fn is_signature_line(t: &str) -> bool {
    if !t.contains('(') {
        return false;
    }
    let tr = t.trim_end();
    // A trailing `;` is a statement or an abstract/interface method — no body.
    if tr.ends_with(';') || tr.ends_with(',') {
        return false;
    }
    !kind_token(t).is_some_and(|w| CONTROL_KW.contains(&w))
}

/// True when a line sitting directly inside a container is a member worth
/// keeping (field, const, doc comment, method signature) — anything that is not
/// a bare brace or empty.
fn is_member_line(t: &str) -> bool {
    !t.is_empty() && !brace_open_only(t) && !brace_close_only(t)
}

/// Extract the declared name from a signature line: the identifier immediately
/// before the first parameter list. Handles Go receiver methods.
fn fn_name(sig: &str) -> Option<String> {
    let s = sig.trim();
    // Strip a Go receiver: `func (r *T) Name(` -> `Name(`.
    let s = if let Some(rest) = s.strip_prefix("func ") {
        let rest = rest.trim_start();
        if rest.starts_with('(') {
            rest.find(')').map(|i| rest[i + 1..].trim()).unwrap_or(rest)
        } else {
            rest
        }
    } else {
        s
    };
    let paren = s.find('(')?;
    let before = &s[..paren];
    let name = before
        .rsplit(|c: char| !c.is_alphanumeric() && c != '_' && c != '$')
        .next()
        .unwrap_or("");
    if name.is_empty() {
        None
    } else {
        Some(name.to_string())
    }
}

/// Count word-boundary occurrences of `needle` inside the already-lowercased
/// `hay`. `needle` must itself be lowercase.
fn count_word(hay: &str, needle: &str) -> usize {
    let mut n = 0;
    let mut from = 0;
    while let Some(rel) = hay[from..].find(needle) {
        let start = from + rel;
        let end = start + needle.len();
        let before_ok = start == 0
            || !hay[..start]
                .chars()
                .next_back()
                .is_some_and(|c| c.is_alphanumeric() || c == '_');
        let after_ok = end >= hay.len()
            || !hay[end..]
                .chars()
                .next()
                .is_some_and(|c| c.is_alphanumeric() || c == '_');
        if before_ok && after_ok {
            n += 1;
        }
        from = end;
    }
    n
}

/// True when a body is error-handling dense: at least two of throw/raise/
/// panic!/return Err/catch/except/rescue.
fn is_error_dense(body: &str) -> bool {
    let l = body.to_ascii_lowercase();
    let mut hits = count_word(&l, "throw");
    hits += count_word(&l, "raise");
    hits += count_word(&l, "catch");
    hits += count_word(&l, "except");
    hits += count_word(&l, "rescue");
    // Punctuation-carrying / multi-word forms: plain substring counts.
    hits += l.matches("panic!").count();
    hits += l.matches("return err").count();
    hits >= 2
}

/// True when a function name or body mentions any (≥2-char) token from `query`.
fn query_hits(name: Option<&str>, body: &str, query: &str) -> bool {
    let body_l = body.to_ascii_lowercase();
    let name_l = name.map(|n| n.to_ascii_lowercase());
    query
        .split(|c: char| !c.is_alphanumeric() && c != '_')
        .filter(|tok| tok.len() >= 2)
        .any(|tok| {
            let tl = tok.to_ascii_lowercase();
            body_l.contains(&tl) || name_l.as_deref().is_some_and(|n| n.contains(&tl))
        })
}

/// Decide whether a body is important enough to keep verbatim rather than
/// collapse: constructors/entrypoints, short accessors, error-dense bodies, or
/// bodies matching the caller's query.
fn keep_body(name: Option<&str>, body: &str, interior: usize, query: Option<&str>) -> bool {
    if let Some(n) = name {
        let nl = n.to_ascii_lowercase();
        if matches!(
            nl.as_str(),
            "main" | "new" | "init" | "__init__" | "__construct" | "constructor"
        ) {
            return true;
        }
        let is_accessor = nl.starts_with("get")
            || nl.starts_with("set")
            || nl.starts_with("is")
            || nl.starts_with("has");
        if is_accessor && interior <= 6 {
            return true;
        }
    }
    if is_error_dense(body) {
        return true;
    }
    if let Some(q) = query
        && query_hits(name, body, q)
    {
        return true;
    }
    false
}

/// Count `{`/`}` on a line, ignoring those inside string/char literals,
/// line comments, and `/* … */` block comments (Javadoc is full of literal
/// braces via `{@link …}` — counting them drifts the depth for the rest of
/// the file). `in_block_comment` carries block-comment state across lines.
fn brace_delta(line: &str, in_block_comment: &mut bool) -> (usize, usize) {
    // A char literal ('x', '\n', '{') closes within a few characters; a Rust
    // lifetime ('a in generics) or a stray apostrophe never does. Treating
    // every ' as a string opener would flip the rest of a lifetime-carrying
    // line into phantom string mode and stop counting its braces.
    const CHAR_LITERAL_WINDOW: usize = 10;

    let mut opens = 0usize;
    let mut closes = 0usize;
    let mut in_str: Option<char> = None;
    let mut prev = '\0';
    let mut iter = line.char_indices().peekable();
    while let Some((i, c)) = iter.next() {
        if *in_block_comment {
            if c == '/' && prev == '*' {
                *in_block_comment = false;
            }
            prev = c;
            continue;
        }
        match in_str {
            Some(q) => {
                if c == q && prev != '\\' {
                    in_str = None;
                }
            }
            None => match c {
                '"' | '`' => in_str = Some(c),
                '/' if iter.peek().map(|&(_, ch)| ch) == Some('*') => {
                    iter.next();
                    *in_block_comment = true;
                    prev = '\0';
                    continue;
                }
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
/// imports, type declarations and struct/enum fields exactly. Constructors,
/// entrypoints, short accessors, error-dense bodies and query-matching bodies
/// are pinned open; large bodies keep a head/tail skeleton.
#[cfg(feature = "tinyjuice-treesitter")]
mod treesitter {
    use super::{
        BLOCK_NOTE, CompressOutput, CompressorKind, MIN_BODY_LINES_TO_COLLAPSE,
        SKELETON_MIN_INTERIOR, block_token, keep_body, leading_ws,
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

    /// A collapsible body plus the name of its enclosing function (when known).
    struct Body {
        start: usize,
        end: usize,
        name: Option<String>,
    }

    pub fn compress(
        content: &str,
        ext: &str,
        per_body_tokens: bool,
        query: Option<&str>,
    ) -> Option<CompressOutput> {
        let (language, braced) = language_for(ext)?;
        let mut parser = Parser::new();
        parser.set_language(&language).ok()?;
        let tree = parser.parse(content, None)?;
        let src = content.as_bytes();

        // Collect outermost collapsible bodies.
        let mut bodies: Vec<Body> = Vec::new();
        collect_bodies(tree.root_node(), src, &mut bodies);
        bodies.sort_by_key(|b| b.start);
        let mut merged: Vec<Body> = Vec::new();
        for b in bodies {
            if let Some(last) = merged.last()
                && b.start < last.end
            {
                continue; // nested inside a body we're already collapsing
            }
            merged.push(b);
        }
        if merged.is_empty() {
            return None;
        }

        let mut out = String::with_capacity(content.len());
        let mut cursor = 0usize;
        let mut any_token = false;
        for b in merged {
            if b.start < cursor {
                continue;
            }
            out.push_str(&content[cursor..b.start]);
            let body = &content[b.start..b.end];
            let n_lines = body.lines().count();
            // Interior excludes the brace lines for braced bodies.
            let interior = if braced {
                n_lines.saturating_sub(2)
            } else {
                n_lines
            };
            let collapse = interior >= MIN_BODY_LINES_TO_COLLAPSE
                && !keep_body(b.name.as_deref(), body, interior, query);
            if !collapse {
                out.push_str(body);
                cursor = b.end;
                continue;
            }
            let token = block_token(body, per_body_tokens);
            any_token |= !token.is_empty();
            let rendered = render_collapse(body, braced, interior, n_lines, &token);
            out.push_str(&rendered);
            cursor = b.end;
        }
        out.push_str(&content[cursor..]);

        let mut out = out.trim_end().to_string();
        if any_token {
            out.push_str(BLOCK_NOTE);
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

    /// Render a collapsed body: a head/tail skeleton for large bodies, a single
    /// placeholder otherwise. The `token` recovers the FULL original body.
    fn render_collapse(
        body: &str,
        braced: bool,
        interior: usize,
        n_lines: usize,
        token: &str,
    ) -> String {
        if braced {
            let lines: Vec<&str> = body.lines().collect();
            if interior >= SKELETON_MIN_INTERIOR && lines.len() >= 5 {
                let inner = &lines[1..lines.len() - 1];
                let indent = leading_ws(inner[0]);
                let elided = inner.len() - 3;
                return format!(
                    "{{\n{}\n{}\n{indent}{{ … {elided} line(s) …{token} }}\n{}\n}}",
                    inner[0],
                    inner[1],
                    inner[inner.len() - 1],
                );
            }
            format!("{{ … {n_lines} line(s) …{token} }}")
        } else {
            // Python suite — indented ellipsis so it still reads.
            let lines: Vec<&str> = body.lines().collect();
            if interior >= SKELETON_MIN_INTERIOR && lines.len() >= 4 {
                let indent = leading_ws(lines[0]);
                let elided = lines.len() - 3;
                return format!(
                    "{}\n{}\n{indent}...  # {elided} line(s) collapsed{token}\n{}",
                    lines[0],
                    lines[1],
                    lines[lines.len() - 1],
                );
            }
            format!("...  # {n_lines} line(s) collapsed{token}")
        }
    }

    /// Recursively collect the byte-ranges (and names) of function/method bodies.
    fn collect_bodies(node: Node, src: &[u8], out: &mut Vec<Body>) {
        if BODY_PARENTS.contains(&node.kind())
            && let Some(body) = node.child_by_field_name("body")
        {
            let name = node
                .child_by_field_name("name")
                .and_then(|n| n.utf8_text(src).ok())
                .map(|s| s.to_string());
            out.push(Body {
                start: body.start_byte(),
                end: body.end_byte(),
                name,
            });
            // Don't descend into a collapsed body.
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
    use crate::cache;

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
        let out = compress_heuristic(&src, false, None).expect("compresses");
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
        assert!(compress_heuristic(src, false, None).is_none());
    }

    /// A large body keeps a head/tail skeleton: the first two and the last
    /// interior lines survive, only the middle is elided.
    #[test]
    fn skeleton_keeps_head_and_tail_lines() {
        let mut src = String::from("pub fn walk(items: &[i32]) -> i32 {\n");
        src.push_str("    let head_a = items.len();\n");
        src.push_str("    let head_b = items.first().copied().unwrap_or(0);\n");
        for i in 0..20 {
            src.push_str(&format!("    let mid_{i} = compute(items, {i});\n"));
        }
        src.push_str("    head_a + head_b\n}\n");
        // Pad so the file clears the 12-line minimum with unrelated top-level lines.
        for i in 0..4 {
            src.push_str(&format!("pub const K_{i}: i32 = {i};\n"));
        }
        let out = compress_heuristic(&src, false, None).expect("compresses");
        assert!(
            out.text.contains("let head_a = items.len();"),
            "{}",
            out.text
        );
        assert!(
            out.text.contains("let head_b = items.first"),
            "second head line kept: {}",
            out.text
        );
        assert!(
            out.text.contains("head_a + head_b"),
            "tail line kept: {}",
            out.text
        );
        assert!(!out.text.contains("mid_10"), "middle elided: {}", out.text);
        assert!(out.text.contains("line(s) …"));
    }

    /// The full body is recoverable from the per-block token even when only a
    /// skeleton is shown.
    #[test]
    fn skeleton_token_recovers_full_body() {
        let mut src = String::from("pub fn scan(items: &[i32]) -> i32 {\n");
        src.push_str("    let a = items.len();\n");
        src.push_str("    let b = 0;\n");
        for i in 0..20 {
            src.push_str(&format!("    let mid_{i} = compute(items, {i});\n"));
        }
        src.push_str("    a + b\n}\n");
        for i in 0..4 {
            src.push_str(&format!("pub const C_{i}: i32 = {i};\n"));
        }
        let out = compress_heuristic(&src, true, None).expect("compresses");
        let tokens = cache::parse_markers(&out.text);
        assert_eq!(tokens.len(), 1, "{}", out.text);
        let body = cache::retrieve(&tokens[0]).expect("stored");
        assert!(body.contains("let mid_10 = compute(items, 10);"), "{body}");
        assert!(body.contains("a + b"), "full body recovered: {body}");
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
        let out = treesitter::compress(&src, "rs", false, None).expect("compresses");
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
        let out = treesitter::compress(&src, "py", false, None).expect("compresses");
        assert!(out.text.contains("def handler(event):"), "{}", out.text);
        assert!(out.text.contains("collapsed"), "{}", out.text);
        assert!(!out.text.contains("x_15"));
    }

    #[cfg(feature = "tinyjuice-treesitter")]
    #[test]
    fn treesitter_keeps_main() {
        let mut src = String::from("fn helper(items: &[i32]) -> i32 {\n");
        for i in 0..20 {
            src.push_str(&format!("    let h_{i} = items.len() + {i};\n"));
        }
        src.push_str("    h_0\n}\n\n");
        src.push_str("fn main() {\n");
        for i in 0..20 {
            src.push_str(&format!("    println!(\"step {i}\");\n"));
        }
        src.push_str("    helper(&[]);\n}\n");
        let out = treesitter::compress(&src, "rs", false, None).expect("compresses");
        // main is an entrypoint — it must survive verbatim.
        assert!(out.text.contains("println!(\"step 10\")"), "{}", out.text);
        // helper's body still collapses.
        assert!(!out.text.contains("h_10"), "{}", out.text);
    }

    #[test]
    fn brace_delta_handles_lifetimes_and_char_literals() {
        // Lifetimes must not open phantom string mode.
        assert_eq!(
            brace_delta("pub fn f<'a>(x: &'a str) {", &mut false),
            (1, 0)
        );
        assert_eq!(
            brace_delta("impl<'de> Deserialize<'de> for X {", &mut false),
            (1, 0)
        );
        assert_eq!(
            brace_delta(
                "fn g<'a, 'b>(x: &'a str, y: &'b str) -> &'a str {",
                &mut false
            ),
            (1, 0)
        );
        // Real char literals still shield their contents.
        assert_eq!(brace_delta("let c = '{';", &mut false), (0, 0));
        assert_eq!(brace_delta("if c == '}' { close(); }", &mut false), (1, 1));
        // Strings still shield braces.
        assert_eq!(brace_delta(r#"let s = "{}"; f(|| {"#, &mut false), (1, 0));
    }

    #[test]
    fn lifetime_heavy_rust_keeps_signatures() {
        let mut src = String::from("use serde::Deserialize;\n\n");
        src.push_str("pub fn parse<'de>(map: &Map) -> Config {\n");
        for i in 0..20 {
            src.push_str(&format!("        let field_{i} = map.next_value();\n"));
        }
        src.push_str("        Config::default()\n}\n\n");
        src.push_str("pub fn lookup<'a>(map: &'a Map, key: &str) -> Option<&'a str> {\n");
        for i in 0..20 {
            src.push_str(&format!("        let probe_{i} = map.get(key);\n"));
        }
        src.push_str("        None\n}\n\n");
        src.push_str("pub struct Cursor<'a> {\n    buf: &'a [u8],\n    pos: usize,\n}\n");
        let out = compress_heuristic(&src, false, None).expect("compresses");
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

        let out = compress_heuristic(&src, true, None).expect("compresses");
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
        let out = compress_heuristic(&src, false, None).expect("compresses");
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
        let out = treesitter::compress(&src, "rs", true, None).expect("compresses");
        let tokens = cache::parse_markers(&out.text);
        assert_eq!(tokens.len(), 1, "{}", out.text);
        let body = cache::retrieve(&tokens[0]).expect("stored");
        assert!(body.contains("let d_7"), "{body}");
    }

    /// A body whose name or contents matches the caller's query stays open.
    #[test]
    fn query_keeps_matching_function() {
        let mut src = String::from("pub fn unrelated() -> i32 {\n");
        for i in 0..20 {
            src.push_str(&format!("    let u_{i} = {i};\n"));
        }
        src.push_str("    u_0\n}\n\n");
        src.push_str("pub fn compute_checksum(data: &[u8]) -> u32 {\n");
        for i in 0..20 {
            src.push_str(&format!("    let c_{i} = data.len() as u32 + {i};\n"));
        }
        src.push_str("    c_0\n}\n");
        let out = compress_heuristic(&src, false, Some("checksum")).expect("compresses");
        // The queried function stays open; the unrelated one collapses.
        assert!(
            out.text.contains("let c_10"),
            "queried body kept: {}",
            out.text
        );
        assert!(
            !out.text.contains("u_10"),
            "other body collapsed: {}",
            out.text
        );
    }

    /// An error-handling dense body is kept even though it is long.
    #[test]
    fn error_dense_body_is_kept() {
        let mut src = String::from("pub fn validate(input: &str) -> Result<(), E> {\n");
        src.push_str("    if input.is_empty() {\n");
        src.push_str("        return Err(E::Empty);\n");
        src.push_str("    }\n");
        for i in 0..8 {
            src.push_str(&format!("    let step_{i} = check(input, {i});\n"));
        }
        src.push_str("    if bad {\n");
        src.push_str("        return Err(E::Bad);\n");
        src.push_str("    }\n");
        src.push_str("    Ok(())\n}\n\n");
        // Add an unrelated collapsible function so the file still shrinks.
        src.push_str("pub fn filler() -> i32 {\n");
        for i in 0..20 {
            src.push_str(&format!("    let f_{i} = {i};\n"));
        }
        src.push_str("    f_0\n}\n");
        let out = compress_heuristic(&src, false, None).expect("compresses");
        assert!(
            out.text.contains("return Err(E::Bad)"),
            "error-dense body kept: {}",
            out.text
        );
    }

    /// The byte guard prevents collapsing a body when the marker would be
    /// longer than the code it replaces (no inflation).
    #[test]
    fn byte_guard_does_not_inflate() {
        // A container of short one-line methods: each body is tiny, so even
        // though there are many of them, none should be replaced by a marker
        // that is longer than the body.
        let mut src = String::from("class Tiny {\n");
        for i in 0..14 {
            src.push_str(&format!("    fn m{i}(&self) -> i32 {{ {i} }}\n"));
        }
        src.push_str("}\n");
        // Nothing collapses, so it does not shrink -> None (no inflation, no
        // bogus compression).
        let out = compress_heuristic(&src, false, None);
        if let Some(o) = out {
            assert!(o.text.len() < src.len(), "must never inflate: {}", o.text);
            // The one-line method bodies must not have been swapped for a
            // longer marker.
            assert!(
                !o.text.contains("{ … "),
                "tiny bodies untouched: {}",
                o.text
            );
        }
    }

    /// PHP class: method signatures and PHPDoc survive, only long bodies
    /// collapse. The class must never shrink to a single marker.
    #[test]
    fn php_class_keeps_method_signatures() {
        let mut src = String::from("<?php\n\nnamespace App;\n\nclass Repository\n{\n");
        src.push_str("    private ?Connection $conn;\n\n");
        src.push_str("    public function __construct(Connection $conn)\n    {\n");
        src.push_str("        $this->conn = $conn;\n    }\n\n");
        src.push_str("    /**\n     * Find a record by id.\n     */\n");
        src.push_str("    public function findById(int $id): ?Record\n    {\n");
        for i in 0..20 {
            src.push_str(&format!("        $step_{i} = $this->query($id, {i});\n"));
        }
        src.push_str("        return $step_0;\n    }\n\n");
        src.push_str("    public function deleteAll(): void\n    {\n");
        for i in 0..20 {
            src.push_str(&format!("        $this->purge({i});\n"));
        }
        src.push_str("    }\n}\n");
        let out = compress_heuristic(&src, false, None).expect("compresses");
        assert!(out.text.contains("class Repository"), "{}", out.text);
        assert!(
            out.text
                .contains("public function findById(int $id): ?Record"),
            "method signature kept: {}",
            out.text
        );
        assert!(
            out.text.contains("public function deleteAll(): void"),
            "method signature kept: {}",
            out.text
        );
        assert!(
            out.text.contains("private ?Connection $conn;"),
            "field kept: {}",
            out.text
        );
        assert!(
            out.text.contains("Find a record by id"),
            "phpdoc kept: {}",
            out.text
        );
        // Long bodies collapsed.
        assert!(!out.text.contains("step_10"), "{}", out.text);
        // Class must not have been reduced to a lone marker.
        assert!(
            out.text.matches("public function").count() >= 2,
            "container must keep its member signatures: {}",
            out.text
        );
    }

    /// The short constructor stays verbatim.
    #[test]
    fn php_class_keeps_short_constructor() {
        let mut src = String::from("<?php\n\nclass Point\n{\n");
        src.push_str("    private float $x;\n    private float $y;\n\n");
        src.push_str("    public function __construct(float $x, float $y)\n    {\n");
        src.push_str("        $this->x = $x;\n        $this->y = $y;\n    }\n\n");
        src.push_str("    public function distance(Point $o): float\n    {\n");
        for i in 0..20 {
            src.push_str(&format!("        $d_{i} = hypot($this->x, {i});\n"));
        }
        src.push_str("        return $d_0;\n    }\n}\n");
        let out = compress_heuristic(&src, false, None).expect("compresses");
        assert!(
            out.text.contains("$this->x = $x;"),
            "constructor body kept: {}",
            out.text
        );
        assert!(
            !out.text.contains("$d_10"),
            "long body collapsed: {}",
            out.text
        );
    }

    /// Java class with Javadoc: class + method signatures + Javadoc survive.
    #[test]
    fn java_class_with_javadoc_keeps_signatures() {
        let mut src = String::from("package com.example;\n\n");
        src.push_str("/**\n * A widget registry. Uses {@link Widget}.\n */\n");
        src.push_str("public final class Registry {\n");
        src.push_str("    private final Map<String, Widget> items = new HashMap<>();\n\n");
        src.push_str("    /**\n     * Register a widget under a name.\n     */\n");
        src.push_str("    public void register(String name, Widget w) {\n");
        for i in 0..20 {
            src.push_str(&format!("        this.items.put(name + {i}, w);\n"));
        }
        src.push_str("    }\n\n");
        src.push_str("    public Widget lookup(String name) {\n");
        for i in 0..20 {
            src.push_str(&format!("        var candidate_{i} = items.get(name);\n"));
        }
        src.push_str("        return null;\n    }\n}\n");
        let out = compress_heuristic(&src, false, None).expect("compresses");
        assert!(
            out.text.contains("public final class Registry"),
            "{}",
            out.text
        );
        assert!(
            out.text
                .contains("public void register(String name, Widget w)"),
            "method signature kept: {}",
            out.text
        );
        assert!(
            out.text.contains("public Widget lookup(String name)"),
            "method signature kept: {}",
            out.text
        );
        assert!(
            out.text.contains("Register a widget under a name"),
            "javadoc kept: {}",
            out.text
        );
        assert!(
            out.text.contains("private final Map<String, Widget> items"),
            "field kept: {}",
            out.text
        );
        assert!(
            !out.text.contains("candidate_10"),
            "body collapsed: {}",
            out.text
        );
    }

    /// C# namespace + class: members inside the nested container stay visible
    /// even though the namespace adds a level of nesting.
    #[test]
    fn csharp_namespace_class_keeps_members() {
        let mut src = String::from("using System;\n\nnamespace Acme.Core\n{\n");
        src.push_str("    public class Service\n    {\n");
        src.push_str("        private readonly ILogger _log;\n\n");
        src.push_str("        public void Run(int count)\n        {\n");
        for i in 0..20 {
            src.push_str(&format!("            _log.Info($\"tick {i}\");\n"));
        }
        src.push_str("        }\n\n");
        src.push_str("        public int Compute(int seed)\n        {\n");
        for i in 0..20 {
            src.push_str(&format!("            var v_{i} = seed + {i};\n"));
        }
        src.push_str("            return seed;\n        }\n    }\n}\n");
        let out = compress_heuristic(&src, false, None).expect("compresses");
        assert!(out.text.contains("public class Service"), "{}", out.text);
        assert!(
            out.text.contains("public void Run(int count)"),
            "method signature kept: {}",
            out.text
        );
        assert!(
            out.text.contains("public int Compute(int seed)"),
            "method signature kept: {}",
            out.text
        );
        assert!(
            out.text.contains("private readonly ILogger _log;"),
            "field kept: {}",
            out.text
        );
        assert!(!out.text.contains("v_10"), "body collapsed: {}", out.text);
    }

    /// C++ class inside a namespace keeps its method signatures.
    #[test]
    fn cpp_namespace_class_keeps_signatures() {
        let mut src = String::from("#include <vector>\n\nnamespace engine {\n\n");
        src.push_str("class SceneNode {\npublic:\n");
        src.push_str("    void attach(SceneNode* child) {\n");
        for i in 0..20 {
            src.push_str(&format!("        this->children.push_back({i});\n"));
        }
        src.push_str("    }\n");
        src.push_str("    void detach(SceneNode* child) {\n");
        for i in 0..20 {
            src.push_str(&format!("        this->children.erase({i});\n"));
        }
        src.push_str("    }\n");
        src.push_str("};\n\n}  // namespace engine\n");
        let out = compress_heuristic(&src, false, None).expect("compresses");
        assert!(
            out.text.contains("class SceneNode"),
            "class inside namespace must stay visible: {}",
            out.text
        );
        assert!(
            out.text.contains("void attach(SceneNode* child)"),
            "method signature kept: {}",
            out.text
        );
        assert!(
            out.text.contains("void detach(SceneNode* child)"),
            "method signature kept: {}",
            out.text
        );
    }

    #[test]
    fn block_comments_do_not_drift_depth() {
        // Javadoc braces ({@link ...}) must not count.
        let mut in_c = false;
        assert_eq!(
            brace_delta("/** see {@link Foo#bar} for details", &mut in_c),
            (0, 0)
        );
        assert!(in_c);
        assert_eq!(
            brace_delta(" * more prose with { braces } inside", &mut in_c),
            (0, 0)
        );
        assert_eq!(brace_delta(" */ fn real() {", &mut in_c), (1, 0));
        assert!(!in_c);
    }

    #[test]
    fn fn_name_extracts_across_languages() {
        assert_eq!(
            fn_name("pub fn process(items: &[i32]) -> i32 {").as_deref(),
            Some("process")
        );
        assert_eq!(fn_name("def handler(event):").as_deref(), Some("handler"));
        assert_eq!(
            fn_name("public function getRoot(): ?AVLTreeNode").as_deref(),
            Some("getRoot")
        );
        assert_eq!(
            fn_name("public void doThing(int x) {").as_deref(),
            Some("doThing")
        );
        assert_eq!(
            fn_name("func (r *Repo) Save(x int) error {").as_deref(),
            Some("Save")
        );
        assert_eq!(
            fn_name("func Serve(addr string) {").as_deref(),
            Some("Serve")
        );
    }

    #[test]
    fn decl_opener_and_signature_classification() {
        assert!(is_decl_opener_line("public void doThing(int x) {"));
        assert!(is_decl_opener_line("class Foo {"));
        assert!(is_decl_opener_line("pub fn f<'a>(x: &'a str) {"));
        assert!(!is_decl_opener_line("if (x > 0) {"));
        assert!(!is_decl_opener_line("for (int i = 0; i < n; i++) {"));
        assert!(!is_decl_opener_line("{"));
        assert!(is_signature_line("public function getRoot(): ?AVLTreeNode"));
        assert!(!is_signature_line("$this->root = null;"));
        assert!(!is_signature_line("return doThing(x);"));
    }
}
