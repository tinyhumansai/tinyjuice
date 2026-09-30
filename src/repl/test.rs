use super::*;
use crate::cache::store::MemoryCcrStore;

fn find(query: &str, mode: FindMode) -> ReplOp {
    ReplOp::Find {
        query: query.into(),
        mode,
        ignore_case: false,
        context: 0,
        top_k: None,
        scope: None,
        unit: ScopeUnit::Lines,
    }
}

fn lim() -> ReplLimits {
    ReplLimits::default()
}

const DOC: &str = "# Title\nintro line\n## Setup\nrun cargo build\nERROR: boom at 12\nsee [docs](https://example.com/a) and [docs](https://example.com/a)\n```\n# not a heading\n```\n### Deep\nERROR: bang at 99\n";

#[test]
fn grep_literal_and_context() {
    let (hits, t) = ops::grep(DOC, "error:", true, true, 1, &lim()).unwrap();
    assert_eq!(t, 0);
    let lines: Vec<_> = hits.iter().map(|h| h.line).collect();
    assert_eq!(lines, vec![4, 5, 6, 10, 11]);
}

#[test]
fn grep_caps_hits_and_reports_truncation() {
    let l = ReplLimits {
        max_hits: 1,
        ..lim()
    };
    let (hits, t) = ops::grep(DOC, "ERROR", true, false, 0, &l).unwrap();
    assert_eq!((hits.len(), t), (1, 1));
}

#[test]
fn invalid_regex_is_error_not_panic() {
    assert!(matches!(
        ops::grep("a", "(", false, false, 0, &lim()),
        Err(ReplError::InvalidPattern(_))
    ));
    assert!(matches!(
        ops::regex_matches("a", "", &lim()),
        Err(ReplError::EmptyQuery)
    ));
}

#[test]
fn regex_returns_captures() {
    let (m, _) = ops::regex_matches(DOC, r"ERROR: (\w+) at (\d+)", &lim()).unwrap();
    assert_eq!(m.len(), 2);
    assert_eq!(m[1].captures, vec![Some("bang".into()), Some("99".into())]);
}

#[test]
fn headings_skip_fenced_code() {
    let (h, _) = ops::extract_headings(DOC, &lim());
    let texts: Vec<_> = h.iter().map(|h| (h.level, h.text.as_str())).collect();
    assert_eq!(texts, vec![(1, "Title"), (2, "Setup"), (3, "Deep")]);
}

#[test]
fn links_dedupe() {
    let (l, _) = ops::extract_links(DOC, &lim());
    assert_eq!(l.len(), 1);
    assert_eq!(l[0].href, "https://example.com/a");
}

#[test]
fn html_headings_and_links() {
    let html = "<html><body><h2>Intro</h2><p>see <a href=\"https://example.com/x\">X</a></p><a href=\"https://example.com/y\">unclosed</body></html>";
    let (h, _) = ops::extract_headings(html, &lim());
    assert_eq!(h[0].text, "Intro");
    let (l, _) = ops::extract_links(html, &lim());
    assert!(l.iter().any(|l| l.href == "https://example.com/x"));
}

#[test]
fn sensitive_href_params_are_redacted() {
    let html =
        "<html><body><a href=\"https://example.com/p?token=SECRET123&q=1\">p</a></body></html>";
    let (l, _) = ops::extract_links(html, &lim());
    assert!(l.iter().all(|l| !l.href.contains("SECRET123")));
}

#[test]
fn search_ranks_relevant_window() {
    let mut text = String::new();
    for i in 0..30 {
        text.push_str(&format!("filler line {i}\n"));
    }
    text.push_str("the quokka configuration lives here\n");
    let (hits, _) = ops::search(&text, "quokka", 3, &lim()).unwrap();
    assert_eq!(hits.len(), 1);
    assert!(hits[0].text.contains("quokka"));
}

#[test]
fn unicode_and_long_lines_are_clipped() {
    let text = "é".repeat(1000);
    let (hits, _) = ops::grep(&text, "é", true, false, 0, &lim()).unwrap();
    assert!(hits[0].text.chars().count() <= lim().max_line_chars + 1);
}

#[test]
fn empty_input_is_fine() {
    assert!(
        ops::grep("", "x", true, false, 0, &lim())
            .unwrap()
            .0
            .is_empty()
    );
    assert!(ops::summarize("", 500, &lim()).starts_with("0 lines"));
}

#[test]
fn summarize_respects_budget() {
    let text = (0..500)
        .map(|i| format!("line number {i}\n"))
        .collect::<String>();
    let s = ops::summarize(&text, 300, &lim());
    assert!(s.chars().count() < 400);
    assert!(s.contains("line number 0") && s.contains("line number 499"));
}

#[test]
fn run_op_through_store_and_missing_handle() {
    let store = MemoryCcrStore::default();
    let put = store.put(DOC);
    let out = run_op(&store, put.token(), &find("deep", FindMode::Text), &lim()).unwrap();
    assert!(matches!(out, ReplOutput::Lines { ref hits, .. } if hits.len() == 1));
    let sl = run_op(&store, put.token(), &find("-n 1,2p", FindMode::Sed), &lim()).unwrap();
    assert!(
        matches!(sl, ReplOutput::Lines { ref hits, .. } if hits.len() == 2 && hits[0].text == "# Title")
    );
    assert_eq!(
        run_op(
            &store,
            "nope",
            &ReplOp::Extract {
                what: ExtractKind::Links,
                scope: None,
                unit: ScopeUnit::Lines
            },
            &lim()
        ),
        Err(ReplError::HandleNotFound)
    );
}

#[test]
fn output_cap_drops_hits() {
    let text = "match me please\n".repeat(200);
    let l = ReplLimits {
        max_hits: 200,
        max_output_chars: 300,
        ..lim()
    };
    let out = run_on_text(&text, &find("match", FindMode::Text), &l).unwrap();
    match out {
        ReplOutput::Lines { hits, truncated } => assert!(hits.len() < 200 && truncated > 0),
        _ => panic!(),
    }
}

#[cfg(feature = "tinytools")]
mod tools {
    use super::*;
    use std::sync::Arc;
    use tinytools::Tool;

    fn toolset() -> (Vec<Box<dyn Tool>>, String) {
        let store = Arc::new(MemoryCcrStore::default());
        let token = store.put(DOC).token().to_string();
        (super::super::tools::repl_tools(store, lim()), token)
    }

    #[tokio::test]
    async fn every_tool_and_mode_runs_through_the_trait() {
        let (tools, h) = toolset();
        assert_eq!(tools.len(), 3);
        let call = |name: &str, mut a: serde_json::Value| {
            let tool = tools.iter().find(|t| t.name() == name).unwrap();
            a["handle"] = h.clone().into();
            async move { tool.execute(a).await.unwrap() }
        };
        for (mode, query) in [
            ("text", "deep"),
            ("grep", "ERROR"),
            ("regex", "at (\\d+)"),
            ("rank", "build"),
            ("sed", "-n 1,2p"),
            ("awk", "/ERROR/ { print $2 }"),
        ] {
            let r = call(
                "juice_find",
                serde_json::json!({"query": query, "mode": mode}),
            )
            .await;
            assert!(!r.is_error, "{mode}: {r:?}");
        }
        assert!(
            !call("juice_extract", serde_json::json!({"what": "links"}))
                .await
                .is_error
        );
        assert!(
            !call("juice_extract", serde_json::json!({"what": "headings"}))
                .await
                .is_error
        );
        assert!(
            !call("juice_summarize", serde_json::json!({}))
                .await
                .is_error
        );
    }

    #[tokio::test]
    async fn bad_handle_and_bad_args_are_errors() {
        let (tools, _) = toolset();
        let find = tools.iter().find(|t| t.name() == "juice_find").unwrap();
        assert!(
            find.execute(serde_json::json!({"handle":"nope","query":"x"}))
                .await
                .unwrap()
                .is_error
        );
        assert!(
            find.execute(serde_json::json!({"query":"x"}))
                .await
                .unwrap()
                .is_error
        );
        assert!(
            find.execute(serde_json::json!({"handle":"h"}))
                .await
                .unwrap()
                .is_error
        );
        assert!(
            find.execute(serde_json::json!({"handle":"h","query":"x","mode":"bogus"}))
                .await
                .unwrap()
                .is_error
        );
    }
}

#[tokio::test]
async fn router_repl_handle_mode_returns_preview_and_usable_handle() {
    use crate::compress::route_with_store_report;
    use crate::types::{CompressInput, CompressOptions, CompressorKind, ContentHint, ContentKind};
    let store = MemoryCcrStore::default();
    let body: String = (0..800)
        .map(|i| format!("row {i}: value {}\n", i * 7))
        .collect();
    let content = format!("# Report\n{body}ERROR: needle at end\n");
    let opts = CompressOptions {
        repl_handle: true,
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
    let (out, report) = route_with_store_report(input, &opts, &store).await;
    assert_eq!(
        out.compressor,
        CompressorKind::Repl,
        "skip: {:?}",
        report.skip_reason
    );
    assert!(out.text.len() < content.len() / 4);
    let token = out.ccr_token.expect("handle");
    assert!(out.text.contains(&token));
    let hit = run_op(&store, &token, &find("needle", FindMode::Text), &lim()).unwrap();
    assert!(matches!(hit, ReplOutput::Lines { ref hits, .. } if hits.len() == 1));
}

#[test]
fn json_summary_shows_shape_not_one_giant_line() {
    let items: Vec<String> = (0..50)
        .map(|i| format!(r#"{{"id":{i},"name":"n{i}","tags":["a","b"]}}"#))
        .collect();
    let text = format!(r#"{{"total":50,"items":[{}]}}"#, items.join(","));
    let s = ops::summarize(&text, 500, &lim());
    assert!(
        s.contains("items: [50×{id: num, name: str, tags: [2×str]}]"),
        "{s}"
    );
    assert!(s.len() < 300);
}

#[test]
fn sed_selects_ranges_patterns_and_substitutes() {
    let text = "a1\nb2\nc3\nd4\ne5\n";
    let run = |q: &str| run_on_text(text, &find(q, FindMode::Sed), &lim()).unwrap();
    let lines = |o: ReplOutput| match o {
        ReplOutput::Lines { hits, .. } => hits.into_iter().map(|h| h.text).collect::<Vec<_>>(),
        _ => panic!(),
    };
    assert_eq!(lines(run("-n 2,3p")), ["b2", "c3"]);
    assert_eq!(lines(run("-n /c/,$p")), ["c3", "d4", "e5"]);
    assert_eq!(lines(run("2d")), ["a1", "c3", "d4", "e5"]);
    assert_eq!(
        lines(run("-n s/([a-z])(\\d)/\\2\\1/p")),
        ["1a", "2b", "3c", "4d", "5e"]
    );
    assert_eq!(lines(run("-n 2!p")).len(), 4);
    assert!(run_on_text(text, &find("w out.txt", FindMode::Sed), &lim()).is_err());
    assert!(run_on_text(text, &find("s/(/x/", FindMode::Sed), &lim()).is_err());
}

#[test]
fn awk_filters_and_projects_fields() {
    let text = "alice 30 nyc\nbob 25 sf\ncarol 41 nyc\n";
    let run = |q: &str| match run_on_text(text, &find(q, FindMode::Awk), &lim()).unwrap() {
        ReplOutput::Lines { hits, .. } => hits.into_iter().map(|h| h.text).collect::<Vec<_>>(),
        _ => panic!(),
    };
    assert_eq!(run("$2 > 28 { print $1, $NF }"), ["alice nyc", "carol nyc"]);
    assert_eq!(run("/bob/ { print $2 }"), ["25"]);
    assert_eq!(run("NR == 2"), ["bob 25 sf"]);
    assert_eq!(run("$3 ~ /nyc/ { print NR }"), ["1", "3"]);
    assert_eq!(run("-F, { print $1 }").len(), 3);
    assert!(run_on_text(text, &find("{ system(\"ls\") }", FindMode::Awk), &lim()).is_err());
}

#[cfg(feature = "jq")]
#[test]
fn jq_queries_json_and_refuses_environment() {
    let json =
        r#"{"items":[{"name":"a","size":5},{"name":"b","size":50},{"name":"c","size":500}]}"#;
    let out = run_on_text(
        json,
        &find(".items[] | select(.size > 10) | .name", FindMode::Jq),
        &lim(),
    )
    .unwrap();
    assert!(matches!(out, ReplOutput::Values { ref values, .. } if values == &["\"b\"", "\"c\""]));
    let l = ReplLimits {
        max_hits: 1,
        ..lim()
    };
    let out = run_on_text(json, &find(".items[]", FindMode::Jq), &l).unwrap();
    assert!(matches!(out, ReplOutput::Values { ref values, truncated: 2 } if values.len() == 1));
    for bad in [
        "env",
        "$ENV.HOME",
        "input",
        "debug",
        ".items[",
        ".nope | bogus_fn",
    ] {
        assert!(
            run_on_text(json, &find(bad, FindMode::Jq), &lim()).is_err(),
            "{bad}"
        );
    }
    assert!(run_on_text("not json", &find(".", FindMode::Jq), &lim()).is_err());
}

#[cfg(feature = "jq")]
#[test]
fn jq_deadline_stops_a_runaway_query() {
    let start = std::time::Instant::now();
    let r = run_on_text(
        "[1]",
        &find("[range(100000000000)] | length", FindMode::Jq),
        &lim(),
    );
    assert!(r.is_err());
    assert!(start.elapsed() < std::time::Duration::from_secs(5));
}

#[test]
fn scope_follows_python_slice_semantics() {
    let r = |spec: &str, n| scope::resolve(spec, n).unwrap();
    assert_eq!(r("[:]", 10), (0, 10));
    assert_eq!(r("[2:5]", 10), (2, 5));
    assert_eq!(r("[:-3]", 10), (0, 7));
    assert_eq!(r("[-3:]", 10), (7, 10));
    assert_eq!(r("[5:2]", 10), (5, 5));
    assert_eq!(r("[0:99]", 10), (0, 10));
    assert_eq!(r("[-99:]", 10), (0, 10));
    assert!(scope::resolve("[1:2:3]", 10).is_err());
    assert!(scope::resolve("[a:2]", 10).is_err());
    assert!(scope::resolve("5", 10).is_err());
}

#[test]
fn scoped_find_reports_original_line_numbers() {
    let text = "x\nneedle a\ny\nz\nneedle b\nw\n";
    let scoped = |spec: &str, unit| ReplOp::Find {
        query: "needle".into(),
        mode: FindMode::Text,
        ignore_case: false,
        context: 0,
        top_k: None,
        scope: Some(spec.into()),
        unit,
    };
    let lines = |op: ReplOp| match run_on_text(text, &op, &lim()).unwrap() {
        ReplOutput::Lines { hits, .. } => hits.into_iter().map(|h| h.line).collect::<Vec<_>>(),
        _ => panic!(),
    };
    assert_eq!(lines(scoped("[:]", ScopeUnit::Lines)), [2, 5]);
    assert_eq!(lines(scoped("[3:]", ScopeUnit::Lines)), [5]);
    assert_eq!(lines(scoped("[:-2]", ScopeUnit::Lines)), [2]);
    // Characters: drop the last 8 chars ("eedle b\nw\n" region), keeping the first hit.
    assert_eq!(lines(scoped("[:-12]", ScopeUnit::Chars)), [2]);
    assert_eq!(lines(scoped("[12:]", ScopeUnit::Chars)), [5]);
    assert!(lines(scoped("[5:5]", ScopeUnit::Lines)).is_empty());
}

#[test]
fn summarize_hint_spends_the_budget_on_relevant_lines() {
    let mut text = String::new();
    for i in 0..200 {
        text.push_str(&format!("routine heartbeat line {i}\n"));
    }
    text.push_str("the rollback flag ROLLBACK_SAFE_MODE must be set\n");
    for i in 0..200 {
        text.push_str(&format!("more routine heartbeat {i}\n"));
    }
    let with = |hint: Option<&str>| match run_on_text(
        &text,
        &ReplOp::Summarize {
            max_chars: Some(600),
            hint: hint.map(Into::into),
            scope: None,
            unit: ScopeUnit::Lines,
        },
        &lim(),
    )
    .unwrap()
    {
        ReplOutput::Text { text } => text,
        _ => panic!(),
    };
    assert!(with(Some("rollback flag")).contains("ROLLBACK_SAFE_MODE"));
    assert!(!with(None).contains("ROLLBACK_SAFE_MODE"));
    assert!(with(Some("zzzz-no-match")).contains("nothing matched"));
}

#[test]
fn grep_context_max_does_not_overflow() {
    let (hits, _) = ops::grep(DOC, "boom", true, true, usize::MAX, &lim()).unwrap();
    assert!(hits.iter().any(|h| h.line == 5));
}

#[test]
fn awk_rejects_negation_before_non_regex_tests() {
    let text = "a 1\nb 2\n";
    for q in ["!$1 ~ /a/ { print }", "!NR == 2 { print }"] {
        assert!(
            run_on_text(text, &find(q, FindMode::Awk), &lim()).is_err(),
            "{q}"
        );
    }
}

#[test]
fn sed_rejects_exploding_substitution() {
    let text = "x".repeat(1024);
    let script = "s/./&&&&&&&&/g; s/./&&&&&&&&/g; s/./&&&&&&&&/g; s/./&&&&&&&&/g";
    assert!(run_on_text(&text, &find(script, FindMode::Sed), &lim()).is_err());
}

#[cfg(feature = "jq")]
#[test]
fn jq_clips_a_single_large_value() {
    let big = format!("\"{}\"", "y".repeat(100_000));
    let r = run_on_text(&big, &find(".", FindMode::Jq), &lim()).unwrap();
    let s = serde_json::to_string(&r).unwrap();
    assert!(s.len() < 50_000, "{}", s.len());
}

#[test]
fn handle_view_body_never_exceeds_the_preview_budget() {
    let doc: String = (0..40)
        .map(|i| format!("## {} {}\nbody\n", i, "h".repeat(200)))
        .collect();
    for budget in [0usize, 100, 1200] {
        let (body, _) = handle_view(&doc, "tok", budget);
        assert!(
            body.chars().count() <= budget,
            "{budget}: {}",
            body.chars().count()
        );
    }
}

#[test]
fn a_single_oversized_link_is_clipped() {
    let out = cap(
        ReplOutput::Links {
            links: vec![Link {
                text: "t".into(),
                href: "h".repeat(50_000),
            }],
            truncated: 0,
        },
        &lim(),
    );
    assert!(serde_json::to_string(&out).unwrap().len() < 20_000);
}

#[cfg(feature = "jq")]
#[test]
fn jq_allows_blocked_names_used_as_data() {
    let json = r#"{"input":1,"env":2,"items":[{"type":"debug","n":3}]}"#;
    for q in [
        ".input",
        ".env",
        r#".items[] | select(.type == "debug") | .n"#,
        "{env: .env}",
    ] {
        assert!(
            run_on_text(json, &find(q, FindMode::Jq), &lim()).is_ok(),
            "{q}"
        );
    }
    for q in ["env", "$ENV.HOME", r#""\(env)""#, ".items | input"] {
        assert!(
            run_on_text(json, &find(q, FindMode::Jq), &lim()).is_err(),
            "{q}"
        );
    }
}

#[test]
fn summarize_honors_the_requested_character_limit() {
    let op = |max: usize| ReplOp::Summarize {
        max_chars: Some(max),
        hint: None,
        scope: None,
        unit: ScopeUnit::Lines,
    };
    let out = run_on_text(DOC, &op(0), &lim()).unwrap();
    assert!(matches!(out, ReplOutput::Text { ref text } if text.is_empty()));
    let out = run_on_text(DOC, &op(40), &lim()).unwrap();
    assert!(matches!(out, ReplOutput::Text { ref text } if text.chars().count() <= 40));
}

#[test]
fn a_match_with_many_captures_is_bounded() {
    let out = cap(
        ReplOutput::Matches {
            matches: vec![RegexMatch {
                line: 1,
                text: "x".into(),
                captures: (0..2000).map(|_| Some("c".repeat(100))).collect(),
            }],
            truncated: 0,
        },
        &lim(),
    );
    assert!(serde_json::to_string(&out).unwrap().chars().count() <= lim().max_output_chars);
}

#[test]
fn a_single_line_hit_is_clipped_to_a_small_output_cap() {
    let small = ReplLimits {
        max_output_chars: 100,
        ..lim()
    };
    let out = cap(
        ReplOutput::Lines {
            hits: vec![Hit {
                line: 1,
                text: "z".repeat(240),
            }],
            truncated: 0,
        },
        &small,
    );
    assert!(serde_json::to_string(&out).unwrap().chars().count() <= 100);
}
