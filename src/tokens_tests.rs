use super::*;

#[test]
fn empty_is_zero() {
    assert_eq!(estimate_tokens(""), 0);
}

#[test]
fn rough_quarter_of_chars() {
    assert_eq!(estimate_tokens("abcd"), 1);
    assert_eq!(estimate_tokens(&"x".repeat(400)), 100);
    // Any non-empty input is at least one token.
    assert_eq!(estimate_tokens("a"), 1);
}

#[test]
fn default_ratio_matches_estimate_tokens() {
    for text in ["", "a", "abcd", "abcde", &"x".repeat(401)] {
        assert_eq!(estimate_tokens_with(text, 4.0), estimate_tokens(text));
    }
}

#[test]
fn custom_ratio_rounds_half_up() {
    // 10 chars at 3 cpt → 3.33 + 0.5 → 3; at 4.5 cpt → 2.22 + 0.5 → 2.
    assert_eq!(estimate_tokens_with(&"x".repeat(10), 3.0), 3);
    assert_eq!(estimate_tokens_with(&"x".repeat(10), 4.5), 2);
    // Half rounds up: 9 chars at 6 cpt → 1.5 + 0.5 → 2.
    assert_eq!(estimate_tokens_with(&"x".repeat(9), 6.0), 2);
    // Non-empty input is at least one token.
    assert_eq!(estimate_tokens_with("a", 100.0), 1);
    // Invalid ratios fall back to the default estimate.
    assert_eq!(estimate_tokens_with("abcdefgh", 0.0), 2);
    assert_eq!(estimate_tokens_with("abcdefgh", -1.0), 2);
    assert_eq!(estimate_tokens_with("abcdefgh", f32::NAN), 2);
}

#[test]
fn counts_chars_not_bytes() {
    // 4 multi-byte chars → 1 token, not 12 bytes → 3 tokens.
    assert_eq!(estimate_tokens("日本語訳"), 1);
}
