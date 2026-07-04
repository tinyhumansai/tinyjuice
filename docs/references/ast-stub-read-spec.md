# AST Stub Read Spec

## Status

Design reference for reducing full-file reads by preserving source structure
while eliding low-value implementation bodies.

## Goal

Return a structural view of a source file:

- imports;
- type declarations;
- exported symbols;
- function signatures;
- selected comments;
- body stubs instead of full bodies.

## Inputs

```text
AstStubInput {
  path: PathBuf,
  language: Language,
  source: String,
  mode: StubMode,
  max_bytes: usize,
}

StubMode {
  SignaturesOnly,
  PublicApi,
  MatchedSymbols(Vec<String>),
  ExpandAroundLines(Vec<LineRange>),
}
```

## Output

```text
AstStubOutput {
  text: String,
  symbols: Vec<SymbolSummary>,
  elisions: Vec<Elision>,
  parse_status: ParseStatus,
}
```

## Algorithm

1. Detect language from extension and content.
2. Parse with a real parser where available.
3. Preserve top-level imports and exports.
4. Preserve type, class, interface, trait, enum, and public API declarations.
5. Replace function or method bodies with stable placeholders:

```text
fn example(arg: Type) -> ReturnType { /* ... */ }
```

6. Expand selected bodies for requested symbols or line ranges.
7. If parsing fails, fall back to lexical brace-aware truncation and report the
   fallback.

## Initial Language Priority

- TypeScript / JavaScript;
- Rust;
- Python;
- Go;
- JSON/YAML/TOML as structured data.

## Safety Rules

- Do not stub bodies when exact implementation behavior is requested.
- Mark elided ranges with line numbers.
- Do not claim the stubbed result is compilable.
- Prefer parse failure plus fallback over silent omission.

## TinyJuice Fit

Suggested modules:

```text
src/compressors/code.rs
src/text/process.rs
src/reduce.rs
```

## Test Fixtures

- exported function body becomes a stub;
- type declarations remain intact;
- matched symbol body expands;
- parse failure records fallback;
- budget truncates at structural boundaries.
