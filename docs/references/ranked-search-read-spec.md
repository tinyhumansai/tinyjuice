# Ranked Search and Selective Read Spec

## Status

Design reference for reducing tool I/O before it enters model context. This is
not an implementation commitment or benchmark claim.

## Problem

Naive code exploration often expands into separate calls:

```text
glob -> grep -> read full file(s)
```

Each result is then re-ingested on later turns. The strategy is to collapse that
sequence into one structured operation that returns only the strongest evidence.

## Inputs

```text
SearchReadInput {
  root: Path,
  include_globs: Vec<String>,
  exclude_globs: Vec<String>,
  query: Query,
  max_files: usize,
  max_snippets_per_file: usize,
  max_bytes: usize,
}

Query {
  literal: Option<String>,
  regex: Option<String>,
  symbols: Vec<String>,
  file_kinds: Vec<FileKind>,
}
```

## Output

```text
SearchReadOutput {
  matches: Vec<FileMatch>,
  omitted: OmissionReport,
  report: CompressionReport,
}

FileMatch {
  path: PathBuf,
  score: f32,
  snippets: Vec<Snippet>,
  imports: Vec<ImportEdge>,
  exports: Vec<String>,
}
```

## Ranking

Rank files using a weighted score:

```text
score =
  exact_symbol_match * 8.0 +
  path_match * 4.0 +
  regex_match_density * 3.0 +
  import_export_match * 2.0 +
  recency_or_git_touch * 1.0 -
  generated_file_penalty -
  vendor_path_penalty
```

Weights are placeholders until fixtures prove them.

## Snippet Selection

1. Include lines around direct matches.
2. Include surrounding symbol signatures when available.
3. Include imports and exports that explain dependencies.
4. Merge overlapping windows.
5. Enforce `max_snippets_per_file` and `max_bytes`.
6. Record omitted matches.

## Safety Rules

- Do not synthesize code.
- Make truncation and omitted matches explicit.
- Prefer precise snippets over broad file dumps.
- Preserve path and line metadata for follow-up reads.
- Deprioritize generated, vendored, and lock files unless requested.

## TinyJuice Fit

Suggested modules:

```text
src/compressors/search.rs
src/detect/kind.rs
src/reduce.rs
src/types.rs
```

## Test Fixtures

- exact symbol ranking;
- noisy regex ranking;
- snippet window merge;
- generated-file penalty;
- max-byte truncation with omitted counts.
