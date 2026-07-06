//! Rule loading, compilation, and the built-in rule set.

pub mod builtin;
pub mod compiler;
pub mod loader;
pub mod verify;

pub use compiler::compile_rule;
pub use loader::{LoadRuleOptions, load_builtin_rules, load_rules};
pub use verify::{
    RuleDescriptorRef, RuleDiscoveryFamily, RuleDiscoveryReport, RuleDuplicateId,
    RuleFixtureFailure, RuleFixtureParseError, RuleFixtureVerificationReport, RuleParseError,
    RuleRegexError, RuleShadowedRule, RuleVerificationReport, discover_fallback_outputs,
    verify_rule_fixtures, verify_rules,
};
