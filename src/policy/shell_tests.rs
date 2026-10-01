use super::*;

fn input(command: &str) -> ToolExecutionInput {
    ToolExecutionInput {
        tool_name: "shell".to_owned(),
        command: Some(command.to_owned()),
        ..Default::default()
    }
}

fn decide(command: &str) -> ShellPolicyDecision {
    apply_shell_compaction_policy(&input(command), ShellCompactionPolicy::AllowSafeInventory)
}

#[test]
fn exact_file_reads_are_skipped() {
    assert_eq!(
        decide("cat src/lib.rs"),
        ShellPolicyDecision::SkipFileContent
    );
    assert_eq!(
        decide("sed -n '1,200p' src/lib.rs"),
        ShellPolicyDecision::SkipFileContent
    );
    assert_eq!(
        decide("jq . package.json"),
        ShellPolicyDecision::SkipFileContent
    );
}

#[test]
fn inventory_pipelines_are_allowed() {
    assert_eq!(
        decide("find . -type f | sort | head -n 20"),
        ShellPolicyDecision::Compact
    );
    assert_eq!(decide("rg --files"), ShellPolicyDecision::Compact);
    assert_eq!(decide("git ls-files"), ShellPolicyDecision::Compact);
    assert_eq!(
        decide("cd crate && rg --files"),
        ShellPolicyDecision::Compact
    );
}

#[test]
fn unsafe_inventory_actions_are_skipped() {
    assert_eq!(
        decide(r"find . -exec cat {} \;"),
        ShellPolicyDecision::SkipUnsafeAction
    );
    assert_eq!(
        decide("fd --exec cat {}"),
        ShellPolicyDecision::SkipUnsafeAction
    );
    assert_eq!(
        decide("find . -type f > files.txt"),
        ShellPolicyDecision::SkipUnsafeAction
    );
}

#[test]
fn mixed_shell_sequences_are_skipped() {
    assert_eq!(
        decide("git status; cat src/lib.rs"),
        ShellPolicyDecision::SkipMixedShell
    );
    assert_eq!(
        decide("cat src/lib.rs && git status"),
        ShellPolicyDecision::SkipMixedShell
    );
}
