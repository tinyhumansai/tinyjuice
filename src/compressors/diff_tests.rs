use super::*;
use crate::cache::{CcrStore, MemoryCcrStore};
use crate::types::ContentKind;

#[test]
fn keeps_changed_lines_collapses_context() {
    let mut s = String::from("diff --git a/x.rs b/x.rs\n@@ -1,40 +1,41 @@\n");
    for i in 0..20 {
        let _ = writeln!(s, " context line {i} unchanged here");
    }
    let _ = writeln!(s, "-old changed line");
    let _ = writeln!(s, "+new changed line");
    for i in 0..20 {
        let _ = writeln!(s, " more context {i} unchanged");
    }
    let out = compress(&s).expect("compresses").text;
    assert!(out.contains("-old changed line"), "{out}");
    assert!(out.contains("+new changed line"), "{out}");
    assert!(out.contains("context line(s) omitted"), "{out}");
    assert!(out.contains("@@ -1,40 +1,41 @@"));
    assert!(out.len() < s.len());
}

#[test]
fn summarizes_lockfile_hunk() {
    let mut s = String::from("diff --git a/Cargo.lock b/Cargo.lock\n@@ -1,60 +1,80 @@\n");
    for i in 0..40 {
        let _ = writeln!(s, "+ new dep entry {i}");
    }
    for i in 0..20 {
        let _ = writeln!(s, "- old dep entry {i}");
    }
    let out = compress(&s).expect("compresses").text;
    assert!(
        out.contains("diff-noise hunk omitted reason=lockfile"),
        "{out}"
    );
    assert!(!out.contains("new dep entry 7"), "{out}");
    assert!(out.len() < s.len());
}

#[test]
fn empty_noisy_path_patterns_disable_lockfile_omission() {
    let mut s = String::from("diff --git a/Cargo.lock b/Cargo.lock\n@@ -1,60 +1,80 @@\n");
    for i in 0..20 {
        let _ = writeln!(s, " context before {i}");
    }
    for i in 0..20 {
        let _ = writeln!(s, "+ new dep entry {i}");
    }
    for i in 0..20 {
        let _ = writeln!(s, "- old dep entry {i}");
    }
    for i in 0..20 {
        let _ = writeln!(s, " context after {i}");
    }
    let options = DiffNoiseOptions {
        noisy_path_substrings: Vec::new(),
        ..Default::default()
    };

    let out = compress_with_options(&s, &options)
        .expect("compresses")
        .text;

    assert!(!out.contains("diff-noise hunk omitted"), "{out}");
    assert!(out.contains("new dep entry 7"), "{out}");
}

#[test]
fn summarizes_generated_bundle_hunk_with_reason() {
    let mut s = String::from("diff --git a/dist/app.min.js b/dist/app.min.js\n@@ -1,80 +1,80 @@\n");
    for i in 0..40 {
        let _ = writeln!(s, "+ minified chunk payload {i}");
    }
    for i in 0..40 {
        let _ = writeln!(s, "- previous minified payload {i}");
    }

    let out = compress(&s).expect("compresses").text;

    assert!(
        out.contains("diff-noise hunk omitted reason=generated_bundle"),
        "{out}"
    );
    assert!(!out.contains("minified chunk payload 7"), "{out}");
}

#[test]
fn custom_noisy_path_patterns_get_configured_reason() {
    let mut s = String::from(
        "diff --git a/fixtures/snapshot.txt b/fixtures/snapshot.txt\n@@ -1,80 +1,80 @@\n",
    );
    for i in 0..40 {
        let _ = writeln!(s, "+ snapshot chunk payload {i}");
    }
    for i in 0..40 {
        let _ = writeln!(s, "- previous snapshot payload {i}");
    }
    let options = DiffNoiseOptions {
        noisy_path_substrings: vec!["snapshot.txt".to_string()],
        ..Default::default()
    };

    let out = compress_with_options(&s, &options)
        .expect("compresses")
        .text;

    assert!(
        out.contains("diff-noise hunk omitted reason=configured_noisy_path"),
        "{out}"
    );
    assert!(!out.contains("snapshot chunk payload 7"), "{out}");
}

#[test]
fn whitespace_only_hunks_are_preserved_by_default() {
    let mut s = String::from("diff --git a/src/lib.rs b/src/lib.rs\n@@ -1,40 +1,40 @@\n");
    for i in 0..20 {
        let _ = writeln!(s, " context before {i}");
    }
    let _ = writeln!(s, "-fn main(){{println!(\"hi\");}}");
    let _ = writeln!(s, "+fn main() {{ println!(\"hi\"); }}");
    for i in 0..20 {
        let _ = writeln!(s, " context after {i}");
    }
    let out = compress(&s).expect("compresses").text;

    assert!(out.contains("-fn main()"), "{out}");
    assert!(out.contains("+fn main()"), "{out}");
    assert!(!out.contains("reason=whitespace_only"), "{out}");
}

#[test]
fn whitespace_only_hunks_drop_when_enabled() {
    let mut s = String::from("diff --git a/src/lib.rs b/src/lib.rs\n@@ -1,60 +1,60 @@\n");
    for i in 0..50 {
        let _ = writeln!(s, " context before {i}");
    }
    let _ = writeln!(s, "-fn main(){{println!(\"hi\");}}");
    let _ = writeln!(s, "+fn main() {{ println!(\"hi\"); }}");
    for i in 0..50 {
        let _ = writeln!(s, " context after {i}");
    }
    let options = DiffNoiseOptions {
        drop_whitespace_only_hunks: true,
        ..Default::default()
    };
    let out = compress_with_options(&s, &options)
        .expect("compresses")
        .text;

    assert!(
        out.contains("diff-noise hunk omitted reason=whitespace_only"),
        "{out}"
    );
    assert!(!out.contains("-fn main()"), "{out}");
    assert!(!out.contains("+fn main()"), "{out}");
}

#[test]
fn semantic_hunks_survive_whitespace_noise_mode() {
    let mut s = String::from("diff --git a/src/lib.rs b/src/lib.rs\n@@ -1,40 +1,40 @@\n");
    for i in 0..20 {
        let _ = writeln!(s, " context before {i}");
    }
    let _ = writeln!(s, "-fn answer() -> i32 {{ 41 }}");
    let _ = writeln!(s, "+fn answer() -> i32 {{ 42 }}");
    for i in 0..20 {
        let _ = writeln!(s, " context after {i}");
    }
    let options = DiffNoiseOptions {
        drop_whitespace_only_hunks: true,
        ..Default::default()
    };
    let out = compress_with_options(&s, &options)
        .expect("compresses")
        .text;

    assert!(out.contains("41"), "{out}");
    assert!(out.contains("42"), "{out}");
    assert!(!out.contains("diff-noise hunk omitted"), "{out}");
}

#[test]
fn diff_noise_transform_requires_retained_ccr() {
    let mut s = String::from("diff --git a/Cargo.lock b/Cargo.lock\n@@ -1,80 +1,80 @@\n");
    for i in 0..80 {
        let _ = writeln!(s, "+ new dep entry {i}");
    }
    for i in 0..80 {
        let _ = writeln!(s, "- old dep entry {i}");
    }
    let input = PipelineInput {
        content: &s,
        original_content: &s,
        content_kind: ContentKind::Diff,
        original_bytes: s.len(),
    };
    let transform = DiffNoiseTransform::default();

    let rejecting_store = MemoryCcrStore::new(1, 1);
    assert!(transform.apply(&input, &rejecting_store).is_none());

    let store = MemoryCcrStore::default();
    let out = transform.apply(&input, &store).expect("retained offload");

    assert_eq!(out.kind(), CompressorKind::Diff);
    assert!(out.text().contains("reason=lockfile"), "{}", out.text());
    assert_eq!(store.get(out.token()).as_deref(), Some(s.as_str()));
}

#[test]
fn diff_noise_transform_skips_non_diff_input() {
    let input = PipelineInput {
        content: "plain text",
        original_content: "plain text",
        content_kind: ContentKind::PlainText,
        original_bytes: "plain text".len(),
    };
    let transform = DiffNoiseTransform::default();
    let store = MemoryCcrStore::default();

    assert_eq!(transform.estimate_bloat(&input), 0.0);
    assert!(transform.apply(&input, &store).is_none());
}

#[test]
fn non_diff_returns_none() {
    assert!(compress("just some text\nno hunks here").is_none());
}
