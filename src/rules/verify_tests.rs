use super::super::load_builtin_rules;
use super::*;

fn write_rule(dir: &Path, filename: &str, json: &str) {
    std::fs::write(dir.join(filename), json).expect("write rule");
}

fn write_fixture(dir: &Path, filename: &str, json: &str) {
    std::fs::write(dir.join(filename), json).expect("write fixture");
}

#[test]
fn builtin_rules_verify_cleanly() {
    let report = verify_rules(&LoadRuleOptions {
        exclude_user: true,
        exclude_project: true,
        ..Default::default()
    });

    assert_eq!(report.descriptors_seen, 101);
    assert_eq!(report.valid_rules, 101);
    assert_eq!(report.final_rules, 101);
    assert!(report.parse_errors.is_empty(), "{report:#?}");
    assert!(report.duplicate_ids.is_empty(), "{report:#?}");
    assert!(report.invalid_regexes.is_empty(), "{report:#?}");
    assert!(report.shadowed_rules.is_empty());
}

#[test]
fn verifier_reports_parse_and_regex_errors() {
    let dir = tempfile::tempdir().expect("tempdir");
    write_rule(dir.path(), "bad.json", "{ not json }");
    write_rule(
        dir.path(),
        "bad_regex.json",
        r#"{
                "id": "test/bad-regex",
                "family": "test",
                "match": {},
                "filters": { "skipPatterns": ["[invalid"] },
                "counters": [{ "name": "bad", "pattern": "(unclosed" }],
                "matchOutput": [{ "pattern": "[bad", "message": "ignored" }]
            }"#,
    );

    let report = verify_rules(&LoadRuleOptions {
        project_rules_dir: Some(dir.path().to_owned()),
        exclude_user: true,
        ..Default::default()
    });

    assert_eq!(report.parse_errors.len(), 1);
    assert_eq!(report.parse_errors[0].source, RuleOrigin::Project);
    let project_regex_errors = report
        .invalid_regexes
        .iter()
        .filter(|error| error.source == RuleOrigin::Project)
        .collect::<Vec<_>>();
    assert_eq!(project_regex_errors.len(), 3);
    assert!(
        project_regex_errors
            .iter()
            .any(|error| error.field == "filters.skipPatterns")
    );
    assert!(
        project_regex_errors
            .iter()
            .any(|error| error.field == "counters.pattern")
    );
    assert!(
        project_regex_errors
            .iter()
            .any(|error| error.field == "matchOutput.pattern")
    );
}

#[test]
fn verifier_reports_duplicate_and_shadowed_rules() {
    let user_dir = tempfile::tempdir().expect("user tempdir");
    let project_dir = tempfile::tempdir().expect("project tempdir");
    write_rule(
        user_dir.path(),
        "a.json",
        r#"{"id":"git/status","family":"user","match":{}}"#,
    );
    write_rule(
        project_dir.path(),
        "b.json",
        r#"{"id":"git/status","family":"project","match":{}}"#,
    );

    let report = verify_rules(&LoadRuleOptions {
        user_rules_dir: Some(user_dir.path().to_owned()),
        project_rules_dir: Some(project_dir.path().to_owned()),
        ..Default::default()
    });

    let duplicate = report
        .duplicate_ids
        .iter()
        .find(|duplicate| duplicate.rule_id == "git/status")
        .expect("git/status duplicate reported");
    assert_eq!(duplicate.occurrences.len(), 3);

    let shadowed = report
        .shadowed_rules
        .iter()
        .find(|shadowed| shadowed.rule_id == "git/status")
        .expect("git/status shadowing reported");
    assert_eq!(shadowed.active.source, RuleOrigin::Project);
    assert_eq!(shadowed.shadowed.len(), 2);
}

#[test]
fn fixture_verifier_reports_existing_fixtures_pass() {
    let fixtures_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures");
    assert!(
        fixtures_dir.is_dir(),
        "missing fixture directory {}",
        fixtures_dir.display()
    );

    let rules = load_builtin_rules();
    let report = verify_rule_fixtures(&fixtures_dir, &rules);

    assert_eq!(report.fixtures_seen, 5, "{report:#?}");
    assert_eq!(report.passed, 5, "{report:#?}");
    assert!(report.parse_errors.is_empty(), "{report:#?}");
    assert!(report.failures.is_empty(), "{report:#?}");
    assert!(report.is_clean());
}

#[test]
fn fixture_verifier_reports_hash_only_mismatch() {
    let dir = tempfile::tempdir().expect("tempdir");
    write_fixture(
        dir.path(),
        "mismatch.fixture.json",
        r#"{
                "description": "non-sensitive mismatch marker",
                "input": {
                    "toolName": "bash",
                    "argv": ["git", "status"],
                    "stdout": "On branch main\n\nChanges not staged for commit:\n\tmodified:   src/foo.rs\n"
                },
                "expectedOutput": "wrong compact text"
            }"#,
    );

    let rules = load_builtin_rules();
    let report = verify_rule_fixtures(dir.path(), &rules);

    assert_eq!(report.fixtures_seen, 1, "{report:#?}");
    assert_eq!(report.passed, 0, "{report:#?}");
    assert!(report.parse_errors.is_empty(), "{report:#?}");
    assert_eq!(report.failures.len(), 1, "{report:#?}");

    let failure = &report.failures[0];
    assert_eq!(failure.family, "git-status");
    assert_eq!(failure.matched_rule_id.as_deref(), Some("git/status"));
    assert_ne!(failure.expected_sha256, failure.actual_sha256);
    assert_eq!(failure.expected_bytes, "wrong compact text".len());

    let diagnostic = format!("{failure:?}");
    assert!(!diagnostic.contains("wrong compact text"));
    assert!(!diagnostic.contains("On branch main"));
    assert!(!diagnostic.contains("src/foo.rs"));
}

#[test]
fn fixture_verifier_reports_parse_errors() {
    let dir = tempfile::tempdir().expect("tempdir");
    write_fixture(dir.path(), "bad.fixture.json", "{ not json }");

    let rules = load_builtin_rules();
    let report = verify_rule_fixtures(dir.path(), &rules);

    assert_eq!(report.fixtures_seen, 1, "{report:#?}");
    assert_eq!(report.passed, 0, "{report:#?}");
    assert_eq!(report.parse_errors.len(), 1, "{report:#?}");
    assert!(report.failures.is_empty(), "{report:#?}");
    assert!(!report.is_clean());
}

#[test]
fn discovery_groups_generic_fallback_outputs_by_command_family() {
    let rules = load_builtin_rules();
    let inputs = vec![
        ToolExecutionInput {
            tool_name: "bash".to_owned(),
            argv: Some(vec!["unknown-build".to_owned(), "--json".to_owned()]),
            stdout: Some("secret output should not be reported".to_owned()),
            ..Default::default()
        },
        ToolExecutionInput {
            tool_name: "bash".to_owned(),
            command: Some("/tmp/work/unknown-build --again".to_owned()),
            stderr: Some("another secret output".to_owned()),
            ..Default::default()
        },
        ToolExecutionInput {
            tool_name: "bash".to_owned(),
            argv: Some(vec!["git".to_owned(), "status".to_owned()]),
            stdout: Some("On branch main".to_owned()),
            ..Default::default()
        },
    ];

    let report = discover_fallback_outputs(inputs, &rules);

    assert_eq!(report.inputs_seen, 3, "{report:#?}");
    assert_eq!(report.fallback_outputs, 2, "{report:#?}");
    assert_eq!(report.families.len(), 1, "{report:#?}");
    assert_eq!(report.families[0].tool_name, "bash");
    assert_eq!(report.families[0].argv0, "unknown-build");
    assert_eq!(report.families[0].count, 2);
    assert!(!report.is_empty());

    let diagnostic = format!("{report:?}");
    assert!(!diagnostic.contains("secret output"));
    assert!(!diagnostic.contains("/tmp/work"));
    assert!(!diagnostic.contains("On branch main"));
}

#[test]
fn discovery_sorts_fallback_families_by_count_then_label() {
    let rules = load_builtin_rules();
    let inputs = vec![
        ToolExecutionInput {
            tool_name: "bash".to_owned(),
            argv: Some(vec!["beta".to_owned()]),
            ..Default::default()
        },
        ToolExecutionInput {
            tool_name: "bash".to_owned(),
            argv: Some(vec!["alpha".to_owned()]),
            ..Default::default()
        },
        ToolExecutionInput {
            tool_name: "bash".to_owned(),
            argv: Some(vec!["beta".to_owned(), "again".to_owned()]),
            ..Default::default()
        },
    ];

    let report = discover_fallback_outputs(inputs, &rules);

    assert_eq!(
        report
            .families
            .iter()
            .map(|family| (family.argv0.as_str(), family.count))
            .collect::<Vec<_>>(),
        vec![("beta", 2), ("alpha", 1)]
    );
}

#[test]
fn discovery_normalizes_command_only_inputs_before_classification() {
    let rules = load_builtin_rules();
    let report = discover_fallback_outputs(
        [ToolExecutionInput {
            tool_name: "bash".to_owned(),
            command: Some("git status".to_owned()),
            ..Default::default()
        }],
        &rules,
    );

    assert!(report.is_empty(), "{report:#?}");
}
