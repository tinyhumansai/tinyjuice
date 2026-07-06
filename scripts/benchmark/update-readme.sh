#!/usr/bin/env bash
# Run the benchmark corpus and refresh everything derived from it:
# per-case artifacts, per-category READMEs, and the summary table in the
# top-level README.md (between the <!-- bench:table --> markers, plus the
# inline <!-- bench:corpus --> stats).
#
# Usage:
#   scripts/benchmark/update-readme.sh              # full pipeline, 3 iterations
#   scripts/benchmark/update-readme.sh -n 10        # more iterations
#   scripts/benchmark/update-readme.sh --skip-dump  # keep existing case artifacts
#   scripts/benchmark/update-readme.sh --report /path/to/report.json
#                                                   # reuse a report, skip the run
set -euo pipefail

cd "$(git rev-parse --show-toplevel)"

iterations=3
skip_dump=0
report=""
while [ $# -gt 0 ]; do
  case "$1" in
    -n) iterations="$2"; shift 2 ;;
    --skip-dump) skip_dump=1; shift ;;
    --report) report="$2"; shift 2 ;;
    *) echo "unknown argument: $1" >&2; exit 2 ;;
  esac
done

if [ -z "$report" ]; then
  report="/tmp/tinyjuice-corpus-benchmark.json"
  if [ "$skip_dump" -eq 0 ]; then
    echo "==> regenerating per-case artifacts (docs/benchmark)"
    cargo run --release --example compression_benchmark -- --dump-samples docs/benchmark >/dev/null
  fi
  echo "==> running benchmark ($iterations iteration(s))"
  cargo run --release --example compression_benchmark -- -n "$iterations" --format json > "$report"
else
  [ -f "$report" ] || { echo "report not found: $report" >&2; exit 1; }
  echo "==> reusing report: $report"
fi

echo "==> rendering per-category reports"
bash scripts/benchmark/render-reports.sh "$report"

echo "==> updating README.md"
REPORT_PATH="$report" python3 - <<'PY'
import json, os, re, sys

report = json.load(open(os.environ["REPORT_PATH"]))
cases = report["cases"]

fails = [c["id"] for c in cases if c.get("accuracyGatePassed") is False]
if fails:
    print("refusing to update README: accuracy gate failures:", file=sys.stderr)
    for cid in fails:
        print(f"  {cid}", file=sys.stderr)
    sys.exit(1)

# Category dir -> README table label. New categories must be added here;
# failing loudly beats silently dropping a row.
LABELS = {
    "service-log": "[Service logs and crash reports](docs/benchmark/service-log/README.md)",
    "polyglot-source": "[Polyglot source and XML](docs/benchmark/polyglot-source/README.md) (TS/Py/C++/Go/Rust/XML)",
    "test-failure-log": "[Test failure logs](docs/benchmark/test-failure-log/README.md)",
    "html-status-report": "[HTML, RSS, and page snapshots](docs/benchmark/html-status-report/README.md)",
    "unified-diff": "[Unified diffs](docs/benchmark/unified-diff/README.md)",
    "github-logs": "[GitHub log files](docs/benchmark/github-logs/README.md) (loghub, Elastic, CrowdSec, lnav, fail2ban)",
    "json-smartcrusher": "[JSON SmartCrusher](docs/benchmark/json-smartcrusher/README.md)",
    "github-source": "[GitHub source files](docs/benchmark/github-source/README.md) (13 languages, real repos + algorithms)",
    "search-results": "[Search results](docs/benchmark/search-results/README.md)",
    "rust-source": "[Rust source](docs/benchmark/rust-source/README.md)",
    "plain-text": "[Plain text with ML off](docs/benchmark/plain-text/README.md)",
}

cats = {}
for c in cases:
    cats.setdefault(c["docDir"].split("/")[0], []).append(c)

unknown = sorted(set(cats) - set(LABELS))
if unknown:
    print(f"unmapped benchmark categories (add to LABELS in {__file__ or 'update-readme.sh'}): {unknown}",
          file=sys.stderr)
    sys.exit(1)

def mb(n):
    return f"{n / 1e6:.1f} MB"

rows = []
for name, cs in cats.items():
    n = len(cs)
    rows.append((
        sum(c["noCcrTokenReductionPercent"] for c in cs) / n,
        sum(c["tokenReductionPercent"] for c in cs) / n,
        sum(c["avgLatencyMs"] for c in cs) / n,
        n,
        sum(1 for c in cs if c["applied"]),
        LABELS[name],
    ))
rows.sort(key=lambda r: -r[0])

table = ["| Category | Cases | Applied | Pass 1: without CCR | Pass 2: with CCR | Avg latency |",
         "| --- | ---: | ---: | ---: | ---: | ---: |"]
for p1, p2, lat, n, applied, label in rows:
    table.append(f"| {label} | {n} | {applied} | {p1:.1f}% | {p2:.1f}% | {lat:.3f} ms |")

total_in = sum(c["originalBytes"] for c in cases)
total_out = sum(c["compactedBytes"] for c in cases)
totals = (
    f"Across the whole corpus TinyJuice cut {mb(total_in)} of content down to\n"
    f"{mb(total_out)}, and every case passes its accuracy gates: signal checks\n"
    "(errors, changed lines, matches, class/function signatures survive), task\n"
    "checks, structural invariants (no inflation, no encoding damage), and a\n"
    "byte-exact CCR recovery compare."
)
block = "<!-- bench:table -->\n" + "\n".join(table) + "\n\n" + totals + "\n<!-- /bench:table -->"
corpus = (f"<!-- bench:corpus -->{mb(total_in)} of real content across "
          f"{len(cases)} cases<!-- /bench:corpus -->")

readme = open("README.md").read()
updated, n1 = re.subn(r"<!-- bench:table -->.*?<!-- /bench:table -->", block,
                      readme, flags=re.S)
updated, n2 = re.subn(r"<!-- bench:corpus -->.*?<!-- /bench:corpus -->", corpus,
                      updated, flags=re.S)
if n1 != 1 or n2 != 1:
    print(f"README markers missing or duplicated (table={n1}, corpus={n2})", file=sys.stderr)
    sys.exit(1)
open("README.md", "w").write(updated)
print(f"README.md updated: {len(cases)} cases, {mb(total_in)} -> {mb(total_out)}, 0 gate failures")
PY
