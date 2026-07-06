//! Host-facing compaction policy helpers.

pub mod shell;

pub use shell::{
    ShellCompactionPolicy, ShellPolicyDecision, apply_shell_compaction_policy,
    is_file_content_inspection_command,
};
