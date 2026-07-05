//! Structural tag protection for the lossy ML text compressor.
//!
//! Clean-room implementation inspired by Headroom's tag-preservation
//! approach (Apache-2.0) — no code copied, only the idea: agent transcripts
//! embed XML-ish structural markers (`<system-reminder>`, `<tool_call>`, …)
//! that downstream code parses, and a learned compressor treats them as
//! ordinary low-salience text and destroys them. Before the text reaches the
//! model, [`protect`] swaps every non-HTML5 XML-style tag for an opaque
//! placeholder (`{{TJ_TAG_0}}`, `{{TJ_TAG_1}}`, …); after compression,
//! [`restore`] splices the originals back. Only the tag delimiters are
//! protected — the text between an open/close pair still gets compressed.
//!
//! HTML5 element tags are left alone (they are prose-level markup, not
//! structural markers), and anything that does not parse as a tag is left
//! verbatim — never "repaired". If the input already contains the placeholder
//! pattern, the prefix is salted (`{{TJ1_TAG_0}}`, `{{TJ2_TAG_0}}`, …) until
//! collision-free. [`all_placeholders_present`] lets the caller detect a
//! model that deleted a placeholder and decline instead of emitting output
//! with lost structural tags.

/// HTML5 element names (lowercase). Tags whose name is in this list are left
/// untouched by [`protect`]; everything else XML-shaped is placeholdered.
const HTML5_ELEMENTS: &[&str] = &[
    "a",
    "abbr",
    "address",
    "area",
    "article",
    "aside",
    "audio",
    "b",
    "base",
    "bdi",
    "bdo",
    "blockquote",
    "body",
    "br",
    "button",
    "canvas",
    "caption",
    "cite",
    "code",
    "col",
    "colgroup",
    "data",
    "datalist",
    "dd",
    "del",
    "details",
    "dfn",
    "dialog",
    "div",
    "dl",
    "dt",
    "em",
    "embed",
    "fieldset",
    "figcaption",
    "figure",
    "footer",
    "form",
    "h1",
    "h2",
    "h3",
    "h4",
    "h5",
    "h6",
    "head",
    "header",
    "hgroup",
    "hr",
    "html",
    "i",
    "iframe",
    "img",
    "input",
    "ins",
    "kbd",
    "label",
    "legend",
    "li",
    "link",
    "main",
    "map",
    "mark",
    "menu",
    "meta",
    "meter",
    "nav",
    "noscript",
    "object",
    "ol",
    "optgroup",
    "option",
    "output",
    "p",
    "picture",
    "pre",
    "progress",
    "q",
    "rp",
    "rt",
    "ruby",
    "s",
    "samp",
    "script",
    "search",
    "section",
    "select",
    "slot",
    "small",
    "source",
    "span",
    "strong",
    "style",
    "sub",
    "summary",
    "sup",
    "table",
    "tbody",
    "td",
    "template",
    "textarea",
    "tfoot",
    "th",
    "thead",
    "time",
    "title",
    "tr",
    "track",
    "u",
    "ul",
    "var",
    "video",
    "wbr",
];

fn is_html5_element(name: &str) -> bool {
    HTML5_ELEMENTS
        .binary_search_by(|e| {
            e.bytes()
                .map(|b| b.to_ascii_lowercase())
                .cmp(name.bytes().map(|b| b.to_ascii_lowercase()))
        })
        .is_ok()
}

/// Try to parse an XML-style tag starting at byte offset `start` (which must
/// point at `<`). Returns `(end_exclusive, name)` when `text[start..end]` is a
/// well-formed open, close, or self-closing tag; `None` otherwise (the caller
/// leaves the `<` verbatim).
fn parse_tag(text: &str, start: usize) -> Option<(usize, &str)> {
    let bytes = text.as_bytes();
    debug_assert_eq!(bytes[start], b'<');
    let mut i = start + 1;
    // Optional close-tag slash.
    if i < bytes.len() && bytes[i] == b'/' {
        i += 1;
    }
    // Tag name: [A-Za-z][A-Za-z0-9._:-]*
    let name_start = i;
    if i >= bytes.len() || !bytes[i].is_ascii_alphabetic() {
        return None;
    }
    while i < bytes.len()
        && (bytes[i].is_ascii_alphanumeric() || matches!(bytes[i], b'.' | b'_' | b':' | b'-'))
    {
        i += 1;
    }
    let name_end = i;
    // Anything after the name up to `>` is attributes/whitespace; a nested `<`
    // or a newline means this is not a tag we recognise — leave it verbatim.
    if i < bytes.len() && !matches!(bytes[i], b'>' | b'/' | b' ' | b'\t') {
        return None;
    }
    while i < bytes.len() {
        match bytes[i] {
            b'>' => return Some((i + 1, &text[name_start..name_end])),
            b'<' | b'\n' | b'\r' => return None,
            _ => i += 1,
        }
    }
    None
}

/// Pick a placeholder prefix (`TJ`, `TJ1`, `TJ2`, …) whose pattern
/// `{{<prefix>_TAG_` does not occur in `text`, so placeholders can never
/// collide with pre-existing content.
fn salt_prefix(text: &str) -> String {
    let mut prefix = String::from("TJ");
    let mut salt = 0u32;
    while text.contains(&format!("{{{{{prefix}_TAG_")) {
        salt += 1;
        prefix = format!("TJ{salt}");
    }
    prefix
}

/// Replace every non-HTML5 XML-style tag in `text` with an opaque placeholder.
///
/// Returns the protected text and the `(placeholder, original)` pairs in
/// order of appearance. Mismatched or unparseable `<…` sequences are left
/// verbatim; HTML5 element tags pass through untouched.
pub(crate) fn protect(text: &str) -> (String, Vec<(String, String)>) {
    let prefix = salt_prefix(text);
    let mut out = String::with_capacity(text.len());
    let mut saved: Vec<(String, String)> = Vec::new();
    let mut rest = text;

    while let Some(lt) = rest.find('<') {
        out.push_str(&rest[..lt]);
        match parse_tag(rest, lt) {
            Some((end, name)) if !is_html5_element(name) => {
                let placeholder = format!("{{{{{prefix}_TAG_{}}}}}", saved.len());
                out.push_str(&placeholder);
                saved.push((placeholder, rest[lt..end].to_string()));
                rest = &rest[end..];
            }
            Some((end, _)) => {
                // HTML5 tag: pass through untouched.
                out.push_str(&rest[lt..end]);
                rest = &rest[end..];
            }
            None => {
                // Not a tag; leave the `<` verbatim and keep scanning.
                out.push('<');
                rest = &rest[lt + 1..];
            }
        }
    }
    out.push_str(rest);
    (out, saved)
}

/// True when every placeholder in `saved` still occurs in `text`. A missing
/// placeholder means the model deleted a structural tag and the caller must
/// decline rather than emit output with lost markers.
pub(crate) fn all_placeholders_present(text: &str, saved: &[(String, String)]) -> bool {
    saved.iter().all(|(ph, _)| text.contains(ph.as_str()))
}

/// Splice the saved originals back over their placeholders.
pub(crate) fn restore(text: &str, saved: &[(String, String)]) -> String {
    let mut out = text.to_string();
    for (placeholder, original) in saved {
        out = out.replace(placeholder.as_str(), original);
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn html5_whitelist_is_sorted_for_binary_search() {
        assert!(HTML5_ELEMENTS.windows(2).all(|w| w[0] < w[1]));
    }

    #[test]
    fn protects_custom_tags_and_restores_byte_identical() {
        let text = "before <system-reminder>note</system-reminder> \
                    <tool_call name=\"x\">body</tool_call> <selfclosing/> after";
        let (protected, saved) = protect(text);
        assert_eq!(saved.len(), 5);
        assert!(!protected.contains("<system-reminder>"));
        assert!(!protected.contains("</tool_call>"));
        assert!(!protected.contains("<selfclosing/>"));
        // Text between tags is not protected.
        assert!(protected.contains("note"));
        assert!(protected.contains("body"));
        assert!(protected.contains("{{TJ_TAG_0}}"));
        assert_eq!(restore(&protected, &saved), text);
    }

    #[test]
    fn html5_tags_untouched() {
        let text = "<div class=\"a\"><p>hi <B>bold</B></p><br/></div>";
        let (protected, saved) = protect(text);
        assert!(saved.is_empty());
        assert_eq!(protected, text);
    }

    #[test]
    fn mixed_html5_and_custom_tags() {
        let text = "<p>para</p> <tool_use id=\"1\">x</tool_use>";
        let (protected, saved) = protect(text);
        assert_eq!(saved.len(), 2);
        assert!(protected.contains("<p>para</p>"));
        assert!(!protected.contains("<tool_use"));
        assert_eq!(restore(&protected, &saved), text);
    }

    #[test]
    fn placeholder_collision_salts_prefix() {
        let text = "already has {{TJ_TAG_0}} literal <custom>x</custom>";
        let (protected, saved) = protect(text);
        assert_eq!(saved.len(), 2);
        assert!(saved[0].0.starts_with("{{TJ1_TAG_"));
        // Pre-existing literal is untouched and round-trip is byte-identical.
        assert!(protected.contains("{{TJ_TAG_0}}"));
        assert_eq!(restore(&protected, &saved), text);
    }

    #[test]
    fn placeholder_collision_salts_until_collision_free() {
        let text = "{{TJ_TAG_ {{TJ1_TAG_ {{TJ2_TAG_ <custom/>";
        let (protected, saved) = protect(text);
        assert_eq!(saved.len(), 1);
        assert_eq!(saved[0].0, "{{TJ3_TAG_0}}");
        assert_eq!(restore(&protected, &saved), text);
    }

    #[test]
    fn mismatched_or_unparseable_tags_left_verbatim() {
        for text in [
            "a < b and a <= b",               // bare comparison operators
            "unterminated <tag never closes", // no `>` before EOF
            "newline splits <tag\n> here",    // newline inside tag
            "nested <ta <g never closes",     // `<` inside tag body, rest unterminated
            "<3 hearts",                      // name must start alphabetic
            "<-flag>",                        // ditto
        ] {
            let (protected, saved) = protect(text);
            assert!(saved.is_empty(), "expected no tags in {text:?}");
            assert_eq!(protected, text);
        }
    }

    #[test]
    fn unterminated_then_valid_tag_still_protected() {
        let text = "start <broken then <real-tag>end";
        let (protected, saved) = protect(text);
        assert_eq!(saved.len(), 1);
        assert_eq!(saved[0].1, "<real-tag>");
        assert!(protected.contains("<broken then "));
        assert_eq!(restore(&protected, &saved), text);
    }

    #[test]
    fn missing_placeholder_detected() {
        let (protected, saved) = protect("<tool_call>a</tool_call> <p>b</p>");
        assert!(all_placeholders_present(&protected, &saved));
        // Simulate the model deleting the second placeholder.
        let mangled = protected.replace(&saved[1].0, "");
        assert!(!all_placeholders_present(&mangled, &saved));
        // And an empty saved set is trivially satisfied.
        assert!(all_placeholders_present("anything", &[]));
    }

    #[test]
    fn empty_and_tagless_inputs() {
        for text in ["", "plain text, no angle brackets"] {
            let (protected, saved) = protect(text);
            assert!(saved.is_empty());
            assert_eq!(protected, text);
        }
    }
}
