use super::*;

#[test]
fn tool_priors_map_known_tools() {
    assert_eq!(tool_prior("grep"), ToolPrior::Search);
    assert_eq!(tool_prior("run_tests"), ToolPrior::Log);
    assert_eq!(tool_prior("read_diff"), ToolPrior::Diff);
    assert_eq!(tool_prior("file_read"), ToolPrior::Auto);
    assert_eq!(tool_prior("shell"), ToolPrior::Auto);
}

#[test]
fn extensions_map() {
    assert_eq!(extension_to_kind("rs"), Some(ContentKind::Code));
    assert_eq!(extension_to_kind("json"), Some(ContentKind::Json));
    assert_eq!(extension_to_kind("html"), Some(ContentKind::Html));
    assert_eq!(extension_to_kind("patch"), Some(ContentKind::Diff));
    assert_eq!(extension_to_kind("xyz"), None);
}

#[test]
fn mimes_map() {
    assert_eq!(mime_to_kind("application/json"), Some(ContentKind::Json));
    assert_eq!(
        mime_to_kind("text/html; charset=utf-8"),
        Some(ContentKind::Html)
    );
    assert_eq!(mime_to_kind("text/x-rust"), Some(ContentKind::Code));
    assert_eq!(mime_to_kind("text/plain"), None);
}
