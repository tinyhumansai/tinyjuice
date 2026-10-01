use super::*;
use crate::cache::{CcrStore, MemoryCcrStore};
use crate::pipeline::PipelineInput;

fn big_results() -> String {
    let mut s = String::from("120 match(es); scanned 3 file(s)\n");
    for i in 0..60 {
        let _ = writeln!(s, "src/a.rs:{i}:let value_{i} = compute_long_name_{i}();");
    }
    for i in 0..60 {
        let _ = writeln!(s, "src/b.rs:{i}:fn helper_function_number_{i}() {{}}");
    }
    s
}

#[test]
fn keeps_top_k_per_file_and_tally() {
    let input = big_results();
    let out = compress(&input, None).expect("compresses").text;
    assert!(out.contains("more match(es) in src/a.rs"), "{out}");
    assert!(out.contains("more match(es) in src/b.rs"));
    assert!(
        out.contains("[search omitted: 110 match(es) not shown across 2 file(s)]"),
        "{out}"
    );
    // Preamble survives.
    assert!(out.contains("120 match(es)"));
    assert!(out.len() < input.len());
}

#[test]
fn query_ranks_relevant_matches() {
    let mut s = String::new();
    for i in 0..50 {
        let body = if i == 7 {
            "the special needle token appears here".to_string()
        } else {
            format!("ordinary line content number {i}")
        };
        let _ = writeln!(s, "src/x.rs:{i}:{body}");
    }
    let out = compress(&s, Some("needle token")).expect("compresses").text;
    assert!(
        out.contains("special needle token"),
        "ranked-in match missing:\n{out}"
    );
}

#[test]
fn query_ranking_tokenizes_regex_identifier_context() {
    let mut s = String::new();
    for i in 0..50 {
        let body = if i == 37 {
            "fn sync_worker_v2() -> Result<()> { run_worker() }".to_string()
        } else {
            format!("ordinary worker helper number {i}")
        };
        let _ = writeln!(s, "src/x.rs:{i}:{body}");
    }

    let out = compress(&s, Some(r"sync_worker_v2\("))
        .expect("compresses")
        .text;

    assert!(
        out.contains("fn sync_worker_v2()"),
        "regex-style query identifier was not ranked in:\n{out}"
    );
}

#[test]
fn search_transform_requires_retained_ccr() {
    let input = big_results();
    let pipeline_input = PipelineInput {
        content: &input,
        original_content: &input,
        content_kind: ContentKind::Search,
        original_bytes: input.len(),
    };
    let transform = SearchTransform::new().with_query("compute_long_name_7");

    let rejecting_store = MemoryCcrStore::new(1, 1);
    assert!(transform.apply(&pipeline_input, &rejecting_store).is_none());

    let store = MemoryCcrStore::default();
    let out = transform
        .apply(&pipeline_input, &store)
        .expect("retained search output");

    assert_eq!(out.kind(), CompressorKind::Search);
    assert!(out.text().contains("compute_long_name_7"), "{}", out.text());
    assert_eq!(store.get(out.token()).as_deref(), Some(input.as_str()));
}

#[test]
fn search_transform_skips_non_search_input() {
    let input = PipelineInput {
        content: "plain text",
        original_content: "plain text",
        content_kind: ContentKind::PlainText,
        original_bytes: "plain text".len(),
    };
    let transform = SearchTransform::new();
    let store = MemoryCcrStore::default();

    assert_eq!(transform.estimate_bloat(&input), 0.0);
    assert!(transform.apply(&input, &store).is_none());
}

#[test]
fn small_result_set_passes_through() {
    let s = "a.rs:1:hit\nb.rs:2:hit\n";
    assert!(compress(s, None).is_none());
}

#[test]
fn ranked_search_exact_symbol_beats_path_and_density() {
    let query = SearchReadQuery {
        literal: Some("payment".to_string()),
        symbols: vec!["settle_invoice".to_string()],
        ..Default::default()
    };
    let symbol = SearchReadCandidate {
        path: "src/billing/worker.rs".into(),
        matched_lines: vec![SearchReadLine::new(42, "fn settle_invoice() -> Result<()>")],
        max_line: 100,
        ..SearchReadCandidate::new("src/billing/worker.rs")
    };
    let path_only = SearchReadCandidate {
        path: "src/payment/readme.md".into(),
        matched_lines: vec![SearchReadLine::new(1, "general overview")],
        max_line: 10,
        ..SearchReadCandidate::new("src/payment/readme.md")
    };
    let density = SearchReadCandidate {
        path: "src/other.rs".into(),
        matched_lines: vec![
            SearchReadLine::new(10, "payment retry"),
            SearchReadLine::new(11, "payment queue"),
        ],
        max_line: 20,
        ..SearchReadCandidate::new("src/other.rs")
    };

    let symbol_score = rank_search_read_candidate(&symbol, &query);
    let path_score = rank_search_read_candidate(&path_only, &query);
    let density_score = rank_search_read_candidate(&density, &query);

    assert!(symbol_score > path_score, "{symbol_score} <= {path_score}");
    assert!(
        path_score > density_score,
        "{path_score} <= {density_score}"
    );
}

#[test]
fn ranked_search_deprioritizes_vendor_and_generated_paths() {
    let query = SearchReadQuery {
        symbols: vec!["hydrateRoot".to_string()],
        ..Default::default()
    };
    let first_party = SearchReadCandidate {
        path: "src/app/root.tsx".into(),
        matched_lines: vec![SearchReadLine::new(8, "export function hydrateRoot() {}")],
        max_line: 20,
        ..SearchReadCandidate::new("src/app/root.tsx")
    };
    let vendor = SearchReadCandidate {
        path: "node_modules/pkg/root.tsx".into(),
        matched_lines: vec![SearchReadLine::new(8, "export function hydrateRoot() {}")],
        max_line: 20,
        vendor: true,
        ..SearchReadCandidate::new("node_modules/pkg/root.tsx")
    };

    assert!(
        rank_search_read_candidate(&first_party, &query)
            > rank_search_read_candidate(&vendor, &query)
    );
}

#[test]
fn ranked_search_can_honor_explicit_vendor_scope() {
    let query = SearchReadQuery {
        symbols: vec!["hydrateRoot".to_string()],
        penalize_vendor: false,
        ..Default::default()
    };
    let first_party = SearchReadCandidate {
        path: "src/app/root.tsx".into(),
        matched_lines: vec![SearchReadLine::new(8, "export function hydrateRoot() {}")],
        max_line: 20,
        ..SearchReadCandidate::new("src/app/root.tsx")
    };
    let vendor = SearchReadCandidate {
        path: "vendor/pkg/root.tsx".into(),
        matched_lines: vec![SearchReadLine::new(8, "export function hydrateRoot() {}")],
        max_line: 20,
        vendor: true,
        ..SearchReadCandidate::new("vendor/pkg/root.tsx")
    };

    assert_eq!(
        rank_search_read_candidate(&first_party, &query),
        rank_search_read_candidate(&vendor, &query)
    );
}

#[test]
fn snippet_windows_merge_and_report_omitted_matches() {
    let selected = select_snippet_windows(&[10, 11, 20, 50], 2, 100, 2);

    assert_eq!(
        selected.windows,
        vec![LineRange::new(8, 13), LineRange::new(18, 22)]
    );
    assert_eq!(selected.omitted_matches, 1);
}
