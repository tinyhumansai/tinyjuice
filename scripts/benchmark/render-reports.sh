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
    polyglot-source) echo "Polyglot Source And XML" ;;
    github-source) echo "GitHub Source Files" ;;
    github-logs) echo "GitHub Log Files" ;;
    dockerfiles) echo "Dockerfiles" ;;
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
    polyglot-source)
      echo "Representative TypeScript, Python, C++, Go, and Rust sources plus an XML document. The language-agnostic code compressor collapses deep bodies across all of them (tree-sitter grammars refine Rust/TS/Python), and XML routes through the readable-text extractor."
      ;;
    github-source)
      echo "Real source files fetched from popular public GitHub repositories (vscode, django, flask, requests, gin, cobra, leveldb, redis, curl, tokio, ripgrep, gson, guava, okhttp, rails, laravel, swift-argument-parser, Newtonsoft.Json, Maven POMs). Sources and licenses are listed in ATTRIBUTION.md."
      ;;
    github-logs)
      echo "Real log files from public GitHub repositories: the loghub corpus (HDFS, Hadoop, Spark, Zookeeper, BGL, HPC, Thunderbird, Windows, Linux, Android, HealthApp, Apache, Proxifier, OpenSSH, OpenStack, macOS), Elastic example datasets (Apache/nginx access logs, auth.log), and CrowdSec parser test fixtures (WAF, Traefik, Authelia, GitLab, Suricata). Sources and licenses in ATTRIBUTION.md."
      ;;
    dockerfiles)
      echo "Real Dockerfiles from official Docker library images and popular projects (postgres, redis, python, golang, mongo, nginx, kubernetes, moby, grafana, prometheus, airflow, and more). Sources and licenses in ATTRIBUTION.md."
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
  output_name="$(output_file_name "$1" "${2:-}")"
  lang_for_file "$output_name" "$1" "${2:-}"
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
    polyglot-source|github-source)
      case "$doc_dir" in
        */??-ts-*) echo "input.ts" ;;
        */??-js-*) echo "input.js" ;;
        */??-py-*) echo "input.py" ;;
        */??-cpp-*) echo "input.cpp" ;;
        */??-c-*) echo "input.c" ;;
        */??-go-*) echo "input.go" ;;
        */??-rs-*) echo "input.rs" ;;
        */??-java-*) echo "input.java" ;;
        */??-kt-*) echo "input.kt" ;;
        */??-rb-*) echo "input.rb" ;;
        */??-php-*) echo "input.php" ;;
        */??-swift-*) echo "input.swift" ;;
        */??-cs-*) echo "input.cs" ;;
        */??-xml-*) echo "input.xml" ;;
        *) echo "input.txt" ;;
      esac
      ;;
    github-logs) echo "input.log" ;;
    dockerfiles) echo "input.dockerfile" ;;
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
    polyglot-source|github-source)
      case "${2:-}" in
        */??-ts-*) echo "output.ts" ;;
        */??-js-*) echo "output.js" ;;
        */??-py-*) echo "output.py" ;;
        */??-cpp-*) echo "output.cpp" ;;
        */??-c-*) echo "output.c" ;;
        */??-go-*) echo "output.go" ;;
        */??-rs-*) echo "output.rs" ;;
        */??-java-*) echo "output.java" ;;
        */??-kt-*) echo "output.kt" ;;
        */??-rb-*) echo "output.rb" ;;
        */??-php-*) echo "output.php" ;;
        */??-swift-*) echo "output.swift" ;;
        */??-cs-*) echo "output.cs" ;;
        */??-xml-*) echo "output.txt" ;;
        *) echo "output.txt" ;;
      esac
      ;;
    github-logs) echo "output.log" ;;
    dockerfiles) echo "output.dockerfile" ;;
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
  polyglot-source
  github-source
  github-logs
  dockerfiles
  plain-text
)

for category in "${categories[@]}"; do
  readme="$bench_root/$category/README.md"
  title="$(category_title "$category")"
  {
    printf '# %s\n\n' "$title"
    printf '%s\n\n' "$(category_summary "$category")"
    printf 'Each row links to the full raw input and the exact compacted output used by the benchmark. Percentages are **token reduction: higher is better**; 0%% means pass-through. `Algorithm` is the compressor-only reduction. `Pass 1` disables CCR (compressed with omission markers, no recovery footer). `Pass 2` is the final model-facing result with CCR enabled — it reads marginally *lower* than Pass 1 only because the recovery footer adds a few dozen bytes to the output.\n\n'
    if [[ "$category" == "unified-diff" ]]; then
      printf 'Inline previews are fenced as `diff`, so GitHub highlights additions and removals directly in the report.\n\n'
    fi
    printf '## Cases\n\n'
    printf 'Every case links to the raw input, the exact model-facing output (with the CCR recovery footer), and a unified diff between the two.\n\n'
    printf '| Case | Input | Output (after CCR) | Diff | Original | Algorithm | Pass 1: no CCR | Pass 2: with CCR | Avg latency | CCR |\n'
    printf '| --- | --- | --- | --- | ---: | ---: | ---: | ---: | ---: | --- |\n'

    jq -r --arg category "$category" '
      [.cases[] | select(.docDir | startswith($category + "/cases/"))]
      | sort_by(.algorithmTokenReductionPercent, .tokenReductionPercent)
      | reverse
      | .[]
      | [
          .docDir,
          .originalBytes,
          (.algorithmTokenReductionPercent | tostring),
          (.noCcrTokenReductionPercent | tostring),
          (.tokenReductionPercent | tostring),
          (.avgLatencyMs | tostring),
          ((.ccrRecoverable // "n/a") | tostring)
        ]
      | @tsv
    ' "$json_report" | while IFS=$'\t' read -r doc_dir original algo pass1 pass2 latency ccr; do
      case_name="${doc_dir##*/}"
      rel_dir="${doc_dir#"$category/"}"
      input_name="$(input_file_name "$category" "$doc_dir")"
      output_name="$(output_file_name "$category" "$doc_dir")"
      input_file="$bench_root/$doc_dir/$input_name"
      output_file="$bench_root/$doc_dir/$output_name"
      # Render a unified diff between input and compacted output so the
      # removed noise / kept signal is reviewable at a glance.
      diff -u \
        -L "input/$input_name" \
        -L "output/$output_name" \
        "$input_file" "$output_file" \
        >"$bench_root/$doc_dir/compression.diff" || true
      printf '| `%s` | [input](%s/%s) | [output](%s/%s) | [diff](%s/compression.diff) | %s | %.1f%% | %.1f%% | %.1f%% | %.3f ms | %s |\n' \
        "$case_name" \
        "$rel_dir" \
        "$input_name" \
        "$rel_dir" \
        "$output_name" \
        "$rel_dir" \
        "$(format_bytes "$original")" \
        "$algo" \
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
      github-source)
        printf 'Real-world source compresses the same way as the synthetic corpus: signatures, imports, and top-level structure stay; deep bodies collapse behind per-block retrieval tokens. Languages without a tree-sitter grammar use the brace-depth heuristic; brace-less languages (Ruby) pass through.\n'
        ;;
      github-logs)
        printf 'The signal-based log path keeps errors, warnings, stack frames, and summaries and collapses the rest behind per-gap retrieval tokens. Logs with no failure signal (pure access logs) are deliberately passed through rather than blindly truncated.\n'
        ;;
      dockerfiles)
        printf 'Dockerfiles are a coverage/honesty category: they carry no collapsible structure the current compressors target, so the router should pass them through untouched rather than mangle them.\n'
        ;;
      polyglot-source)
        printf 'The brace-depth heuristic is language-agnostic, so TypeScript, C++, and Go compress with the same signature-preserving collapse as Rust; tree-sitter grammars refine Rust, TypeScript, and Python. XML goes through the readable-text extractor, keeping element text while dropping markup. Every collapsed block carries its own retrieval token.\n'
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
      output_name="$(output_file_name "$category" "$doc_dir")"
      input_file="$bench_root/$doc_dir/$input_name"
      output_file="$bench_root/$doc_dir/$output_name"
      printf '### `%s`\n\n' "$case_name"
      printf -- '- [Full input](%s/%s)\n' "$rel_dir" "$input_name"
      printf -- '- [Full output](%s/%s)\n' "$rel_dir" "$output_name"
      printf -- '- [Input vs output diff](%s/compression.diff)\n\n' "$rel_dir"
      printf 'Input excerpt:\n\n'
      printf '```%s\n' "$(input_lang "$category" "$doc_dir")"
      snippet "$input_file"
      printf '\n```\n\n'
      printf 'Output excerpt:\n\n'
      printf '```%s\n' "$(output_lang "$category" "$doc_dir")"
      snippet "$output_file"
      printf '\n```\n\n'
    done
  } >"$readme"
done

{
  printf '# Benchmark Sample Reports\n\n'
  printf 'This folder contains ordered, human-readable benchmark reports. Each category has 10 real snapshots under `cases/`, and every case links to its full raw input and full compacted output.\n\n'
  printf 'The current corpus is generated by `scripts/benchmark/update-real-samples.sh`. It uses an adjacent OpenHuman checkout, live OpenHuman Docker logs when available, public RSS/page snapshots when reachable, and checked-in OpenHuman artifacts as fallbacks.\n\n'
  printf 'Percentages are **token reduction: higher is better** (90%% means the output shrank to a tenth of its size; 0%% means it passed through untouched). `Applied` counts the cases where compression actually fired — the rest pass through because they are too small or a shape the compressor declines.\n\n'
  printf '| Category | Cases | Applied | Token reduction (mean) | Avg latency | Report |\n'
  printf '| --- | ---: | ---: | ---: | ---: | --- |\n'
  {
    for category in "${categories[@]}"; do
      title="$(category_title "$category")"
      stats="$(jq -r --arg category "$category" '
        [.cases[] | select(.docDir | startswith($category + "/cases/"))] as $cases
        | [
            ($cases | length),
            ([$cases[] | select(.applied)] | length),
            (($cases | map(.algorithmTokenReductionPercent) | add) / ($cases | length)),
            (($cases | map(.avgLatencyMs) | add) / ($cases | length))
          ]
        | @tsv
      ' "$json_report")"
      IFS=$'\t' read -r count applied avg_algo avg_latency <<<"$stats"
      printf '%.12f\t| %s | %s | %s | %.1f%% | %.3f ms | [report](%s/README.md) |\n' \
        "$avg_algo" "$title" "$count" "$applied" "$avg_algo" "$avg_latency" "$category"
    done
  } | sort -t $'\t' -k1,1nr | cut -f2-
} >"$bench_root/README.md"
