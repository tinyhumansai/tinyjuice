//! CCR retrieval markers.
//!
//! When the router offloads an original to the [`super::store`], it embeds a
//! marker carrying the CCR token so the model knows the content is recoverable
//! and how to fetch it. The canonical marker is `⟦tj:<hash>⟧`; for backward
//! compatibility we also parse the legacy `retrieve_tool_output("<hash>")` form
//! that older histories may still contain.

/// The retrieve tool's name, surfaced in footers and used by the harness to
/// keep the tool's own output from being re-compacted and to always advertise it.
pub const RETRIEVE_TOOL_NAME: &str = "tinyjuice_retrieve";

/// Legacy retrieve tool names (kept as aliases during migration): the
/// pre-rename `tokenjuice_retrieve` and the original `retrieve_tool_output`.
pub const LEGACY_RETRIEVE_TOOL_NAME: &str = "retrieve_tool_output";
pub const LEGACY_TOKENJUICE_RETRIEVE_TOOL_NAME: &str = "tokenjuice_retrieve";

/// All CCR recovery tool names. Each must be (a) always advertised to every
/// agent — any agent that sees a retrieval footer must be able to call the tool
/// — and (b) never re-compacted (their job is to return an original in full).
pub const RECOVERY_TOOL_NAMES: &[&str] = &[
    RETRIEVE_TOOL_NAME,
    LEGACY_TOKENJUICE_RETRIEVE_TOOL_NAME,
    LEGACY_RETRIEVE_TOOL_NAME,
];

/// Tools whose output must never be re-compacted. See [`RECOVERY_TOOL_NAMES`].
pub const NEVER_COMPACT_TOOLS: &[&str] = RECOVERY_TOOL_NAMES;

/// True if `tool_name` is one of the CCR recovery tools.
pub fn is_recovery_tool(tool_name: &str) -> bool {
    RECOVERY_TOOL_NAMES.contains(&tool_name)
}

/// Format the canonical inline marker for a CCR `hash`.
pub fn format_marker(hash: &str) -> String {
    format!("⟦tj:{hash}⟧")
}

/// Build the human-facing recovery footer appended to compacted output.
///
/// `lossy` distinguishes a partial view (data dropped) from a faithful reformat
/// (no data lost, layout changed); both offer exact recovery.
pub fn recovery_footer(hash: &str, original_bytes: usize, lossy: bool) -> String {
    // Fixed-cost overhead on every compacted output, so keep it tight: the
    // hash appears exactly once, in the `token "<hash>"` form parse_markers
    // recognizes (the tool name in front satisfies its proximity guard).
    if lossy {
        format!(
            "\n\n[PARTIAL view — full original ({original_bytes} bytes): call \
             {RETRIEVE_TOOL_NAME} with token \"{hash}\"]"
        )
    } else {
        format!(
            "\n\n[reformatted, no data lost — exact original ({original_bytes} bytes): call \
             {RETRIEVE_TOOL_NAME} with token \"{hash}\"]"
        )
    }
}

/// Extract all CCR tokens referenced in `text`, from both the canonical
/// `⟦tj:<hash>⟧` markers and the legacy `retrieve_tool_output("<hash>")` form.
/// Order-preserving, de-duplicated.
pub fn parse_markers(text: &str) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    let mut push = |h: &str| {
        let h = h.trim();
        if !h.is_empty() && !out.iter().any(|e| e == h) {
            out.push(h.to_string());
        }
    };

    // Canonical: ⟦tj:HASH⟧
    let mut rest = text;
    while let Some(start) = rest.find("⟦tj:") {
        let after = &rest[start + "⟦tj:".len()..];
        if let Some(end) = after.find('⟧') {
            push(&after[..end]);
            rest = &after[end..];
        } else {
            break;
        }
    }

    // Call-shaped and legacy forms: tinyjuice_retrieve("HASH"),
    // tokenjuice_retrieve("HASH"), retrieve_tool_output("HASH"). The bare
    // `token "` form is guarded: it only counts when one of the recovery tool
    // names appears just before it, matching the footer wording
    // `... calling <tool> with token "<hash>"`. Unguarded it would extract
    // from ordinary prose like `the auth token "sk-abc"`.
    for (needle, guarded) in [
        ("tinyjuice_retrieve(\"", false),
        ("retrieve_tool_output(\"", false),
        ("tokenjuice_retrieve(\"", false),
        ("token \"", true),
    ] {
        let mut pos = 0;
        while let Some(start) = text[pos..].find(needle) {
            let abs = pos + start;
            let after = &text[abs + needle.len()..];
            let Some(end) = after.find('"') else {
                break;
            };
            if !guarded || preceded_by_recovery_tool(text, abs) {
                push(&after[..end]);
            }
            pos = abs + needle.len() + end + 1;
        }
    }

    out
}

/// Chars scanned backwards from a `token "` needle looking for a recovery tool
/// name. The footer puts only ` with ` between the tool name and the needle;
/// this window is generous to tolerate wrapping/reflow.
const TOKEN_NEEDLE_LOOKBACK: usize = 96;

/// True if one of [`RECOVERY_TOOL_NAMES`] occurs within the lookback window
/// ending at byte offset `at` in `text`.
fn preceded_by_recovery_tool(text: &str, at: usize) -> bool {
    let mut start = at.saturating_sub(TOKEN_NEEDLE_LOOKBACK);
    while !text.is_char_boundary(start) {
        start += 1;
    }
    let window = &text[start..at];
    RECOVERY_TOOL_NAMES.iter().any(|tool| window.contains(tool))
}

#[cfg(test)]
mod tests {
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
}
