use super::*;

#[test]
fn formats_and_parses_canonical() {
    let m = format_marker("ab12cd34");
    assert_eq!(m, "⟦tj:ab12cd34⟧");
    assert_eq!(
        parse_markers(&format!("see {m} for more")),
        vec!["ab12cd34"]
    );
}

#[test]
fn parses_legacy_form() {
    let text = "partial; call retrieve_tool_output(\"deadbeef00\") to recover";
    assert_eq!(parse_markers(text), vec!["deadbeef00"]);
}

#[test]
fn parses_multiple_dedup() {
    let text = "⟦tj:aaa⟧ and ⟦tj:bbb⟧ and again ⟦tj:aaa⟧";
    assert_eq!(parse_markers(text), vec!["aaa", "bbb"]);
}

#[test]
fn footer_carries_token() {
    let f = recovery_footer("c0ffee", 1234, true);
    assert!(f.contains("PARTIAL view"));
    assert!(f.contains("c0ffee"));
    assert_eq!(parse_markers(&f), vec!["c0ffee"]);
}

#[test]
fn lossless_footer_round_trips() {
    let f = recovery_footer("deadbeef", 4321, false);
    assert!(f.contains("no data lost"));
    assert_eq!(parse_markers(&f), vec!["deadbeef"]);
}

#[test]
fn prose_token_quotes_are_not_markers() {
    assert!(parse_markers("the auth token \"sk-abc123\" expired").is_empty());
    assert!(parse_markers("set your API token \"foo\" in the env").is_empty());
}

#[test]
fn token_needle_scoped_to_recovery_tools() {
    // Footer-style wording (tool name nearby) still parses…
    for tool in RECOVERY_TOOL_NAMES {
        let text = format!("call {tool} with token \"abc123\" to recover");
        assert_eq!(parse_markers(&text), vec!["abc123"], "tool {tool}");
    }
    // …but a tool name far away (outside the lookback window) does not
    // legitimize an unrelated token-quote.
    let far = format!(
        "{} was mentioned earlier. {} Then the auth token \"sk-999\" leaked.",
        RETRIEVE_TOOL_NAME,
        "filler ".repeat(30)
    );
    assert!(parse_markers(&far).is_empty());
}

#[test]
fn repl_footer_names_the_slice_read_first_and_still_round_trips() {
    for lossy in [true, false] {
        let f = recovery_footer_with("c0ffee", 16_454, lossy, true);
        assert!(f.contains("handle \"c0ffee\""), "{f}");
        let find = f.find("juice_find").expect("names juice_find");
        let retrieve = f.find(RETRIEVE_TOOL_NAME).expect("names the retrieve tool");
        assert!(find < retrieve, "the slice read comes before the whole original: {f}");
        assert!(f.contains("mode \"sed\"") && f.contains("-n 120,200p"));
        assert!(f.contains(if lossy { "PARTIAL view" } else { "no data lost" }));
        assert_eq!(parse_markers(&f), vec!["c0ffee"]);
    }
}

#[test]
fn footer_without_repl_is_unchanged() {
    assert_eq!(
        recovery_footer_with("c0ffee", 1234, true, false),
        recovery_footer("c0ffee", 1234, true)
    );
    assert!(!recovery_footer("c0ffee", 1234, true).contains("juice_find"));
}
