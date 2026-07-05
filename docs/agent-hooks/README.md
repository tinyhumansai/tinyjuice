# TinyJuice Agent Hooks

TinyJuice currently ships first-class hook installers for Codex and Claude Code.

| Logo | Client | Command |
| --- | --- | --- |
| <img width="48px" src="https://raw.githubusercontent.com/vincentkoc/tokenjuice/main/docs/client-openai.jpg" alt="Codex" /> | [Codex CLI](https://github.com/openai/codex) | `tinyjuice install codex` |
| <img width="48px" src="https://raw.githubusercontent.com/vincentkoc/tokenjuice/main/docs/client-claude.jpg" alt="Claude Code" /> | [Claude Code](https://docs.anthropic.com/en/docs/claude-code) | `tinyjuice install claude-code` |

## Install The CLI

Install from crates.io:

```sh
cargo install tinyjuice --locked
```

For development builds, install from Git:

```sh
cargo install --git https://github.com/tinyhumansai/tinyjuice --locked --bin tinyjuice
```

Or install the exact code in a local checkout:

```sh
cargo install --path . --locked --bin tinyjuice
```

Make sure Cargo's bin directory is on `PATH` before installing hooks:

```sh
export PATH="$HOME/.cargo/bin:$PATH"
tinyjuice --help
```

## Install Agent Hooks

Install the Codex hook:

```sh
tinyjuice install codex
```

Install the Claude Code hook:

```sh
tinyjuice install claude-code
```

The Codex installer updates `~/.codex/hooks.json`. When TinyJuice compacts a
large result, it emits `hookSpecificOutput.additionalContext`, matching Codex's
hook output model.

The Claude Code installer updates `~/.claude/settings.json`. When TinyJuice
compacts a large result, it emits `hookSpecificOutput.updatedToolOutput`, so
Claude Code sees the compacted tool result rather than the noisy original.

Both installers:

- preserve existing hooks and settings
- replace an older TinyJuice hook for the same host
- write a `.bak` file next to the edited JSON file
- expect `tinyjuice` to be on `PATH` unless `--binary` is supplied

## Custom Paths

Point an agent hook at a specific binary:

```sh
tinyjuice install codex --binary "$HOME/.cargo/bin/tinyjuice"
```

Install into a non-default Claude Code settings file:

```sh
tinyjuice install claude-code --path ~/.claude/settings.json
```

The raw hook entrypoints are also available for custom installers:

```sh
tinyjuice codex-post-tool-use
tinyjuice claude-code-post-tool-use
```

## Recovery Store

Hook invocations use a disk-backed CCR store so recovery tokens survive the
short-lived hook process. By default the store lives under the user's cache
directory at `tinyjuice/ccr`.

Override the store location:

```sh
export TINYJUICE_CCR_DIR=/path/to/tinyjuice-ccr
```

Recover a full original from a hook footer:

```sh
tinyjuice retrieve <token>
```

## Hook Tuning

Useful hook tuning variables:

```sh
export TINYJUICE_MIN_BYTES_TO_COMPRESS=2048
export TINYJUICE_MAX_INLINE_CHARS=1200
export TINYJUICE_CCR_MIN_TOKENS=500
export TINYJUICE_CCR_ENABLED=true
```

Templates remain available for inspection or custom packaging:

```sh
tinyjuice hosts
tinyjuice host-template codex
tinyjuice host-template claude-code
tinyjuice host-template generic-json
```
