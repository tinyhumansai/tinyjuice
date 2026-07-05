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
            // Find the end of this tag.
            let Some(rel_end) = html[i..].find('>') else {
                break;
            };
            let tag_raw = &html[i + 1..i + rel_end];
            let (name, is_close) = parse_tag_name(tag_raw);

            if !is_close && DROP_BODY_TAGS.contains(&name.as_str()) && !tag_raw.ends_with('/') {
                skip_until = Some(static_tag(&name));
                i += rel_end + 1;
                continue;
            }

            if BLOCK_TAGS.contains(&name.as_str()) && !out.ends_with('\n') {
                out.push('\n');
            }
            i += rel_end + 1;
            continue;
        }

        if skip_until.is_some() {
            i += 1;
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
