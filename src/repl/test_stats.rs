use super::*;
use crate::cache::store::MemoryCcrStore;
use crate::types::ContentKind;

fn json_doc() -> String {
    let items: Vec<String> = (0..60)
        .map(|i| format!(r#"{{"id":{i},"name":"n{i}","secret":"hunter2-{i}"}}"#))
        .collect();
    format!(
        r#"{{"total":60,"items":[{}],"meta":{{"a":1}}}}"#,
        items.join(",")
    )
}

#[test]
fn stats_describe_json_shape_without_leaking_values() {
    let s = stats::describe(&json_doc(), ContentKind::Json, 4.0);
    assert!(s.contains("JSON object"), "{s}");
    assert!(s.contains("3 keys"), "{s}");
    assert!(s.contains("items[60]"), "{s}");
    assert!(s.contains("largest: items"), "{s}");
    assert!(!s.contains("hunter2"), "{s}");
    assert!(crate::tokens::estimate_tokens_with(&s, 4.0) <= 110, "{s}");
}

#[test]
fn stats_describe_array_other_kinds_and_bad_json() {
    let s = stats::describe(r#"[{"id":1,"name":"x"},{"id":2}]"#, ContentKind::Json, 4.0);
    assert!(
        s.contains("2 items of object") && s.contains("item keys: id, name"),
        "{s}"
    );
    let bad = stats::describe("{not json", ContentKind::Json, 4.0);
    assert!(bad.starts_with("json"), "{bad}");
    let log = stats::describe("ok\nERROR x\nwarn y\n", ContentKind::Log, 4.0);
    assert!(log.contains("1 error-like, 1 warn-like"), "{log}");
    let diff = stats::describe(
        "diff --git a b\n@@ -1 +1 @@\n-a\n+b\n",
        ContentKind::Diff,
        4.0,
    );
    assert!(diff.contains("1 files, 1 hunks"), "{diff}");
    let md = stats::describe("# Title\nbody\n## Two\n", ContentKind::PlainText, 4.0);
    assert!(md.contains("2 headings") && !md.contains("Title"), "{md}");
    let secret = stats::describe("# sk-secret-token\nbody\n", ContentKind::PlainText, 4.0);
    assert!(!secret.contains("sk-secret"), "{secret}");
    let long_key = format!(r#"{{"{}":1}}"#, "é".repeat(80));
    assert!(stats::describe(&long_key, ContentKind::Json, 4.0).contains('…'));
}

#[test]
fn head_is_capped_at_500_chars() {
    let text = "é".repeat(2_000);
    assert_eq!(stats::head(&text).chars().count(), stats::HEAD_CHARS);
    assert_eq!(stats::head("short"), "short");
}

#[test]
fn write_handle_file_round_trips_and_is_idempotent() {
    let dir = std::env::temp_dir().join(format!("tj-handle-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    let p = write_handle_file(&dir, "abc123", "hello\nworld").unwrap();
    assert_eq!(std::fs::read_to_string(&p).unwrap(), "hello\nworld");
    assert_eq!(p.extension().unwrap(), "txt");
    assert_eq!(write_handle_file(&dir, "abc123", "ignored").unwrap(), p);
    assert!(write_handle_file(&dir, "../evil", "x").is_none());
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mode = std::fs::metadata(&p).unwrap().permissions().mode();
        assert_eq!(mode & 0o777, 0o600);
    }
    let _ = std::fs::remove_dir_all(&dir);
    assert!(write_handle_file(std::path::Path::new("/proc/nope/x"), "abc", "x").is_none());
}

#[tokio::test]
async fn router_repl_handle_reports_stats_head_and_saved_file() {
    use crate::compress::route_with_store_report;
    use crate::types::{CompressInput, CompressOptions, CompressorKind, ContentHint};
    let store = MemoryCcrStore::default();
    let content = json_doc();
    let dir = std::env::temp_dir().join(format!("tj-router-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    let opts = CompressOptions {
        repl_handle: true,
        repl_save_dir: Some(dir.clone()),
        ..CompressOptions::default()
    };
    let hint = ContentHint::default();
    let input = CompressInput {
        content: &content,
        kind: ContentKind::Json,
        hint: &hint,
        exit_code: None,
        command: None,
        argv: None,
        original_bytes: content.len(),
    };
    let (out, report) = route_with_store_report(input, &opts, &store).await;
    assert_eq!(
        out.compressor,
        CompressorKind::Repl,
        "{:?}",
        report.skip_reason
    );
    let stats = out.stats.expect("stats");
    assert!(out.body.starts_with(&format!("[{stats}]")));
    assert!(out.body.contains(&content[..200]), "head snippet missing");
    let path = out.saved_path.expect("saved path");
    assert!(out.text.contains(path.to_str().unwrap()));
    assert_eq!(std::fs::read_to_string(&path).unwrap(), content);
    let _ = std::fs::remove_dir_all(&dir);
}

#[tokio::test]
async fn rejected_handle_view_leaves_no_saved_file() {
    use crate::compress::route_with_store_report;
    use crate::types::{CompressInput, CompressOptions, ContentHint};
    let store = MemoryCcrStore::default();
    let content = "tiny payload".to_string();
    let dir = std::env::temp_dir().join(format!("tj-reject-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    let opts = CompressOptions {
        repl_handle: true,
        repl_save_dir: Some(dir.clone()),
        ccr_min_tokens: 0,
        ..CompressOptions::default()
    };
    let hint = ContentHint::default();
    let input = CompressInput {
        content: &content,
        kind: ContentKind::PlainText,
        hint: &hint,
        exit_code: None,
        command: None,
        argv: None,
        original_bytes: content.len(),
    };
    let (out, _) = route_with_store_report(input, &opts, &store).await;
    assert!(out.saved_path.is_none());
    let leftover = std::fs::read_dir(&dir).map(|d| d.count()).unwrap_or(0);
    assert_eq!(leftover, 0, "unreferenced copy left behind");
    let _ = std::fs::remove_dir_all(&dir);
}
