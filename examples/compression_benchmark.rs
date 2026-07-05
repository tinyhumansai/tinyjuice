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
use std::path::{Path, PathBuf};
use std::time::Instant;
use tinyjuice::cache;
use tinyjuice::tokens::estimate_tokens;
use tinyjuice::types::{CompressInput, CompressOptions, ContentHint, ContentKind};
use tinyjuice::{compressor_for, detect_content_kind, generic_compressor};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ReportFormat {
    Markdown,
    Json,
}

#[derive(Debug, Clone)]
struct Args {
    iterations: usize,
    format: ReportFormat,
    dump_samples: Option<PathBuf>,
}

#[derive(Debug, Clone)]
struct SignalCheck {
    label: String,
    needle: String,
    expectation: CheckExpectation,
}

#[derive(Debug, Clone, Copy)]
enum CheckExpectation {
    Present,
    Absent,
}

#[derive(Debug, Clone)]
struct BenchCase {
    id: String,
    doc_dir: String,
    family: String,
    description: String,
    payload: String,
    hint: ContentHint,
    command: Option<String>,
    argv: Option<Vec<String>>,
    exit_code: Option<i32>,
    checks: Vec<SignalCheck>,
    task_checks: Vec<TaskCheck>,
}

fn sample_payload(doc_dir: &str, fallback: String) -> String {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("docs/benchmark")
        .join(doc_dir)
        .join(input_artifact_name(doc_dir));
    std::fs::read_to_string(&path)
        .or_else(|_| {
            std::fs::read_to_string(
                Path::new(env!("CARGO_MANIFEST_DIR"))
                    .join("docs/benchmark")
                    .join(doc_dir)
                    .join("full-input.txt"),
            )
        })
        .unwrap_or(fallback)
}

fn category_name(doc_dir: &str) -> &str {
    doc_dir.split('/').next().unwrap_or(doc_dir)
}

fn input_artifact_name(doc_dir: &str) -> &'static str {
    match category_name(doc_dir) {
        "json-smartcrusher" => "input.json",
        "test-failure-log" => "input.log",
        "service-log" => "input.log",
        "search-results" => "input.rg",
        "unified-diff" => "input.diff",
        "html-status-report" if doc_dir.contains("-rss-") => "input.xml",
        "html-status-report" => "input.html",
        "rust-source" => "input.rs",
        "plain-text" => "input.md",
        _ => "input.txt",
    }
}

fn output_artifact_name(doc_dir: &str) -> &'static str {
    match category_name(doc_dir) {
        "json-smartcrusher" => "output.md",
        "test-failure-log" => "output.log",
        "service-log" => "output.log",
        "search-results" => "output.rg",
        "unified-diff" => "output.diff",
        "rust-source" => "output.rs",
        "plain-text" => "output.md",
        _ => "output.txt",
    }
}

#[derive(Debug, Clone)]
struct TaskCheck {
    label: String,
    question: String,
    answer_needles: Vec<String>,
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
    doc_dir: String,
    family: String,
    description: String,
    applied: bool,
    content_kind: String,
    compressor: String,
    lossy: bool,
    ccr_marker: bool,
    ccr_recoverable: Option<bool>,
    original_bytes: usize,
    algorithm_bytes: usize,
    algorithm_byte_ratio: f64,
    algorithm_byte_reduction_percent: f64,
    algorithm_tokens_est: u64,
    algorithm_token_ratio_est: f64,
    algorithm_token_reduction_percent: f64,
    algorithm_applied: bool,
    algorithm_lossy: bool,
    no_ccr_bytes: usize,
    no_ccr_tokens_est: u64,
    no_ccr_token_reduction_percent: f64,
    no_ccr_applied: bool,
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

#[derive(Debug, Clone)]
struct AlgorithmRun {
    text: String,
    applied: bool,
    content_kind: ContentKind,
    compressor: tinyjuice::CompressorKind,
    lossy: bool,
}

fn main() {
    let args = parse_args();
    let options = CompressOptions::default();

    let rt = tokio::runtime::Builder::new_current_thread()
        .build()
        .expect("tokio runtime");

    let cases = benchmark_cases();
    if let Some(root) = args.dump_samples.as_deref() {
        rt.block_on(dump_samples(root, &cases, &options));
        return;
    }

    let mut reports = Vec::new();
    for case in cases {
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
    let algorithm = run_algorithm_once(&case, options).await;
    let mut no_ccr_options = options.clone();
    no_ccr_options.ccr_enabled = false;
    let no_ccr_result = run_once(&case, &no_ccr_options).await;

    let original_tokens_est = estimate_tokens(&case.payload);
    let algorithm_tokens_est = estimate_tokens(&algorithm.text);
    let no_ccr_tokens_est = estimate_tokens(&no_ccr_result.text);
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
    let algorithm_bytes = algorithm.text.len();
    let algorithm_byte_ratio = ratio(algorithm_bytes, result.original_bytes);
    let algorithm_token_ratio = ratio(algorithm_tokens_est as usize, original_tokens_est as usize);
    let no_ccr_token_ratio = ratio(no_ccr_tokens_est as usize, original_tokens_est as usize);
    let token_ratio = ratio(compacted_tokens_est as usize, original_tokens_est as usize);

    CaseReport {
        id: case.id.to_string(),
        doc_dir: case.doc_dir.to_string(),
        family: case.family.to_string(),
        description: case.description.to_string(),
        applied: result.applied,
        content_kind: algorithm.content_kind.as_str().to_string(),
        compressor: algorithm.compressor.as_str().to_string(),
        lossy: result.lossy,
        ccr_marker: result.ccr_token.is_some(),
        ccr_recoverable,
        original_bytes: result.original_bytes,
        algorithm_bytes,
        algorithm_byte_ratio,
        algorithm_byte_reduction_percent: reduction_percent(algorithm_byte_ratio),
        algorithm_tokens_est,
        algorithm_token_ratio_est: algorithm_token_ratio,
        algorithm_token_reduction_percent: reduction_percent(algorithm_token_ratio),
        algorithm_applied: algorithm.applied,
        algorithm_lossy: algorithm.lossy,
        no_ccr_bytes: no_ccr_result.compacted_bytes,
        no_ccr_tokens_est,
        no_ccr_token_reduction_percent: reduction_percent(no_ccr_token_ratio),
        no_ccr_applied: no_ccr_result.applied,
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

async fn run_algorithm_once(case: &BenchCase, options: &CompressOptions) -> AlgorithmRun {
    let mut input = CompressInput {
        content: &case.payload,
        kind: ContentKind::PlainText,
        hint: &case.hint,
        exit_code: case.exit_code,
        command: case.command.clone(),
        argv: case.argv.clone(),
        original_bytes: case.payload.len(),
    };
    let kind = detect_content_kind(&case.payload, &case.hint);
    input.kind = kind;

    if !options.router_enabled || case.payload.len() < options.min_bytes_to_compress {
        return AlgorithmRun::passthrough(case.payload.clone(), kind);
    }

    let primary = match kind {
        ContentKind::Search if !options.search_enabled => None,
        ContentKind::Code if !options.code_enabled => None,
        ContentKind::Html if !options.html_enabled => None,
        _ => Some(compressor_for(kind)),
    };

    let mut produced = match primary {
        Some(compressor) => compressor.compress(&input, options).await,
        None => None,
    };
    if produced.is_none() {
        produced = generic_compressor().compress(&input, options).await;
    }

    let Some(out) = produced else {
        return AlgorithmRun::passthrough(case.payload.clone(), kind);
    };
    if out.text.len() >= case.payload.len() {
        return AlgorithmRun::passthrough(case.payload.clone(), kind);
    }

    AlgorithmRun {
        text: out.text,
        applied: true,
        content_kind: kind,
        compressor: out.kind,
        lossy: out.lossy,
    }
}

impl AlgorithmRun {
    fn passthrough(text: String, content_kind: ContentKind) -> Self {
        Self {
            text,
            applied: false,
            content_kind,
            compressor: tinyjuice::CompressorKind::None,
            lossy: false,
        }
    }
}

async fn dump_samples(root: &Path, cases: &[BenchCase], options: &CompressOptions) {
    for case in cases {
        let result = run_once(case, options).await;
        let dir = root.join(&case.doc_dir);
        std::fs::create_dir_all(&dir)
            .unwrap_or_else(|e| panic!("cannot create {}: {e}", dir.display()));
        let input_path = dir.join(input_artifact_name(&case.doc_dir));
        let output_path = dir.join(output_artifact_name(&case.doc_dir));
        std::fs::write(&input_path, &case.payload)
            .unwrap_or_else(|e| panic!("cannot write {}: {e}", input_path.display()));
        std::fs::write(&output_path, &result.text)
            .unwrap_or_else(|e| panic!("cannot write {}: {e}", output_path.display()));
        let _ = std::fs::remove_file(dir.join("full-input.txt"));
        let _ = std::fs::remove_file(dir.join("full-output.txt"));
        eprintln!(
            "wrote {} and {}",
            input_path.display(),
            output_path.display()
        );
    }
}

impl SignalCheck {
    fn present(label: impl Into<String>, needle: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            needle: needle.into(),
            expectation: CheckExpectation::Present,
        }
    }

    fn absent(label: impl Into<String>, needle: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            needle: needle.into(),
            expectation: CheckExpectation::Absent,
        }
    }

    fn evaluate(&self, output: &str) -> bool {
        match self.expectation {
            CheckExpectation::Present => output.contains(&self.needle),
            CheckExpectation::Absent => !output.contains(&self.needle),
        }
    }
}

impl TaskCheck {
    fn answer(
        label: impl Into<String>,
        question: impl Into<String>,
        answer_needles: Vec<String>,
    ) -> Self {
        Self {
            label: label.into(),
            question: question.into(),
            answer_needles,
        }
    }

    fn evaluate(&self, output: &str) -> bool {
        self.answer_needles
            .iter()
            .all(|needle| output.contains(needle))
    }
}

fn category_case_dirs(category: &str) -> Vec<String> {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("docs/benchmark")
        .join(category)
        .join("cases");

    let mut dirs: Vec<String> = std::fs::read_dir(&root)
        .ok()
        .into_iter()
        .flat_map(|entries| entries.filter_map(Result::ok))
        .filter_map(|entry| {
            let path = entry.path();
            let doc_dir = entry
                .file_name()
                .to_str()
                .map(|name| format!("{category}/cases/{name}"))?;
            if path.join(input_artifact_name(&doc_dir)).is_file()
                || path.join("full-input.txt").is_file()
            {
                Some(doc_dir)
            } else {
                None
            }
        })
        .collect();
    dirs.sort();

    if dirs.is_empty() {
        vec![category.to_string()]
    } else {
        dirs
    }
}

fn case_slug(doc_dir: &str) -> String {
    doc_dir
        .rsplit('/')
        .next()
        .unwrap_or(doc_dir)
        .trim_start_matches(|c: char| c.is_ascii_digit() || c == '-')
        .replace('-', "_")
}

struct CaseSeed {
    category: &'static str,
    family: &'static str,
    description: &'static str,
    fallback: String,
    hint: ContentHint,
    checks: Vec<SignalCheck>,
    task_checks: Vec<TaskCheck>,
}

fn make_case(doc_dir: String, seed: CaseSeed) -> BenchCase {
    BenchCase {
        id: format!(
            "{}_{}",
            seed.category.replace('-', "_"),
            case_slug(&doc_dir)
        ),
        doc_dir: doc_dir.clone(),
        family: seed.family.to_string(),
        description: seed.description.to_string(),
        payload: sample_payload(&doc_dir, seed.fallback),
        hint: seed.hint,
        command: None,
        argv: None,
        exit_code: None,
        checks: seed.checks,
        task_checks: seed.task_checks,
    }
}

fn benchmark_cases() -> Vec<BenchCase> {
    let mut cases = Vec::new();

    for doc_dir in category_case_dirs("json-smartcrusher") {
        cases.push(make_case(
            doc_dir,
            CaseSeed {
                category: "json-smartcrusher",
                family: "json",
                description: "Real OpenHuman Composio tool-catalog JSON slice.",
                fallback: json_service_inventory(260),
                hint: ContentHint {
                    extension: Some("json".to_string()),
                    source_tool: Some("read_file".to_string()),
                    ..Default::default()
                },
                checks: vec![],
                task_checks: vec![],
            },
        ));
    }

    for doc_dir in category_case_dirs("test-failure-log") {
        let mut case = make_case(
            doc_dir,
            CaseSeed {
                category: "test-failure-log",
                family: "command_log",
                description: "Real OpenHuman Vitest command output.",
                fallback: cargo_test_failure_log(360),
                hint: ContentHint {
                    source_tool: Some("exec".to_string()),
                    explicit: Some(ContentKind::Log),
                    ..Default::default()
                },
                checks: vec![
                    SignalCheck::present("vitest run retained", "RUN"),
                    SignalCheck::present("test summary retained", "Test Files"),
                ],
                task_checks: vec![TaskCheck::answer(
                    "summary answer",
                    "Does the compacted output preserve the test summary?",
                    vec!["Test Files".to_string()],
                )],
            },
        );
        case.command = Some("pnpm vitest".to_string());
        case.argv = Some(vec!["pnpm".to_string(), "vitest".to_string()]);
        case.exit_code = Some(101);
        cases.push(case);
    }

    for doc_dir in category_case_dirs("service-log") {
        cases.push(make_case(
            doc_dir,
            CaseSeed {
                category: "service-log",
                family: "log",
                description: "Real OpenHuman runtime or Docker-style service log snapshot.",
                fallback: docker_error_log(5_000),
                hint: ContentHint {
                    explicit: Some(ContentKind::Log),
                    source_tool: Some("docker_logs".to_string()),
                    ..Default::default()
                },
                checks: vec![],
                task_checks: vec![],
            },
        ));
    }

    for doc_dir in category_case_dirs("search-results") {
        let mut case = make_case(
            doc_dir,
            CaseSeed {
                category: "search-results",
                family: "search",
                description: "Real ripgrep output from an OpenHuman checkout.",
                fallback: ripgrep_results(150),
                hint: ContentHint {
                    source_tool: Some("rg".to_string()),
                    query: Some("tokenjuice recover compression".to_string()),
                    explicit: Some(ContentKind::Search),
                    ..Default::default()
                },
                checks: vec![SignalCheck::present("search summary retained", "match(es)")],
                task_checks: vec![TaskCheck::answer(
                    "search summary answer",
                    "Does the compacted output retain the search result shape?",
                    vec!["match(es)".to_string()],
                )],
            },
        );
        case.command = Some("rg tokenjuice recover compression".to_string());
        case.argv = Some(vec![
            "rg".to_string(),
            "tokenjuice".to_string(),
            "recover".to_string(),
            "compression".to_string(),
        ]);
        case.exit_code = Some(0);
        cases.push(case);
    }

    for doc_dir in category_case_dirs("unified-diff") {
        cases.push(make_case(
            doc_dir,
            CaseSeed {
                category: "unified-diff",
                family: "diff",
                description: "Real TinyJuice or OpenHuman unified diff snapshot.",
                fallback: unified_diff(80),
                hint: ContentHint {
                    extension: Some("diff".to_string()),
                    explicit: Some(ContentKind::Diff),
                    ..Default::default()
                },
                checks: vec![
                    SignalCheck::present("diff header retained", "diff --git"),
                    SignalCheck::present("hunk retained", "@@"),
                ],
                task_checks: vec![TaskCheck::answer(
                    "diff shape answer",
                    "Does the compacted output remain recognizable as a diff?",
                    vec!["diff --git".to_string(), "@@".to_string()],
                )],
            },
        ));
    }

    for doc_dir in category_case_dirs("html-status-report") {
        cases.push(make_case(
            doc_dir,
            CaseSeed {
                category: "html-status-report",
                family: "html",
                description: "Real HTML, RSS, coverage, or forum-style page snapshot.",
                fallback: html_status_report(160),
                hint: ContentHint {
                    mime: Some("text/html".to_string()),
                    source_tool: Some("browser".to_string()),
                    explicit: Some(ContentKind::Html),
                    ..Default::default()
                },
                checks: vec![SignalCheck::absent("script removed", "<script>")],
                task_checks: vec![],
            },
        ));
    }

    for doc_dir in category_case_dirs("rust-source") {
        cases.push(make_case(
            doc_dir,
            CaseSeed {
                category: "rust-source",
                family: "code",
                description: "Real OpenHuman Rust source file.",
                fallback: rust_source_file(70),
                hint: ContentHint {
                    extension: Some("rs".to_string()),
                    source_tool: Some("read_file".to_string()),
                    explicit: Some(ContentKind::Code),
                    ..Default::default()
                },
                checks: vec![SignalCheck::present("rust item retained", "fn")],
                task_checks: vec![TaskCheck::answer(
                    "source shape answer",
                    "Does the compacted output retain Rust function structure?",
                    vec!["fn".to_string()],
                )],
            },
        ));
    }

    for doc_dir in category_case_dirs("plain-text") {
        cases.push(make_case(
            doc_dir,
            CaseSeed {
                category: "plain-text",
                family: "plain_text",
                description: "Real OpenHuman Markdown or prose while ML text compression is off.",
                fallback: plain_text_notes(180),
                hint: ContentHint {
                    explicit: Some(ContentKind::PlainText),
                    source_tool: Some("notes".to_string()),
                    ..Default::default()
                },
                checks: vec![],
                task_checks: vec![],
            },
        ));
    }

    cases
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
        dump_samples: None,
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
            "--dump-samples" => {
                let value = iter
                    .next()
                    .unwrap_or_else(|| panic!("{arg} requires a directory"));
                args.dump_samples = Some(PathBuf::from(value));
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
Options:\n  -n, --iterations <N>       Timed iterations per fixture (default: 10)\n      --format <markdown|json>  Report format (default: markdown)\n      --dump-samples <DIR>      Write full input/output files under DIR\n  -h, --help                 Show this help"
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
        "| case | kind | compressor | Pass 1: no CCR | Pass 2: with CCR | avg ms | signals | tasks | inline accuracy | CCR recoverable | gate |"
    );
    println!("| --- | --- | --- | ---: | ---: | ---: | ---: | ---: | ---: | --- | --- |");
    for case in &report.cases {
        println!(
            "| {} | {} | {} | {:.1}% | {:.1}% | {:.3} | {}/{} | {}/{} | {:.1}% | {} | {} |",
            case.id,
            case.content_kind,
            case.compressor,
            case.no_ccr_token_reduction_percent,
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
