# Reduce-JSON Protocol

`reduce_json_str` and `reduce_json_request` provide the library form of the
`tinyjuice reduce-json` machine protocol. The current surface is meant for Rust
hosts, the CLI, and tests; artifact records and durable stats recording are
still product-roadmap work.

## Request Shapes

The protocol accepts either a direct `ToolExecutionInput` JSON object or an
envelope with `input` and `options`.

Direct input:

```json
{
  "toolName": "bash",
  "argv": ["git", "status"],
  "stdout": "On branch main\n\nChanges not staged for commit:\n\tmodified:   src/lib.rs\n"
}
```

Envelope input:

```json
{
  "input": {
    "toolName": "bash",
    "argv": ["cargo", "test"],
    "exitCode": 101,
    "stdout": "running 1 test\ntest api::works ... FAILED\n"
  },
  "options": {
    "maxInlineChars": 1200,
    "trace": true,
    "recordStats": true
  }
}
```

`ToolExecutionInput` fields use camelCase. Common fields are:

- `toolName`: host tool name such as `bash`, `shell`, or `exec`.
- `command` and `argv`: command identity used by rule classification and shell
  policy.
- `stdout`, `stderr`, or `combinedText`: raw tool output. If `combinedText` is
  present, it takes priority.
- `exitCode`: optional process status for failure-preserving reducers.
- `cwd`: optional project-rule lookup root.
- `args`, `metadata`, and timing fields: accepted for host context; raw values
  are not echoed in trace output.

## Options

`options` is optional. Supported fields are:

- `classifier`: force a reducer id, falling back to normal matching if invalid.
- `maxInlineChars`: cap returned inline text.
- `raw`: return raw text without reducing it.
- `noOmit`: compatibility flag recorded as metadata.
- `store`: when reduction omits content and CCR retains the original, return a
  metadata-only CCR reference.
- `storeDir`: accepted for compatibility; explicit per-call artifact storage is
  not wired in the library surface.
- `cwd`: copied to `input.cwd` when the input does not already set it.
- `trace`: include metadata-only classification trace fields.
- `recordStats`: compatibility flag recorded as metadata.

## Response

Successful responses serialize as:

```json
{
  "inlineText": "Changes not staged:\nM: src/lib.rs",
  "stats": {
    "rawChars": 71,
    "reducedChars": 32,
    "ratio": 0.4507042253521127
  },
  "classification": {
    "family": "git-status",
    "matchedReducer": "git/status",
    "confidence": "high",
    "reasons": ["argv0 matched"]
  }
}
```

Optional fields:

- `previewText`: reducer preview when a future surface supplies one.
- `facts`: counter results such as failed tests or errors.
- `metadata`: booleans for `noOmit`, `store`, and `recordStats` requests.
  When `store` succeeds, `metadata.ccr` contains a `token` plus
  `originalChars`. The token references the original raw tool output in CCR;
  it is not embedded in `inlineText`.
- `trace`: metadata-only reducer trace with `toolName`, `argv0`, `rawMode`,
  `maxInlineChars`, `family`, and `matchedReducer`.

Trace output intentionally excludes raw command output, raw arguments, and full
metadata maps.

Example `store` metadata:

```json
{
  "metadata": {
    "storeRequested": true,
    "noOmitRequested": false,
    "recordStatsRequested": false,
    "ccr": {
      "token": "0123456789abcdef0123456789abcdef",
      "originalChars": 4096
    }
  }
}
```

## Errors

Malformed protocol payloads return structured errors instead of echoing input:

```json
{
  "code": "invalid_json",
  "message": "expected `,` or `}` at line 1 column 39"
}
```

Current error codes:

- `invalid_json`: JSON parsing or request-shape decoding failed.
- `nul_byte`: the payload contained a NUL byte.

## Safety Notes

The protocol reducer uses the same built-in, user, and project rule layers as
the Rust reducer. Runtime rule loading remains lenient, while `verify_rules`
and `verify_rule_fixtures` are available for callers that want strict
diagnostics before accepting a rule set.

Shell output uses the default safe-inventory policy. Exact file-content reads,
mixed shell sequences, and unsafe inventory actions pass through unchanged;
safe inventory output can still be summarized.
