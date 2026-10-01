use super::*;
use crate::cache::{CcrStore, MemoryCcrStore};
use crate::pipeline::PipelineInput;

#[test]
fn strips_tags_and_scripts() {
    let html = "<html><head><style>.a{color:red}</style></head><body>\
            <script>alert('x')</script><h1>Title</h1><p>Hello <b>world</b>.</p></body></html>";
    let text = html_to_text(html);
    assert!(text.contains("Title"));
    assert!(text.contains("Hello"));
    assert!(text.contains("world"));
    assert!(
        !text.contains("alert"),
        "script body must be dropped: {text}"
    );
    assert!(!text.contains("color:red"), "style body must be dropped");
}

#[test]
fn stray_lt_in_script_body_does_not_swallow_document() {
    // `a<b` inside the script must not be parsed as a tag whose end is
    // the `>` of `</script>` — that would leave skip mode armed forever
    // and drop everything after the script.
    let html = "<body><script>if(a<b){run()}</script><h1>Title</h1><p>Body text.</p></body>";
    let text = html_to_text(html);
    assert!(text.contains("Title"), "content after script lost: {text}");
    assert!(text.contains("Body text."), "{text}");
    assert!(!text.contains("run()"), "script body leaked: {text}");
}

#[test]
fn stray_lt_in_style_and_uppercase_close_tag() {
    let html = "<style>a{width:calc(1<2?1px:2px)}</style><p>kept</p>\
            <SCRIPT>x<y</SCRIPT><p>also kept</p>";
    let text = html_to_text(html);
    assert!(text.contains("kept"), "{text}");
    assert!(text.contains("also kept"), "{text}");
    assert!(!text.contains("calc"), "{text}");
}

#[test]
fn non_matching_close_tag_inside_script_stays_dropped() {
    let html = "<script>document.write('</b>')</script><p>after</p>";
    let text = html_to_text(html);
    assert!(text.contains("after"), "{text}");
    assert!(!text.contains("document.write"), "{text}");
}

#[test]
fn decodes_entities() {
    let text = html_to_text("<p>a &amp; b &lt; c &gt; d &nbsp;e</p>");
    assert!(text.contains("a & b < c > d"), "{text}");
}

#[test]
fn decodes_numeric_entities() {
    let text = html_to_text("<p>em&#8212;dash it&#x27;s &#169;</p>");
    assert!(text.contains("em—dash"), "{text}");
    assert!(text.contains("it's"), "{text}");
    assert!(text.contains("©"), "{text}");
    // Malformed references pass through as text rather than panicking.
    let text = html_to_text("<p>&#xZZ; &#; &#999999999;</p>");
    assert!(text.contains("&#xZZ;"), "{text}");
}

#[test]
fn title_survives_head() {
    let html = "<html><head><title>Deploy Status — prod</title>\
            <meta charset=\"utf-8\"><style>.x{}</style></head>\
            <body><p>body text</p></body></html>";
    let text = html_to_text(html);
    assert!(text.contains("Deploy Status"), "title dropped: {text}");
    assert!(text.contains("body text"), "{text}");
    assert!(!text.contains(".x{}"), "style leaked: {text}");
}

#[test]
fn block_tags_insert_newlines() {
    let text = html_to_text("<p>one</p><p>two</p><li>three</li>");
    let lines: Vec<&str> = text.lines().filter(|l| !l.trim().is_empty()).collect();
    assert!(lines.len() >= 3, "expected separate lines, got {lines:?}");
}

#[test]
fn gt_inside_quoted_attribute_does_not_split_tag() {
    // Discourse ships `<link media="(width >= 40rem)" ...>`; the `>` in
    // `>=` must not terminate the tag and leak the tail as text.
    let html = "<head><link href=\"a.css\" media=\"(width >= 40rem)\" \
            rel=\"stylesheet\" data-target=\"desktop\" /></head><body><p>real text</p></body>";
    let text = html_to_text(html);
    assert!(text.contains("real text"), "{text}");
    assert!(!text.contains("40rem"), "attribute leaked: {text}");
    assert!(!text.contains("stylesheet"), "attribute leaked: {text}");
    // `<` inside quotes must also be harmless.
    let text = html_to_text("<link media=\"(width < 40rem)\" /><p>kept</p>");
    assert!(text.contains("kept"), "{text}");
    assert!(!text.contains("40rem"), "{text}");
}

#[test]
fn unterminated_quote_falls_back_and_terminates() {
    // A quote that never closes must not hang or swallow the document:
    // fall back to the next `>` and keep going.
    let text = html_to_text("<a href=\"broken>after</a> tail");
    assert!(text.contains("after"), "{text}");
    assert!(text.contains("tail"), "{text}");
    // Unterminated quote with no `>` at all: stop cleanly.
    let text = html_to_text("before<a href=\"never closed");
    assert!(text.contains("before"), "{text}");
}

#[test]
fn cdata_delimiters_are_stripped_payload_kept() {
    let html = "<item><title><![CDATA[Big <b>payout</b> story]]></title>\
            <pubDate>Sun, 05 Jul 2026</pubDate></item>";
    let text = html_to_text(html);
    assert!(!text.contains("CDATA"), "{text}");
    assert!(!text.contains("]]>"), "CDATA closer leaked: {text}");
    assert!(text.contains("Big"), "{text}");
    assert!(text.contains("payout"), "{text}");
    assert!(
        !text.contains("<b>"),
        "markup in CDATA not stripped: {text}"
    );
    assert!(text.contains("Sun, 05 Jul 2026"), "{text}");
    // Unterminated CDATA: payload still emitted, no hang.
    let text = html_to_text("<title><![CDATA[open ended");
    assert!(text.contains("open ended"), "{text}");
}

#[test]
fn rss_sibling_elements_are_separated() {
    let html = "<item><guid>https://news.ycombinator.com/item?id=48793726</guid>\
            <comments>https://news.ycombinator.com/item?id=48793726</comments>\
            <dc:creator>alice</dc:creator></item>";
    let text = html_to_text(html);
    assert!(
        !text.contains("48793726https"),
        "sibling values ran together: {text}"
    );
    assert!(!text.contains("48793726alice"), "{text}");
    // Inline tags still don't split words.
    let text = html_to_text("<p>Hello <b>world</b>.</p>");
    assert!(text.contains("Hello world."), "{text}");
}

#[test]
fn compress_shrinks_real_doc() {
    let mut html = String::from("<html><body>");
    for i in 0..50 {
        html.push_str(&format!(
            "<div class=\"row item-{i}\"><span>cell {i}</span></div>"
        ));
    }
    html.push_str("</body></html>");
    let out = compress(&html).expect("compresses");
    // Extraction is an information-preserving reshape, not a drop: every
    // cell's text survives, so it reports as a faithful reformat and ships
    // without needing CCR recovery.
    assert!(!out.lossy, "html extraction is a faithful reshape");
    assert!(out.text.len() < html.len());
    assert!(out.text.contains("cell 7"));
    for i in 0..50 {
        assert!(out.text.contains(&format!("cell {i}")), "cell {i} kept");
    }
}

// --- html_to_markdown -------------------------------------------------

#[test]
fn markdown_drops_script_and_style_bodies_entirely() {
    let html = r#"<html><head><style>body{color:red}</style></head>
            <body><script>var x = 1 < 2 && 3 > 2;</script><p>Hello</p></body></html>"#;
    let md = html_to_markdown(html);
    assert_eq!(md, "Hello");
    assert!(!md.contains("color"), "style body leaked: {md}");
    assert!(!md.contains("var x"), "script body leaked: {md}");
}

#[test]
fn markdown_keeps_headings_at_their_level() {
    let html = "<h1>Title</h1><p>Intro.</p><h3>Detail</h3><p>Body.</p>";
    assert_eq!(html_to_markdown(html), "# Title\nIntro.\n### Detail\nBody.");
}

#[test]
fn markdown_keeps_the_document_title_as_a_top_level_heading() {
    let html = "<html><head><title>Rust Docs</title></head><body><p>x</p></body></html>";
    assert!(html_to_markdown(html).starts_with("# Rust Docs"));
}

#[test]
fn markdown_keeps_link_targets_so_a_model_can_follow_them() {
    let html = r#"<p>See <a href="https://example.com/spec">the spec</a> for more.</p>"#;
    assert_eq!(
        html_to_markdown(html),
        "See [the spec](https://example.com/spec) for more."
    );
}

#[test]
fn markdown_unwraps_navigation_chrome_links_but_keeps_their_words() {
    for href in ["#", "#section-2", "javascript:void(0)", "data:text/html,x"] {
        let html = format!(r#"<p>Go <a href="{href}">here</a> now.</p>"#);
        assert_eq!(html_to_markdown(&html), "Go here now.", "href={href}");
    }
}

#[test]
fn markdown_drops_an_overlong_tracking_url_rather_than_paying_for_it() {
    let href = format!("https://e.com/?{}", "utm=x&".repeat(100));
    assert!(href.len() > MD_MAX_HREF_CHARS);
    assert_eq!(
        html_to_markdown(&format!(r#"<a href="{href}">click</a>"#)),
        "click"
    );
}

#[test]
fn markdown_nests_list_items_by_depth() {
    let html = "<ul><li>one</li><li>two<ul><li>inner</li></ul></li></ul>";
    assert_eq!(html_to_markdown(html), "- one\n- two\n  - inner");
}

#[test]
fn markdown_fences_pre_blocks_and_keeps_their_indentation() {
    let html = "<pre><code>fn main() {\n    println!(\"hi\");\n}</code></pre>";
    assert_eq!(
        html_to_markdown(html),
        "```\nfn main() {\n    println!(\"hi\");\n}\n```"
    );
}

#[test]
fn markdown_keeps_emphasis() {
    let html = "<p><strong>bold</strong> and <em>italic</em> and <code>lit</code></p>";
    assert_eq!(html_to_markdown(html), "**bold** and *italic* and `lit`");
}

#[test]
fn markdown_keeps_an_image_caption_and_never_its_base64_payload() {
    let html = r#"<p><img alt="A chart" src="data:image/png;base64,AAAAAAAAAAAA"></p>"#;
    let md = html_to_markdown(html);
    assert_eq!(md, "[IMAGE: A chart]");
    assert!(!md.contains("base64"), "data URI leaked: {md}");
}

#[test]
fn markdown_ignores_an_image_with_no_alt_text() {
    assert_eq!(html_to_markdown(r#"<p>a<img src="/x.png">b</p>"#), "ab");
}

#[test]
fn markdown_decodes_entities_including_numeric_and_hex_forms() {
    let html = "<p>A &amp; B &lt;tag&gt; &#39;q&#39; &#x2014; &nbsp;done&hellip;</p>";
    assert_eq!(html_to_markdown(html), "A & B <tag> 'q' — done…");
}

#[test]
fn markdown_does_not_mistake_a_data_href_attribute_for_href() {
    assert_eq!(
        html_to_markdown(r#"<a data-href="/wrong" href="/right">t</a>"#),
        "[t](/right)"
    );
}

#[test]
fn markdown_reads_unquoted_attribute_values() {
    assert_eq!(html_to_markdown("<a href=/plain>t</a>"), "[t](/plain)");
}

#[test]
fn markdown_tolerates_a_greater_than_inside_a_quoted_attribute() {
    let html = r#"<div media="(width >= 40rem)"><p>kept</p></div>"#;
    assert_eq!(html_to_markdown(html), "kept");
}

#[test]
fn markdown_does_not_emit_markup_from_inside_a_fenced_block() {
    let html = "<pre><span class=\"k\">let</span> x = 1;</pre>";
    assert_eq!(html_to_markdown(html), "```\nlet x = 1;\n```");
}

#[test]
fn markdown_shrinks_a_markup_heavy_page_by_an_order_of_magnitude() {
    // The property that matters for cost: a page whose bytes are mostly
    // machinery must come out near the size of its prose.
    let mut html = String::from("<html><head>");
    for i in 0..200 {
        html.push_str(&format!(
            "<script>function f{i}(){{return {i}*2;}}</script>"
        ));
    }
    html.push_str("</head><body>");
    for _ in 0..10 {
        html.push_str("<div class=\"a b c\"><span>Real sentence of prose.</span></div>");
    }
    html.push_str("</body></html>");

    let md = html_to_markdown(&html);
    assert!(
        md.len() * 10 < html.len(),
        "expected >10x shrink, got {} -> {}",
        html.len(),
        md.len()
    );
    assert!(md.contains("Real sentence of prose."));
    assert!(!md.contains("return"));
}

#[test]
fn markdown_treats_embed_as_a_void_element() {
    // `embed` has no closing tag in real HTML. Treating it like the
    // other drop-body tags arms skip-until-EOF and silently discards
    // everything after it.
    let html = r#"<embed src="movie.mp4"><h1>Details</h1><p>Body text.</p>"#;
    let md = html_to_markdown(html);
    assert!(md.contains("# Details"), "{md}");
    assert!(md.contains("Body text."), "{md}");
}

#[test]
fn markdown_tracks_nesting_depth_of_dropped_elements() {
    let html = "<template><template>inner</template>secret</template><p>after</p>";
    let md = html_to_markdown(html);
    assert!(!md.contains("secret"), "{md}");
    assert!(!md.contains("inner"), "{md}");
    assert!(md.contains("after"), "{md}");
}

#[test]
fn markdown_escapes_markdown_syntax_in_text_nodes() {
    let html = "<p># This is paragraph text</p><p>- not a list item</p>\
            <p>See [internal](https://example.com) reference and a_var*b*c and `code`.</p>";
    let md = html_to_markdown(html);
    assert!(md.contains("\\# This is paragraph text"), "{md}");
    assert!(md.contains("\\- not a list item"), "{md}");
    assert!(
        md.contains("See \\[internal\\](https://example.com) reference"),
        "{md}"
    );
    assert!(md.contains("a\\_var\\*b\\*c and \\`code\\`."), "{md}");
    // Structural markers this module emits itself are untouched.
    let md = html_to_markdown("<h1>Title</h1><ul><li>item</li></ul>");
    assert_eq!(md, "# Title\n- item");
}

#[test]
fn markdown_escaping_applies_to_text_nodes_only_and_leaves_emitted_syntax_valid() {
    // Text-node escaping (push_markdown_text) must never reach the
    // structural markers the emitters themselves push_str directly:
    // `[text](href)`, `## Heading`, `- bullet`, and a ``` fence. If
    // escaping ever migrated onto those call sites, the output would
    // stop parsing as the Markdown construct it claims to be.
    let html = concat!(
        r#"<h2>Section [1] * notes</h2>"#,
        r#"<p>See <a href="https://example.com/a_b">the [spec] *here*</a> for more.</p>"#,
        r#"<ul><li>item # one *important*</li><li>item [two]</li></ul>"#,
        r#"<pre>fn f() { let x = a * b; a[0] = 1; }</pre>"#,
    );
    let md = html_to_markdown(html);

    // Heading marker is a literal "## ", not escaped, and the text after
    // it still carries the escaped metacharacters.
    assert!(
        md.contains("## Section \\[1\\] \\* notes"),
        "heading marker must stay unescaped: {md}"
    );

    // The link wrapper `[...](...)` is intact — only the link text's
    // interior metacharacters are escaped, not the brackets/parens the
    // emitter itself wrote.
    assert!(
        md.contains("[the \\[spec\\] \\*here\\*](https://example.com/a_b)"),
        "link wrapper must stay valid and unescaped: {md}"
    );

    // List markers are literal "- ", not "\\- ", while item text is
    // escaped.
    // "#" is only escaped at line start (where it would read as a
    // heading); mid-line it needs no escaping to stay literal text.
    assert!(
        md.contains("- item # one \\*important\\*"),
        "list marker must stay unescaped: {md}"
    );
    assert!(
        md.contains("- item \\[two\\]"),
        "list marker must stay unescaped: {md}"
    );

    // The fenced block is untouched: no escaping inside `<pre>`, and the
    // fence delimiters themselves are the literal backtick run.
    assert!(
        md.contains("```\nfn f() { let x = a * b; a[0] = 1; }\n```"),
        "pre content and fence must stay unescaped: {md}"
    );
}

#[test]
fn markdown_keeps_literal_comparison_operators_inside_pre() {
    let html = "<pre>if a < b && c > d</pre>";
    assert_eq!(html_to_markdown(html), "```\nif a < b && c > d\n```");
}

#[test]
fn markdown_selects_a_fence_longer_than_embedded_backtick_runs() {
    let html = "<pre>```\nhello\n```</pre>";
    let md = html_to_markdown(html);
    assert_eq!(md, "````\n```\nhello\n```\n````");
}

#[test]
fn markdown_terminates_the_closing_fence_with_a_newline() {
    let html = "<pre>x</pre>tail";
    let md = html_to_markdown(html);
    assert_eq!(md, "```\nx\n```\ntail");
}

#[test]
fn markdown_keeps_a_list_marker_when_the_item_wraps_a_block_child() {
    let html = "<ul><li><p>one</p></li></ul>";
    assert_eq!(html_to_markdown(html), "- one");
}

#[test]
fn markdown_handles_nested_emphasis_of_the_same_type() {
    let html = "<strong>outer <strong>inner</strong></strong>";
    assert_eq!(html_to_markdown(html), "**outer **inner****");
}

#[test]
fn markdown_preserves_literal_marker_text_when_closing_emphasis() {
    let html = "<strong>foo**</strong>";
    assert_eq!(html_to_markdown(html), "**foo\\*\\***");
}

#[test]
fn markdown_keeps_link_edge_whitespace_outside_the_wrapper() {
    let html = r#"<p>Hello<a href="/x"> world </a>again</p>"#;
    assert_eq!(html_to_markdown(html), "Hello [world](/x) again");
}

#[test]
fn markdown_redacts_secrets_from_link_targets() {
    let html =
        r#"<a href="https://example.test/reset?token=secret&session=private&page=2">click</a>"#;
    let md = html_to_markdown(html);
    assert!(!md.contains("secret"), "{md}");
    assert!(!md.contains("private"), "{md}");
    assert!(md.contains("token=REDACTED"), "{md}");
    assert!(md.contains("session=REDACTED"), "{md}");
    assert!(md.contains("page=2"), "non-sensitive params survive: {md}");
}

#[test]
fn markdown_wraps_link_destinations_with_spaces_or_parens_in_angle_brackets() {
    let html = r#"<a href="/a path (final)">click</a>"#;
    assert_eq!(html_to_markdown(html), "[click](</a path (final)>)");
}

#[test]
fn html_extract_transform_requires_retained_ccr() {
    let mut html = String::from("<html><body>");
    for i in 0..50 {
        html.push_str(&format!(
            "<div class=\"row item-{i}\"><span>cell {i}</span></div>"
        ));
    }
    html.push_str("</body></html>");
    let input = PipelineInput {
        content: &html,
        original_content: &html,
        content_kind: ContentKind::Html,
        original_bytes: html.len(),
    };
    let transform = HtmlExtractTransform;

    let rejecting_store = MemoryCcrStore::new(1, 1);
    assert!(transform.apply(&input, &rejecting_store).is_none());

    let store = MemoryCcrStore::default();
    let out = transform.apply(&input, &store).expect("retained HTML");

    assert_eq!(out.kind(), CompressorKind::Html);
    assert!(out.text().contains("cell 7"), "{}", out.text());
    assert_eq!(store.get(out.token()).as_deref(), Some(html.as_str()));
}

#[test]
fn html_extract_transform_skips_non_html_input() {
    let input = PipelineInput {
        content: "plain text",
        original_content: "plain text",
        content_kind: ContentKind::PlainText,
        original_bytes: "plain text".len(),
    };
    let transform = HtmlExtractTransform;
    let store = MemoryCcrStore::default();

    assert_eq!(transform.estimate_bloat(&input), 0.0);
    assert!(transform.apply(&input, &store).is_none());
}
