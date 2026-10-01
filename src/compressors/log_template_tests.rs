use super::*;

#[test]
fn parses_common_timestamp_shapes() {
    assert_eq!(
        parse_timestamp("081109 203615 148 INFO dfs.DataNode: x").as_deref(),
        Some("081109 203615")
    );
    assert_eq!(
        parse_timestamp("2024-01-02T10:11:12.500Z did a thing").as_deref(),
        Some("2024-01-02T10:11:12.500Z")
    );
    assert_eq!(
        parse_timestamp("Dec  fake").as_deref(),
        None,
        "single month token isn't a timestamp"
    );
    assert_eq!(
        parse_timestamp("Nov 10 04:41:03 host sshd: boom").as_deref(),
        Some("Nov 10 04:41:03")
    );
    assert_eq!(
        parse_timestamp("12:34:56.789 request served").as_deref(),
        Some("12:34:56.789")
    );
    assert_eq!(parse_timestamp("no timestamp at all here"), None);
}

#[test]
fn template_key_folds_variable_tail_but_keeps_prefix() {
    let a = "081109 214043 2561 WARN dfs.DataNode: Got exception while serving blk_-2918118818249673980 to /10.251.90.64:";
    let b = "081109 214402 2677 WARN dfs.DataNode: Got exception while serving blk_8376667364205250596 to /10.251.91.159:";
    assert_eq!(
        template_key(a),
        template_key(b),
        "same template must collapse"
    );
}

#[test]
fn template_key_prefix_distinguishes_numeric_codes() {
    // Different status codes before the `:` must not merge.
    let a = template_key("HTTP 500 backend: request handling failed");
    let b = template_key("HTTP 404 backend: request handling failed");
    assert_ne!(a, b, "distinct prefixes must stay distinct");
}

#[test]
fn mine_folds_a_run_into_one_wildcard_template() {
    let lines: Vec<String> = (0..20)
        .map(|i| format!("worker {i} processed batch ok"))
        .collect();
    let refs: Vec<&str> = lines.iter().map(String::as_str).collect();
    let runs = mine(&refs);
    assert_eq!(runs.len(), 1, "one run expected");
    assert_eq!(runs[0].count, 20);
    assert!(runs[0].display().contains("<*>"), "{}", runs[0].display());
}

#[test]
fn mine_rejects_all_wildcard_and_short_runs() {
    // Every column varies → all-wildcard → rejected.
    let lines = ["a b c", "d e f", "g h i", "j k l"];
    assert!(mine(&lines).is_empty());
    // Fewer than MIN_RUN identical-shape lines → rejected.
    let short = ["x 1 y", "x 2 y"];
    assert!(mine(&short).is_empty());
}

#[test]
fn summarize_region_reports_dominant_template() {
    let lines: Vec<String> = (0..30)
        .map(|i| format!("Compiling crate_{i} v0.1.0"))
        .collect();
    let refs: Vec<&str> = lines.iter().map(String::as_str).collect();
    let summary = summarize_region(&refs).expect("dominant template");
    assert_eq!(summary.len(), 1);
    assert!(summary[0].contains("[Template:"), "{}", summary[0]);
    assert!(summary[0].contains("30 lines"), "{}", summary[0]);
}

#[test]
fn summarize_region_declines_when_diverse() {
    // Many distinct shapes → not dominated by a few templates.
    let lines: Vec<String> = (0..30)
        .map(|i| {
            let extra = "x ".repeat(i % 7);
            format!("{extra}line {i} end")
        })
        .collect();
    let refs: Vec<&str> = lines.iter().map(String::as_str).collect();
    assert!(summarize_region(&refs).is_none());
}

#[test]
fn miner_never_inflates_via_region_summary() {
    // The summary lines must be shorter than the region they describe.
    let lines: Vec<String> = (0..40)
        .map(|i| {
            format!(
                "081109 20{:04} INFO dfs.DataNode: block blk_{i} stored ok",
                i
            )
        })
        .collect();
    let refs: Vec<&str> = lines.iter().map(String::as_str).collect();
    let region_len: usize = lines.iter().map(|l| l.len() + 1).sum();
    if let Some(summary) = summarize_region(&refs) {
        let summary_len: usize = summary.iter().map(|l| l.len() + 1).sum();
        assert!(summary_len < region_len, "summary must not inflate");
    }
}
