use std::path::Path;

use crate::types::ToolExecutionInput;

/// Host policy for shell-command output.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShellCompactionPolicy {
    CompactAll,
    SkipAll,
    SkipFileContent,
    AllowSafeInventory,
}

/// Redacted decision for shell-command output.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShellPolicyDecision {
    Compact,
    SkipAll,
    SkipFileContent,
    SkipMixedShell,
    SkipUnsafeAction,
}

impl ShellPolicyDecision {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Compact => "compact",
            Self::SkipAll => "skip_all",
            Self::SkipFileContent => "skip_file_content",
            Self::SkipMixedShell => "skip_mixed_shell",
            Self::SkipUnsafeAction => "skip_unsafe_action",
        }
    }

    pub fn should_compact(self) -> bool {
        self == Self::Compact
    }
}

/// Apply shell-output compaction policy to a command-bearing tool input.
pub fn apply_shell_compaction_policy(
    input: &ToolExecutionInput,
    policy: ShellCompactionPolicy,
) -> ShellPolicyDecision {
    match policy {
        ShellCompactionPolicy::CompactAll => ShellPolicyDecision::Compact,
        ShellCompactionPolicy::SkipAll => ShellPolicyDecision::SkipAll,
        ShellCompactionPolicy::SkipFileContent => {
            if command_parts(input).is_some_and(|parts| parts.contains_file_content_read()) {
                ShellPolicyDecision::SkipFileContent
            } else {
                ShellPolicyDecision::Compact
            }
        }
        ShellCompactionPolicy::AllowSafeInventory => {
            let Some(parts) = command_parts(input) else {
                return ShellPolicyDecision::Compact;
            };
            if parts.has_mixed_sequence {
                return ShellPolicyDecision::SkipMixedShell;
            }
            if parts.has_unsafe_action() {
                return ShellPolicyDecision::SkipUnsafeAction;
            }
            if parts.contains_file_content_read() {
                return ShellPolicyDecision::SkipFileContent;
            }
            ShellPolicyDecision::Compact
        }
    }
}

/// True when the command is a well-known file-content inspection tool.
pub fn is_file_content_inspection_command(input: &ToolExecutionInput) -> bool {
    command_parts(input).is_some_and(|parts| parts.contains_file_content_read())
}

fn command_parts(input: &ToolExecutionInput) -> Option<CommandParts> {
    if let Some(command) = input.command.as_deref().filter(|c| !c.trim().is_empty()) {
        return parse_command(command);
    }

    let argv = input.argv.as_deref()?.to_vec();
    if argv.is_empty() {
        return None;
    }
    Some(CommandParts {
        has_redirect: argv.iter().any(|token| is_redirect_token(token)),
        segments: vec![argv],
        has_mixed_sequence: false,
    })
}

struct CommandParts {
    segments: Vec<Vec<String>>,
    has_mixed_sequence: bool,
    has_redirect: bool,
}

impl CommandParts {
    fn contains_file_content_read(&self) -> bool {
        self.segments
            .first()
            .is_some_and(|tokens| is_file_content_read(tokens))
    }

    fn has_unsafe_action(&self) -> bool {
        self.has_redirect
            || self
                .segments
                .iter()
                .any(|tokens| contains_unsafe_action(tokens))
    }
}

fn parse_command(command: &str) -> Option<CommandParts> {
    let stripped = strip_leading_cd_prefix(command.trim());
    if stripped.is_empty() {
        return None;
    }

    let has_mixed_sequence = contains_sequence_operator(stripped);
    let has_redirect = contains_unquoted_redirect(stripped);
    let segments: Vec<Vec<String>> = split_unquoted_pipes(stripped)
        .into_iter()
        .map(crate::reduce::tokenize_command)
        .filter(|tokens| !tokens.is_empty())
        .collect();

    if segments.is_empty() {
        None
    } else {
        Some(CommandParts {
            segments,
            has_mixed_sequence,
            has_redirect,
        })
    }
}

fn strip_leading_cd_prefix(mut command: &str) -> &str {
    loop {
        let Some((left, right)) = split_once_unquoted_and(command) else {
            return command.trim();
        };
        let tokens = crate::reduce::tokenize_command(left);
        if tokens.first().map(|s| s.as_str()) != Some("cd") || tokens.len() < 2 {
            return command.trim();
        }
        command = right.trim();
    }
}

fn split_once_unquoted_and(s: &str) -> Option<(&str, &str)> {
    let bytes = s.as_bytes();
    let mut quote: Option<char> = None;
    let mut escaping = false;
    let mut i = 0;
    while i + 1 < bytes.len() {
        let ch = s[i..].chars().next()?;
        if escaping {
            escaping = false;
            i += ch.len_utf8();
            continue;
        }
        if ch == '\\' {
            escaping = true;
            i += ch.len_utf8();
            continue;
        }
        if let Some(q) = quote {
            if ch == q {
                quote = None;
            }
            i += ch.len_utf8();
            continue;
        }
        if ch == '\'' || ch == '"' {
            quote = Some(ch);
            i += ch.len_utf8();
            continue;
        }
        if bytes[i] == b'&' && bytes[i + 1] == b'&' {
            return Some((&s[..i], &s[i + 2..]));
        }
        i += ch.len_utf8();
    }
    None
}

fn contains_sequence_operator(s: &str) -> bool {
    let bytes = s.as_bytes();
    let mut quote: Option<char> = None;
    let mut escaping = false;
    let mut i = 0;
    while i < bytes.len() {
        let ch = s[i..].chars().next().expect("valid char boundary");
        if escaping {
            escaping = false;
            i += ch.len_utf8();
            continue;
        }
        if ch == '\\' {
            escaping = true;
            i += ch.len_utf8();
            continue;
        }
        if let Some(q) = quote {
            if ch == q {
                quote = None;
            }
            i += ch.len_utf8();
            continue;
        }
        if ch == '\'' || ch == '"' {
            quote = Some(ch);
            i += ch.len_utf8();
            continue;
        }
        if ch == ';' || ch == '\n' {
            return true;
        }
        if i + 1 < bytes.len()
            && ((bytes[i] == b'&' && bytes[i + 1] == b'&')
                || (bytes[i] == b'|' && bytes[i + 1] == b'|'))
        {
            return true;
        }
        i += ch.len_utf8();
    }
    false
}

fn contains_unquoted_redirect(s: &str) -> bool {
    let mut quote: Option<char> = None;
    let mut escaping = false;
    for ch in s.chars() {
        if escaping {
            escaping = false;
            continue;
        }
        if ch == '\\' {
            escaping = true;
            continue;
        }
        if let Some(q) = quote {
            if ch == q {
                quote = None;
            }
            continue;
        }
        if ch == '\'' || ch == '"' {
            quote = Some(ch);
            continue;
        }
        if ch == '<' || ch == '>' {
            return true;
        }
    }
    false
}

fn split_unquoted_pipes(s: &str) -> Vec<&str> {
    let bytes = s.as_bytes();
    let mut out = Vec::new();
    let mut quote: Option<char> = None;
    let mut escaping = false;
    let mut start = 0;
    let mut i = 0;
    while i < bytes.len() {
        let ch = s[i..].chars().next().expect("valid char boundary");
        if escaping {
            escaping = false;
            i += ch.len_utf8();
            continue;
        }
        if ch == '\\' {
            escaping = true;
            i += ch.len_utf8();
            continue;
        }
        if let Some(q) = quote {
            if ch == q {
                quote = None;
            }
            i += ch.len_utf8();
            continue;
        }
        if ch == '\'' || ch == '"' {
            quote = Some(ch);
            i += ch.len_utf8();
            continue;
        }
        if ch == '|' {
            if i + 1 < bytes.len() && bytes[i + 1] == b'|' {
                i += 2;
                continue;
            }
            out.push(s[start..i].trim());
            start = i + 1;
        }
        i += ch.len_utf8();
    }
    out.push(s[start..].trim());
    out
}

fn argv0(tokens: &[String]) -> Option<String> {
    tokens.first().map(|cmd| {
        Path::new(cmd)
            .file_name()
            .map(|name| name.to_string_lossy().to_string())
            .unwrap_or_else(|| cmd.to_string())
    })
}

fn is_file_content_read(tokens: &[String]) -> bool {
    let Some(cmd) = argv0(tokens) else {
        return false;
    };
    matches!(
        cmd.as_str(),
        "cat" | "nl" | "bat" | "batcat" | "jq" | "yq" | "head" | "tail" | "sed"
    )
}

fn contains_unsafe_action(tokens: &[String]) -> bool {
    let Some(cmd) = argv0(tokens) else {
        return false;
    };
    if tokens.iter().any(|token| is_redirect_token(token)) {
        return true;
    }
    match cmd.as_str() {
        "find" => tokens.iter().skip(1).any(|t| {
            matches!(
                t.as_str(),
                "-exec" | "-execdir" | "-ok" | "-okdir" | "-delete"
            )
        }),
        "fd" => tokens.iter().skip(1).any(|t| {
            matches!(
                t.as_str(),
                "-x" | "-X" | "--exec" | "--exec-batch" | "--batch-size"
            )
        }),
        "rm" | "mv" | "cp" | "install" | "chmod" | "chown" | "ln" | "mkdir" | "rmdir" => true,
        "git" => tokens.get(1).is_some_and(|sub| {
            matches!(
                sub.as_str(),
                "checkout"
                    | "switch"
                    | "reset"
                    | "clean"
                    | "restore"
                    | "apply"
                    | "am"
                    | "merge"
                    | "rebase"
                    | "commit"
            )
        }),
        _ => false,
    }
}

fn is_redirect_token(token: &str) -> bool {
    token.contains('>') || token.contains('<')
}

#[cfg(test)]
mod tests {
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
}
