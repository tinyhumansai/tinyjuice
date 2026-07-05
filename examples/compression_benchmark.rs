//! Fixture-driven TinyJuice compression benchmark.
//!
//! Run with:
//!
//! ```sh
//! cargo run --release --example compression_benchmark -- --iterations 20
//! cargo run --release --example compression_benchmark -- --format json
//! ```
//!
//! The report intentionally emits metadata only: sizes, token estimates,
//! latency, compressor labels, CCR recovery, and named accuracy checks.

use serde::Serialize;
use serde_json::json;
use std::fmt::Write as _;
use std::time::Instant;
use tinyjuice::cache;
use tinyjuice::tokens::estimate_tokens;
use tinyjuice::types::{CompressInput, CompressOptions, ContentHint, ContentKind};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ReportFormat {
    Markdown,
    Json,
}

#[derive(Debug, Clone)]
struct Args {
    iterations: usize,
    format: ReportFormat,
}

#[derive(Debug, Clone)]
struct SignalCheck {
    label: &'static str,
    needle: &'static str,
    expectation: CheckExpectation,
}

#[derive(Debug, Clone, Copy)]
enum CheckExpectation {
    Present,
    Absent,
}

#[derive(Debug, Clone)]
struct BenchCase {
    id: &'static str,
    family: &'static str,
    description: &'static str,
    payload: String,
    hint: ContentHint,
    command: Option<String>,
    argv: Option<Vec<String>>,
    exit_code: Option<i32>,
    checks: Vec<SignalCheck>,
    task_checks: Vec<TaskCheck>,
}

#[derive(Debug, Clone)]
struct TaskCheck {
    label: &'static str,
    question: &'static str,
    answer_needles: Vec<&'static str>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct BenchmarkReport {
    iterations: usize,
    options: BenchmarkOptionsReport,
    cases: Vec<CaseReport>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct BenchmarkOptionsReport {
    router_enabled: bool,
    ccr_enabled: bool,
    min_bytes_to_compress: usize,
    ccr_min_tokens: usize,
    max_inline_chars: Option<usize>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct CaseReport {
    id: String,
    family: String,
    description: String,
    applied: bool,
    content_kind: String,
    compressor: String,
    lossy: bool,
    ccr_marker: bool,
    ccr_recoverable: Option<bool>,
    original_bytes: usize,
    compacted_bytes: usize,
    byte_ratio: f64,
    byte_reduction_percent: f64,
    original_tokens_est: u64,
    compacted_tokens_est: u64,
    token_ratio_est: f64,
    token_reduction_percent: f64,
    avg_latency_ms: f64,
    checks_passed: usize,
    checks_total: usize,
    task_checks_passed: usize,
    task_checks_total: usize,
    inline_accuracy_percent: f64,
    accuracy_gate_passed: bool,
    failed_checks: Vec<String>,
    failed_task_checks: Vec<String>,
}

fn main() {
    let args = parse_args();
    let options = CompressOptions::default();

    let rt = tokio::runtime::Builder::new_current_thread()
        .build()
        .expect("tokio runtime");

    let mut reports = Vec::new();
    for case in benchmark_cases() {
        let report = rt.block_on(run_case(case, &options, args.iterations));
        reports.push(report);
    }

    let report = BenchmarkReport {
        iterations: args.iterations,
        options: BenchmarkOptionsReport {
            router_enabled: options.router_enabled,
            ccr_enabled: options.ccr_enabled,
            min_bytes_to_compress: options.min_bytes_to_compress,
            ccr_min_tokens: options.ccr_min_tokens,
            max_inline_chars: options.max_inline_chars,
        },
        cases: reports,
    };

    match args.format {
        ReportFormat::Markdown => print_markdown(&report),
        ReportFormat::Json => {
            println!(
                "{}",
                serde_json::to_string_pretty(&report).expect("serialize report")
            );
        }
    }
}

async fn run_case(case: BenchCase, options: &CompressOptions, iterations: usize) -> CaseReport {
    assert!(iterations > 0, "iterations must be greater than zero");

    let _ = run_once(&case, options).await;

    let mut elapsed_nanos: u128 = 0;
    let mut last = None;
    for _ in 0..iterations {
        let start = Instant::now();
        let result = run_once(&case, options).await;
        elapsed_nanos += start.elapsed().as_nanos();
        last = Some(result);
    }
    let result = last.expect("at least one benchmark iteration");

    let original_tokens_est = estimate_tokens(&case.payload);
    let compacted_tokens_est = estimate_tokens(&result.text);
    let failed_checks: Vec<String> = case
        .checks
        .iter()
        .filter(|check| !check.evaluate(&result.text))
        .map(|check| check.label.to_string())
        .collect();
    let failed_task_checks: Vec<String> = case
        .task_checks
        .iter()
        .filter(|check| !check.evaluate(&result.text))
        .map(|check| format!("{} ({})", check.label, check.question))
        .collect();
    let ccr_recoverable = recovery_status(&case.payload, &result);
    let checks_passed = case.checks.len().saturating_sub(failed_checks.len());
    let task_checks_passed = case
        .task_checks
        .len()
        .saturating_sub(failed_task_checks.len());
    let inline_total = case.checks.len() + case.task_checks.len();
    let inline_passed = checks_passed + task_checks_passed;
    let inline_accuracy_percent = if inline_total == 0 {
        100.0
    } else {
        inline_passed as f64 * 100.0 / inline_total as f64
    };
    let recovery_gate = ccr_recoverable.unwrap_or(true);
    let accuracy_gate_passed =
        failed_checks.is_empty() && failed_task_checks.is_empty() && recovery_gate;
    let byte_ratio = ratio(result.compacted_bytes, result.original_bytes);
    let token_ratio = ratio(compacted_tokens_est as usize, original_tokens_est as usize);

    CaseReport {
        id: case.id.to_string(),
        family: case.family.to_string(),
        description: case.description.to_string(),
        applied: result.applied,
        content_kind: result.content_kind.as_str().to_string(),
        compressor: result.compressor.as_str().to_string(),
        lossy: result.lossy,
        ccr_marker: result.ccr_token.is_some(),
        ccr_recoverable,
        original_bytes: result.original_bytes,
        compacted_bytes: result.compacted_bytes,
        byte_ratio,
        byte_reduction_percent: reduction_percent(byte_ratio),
        original_tokens_est,
        compacted_tokens_est,
        token_ratio_est: token_ratio,
        token_reduction_percent: reduction_percent(token_ratio),
        avg_latency_ms: elapsed_nanos as f64 / iterations as f64 / 1_000_000.0,
        checks_passed,
        checks_total: case.checks.len(),
        task_checks_passed,
        task_checks_total: case.task_checks.len(),
        inline_accuracy_percent,
        accuracy_gate_passed,
        failed_checks,
        failed_task_checks,
    }
}

async fn run_once(case: &BenchCase, options: &CompressOptions) -> tinyjuice::CompressedOutput {
    let input = CompressInput {
        content: &case.payload,
        kind: ContentKind::PlainText,
        hint: &case.hint,
        exit_code: case.exit_code,
        command: case.command.clone(),
        argv: case.argv.clone(),
        original_bytes: case.payload.len(),
    };
    tinyjuice::route(input, options).await
}

impl SignalCheck {
    fn present(label: &'static str, needle: &'static str) -> Self {
        Self {
            label,
            needle,
            expectation: CheckExpectation::Present,
        }
    }

    fn absent(label: &'static str, needle: &'static str) -> Self {
        Self {
            label,
            needle,
            expectation: CheckExpectation::Absent,
        }
    }

    fn evaluate(&self, output: &str) -> bool {
        match self.expectation {
            CheckExpectation::Present => output.contains(self.needle),
            CheckExpectation::Absent => !output.contains(self.needle),
        }
    }
}

impl TaskCheck {
    fn answer(
        label: &'static str,
        question: &'static str,
        answer_needles: Vec<&'static str>,
    ) -> Self {
        Self {
            label,
            question,
            answer_needles,
        }
    }

    fn evaluate(&self, output: &str) -> bool {
        self.answer_needles
            .iter()
            .all(|needle| output.contains(needle))
    }
}

fn benchmark_cases() -> Vec<BenchCase> {
    vec![
        BenchCase {
            id: "json_service_inventory",
            family: "json",
            description: "Large API list with a rare error row and latency outlier.",
            payload: json_service_inventory(260),
            hint: ContentHint {
                extension: Some("json".to_string()),
                source_tool: Some("read_file".to_string()),
                ..Default::default()
            },
            command: None,
            argv: None,
            exit_code: Some(0),
            checks: vec![
                SignalCheck::present("error row retained", "critical"),
                SignalCheck::present("omission marker", "omitted"),
                SignalCheck::present("schema retained", "latency_ms"),
            ],
            task_checks: vec![
                TaskCheck::answer(
                    "critical service answer",
                    "Which service is critical?",
                    vec!["svc-137", "critical"],
                ),
                TaskCheck::answer(
                    "latency outlier answer",
                    "What latency marks the anomaly?",
                    vec!["9250"],
                ),
            ],
        },
        BenchCase {
            id: "cargo_test_failure",
            family: "command_log",
            description: "Noisy cargo test output with one failing test and panic details.",
            payload: cargo_test_failure_log(360),
            hint: ContentHint {
                source_tool: Some("shell".to_string()),
                explicit: Some(ContentKind::Log),
                ..Default::default()
            },
            command: Some("cargo test --all-targets".to_string()),
            argv: Some(vec![
                "cargo".to_string(),
                "test".to_string(),
                "--all-targets".to_string(),
            ]),
            exit_code: Some(101),
            checks: vec![
                SignalCheck::present("failing test retained", "tests::panics_on_empty_payload"),
                SignalCheck::present("panic retained", "panicked at"),
                SignalCheck::present("summary retained", "test result: FAILED"),
            ],
            task_checks: vec![
                TaskCheck::answer(
                    "failed test answer",
                    "Which test failed?",
                    vec!["tests::panics_on_empty_payload"],
                ),
                TaskCheck::answer(
                    "panic reason answer",
                    "What was the panic reason?",
                    vec!["empty payload should be rejected before compression"],
                ),
            ],
        },
        BenchCase {
            id: "docker_error_log",
            family: "log",
            description: "High-volume service log with sparse errors and warnings.",
            payload: docker_error_log(5_000),
            hint: ContentHint {
                explicit: Some(ContentKind::Log),
                source_tool: Some("docker_logs".to_string()),
                ..Default::default()
            },
            command: None,
            argv: None,
            exit_code: None,
            checks: vec![
                SignalCheck::present("error retained", "ERROR worker-7"),
                SignalCheck::present("warning retained", "warning: queue-depth"),
                SignalCheck::present("noise dropped", "omitted"),
            ],
            task_checks: vec![
                TaskCheck::answer(
                    "timeout worker answer",
                    "Which worker hit an upstream timeout?",
                    vec!["worker-7", "upstream timeout"],
                ),
                TaskCheck::answer(
                    "queue warning answer",
                    "What warning class appeared?",
                    vec!["queue-depth high"],
                ),
            ],
        },
        BenchCase {
            id: "ripgrep_many_matches",
            family: "search",
            description: "Ripgrep-style output with many matches across files.",
            payload: ripgrep_results(150),
            hint: ContentHint {
                source_tool: Some("rg".to_string()),
                query: Some("tokenjuice recover compression".to_string()),
                explicit: Some(ContentKind::Search),
                ..Default::default()
            },
            command: Some("rg tokenjuice recover compression".to_string()),
            argv: Some(vec![
                "rg".to_string(),
                "tokenjuice".to_string(),
                "recover".to_string(),
                "compression".to_string(),
            ]),
            exit_code: Some(0),
            checks: vec![
                SignalCheck::present("ranked hit retained", "recover exact original"),
                SignalCheck::present("per-file tally", "more match(es)"),
            ],
            task_checks: vec![TaskCheck::answer(
                "best search hit answer",
                "Where is the recovery hit?",
                vec!["src/cache.rs:83", "recover exact original"],
            )],
        },
        BenchCase {
            id: "unified_diff_router_change",
            family: "diff",
            description: "Patch with long unchanged context around a small router change.",
            payload: unified_diff(80),
            hint: ContentHint {
                extension: Some("diff".to_string()),
                explicit: Some(ContentKind::Diff),
                ..Default::default()
            },
            command: None,
            argv: None,
            exit_code: None,
            checks: vec![
                SignalCheck::present("added config retained", "+    ccr_enabled: true,"),
                SignalCheck::present("removed config retained", "-    ccr_enabled: false,"),
                SignalCheck::present("context collapsed", "context line(s) omitted"),
            ],
            task_checks: vec![TaskCheck::answer(
                "config change answer",
                "What changed in the router config?",
                vec!["-    ccr_enabled: false,", "+    ccr_enabled: true,"],
            )],
        },
        BenchCase {
            id: "html_status_report",
            family: "html",
            description: "Rendered admin status page with repeated table markup.",
            payload: html_status_report(160),
            hint: ContentHint {
                mime: Some("text/html".to_string()),
                source_tool: Some("browser".to_string()),
                explicit: Some(ContentKind::Html),
                ..Default::default()
            },
            command: None,
            argv: None,
            exit_code: None,
            checks: vec![
                SignalCheck::present("visible error retained", "critical backlog"),
                SignalCheck::absent("script removed", "<script>"),
            ],
            task_checks: vec![TaskCheck::answer(
                "critical row answer",
                "Which service has the critical backlog?",
                vec!["service-91", "critical backlog on ingestion queue"],
            )],
        },
        BenchCase {
            id: "rust_source_large_file",
            family: "code",
            description: "Rust source with long function bodies and important markers.",
            payload: rust_source_file(70),
            hint: ContentHint {
                extension: Some("rs".to_string()),
                source_tool: Some("read_file".to_string()),
                explicit: Some(ContentKind::Code),
                ..Default::default()
            },
            command: None,
            argv: None,
            exit_code: None,
            checks: vec![
                SignalCheck::present("public signature retained", "pub fn compress_payload"),
                SignalCheck::present("todo retained", "TODO"),
                SignalCheck::present("body collapsed", "line(s)"),
            ],
            task_checks: vec![
                TaskCheck::answer(
                    "signature answer",
                    "What public function is available?",
                    vec!["pub fn compress_payload", "CompressionJob"],
                ),
                TaskCheck::answer(
                    "todo answer",
                    "What TODO marker survived?",
                    vec!["TODO", "preserve anomaly rows"],
                ),
            ],
        },
        BenchCase {
            id: "plain_text_decline",
            family: "plain_text",
            description: "Natural-language notes should pass through while ML text compression is off.",
            payload: plain_text_notes(180),
            hint: ContentHint {
                explicit: Some(ContentKind::PlainText),
                source_tool: Some("notes".to_string()),
                ..Default::default()
            },
            command: None,
            argv: None,
            exit_code: None,
            checks: vec![SignalCheck::present(
                "original text retained",
                "Decision record 42",
            )],
            task_checks: vec![TaskCheck::answer(
                "pass-through answer",
                "Did decision record 42 survive?",
                vec!["Decision record 42"],
            )],
        },
    ]
}

fn json_service_inventory(rows: usize) -> String {
    let rows: Vec<serde_json::Value> = (0..rows)
        .map(|i| {
            let status = if i == 137 { "critical" } else { "ok" };
            let latency_ms = if i == 137 { 9_250 } else { 20 + (i % 45) };
            let region = ["us-east-1", "eu-west-1", "ap-south-1"][i % 3];
            json!({
                "id": i,
                "service": format!("svc-{i:03}"),
                "owner": format!("team-{}", i % 9),
                "region": region,
                "status": status,
                "latency_ms": latency_ms,
                "replicas": 2 + (i % 4),
                "updated_at": format!("2026-07-{:02}T12:{:02}:00Z", 1 + (i % 5), i % 60),
            })
        })
        .collect();
    serde_json::to_string_pretty(&rows).expect("json fixture")
}

fn cargo_test_failure_log(lines: usize) -> String {
    let mut out = String::new();
    for i in 0..lines {
        let _ = writeln!(
            out,
            "test integration::case_{i:03} ... ok stdout: generated {} rows",
            50 + (i % 30)
        );
    }
    out.push_str(
        "\n---- tests::panics_on_empty_payload stdout ----\n\
thread 'tests::panics_on_empty_payload' panicked at src/compress.rs:91:9:\n\
empty payload should be rejected before compression\n\
stack backtrace:\n\
   0: tinyjuice::compress::route\n\
   1: tinyjuice::tool_integration::compact_tool_output_with_policy\n\
\nfailures:\n\
    tests::panics_on_empty_payload\n\n\
test result: FAILED. 359 passed; 1 failed; 0 ignored; finished in 3.42s\n",
    );
    out
}

fn docker_error_log(lines: usize) -> String {
    let mut out = String::new();
    for i in 0..lines {
        if i == 977 || i == 4_311 {
            let _ = writeln!(
                out,
                "2026-07-05T09:{:02}:12Z ERROR worker-7 request failed: upstream timeout request_id=req-{i}",
                i % 60
            );
        } else if i % 1_003 == 0 {
            let _ = writeln!(
                out,
                "2026-07-05T09:{:02}:45Z warning: queue-depth high partition={} depth={}",
                i % 60,
                i % 8,
                900 + i
            );
        } else {
            let _ = writeln!(
                out,
                "2026-07-05T09:{:02}:00Z INFO worker-{} handled request in {}ms",
                i % 60,
                i % 12,
                20 + (i % 45)
            );
        }
    }
    out
}

fn ripgrep_results(lines: usize) -> String {
    let mut out = String::from("150 matches across 6 files\n");
    for i in 0..lines {
        let file = match i % 6 {
            0 => "src/compress.rs",
            1 => "src/cache.rs",
            2 => "src/tool_integration.rs",
            3 => "wiki/CCR-Recovery.md",
            4 => "README.md",
            _ => "docs/architecture.md",
        };
        let body = if i == 73 {
            "recover exact original when tokenjuice compression emits a footer".to_string()
        } else {
            format!("ordinary tokenjuice mention number {i} in compression path")
        };
        let _ = writeln!(out, "{file}:{}:{body}", 10 + i);
    }
    out
}

fn unified_diff(context_lines: usize) -> String {
    let mut out = String::from(
        "diff --git a/src/config.rs b/src/config.rs\n\
index 1111111..2222222 100644\n\
--- a/src/config.rs\n\
+++ b/src/config.rs\n\
@@ -12,90 +12,90 @@ pub fn default_options() -> CompressOptions {\n",
    );
    for i in 0..context_lines {
        let _ = writeln!(out, "     unchanged_config_line_{i}: true,");
    }
    out.push_str("-    ccr_enabled: false,\n+    ccr_enabled: true,\n");
    for i in 0..context_lines {
        let _ = writeln!(out, "     trailing_context_line_{i}: None,");
    }
    out
}

fn html_status_report(rows: usize) -> String {
    let mut out = String::from(
        "<!doctype html><html><head><title>Status</title><script>window.secret='nope'</script></head><body><main><table>",
    );
    for i in 0..rows {
        let cell = if i == 91 {
            "critical backlog on ingestion queue"
        } else {
            "healthy"
        };
        let _ = write!(
            out,
            "<tr><td>service-{i}</td><td>{cell}</td><td>{}</td></tr>",
            20 + (i % 12)
        );
    }
    out.push_str("</table></main></body></html>");
    out
}

fn rust_source_file(body_lines: usize) -> String {
    let mut out = String::from(
        "use std::collections::HashMap;\n\n\
// TODO: preserve anomaly rows during future scoring changes.\n\n\
pub struct CompressionJob {\n    pub id: String,\n    pub bytes: usize,\n}\n\n\
pub fn compress_payload(job: CompressionJob) -> Result<String, String> {\n",
    );
    for i in 0..body_lines {
        let _ = writeln!(
            out,
            "    let intermediate_{i} = job.bytes.saturating_add({i});"
        );
    }
    out.push_str("    Ok(format!(\"{}:{}\", job.id, job.bytes))\n}\n");
    out
}

fn plain_text_notes(paragraphs: usize) -> String {
    let mut out = String::new();
    for i in 0..paragraphs {
        let _ = writeln!(
            out,
            "Decision record {i}: keep benchmark inputs deterministic, avoid raw context logging, and compare compressed output against named signal checks before making any claims.\n"
        );
    }
    out
}

fn parse_args() -> Args {
    let mut args = Args {
        iterations: 10,
        format: ReportFormat::Markdown,
    };
    let mut iter = std::env::args().skip(1);
    while let Some(arg) = iter.next() {
        match arg.as_str() {
            "--iterations" | "-n" => {
                let value = iter
                    .next()
                    .unwrap_or_else(|| panic!("{arg} requires a value"));
                args.iterations = value
                    .parse()
                    .unwrap_or_else(|_| panic!("invalid --iterations value: {value}"));
                assert!(
                    args.iterations > 0,
                    "--iterations must be greater than zero"
                );
            }
            "--format" => {
                let value = iter
                    .next()
                    .unwrap_or_else(|| panic!("{arg} requires a value"));
                args.format = match value.as_str() {
                    "markdown" | "md" => ReportFormat::Markdown,
                    "json" => ReportFormat::Json,
                    _ => panic!("invalid --format value: {value}; expected markdown or json"),
                };
            }
            "--help" | "-h" => {
                print_help();
                std::process::exit(0);
            }
            other => panic!("unknown argument: {other}"),
        }
    }
    args
}

fn print_help() {
    println!(
        "TinyJuice compression benchmark\n\n\
Usage:\n  cargo run --release --example compression_benchmark -- [OPTIONS]\n\n\
Options:\n  -n, --iterations <N>       Timed iterations per fixture (default: 10)\n      --format <markdown|json>  Report format (default: markdown)\n  -h, --help                 Show this help"
    );
}

fn print_markdown(report: &BenchmarkReport) {
    println!("# TinyJuice Compression Benchmark\n");
    println!("- iterations: {}", report.iterations);
    println!(
        "- options: router={}, ccr={}, min_bytes={}, ccr_min_tokens={}\n",
        report.options.router_enabled,
        report.options.ccr_enabled,
        report.options.min_bytes_to_compress,
        report.options.ccr_min_tokens
    );
    println!(
        "| case | kind | compressor | byte reduction | est token reduction | avg ms | signals | tasks | inline accuracy | CCR recoverable | gate |"
    );
    println!("| --- | --- | --- | ---: | ---: | ---: | ---: | ---: | ---: | --- | --- |");
    for case in &report.cases {
        println!(
            "| {} | {} | {} | {:.1}% | {:.1}% | {:.3} | {}/{} | {}/{} | {:.1}% | {} | {} |",
            case.id,
            case.content_kind,
            case.compressor,
            case.byte_reduction_percent,
            case.token_reduction_percent,
            case.avg_latency_ms,
            case.checks_passed,
            case.checks_total,
            case.task_checks_passed,
            case.task_checks_total,
            case.inline_accuracy_percent,
            recovery_label(case.ccr_recoverable),
            if case.accuracy_gate_passed {
                "pass"
            } else {
                "fail"
            }
        );
    }

    let failures: Vec<&CaseReport> = report
        .cases
        .iter()
        .filter(|case| !case.accuracy_gate_passed)
        .collect();
    if failures.is_empty() {
        println!("\nAll accuracy gates passed.");
    } else {
        println!("\nAccuracy gate failures:");
        for case in failures {
            let mut reasons = Vec::new();
            reasons.extend(case.failed_checks.iter().map(String::as_str));
            reasons.extend(case.failed_task_checks.iter().map(String::as_str));
            if case.ccr_recoverable == Some(false) {
                reasons.push("CCR recovery byte-compare failed");
            }
            println!("- {}: {}", case.id, reasons.join(", "));
        }
    }
}

fn recovery_status(original: &str, result: &tinyjuice::CompressedOutput) -> Option<bool> {
    if !result.lossy {
        return None;
    }
    let token = result.ccr_token.as_ref()?;
    Some(cache::retrieve(token).as_deref() == Some(original))
}

fn recovery_label(value: Option<bool>) -> &'static str {
    match value {
        Some(true) => "yes",
        Some(false) => "no",
        None => "n/a",
    }
}

fn ratio(numerator: usize, denominator: usize) -> f64 {
    if denominator == 0 {
        1.0
    } else {
        numerator as f64 / denominator as f64
    }
}

fn reduction_percent(ratio: f64) -> f64 {
    (1.0 - ratio) * 100.0
}
