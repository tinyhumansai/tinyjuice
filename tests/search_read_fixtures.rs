//! Fixture coverage for ranked search-read host adapter helpers.

use serde::Deserialize;
use std::borrow::Cow;
use std::path::PathBuf;
use tinyjuice::LineRange;
use tinyjuice::compressors::search::{
    SearchReadCandidate, SearchReadLine, SearchReadQuery, rank_search_read_candidate,
    select_snippet_windows,
};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct SearchReadFixture {
    description: String,
    query: FixtureQuery,
    candidates: Vec<FixtureCandidate>,
    expected: FixtureExpectations,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct FixtureQuery {
    #[serde(default)]
    literal: Option<String>,
    #[serde(default)]
    regex: Option<String>,
    #[serde(default)]
    symbols: Vec<String>,
    #[serde(default = "default_true")]
    penalize_vendor: bool,
    #[serde(default = "default_true")]
    penalize_generated: bool,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct FixtureCandidate {
    path: String,
    #[serde(default)]
    matched_lines: Vec<FixtureLine>,
    #[serde(default)]
    imports: Vec<String>,
    #[serde(default)]
    exports: Vec<String>,
    #[serde(default)]
    generated: bool,
    #[serde(default)]
    vendor: bool,
    max_line: usize,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct FixtureLine {
    line_number: usize,
    text: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct FixtureExpectations {
    ordered_paths: Vec<String>,
    #[serde(default)]
    snippets: Vec<SnippetExpectation>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct SnippetExpectation {
    path: String,
    context: usize,
    max_snippets: usize,
    windows: Vec<LineRange>,
    omitted_matches: usize,
}

#[test]
fn search_read_fixtures_match_expectations() {
    for (path, fixture) in load_search_read_fixtures() {
        let query = fixture.query.to_query();
        let candidates = fixture
            .candidates
            .iter()
            .map(FixtureCandidate::to_candidate)
            .collect::<Vec<_>>();
        let mut scored = candidates
            .iter()
            .map(|candidate| {
                (
                    candidate.path.to_string(),
                    rank_search_read_candidate(candidate, &query),
                )
            })
            .collect::<Vec<_>>();
        scored.sort_by(|a, b| {
            b.1.partial_cmp(&a.1)
                .unwrap_or(std::cmp::Ordering::Equal)
                .then_with(|| a.0.cmp(&b.0))
        });
        let ordered_paths = scored
            .iter()
            .map(|(candidate_path, _)| candidate_path.clone())
            .collect::<Vec<_>>();

        assert_eq!(
            ordered_paths,
            fixture.expected.ordered_paths,
            "{}: unexpected ranked path order for {} with scores {scored:?}",
            path.display(),
            fixture.description
        );

        for expectation in &fixture.expected.snippets {
            let candidate = candidates
                .iter()
                .find(|candidate| candidate.path == expectation.path)
                .unwrap_or_else(|| {
                    panic!(
                        "{}: snippet expectation references missing candidate {}",
                        path.display(),
                        expectation.path
                    )
                });
            let match_lines = candidate
                .matched_lines
                .iter()
                .map(|line| line.line_number)
                .collect::<Vec<_>>();
            let selected = select_snippet_windows(
                &match_lines,
                expectation.context,
                candidate.max_line,
                expectation.max_snippets,
            );

            assert_eq!(
                selected.windows,
                expectation.windows,
                "{}: unexpected snippet windows for {} in {}",
                path.display(),
                expectation.path,
                fixture.description
            );
            assert_eq!(
                selected.omitted_matches,
                expectation.omitted_matches,
                "{}: unexpected omitted match count for {} in {}",
                path.display(),
                expectation.path,
                fixture.description
            );
        }
    }
}

fn default_true() -> bool {
    true
}

fn load_search_read_fixtures() -> Vec<(PathBuf, SearchReadFixture)> {
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/search_read_fixtures");
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
        "no search-read fixtures found in {}",
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

impl FixtureQuery {
    fn to_query(&self) -> SearchReadQuery {
        SearchReadQuery {
            literal: self.literal.clone(),
            regex: self.regex.clone(),
            symbols: self.symbols.clone(),
            file_kinds: Vec::new(),
            penalize_vendor: self.penalize_vendor,
            penalize_generated: self.penalize_generated,
        }
    }
}

impl FixtureCandidate {
    fn to_candidate(&self) -> SearchReadCandidate<'static> {
        SearchReadCandidate {
            path: Cow::Owned(self.path.clone()),
            matched_lines: self
                .matched_lines
                .iter()
                .map(|line| SearchReadLine::new(line.line_number, line.text.clone()))
                .collect(),
            imports: self.imports.iter().cloned().map(Cow::Owned).collect(),
            exports: self.exports.iter().cloned().map(Cow::Owned).collect(),
            generated: self.generated,
            vendor: self.vendor,
            max_line: self.max_line,
        }
    }
}
