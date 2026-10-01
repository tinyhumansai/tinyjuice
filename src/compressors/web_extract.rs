//! Reducer for already-extracted web pages.
//!
//! TinyJuice does not fetch URLs. Hosts pass provider-cleaned markdown/text/HTML
//! here, and the reducer bounds large pages with a recoverable head/tail window
//! backed by CCR.

use regex::Regex;

use crate::cache::{self, CcrStore};
use crate::pipeline::OffloadOutput;
use crate::types::{
    CompressorKind, WebExtractBatchInput, WebExtractOptions, WebExtractReduceInput,
    WebExtractReduction,
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

    let put = store.put(&clean);
    let Some(offload) = OffloadOutput::from_retained_put(body, CompressorKind::Html, put) else {
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
    };

    let footer = web_recovery_footer(
        offload.token(),
        original_chars,
        head_chars,
        tail_chars,
        omitted_start_line,
    );
    let body = offload.text().to_string();
    let mut text = body.clone();
    text.push_str(&footer);

    WebExtractReduction {
        inline_chars: text.chars().count(),
        text,
        body,
        recovery_footer: Some(footer),
        ccr_token: Some(offload.token().to_string()),
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
#[path = "web_extract_tests.rs"]
mod tests;
