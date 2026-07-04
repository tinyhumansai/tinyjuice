# SQL Introspection Reduction Spec

## Status

Design reference for replacing migration-file dumps and raw database inspection
output with compact schema answers.

## Goal

Return only the schema facts needed for a task:

- relevant tables;
- columns and types;
- primary keys;
- foreign-key edges;
- indexes;
- compact query results.

## Inputs

```text
SqlContextInput {
  dialect: SqlDialect,
  schema_sources: Vec<SchemaSource>,
  query: SqlQuestion,
  max_tables: usize,
  max_rows: usize,
  max_bytes: usize,
}

SchemaSource {
  MigrationFiles(Vec<PathBuf>),
  LiveConnection(ConnectionRef),
  InlineSql(String),
}
```

## Algorithm

1. Parse migrations or introspect a live database.
2. Build a schema graph of tables, columns, keys, indexes, and views.
3. Rank schema nodes against the user query.
4. Return the connected subgraph around the highest-ranked nodes.
5. For query execution, cap rows and cell widths.
6. Include truncation and rewrite reports.

## Dialect Fixes

Safe rewrite candidates:

- convert backtick identifiers when rejected by the dialect;
- quote reserved aliases;
- rewrite unsupported `COUNT(DISTINCT a, b)` forms;
- normalize date truncation syntax when dialect-specific.

Every rewrite must be reported. Unsafe rewrites should be rejected with a hint.

## Output

```text
SqlContextOutput {
  schema: SchemaSubgraph,
  query_result: Option<CompactRows>,
  rewrites: Vec<SqlRewrite>,
  warnings: Vec<String>,
  report: CompressionReport,
}
```

## Safety Rules

- Never store or report credentials.
- Do not execute mutating SQL in a reducer path.
- Label live query results separately from parsed migration facts.
- Include omitted table and row counts.

## TinyJuice Fit

Suggested modules:

```text
src/compressors/sql.rs
src/types.rs
src/rules/
```

Database drivers should sit behind features or adapters. The core crate can
accept already-extracted schema data.

## Test Fixtures

- migration parser;
- foreign-key graph;
- reserved alias rewrite;
- row/cell truncation;
- mutating query rejection.
