use super::builtin::BUILTIN_RULE_JSONS;
use super::loader::{LoadRuleOptions, list_rule_files, project_rules_root, user_rules_root};
use crate::classify::classify_execution;
use crate::reduce::reduce_execution_with_rules;
use crate::types::{CompiledRule, JsonRule, RuleFixture, RuleOrigin, ToolExecutionInput};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RuleVerificationReport {
    pub descriptors_seen: usize,
    pub valid_rules: usize,
    pub final_rules: usize,
    pub parse_errors: Vec<RuleParseError>,
    pub invalid_regexes: Vec<RuleRegexError>,
    pub duplicate_ids: Vec<RuleDuplicateId>,
    pub shadowed_rules: Vec<RuleShadowedRule>,
}

impl RuleVerificationReport {
    pub fn is_clean(&self) -> bool {
        self.parse_errors.is_empty()
            && self.invalid_regexes.is_empty()
            && self.duplicate_ids.is_empty()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RuleParseError {
    pub source: RuleOrigin,
    pub path: String,
    pub error: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RuleRegexError {
    pub source: RuleOrigin,
    pub path: String,
    pub rule_id: String,
    pub field: String,
    pub index: usize,
    pub error: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RuleDuplicateId {
    pub rule_id: String,
    pub occurrences: Vec<RuleDescriptorRef>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RuleShadowedRule {
    pub rule_id: String,
    pub active: RuleDescriptorRef,
    pub shadowed: Vec<RuleDescriptorRef>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RuleDescriptorRef {
    pub source: RuleOrigin,
    pub path: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RuleFixtureVerificationReport {
    pub fixtures_seen: usize,
    pub passed: usize,
    pub parse_errors: Vec<RuleFixtureParseError>,
    pub failures: Vec<RuleFixtureFailure>,
}

impl RuleFixtureVerificationReport {
    pub fn is_clean(&self) -> bool {
        self.parse_errors.is_empty() && self.failures.is_empty()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RuleFixtureParseError {
    pub path: String,
    pub error: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RuleFixtureFailure {
    pub path: String,
    pub family: String,
    #[serde(default)]
    pub matched_rule_id: Option<String>,
    pub expected_sha256: String,
    pub actual_sha256: String,
    pub expected_bytes: usize,
    pub actual_bytes: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RuleDiscoveryReport {
    pub inputs_seen: usize,
    pub fallback_outputs: usize,
    pub families: Vec<RuleDiscoveryFamily>,
}

impl RuleDiscoveryReport {
    pub fn is_empty(&self) -> bool {
        self.fallback_outputs == 0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RuleDiscoveryFamily {
    pub tool_name: String,
    pub argv0: String,
    pub count: usize,
}

#[derive(Debug, Clone)]
struct RuleDescriptor {
    source: RuleOrigin,
    path: String,
    rule: JsonRule,
}

/// Verify rule roots without changing the lenient loader contract.
pub fn verify_rules(opts: &LoadRuleOptions) -> RuleVerificationReport {
    let mut descriptors = Vec::new();
    let mut parse_errors = Vec::new();

    collect_builtin_descriptors(&mut descriptors, &mut parse_errors);
    if !opts.exclude_user {
        let root = user_rules_root(opts.user_rules_dir.as_deref());
        collect_disk_descriptors(&root, RuleOrigin::User, &mut descriptors, &mut parse_errors);
    }
    if !opts.exclude_project {
        let root = project_rules_root(opts.cwd.as_deref(), opts.project_rules_dir.as_deref());
        collect_disk_descriptors(
            &root,
            RuleOrigin::Project,
            &mut descriptors,
            &mut parse_errors,
        );
    }

    let descriptors_seen = descriptors.len() + parse_errors.len();
    let invalid_regexes = collect_regex_errors(&descriptors);
    let duplicate_ids = collect_duplicate_ids(&descriptors);
    let shadowed_rules = collect_shadowed_rules(&descriptors);
    let final_rules = descriptors
        .iter()
        .map(|descriptor| descriptor.rule.id.as_str())
        .collect::<std::collections::HashSet<_>>()
        .len();

    RuleVerificationReport {
        descriptors_seen,
        valid_rules: descriptors.len(),
        final_rules,
        parse_errors,
        invalid_regexes,
        duplicate_ids,
        shadowed_rules,
    }
}

/// Verify recorded rule fixtures without exposing raw fixture input or output.
pub fn verify_rule_fixtures(
    fixtures_dir: impl AsRef<Path>,
    rules: &[CompiledRule],
) -> RuleFixtureVerificationReport {
    let mut report = RuleFixtureVerificationReport {
        fixtures_seen: 0,
        passed: 0,
        parse_errors: Vec::new(),
        failures: Vec::new(),
    };

    for path in list_fixture_files(fixtures_dir.as_ref()) {
        report.fixtures_seen += 1;
        let path_label = path.display().to_string();
        match read_fixture(&path) {
            Ok(fixture) => verify_fixture(&path_label, fixture, rules, &mut report),
            Err(error) => report.parse_errors.push(RuleFixtureParseError {
                path: path_label,
                error,
            }),
        }
    }

    report
}

/// Count command families that currently fall through to `generic/fallback`.
pub fn discover_fallback_outputs<I>(inputs: I, rules: &[CompiledRule]) -> RuleDiscoveryReport
where
    I: IntoIterator<Item = ToolExecutionInput>,
{
    let mut inputs_seen = 0;
    let mut fallback_outputs = 0;
    let mut counts: HashMap<(String, String), usize> = HashMap::new();

    for input in inputs {
        inputs_seen += 1;
        let classification = classify_execution(&input, rules, None);
        if !is_generic_fallback(&classification) {
            continue;
        }

        fallback_outputs += 1;
        let key = discovery_key_for_input(&input);
        *counts.entry(key).or_default() += 1;
    }

    let mut families = counts
        .into_iter()
        .map(|((tool_name, argv0), count)| RuleDiscoveryFamily {
            tool_name,
            argv0,
            count,
        })
        .collect::<Vec<_>>();
    families.sort_by(|a, b| {
        b.count
            .cmp(&a.count)
            .then_with(|| a.tool_name.cmp(&b.tool_name))
            .then_with(|| a.argv0.cmp(&b.argv0))
    });

    RuleDiscoveryReport {
        inputs_seen,
        fallback_outputs,
        families,
    }
}

fn is_generic_fallback(classification: &crate::types::ClassificationResult) -> bool {
    classification.matched_reducer.as_deref() == Some("generic/fallback")
        || (classification.family == "generic" && classification.matched_reducer.is_none())
}

fn discovery_key_for_input(input: &ToolExecutionInput) -> (String, String) {
    (
        sanitize_discovery_label(&input.tool_name),
        discovery_argv0(input)
            .map(sanitize_discovery_label)
            .unwrap_or_else(|| "unknown".to_owned()),
    )
}

fn discovery_argv0(input: &ToolExecutionInput) -> Option<&str> {
    input
        .argv
        .as_ref()
        .and_then(|argv| argv.first().map(String::as_str))
        .or_else(|| {
            input
                .command
                .as_deref()
                .and_then(|command| command.split_whitespace().next())
        })
}

fn sanitize_discovery_label(raw: &str) -> String {
    let trimmed = raw.trim_matches(|ch: char| ch.is_whitespace() || ch == '"' || ch == '\'');
    let basename = trimmed.rsplit(['/', '\\']).next().unwrap_or(trimmed);
    let label = basename
        .chars()
        .filter(|ch| ch.is_ascii_alphanumeric() || matches!(ch, '.' | '_' | '-' | '+' | ':'))
        .take(80)
        .collect::<String>();
    if label.is_empty() {
        "unknown".to_owned()
    } else {
        label
    }
}

fn list_fixture_files(root: &Path) -> Vec<PathBuf> {
    let mut out = Vec::new();
    collect_fixture_files(root, &mut out);
    out.sort();
    out
}

fn collect_fixture_files(root: &Path, out: &mut Vec<PathBuf>) {
    let Ok(entries) = std::fs::read_dir(root) else {
        return;
    };

    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            collect_fixture_files(&path, out);
        } else if path
            .file_name()
            .and_then(|name| name.to_str())
            .is_some_and(|name| name.ends_with(".fixture.json"))
        {
            out.push(path);
        }
    }
}

fn read_fixture(path: &Path) -> Result<RuleFixture, String> {
    let raw = std::fs::read_to_string(path).map_err(|error| error.to_string())?;
    serde_json::from_str(&raw).map_err(|error| error.to_string())
}

fn verify_fixture(
    path: &str,
    fixture: RuleFixture,
    rules: &[CompiledRule],
    report: &mut RuleFixtureVerificationReport,
) {
    let expected = fixture.expected_output.trim_end();
    let options = fixture.options.unwrap_or_default();
    let result = reduce_execution_with_rules(fixture.input, rules, &options);
    let actual = result.inline_text.trim_end();

    if actual == expected {
        report.passed += 1;
        return;
    }

    report.failures.push(RuleFixtureFailure {
        path: path.to_owned(),
        family: result.classification.family,
        matched_rule_id: result.classification.matched_reducer,
        expected_sha256: sha256_hex(expected),
        actual_sha256: sha256_hex(actual),
        expected_bytes: expected.len(),
        actual_bytes: actual.len(),
    });
}

fn sha256_hex(text: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(text.as_bytes());
    hex::encode(hasher.finalize())
}

fn collect_builtin_descriptors(
    descriptors: &mut Vec<RuleDescriptor>,
    parse_errors: &mut Vec<RuleParseError>,
) {
    for (id, json) in BUILTIN_RULE_JSONS {
        let path = format!("builtin:{id}");
        match serde_json::from_str::<JsonRule>(json) {
            Ok(rule) => descriptors.push(RuleDescriptor {
                source: RuleOrigin::Builtin,
                path,
                rule,
            }),
            Err(error) => parse_errors.push(RuleParseError {
                source: RuleOrigin::Builtin,
                path,
                error: error.to_string(),
            }),
        }
    }
}

fn collect_disk_descriptors(
    root: &Path,
    source: RuleOrigin,
    descriptors: &mut Vec<RuleDescriptor>,
    parse_errors: &mut Vec<RuleParseError>,
) {
    for path in list_rule_files(root) {
        let path_label = path.display().to_string();
        match std::fs::read_to_string(&path) {
            Ok(json) => match serde_json::from_str::<JsonRule>(&json) {
                Ok(rule) => descriptors.push(RuleDescriptor {
                    source: source.clone(),
                    path: path_label,
                    rule,
                }),
                Err(error) => parse_errors.push(RuleParseError {
                    source: source.clone(),
                    path: path_label,
                    error: error.to_string(),
                }),
            },
            Err(error) => parse_errors.push(RuleParseError {
                source: source.clone(),
                path: path_label,
                error: error.to_string(),
            }),
        }
    }
}

fn collect_regex_errors(descriptors: &[RuleDescriptor]) -> Vec<RuleRegexError> {
    let mut out = Vec::new();
    for descriptor in descriptors {
        if let Some(filters) = &descriptor.rule.filters {
            if let Some(patterns) = &filters.skip_patterns {
                collect_pattern_errors(
                    descriptor,
                    "filters.skipPatterns",
                    patterns,
                    None,
                    &mut out,
                );
            }
            if let Some(patterns) = &filters.keep_patterns {
                collect_pattern_errors(
                    descriptor,
                    "filters.keepPatterns",
                    patterns,
                    None,
                    &mut out,
                );
            }
        }
        if let Some(counters) = &descriptor.rule.counters {
            for (index, counter) in counters.iter().enumerate() {
                collect_single_regex_error(
                    descriptor,
                    "counters.pattern",
                    index,
                    &counter.pattern,
                    counter.flags.as_deref(),
                    &mut out,
                );
            }
        }
        if let Some(entries) = &descriptor.rule.match_output {
            for (index, entry) in entries.iter().enumerate() {
                collect_single_regex_error(
                    descriptor,
                    "matchOutput.pattern",
                    index,
                    &entry.pattern,
                    entry.flags.as_deref(),
                    &mut out,
                );
            }
        }
    }
    out
}

fn collect_pattern_errors(
    descriptor: &RuleDescriptor,
    field: &'static str,
    patterns: &[String],
    flags: Option<&str>,
    out: &mut Vec<RuleRegexError>,
) {
    for (index, pattern) in patterns.iter().enumerate() {
        collect_single_regex_error(descriptor, field, index, pattern, flags, out);
    }
}

fn collect_single_regex_error(
    descriptor: &RuleDescriptor,
    field: &'static str,
    index: usize,
    pattern: &str,
    flags: Option<&str>,
    out: &mut Vec<RuleRegexError>,
) {
    if let Err(error) = build_rule_regex(pattern, flags) {
        out.push(RuleRegexError {
            source: descriptor.source.clone(),
            path: descriptor.path.clone(),
            rule_id: descriptor.rule.id.clone(),
            field: field.to_owned(),
            index,
            error,
        });
    }
}

fn build_rule_regex(pattern: &str, flags: Option<&str>) -> Result<regex::Regex, String> {
    let case_insensitive = flags.map(|f| f.contains('i')).unwrap_or(false);
    let multiline = flags.map(|f| f.contains('m')).unwrap_or(false);
    let prefix = match (case_insensitive, multiline) {
        (true, true) => "(?im)",
        (true, false) => "(?i)",
        (false, true) => "(?m)",
        (false, false) => "",
    };
    regex::Regex::new(&format!("{prefix}{pattern}")).map_err(|error| error.to_string())
}

fn collect_duplicate_ids(descriptors: &[RuleDescriptor]) -> Vec<RuleDuplicateId> {
    let mut by_id: HashMap<&str, Vec<RuleDescriptorRef>> = HashMap::new();
    for descriptor in descriptors {
        by_id
            .entry(descriptor.rule.id.as_str())
            .or_default()
            .push(ref_for_descriptor(descriptor));
    }
    let mut duplicates = by_id
        .into_iter()
        .filter_map(|(rule_id, occurrences)| {
            (occurrences.len() > 1).then(|| RuleDuplicateId {
                rule_id: rule_id.to_owned(),
                occurrences,
            })
        })
        .collect::<Vec<_>>();
    duplicates.sort_by(|a, b| a.rule_id.cmp(&b.rule_id));
    duplicates
}

fn collect_shadowed_rules(descriptors: &[RuleDescriptor]) -> Vec<RuleShadowedRule> {
    let mut by_id: HashMap<&str, Vec<&RuleDescriptor>> = HashMap::new();
    for descriptor in descriptors {
        by_id
            .entry(descriptor.rule.id.as_str())
            .or_default()
            .push(descriptor);
    }
    let mut shadowed = by_id
        .into_iter()
        .filter_map(|(rule_id, occurrences)| {
            let (active, shadowed) = occurrences.split_last()?;
            (!shadowed.is_empty()).then(|| RuleShadowedRule {
                rule_id: rule_id.to_owned(),
                active: ref_for_descriptor(active),
                shadowed: shadowed
                    .iter()
                    .map(|descriptor| ref_for_descriptor(descriptor))
                    .collect(),
            })
        })
        .collect::<Vec<_>>();
    shadowed.sort_by(|a, b| a.rule_id.cmp(&b.rule_id));
    shadowed
}

fn ref_for_descriptor(descriptor: &RuleDescriptor) -> RuleDescriptorRef {
    RuleDescriptorRef {
        source: descriptor.source.clone(),
        path: descriptor.path.clone(),
    }
}

#[cfg(test)]
mod tests {
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
        assert_eq!(report.invalid_regexes.len(), 11, "{report:#?}");
        assert!(
            report
                .invalid_regexes
                .iter()
                .any(|error| error.rule_id == "filesystem/find")
        );
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

        assert_eq!(report.fixtures_seen, 4, "{report:#?}");
        assert_eq!(report.passed, 4, "{report:#?}");
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
}
