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
