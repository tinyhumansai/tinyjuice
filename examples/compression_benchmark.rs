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
        .unwrap_or_else(|_| {
            // A silent fallback would benchmark generator output while the
            // report still claims a real fixture — make the substitution loud.
            eprintln!(
                "warning: fixture missing for {doc_dir} ({}); using synthetic fallback payload",
                path.display()
            );
            fallback
        })
}

fn category_name(doc_dir: &str) -> &str {
    doc_dir.split('/').next().unwrap_or(doc_dir)
}

/// Language tag for a polyglot-source / github-source case, taken from the
/// segment after the `NN-` ordinal in the case dir name
/// (e.g. `01-ts-api-client` → `ts`).
fn polyglot_lang(doc_dir: &str) -> &str {
    doc_dir
        .rsplit('/')
        .next()
        .unwrap_or(doc_dir)
        .split('-')
        .nth(1)
        .unwrap_or("txt")
}

/// Input artifact extension for a source-file language tag.
fn source_input_name(lang: &str) -> &'static str {
    match lang {
        "ts" => "input.ts",
        "js" => "input.js",
        "py" => "input.py",
        "cpp" => "input.cpp",
        "c" => "input.c",
        "go" => "input.go",
        "rs" => "input.rs",
        "java" => "input.java",
        "kt" => "input.kt",
        "rb" => "input.rb",
        "php" => "input.php",
        "swift" => "input.swift",
        "cs" => "input.cs",
        "xml" => "input.xml",
        _ => "input.txt",
    }
}

/// Output artifact extension for a source-file language tag. XML output is
/// extracted text, not markup, so it dumps as `.txt`.
fn source_output_name(lang: &str) -> &'static str {
    match lang {
        "ts" => "output.ts",
        "js" => "output.js",
        "py" => "output.py",
        "cpp" => "output.cpp",
        "c" => "output.c",
        "go" => "output.go",
        "rs" => "output.rs",
        "java" => "output.java",
        "kt" => "output.kt",
        "rb" => "output.rb",
        "php" => "output.php",
        "swift" => "output.swift",
        "cs" => "output.cs",
        _ => "output.txt",
    }
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
        "polyglot-source" | "github-source" => source_input_name(polyglot_lang(doc_dir)),
        "github-logs" => "input.log",
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
        "polyglot-source" | "github-source" => source_output_name(polyglot_lang(doc_dir)),
        "github-logs" => "output.log",
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
    /// `None` when the case defines no checks — an unverified case must not
    /// read as 100% accurate.
    inline_accuracy_percent: Option<f64>,
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
    let mut failed_checks: Vec<String> = case
        .checks
        .iter()
        .filter(|check| !check.evaluate(&result.text))
        .map(|check| check.label.to_string())
        .collect();
    // Structural invariants shared by every case: valid text with no
    // compressor-introduced junk bytes, and no inflation when compression
    // claims to have been applied.
    let mut checks_total = case.checks.len();
    for (label, ok) in structural_invariants(&case.payload, &result) {
        checks_total += 1;
        if !ok {
            failed_checks.push(format!("structural: {label}"));
        }
    }
    let failed_task_checks: Vec<String> = case
        .task_checks
        .iter()
        .filter(|check| !check.evaluate(&result.text))
        .map(|check| format!("{} ({})", check.label, check.question))
        .collect();
    let ccr_recoverable = recovery_status(&case.payload, &result);
    let checks_passed = checks_total.saturating_sub(failed_checks.len());
    let task_checks_passed = case
        .task_checks
        .len()
        .saturating_sub(failed_task_checks.len());
    let inline_total = checks_total + case.task_checks.len();
    let inline_passed = checks_passed + task_checks_passed;
    let inline_accuracy_percent = if inline_total == 0 {
        None
    } else {
        Some(inline_passed as f64 * 100.0 / inline_total as f64)
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
        checks_total,
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

/// Cheap meaning-independent invariants every routed output must satisfy.
/// Output is a Rust `String`, so UTF-8 validity holds by construction; these
/// guard the remaining mojibake and inflation bug classes:
///
/// - the compressor must not introduce control bytes (beyond `\n`, `\t`,
///   `\r`) or U+FFFD replacement characters that were not already present in
///   the input (byte-slicing multi-byte sequences is how mojibake shipped);
/// - when `applied` is true the output must be strictly smaller than the
///   input, CCR footer included.
fn structural_invariants(
    input: &str,
    result: &tinyjuice::CompressedOutput,
) -> Vec<(&'static str, bool)> {
    let no_new_junk_chars = !result.text.chars().any(|c| {
        let junk = (c.is_control() && !matches!(c, '\n' | '\t' | '\r')) || c == '\u{FFFD}';
        junk && !input.contains(c)
    });
    let no_inflation = !result.applied || result.compacted_bytes < result.original_bytes;
    vec![
        ("no control bytes or U+FFFD introduced", no_new_junk_chars),
        ("applied output smaller than input", no_inflation),
    ]
}

/// Last path segment of a case dir (`json-smartcrusher/cases/01-x` → `01-x`).
fn case_name(doc_dir: &str) -> &str {
    doc_dir.rsplit('/').next().unwrap_or(doc_dir)
}

/// Per-fixture JSON checks: a row identifier from the middle band of each
/// fixture's dominant array (or a deep key for object fixtures) must survive
/// compression. Middle-band rows are exactly what sampling-style bugs drop
/// silently, so plain presence is the gate.
fn json_case_checks(doc_dir: &str) -> Vec<SignalCheck> {
    let (label, needle): (&str, &str) = match case_name(doc_dir) {
        "01-github-tools-array" => (
            "middle-band tool retained",
            "GITHUB_AUTH_USER_DOCKER_CONFLICT_PACKAGES_LIST",
        ),
        "02-notion-tools-array" => ("middle-band tool retained", "NOTION_GET_ABOUT_USER"),
        "03-slack-tools-array" => ("middle-band tool retained", "SLACK_DOWNLOAD_SLACK_FILE"),
        "04-polymarket-markets-list" => ("middle-band market retained", "will-btc-hit-200k"),
        "05-polymarket-events-list" => ("middle-band event retained", "bitcoin-milestones"),
        "06-tauri-capabilities-schema" => {
            ("capability identifier retained", "webview-accounts-recipes")
        }
        "07-app-schema-object" => (
            "middle-band method retained",
            "openhuman.config_update_model_settings",
        ),
        "08-lottie-animation" => ("middle-band layer name retained", "shield"),
        "09-package-manifest" => ("middle-band dependency retained", "os-browserify"),
        "10-cargo-metadata" => ("middle-band target retained", "slack-backfill"),
        _ => return vec![],
    };
    vec![SignalCheck::present(label, needle)]
}

/// Per-fixture HTML/XML presence checks: 1-2 key readable strings from each
/// page (page/feed title plus one article/topic/file identifier) must survive
/// text extraction.
fn html_case_checks(doc_dir: &str) -> Vec<SignalCheck> {
    match case_name(doc_dir) {
        "01-rss-rust-blog" => vec![
            SignalCheck::present("feed title retained", "Rust Blog"),
            SignalCheck::present("article title retained", "Announcing Rust 1.96.1"),
        ],
        "02-rss-hacker-news" => vec![
            SignalCheck::present("feed title retained", "Hacker News: Front Page"),
            SignalCheck::present("item title retained", "The Engineer in the Half-Space"),
        ],
        "03-noisy-hacker-news" => vec![
            SignalCheck::present("page title retained", "Hacker News"),
            SignalCheck::present("story title retained", "Claude Design System Prompt"),
        ],
        "04-forum-rust-users" => vec![
            SignalCheck::present("page title retained", "The Rust Programming Language Forum"),
            SignalCheck::present(
                "topic title retained",
                "Forum Code Formatting and Syntax Highlighting",
            ),
        ],
        "05-openhuman-coverage-5" => vec![
            SignalCheck::present("report title retained", "Code coverage report"),
            SignalCheck::present("covered file retained", "GoogleIcon.tsx"),
        ],
        "06-openhuman-coverage-6" => vec![
            SignalCheck::present("report title retained", "Code coverage report"),
            SignalCheck::present("covered path retained", "src/assets/icons"),
        ],
        "07-openhuman-coverage-7" => vec![
            SignalCheck::present("report title retained", "Code coverage report"),
            SignalCheck::present("covered file retained", "chatSendError.ts"),
        ],
        "08-openhuman-coverage-8" => vec![
            SignalCheck::present("report title retained", "Code coverage report"),
            SignalCheck::present("covered path retained", "src/chat"),
        ],
        "09-openhuman-coverage-9" => vec![
            SignalCheck::present("report title retained", "Code coverage report"),
            SignalCheck::present("covered file retained", "promptInjectionGuard.ts"),
        ],
        "10-openhuman-coverage-10" => vec![
            SignalCheck::present("report title retained", "Code coverage report"),
            SignalCheck::present("covered file retained", "AppBackground.tsx"),
        ],
        _ => vec![],
    }
}

/// Per-fixture service-log checks. These fixtures are macOS crash-report
/// slices of OpenHuman: every one carries `panic`-path frames, and a log
/// compressor that replaces all error/crash content with `[Template: …]`/`×N`
/// summaries must still surface that substring somewhere in the output.
fn service_log_case_checks(doc_dir: &str) -> Vec<SignalCheck> {
    let name = case_name(doc_dir);
    if !name.starts_with(|c: char| c.is_ascii_digit()) {
        // Synthetic fallback case (fixtures missing); needles below are tied
        // to the real crash-report fixtures.
        return vec![];
    }
    let mut checks = vec![SignalCheck::present("panic frame retained", "panic")];
    if name == "01-openhuman-crash-slice-1" {
        checks.push(SignalCheck::present(
            "termination reason retained",
            "Bus error: 10",
        ));
    }
    checks
}

/// Per-fixture github-logs checks: a distinctive constant substring from a
/// real ERROR/failure line in each error-bearing fixture. Template-style
/// summaries keep the constant message text, so these pass for both exemplar
/// retention and `[Template: …] ×N` output — but fail if error content is
/// dropped wholesale. Fixtures without error lines get no per-case check.
fn github_log_case_checks(doc_dir: &str) -> Vec<SignalCheck> {
    let needle: &str = match case_name(doc_dir) {
        "02-hadoop" => "Container complete event for unknown container",
        "04-zookeeper" => "Connection broken for id",
        "05-bgl" => "instruction cache parity error",
        "06-hpc" => "psu failure",
        "07-thunderbird" => "Wait for ready failed before probe",
        "08-windows" => "Failed to start upload with file pattern",
        "09-linux" => "authentication failure; logname=",
        "11-healthapp" => "saveOneDetailData fail",
        "12-apache-error" => "workerEnv in error state",
        "13-proxifier" => "A connection request was canceled",
        "14-openssh" => "POSSIBLE BREAK-IN ATTEMPT",
        "16-mac" => "AuthFail",
        "19-auth-log" => "maximum authentication attempts exceeded",
        "20-caddy-coraza-waf" => "Coraza",
        "23-authelia-bf" => "Unsuccessful 1FA authentication attempt",
        "26-gitlab-bf" => "\"action\":\"failure\"",
        "30-laravel-app" => "local.ERROR",
        _ => return vec![],
    };
    vec![SignalCheck::present("error exemplar retained", needle)]
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
        eprintln!(
            "warning: no fixture cases found under docs/benchmark/{category}/cases; \
             benchmarking a single synthetic case instead"
        );
        vec![category.to_string()]
    } else {
        dirs
    }
}

fn case_slug(doc_dir: &str) -> String {
    // Keep the NN- ordinal: stripping it collapsed distinct fixtures with the
    // same stem (03-socket and 06-socket) into one indistinguishable id.
    doc_dir
        .rsplit('/')
        .next()
        .unwrap_or(doc_dir)
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
        let checks = json_case_checks(&doc_dir);
        cases.push(make_case(
            doc_dir,
            CaseSeed {
                category: "json-smartcrusher",
                family: "json",
                description: "Real JSON payload: tool catalogs, market lists, schemas, manifests.",
                fallback: json_service_inventory(260),
                hint: ContentHint {
                    extension: Some("json".to_string()),
                    source_tool: Some("read_file".to_string()),
                    ..Default::default()
                },
                checks,
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
        let checks = service_log_case_checks(&doc_dir);
        cases.push(make_case(
            doc_dir,
            CaseSeed {
                category: "service-log",
                family: "log",
                description: "Real OpenHuman crash-report/service log snapshot.",
                fallback: docker_error_log(5_000),
                hint: ContentHint {
                    explicit: Some(ContentKind::Log),
                    source_tool: Some("docker_logs".to_string()),
                    ..Default::default()
                },
                checks,
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
                    query: Some("tinyjuice recover compression".to_string()),
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
        case.command = Some("rg tinyjuice recover compression".to_string());
        case.argv = Some(vec![
            "rg".to_string(),
            "tinyjuice".to_string(),
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
        // Markup-leak absence checks apply to every HTML/XML case (they pass
        // trivially where the input never contained the construct); presence
        // checks for key readable strings are per fixture.
        let mut checks = vec![
            SignalCheck::absent("script removed", "<script>"),
            SignalCheck::absent("no CDATA terminator leak", "]]>"),
            SignalCheck::absent("no media-query attribute leak", "= 40rem)\""),
            SignalCheck::absent("no stylesheet attribute leak", "rel=\"stylesheet\""),
        ];
        checks.extend(html_case_checks(&doc_dir));
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
                checks,
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

    for doc_dir in category_case_dirs("polyglot-source") {
        let lang = polyglot_lang(&doc_dir).to_string();
        // XML routes to the HTML/text extractor; everything else is Code.
        let kind = if lang == "xml" {
            ContentKind::Html
        } else {
            ContentKind::Code
        };
        let (check_label, needle) = match lang.as_str() {
            "ts" => ("ts class retained", "export class ApiClient"),
            "py" => ("py def retained", "def transform_records"),
            "cpp" => ("cpp class retained", "class SceneNode"),
            "go" => ("go func retained", "func (s *Server) handleMetrics"),
            "rs" => ("rust impl retained", "impl<'a> Lexer<'a>"),
            "xml" => ("xml artifact text retained", "orchestrator"),
            _ => ("content retained", "fn"),
        };
        cases.push(make_case(
            doc_dir,
            CaseSeed {
                category: "polyglot-source",
                family: if lang == "xml" { "html" } else { "code" },
                description: "Representative source/markup file in a non-Rust language.",
                fallback: rust_source_file(70),
                hint: ContentHint {
                    extension: Some(lang.clone()),
                    source_tool: Some("read_file".to_string()),
                    explicit: Some(kind),
                    ..Default::default()
                },
                checks: vec![SignalCheck::present(check_label, needle)],
                task_checks: vec![],
            },
        ));
    }

    for doc_dir in category_case_dirs("github-source") {
        let lang = polyglot_lang(&doc_dir).to_string();
        // Weak per-language needle that must survive signature-preserving
        // compaction (imports and top-level declarations are always kept).
        let needle = match lang.as_str() {
            "ts" => "export",
            // Some JS fixtures are CJS (require) and some ESM (import).
            "js" => "port",
            "py" => "def ",
            "go" => "func ",
            "cpp" | "c" => "#include",
            "rs" => "fn ",
            "java" | "kt" | "cs" => "class",
            "rb" => "def ",
            "php" => "namespace",
            "swift" => "struct",
            "xml" => "4.0.0",
            _ => "fn",
        };
        cases.push(make_case(
            doc_dir,
            CaseSeed {
                category: "github-source",
                family: if lang == "xml" { "html" } else { "code" },
                description: "Real source file fetched from a public GitHub repository.",
                fallback: rust_source_file(70),
                hint: ContentHint {
                    extension: Some(lang.clone()),
                    source_tool: Some("read_file".to_string()),
                    ..Default::default()
                },
                checks: vec![SignalCheck::present("structure retained", needle)],
                task_checks: vec![],
            },
        ));
    }

    for doc_dir in category_case_dirs("github-logs") {
        let checks = github_log_case_checks(&doc_dir);
        cases.push(make_case(
            doc_dir,
            CaseSeed {
                category: "github-logs",
                family: "log",
                description: "Real log file fetched from a public GitHub repository.",
                fallback: docker_error_log(400),
                hint: ContentHint {
                    extension: Some("log".to_string()),
                    ..Default::default()
                },
                checks,
                task_checks: vec![],
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
            "recover exact original when tinyjuice compression emits a footer".to_string()
        } else {
            format!("ordinary tinyjuice mention number {i} in compression path")
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
        "| case | kind | compressor | algorithm | Pass 1: no CCR | Pass 2: with CCR | avg ms | signals | tasks | inline accuracy | CCR recoverable | gate |"
    );
    println!("| --- | --- | --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | --- | --- |");
    for case in &report.cases {
        let accuracy = case
            .inline_accuracy_percent
            .map(|p| format!("{p:.1}%"))
            .unwrap_or_else(|| "n/a".to_string());
        println!(
            "| {} | {} | {} | {:.1}% | {:.1}% | {:.1}% | {:.3} | {}/{} | {}/{} | {} | {} | {} |",
            case.id,
            case.content_kind,
            case.compressor,
            case.algorithm_token_reduction_percent,
            case.no_ccr_token_reduction_percent,
            case.token_reduction_percent,
            case.avg_latency_ms,
            case.checks_passed,
            case.checks_total,
            case.task_checks_passed,
            case.task_checks_total,
            accuracy,
            recovery_label(case.ccr_recoverable),
            if case.accuracy_gate_passed {
                "pass"
            } else {
                "fail"
            }
        );
    }

    print_summary(report);

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

/// Byte-weighted (micro-average) totals plus coverage lint. Per-case means
/// over reduction percentages over-weight small fixtures; total-in vs
/// total-out reflects what actually reaches the context window.
fn print_summary(report: &BenchmarkReport) {
    let total_in: usize = report.cases.iter().map(|c| c.original_bytes).sum();
    let algo_out: usize = report.cases.iter().map(|c| c.algorithm_bytes).sum();
    let pass2_out: usize = report.cases.iter().map(|c| c.compacted_bytes).sum();
    if total_in > 0 {
        println!("\n## Byte-weighted totals (all cases)\n");
        println!(
            "- algorithm only: {:.1}% reduction ({} -> {} bytes)",
            reduction_percent(ratio(algo_out, total_in)),
            total_in,
            algo_out
        );
        println!(
            "- with CCR footer: {:.1}% reduction ({} -> {} bytes)",
            reduction_percent(ratio(pass2_out, total_in)),
            total_in,
            pass2_out
        );
    }

    let applied = report.cases.iter().filter(|c| c.applied).count();
    println!(
        "- compression applied on {}/{} cases",
        applied,
        report.cases.len()
    );

    let below_threshold: Vec<&CaseReport> = report
        .cases
        .iter()
        .filter(|c| c.original_bytes < report.options.min_bytes_to_compress)
        .collect();
    if !below_threshold.is_empty() {
        println!(
            "\n## Fixture lint: {} case(s) below min_bytes_to_compress ({}B) — always pass through and dilute averages\n",
            below_threshold.len(),
            report.options.min_bytes_to_compress
        );
        for case in below_threshold {
            println!("- {} ({} bytes)", case.id, case.original_bytes);
        }
    }
}

fn recovery_status(original: &str, result: &tinyjuice::CompressedOutput) -> Option<bool> {
    if !result.lossy {
        return None;
    }
    // A lossy result without a recovery token breaks the router's invariant
    // (lossy ⇒ original retained); fail the gate rather than skip it.
    let Some(token) = result.ccr_token.as_ref() else {
        return Some(false);
    };
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
