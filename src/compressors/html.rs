//! HTML → readable-text extractor.
//!
//! Strips markup and returns the readable text content, in the spirit of
//! Headroom's `HTMLExtractor`. Linear-time, allocation-light (no DOM, no
//! regex): it scans once, dropping `<script>`/`<style>`/`<head>` bodies and
//! comments, inserting newlines at block-level boundaries, and decoding the
//! handful of common HTML entities. Lossy — the router offloads the original
//! HTML to CCR so the exact markup is recoverable.

use async_trait::async_trait;

use super::Compressor;
use crate::types::{CompressInput, CompressOptions, CompressOutput, CompressorKind};

/// Block-level tags after which we emit a newline so the extracted text keeps
/// document structure (paragraphs, list items, headings, rows).
const BLOCK_TAGS: &[&str] = &[
    "p",
    "div",
    "br",
    "li",
    "ul",
    "ol",
    "tr",
    "table",
    "h1",
    "h2",
    "h3",
    "h4",
    "h5",
    "h6",
    "section",
    "article",
    "header",
    "footer",
    "blockquote",
    "pre",
    "hr",
    "title",
];

/// Tags whose entire body is dropped (non-content). `head` is deliberately
/// not here: dropping it would lose `<title>` — often the highest-signal
/// string on the page — while scripts/styles inside the head are still
/// dropped by their own tags and meta/link carry no text.
const DROP_BODY_TAGS: &[&str] = &["script", "style", "noscript", "svg"];

/// Inline formatting tags that do not break the text flow. Any other tag acts
/// as a separator (space or newline) so adjacent element values — e.g. RSS
/// `<guid>` / `<comments>` siblings — don't run together in the output.
const INLINE_TAGS: &[&str] = &[
    "a", "b", "i", "em", "strong", "span", "code", "small", "sub", "sup",
];

pub struct HtmlCompressor;

#[async_trait]
impl Compressor for HtmlCompressor {
    fn kind(&self) -> CompressorKind {
        CompressorKind::Html
    }

    async fn compress(
        &self,
        input: &CompressInput<'_>,
        _opts: &CompressOptions,
    ) -> Option<CompressOutput> {
        compress(input.content)
    }
}

/// Extract readable text from an HTML document. Returns `None` if extraction
/// wouldn't shrink the content or yields nothing useful.
pub fn compress(content: &str) -> Option<CompressOutput> {
    let text = html_to_text(content);
    let text = collapse_blank_lines(&text);
    if text.trim().is_empty() || text.len() >= content.len() {
        return None;
    }
    log::debug!(
        "[tinyjuice][html] {} -> {} bytes",
        content.len(),
        text.len()
    );
    Some(CompressOutput::lossy(text, CompressorKind::Html))
}

/// Single-pass HTML tag stripper that drops non-content bodies, honours block
/// boundaries, and decodes common entities.
pub fn html_to_text(html: &str) -> String {
    let bytes = html.as_bytes();
    let mut out = String::with_capacity(html.len() / 2);
    let mut i = 0usize;
    let mut skip_until: Option<&'static str> = None;
    // Number of `<![CDATA[` openers whose `]]>` closer we still owe. CDATA
    // contents are scanned by this same loop (HN RSS wraps HTML in CDATA, so
    // stripping its tags is what we want); only the delimiters are consumed
    // as markup. Tracked iteratively rather than recursing on the payload so
    // pathological nesting can't blow the stack.
    let mut cdata_depth = 0usize;

    while i < bytes.len() {
        if bytes[i] == b'<' {
            if let Some(skip_tag) = skip_until {
                // Inside a dropped body only the matching close tag is markup.
                // Anything else — comparison operators in inline JS, stray
                // `<` in CSS or CDATA — is body text, so a lone `<` must not
                // consume up to the next `>` (that could swallow the real
                // close tag and drop the rest of the document).
                if html[i + 1..].starts_with('/')
                    && let Some(rel_end) = html[i..].find('>')
                {
                    let (name, is_close) = parse_tag_name(&html[i + 1..i + rel_end]);
                    if is_close && name == skip_tag {
                        skip_until = None;
                        i += rel_end + 1;
                        continue;
                    }
                }
                i += 1;
                continue;
            }
            // Comment?
            if html[i..].starts_with("<!--") {
                if let Some(end) = html[i..].find("-->") {
                    i += end + 3;
                    continue;
                }
                break;
            }
            // CDATA section: the delimiters are markup, the payload is
            // scanned by this same loop (see `cdata_depth`).
            if html[i..].starts_with("<![CDATA[") {
                cdata_depth += 1;
                i += "<![CDATA[".len();
                continue;
            }
            // Find the end of this tag, skipping `>` inside quoted attribute
            // values (e.g. `media="(width >= 40rem)"`).
            let Some(rel_end) = find_tag_end(html, i) else {
                break;
            };
            let tag_raw = &html[i + 1..i + rel_end];
            let (name, is_close) = parse_tag_name(tag_raw);

            if !is_close && DROP_BODY_TAGS.contains(&name.as_str()) && !tag_raw.ends_with('/') {
                skip_until = Some(static_tag(&name));
                i += rel_end + 1;
                continue;
            }

            if BLOCK_TAGS.contains(&name.as_str()) {
                if !out.ends_with('\n') {
                    out.push('\n');
                }
            } else if !INLINE_TAGS.contains(&name.as_str())
                && !out.is_empty()
                && !out.ends_with(|c: char| c.is_whitespace())
            {
                // Unrecognised tag: emit a separator so sibling element
                // values (RSS `<guid>`, `<pubDate>`, ...) don't concatenate.
                out.push(' ');
            }
            i += rel_end + 1;
            continue;
        }

        if skip_until.is_some() {
            i += 1;
            continue;
        }

        // Consume a pending CDATA closer as markup, not text.
        if cdata_depth > 0 && html[i..].starts_with("]]>") {
            cdata_depth -= 1;
            i += "]]>".len();
            continue;
        }

        // Decode an entity or copy the char.
        if bytes[i] == b'&'
            && let Some((decoded, consumed)) = decode_entity(&html[i..])
        {
            out.push_str(&decoded);
            i += consumed;
            continue;
        }
        let ch = html[i..].chars().next().unwrap();
        out.push(ch);
        i += ch.len_utf8();
    }
    out
}

/// Find the offset (relative to `from`, which points at `<`) of the `>` that
/// terminates the tag, skipping `>` inside single- or double-quoted attribute
/// values. If a quote is left unterminated, falls back to the first raw `>`
/// so one malformed tag can't swallow the rest of the document.
fn find_tag_end(html: &str, from: usize) -> Option<usize> {
    let bytes = html.as_bytes();
    let mut j = from + 1;
    while j < bytes.len() {
        match bytes[j] {
            b'>' => return Some(j - from),
            quote @ (b'"' | b'\'') => match bytes[j + 1..].iter().position(|&b| b == quote) {
                Some(close) => j += close + 2,
                None => return html[from..].find('>'),
            },
            _ => j += 1,
        }
    }
    None
}

/// Parse `<...>` inner text into `(lowercased name, is_closing)`.
fn parse_tag_name(tag_raw: &str) -> (String, bool) {
    let trimmed = tag_raw.trim();
    let (is_close, rest) = if let Some(r) = trimmed.strip_prefix('/') {
        (true, r)
    } else {
        (false, trimmed)
    };
    let name: String = rest
        .chars()
        .take_while(|c| c.is_ascii_alphanumeric())
        .collect::<String>()
        .to_ascii_lowercase();
    (name, is_close)
}

/// Return the `'static` slice matching a recognised drop-body tag name.
fn static_tag(name: &str) -> &'static str {
    DROP_BODY_TAGS
        .iter()
        .copied()
        .find(|t| *t == name)
        .unwrap_or("script")
}

/// Decode a leading HTML entity at the start of `s`. Returns the decoded text
/// and the number of bytes consumed (including `&` and `;`).
fn decode_entity(s: &str) -> Option<(std::borrow::Cow<'static, str>, usize)> {
    const ENTITIES: &[(&str, &str)] = &[
        ("&amp;", "&"),
        ("&lt;", "<"),
        ("&gt;", ">"),
        ("&quot;", "\""),
        ("&#39;", "'"),
        ("&apos;", "'"),
        ("&nbsp;", " "),
        ("&mdash;", "—"),
        ("&ndash;", "–"),
        ("&hellip;", "…"),
        ("&copy;", "©"),
    ];
    for (ent, decoded) in ENTITIES {
        if s.starts_with(ent) {
            return Some(((*decoded).into(), ent.len()));
        }
    }
    // Numeric character references: &#8212; and &#x27;
    let rest = s.strip_prefix("&#")?;
    let (digits, radix) = match rest.strip_prefix(['x', 'X']) {
        Some(hex) => (hex, 16),
        None => (rest, 10),
    };
    let end = digits
        .char_indices()
        .take(8)
        .take_while(|(_, c)| c.is_ascii_hexdigit())
        .last()
        .map(|(i, c)| i + c.len_utf8())?;
    if !digits[end..].starts_with(';') {
        return None;
    }
    let code = u32::from_str_radix(&digits[..end], radix).ok()?;
    let ch = char::from_u32(code).filter(|c| !c.is_control() || *c == '\n' || *c == '\t')?;
    let consumed = s.len() - digits.len() + end + 1;
    Some((ch.to_string().into(), consumed))
}

/// Collapse runs of blank lines and trim trailing whitespace per line.
fn collapse_blank_lines(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    let mut blanks = 0usize;
    for line in text.lines() {
        let trimmed = line.trim_end();
        let collapsed = trimmed.split_whitespace().collect::<Vec<_>>().join(" ");
        if collapsed.is_empty() {
            blanks += 1;
            if blanks <= 1 {
                out.push('\n');
            }
        } else {
            blanks = 0;
            out.push_str(&collapsed);
            out.push('\n');
        }
    }
    out.trim().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

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
        assert!(!text.contains("<b>"), "markup in CDATA not stripped: {text}");
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
        assert!(out.lossy);
        assert!(out.text.len() < html.len());
        assert!(out.text.contains("cell 7"));
    }
}
