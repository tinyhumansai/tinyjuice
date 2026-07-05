#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
bench_root="$repo_root/docs/benchmark"
json_report="${1:-/tmp/tinyjuice-corpus-benchmark.json}"

if [[ ! -s "$json_report" ]]; then
  echo "benchmark JSON not found: $json_report" >&2
  exit 1
fi

format_bytes() {
  local bytes="$1"
  if [[ "$bytes" -ge 1000000 ]]; then
    awk -v b="$bytes" 'BEGIN { printf "%.1f MB", b / 1000000 }'
  elif [[ "$bytes" -ge 1000 ]]; then
    awk -v b="$bytes" 'BEGIN { printf "%.1f KB", b / 1000 }'
  else
    printf '%s B' "$bytes"
  fi
}

snippet() {
  local file="$1"
  awk 'NR <= 36 {
    line = $0
    gsub(/```/, "` ` `", line)
    if (length(line) > 220) {
      print substr(line, 1, 220) "..."
    } else {
      print line
    }
  }' "$file"
}

category_title() {
  case "$1" in
    json-smartcrusher) echo "JSON SmartCrusher" ;;
    test-failure-log) echo "Test Failure Logs" ;;
    service-log) echo "Service And Docker Logs" ;;
    search-results) echo "Search Results" ;;
    unified-diff) echo "Unified Diffs" ;;
    html-status-report) echo "HTML, RSS, And Page Snapshots" ;;
    rust-source) echo "Rust Source" ;;
    plain-text) echo "Plain Text" ;;
    *) echo "$1" ;;
  esac
}

category_summary() {
  case "$1" in
    json-smartcrusher)
      echo "Real OpenHuman JSON snapshots: tool catalogs, API responses, schemas, package metadata, Lottie payloads, and config files. TinyJuice now chooses the smallest useful JSON representation before CCR, using Markdown tables only when they beat minified JSON."
      ;;
    test-failure-log)
      echo "Real OpenHuman Vitest command logs. The command-aware reducer keeps failure summaries and drops repetitive success or setup noise."
      ;;
    service-log)
      echo "Real OpenHuman runtime crash-log slices, with live Docker OpenHuman logs used first when a container is available. The log compressor keeps incident signals and collapses repeated low-value lines."
      ;;
    search-results)
      echo "Real ripgrep result sets from an OpenHuman checkout. TinyJuice groups matches by file, keeps top hits, and records omitted match counts."
      ;;
    unified-diff)
      echo "Real TinyJuice and OpenHuman patches. The diff compressor keeps file headers, hunk headers, and changed lines while collapsing long unchanged context."
      ;;
    html-status-report)
      echo "Real RSS feeds, noisy web pages, forum pages, and OpenHuman coverage HTML. The HTML compressor strips markup/script noise and keeps readable page text."
      ;;
    rust-source)
      echo "Real OpenHuman Rust files. The source compressor keeps imports, signatures, and top-level structure while collapsing large bodies when useful."
      ;;
    plain-text)
      echo "Real OpenHuman Markdown/prose. With deterministic ML text compression disabled, TinyJuice passes plain text through unchanged."
      ;;
  esac
}

input_lang() {
  local input_name
  input_name="$(input_file_name "$1" "${2:-}")"
  lang_for_file "$input_name" "$1" "${2:-}"
}

output_lang() {
  local output_name
  output_name="$(output_file_name "$1")"
  lang_for_file "$output_name" "$1" ""
}

lang_for_file() {
  local file_name="$1"
  local category="$2"
  local doc_dir="${3:-}"

  case "$file_name" in
    *.diff|*.patch) echo "diff" ;;
    *.json) echo "json" ;;
    *.rs) echo "rust" ;;
    *.md) echo "markdown" ;;
    *.xml) echo "xml" ;;
    *.html|*.htm) echo "html" ;;
    *.log|*.rg|*.txt) echo "text" ;;
    *) fallback_lang "$category" "$doc_dir" ;;
  esac
}

fallback_lang() {
  if [[ "${2:-}" == *rss-* ]]; then
    echo "xml"
    return
  fi

  case "$1" in
    json-smartcrusher) echo "json" ;;
    unified-diff) echo "diff" ;;
    html-status-report) echo "html" ;;
    rust-source) echo "rust" ;;
    plain-text) echo "markdown" ;;
    *) echo "text" ;;
  esac
}

input_file_name() {
  local category="$1"
  local doc_dir="${2:-}"
  case "$category" in
    json-smartcrusher) echo "input.json" ;;
    test-failure-log|service-log) echo "input.log" ;;
    search-results) echo "input.rg" ;;
    unified-diff) echo "input.diff" ;;
    html-status-report)
      if [[ "$doc_dir" == *rss-* ]]; then
        echo "input.xml"
      else
        echo "input.html"
      fi
      ;;
    rust-source) echo "input.rs" ;;
    plain-text) echo "input.md" ;;
    *) echo "input.txt" ;;
  esac
}

output_file_name() {
  local category="$1"
  case "$category" in
    json-smartcrusher) echo "output.md" ;;
    test-failure-log|service-log) echo "output.log" ;;
    search-results) echo "output.rg" ;;
    unified-diff) echo "output.diff" ;;
    rust-source) echo "output.rs" ;;
    plain-text) echo "output.md" ;;
    *) echo "output.txt" ;;
  esac
}

categories=(
  json-smartcrusher
  test-failure-log
  service-log
  search-results
  unified-diff
  html-status-report
  rust-source
  plain-text
)

for category in "${categories[@]}"; do
  readme="$bench_root/$category/README.md"
  title="$(category_title "$category")"
  {
    printf '# %s\n\n' "$title"
    printf '%s\n\n' "$(category_summary "$category")"
    printf 'Each row links to the full raw input and the exact compacted output used by the benchmark. `Pass 1` is the accepted result with CCR disabled. `Pass 2` is the final model-facing result with CCR enabled.\n\n'
    if [[ "$category" == "unified-diff" ]]; then
      printf 'Inline previews are fenced as `diff`, so GitHub highlights additions and removals directly in the report.\n\n'
    fi
    printf '## Cases\n\n'
    printf '| Case | Input | Output | Original | Pass 1: no CCR | Pass 2: with CCR | Avg latency | CCR |\n'
    printf '| --- | --- | --- | ---: | ---: | ---: | ---: | --- |\n'

    jq -r --arg category "$category" '
      [.cases[] | select(.docDir | startswith($category + "/cases/"))]
      | sort_by(.noCcrTokenReductionPercent, .tokenReductionPercent)
      | reverse
      | .[]
      | [
          .docDir,
          .originalBytes,
          (.noCcrTokenReductionPercent | tostring),
          (.tokenReductionPercent | tostring),
          (.avgLatencyMs | tostring),
          ((.ccrRecoverable // "n/a") | tostring)
        ]
      | @tsv
    ' "$json_report" | while IFS=$'\t' read -r doc_dir original pass1 pass2 latency ccr; do
      case_name="${doc_dir##*/}"
      rel_dir="${doc_dir#"$category/"}"
      input_name="$(input_file_name "$category" "$doc_dir")"
      output_name="$(output_file_name "$category")"
      printf '| `%s` | [input](%s/%s) | [output](%s/%s) | %s | %.1f%% | %.1f%% | %.3f ms | %s |\n' \
        "$case_name" \
        "$rel_dir" \
        "$input_name" \
        "$rel_dir" \
        "$output_name" \
        "$(format_bytes "$original")" \
        "$pass1" \
        "$pass2" \
        "$latency" \
        "$ccr"
    done

    printf '\n## What TinyJuice Is Doing\n\n'
    case "$category" in
      json-smartcrusher)
        printf 'TinyJuice parses JSON before choosing a representation. Homogeneous object arrays can become GitHub-renderable Markdown tables, but minified JSON wins when it is smaller or when the JSON shape is too nested for a readable table. If neither representation saves space, the router returns the original.\n'
        ;;
      test-failure-log)
        printf 'The command context routes these logs through the Vitest rule. Setup chatter and repeated passing output are removed, while failure blocks, summaries, and locations remain visible.\n'
        ;;
      service-log)
        printf 'The log path scores lines by signal. Errors, warnings, exception metadata, stack frames, and summaries are favored; repetitive routine lines are collapsed behind omission markers.\n'
        ;;
      search-results)
        printf 'Search results are parsed as file/line/body records. TinyJuice groups by file, keeps high-value matches per file, and tells the reader how many additional matches were hidden.\n'
        ;;
      unified-diff)
        printf 'Diff compression preserves review-critical structure: file headers, hunk headers, additions, and removals. Long unchanged context is collapsed because it is recoverable and usually lower value.\n'
        ;;
      html-status-report)
        printf 'HTML snapshots are converted into readable text. Script/style payloads and repeated markup disappear; the output keeps the content an agent would normally inspect.\n'
        ;;
      rust-source)
        printf 'The code path keeps the navigation surface: imports, signatures, top-level items, and important comments. Large function bodies can be collapsed and recovered through CCR.\n'
        ;;
      plain-text)
        printf 'Plain text is the control group. With ML text compression off, the router declines compression and returns the original unchanged whenever deterministic structure is not available.\n'
        ;;
    esac

    printf '\n## Syntax-Aware Samples\n\n'
    jq -r --arg category "$category" '
      [.cases[] | select(.docDir | startswith($category + "/cases/"))]
      | sort_by(.noCcrTokenReductionPercent, .tokenReductionPercent)
      | reverse
      | .[]
      | .docDir
    ' "$json_report" | while IFS= read -r doc_dir; do
      case_name="${doc_dir##*/}"
      rel_dir="${doc_dir#"$category/"}"
      input_name="$(input_file_name "$category" "$doc_dir")"
      output_name="$(output_file_name "$category")"
      input_file="$bench_root/$doc_dir/$input_name"
      output_file="$bench_root/$doc_dir/$output_name"
      printf '### `%s`\n\n' "$case_name"
      printf -- '- [Full input](%s/%s)\n' "$rel_dir" "$input_name"
      printf -- '- [Full output](%s/%s)\n\n' "$rel_dir" "$output_name"
      printf 'Input excerpt:\n\n'
      printf '```%s\n' "$(input_lang "$category" "$doc_dir")"
      snippet "$input_file"
      printf '\n```\n\n'
      printf 'Output excerpt:\n\n'
      printf '```%s\n' "$(output_lang "$category")"
      snippet "$output_file"
      printf '\n```\n\n'
    done
  } >"$readme"
done

{
  printf '# Benchmark Sample Reports\n\n'
  printf 'This folder contains ordered, human-readable benchmark reports. Each category has 10 real snapshots under `cases/`, and every case links to its full raw input and full compacted output.\n\n'
  printf 'The current corpus is generated by `scripts/benchmark/update-real-samples.sh`. It uses an adjacent OpenHuman checkout, live OpenHuman Docker logs when available, public RSS/page snapshots when reachable, and checked-in OpenHuman artifacts as fallbacks.\n\n'
  printf '`Pass 1` is the accepted result with CCR disabled. `Pass 2` is the final model-facing result with CCR enabled.\n\n'
  printf '| Category | Cases | Pass 1: no CCR | Pass 2: with CCR | Avg latency | Report |\n'
  printf '| --- | ---: | ---: | ---: | ---: | --- |\n'
  {
    for category in "${categories[@]}"; do
      title="$(category_title "$category")"
      stats="$(jq -r --arg category "$category" '
        [.cases[] | select(.docDir | startswith($category + "/cases/"))] as $cases
        | [
            ($cases | length),
            (($cases | map(.noCcrTokenReductionPercent) | add) / ($cases | length)),
            (($cases | map(.tokenReductionPercent) | add) / ($cases | length)),
            (($cases | map(.avgLatencyMs) | add) / ($cases | length))
          ]
        | @tsv
      ' "$json_report")"
      IFS=$'\t' read -r count avg_pass1 avg_pass2 avg_latency <<<"$stats"
      printf '%.12f\t%.12f\t| %s | %s | %.1f%% | %.1f%% | %.3f ms | [report](%s/README.md) |\n' \
        "$avg_pass1" "$avg_pass2" "$title" "$count" "$avg_pass1" "$avg_pass2" "$avg_latency" "$category"
    done
  } | sort -t $'\t' -k1,1nr -k2,2nr | cut -f3-
} >"$bench_root/README.md"
