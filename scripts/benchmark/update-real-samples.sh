#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
openh_root="${OPENHUMAN_ROOT:-$(cd "$repo_root/.." && pwd)/openhuman-2}"
bench_root="$repo_root/docs/benchmark"
case_count="${BENCHMARK_CASES_PER_CATEGORY:-10}"

require_cmd() {
  if ! command -v "$1" >/dev/null 2>&1; then
    echo "missing required command: $1" >&2
    exit 1
  fi
}

case_dir() {
  local category="$1"
  local index="$2"
  local slug="$3"
  printf '%s/%s/cases/%02d-%s' "$bench_root" "$category" "$index" "$slug"
}

input_name() {
  local category="$1"
  local slug="${2:-}"
  case "$category" in
    json-smartcrusher) echo "input.json" ;;
    test-failure-log|service-log) echo "input.log" ;;
    search-results) echo "input.rg" ;;
    unified-diff) echo "input.diff" ;;
    html-status-report)
      if [[ "$slug" == rss-* ]]; then
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

output_name() {
  local category="$1"
  case "$category" in
    test-failure-log|service-log) echo "output.log" ;;
    search-results) echo "output.rg" ;;
    unified-diff) echo "output.diff" ;;
    rust-source) echo "output.rs" ;;
    plain-text) echo "output.md" ;;
    *) echo "output.txt" ;;
  esac
}

reset_category() {
  local category="$1"
  rm -rf "$bench_root/$category/cases"
  mkdir -p "$bench_root/$category/cases"
  rm -f "$bench_root/$category"/full-input.txt "$bench_root/$category"/full-output.txt
}

copy_case() {
  local category="$1"
  local index="$2"
  local slug="$3"
  local source="$4"
  local dir
  local input_file
  dir="$(case_dir "$category" "$index" "$slug")"
  input_file="$(input_name "$category" "$slug")"
  mkdir -p "$dir"
  cp "$source" "$dir/$input_file"
  if [[ ! -s "$dir/$input_file" ]]; then
    echo "empty case input: $dir/$input_file" >&2
    exit 1
  fi
}

write_case() {
  local category="$1"
  local index="$2"
  local slug="$3"
  shift 3
  local dir
  local input_file
  dir="$(case_dir "$category" "$index" "$slug")"
  input_file="$(input_name "$category" "$slug")"
  mkdir -p "$dir"
  "$@" >"$dir/$input_file"
  if [[ ! -s "$dir/$input_file" ]]; then
    echo "empty case input: $dir/$input_file" >&2
    exit 1
  fi
}

sanitize_openhuman_paths() {
  sed "s#$openh_root#<OPENHUMAN_ROOT>#g"
}

require_cmd cargo
require_cmd jq
require_cmd rg

if [[ ! -d "$openh_root" ]]; then
  echo "OpenHuman checkout not found: $openh_root" >&2
  echo "Set OPENHUMAN_ROOT=/path/to/openhuman to use another checkout." >&2
  exit 1
fi

echo "Using OpenHuman checkout: $openh_root"

mkdir -p "$bench_root"

for category in \
  json-smartcrusher \
  test-failure-log \
  service-log \
  search-results \
  unified-diff \
  html-status-report \
  rust-source \
  plain-text
do
  reset_category "$category"
done

echo "Writing JSON tool-catalog cases"
for i in $(seq 1 "$case_count"); do
  start=$(( (i - 1) * 50 ))
  write_case "json-smartcrusher" "$i" "github-tools-$i" \
    jq ".result.result.tools[$start:$((start + 50))]" "$openh_root/tests/fixtures/composio_github.json"
done

echo "Writing Vitest command-log cases"
vitest_logs=()
while IFS= read -r path; do
  vitest_logs+=("$path")
done < <(find "$openh_root/target/debug-logs" -type f -name '*.log' -print 2>/dev/null | sort)
if [[ "${#vitest_logs[@]}" -eq 0 ]]; then
  echo "no OpenHuman debug logs found under $openh_root/target/debug-logs" >&2
  exit 1
fi
for i in $(seq 1 "$case_count"); do
  source="${vitest_logs[$(( (i - 1) % ${#vitest_logs[@]} ))]}"
  if [[ "$i" -le "${#vitest_logs[@]}" ]]; then
    write_case "test-failure-log" "$i" "vitest-$(basename "$source" .log)" \
      sanitize_openhuman_paths <"$source"
  else
    write_case "test-failure-log" "$i" "vitest-excerpt-$i" \
      bash -c '{ sed -n "1,80p" "$1"; tail -n 80 "$1"; } | sed "s#$2#<OPENHUMAN_ROOT>#g"' _ "$source" "$openh_root"
  fi
done

echo "Writing runtime/service log cases"
service_source="$openh_root/crahs.log"
service_first_written=0
if command -v docker >/dev/null 2>&1; then
  container="$(docker ps --format '{{.Names}}' 2>/dev/null | rg -i 'openhuman' | head -n 1 || true)"
  if [[ -n "${container:-}" ]]; then
    write_case "service-log" 1 "docker-openhuman-live" \
      docker logs --tail 5000 "$container"
    service_first_written=1
  fi
fi
if [[ ! -s "$service_source" ]]; then
  echo "missing service source log: $service_source" >&2
  exit 1
fi
total_lines="$(wc -l <"$service_source" | tr -d ' ')"
chunk_size=$(( (total_lines + case_count - 1) / case_count ))
for i in $(seq 1 "$case_count"); do
  if [[ "$i" -eq 1 && "$service_first_written" -eq 1 ]]; then
    continue
  fi
  start=$(( (i - 1) * chunk_size + 1 ))
  end=$(( i * chunk_size ))
  write_case "service-log" "$i" "openhuman-crash-slice-$i" \
    bash -c 'sed -n "${2},${3}p" "$1"' _ "$service_source" "$start" "$end"
done

echo "Writing ripgrep result cases"
queries=(
  "tokenjuice"
  "compression"
  "retrieve"
  "OpenHuman"
  "agent"
  "memory"
  "workflow"
  "tinyplace"
  "provider"
  "subconscious"
)
for i in $(seq 1 "$case_count"); do
  query="${queries[$(( (i - 1) % ${#queries[@]} ))]}"
  slug="$(printf '%s' "$query" | tr '[:upper:]' '[:lower:]' | tr -c 'a-z0-9' '-')"
  write_case "search-results" "$i" "rg-$slug" \
    bash -c 'rg -n -i "$2" "$1" --glob "!node_modules/**" --glob "!target/**" --glob "!app/dist/**" --glob "!app/dist-web/**" | sed "s#$1#<OPENHUMAN_ROOT>#g" | sed -n "1,500p"' _ "$openh_root" "$query"
done

echo "Writing unified diff cases"
write_case "unified-diff" 1 "tinyjuice-worktree" \
  bash -c 'cd "$1" && git diff --no-ext-diff --unified=80 -- README.md docs/benchmarking.md examples/compression_benchmark.rs scripts/benchmark/update-real-samples.sh' _ "$repo_root"
for i in $(seq 2 "$case_count"); do
  commit="$(git -C "$openh_root" rev-list --max-count=1 --skip=$((i - 2)) HEAD)"
  write_case "unified-diff" "$i" "openhuman-commit-$i" \
    bash -c 'git -C "$1" show --format= --no-ext-diff --unified=80 "$2" | sed "s#$1#<OPENHUMAN_ROOT>#g"' _ "$openh_root" "$commit"
done

echo "Writing HTML/RSS/page cases"
html_index=1
fetch_page() {
  local url="$1"
  local slug="$2"
  if command -v curl >/dev/null 2>&1; then
    local dir
    local input_file
    dir="$(case_dir "html-status-report" "$html_index" "$slug")"
    input_file="$(input_name "html-status-report" "$slug")"
    mkdir -p "$dir"
    if curl -L --max-time 15 --retry 1 -A 'tinyjuice-benchmark-snapshot/1.0' "$url" >"$dir/$input_file" 2>/dev/null \
      && [[ -s "$dir/$input_file" ]]; then
      html_index=$((html_index + 1))
      return 0
    fi
    rm -f "$dir/$input_file"
  fi
  return 1
}
fetch_page "https://blog.rust-lang.org/feed.xml" "rss-rust-blog" || true
fetch_page "https://hnrss.org/frontpage" "rss-hacker-news" || true
fetch_page "https://news.ycombinator.com/" "noisy-hacker-news" || true
fetch_page "https://users.rust-lang.org/" "forum-rust-users" || true

html_files=()
while IFS= read -r path; do
  html_files+=("$path")
done < <(find "$openh_root/app/coverage/lcov-report" -type f -name '*.html' -print 2>/dev/null | sort)
if [[ "${#html_files[@]}" -eq 0 ]]; then
  echo "no OpenHuman coverage HTML files found" >&2
  exit 1
fi
while [[ "$html_index" -le "$case_count" ]]; do
  source="${html_files[$(( (html_index - 1) % ${#html_files[@]} ))]}"
  write_case "html-status-report" "$html_index" "openhuman-coverage-$html_index" \
    sanitize_openhuman_paths <"$source"
  html_index=$((html_index + 1))
done

echo "Writing Rust source cases"
rust_files=()
while IFS= read -r path; do
  if rg -q '(^|[[:space:]])fn[[:space:]]+[A-Za-z_]' "$path"; then
    rust_files+=("$path")
  fi
done < <(find "$openh_root/src" "$openh_root/tests" -type f -name '*.rs' -not -path '*/target/*' -not -path '*/vendor/*' -print 2>/dev/null | sort)
if [[ "${#rust_files[@]}" -eq 0 ]]; then
  echo "no OpenHuman Rust files found" >&2
  exit 1
fi
for i in $(seq 1 "$case_count"); do
  source="${rust_files[$(( (i - 1) % ${#rust_files[@]} ))]}"
  slug="$(basename "$source" .rs | tr '_' '-')"
  write_case "rust-source" "$i" "$slug" \
    sanitize_openhuman_paths <"$source"
done

echo "Writing plain-text/Markdown cases"
text_files=()
while IFS= read -r path; do
  text_files+=("$path")
done < <(find "$openh_root" -maxdepth 3 \( -name '*.md' -o -name '*.txt' \) -not -path '*/target/*' -not -path '*/node_modules/*' -not -path '*/vendor/*' -print 2>/dev/null | sort)
if [[ "${#text_files[@]}" -eq 0 ]]; then
  echo "no OpenHuman text files found" >&2
  exit 1
fi
for i in $(seq 1 "$case_count"); do
  source="${text_files[$(( (i - 1) % ${#text_files[@]} ))]}"
  slug="$(basename "$source" | tr '[:upper:]' '[:lower:]' | tr -c 'a-z0-9' '-')"
  write_case "plain-text" "$i" "$slug" \
    sanitize_openhuman_paths <"$source"
done

(
  cd "$repo_root"
  cargo run --release --example compression_benchmark -- --dump-samples docs/benchmark
)

if command -v perl >/dev/null 2>&1; then
  escaped_openh_root="$(perl -e 'print quotemeta shift' "$openh_root")"
  escaped_home="$(perl -e 'print quotemeta shift' "${HOME:-}")"
  find "$bench_root" -type f \( -name 'input.*' -o -name 'output.*' \) -print0 \
    | xargs -0 perl -pi -e "s#$escaped_openh_root#<OPENHUMAN_ROOT>#g; s#$escaped_home#<USER_HOME>#g"
fi
