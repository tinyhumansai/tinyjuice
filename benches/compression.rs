//! Criterion benchmarks for the compression hot paths.
//!
//! Run with `cargo bench`. These measure throughput of the router and the
//! rule engine on deterministic synthetic payloads. They are NOT compression-
//! ratio benchmarks — ratio/savings claims require the fixture benchmark
//! suite (see plan/openhuman-algorithm-port-plan.md, savings accounting).

use criterion::{Criterion, criterion_group, criterion_main};
use serde_json::json;
use std::hint::black_box;
use tinyjuice::reduce_execution_with_rules;
use tinyjuice::rules::load_builtin_rules;
use tinyjuice::types::{
    AgentTokenjuiceCompression, CompressOptions, ReduceOptions, ToolExecutionInput,
};

fn json_payload(rows: usize) -> String {
    let rows: Vec<serde_json::Value> = (0..rows)
        .map(|i| {
            let region = ["us-east-1", "eu-west-1", "ap-south-1"][i % 3];
            json!({
                "id": i,
                "name": format!("service-{i}"),
                "status": if i % 97 == 0 { "error" } else { "ok" },
                "region": region,
                "latency_ms": 20 + (i % 40),
            })
        })
        .collect();
    serde_json::to_string_pretty(&rows).unwrap()
}

fn log_payload(lines: usize) -> String {
    (0..lines)
        .map(|i| {
            if i % 211 == 0 {
                format!(
                    "2026-07-04T12:00:{:02}Z ERROR worker-{} request failed: timeout\n",
                    i % 60,
                    i % 8
                )
            } else {
                format!(
                    "2026-07-04T12:00:{:02}Z INFO worker-{} handled request in {}ms\n",
                    i % 60,
                    i % 8,
                    20 + i % 40
                )
            }
        })
        .collect()
}

fn plain_text_payload(paragraphs: usize) -> String {
    (0..paragraphs)
        .map(|i| format!("Paragraph {i}: the quick brown fox jumps over the lazy dog, again and again, without any log signal or structure worth compressing.\n\n"))
        .collect()
}

fn html_payload(sections: usize) -> String {
    let mut out = String::from("<html><head><title>Docs</title></head><body>");
    for i in 0..sections {
        out.push_str(&format!(
            "<h2>Section {i}</h2><p>Details for item {i} with <a href=\"https://example.com/item/{i}?ref=nav\">a link</a> and <b>bold</b> text &amp; entities.</p><ul><li>alpha {i}</li><li>beta</li></ul>"
        ));
    }
    out.push_str("</body></html>");
    out
}

/// Distinct paragraphs, so the extractive text path has real work to do.
fn varied_text_payload(paragraphs: usize) -> String {
    (0..paragraphs)
        .map(|i| {
            format!(
                "Note {i}: component c{} reported metric m{} at value {} while region r{} stayed stable; follow-up owner o{} is tracking ticket T-{}.\n\n",
                i % 37,
                i % 53,
                i * 31 % 997,
                i % 11,
                i % 19,
                1000 + i
            )
        })
        .collect()
}

fn bench_route(c: &mut Criterion) {
    tinyjuice::configure(CompressOptions::default());
    let rt = tokio::runtime::Builder::new_current_thread()
        .build()
        .unwrap();

    let mut group = c.benchmark_group("compact_tool_output_with_policy");
    for (name, tool, args, payload) in [
        (
            "json_300_rows",
            "read_file",
            json!({ "path": "services.json" }),
            json_payload(300),
        ),
        (
            "log_5000_lines",
            "shell",
            json!({ "command": "docker logs app" }),
            log_payload(5000),
        ),
        (
            "plain_text_decline",
            "some_tool",
            json!({}),
            plain_text_payload(200),
        ),
        (
            "html_400_sections",
            "web_fetch",
            json!({ "url": "https://example.com/docs" }),
            html_payload(400),
        ),
        (
            "html_4000_sections",
            "web_fetch",
            json!({ "url": "https://example.com/docs" }),
            html_payload(4000),
        ),
        (
            "varied_text_15000_paragraphs",
            "some_tool",
            json!({}),
            varied_text_payload(15000),
        ),
        (
            "varied_text_1500_paragraphs",
            "some_tool",
            json!({}),
            varied_text_payload(1500),
        ),
    ] {
        group.throughput(criterion::Throughput::Bytes(payload.len() as u64));
        group.bench_function(name, |b| {
            b.iter(|| {
                rt.block_on(tinyjuice::compact_tool_output_with_policy(
                    black_box(tool),
                    Some(black_box(&args)),
                    black_box(&payload),
                    Some(0),
                    AgentTokenjuiceCompression::Full,
                ))
            })
        });
    }
    group.finish();
}

fn bench_rule_engine(c: &mut Criterion) {
    let rules = load_builtin_rules();
    let input = ToolExecutionInput {
        tool_name: "bash".to_string(),
        argv: Some(vec!["git".to_string(), "status".to_string()]),
        stdout: Some(
            "On branch main\n\nChanges not staged for commit:\n\tmodified:   src/foo.rs\n"
                .repeat(50),
        ),
        exit_code: Some(0),
        ..Default::default()
    };

    c.bench_function("reduce_execution_with_rules/git_status", |b| {
        // Clone in setup, not inside the timed closure — otherwise the
        // measurement includes a multi-KB string clone per iteration.
        b.iter_batched(
            || input.clone(),
            |input| {
                reduce_execution_with_rules(
                    black_box(input),
                    black_box(&rules),
                    &ReduceOptions::default(),
                )
            },
            criterion::BatchSize::SmallInput,
        )
    });

    c.bench_function("load_builtin_rules", |b| {
        b.iter(|| black_box(load_builtin_rules()))
    });
}

fn bench_repl(c: &mut Criterion) {
    use tinyjuice::repl::{ExtractKind, FindMode, ReplLimits, ReplOp, run_on_text};
    let find = |query: &str, mode| ReplOp::Find {
        query: query.into(),
        mode,
        ignore_case: false,
        context: 0,
        top_k: None,
        scope: None,
        unit: Default::default(),
    };
    let extract = |what| ReplOp::Extract {
        what,
        scope: None,
        unit: Default::default(),
    };
    let log = log_payload(5000);
    let html = html_payload(400);
    let limits = ReplLimits::default();
    let mut group = c.benchmark_group("repl");
    for (name, text, op) in [
        ("find_text_log_5000", &log, find("ERROR", FindMode::Text)),
        (
            "find_rank_log_5000",
            &log,
            find("timeout worker-3", FindMode::Rank),
        ),
        (
            "find_sed_log_5000",
            &log,
            find("-n /ERROR/p", FindMode::Sed),
        ),
        (
            "find_awk_log_5000",
            &log,
            find("/ERROR/ { print $3 }", FindMode::Awk),
        ),
        (
            "summarize_log_5000",
            &log,
            ReplOp::Summarize {
                max_chars: None,
                hint: None,
                scope: None,
                unit: Default::default(),
            },
        ),
        ("links_html_400", &html, extract(ExtractKind::Links)),
        ("headings_html_400", &html, extract(ExtractKind::Headings)),
    ] {
        group.throughput(criterion::Throughput::Bytes(text.len() as u64));
        group.bench_function(name, |b| {
            b.iter(|| run_on_text(black_box(text), &op, &limits))
        });
    }
    group.finish();
}

criterion_group!(benches, bench_route, bench_rule_engine, bench_repl);
criterion_main!(benches);
