# REPL tools

Instead of one compressed blob, TinyJuice can hand the host a short preview and
a handle, then let the host inspect the stored original with small deterministic
ops. This follows the idea in [Recursive Language Models](https://arxiv.org/abs/2512.24601):
treat a long input as an environment to query, not text to read whole. No op
calls a model.

## Turning it on

Set `CompressOptions { repl_handle: true, .. }`. Inputs at or above
`ccr_min_tokens` (with `ccr_enabled`) are stored in the CCR store and replaced by
a preview of at most `repl_preview_chars`, plus a footer naming the handle.
Smaller inputs take the normal pipeline. Default is off. `CompressorKind::Repl`
marks these results; `ccr_token` is the handle.

The preview starts with a one-line stats description (about 100 tokens at most):
estimated tokens, bytes, lines, and a shape by kind, e.g.
`JSON object · 3 keys (total, items[60], meta{1}) · depth 3 · largest: items ~1.2k tok`.
It reports structure only (key names, counts), never values. A head snippet of the
first 500 characters of the input follows, then the extractive preview. The stats
line is also on `CompressedOutput.stats`.

With `repl_handle` on, every footer that offers recovery names the REPL tools,
not only the handle stub's: the partial-view and reformat footers of the
compressors and the LLM summary (`cache::recovery_footer_with`) say
`handle "<hash>"`, a `juice_find` slice read (`mode "sed"`, `-n 120,200p`) and a
`grep` search first, then `juice_summarize`, and the whole-original
`juice_retrieve` last. The hash is the same value either way. An agent shown a
footer offering only `juice_retrieve` fetched whole originals every time; with
the slice read named first it can take the part it needs.

Set `repl_save_dir` to also write the full original to `<dir>/<handle>.txt`
(mode 0600, written once per handle). The path is in the footer and in
`CompressedOutput.saved_path`, so an agent can grep or script over the file.
The host owns cleanup of that directory.

## Ops

Three tools, so the schema stays small.

| Tool | Does |
| --- | --- |
| `juice_find` | Query the output. `query` is read per `mode`. |
| `juice_extract` | `what: links \| headings` from HTML or Markdown. |
| `juice_summarize` | Model-free summary: size, outline or JSON shape, then head and tail, or with a `hint` the parts most relevant to it. |

`juice_find` modes:

| `mode` | `query` is | Returns |
| --- | --- | --- |
| `text` (default) | a case-insensitive substring | matching lines |
| `grep` | a regex (`ignore_case`, `context` lines) | matching lines |
| `regex` | a regex | matches with capture groups |
| `rank` | natural language (`top_k`) | BM25-ranked windows of lines |
| `sed` | a sed subset: `-n 10,20p`, `/re/d`, `s/a/b/g` | lines, numbered against the original |
| `awk` | an awk subset: `-F, '$3 > 5 { print $1, $NF }'` | projected lines |
| `jq` | a jq filter (`jaq`): `.items[] \| select(.size > 10) \| .name` | JSON values |

`sed` supports `p`, `d` and `s///[gip]` with `N`, `$`, `/re/` addresses, ranges and `!`.
`awk` supports one `pattern { print terms }` rule. Neither can write files or run
commands. `jq` rejects `env`, `input`, `debug`, `stderr` and `halt`, caps its output,
and runs under a 2 s deadline.

### Scope

Every tool takes `scope`, a Python-style slice: `[:]`, `[10:50]`, `[:-100]`, `[-200:]`.
It is counted in lines by default; set `unit: "chars"` for characters. Line numbers in
results still refer to the original. Addresses inside `sed` and `awk` (`NR`, `2,3p`)
are relative to the scope, as if the slice were piped in.

### Limits

Every op is capped (`ReplLimits`): hits, `sed`/`awk` lines, output characters, line length
and regex size. A capped result reports how many hits were dropped. Sensitive query
parameters in HTML hrefs are redacted. An unknown or expired handle is an error. For HTML
input, line numbers refer to its Markdown rendering.

## Summary stage settings

`llm_summary_timeout_ms` (default 8000) bounds the host model call. On timeout the result
falls back to a preview plus a handle. `llm_summary_max_output_tokens` (default 1000) caps
the summary. It does not help a host model that spends its cap on reasoning tokens: the
reply comes back empty and the summary fails. Turn reasoning off for the summarizer route.

## Surfaces

- Rust: `tinyjuice::repl::{run_op, run_on_text, ReplOp, ReplOutput, ReplLimits}`.
- `tinytools` feature: `tinyjuice::repl::tools::repl_tools(store, limits)` returns
  `Vec<Box<dyn tinytools::Tool>>` (enables the `jq` feature). Each tool takes `handle` plus
  its own args and is declared read-only and concurrency-safe. Hosts register them; TinyJuice does no dispatch.
- TinyBus: `Repl(handle, op_json) -> reply_json` on the Compression interface.

No speed or compression claims are made here; see `docs/benchmarking.md`.
