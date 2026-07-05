//! Reducer for already-extracted web pages.
//!
//! TinyJuice does not fetch URLs. Hosts pass provider-cleaned markdown/text/HTML
//! here, and the reducer bounds large pages with a recoverable head/tail window
//! backed by CCR.

use regex::Regex;

use crate::cache::{self, CcrStore};
use crate::types::{
    WebExtractBatchInput, WebExtractOptions, WebExtractReduceInput, WebExtractReduction,
};

const FOOTER_RULE: &str = "-------- [TOKENJUICE WEB TRUNCATED] --------";

/// Reduce one extracted web page with the global CCR store.
pub fn reduce_web_extract(
    input: &WebExtractReduceInput,
    options: &WebExtractOptions,
) -> WebExtractReduction {
    reduce_web_extract_with_store(input, options, &cache::GlobalCcrStore)
}

/// Reduce one extracted web page with an injected CCR store.
pub fn reduce_web_extract_with_store(
    input: &WebExtractReduceInput,
    options: &WebExtractOptions,
    store: &dyn CcrStore,
) -> WebExtractReduction {
    let (clean, replaced) = clean_content(&input.content, options.convert_base64_images);
    let original_chars = clean.chars().count();
    let limit = clamp_limit(input.char_limit.unwrap_or(options.char_limit), options);
    let source_url_hash = cache::short_hash(&input.url);
    let source_host = source_host(&input.url);

    if original_chars <= limit {
        let inline_chars = clean.chars().count();
        return WebExtractReduction {
            text: clean.clone(),
            body: clean,
            recovery_footer: None,
            ccr_token: None,
            source_host,
            source_url_hash,
            title: input.title.clone(),
            format: input.format,
            original_chars,
            inline_chars,
            head_chars: inline_chars,
            tail_chars: 0,
            omitted_chars: 0,
            truncated: false,
            full_text_retained: false,
            base64_images_replaced: replaced,
        };
    }

    let put = store.put(&clean);
    if !put.retained() {
        // Hard repository invariant: do not emit unrecoverable lossy output.
        let inline_chars = clean.chars().count();
        return WebExtractReduction {
            text: clean.clone(),
            body: clean,
            recovery_footer: None,
            ccr_token: None,
            source_host,
            source_url_hash,
            title: input.title.clone(),
            format: input.format,
            original_chars,
            inline_chars,
            head_chars: inline_chars,
            tail_chars: 0,
            omitted_chars: 0,
            truncated: false,
            full_text_retained: false,
            base64_images_replaced: replaced,
        };
    }

    let head_budget = ((limit as f32) * options.head_ratio.clamp(0.1, 0.9)).floor() as usize;
    let tail_budget = limit.saturating_sub(head_budget).max(1);
    let head = snap_head_to_line(&char_prefix(&clean, head_budget), head_budget);
    let tail = snap_tail_to_line(&char_suffix(&clean, tail_budget), tail_budget);
    let head_chars = head.chars().count();
    let tail_chars = tail.chars().count();
    let omitted_chars = original_chars.saturating_sub(head_chars + tail_chars);
    let omitted_start_line = head.lines().count().saturating_add(1);

    let mut body = String::with_capacity(head.len() + tail.len() + 160);
    body.push_str(&head);
    if !body.ends_with('\n') {
        body.push('\n');
    }
    body.push_str("\n-------- [OMITTED MIDDLE] --------\n");
    body.push_str(&tail);

    let footer = web_recovery_footer(
        put.token(),
        original_chars,
        head_chars,
        tail_chars,
        omitted_start_line,
    );
    let mut text = body.clone();
    text.push_str(&footer);

    WebExtractReduction {
        inline_chars: text.chars().count(),
        text,
        body,
        recovery_footer: Some(footer),
        ccr_token: Some(put.token().to_string()),
        source_host,
        source_url_hash,
        title: input.title.clone(),
        format: input.format,
        original_chars,
        head_chars,
        tail_chars,
        omitted_chars,
        truncated: true,
        full_text_retained: true,
        base64_images_replaced: replaced,
    }
}

/// Reduce a batch while preserving per-page retrieval footers.
pub fn reduce_web_extract_batch_with_store(
    input: &WebExtractBatchInput,
    options: &WebExtractOptions,
    store: &dyn CcrStore,
) -> Vec<WebExtractReduction> {
    let mut page_options = *options;
    if let Some(default_char_limit) = input.default_char_limit {
        page_options.char_limit = default_char_limit;
    }
    if let Some(max_combined_inline_chars) = input.max_combined_inline_chars {
        page_options.max_combined_inline_chars = max_combined_inline_chars;
    }
    let mut reductions: Vec<WebExtractReduction> = input
        .pages
        .iter()
        .map(|page| reduce_web_extract_with_store(page, &page_options, store))
        .collect();

    if combined_inline_chars(&reductions) <= page_options.max_combined_inline_chars
        || input.pages.is_empty()
    {
        return reductions;
    }

    let per_page_limit = (page_options.max_combined_inline_chars / input.pages.len())
        .max(page_options.min_char_limit)
        .min(page_options.char_limit);
    let tightened_options = WebExtractOptions {
        char_limit: per_page_limit,
        ..page_options
    };
    reductions = input
        .pages
        .iter()
        .map(|page| reduce_web_extract_with_store(page, &tightened_options, store))
        .collect();

    // Never enforce the combined budget by slicing text after reduction; that
    // could sever a CCR footer and make omitted text unreachable.
    reductions
}

fn combined_inline_chars(reductions: &[WebExtractReduction]) -> usize {
    reductions.iter().map(|r| r.inline_chars).sum()
}

fn clean_content(content: &str, convert_base64_images: bool) -> (String, usize) {
    if !convert_base64_images {
        return (content.to_string(), 0);
    }
    replace_inline_base64_images(content)
}

/// Replace embedded image bytes while preserving ordinary remote image URLs.
pub fn replace_inline_base64_images(content: &str) -> (String, usize) {
    static MARKDOWN_IMAGE: std::sync::OnceLock<Regex> = std::sync::OnceLock::new();
    static PAREN_IMAGE: std::sync::OnceLock<Regex> = std::sync::OnceLock::new();
    static RAW_IMAGE: std::sync::OnceLock<Regex> = std::sync::OnceLock::new();

    let markdown = MARKDOWN_IMAGE.get_or_init(|| {
        Regex::new(r"!\[([^\]]*)\]\(data:image/[A-Za-z0-9.+-]+;base64,[^)]+\)").unwrap()
    });
    let paren = PAREN_IMAGE
        .get_or_init(|| Regex::new(r"\(data:image/[A-Za-z0-9.+-]+;base64,[^)]+\)").unwrap());
    let raw = RAW_IMAGE.get_or_init(|| {
        Regex::new(r"data:image/[A-Za-z0-9.+-]+;base64,[A-Za-z0-9+/=_-]+").unwrap()
    });

    let mut count = 0usize;
    let text = markdown.replace_all(content, |caps: &regex::Captures<'_>| {
        count += 1;
        let alt = caps.get(1).map(|m| m.as_str().trim()).unwrap_or_default();
        if alt.is_empty() {
            "[IMAGE]".to_string()
        } else {
            format!("[IMAGE: {alt}]")
        }
    });
    let text = paren.replace_all(&text, |_caps: &regex::Captures<'_>| {
        count += 1;
        "[IMAGE]".to_string()
    });
    let text = raw.replace_all(&text, |_caps: &regex::Captures<'_>| {
        count += 1;
        "[IMAGE]".to_string()
    });
    (text.into_owned(), count)
}

fn clamp_limit(limit: usize, options: &WebExtractOptions) -> usize {
    let min = options.min_char_limit.min(options.max_char_limit);
    let max = options.max_char_limit.max(min);
    limit.clamp(min, max)
}

fn char_prefix(s: &str, char_count: usize) -> String {
    s.chars().take(char_count).collect()
}

fn char_suffix(s: &str, char_count: usize) -> String {
    let chars: Vec<char> = s.chars().collect();
    let start = chars.len().saturating_sub(char_count);
    chars[start..].iter().collect()
}

fn snap_head_to_line(head: &str, budget: usize) -> String {
    let Some(idx) = head.rfind('\n') else {
        return head.to_string();
    };
    let snapped = &head[..idx + 1];
    if snapped.chars().count() >= budget / 2 {
        snapped.to_string()
    } else {
        head.to_string()
    }
}

fn snap_tail_to_line(tail: &str, budget: usize) -> String {
    let Some(idx) = tail.find('\n') else {
        return tail.to_string();
    };
    let snapped = &tail[idx + 1..];
    if snapped.chars().count() >= budget / 2 {
        snapped.to_string()
    } else {
        tail.to_string()
    }
}

fn web_recovery_footer(
    token: &str,
    original_chars: usize,
    head_chars: usize,
    tail_chars: usize,
    omitted_start_line: usize,
) -> String {
    let marker = cache::format_marker(token);
    format!(
        "\n\n{FOOTER_RULE}\n\
         Showing {head_chars} chars (head) + {tail_chars} chars (tail) of \
         {original_chars} total clean characters.\n\
         Full text token: {token} (marker {marker}).\n\
         To read the omitted middle: {} token=\"{token}\" offset={omitted_start_line} limit=<n>\n\
         ----------------------------------------------",
        cache::RETRIEVE_TOOL_NAME
    )
}

fn source_host(url: &str) -> Option<String> {
    let after_scheme = url
        .split_once("://")
        .map(|(_, rest)| rest)
        .unwrap_or(url)
        .trim_start_matches('/');
    let host = after_scheme
        .split(['/', '?', '#'])
        .next()
        .unwrap_or_default()
        .trim();
    if host.is_empty() {
        None
    } else {
        Some(host.to_ascii_lowercase())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cache::{MemoryCcrStore, parse_markers};
    use crate::types::WebExtractFormat;
    use serde_json::Map;

    fn page(content: String) -> WebExtractReduceInput {
        WebExtractReduceInput {
            url: "https://example.com/path?token=secret".to_string(),
            title: Some("Example".to_string()),
            content,
            format: WebExtractFormat::Markdown,
            provider: Some("test".to_string()),
            char_limit: None,
            metadata: Map::new(),
        }
    }

    fn options() -> WebExtractOptions {
        WebExtractOptions {
            char_limit: 80,
            min_char_limit: 20,
            max_char_limit: 500,
            head_ratio: 0.75,
            convert_base64_images: true,
            max_combined_inline_chars: 1000,
        }
    }

    #[test]
    fn small_page_returns_cleaned_content_without_footer() {
        let store = MemoryCcrStore::new(10, 10_000);
        let input = page(
            "intro ![chart](data:image/png;base64,AAAA1111) ![remote](https://x/y.png)".to_string(),
        );

        let out = reduce_web_extract_with_store(&input, &options(), &store);

        assert_eq!(out.text, "intro [IMAGE: chart] ![remote](https://x/y.png)");
        assert_eq!(out.base64_images_replaced, 1);
        assert!(out.recovery_footer.is_none());
        assert!(!out.text.contains("AAAA1111"));
        assert!(out.text.contains("https://x/y.png"));
    }

    #[test]
    fn large_page_returns_head_tail_footer_and_retains_full_text() {
        let store = MemoryCcrStore::new(10, 100_000);
        let input = page((0..40).map(|i| format!("line-{i:02}\n")).collect());

        let out = reduce_web_extract_with_store(&input, &options(), &store);

        assert!(out.truncated);
        assert!(out.full_text_retained);
        assert!(out.text.contains(FOOTER_RULE));
        assert!(out.text.contains("tokenjuice_retrieve"));
        assert_eq!(
            parse_markers(&out.text),
            vec![out.ccr_token.clone().unwrap()]
        );
        assert_eq!(
            store.get(out.ccr_token.as_deref().unwrap()).as_deref(),
            Some(input.content.as_str())
        );
        assert!(out.text.contains("line-00"));
        assert!(out.text.contains("line-39"));
    }

    #[test]
    fn store_failure_returns_cleaned_whole_page_without_unrecoverable_footer() {
        let store = MemoryCcrStore::new(1, 16);
        let input = page("x".repeat(200));

        let out = reduce_web_extract_with_store(&input, &options(), &store);

        assert!(!out.truncated);
        assert!(!out.full_text_retained);
        assert!(out.recovery_footer.is_none());
        assert_eq!(out.text, input.content);
    }

    #[test]
    fn clamps_invalid_limits() {
        let store = MemoryCcrStore::new(10, 100_000);
        let mut input = page("x\n".repeat(300));
        input.char_limit = Some(1);

        let out = reduce_web_extract_with_store(&input, &options(), &store);

        assert!(out.head_chars + out.tail_chars >= 20 / 2);
        assert!(out.truncated);
    }

    #[test]
    fn defaults_and_huge_limits_are_clamped() {
        let store = MemoryCcrStore::new(10, 100_000);
        let mut input = page("x\n".repeat(300));
        let options = WebExtractOptions {
            char_limit: 30,
            min_char_limit: 20,
            max_char_limit: 50,
            ..options()
        };

        let defaulted = reduce_web_extract_with_store(&input, &options, &store);
        assert!(defaulted.truncated);
        assert!(defaulted.head_chars + defaulted.tail_chars <= 30);

        input.char_limit = Some(usize::MAX);
        let clamped = reduce_web_extract_with_store(&input, &options, &store);
        assert!(clamped.truncated);
        assert!(clamped.head_chars + clamped.tail_chars <= 50);
    }

    #[test]
    fn batch_tightens_budget_without_dropping_recovery_footers() {
        let store = MemoryCcrStore::new(10, 100_000);
        let input = WebExtractBatchInput {
            pages: vec![page("a\n".repeat(200)), page("b\n".repeat(200))],
            default_char_limit: Some(80),
            max_combined_inline_chars: Some(80),
        };
        let options = WebExtractOptions {
            min_char_limit: 20,
            ..options()
        };

        let out = reduce_web_extract_batch_with_store(&input, &options, &store);

        assert_eq!(out.len(), 2);
        assert!(out.iter().all(|r| r.truncated));
        assert!(out.iter().all(|r| r.text.contains(FOOTER_RULE)));
        assert!(out.iter().all(|r| parse_markers(&r.text).len() == 1));
        assert!(
            out.iter()
                .map(|r| r.head_chars + r.tail_chars)
                .sum::<usize>()
                <= 80
        );
    }

    #[test]
    fn metadata_uses_host_and_url_hash_not_full_url() {
        let store = MemoryCcrStore::new(10, 10_000);
        let input = page("short".to_string());
        let out = reduce_web_extract_with_store(&input, &options(), &store);
        let json = serde_json::to_string(&out).unwrap();

        assert_eq!(out.source_host.as_deref(), Some("example.com"));
        assert!(!json.contains("token=secret"));
        assert!(!json.contains("/path"));
        assert!(!out.source_url_hash.is_empty());
    }
}
