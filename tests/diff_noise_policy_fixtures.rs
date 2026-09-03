//! Fixture coverage for host-tuned DiffNoise options.

use serde::Deserialize;
use std::fmt::Write as _;
use std::path::PathBuf;
use tinyjuice::cache::{CcrStore, MemoryCcrStore};
use tinyjuice::compressors::diff::{DiffNoiseOptions, DiffNoiseTransform};
use tinyjuice::{ContentKind, OffloadTransform, PipelineInput};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct DiffNoisePolicyFixture {
    description: String,
    generator: FixtureGenerator,
    options: FixtureOptions,
    expected: FixtureExpectations,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase", tag = "type")]
enum FixtureGenerator {
    ConfiguredPath {
        #[serde(rename = "lines")]
        lines: usize,
    },
    WhitespaceOnly {
        #[serde(rename = "context")]
        context: usize,
    },
    Semantic {
        #[serde(rename = "context")]
        context: usize,
    },
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct FixtureOptions {
    #[serde(default)]
    noisy_path_substrings: Vec<String>,
    #[serde(default)]
    drop_whitespace_only_hunks: bool,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct FixtureExpectations {
    recovery: bool,
    #[serde(default)]
    contains: Vec<String>,
    #[serde(default)]
    not_contains: Vec<String>,
}

#[test]
fn diff_noise_policy_fixtures_match_expectations() {
    for (path, fixture) in load_diff_noise_policy_fixtures() {
        let input = fixture.generator.render();
        let options = DiffNoiseOptions {
            noisy_path_substrings: fixture.options.noisy_path_substrings,
            drop_whitespace_only_hunks: fixture.options.drop_whitespace_only_hunks,
        };
        let transform = DiffNoiseTransform::new(options);
        let pipeline_input = PipelineInput {
            content: &input,
            original_content: &input,
            content_kind: ContentKind::Diff,
            original_bytes: input.len(),
        };
        let store = MemoryCcrStore::default();
        let out = transform.apply(&pipeline_input, &store).unwrap_or_else(|| {
            panic!(
                "{}: fixture did not produce DiffNoise output for {}",
                path.display(),
                fixture.description
            )
        });

        for needle in &fixture.expected.contains {
            assert!(
                out.text().contains(needle),
                "{}: expected output to contain {needle:?} for {}\n---\n{}\n---",
                path.display(),
                fixture.description,
                out.text()
            );
        }
        for needle in &fixture.expected.not_contains {
            assert!(
                !out.text().contains(needle),
                "{}: expected output not to contain {needle:?} for {}\n---\n{}\n---",
                path.display(),
                fixture.description,
                out.text()
            );
        }

        if fixture.expected.recovery {
            assert_eq!(
                store.get(out.token()).as_deref(),
                Some(input.as_str()),
                "{}: recovery token did not retrieve original input for {}",
                path.display(),
                fixture.description
            );
        }
    }
}

fn load_diff_noise_policy_fixtures() -> Vec<(PathBuf, DiffNoisePolicyFixture)> {
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/diff_noise_policy_fixtures");
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
        "no DiffNoise policy fixtures found in {}",
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

impl FixtureGenerator {
    fn render(&self) -> String {
        match self {
            FixtureGenerator::ConfiguredPath { lines } => {
                let mut out = String::from(
                    "diff --git a/fixtures/snapshot.txt b/fixtures/snapshot.txt\n@@ -1,80 +1,80 @@\n",
                );
                for i in 0..*lines {
                    let _ = writeln!(out, "+ snapshot chunk payload {i}");
                }
                for i in 0..*lines {
                    let _ = writeln!(out, "- previous snapshot payload {i}");
                }
                out
            }
            FixtureGenerator::WhitespaceOnly { context } => {
                let mut out =
                    String::from("diff --git a/src/lib.rs b/src/lib.rs\n@@ -1,60 +1,60 @@\n");
                for i in 0..*context {
                    let _ = writeln!(out, " context before {i}");
                }
                let _ = writeln!(out, "-fn main(){{println!(\"hi\");}}");
                let _ = writeln!(out, "+fn main() {{ println!(\"hi\"); }}");
                for i in 0..*context {
                    let _ = writeln!(out, " context after {i}");
                }
                out
            }
            FixtureGenerator::Semantic { context } => {
                let mut out =
                    String::from("diff --git a/src/lib.rs b/src/lib.rs\n@@ -1,40 +1,40 @@\n");
                for i in 0..*context {
                    let _ = writeln!(out, " context before {i}");
                }
                let _ = writeln!(out, "-fn answer() -> i32 {{ 41 }}");
                let _ = writeln!(out, "+fn answer() -> i32 {{ 42 }}");
                for i in 0..*context {
                    let _ = writeln!(out, " context after {i}");
                }
                out
            }
        }
    }
}
