//! Fixture-driven rule-engine tests.
//!
//! Each `tests/fixtures/*.fixture.json` records a tool execution and the exact
//! reduced text the rule engine must produce. Adding coverage for a new rule
//! or a regression is a new JSON file, not new Rust code. Golden outputs are
//! compared exactly (modulo trailing whitespace).

mod common;

use tinyjuice::reduce_execution_with_rules;
use tinyjuice::rules::load_builtin_rules;
use tinyjuice::types::ReduceOptions;

#[test]
fn all_reduce_fixtures_produce_expected_output() {
    let rules = load_builtin_rules();
    let mut failures = Vec::new();

    for (path, fixture) in common::load_reduce_fixtures() {
        let result =
            reduce_execution_with_rules(fixture.input.clone(), &rules, &ReduceOptions::default());
        let got = result.inline_text.trim_end();
        let want = fixture.expected_output.trim_end();
        if got != want {
            failures.push(format!(
                "fixture {} ({}):\n  family: {}\n  expected:\n---\n{}\n---\n  got:\n---\n{}\n---",
                path.file_name().unwrap().to_string_lossy(),
                fixture.description,
                result.classification.family,
                want,
                got,
            ));
        }
    }

    assert!(
        failures.is_empty(),
        "{} fixture(s) failed:\n\n{}",
        failures.len(),
        failures.join("\n\n")
    );
}

#[test]
fn fixtures_report_shrinkage_stats_consistently() {
    let rules = load_builtin_rules();
    for (path, fixture) in common::load_reduce_fixtures() {
        let result = reduce_execution_with_rules(fixture.input, &rules, &ReduceOptions::default());
        assert!(
            result.stats.reduced_chars <= result.stats.raw_chars,
            "{}: reduced_chars ({}) exceeds raw_chars ({})",
            path.display(),
            result.stats.reduced_chars,
            result.stats.raw_chars,
        );
    }
}
