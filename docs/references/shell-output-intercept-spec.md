# Shell Output Intercept Spec

## Status

Design reference for reducing raw shell output before it reaches model context.

## Goal

Recognize high-volume shell commands and route their output through structured
reducers instead of dumping raw terminal text into context.

Examples:

```text
cat large_file.ts      -> AST stub read
grep -R symbol src/    -> ranked search
find . -type f         -> capped file inventory
git status             -> structured git status
docker logs service    -> log reducer
```

## Inputs

```text
ShellInterceptInput {
  command: String,
  cwd: PathBuf,
  stdout: String,
  stderr: String,
  exit_code: i32,
  max_bytes: usize,
}
```

## Algorithm

1. Parse executable and arguments.
2. Match against a command rule registry.
3. Transform stdout/stderr into a structured reducer input when possible.
4. Preserve exit code and failure state.
5. Fall back to generic line/window truncation when no rule matches.

## Rule Shape

```text
InterceptRule {
  command: String,
  intent: CommandIntent,
  parser: OutputParser,
  reducer: ReducerKind,
  safety: SafetyPolicy,
}
```

## Safety Rules

- Preserve exit code and stderr presence.
- Never hide failing command output entirely.
- Do not reinterpret side-effecting commands as read-only.
- Keep raw excerpts for diagnostics.

## TinyJuice Fit

The existing rule scaffold can support command-intent fixtures:

```text
src/rules/
src/vendor/rules/
src/compressors/log.rs
src/compressors/code.rs
src/compressors/search.rs
```

The host adapter decides whether interception or command rewriting is allowed.

## Test Fixtures

- `grep` routes to search reducer;
- `cat` source file routes to AST stub reducer;
- failing command preserves stderr;
- unknown command uses generic truncation;
- ambiguous rule mappings are rejected.
