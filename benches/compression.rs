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

criterion_group!(benches, bench_route, bench_rule_engine);
criterion_main!(benches);
