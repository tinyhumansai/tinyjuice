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
    assert!(out.text.contains("tinyjuice_retrieve"));
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
fn one_mb_page_returns_recoverable_head_tail_footer() {
    let mut content = String::new();
    let mut line = 0usize;
    while content.len() <= 1024 * 1024 {
        content.push_str(&format!(
            "megapage-line-{line:06}: extracted article text\n"
        ));
        line += 1;
    }
    content.push_str("megapage-final-sentinel\n");
    let store = MemoryCcrStore::new(4, content.len() * 2);
    let input = page(content);
    let options = WebExtractOptions {
        char_limit: 160,
        head_ratio: 0.5,
        ..options()
    };

    let out = reduce_web_extract_with_store(&input, &options, &store);

    assert!(input.content.len() > 1024 * 1024);
    assert!(out.truncated);
    assert!(out.full_text_retained);
    assert!(out.text.contains("megapage-line-000000"));
    assert!(out.text.contains("megapage-final-sentinel"));
    assert!(out.text.contains(FOOTER_RULE));
    assert!(out.text.contains("tinyjuice_retrieve"));
    assert_eq!(
        parse_markers(&out.text),
        vec![out.ccr_token.clone().unwrap()]
    );
    assert_eq!(
        store.get(out.ccr_token.as_deref().unwrap()).as_deref(),
        Some(input.content.as_str())
    );
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
