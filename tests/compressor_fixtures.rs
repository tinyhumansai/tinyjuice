//! Fixture-driven coverage for router-level content compressors.
//!
//! The command-rule fixtures in `tests/fixtures/` cover reducer rules. These
//! fixtures cover non-command content compressors through the store-injected
//! router path that hosts use.

use serde::Deserialize;
use std::fmt::Write as _;
use std::path::PathBuf;
use tinyjuice::cache::{CcrStore, MemoryCcrStore};
use tinyjuice::{
    CompressOptions, CompressorKind, ContentHint, ContentKind, compress_content_with_store_report,
};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct CompressorFixture {
    description: String,
    kind: String,
    #[serde(default)]
    extension: Option<String>,
    #[serde(default)]
    query: Option<String>,
    generator: FixtureGenerator,
    expected: FixtureExpectations,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase", tag = "type")]
enum FixtureGenerator {
    JsonRows {
        #[serde(rename = "rows")]
        rows: usize,
        #[serde(default)]
        #[serde(rename = "specialRow")]
        special_row: Option<usize>,
        #[serde(default)]
        #[serde(rename = "specialNote")]
        special_note: Option<String>,
    },
    JsonSparseRows {
        #[serde(rename = "rows")]
        rows: usize,
        #[serde(rename = "sparseRow")]
        sparse_row: usize,
    },
    JsonErrorRows {
        #[serde(rename = "rows")]
        rows: usize,
        #[serde(rename = "errorRow")]
        error_row: usize,
    },
    JsonNumericOutlierRows {
        #[serde(rename = "rows")]
        rows: usize,
        #[serde(rename = "outlierRow")]
        outlier_row: usize,
    },
    JsonDuplicateClusterRows {
        #[serde(rename = "rows")]
        rows: usize,
        #[serde(rename = "clusterStart")]
        cluster_start: usize,
        #[serde(rename = "clusterEnd")]
        cluster_end: usize,
    },
    JsonConstantRows {
        #[serde(rename = "rows")]
        rows: usize,
    },
    JsonNestedRows {
        #[serde(rename = "rows")]
        rows: usize,
    },
    JsonStringifiedRows {
        #[serde(rename = "rows")]
        rows: usize,
    },
    JsonHeterogeneousRows {
        #[serde(rename = "groups")]
        groups: usize,
    },
    JsonDiscriminatorRows {
        #[serde(rename = "rows")]
        rows: usize,
        #[serde(rename = "specialRow")]
        special_row: usize,
    },
    JsonChangePointRows {
        #[serde(rename = "rows")]
        rows: usize,
        #[serde(rename = "changeRow")]
        change_row: usize,
    },
    JsonInformationDenseRows {
        #[serde(rename = "rows")]
        rows: usize,
        #[serde(rename = "denseRow")]
        dense_row: usize,
    },
    DiffLockfile {
        #[serde(rename = "lines")]
        lines: usize,
    },
    SearchResults {
        #[serde(rename = "matchesPerFile")]
        matches_per_file: usize,
    },
    PlainStatus {
        #[serde(rename = "before")]
        before: usize,
        #[serde(rename = "after")]
        after: usize,
        #[serde(rename = "errorLine")]
        error_line: String,
    },
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct FixtureExpectations {
    compressor: String,
    lossy: bool,
    recovery: bool,
    #[serde(default)]
    contains: Vec<String>,
    #[serde(default)]
    not_contains: Vec<String>,
}

#[tokio::test]
async fn content_compressor_fixtures_match_expectations() {
    for (path, fixture) in load_compressor_fixtures() {
        let input = fixture.generator.render();
        let hint = ContentHint {
            extension: fixture.extension.clone(),
            query: fixture.query.clone(),
            explicit: Some(parse_content_kind(&fixture.kind, &path)),
            ..ContentHint::default()
        };
        let store = MemoryCcrStore::default();
        let (out, report) =
            compress_content_with_store_report(&input, Some(hint), &fixture_options(), &store)
                .await;

        let expected_compressor = parse_compressor_kind(&fixture.expected.compressor, &path);
        assert_eq!(
            out.compressor,
            expected_compressor,
            "{}: {} produced report {report:?}",
            path.display(),
            fixture.description
        );
        assert_eq!(
            out.lossy,
            fixture.expected.lossy,
            "{}: unexpected lossiness for {}",
            path.display(),
            fixture.description
        );
        assert_eq!(
            out.ccr_token.is_some(),
            fixture.expected.recovery,
            "{}: unexpected recovery token state for {}",
            path.display(),
            fixture.description
        );

        for needle in &fixture.expected.contains {
            assert!(
                out.text.contains(needle),
                "{}: expected output to contain {needle:?}\n---\n{}\n---",
                path.display(),
                out.text
            );
        }
        for needle in &fixture.expected.not_contains {
            assert!(
                !out.text.contains(needle),
                "{}: expected output not to contain {needle:?}\n---\n{}\n---",
                path.display(),
                out.text
            );
        }

        if fixture.expected.recovery {
            let token = out.ccr_token.as_deref().expect("checked above");
            assert_eq!(
                store.get(token).as_deref(),
                Some(input.as_str()),
                "{}: recovery token did not retrieve original input",
                path.display()
            );
        }
    }
}

fn fixture_options() -> CompressOptions {
    CompressOptions {
        min_bytes_to_compress: 0,
        ccr_min_tokens: 0,
        ..CompressOptions::default()
    }
}

fn load_compressor_fixtures() -> Vec<(PathBuf, CompressorFixture)> {
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/compressor_fixtures");
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
        "no compressor fixtures found in {}",
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

fn parse_content_kind(kind: &str, path: &std::path::Path) -> ContentKind {
    match kind {
        "json" => ContentKind::Json,
        "diff" => ContentKind::Diff,
        "search" => ContentKind::Search,
        "plainText" => ContentKind::PlainText,
        other => panic!("{}: unknown content kind {other}", path.display()),
    }
}

fn parse_compressor_kind(kind: &str, path: &std::path::Path) -> CompressorKind {
    match kind {
        "smartcrusher" => CompressorKind::SmartCrusher,
        "diff" => CompressorKind::Diff,
        "search" => CompressorKind::Search,
        "textcrusher" => CompressorKind::TextCrusher,
        other => panic!("{}: unknown compressor kind {other}", path.display()),
    }
}

impl FixtureGenerator {
    fn render(&self) -> String {
        match self {
            FixtureGenerator::JsonRows {
                rows,
                special_row,
                special_note,
            } => {
                let rendered_rows: Vec<_> = (0..*rows)
                    .map(|i| {
                        let note = if Some(i) == *special_row {
                            special_note.as_deref().unwrap_or("special row")
                        } else {
                            "ordinary row"
                        };
                        format!(
                            r#"{{"id":{i},"name":"record {i}","status":"active","note":"{note}"}}"#
                        )
                    })
                    .collect();
                format!("[{}]", rendered_rows.join(","))
            }
            FixtureGenerator::JsonSparseRows { rows, sparse_row } => {
                let rendered_rows: Vec<_> = (0..*rows)
                    .map(|i| {
                        let extra = if i == *sparse_row {
                            r#","diagnostic":"rare sparse field""#
                        } else {
                            ""
                        };
                        format!(r#"{{"id":{i},"name":"record {i}","status":"active"{extra}}}"#)
                    })
                    .collect();
                format!("[{}]", rendered_rows.join(","))
            }
            FixtureGenerator::JsonErrorRows { rows, error_row } => {
                let rendered_rows: Vec<_> = (0..*rows)
                    .map(|i| {
                        let status = if i == *error_row {
                            "error: timeout"
                        } else {
                            "ok"
                        };
                        format!(
                            r#"{{"id":{i},"name":"job {i}","status":"{status}","note":"detail {i}"}}"#
                        )
                    })
                    .collect();
                format!("[{}]", rendered_rows.join(","))
            }
            FixtureGenerator::JsonNumericOutlierRows { rows, outlier_row } => {
                let rendered_rows: Vec<_> = (0..*rows)
                    .map(|i| {
                        let latency = if i == *outlier_row {
                            9999
                        } else {
                            10 + (i % 3)
                        };
                        format!(
                            r#"{{"id":{i},"endpoint":"/api/{i}","latency_ms":{latency},"region":"us"}}"#
                        )
                    })
                    .collect();
                format!("[{}]", rendered_rows.join(","))
            }
            FixtureGenerator::JsonDuplicateClusterRows {
                rows,
                cluster_start,
                cluster_end,
            } => {
                let rendered_rows: Vec<_> = (0..*rows)
                    .map(|i| {
                        if (*cluster_start..=*cluster_end).contains(&i) {
                            r#"{"id":"retry-batch-17","status":"retry","message":"duplicate cluster payload"}"#
                                .to_owned()
                        } else {
                            format!(
                                r#"{{"id":"job-{i}","status":"ok","message":"ordinary payload {i}"}}"#
                            )
                        }
                    })
                    .collect();
                format!("[{}]", rendered_rows.join(","))
            }
            FixtureGenerator::JsonConstantRows { rows } => {
                let rendered_rows: Vec<_> = (0..*rows)
                    .map(|i| {
                        format!(
                            r#"{{"id":{i},"status":"active","region":"us-east","latency":{}}}"#,
                            10 + i
                        )
                    })
                    .collect();
                format!("[{}]", rendered_rows.join(","))
            }
            FixtureGenerator::JsonNestedRows { rows } => {
                let rendered_rows: Vec<_> = (0..*rows)
                    .map(|i| {
                        format!(
                            r#"{{"id":{i},"user":{{"name":"user {i}","team":"core"}},"status":"active"}}"#
                        )
                    })
                    .collect();
                format!("[{}]", rendered_rows.join(","))
            }
            FixtureGenerator::JsonStringifiedRows { rows } => {
                let rendered_rows: Vec<_> = (0..*rows)
                    .map(|i| {
                        let metadata = serde_json::json!({
                            "owner": format!("team-{i}"),
                            "flags": { "retry": i % 2 == 0 }
                        })
                        .to_string()
                        .replace('"', "\\\"");
                        format!(r#"{{"id":{i},"metadata":"{metadata}","status":"active"}}"#)
                    })
                    .collect();
                format!("[{}]", rendered_rows.join(","))
            }
            FixtureGenerator::JsonHeterogeneousRows { groups } => {
                let mut rendered_rows = Vec::new();
                for i in 0..*groups {
                    rendered_rows.push(format!(
                        r#"{{"event":"login","user_id":"user-{i}","ip":"10.0.0.{i}","success":true}}"#
                    ));
                    rendered_rows.push(format!(
                        r#"{{"event":"deploy","service":"api-{i}","version":"2026.{i}.0","region":"us-east"}}"#
                    ));
                    rendered_rows.push(format!(
                        r#"{{"event":"metric","name":"cpu-{i}","value":{},"unit":"pct"}}"#,
                        40 + i
                    ));
                }
                format!("[{}]", rendered_rows.join(","))
            }
            FixtureGenerator::JsonDiscriminatorRows { rows, special_row } => {
                let rendered_rows: Vec<_> = (0..*rows)
                    .map(|i| {
                        let kind = if i == *special_row {
                            "audit"
                        } else if i % 2 == 0 {
                            "service"
                        } else {
                            "worker"
                        };
                        let message = if i == *special_row {
                            "audit bucket unique payload"
                        } else {
                            "ordinary payload"
                        };
                        format!(
                            r#"{{"id":{i},"kind":"{kind}","status":"active","message":"{message}"}}"#
                        )
                    })
                    .collect();
                format!("[{}]", rendered_rows.join(","))
            }
            FixtureGenerator::JsonChangePointRows { rows, change_row } => {
                let rendered_rows: Vec<_> = (0..*rows)
                    .map(|i| {
                        let phase = if i < *change_row { 10 } else { 30 };
                        let message = if i == *change_row {
                            "phase transition payload".to_string()
                        } else {
                            format!("ordinary payload {i}")
                        };
                        format!(
                            r#"{{"id":{i},"phase_score":{phase},"status":"active","message":"{message}"}}"#
                        )
                    })
                    .collect();
                format!("[{}]", rendered_rows.join(","))
            }
            FixtureGenerator::JsonInformationDenseRows { rows, dense_row } => {
                let rendered_rows: Vec<_> = (0..*rows)
                    .map(|i| {
                        let message = if i == *dense_row {
                            "trace alpha beta gamma delta epsilon zeta eta theta iota kappa lambda"
                                .to_string()
                        } else {
                            format!("ordinary payload {i}")
                        };
                        format!(r#"{{"id":{i},"status":"active","message":"{message}"}}"#)
                    })
                    .collect();
                format!("[{}]", rendered_rows.join(","))
            }
            FixtureGenerator::DiffLockfile { lines } => {
                let mut out =
                    String::from("diff --git a/Cargo.lock b/Cargo.lock\n@@ -1,80 +1,80 @@\n");
                for i in 0..*lines {
                    let _ = writeln!(out, "+ new dep entry {i}");
                }
                for i in 0..*lines {
                    let _ = writeln!(out, "- old dep entry {i}");
                }
                out
            }
            FixtureGenerator::SearchResults { matches_per_file } => {
                let mut out = format!("{} match(es); scanned 2 file(s)\n", matches_per_file * 2);
                for i in 0..*matches_per_file {
                    let body = if i == 7 {
                        "let needle_value = compute_long_name_7();".to_owned()
                    } else {
                        format!("let value_{i} = compute_long_name_{i}();")
                    };
                    let _ = writeln!(out, "src/a.rs:{i}:{body}");
                }
                for i in 0..*matches_per_file {
                    let _ = writeln!(out, "src/b.rs:{i}:fn helper_function_number_{i}() {{}}");
                }
                out
            }
            FixtureGenerator::PlainStatus {
                before,
                after,
                error_line,
            } => {
                let mut out = String::new();
                for i in 0..*before {
                    let _ = writeln!(
                        out,
                        "ordinary deployment progress line {i} with routine status information.\n"
                    );
                }
                let _ = writeln!(out, "{error_line}\n");
                for i in 0..*after {
                    let _ = writeln!(
                        out,
                        "ordinary deployment progress line {} with routine status information.\n",
                        before + i
                    );
                }
                out
            }
        }
    }
}
