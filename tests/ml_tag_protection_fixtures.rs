//! Fixture coverage for ML text custom tag protection.

use serde::Deserialize;
use std::path::PathBuf;
use std::sync::Arc;
use tinyjuice::compressors::text::compress_ml_with_tag_protection;
use tinyjuice::ml;
use tinyjuice::{CompressOptions, CompressorKind};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct MlTagProtectionFixture {
    description: String,
    generator: FixtureGenerator,
    callback: CallbackBehavior,
    expected: FixtureExpectations,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase", tag = "type")]
enum FixtureGenerator {
    WorkflowBlock {
        #[serde(rename = "fillerWords")]
        filler_words: usize,
    },
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
enum CallbackBehavior {
    KeepPlaceholder,
    DropPlaceholder,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct FixtureExpectations {
    produces_output: bool,
    #[serde(default)]
    contains: Vec<String>,
    #[serde(default)]
    not_contains: Vec<String>,
}

#[tokio::test]
async fn ml_tag_protection_fixtures_match_expectations() {
    for (path, fixture) in load_ml_tag_protection_fixtures() {
        let input = fixture.generator.render();
        ml::configure_callback(Some(callback_for(&fixture.callback)));
        let out = compress_ml_with_tag_protection(&input, &fixture_options()).await;
        ml::configure_callback(None);

        assert_eq!(
            out.is_some(),
            fixture.expected.produces_output,
            "{}: unexpected ML output state for {}",
            path.display(),
            fixture.description
        );

        let Some(out) = out else {
            continue;
        };
        assert_eq!(out.kind, CompressorKind::MlText);
        for needle in &fixture.expected.contains {
            assert!(
                out.text.contains(needle),
                "{}: expected output to contain {needle:?} for {}\n---\n{}\n---",
                path.display(),
                fixture.description,
                out.text
            );
        }
        for needle in &fixture.expected.not_contains {
            assert!(
                !out.text.contains(needle),
                "{}: expected output not to contain {needle:?} for {}\n---\n{}\n---",
                path.display(),
                fixture.description,
                out.text
            );
        }
    }
}

fn fixture_options() -> CompressOptions {
    CompressOptions {
        ml_text_enabled: true,
        min_bytes_to_compress: 0,
        ..CompressOptions::default()
    }
}

fn callback_for(behavior: &CallbackBehavior) -> Arc<ml::MlCompressCallback> {
    match behavior {
        CallbackBehavior::KeepPlaceholder => Arc::new(|text, _opts| {
            Box::pin(async move {
                let placeholder = text
                    .split_whitespace()
                    .find(|part| part.contains("__TOKENJUICE_TAG_"))
                    .unwrap_or("")
                    .to_string();
                Ok(Some(format!("compressed {placeholder}")))
            })
        }),
        CallbackBehavior::DropPlaceholder => Arc::new(|_text, _opts| {
            Box::pin(async { Ok(Some("compressed without protected marker".to_string())) })
        }),
    }
}

fn load_ml_tag_protection_fixtures() -> Vec<(PathBuf, MlTagProtectionFixture)> {
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/ml_tag_protection_fixtures");
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
        "no ML tag protection fixtures found in {}",
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
            FixtureGenerator::WorkflowBlock { filler_words } => {
                let tag = "<workflow id=\"alpha\">\n<context-item value=\"1\" />\n</workflow>";
                format!("{tag}\n{}", "ordinary filler ".repeat(*filler_words))
            }
        }
    }
}
