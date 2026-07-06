//! Rule loading, compilation, and the built-in rule set.

pub mod builtin;
pub mod compiler;
pub mod loader;
pub mod verify;

pub use compiler::compile_rule;
pub use loader::{LoadRuleOptions, load_builtin_rules, load_rules};
pub use verify::{
    RuleDescriptorRef, RuleDuplicateId, RuleParseError, RuleRegexError, RuleShadowedRule,
    RuleVerificationReport, verify_rules,
};
