use super::*;

fn hint() -> ContentHint {
    ContentHint::default()
}

#[test]
fn explicit_override_wins() {
    let h = ContentHint {
        explicit: Some(ContentKind::Html),
        ..Default::default()
    };
    // Body is JSON but the explicit hint forces Html.
    assert_eq!(
        detect_content_kind(r#"[{"a":1},{"b":2}]"#, &h),
        ContentKind::Html
    );
}

#[test]
fn mime_and_extension_route() {
    let h = ContentHint {
        mime: Some("application/json".into()),
        ..Default::default()
    };
    assert_eq!(
        detect_content_kind("{not even json}", &h),
        ContentKind::Json
    );
    let h = ContentHint {
        extension: Some("RS".into()),
        ..Default::default()
    };
    assert_eq!(detect_content_kind("anything", &h), ContentKind::Code);
}

#[test]
fn tool_prior_routes_and_search_is_absolute() {
    let h = ContentHint::for_tool("grep");
    // Even a diff-looking body stays Search under a grep prior.
    assert_eq!(
        detect_content_kind("diff --git a/x b/x\n@@ -1 +1 @@", &h),
        ContentKind::Search
    );
    let h = ContentHint::for_tool("read_diff");
    assert_eq!(
        detect_content_kind("diff --git a/x b/x\n@@ -1 +1 @@\n+a", &h),
        ContentKind::Diff
    );
}

#[test]
fn detect_search_results() {
    let c = "src/main.rs:42:fn process() {\nsrc/lib.rs:7:pub use foo;\nsrc/x.rs:99:    let y = 1;";
    assert_eq!(detect_content_kind(c, &hint()), ContentKind::Search);
}

#[test]
fn detect_diff_json_log() {
    assert_eq!(
        detect_content_kind("diff --git a/x.rs b/x.rs\n@@ -1,3 +1,4 @@\n+added", &hint()),
        ContentKind::Diff
    );
    assert_eq!(
        detect_content_kind(r#"[{"id":1,"name":"a"},{"id":2,"name":"b"}]"#, &hint()),
        ContentKind::Json
    );
    assert_eq!(
        detect_content_kind(
            "Compiling foo\nwarning: unused\nerror[E0382]: borrow of moved value\nerror: aborting",
            &hint()
        ),
        ContentKind::Log
    );
}

#[test]
fn detect_html() {
    let c = "<!DOCTYPE html>\n<html><head><title>x</title></head><body><p>hi</p></body></html>";
    assert_eq!(detect_content_kind(c, &hint()), ContentKind::Html);
}

#[test]
fn looks_like_html_handles_multibyte_at_byte_cutoff() {
    // Build content longer than the 8192-byte head window with a multi-byte
    // char (4-byte emoji) straddling byte index 8192, so a raw byte slice
    // would panic on the non-char-boundary cut. Detection must not panic.
    let mut content = "a".repeat(8190);
    content.push('🦀'); // 4 bytes spanning indices 8190..8194 — crosses 8192
    content.push_str(&"b".repeat(2000));
    assert!(content.len() > 8192);
    // Plain text with no tags: just assert it returns without panicking.
    assert!(!looks_like_html(&content));
    // And the full detector stays reachable on the same input.
    assert_eq!(
        detect_content_kind(&content, &hint()),
        ContentKind::PlainText
    );
}

#[test]
fn detect_code() {
    let c = "use std::fmt;\n\npub fn add(a: i32, b: i32) -> i32 {\n    let c = a + b;\n    return c;\n}\n\nstruct Foo {\n    x: i32,\n}";
    assert_eq!(detect_content_kind(c, &hint()), ContentKind::Code);
}

#[test]
fn plain_text_passes_through() {
    assert_eq!(
        detect_content_kind("just some prose about a topic at length here", &hint()),
        ContentKind::PlainText
    );
}

#[test]
fn parse_unix_and_windows_paths() {
    assert_eq!(
        parse_search_line("src/main.rs:42:fn process() {"),
        Some(("src/main.rs", 42, "fn process() {"))
    );
    assert_eq!(
        parse_search_line(r"C:\Users\me\a.rs:10:let x = 1;"),
        Some((r"C:\Users\me\a.rs", 10, "let x = 1;"))
    );
    assert_eq!(
        parse_search_line("pre-commit-config.yaml:3:foo"),
        Some(("pre-commit-config.yaml", 3, "foo"))
    );
    assert_eq!(parse_search_line("just a sentence"), None);
}
