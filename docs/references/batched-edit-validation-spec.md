# Batched Edit, Fuzzy Match, and Validation Spec

## Status

Design reference for reducing retry loops and verification reads. This is a
tool-round-trip reduction strategy, not text compression.

## Goal

Apply many edits in one operation, tolerate harmless match drift, validate the
result, and return enough evidence to avoid a separate verification read.

## Inputs

```text
BatchEditInput {
  edits: Vec<FileEdit>,
  validation: ValidationPolicy,
}

FileEdit {
  path: PathBuf,
  operations: Vec<EditOperation>,
}

EditOperation {
  old_text: String,
  new_text: String,
  match_policy: MatchPolicy,
}

MatchPolicy {
  Exact,
  WhitespaceInsensitive,
  UnicodePunctuationInsensitive,
  BoundedLevenshtein { max_distance: usize },
}
```

## Algorithm

1. Group edits by file.
2. Load each file once.
3. For each operation:
   - try exact match;
   - optionally normalize whitespace;
   - optionally normalize visually similar punctuation;
   - optionally use bounded Levenshtein search over candidate windows;
   - reject ambiguous matches.
4. Apply edits in offset-descending order.
5. Write each file once.
6. Run selected validators.
7. Return changed files, validation status, and compact actionable errors.

## Validators

Initial validators:

- JSON parse;
- YAML parse;
- TOML parse;
- HTML parse or tag-balance check;
- TypeScript/JavaScript syntax parse where tooling is available;
- SQL parse/lint when dialect is known;
- broader project checks only when explicitly requested.

## Output

```text
BatchEditOutput {
  applied: Vec<AppliedEdit>,
  rejected: Vec<RejectedEdit>,
  validations: Vec<ValidationResult>,
  report: CompressionReport,
}
```

## Safety Rules

- Never apply a fuzzy match when multiple candidate windows are plausible.
- Include before/after line ranges for applied operations.
- Preserve rollback support or fail before writing.
- Avoid returning full post-edit files by default.

## TinyJuice Fit

The core crate should define policy/report types. Filesystem mutation should
live in a host adapter.

Suggested modules:

```text
src/compressor/types.rs
src/openhuman/types.rs
src/text/process.rs
```

## Test Fixtures

- exact replacement;
- indentation drift;
- curly quote vs straight quote equivalence;
- ambiguous fuzzy match rejection;
- multi-edit offset handling;
- validation failure without full-file echo.
