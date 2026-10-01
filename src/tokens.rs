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
#[path = "tokens_tests.rs"]
mod tests;
