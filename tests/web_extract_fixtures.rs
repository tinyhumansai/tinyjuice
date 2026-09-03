//! Fixture-driven coverage for already-extracted web page reduction.

use serde::Deserialize;
use serde_json::Map;
use std::fmt::Write as _;
use std::path::PathBuf;
use tinyjuice::cache::{CcrStore, MemoryCcrStore};
use tinyjuice::{
    WebExtractFormat, WebExtractOptions, WebExtractReduceInput, reduce_web_extract_with_store,
};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct WebExtractFixture {
    description: String,
    url: String,
    #[serde(default)]
    title: Option<String>,
    #[serde(default)]
    format: WebExtractFormat,
    char_limit: usize,
    generator: WebExtractGenerator,
    expected: WebExtractExpectations,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase", tag = "type")]
enum WebExtractGenerator {
    MarkdownArticle {
        #[serde(rename = "paragraphs")]
        paragraphs: usize,
        #[serde(rename = "base64Payload")]
        base64_payload: String,
        #[serde(rename = "remoteImageUrl")]
        remote_image_url: String,
    },
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct WebExtractExpectations {
    truncated: bool,
    full_text_retained: bool,
    base64_images_replaced: usize,
    #[serde(default)]
    contains: Vec<String>,
    #[serde(default)]
    not_contains: Vec<String>,
    #[serde(default)]
    serialized_not_contains: Vec<String>,
    recovered_text_contains: Vec<String>,
    recovered_text_not_contains: Vec<String>,
}

#[test]
fn web_extract_fixtures_match_expectations() {
    for (path, fixture) in load_web_extract_fixtures() {
        let input = WebExtractReduceInput {
            url: fixture.url,
            title: fixture.title,
            content: fixture.generator.render(),
            format: fixture.format,
            provider: Some("fixture".to_owned()),
            char_limit: Some(fixture.char_limit),
            metadata: Map::new(),
        };
        let store = MemoryCcrStore::new(10, input.content.len() * 2);
        let out = reduce_web_extract_with_store(&input, &fixture_options(), &store);

        assert_eq!(
            out.truncated,
            fixture.expected.truncated,
            "{}: unexpected truncation state for {}",
            path.display(),
            fixture.description
        );
        assert_eq!(
            out.full_text_retained,
            fixture.expected.full_text_retained,
            "{}: unexpected retention state for {}",
            path.display(),
            fixture.description
        );
        assert_eq!(
            out.base64_images_replaced,
            fixture.expected.base64_images_replaced,
            "{}: unexpected base64 replacement count",
            path.display()
        );

        for needle in &fixture.expected.contains {
            assert!(
                out.text.contains(needle),
                "{}: output missing {needle:?}\n---\n{}\n---",
                path.display(),
                out.text
            );
        }
        for needle in &fixture.expected.not_contains {
            assert!(
                !out.text.contains(needle),
                "{}: output unexpectedly contained {needle:?}\n---\n{}\n---",
                path.display(),
                out.text
            );
        }

        let serialized = serde_json::to_string(&out).expect("serialize web extract reduction");
        for needle in &fixture.expected.serialized_not_contains {
            assert!(
                !serialized.contains(needle),
                "{}: serialized metadata unexpectedly contained {needle:?}",
                path.display()
            );
        }

        if fixture.expected.full_text_retained {
            let token = out.ccr_token.as_deref().expect("retained token");
            let recovered = store.get(token).expect("recover retained web text");
            for needle in &fixture.expected.recovered_text_contains {
                assert!(
                    recovered.contains(needle),
                    "{}: recovered text missing {needle:?}",
                    path.display()
                );
            }
            for needle in &fixture.expected.recovered_text_not_contains {
                assert!(
                    !recovered.contains(needle),
                    "{}: recovered text unexpectedly contained {needle:?}",
                    path.display()
                );
            }
        }
    }
}

fn fixture_options() -> WebExtractOptions {
    WebExtractOptions {
        char_limit: 120,
        min_char_limit: 40,
        max_char_limit: 500,
        head_ratio: 0.75,
        convert_base64_images: true,
        max_combined_inline_chars: 1000,
    }
}

fn load_web_extract_fixtures() -> Vec<(PathBuf, WebExtractFixture)> {
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/web_extract_fixtures");
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
        "no web extract fixtures found in {}",
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

impl WebExtractGenerator {
    fn render(&self) -> String {
        match self {
            WebExtractGenerator::MarkdownArticle {
                paragraphs,
                base64_payload,
                remote_image_url,
            } => {
                let mut out = format!(
                    "intro ![inline](data:image/png;base64,{base64_payload}) ![remote]({remote_image_url})\n"
                );
                for i in 0..*paragraphs {
                    let _ = writeln!(
                        out,
                        "paragraph-{i:03}: extracted article text with details and repeated context."
                    );
                }
                out.push_str("final web sentinel\n");
                out
            }
        }
    }
}
