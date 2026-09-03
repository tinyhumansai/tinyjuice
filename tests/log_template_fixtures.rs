use std::fmt::Write as _;
use std::path::PathBuf;

use serde::Deserialize;
use tinyjuice::compressors::log::compress_templates;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct LogTemplateFixture {
    description: String,
    generator: LogTemplateGenerator,
    expected: LogTemplateExpectations,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase", tag = "type")]
enum LogTemplateGenerator {
    WorkerProgress {
        #[serde(rename = "lines")]
        lines: usize,
    },
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct LogTemplateExpectations {
    contains: Vec<String>,
    lossless: bool,
    smaller: bool,
}

#[test]
fn log_template_fixtures_reconstruct_original_lines() {
    for (path, fixture) in load_log_template_fixtures() {
        let input = fixture.generator.render();
        let out = compress_templates(&input).unwrap_or_else(|| {
            panic!(
                "{}: fixture did not produce a log template: {}",
                path.display(),
                fixture.description
            )
        });

        for needle in &fixture.expected.contains {
            assert!(
                out.text.contains(needle),
                "{}: output missing {needle:?}\n---\n{}\n---",
                path.display(),
                out.text
            );
        }
        if fixture.expected.smaller {
            assert!(
                out.text.len() < input.len(),
                "{}: template output did not shrink",
                path.display()
            );
        }
        if fixture.expected.lossless {
            assert_eq!(
                reconstruct_template_reformat(&out.text).trim_end(),
                input.trim_end(),
                "{}: reconstructed log did not match original",
                path.display()
            );
        }
    }
}

fn load_log_template_fixtures() -> Vec<(PathBuf, LogTemplateFixture)> {
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/log_template_fixtures");
    let mut paths: Vec<_> = std::fs::read_dir(&dir)
        .unwrap_or_else(|error| panic!("cannot read {}: {error}", dir.display()))
        .filter_map(|entry| entry.ok().map(|entry| entry.path()))
        .filter(|path| {
            path.file_name()
                .and_then(|name| name.to_str())
                .is_some_and(|name| name.ends_with(".fixture.json"))
        })
        .collect();
    paths.sort();
    assert!(
        !paths.is_empty(),
        "no log template fixtures found in {}",
        dir.display()
    );

    paths
        .into_iter()
        .map(|path| {
            let raw = std::fs::read_to_string(&path)
                .unwrap_or_else(|error| panic!("cannot read {}: {error}", path.display()));
            let fixture = serde_json::from_str(&raw)
                .unwrap_or_else(|error| panic!("invalid fixture {}: {error}", path.display()));
            (path, fixture)
        })
        .collect()
}

impl LogTemplateGenerator {
    fn render(&self) -> String {
        match self {
            LogTemplateGenerator::WorkerProgress { lines } => {
                let mut out = String::new();
                for i in 0..*lines {
                    let _ = writeln!(
                        out,
                        "2026-07-05T12:{:02}:00Z worker-{i} processed item id={} shard={}",
                        i % 60,
                        10_000 + i,
                        i % 8
                    );
                }
                out
            }
        }
    }
}

fn reconstruct_template_reformat(output: &str) -> String {
    let mut out = String::new();
    let lines: Vec<&str> = output.lines().collect();
    let mut i = 0;
    while i < lines.len() {
        if lines[i].starts_with("[TOKENJUICE LOG TEMPLATE ") {
            let template = lines[i + 1];
            i += 2;
            while i < lines.len() && lines[i] != "[/TOKENJUICE LOG TEMPLATE]" {
                let captures = lines[i]
                    .split('\t')
                    .map(unescape_capture)
                    .collect::<Vec<_>>();
                let _ = writeln!(out, "{}", apply_template(template, &captures));
                i += 1;
            }
            i += 1;
        } else {
            let _ = writeln!(out, "{}", lines[i]);
            i += 1;
        }
    }
    out
}

fn unescape_capture(value: &str) -> String {
    let mut out = String::new();
    let mut chars = value.chars();
    while let Some(ch) = chars.next() {
        if ch == '\\' {
            match chars.next() {
                Some('\\') => out.push('\\'),
                Some('t') => out.push('\t'),
                Some('r') => out.push('\r'),
                Some(other) => {
                    out.push('\\');
                    out.push(other);
                }
                None => out.push('\\'),
            }
        } else {
            out.push(ch);
        }
    }
    out
}

fn apply_template(template: &str, captures: &[String]) -> String {
    let mut out = String::new();
    let mut rest = template;
    for capture in captures {
        let Some((head, tail)) = rest.split_once("{}") else {
            break;
        };
        out.push_str(head);
        out.push_str(capture);
        rest = tail;
    }
    out.push_str(rest);
    out
}
