use super::*;

#[test]
fn json_rows_estimate_is_redacted() {
    let content = r#"[{"secret":"a"},{"secret":"b"},{"secret":"c"}]"#;
    let estimate = estimate_bloat(content, ContentKind::Json);
    assert!(matches!(
        estimate.reason,
        BloatReason::JsonRows | BloatReason::JsonSaturation
    ));
    assert!(estimate.score > 0);
    assert!(!estimate.reason.as_str().contains("secret"));
}

#[test]
fn json_saturation_detects_duplicate_constant_rows() {
    let mut rows = Vec::new();
    for i in 0..80 {
        rows.push(format!(
            r#"{{"id":{},"status":"ok","region":"us","shape":"same"}}"#,
            i % 4
        ));
    }
    let content = format!("[{}]", rows.join(","));
    let estimate = estimate_bloat(&content, ContentKind::Json);

    assert_eq!(estimate.reason, BloatReason::JsonSaturation);
    assert!(estimate.score >= 60, "{estimate:?}");
}

#[test]
fn diff_context_estimate_detects_context_heavy_payload() {
    let mut diff = String::from("diff --git a/a b/a\n@@ -1,8 +1,8 @@\n");
    for i in 0..80 {
        diff.push_str(&format!(" context line {i}\n"));
    }
    diff.push_str("-old\n+new\n");
    let estimate = estimate_bloat(&diff, ContentKind::Diff);
    assert_eq!(estimate.reason, BloatReason::DiffContext);
    assert!(estimate.score > 80);
}

#[test]
fn diff_noise_estimate_detects_lockfile_hunks() {
    let mut diff = String::from("diff --git a/Cargo.lock b/Cargo.lock\n");
    diff.push_str("@@ -1,20 +1,40 @@\n");
    for i in 0..40 {
        diff.push_str(&format!("+ dep version {i}\n"));
    }
    for i in 0..20 {
        diff.push_str(&format!("- old dep version {i}\n"));
    }

    let estimate = estimate_bloat(&diff, ContentKind::Diff);

    assert_eq!(estimate.reason, BloatReason::DiffNoise);
    assert!(estimate.score > 80, "{estimate:?}");
}

#[test]
fn diff_noise_estimate_detects_generated_bundle_hunks() {
    let mut diff = String::from("diff --git a/dist/app.min.js b/dist/app.min.js\n");
    diff.push_str("@@ -1,40 +1,40 @@\n");
    for i in 0..40 {
        diff.push_str(&format!("+ minified chunk {i}\n"));
    }
    for i in 0..40 {
        diff.push_str(&format!("- previous minified chunk {i}\n"));
    }

    let estimate = estimate_bloat(&diff, ContentKind::Diff);

    assert_eq!(estimate.reason, BloatReason::DiffNoise);
    assert!(estimate.score > 80, "{estimate:?}");
}

#[test]
fn diff_noise_estimate_detects_whitespace_only_hunks() {
    let diff = "\
diff --git a/src/lib.rs b/src/lib.rs
@@ -1,2 +1,2 @@
-fn main(){println!(\"hi\");}
+fn main() { println!(\"hi\"); }
 context
";

    let estimate = estimate_bloat(diff, ContentKind::Diff);

    assert_eq!(estimate.reason, BloatReason::DiffNoise);
    assert!(estimate.score > 0, "{estimate:?}");
}

#[test]
fn log_template_estimate_detects_repeated_variants() {
    let mut log = String::new();
    for i in 0..80 {
        log.push_str(&format!(
            "2026-07-05T12:{:02}:00Z worker-{i} processed item id={}\n",
            i % 60,
            10_000 + i
        ));
    }

    let estimate = estimate_bloat(&log, ContentKind::Log);

    assert_eq!(estimate.reason, BloatReason::LogTemplates);
    assert!(estimate.score > 50, "{estimate:?}");
}

#[test]
fn search_clustering_estimate_detects_single_file_fan_in() {
    let mut output = String::new();
    for i in 0..40 {
        output.push_str(&format!("src/lib.rs:{i}:needle match {i}\n"));
    }
    for i in 0..5 {
        output.push_str(&format!("src/other.rs:{i}:needle match {i}\n"));
    }

    let estimate = estimate_bloat(&output, ContentKind::Search);

    assert_eq!(estimate.reason, BloatReason::SearchClustering);
    assert!(estimate.score >= 80, "{estimate:?}");
}

#[test]
fn plain_unique_text_is_low_signal() {
    let estimate = estimate_bloat("alpha\nbeta\ngamma", ContentKind::PlainText);
    assert_eq!(estimate.reason, BloatReason::LowSignal);
    assert_eq!(estimate.score, 0);
}
