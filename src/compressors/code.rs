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
use crate::cache::CcrStore;
use crate::pipeline::{OffloadOutput, OffloadTransform, PipelineInput, estimate_bloat};
use crate::types::{
    CodeElision, CodeStubOutput, CompressInput, CompressOptions, CompressOutput, CompressorKind,
    ContentKind, LineRange, ParseStatus, ReadIntent, StubMode, SymbolSummary,
};

/// Bodies with more than this many collapsed lines get a placeholder; shorter
/// ones are kept verbatim (collapsing tiny bodies isn't worth the marker).
pub const MIN_BODY_LINES_TO_COLLAPSE: usize = 4;

/// Markers that force a line to be kept even inside a deep body.
const KEEP_MARKERS: &[&str] = &["TODO", "FIXME", "XXX", "error", "panic", "unsafe"];

pub struct CodeCompressor;

/// Typed offload transform for explicit source-code stub reads.
#[derive(Debug, Clone)]
pub struct CodeStubTransform {
    mode: StubMode,
    extension: Option<String>,
}

impl CodeStubTransform {
    pub fn new(mode: StubMode) -> Self {
        Self {
            mode,
            extension: None,
        }
    }

    pub fn with_extension(mut self, extension: impl Into<String>) -> Self {
        self.extension = Some(extension.into());
        self
    }

    pub fn mode(&self) -> &StubMode {
        &self.mode
    }

    pub fn extension(&self) -> Option<&str> {
        self.extension.as_deref()
    }
}

impl OffloadTransform for CodeStubTransform {
    fn name(&self) -> &'static str {
        "code_stub"
    }

    fn estimate_bloat(&self, input: &PipelineInput<'_>) -> f32 {
        if input.content_kind != ContentKind::Code {
            return 0.0;
        }
        f32::from(estimate_bloat(input.content, input.content_kind).score) / 100.0
    }

    fn apply(&self, input: &PipelineInput<'_>, store: &dyn CcrStore) -> Option<OffloadOutput> {
        if input.content_kind != ContentKind::Code {
            return None;
        }
        let stub = stub_code(
            input.content,
            self.extension(),
            &self.mode,
            input.original_bytes,
        );
        if stub.text.len() >= input.content.len() {
            return None;
        }
        OffloadOutput::from_retained_put(
            stub.text,
            CompressorKind::Code,
            store.put(input.original_content),
        )
    }
}

#[async_trait]
impl Compressor for CodeCompressor {
    fn kind(&self) -> CompressorKind {
        CompressorKind::Code
    }

    async fn compress(
        &self,
        input: &CompressInput<'_>,
        _opts: &CompressOptions,
    ) -> Option<CompressOutput> {
        if let ReadIntent::Stub(mode) = &input.hint.read_intent {
            let stub = stub_code(
                input.content,
                input.hint.extension.as_deref(),
                mode,
                input.original_bytes,
            );
            if stub.text.len() < input.content.len() {
                return Some(CompressOutput::lossy(stub.text, CompressorKind::Code));
            }
            return None;
        }

        // Prefer the AST path when a grammar matches the file's language; fall
        // back to the language-agnostic heuristic otherwise (or when the AST
        // path doesn't shrink the content).
        #[cfg(feature = "tokenjuice-treesitter")]
        if let Some(ext) = input.hint.extension.as_deref()
            && let Some(out) = treesitter::compress(input.content, ext)
        {
            return Some(out);
        }
        compress_heuristic(input.content)
    }
}

/// Language-agnostic brace-depth compressor. Keeps lines at depth ≤ 1, collapses
/// deeper runs. Returns `None` if it wouldn't shrink the content.
pub fn compress_heuristic(content: &str) -> Option<CompressOutput> {
    let lines: Vec<&str> = content.lines().collect();
    if lines.len() < 12 {
        return None;
    }

    let mut out = String::with_capacity(content.len() / 2 + 64);
    let mut depth: i32 = 0;
    let mut collapsed: Vec<&str> = Vec::new();

    let flush = |out: &mut String, collapsed: &mut Vec<&str>| {
        if collapsed.is_empty() {
            return;
        }
        if collapsed.len() >= MIN_BODY_LINES_TO_COLLAPSE {
            let _ = writeln!(out, "    {{ … {} line(s) … }}", collapsed.len());
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
            flush(&mut out, &mut collapsed);
            let _ = writeln!(out, "{line}");
        } else {
            collapsed.push(line);
        }
        depth += opens - closes;
        if depth < 0 {
            depth = 0;
        }
    }
    flush(&mut out, &mut collapsed);

    let out = out.trim_end().to_string();
    if out.len() >= content.len() {
        return None;
    }
    log::debug!(
        "[tokenjuice][code] heuristic {} -> {} bytes ({} lines)",
        content.len(),
        out.len(),
        lines.len()
    );
    Some(CompressOutput::lossy(out, CompressorKind::Code))
}

/// Build a structural source-code stub with elision metadata.
///
/// Hosts call this directly for explicit stub reads. Exact reads should not use
/// this path.
pub fn stub_code(
    content: &str,
    extension: Option<&str>,
    mode: &StubMode,
    max_bytes: usize,
) -> CodeStubOutput {
    #[cfg(feature = "tokenjuice-treesitter")]
    if let Some(ext) = extension
        && let Some(out) = treesitter::stub(content, ext, mode)
    {
        return enforce_stub_budget(out, max_bytes);
    }

    enforce_stub_budget(stub_heuristic(content, mode), max_bytes)
}

#[derive(Debug, Clone)]
struct StubBody {
    declaration_start: usize,
    declaration_range: LineRange,
    body_start: usize,
    body_end: usize,
    body_range: LineRange,
    symbol: SymbolSummary,
}

fn should_expand_body(mode: &StubMode, body: &StubBody) -> bool {
    match mode {
        StubMode::SignaturesOnly | StubMode::PublicApi => false,
        StubMode::MatchedSymbols(symbols) => symbols.iter().any(|s| {
            let wanted = s.trim();
            !wanted.is_empty() && wanted == body.symbol.name
        }),
        StubMode::ExpandAroundLines(ranges) => ranges
            .iter()
            .copied()
            .any(|range| range.intersects(body.body_range)),
    }
}

fn placeholder_for_body(content: &str, body: &StubBody) -> String {
    let body_text = &content[body.body_start..body.body_end];
    let lines = body.body_range.end.saturating_sub(body.body_range.start) + 1;
    if body_text.trim_start().starts_with('{') {
        format!(
            "{{ /* ... lines {}-{} elided ({lines} line(s)) ... */ }}",
            body.body_range.start, body.body_range.end
        )
    } else {
        format!(
            "...  # lines {}-{} elided ({lines} line(s))",
            body.body_range.start, body.body_range.end
        )
    }
}

fn build_stub_from_bodies(
    content: &str,
    bodies: Vec<StubBody>,
    mode: &StubMode,
    parse_status: ParseStatus,
) -> CodeStubOutput {
    let mut out = String::with_capacity(content.len() / 2 + 128);
    let mut cursor = 0usize;
    let mut symbols = Vec::new();
    let mut elisions = Vec::new();

    for body in bodies {
        if body.body_start < cursor {
            continue;
        }
        if matches!(mode, StubMode::PublicApi) && !body.symbol.public {
            out.push_str(&content[cursor..body.declaration_start]);
            let lines = body
                .body_range
                .end
                .saturating_sub(body.declaration_range.start)
                + 1;
            let _ = write!(
                out,
                "/* ... private declaration lines {}-{} elided ({lines} line(s)) ... */",
                body.declaration_range.start, body.body_range.end
            );
            elisions.push(CodeElision {
                start_line: body.declaration_range.start,
                end_line: body.body_range.end,
                reason: "public_api_private_declaration".to_string(),
            });
            cursor = body.body_end;
            continue;
        }
        symbols.push(body.symbol.clone());
        out.push_str(&content[cursor..body.body_start]);
        let body_text = &content[body.body_start..body.body_end];
        let body_lines = body_text.lines().count();
        if body_lines < MIN_BODY_LINES_TO_COLLAPSE || should_expand_body(mode, &body) {
            out.push_str(body_text);
        } else {
            out.push_str(&placeholder_for_body(content, &body));
            elisions.push(CodeElision {
                start_line: body.body_range.start,
                end_line: body.body_range.end,
                reason: format!("body_stub:{:?}", mode),
            });
        }
        cursor = body.body_end;
    }
    out.push_str(&content[cursor..]);

    CodeStubOutput {
        text: out.trim_end().to_string(),
        symbols,
        elisions,
        parse_status,
    }
}

fn stub_heuristic(content: &str, mode: &StubMode) -> CodeStubOutput {
    let mut out = String::with_capacity(content.len() / 2 + 128);
    let mut depth: i32 = 0;
    let mut collapsed: Vec<(usize, &str)> = Vec::new();
    let mut elisions = Vec::new();
    let mut symbols = Vec::new();
    let mut skipped_private_start: Option<usize> = None;
    let mut expanding_body = false;

    let flush =
        |out: &mut String, collapsed: &mut Vec<(usize, &str)>, elisions: &mut Vec<CodeElision>| {
            if collapsed.is_empty() {
                return;
            }
            if collapsed.len() >= MIN_BODY_LINES_TO_COLLAPSE {
                let start = collapsed.first().map(|(n, _)| *n).unwrap_or(1);
                let end = collapsed.last().map(|(n, _)| *n).unwrap_or(start);
                let _ = writeln!(
                    out,
                    "    {{ /* ... lines {start}-{end} elided ({} line(s)) ... */ }}",
                    collapsed.len()
                );
                elisions.push(CodeElision {
                    start_line: start,
                    end_line: end,
                    reason: format!("heuristic_body_stub:{:?}", mode),
                });
            } else {
                for (_, line) in collapsed.iter() {
                    let _ = writeln!(out, "{line}");
                }
            }
            collapsed.clear();
        };
    let flush_verbatim = |out: &mut String, collapsed: &mut Vec<(usize, &str)>| {
        for (_, line) in collapsed.iter() {
            let _ = writeln!(out, "{line}");
        }
        collapsed.clear();
    };

    for (idx, line) in content.lines().enumerate() {
        let line_no = idx + 1;
        let (opens, closes) = brace_delta(line);
        let start_depth = depth;
        let force_keep = KEEP_MARKERS.iter().any(|m| line.contains(m));
        let mut next_depth = depth + opens - closes;
        if next_depth < 0 {
            next_depth = 0;
        }

        if let Some(start_line) = skipped_private_start {
            depth = next_depth;
            if depth == 0 {
                elisions.push(CodeElision {
                    start_line,
                    end_line: line_no,
                    reason: "public_api_private_declaration".to_string(),
                });
                let _ = writeln!(
                    out,
                    "/* ... private declaration lines {start_line}-{line_no} elided ({} line(s)) ... */",
                    line_no.saturating_sub(start_line) + 1
                );
                skipped_private_start = None;
            }
            continue;
        }

        if expanding_body {
            let _ = writeln!(out, "{line}");
            depth = next_depth;
            if depth == 0 {
                expanding_body = false;
            }
            continue;
        }

        if start_depth == 0 || force_keep {
            flush(&mut out, &mut collapsed, &mut elisions);
            if let Some(symbol) = symbol_from_signature(line, line_no, line_no, "heuristic") {
                if matches!(mode, StubMode::PublicApi) && !symbol.public {
                    if next_depth == 0 {
                        elisions.push(CodeElision {
                            start_line: line_no,
                            end_line: line_no,
                            reason: "public_api_private_declaration".to_string(),
                        });
                        let _ = writeln!(
                            out,
                            "/* ... private declaration line {line_no} elided ... */"
                        );
                    } else {
                        skipped_private_start = Some(line_no);
                    }
                    depth = next_depth;
                    continue;
                }
                if should_expand_heuristic_symbol(mode, &symbol, line_no) && next_depth > 0 {
                    expanding_body = true;
                }
                symbols.push(symbol);
            }
            let _ = writeln!(out, "{line}");
        } else {
            if should_expand_heuristic_line(mode, line_no) {
                flush_verbatim(&mut out, &mut collapsed);
                let _ = writeln!(out, "{line}");
                expanding_body = next_depth > 0;
                depth = next_depth;
                continue;
            }
            collapsed.push((line_no, line));
        }
        depth = next_depth;
    }
    if let Some(start_line) = skipped_private_start {
        let end_line = content.lines().count().max(start_line);
        elisions.push(CodeElision {
            start_line,
            end_line,
            reason: "public_api_private_declaration".to_string(),
        });
        let _ = writeln!(
            out,
            "/* ... private declaration lines {start_line}-{end_line} elided ({} line(s)) ... */",
            end_line.saturating_sub(start_line) + 1
        );
    }
    flush(&mut out, &mut collapsed, &mut elisions);

    CodeStubOutput {
        text: out.trim_end().to_string(),
        symbols,
        elisions,
        parse_status: ParseStatus::HeuristicFallback,
    }
}

fn should_expand_heuristic_symbol(mode: &StubMode, symbol: &SymbolSummary, line_no: usize) -> bool {
    match mode {
        StubMode::MatchedSymbols(symbols) => symbols.iter().any(|s| {
            let wanted = s.trim();
            !wanted.is_empty() && wanted == symbol.name
        }),
        StubMode::ExpandAroundLines(ranges) => ranges
            .iter()
            .copied()
            .any(|range| range.intersects(LineRange::new(line_no, line_no))),
        StubMode::SignaturesOnly | StubMode::PublicApi => false,
    }
}

fn should_expand_heuristic_line(mode: &StubMode, line_no: usize) -> bool {
    match mode {
        StubMode::ExpandAroundLines(ranges) => ranges
            .iter()
            .copied()
            .any(|range| range.intersects(LineRange::new(line_no, line_no))),
        StubMode::SignaturesOnly | StubMode::PublicApi | StubMode::MatchedSymbols(_) => false,
    }
}

fn enforce_stub_budget(mut stub: CodeStubOutput, max_bytes: usize) -> CodeStubOutput {
    if max_bytes == 0 || stub.text.len() <= max_bytes {
        return stub;
    }

    let marker = "\n[TOKENJUICE CODE STUB TRUNCATED]\n";
    let budget = max_bytes.saturating_sub(marker.len()).max(marker.len());
    let mut out = String::new();
    let mut kept_lines = 0usize;
    for line in stub.text.lines() {
        let addition = line.len() + 1;
        if out.len() + addition + marker.len() > budget {
            break;
        }
        out.push_str(line);
        out.push('\n');
        kept_lines += 1;
    }
    out.push_str(marker);
    let total_lines = stub.text.lines().count();
    if kept_lines < total_lines {
        stub.elisions.push(CodeElision {
            start_line: kept_lines.saturating_add(1),
            end_line: total_lines,
            reason: "stub_budget".to_string(),
        });
    }
    stub.text = out.trim_end().to_string();
    stub
}

fn symbol_from_signature(
    signature: &str,
    start_line: usize,
    end_line: usize,
    fallback_kind: &str,
) -> Option<SymbolSummary> {
    let trimmed = signature.trim();
    if trimmed.is_empty() {
        return None;
    }

    let patterns = [
        ("fn ", "function"),
        ("function ", "function"),
        ("def ", "function"),
        ("class ", "class"),
        ("struct ", "struct"),
        ("enum ", "enum"),
        ("trait ", "trait"),
        ("interface ", "interface"),
        ("impl ", "impl"),
        ("const ", "const"),
        ("let ", "binding"),
        ("var ", "binding"),
    ];
    let lower = trimmed.to_ascii_lowercase();
    for (needle, kind) in patterns {
        if let Some(idx) = lower.find(needle) {
            let rest = &trimmed[idx + needle.len()..];
            let name = rest
                .trim_start()
                .trim_start_matches("r#")
                .split(|c: char| !(c.is_ascii_alphanumeric() || c == '_' || c == '$'))
                .next()
                .unwrap_or_default();
            if !name.is_empty() {
                return Some(SymbolSummary {
                    name: name.to_string(),
                    kind: kind.to_string(),
                    start_line,
                    end_line,
                    public: is_public_signature(trimmed),
                });
            }
        }
    }

    if fallback_kind != "heuristic" {
        Some(SymbolSummary {
            name: fallback_kind.to_string(),
            kind: fallback_kind.to_string(),
            start_line,
            end_line,
            public: is_public_signature(trimmed),
        })
    } else {
        None
    }
}

fn is_public_signature(signature: &str) -> bool {
    let trimmed = signature.trim_start();
    trimmed.starts_with("pub ")
        || trimmed.starts_with("pub(")
        || trimmed.starts_with("export ")
        || trimmed.starts_with("public ")
}

fn line_starts(content: &str) -> Vec<usize> {
    let mut starts = vec![0usize];
    for (idx, byte) in content.bytes().enumerate() {
        if byte == b'\n' {
            starts.push(idx + 1);
        }
    }
    starts
}

fn byte_to_line(starts: &[usize], byte: usize) -> usize {
    starts.partition_point(|start| *start <= byte).max(1)
}

/// Count `{`/`}` (and `(`/`)`) on a line, ignoring those inside string/char
/// literals and line comments — a cheap approximation good enough for the
/// depth heuristic.
fn brace_delta(line: &str) -> (i32, i32) {
    let mut opens = 0i32;
    let mut closes = 0i32;
    let mut in_str: Option<char> = None;
    let mut prev = '\0';
    let mut chars = line.chars().peekable();
    while let Some(c) = chars.next() {
        match in_str {
            Some(q) => {
                if c == q && prev != '\\' {
                    in_str = None;
                }
            }
            None => match c {
                '"' | '\'' | '`' => in_str = Some(c),
                '/' if chars.peek() == Some(&'/') => break, // line comment
                '#' => break,                               // python/shell comment
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
#[cfg(feature = "tokenjuice-treesitter")]
mod treesitter {
    use super::{
        CompressOutput, CompressorKind, LineRange, MIN_BODY_LINES_TO_COLLAPSE, ParseStatus,
        StubBody, StubMode, SymbolSummary, build_stub_from_bodies, byte_to_line, line_starts,
        symbol_from_signature,
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

    pub fn compress(content: &str, ext: &str) -> Option<CompressOutput> {
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
        for (start, end) in merged {
            if start < cursor {
                continue;
            }
            out.push_str(&content[cursor..start]);
            let body = &content[start..end];
            let n_lines = body.lines().count();
            if n_lines < MIN_BODY_LINES_TO_COLLAPSE {
                out.push_str(body);
            } else if braced {
                out.push_str(&format!("{{ … {n_lines} line(s) … }}"));
            } else {
                // Python suite — keep an indented ellipsis so it still reads.
                out.push_str(&format!("...  # {n_lines} line(s) collapsed"));
            }
            cursor = end;
        }
        out.push_str(&content[cursor..]);

        let out = out.trim_end().to_string();
        if out.len() >= content.len() {
            return None;
        }
        log::debug!(
            "[tokenjuice][code] tree-sitter ext={} {} -> {} bytes",
            ext,
            content.len(),
            out.len()
        );
        Some(CompressOutput::lossy(out, CompressorKind::Code))
    }

    pub fn stub(content: &str, ext: &str, mode: &StubMode) -> Option<super::CodeStubOutput> {
        let (language, _braced) = language_for(ext)?;
        let mut parser = Parser::new();
        parser.set_language(&language).ok()?;
        let tree = parser.parse(content, None)?;
        if tree.root_node().has_error() {
            return None;
        }

        let src = content.as_bytes();
        let starts = line_starts(content);
        let mut bodies = Vec::new();
        collect_stub_bodies(tree.root_node(), content, src, &starts, &mut bodies);
        bodies.sort_by_key(|body| body.body_start);
        bodies.dedup_by_key(|body| (body.body_start, body.body_end));
        if bodies.is_empty() {
            return None;
        }

        Some(build_stub_from_bodies(
            content,
            bodies,
            mode,
            ParseStatus::TreeSitter,
        ))
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

    fn collect_stub_bodies(
        node: Node,
        content: &str,
        src: &[u8],
        starts: &[usize],
        out: &mut Vec<StubBody>,
    ) {
        if BODY_PARENTS.contains(&node.kind())
            && let Some(body) = node.child_by_field_name("body")
        {
            let body_start = body.start_byte();
            let body_end = body.end_byte();
            let declaration_start = node.start_byte();
            let declaration_start_line = byte_to_line(starts, declaration_start);
            let declaration_end_line = byte_to_line(starts, body_start);
            let start_line = byte_to_line(starts, body_start);
            let end_line = byte_to_line(starts, body_end.saturating_sub(1));
            let symbol = symbol_for_node(content, node, body, starts);
            out.push(StubBody {
                declaration_start,
                declaration_range: LineRange::new(declaration_start_line, declaration_end_line),
                body_start,
                body_end,
                body_range: LineRange::new(start_line, end_line),
                symbol,
            });
            let _ = src;
            return;
        }
        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            collect_stub_bodies(child, content, src, starts, out);
        }
    }

    fn symbol_for_node(content: &str, node: Node, body: Node, starts: &[usize]) -> SymbolSummary {
        let signature = &content[node.start_byte()..body.start_byte()];
        let start_line = byte_to_line(starts, node.start_byte());
        let end_line = byte_to_line(starts, body.start_byte());
        symbol_from_signature(signature, start_line, end_line, node.kind()).unwrap_or_else(|| {
            SymbolSummary {
                name: node.kind().to_string(),
                kind: node.kind().to_string(),
                start_line,
                end_line,
                public: false,
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cache::{CcrStore, MemoryCcrStore};

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
        let out = compress_heuristic(&src).expect("compresses");
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
        assert!(compress_heuristic(src).is_none());
    }

    #[cfg(feature = "tokenjuice-treesitter")]
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
        let out = treesitter::compress(&src, "rs").expect("compresses");
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

    #[cfg(feature = "tokenjuice-treesitter")]
    #[test]
    fn treesitter_collapses_python_body() {
        let mut src = String::from("import os\n\ndef handler(event):\n");
        for i in 0..30 {
            src.push_str(&format!("    x_{i} = compute(event, {i})\n"));
        }
        src.push_str("    return x_0\n");
        let out = treesitter::compress(&src, "py").expect("compresses");
        assert!(out.text.contains("def handler(event):"), "{}", out.text);
        assert!(out.text.contains("collapsed"), "{}", out.text);
        assert!(!out.text.contains("x_15"));
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
        let out = compress_heuristic(&src).expect("compresses");
        assert!(out.text.contains("TODO"), "marker line kept:\n{}", out.text);
    }

    #[test]
    fn stub_code_reports_elisions_and_symbols() {
        let mut src = String::from("use std::fmt;\n\npub fn visible() {\n");
        for i in 0..20 {
            src.push_str(&format!("    println!(\"{i}\");\n"));
        }
        src.push_str("}\n");

        let out = stub_code(&src, Some("rs"), &StubMode::SignaturesOnly, usize::MAX);

        assert_eq!(out.parse_status, ParseStatus::TreeSitter);
        assert!(out.text.contains("pub fn visible()"));
        assert!(out.text.contains("lines"));
        assert!(!out.text.contains("println!(\"10\")"));
        assert_eq!(out.symbols[0].name, "visible");
        assert!(out.symbols[0].public);
        assert_eq!(out.elisions.len(), 1);
        assert!(out.elisions[0].start_line <= out.elisions[0].end_line);
    }

    #[test]
    fn code_stub_transform_requires_retained_ccr() {
        let mut src = String::from("use std::fmt;\n\npub fn visible() {\n");
        for i in 0..20 {
            src.push_str(&format!("    println!(\"{i}\");\n"));
        }
        src.push_str("}\n");
        let pipeline_input = PipelineInput {
            content: &src,
            original_content: &src,
            content_kind: ContentKind::Code,
            original_bytes: src.len(),
        };
        let transform = CodeStubTransform::new(StubMode::SignaturesOnly).with_extension("rs");

        let rejecting_store = MemoryCcrStore::new(1, 1);
        assert!(transform.apply(&pipeline_input, &rejecting_store).is_none());

        let store = MemoryCcrStore::default();
        let out = transform
            .apply(&pipeline_input, &store)
            .expect("retained code stub");

        assert_eq!(out.kind(), CompressorKind::Code);
        assert!(out.text().contains("pub fn visible()"), "{}", out.text());
        assert!(!out.text().contains("println!(\"10\")"), "{}", out.text());
        assert_eq!(store.get(out.token()).as_deref(), Some(src.as_str()));
    }

    #[test]
    fn code_stub_transform_skips_non_code_input() {
        let input = PipelineInput {
            content: "plain text",
            original_content: "plain text",
            content_kind: ContentKind::PlainText,
            original_bytes: "plain text".len(),
        };
        let transform = CodeStubTransform::new(StubMode::SignaturesOnly);
        let store = MemoryCcrStore::default();

        assert_eq!(transform.estimate_bloat(&input), 0.0);
        assert!(transform.apply(&input, &store).is_none());
    }

    #[test]
    fn stub_code_expands_matched_symbol() {
        let mut src = String::from("pub fn visible() {\n");
        for i in 0..10 {
            src.push_str(&format!("    println!(\"visible {i}\");\n"));
        }
        src.push_str("}\n\nfn hidden() {\n");
        for i in 0..10 {
            src.push_str(&format!("    println!(\"hidden {i}\");\n"));
        }
        src.push_str("}\n");

        let out = stub_code(
            &src,
            Some("rs"),
            &StubMode::MatchedSymbols(vec!["visible".to_string()]),
            usize::MAX,
        );

        assert!(out.text.contains("visible 9"), "{}", out.text);
        assert!(!out.text.contains("hidden 9"), "{}", out.text);
        assert_eq!(out.elisions.len(), 1);
    }

    #[test]
    fn stub_code_public_api_omits_private_declarations() {
        let mut src = String::from("use std::fmt;\n\npub fn visible() {\n");
        for i in 0..10 {
            src.push_str(&format!("    println!(\"visible {i}\");\n"));
        }
        src.push_str("}\n\nfn hidden() {\n");
        for i in 0..10 {
            src.push_str(&format!("    println!(\"hidden {i}\");\n"));
        }
        src.push_str("}\n");

        let out = stub_code(&src, Some("rs"), &StubMode::PublicApi, usize::MAX);

        assert!(out.text.contains("use std::fmt;"), "{}", out.text);
        assert!(out.text.contains("pub fn visible()"), "{}", out.text);
        assert!(out.text.contains("private declaration"), "{}", out.text);
        assert!(!out.text.contains("fn hidden"), "{}", out.text);
        assert!(!out.text.contains("hidden 9"), "{}", out.text);
        assert_eq!(out.symbols.len(), 1);
        assert_eq!(out.symbols[0].name, "visible");
        assert!(out.symbols[0].public);
        assert!(
            out.elisions
                .iter()
                .any(|elision| elision.reason == "public_api_private_declaration"),
            "{:?}",
            out.elisions
        );
    }

    #[test]
    fn stub_code_reports_heuristic_fallback_for_unknown_language() {
        let mut src = String::from("function f() {\n");
        for i in 0..12 {
            src.push_str(&format!("  call({i});\n"));
        }
        src.push_str("}\n");

        let out = stub_code(&src, Some("unknown"), &StubMode::SignaturesOnly, usize::MAX);

        assert_eq!(out.parse_status, ParseStatus::HeuristicFallback);
        assert!(!out.elisions.is_empty());
        assert!(out.text.contains("lines"));
    }

    #[test]
    fn stub_code_public_api_heuristic_omits_private_blocks() {
        let mut src = String::from("export function visible() {\n");
        for i in 0..10 {
            src.push_str(&format!("  console.log(\"visible {i}\");\n"));
        }
        src.push_str("}\n\nfunction hidden() {\n");
        for i in 0..10 {
            src.push_str(&format!("  console.log(\"hidden {i}\");\n"));
        }
        src.push_str("}\n");

        let out = stub_code(&src, Some("unknown"), &StubMode::PublicApi, usize::MAX);

        assert_eq!(out.parse_status, ParseStatus::HeuristicFallback);
        assert!(
            out.text.contains("export function visible()"),
            "{}",
            out.text
        );
        assert!(out.text.contains("private declaration"), "{}", out.text);
        assert!(!out.text.contains("function hidden"), "{}", out.text);
        assert!(!out.text.contains("hidden 9"), "{}", out.text);
        assert_eq!(out.symbols.len(), 1);
        assert_eq!(out.symbols[0].name, "visible");
        assert!(out.symbols[0].public);
    }

    #[test]
    fn stub_code_matched_symbol_heuristic_expands_requested_body() {
        let mut src = String::from("function visible() {\n");
        for i in 0..10 {
            src.push_str(&format!("  console.log(\"visible {i}\");\n"));
        }
        src.push_str("}\n\nfunction hidden() {\n");
        for i in 0..10 {
            src.push_str(&format!("  console.log(\"hidden {i}\");\n"));
        }
        src.push_str("}\n");

        let out = stub_code(
            &src,
            Some("unknown"),
            &StubMode::MatchedSymbols(vec!["visible".to_string()]),
            usize::MAX,
        );

        assert_eq!(out.parse_status, ParseStatus::HeuristicFallback);
        assert!(out.text.contains("visible 9"), "{}", out.text);
        assert!(!out.text.contains("hidden 9"), "{}", out.text);
        assert_eq!(out.symbols.len(), 2);
        assert!(
            out.elisions
                .iter()
                .any(|elision| elision.reason.contains("MatchedSymbols")),
            "{:?}",
            out.elisions
        );
    }

    #[test]
    fn stub_code_expand_around_lines_heuristic_expands_containing_body() {
        let mut src = String::from("function first() {\n");
        for i in 0..10 {
            src.push_str(&format!("  console.log(\"first {i}\");\n"));
        }
        src.push_str("}\n\nfunction second() {\n");
        for i in 0..10 {
            src.push_str(&format!("  console.log(\"second {i}\");\n"));
        }
        src.push_str("}\n");

        let out = stub_code(
            &src,
            Some("unknown"),
            &StubMode::ExpandAroundLines(vec![LineRange::new(7, 7)]),
            usize::MAX,
        );

        assert_eq!(out.parse_status, ParseStatus::HeuristicFallback);
        assert!(out.text.contains("first 9"), "{}", out.text);
        assert!(!out.text.contains("second 9"), "{}", out.text);
        assert_eq!(out.symbols.len(), 2);
        assert!(
            out.elisions
                .iter()
                .any(|elision| elision.reason.contains("ExpandAroundLines")),
            "{:?}",
            out.elisions
        );
    }
}
