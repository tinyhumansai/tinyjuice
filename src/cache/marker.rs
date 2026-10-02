//! CCR retrieval markers.
//!
//! When the router offloads an original to the [`super::store`], it embeds a
//! marker carrying the CCR token so the model knows the content is recoverable
//! and how to fetch it. The canonical marker is `⟦tj:<hash>⟧`; for backward
//! compatibility we also parse the legacy `retrieve_tool_output("<hash>")` form
//! that older histories may still contain.

/// The retrieve tool's name, surfaced in footers and used by the harness to
/// keep the tool's own output from being re-compacted and to always advertise it.
pub const RETRIEVE_TOOL_NAME: &str = "juice_retrieve";

/// Legacy retrieve tool names (kept as aliases during migration): the
/// pre-rename `tokenjuice_retrieve` and the original `retrieve_tool_output`.
pub const LEGACY_RETRIEVE_TOOL_NAME: &str = "retrieve_tool_output";
pub const LEGACY_TOKENJUICE_RETRIEVE_TOOL_NAME: &str = "tokenjuice_retrieve";
/// The name before `juice_retrieve`, kept so replayed histories still parse.
pub const LEGACY_TINYJUICE_RETRIEVE_TOOL_NAME: &str = "tinyjuice_retrieve";

/// All CCR recovery tool names. Each must be (a) always advertised to every
/// agent — any agent that sees a retrieval footer must be able to call the tool
/// — and (b) never re-compacted (their job is to return an original in full).
pub const RECOVERY_TOOL_NAMES: &[&str] = &[
    RETRIEVE_TOOL_NAME,
    LEGACY_TINYJUICE_RETRIEVE_TOOL_NAME,
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
        ("juice_retrieve(\"", false),
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
#[path = "marker_tests.rs"]
mod tests;
