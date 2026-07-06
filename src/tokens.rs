//! Lightweight token estimation for compaction savings accounting.
//!
//! The agent harness gets authoritative token counts from the provider, but
//! the content router runs *before* any provider call, so it has no exact
//! count for a tool result. For savings insights we use the standard ~4
//! characters-per-token heuristic (close enough for English text / code /
//! JSON to drive cost estimates and the savings dashboard). This is the same
//! order of approximation Headroom reports its savings with.

/// Average characters per token used by the estimate.
pub const CHARS_PER_TOKEN: f64 = 4.0;

/// Estimate the number of tokens in `text` (≈ chars / 4, minimum 1 for any
/// non-empty input). Uses `chars().count()` so multi-byte text isn't
/// over-counted by byte length.
pub fn estimate_tokens(text: &str) -> u64 {
    if text.is_empty() {
        return 0;
    }
    let chars = text.chars().count() as f64;
    (chars / CHARS_PER_TOKEN).ceil().max(1.0) as u64
}

/// Estimate tokens with a caller-calibrated characters-per-token ratio
/// (see `CompressOptions::chars_per_token`).
///
/// With the default ratio (4.0) this is exactly [`estimate_tokens`] — the
/// historical ceiling-division estimate — so default behaviour never shifts.
/// A custom ratio uses round-half-up (`max(1, chars/cpt + 0.5)`), which is a
/// better unbiased estimator when the ratio has been calibrated. Non-positive
/// ratios fall back to the default.
pub fn estimate_tokens_with(text: &str, chars_per_token: f32) -> u64 {
    if text.is_empty() {
        return 0;
    }
    if !chars_per_token.is_finite()
        || chars_per_token <= 0.0
        || (f64::from(chars_per_token) - CHARS_PER_TOKEN).abs() < 1e-9
    {
        return estimate_tokens(text);
    }
    let chars = text.chars().count() as f32;
    ((chars / chars_per_token + 0.5) as u64).max(1)
}

#[cfg(test)]
mod tests {
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
}
