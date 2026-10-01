use super::*;
use crate::rules::load_builtin_rules;

fn make_input(tool_name: &str, argv: &[&str]) -> ToolExecutionInput {
    ToolExecutionInput {
        tool_name: tool_name.to_owned(),
        argv: Some(argv.iter().map(|s| s.to_string()).collect()),
        ..Default::default()
    }
}

#[test]
fn git_status_matches() {
    let rules = load_builtin_rules();
    let input = make_input("bash", &["git", "status"]);
    let result = classify_execution(&input, &rules, None);
    assert_eq!(result.matched_reducer.as_deref(), Some("git/status"));
    assert_eq!(result.family, "git-status");
}

#[test]
fn npm_install_does_not_match_git_status() {
    let rules = load_builtin_rules();
    let input = make_input("exec", &["npm", "install"]);
    let result = classify_execution(&input, &rules, None);
    assert_ne!(result.matched_reducer.as_deref(), Some("git/status"));
}

#[test]
fn no_match_returns_generic() {
    let rules = load_builtin_rules();
    let input = make_input("some_unknown_tool", &["mysterious", "command"]);
    let result = classify_execution(&input, &rules, None);
    assert_eq!(result.family, "generic");
    assert_eq!(result.confidence, 0.2);
}

#[test]
fn forced_rule_id_overrides_matching() {
    let rules = load_builtin_rules();
    // Input would normally match git/status but we force cargo-test
    let input = make_input("bash", &["git", "status"]);
    let result = classify_execution(&input, &rules, Some("tests/cargo-test"));
    assert_eq!(result.matched_reducer.as_deref(), Some("tests/cargo-test"));
    assert_eq!(result.confidence, 1.0);
}

#[test]
fn fallback_confidence_is_low() {
    let rules = load_builtin_rules();
    // Force the fallback explicitly
    let input = make_input("bash", &["some", "arbitrary", "command"]);
    let result = classify_execution(&input, &rules, Some("generic/fallback"));
    assert_eq!(result.confidence, 1.0); // forced always returns 1.0
}

#[test]
fn git_diff_stat_requires_both_args() {
    let rules = load_builtin_rules();
    // Missing --stat → should not match git/diff-stat
    let input_no_stat = make_input("bash", &["git", "diff"]);
    let result = classify_execution(&input_no_stat, &rules, None);
    assert_ne!(result.matched_reducer.as_deref(), Some("git/diff-stat"));

    // With --stat → should match
    let input_with_stat = make_input("bash", &["git", "diff", "--stat"]);
    let result2 = classify_execution(&input_with_stat, &rules, None);
    assert_eq!(result2.matched_reducer.as_deref(), Some("git/diff-stat"));
}

// --- matches_rule: individual dimension tests ---

#[test]
fn tool_names_filter_blocks_wrong_tool() {
    // cargo test rule requires toolNames: ["exec"]
    let rules = load_builtin_rules();
    // "bash" tool should not match tests/cargo-test (requires "exec")
    let input = ToolExecutionInput {
        tool_name: "bash".to_owned(),
        argv: Some(vec!["cargo".to_owned(), "test".to_owned()]),
        ..Default::default()
    };
    let result = classify_execution(&input, &rules, None);
    assert_ne!(result.matched_reducer.as_deref(), Some("tests/cargo-test"));
}

#[test]
fn tool_names_filter_matches_correct_tool() {
    let rules = load_builtin_rules();
    let input = ToolExecutionInput {
        tool_name: "exec".to_owned(),
        argv: Some(vec!["cargo".to_owned(), "test".to_owned()]),
        ..Default::default()
    };
    let result = classify_execution(&input, &rules, None);
    assert_eq!(
        result.matched_reducer.as_deref(),
        Some("tests/cargo-test"),
        "cargo test with exec tool should match tests/cargo-test"
    );
}

#[test]
fn argv_includes_any_matches_at_least_one_group() {
    // Build a custom rule with argvIncludesAny and test it via matches_rule directly
    use crate::types::{JsonRule, RuleMatch};

    let rule = JsonRule {
        id: "test/any".to_owned(),
        family: "test".to_owned(),
        description: None,
        priority: None,
        on_empty: None,
        match_output: None,
        counter_source: None,
        r#match: RuleMatch {
            argv0: Some(vec!["tool".to_owned()]),
            argv_includes_any: Some(vec![vec!["--foo".to_owned()], vec!["--bar".to_owned()]]),
            ..Default::default()
        },
        filters: None,
        transforms: None,
        summarize: None,
        counters: None,
        failure: None,
    };

    // Should match when --foo is present
    let input_foo = ToolExecutionInput {
        tool_name: "bash".to_owned(),
        argv: Some(vec!["tool".to_owned(), "--foo".to_owned()]),
        ..Default::default()
    };
    assert!(matches_rule(&rule, &input_foo));

    // Should match when --bar is present
    let input_bar = ToolExecutionInput {
        tool_name: "bash".to_owned(),
        argv: Some(vec!["tool".to_owned(), "--bar".to_owned()]),
        ..Default::default()
    };
    assert!(matches_rule(&rule, &input_bar));

    // Should NOT match when neither is present
    let input_none = ToolExecutionInput {
        tool_name: "bash".to_owned(),
        argv: Some(vec!["tool".to_owned(), "--baz".to_owned()]),
        ..Default::default()
    };
    assert!(!matches_rule(&rule, &input_none));
}

#[test]
fn command_includes_all_substrings_required() {
    use crate::types::{JsonRule, RuleMatch};

    let rule = JsonRule {
        id: "test/cmd-incl".to_owned(),
        family: "test".to_owned(),
        description: None,
        priority: None,
        on_empty: None,
        match_output: None,
        counter_source: None,
        r#match: RuleMatch {
            command_includes: Some(vec!["git".to_owned(), "status".to_owned()]),
            ..Default::default()
        },
        filters: None,
        transforms: None,
        summarize: None,
        counters: None,
        failure: None,
    };

    let input_match = ToolExecutionInput {
        tool_name: "bash".to_owned(),
        command: Some("git status --short".to_owned()),
        ..Default::default()
    };
    assert!(matches_rule(&rule, &input_match));

    // Missing "status" → no match
    let input_no_match = ToolExecutionInput {
        tool_name: "bash".to_owned(),
        command: Some("git log --oneline".to_owned()),
        ..Default::default()
    };
    assert!(!matches_rule(&rule, &input_no_match));
}

#[test]
fn command_includes_any_at_least_one_substring() {
    use crate::types::{JsonRule, RuleMatch};

    let rule = JsonRule {
        id: "test/cmd-any".to_owned(),
        family: "test".to_owned(),
        description: None,
        priority: None,
        on_empty: None,
        match_output: None,
        counter_source: None,
        r#match: RuleMatch {
            command_includes_any: Some(vec!["install".to_owned(), "update".to_owned()]),
            ..Default::default()
        },
        filters: None,
        transforms: None,
        summarize: None,
        counters: None,
        failure: None,
    };

    let input_install = ToolExecutionInput {
        tool_name: "bash".to_owned(),
        command: Some("npm install".to_owned()),
        ..Default::default()
    };
    assert!(matches_rule(&rule, &input_install));

    let input_update = ToolExecutionInput {
        tool_name: "bash".to_owned(),
        command: Some("npm update".to_owned()),
        ..Default::default()
    };
    assert!(matches_rule(&rule, &input_update));

    let input_neither = ToolExecutionInput {
        tool_name: "bash".to_owned(),
        command: Some("npm run build".to_owned()),
        ..Default::default()
    };
    assert!(!matches_rule(&rule, &input_neither));
}

#[test]
fn forced_rule_id_not_found_falls_back_to_matching() {
    let rules = load_builtin_rules();
    let input = make_input("bash", &["git", "status"]);
    // Force a non-existent rule ID → should fall through to normal matching
    let result = classify_execution(&input, &rules, Some("nonexistent/rule"));
    // Falls through to normal matching; git status should still match git/status
    assert_eq!(result.matched_reducer.as_deref(), Some("git/status"));
}

#[test]
fn multiple_matches_best_score_wins() {
    let rules = load_builtin_rules();
    // "git diff --stat" should match git/diff-stat (more specific) over git/show or others
    let input = make_input("bash", &["git", "diff", "--stat"]);
    let result = classify_execution(&input, &rules, None);
    assert_eq!(result.matched_reducer.as_deref(), Some("git/diff-stat"));
    assert_eq!(result.confidence, 0.9);
}

#[test]
fn generic_fallback_matched_gives_low_confidence() {
    let rules = load_builtin_rules();
    // An unknown command should match generic/fallback with low confidence
    let input = ToolExecutionInput {
        tool_name: "exec".to_owned(),
        argv: Some(vec!["some_nonexistent_program".to_owned()]),
        ..Default::default()
    };
    let result = classify_execution(&input, &rules, None);
    // generic/fallback matches everything, so it will be the winner for unknown commands
    // but confidence should be low (0.2)
    assert_eq!(result.confidence, 0.2);
}

// --- Rust toolchain classification tests ---

#[test]
fn cargo_clippy_matches_lint_cargo_clippy() {
    let rules = load_builtin_rules();
    let input = make_input("exec", &["cargo", "clippy"]);
    let result = classify_execution(&input, &rules, None);
    assert_eq!(
        result.matched_reducer.as_deref(),
        Some("lint/cargo-clippy"),
        "cargo clippy should match lint/cargo-clippy"
    );
    assert_eq!(result.family, "lint-results");
}

#[test]
fn cargo_clippy_with_flags_matches() {
    let rules = load_builtin_rules();
    let input = make_input("exec", &["cargo", "clippy", "--", "-D", "warnings"]);
    let result = classify_execution(&input, &rules, None);
    assert_eq!(
        result.matched_reducer.as_deref(),
        Some("lint/cargo-clippy"),
        "cargo clippy with flags should still match lint/cargo-clippy"
    );
}

#[test]
fn cargo_build_matches_build_cargo_build() {
    let rules = load_builtin_rules();
    let input = make_input("exec", &["cargo", "build"]);
    let result = classify_execution(&input, &rules, None);
    assert_eq!(
        result.matched_reducer.as_deref(),
        Some("build/cargo-build"),
        "cargo build should match build/cargo-build"
    );
    assert_eq!(result.family, "build-rust");
}

#[test]
fn cargo_check_matches_build_cargo_build() {
    let rules = load_builtin_rules();
    let input = make_input("exec", &["cargo", "check"]);
    let result = classify_execution(&input, &rules, None);
    assert_eq!(
        result.matched_reducer.as_deref(),
        Some("build/cargo-build"),
        "cargo check should match build/cargo-build via argvIncludesAny"
    );
    assert_eq!(result.family, "build-rust");
}

#[test]
fn cargo_fmt_matches_lint_cargo_fmt() {
    let rules = load_builtin_rules();
    let input = make_input("exec", &["cargo", "fmt", "--check"]);
    let result = classify_execution(&input, &rules, None);
    assert_eq!(
        result.matched_reducer.as_deref(),
        Some("lint/cargo-fmt"),
        "cargo fmt should match lint/cargo-fmt"
    );
    assert_eq!(result.family, "lint-results");
}

#[test]
fn cargo_doc_matches_build_cargo_doc() {
    let rules = load_builtin_rules();
    let input = make_input("exec", &["cargo", "doc"]);
    let result = classify_execution(&input, &rules, None);
    assert_eq!(
        result.matched_reducer.as_deref(),
        Some("build/cargo-doc"),
        "cargo doc should match build/cargo-doc"
    );
    assert_eq!(result.family, "build-rust");
}

#[test]
fn cargo_clippy_does_not_match_cargo_test() {
    let rules = load_builtin_rules();
    let input = make_input("exec", &["cargo", "clippy"]);
    let result = classify_execution(&input, &rules, None);
    assert_ne!(
        result.matched_reducer.as_deref(),
        Some("tests/cargo-test"),
        "cargo clippy must NOT match tests/cargo-test"
    );
}

#[test]
fn cargo_build_does_not_match_cargo_test() {
    let rules = load_builtin_rules();
    let input = make_input("exec", &["cargo", "build"]);
    let result = classify_execution(&input, &rules, None);
    assert_ne!(
        result.matched_reducer.as_deref(),
        Some("tests/cargo-test"),
        "cargo build must NOT match tests/cargo-test"
    );
}
