//! Adaptive keep-count selection for list-shaped compressors.
//!
//! Clean-room implementation inspired by Headroom's adaptive retention
//! heuristics (Apache-2.0). Compressors that keep "top K of N" items (search
//! matches, log lines, table rows) traditionally hard-code K, which over-keeps
//! redundant sets and over-drops diverse ones. [`compute_optimal_k`] sizes K
//! from the items themselves:
//!
//! 1. **Near-duplicate collapse** — a 64-bit SimHash over character 4-grams
//!    clusters items whose hashes are within Hamming distance 3. When the set
//!    collapses to at most 3 distinct clusters, keeping one exemplar per
//!    cluster is enough.
//! 2. **Knee detection (Kneedle)** — the cumulative unique word-bigram
//!    coverage curve says how fast new information stops arriving. The knee
//!    (max gap between the normalised curve and the diagonal) is where extra
//!    items stop paying for themselves.
//! 3. **Diversity fallback/floor** — when no knee exists, or the set is
//!    highly diverse despite an early knee, K scales with the fraction of
//!    distinct SimHash clusters.

use std::collections::HashSet;
use std::hash::{DefaultHasher, Hash, Hasher};

/// Sets this small are kept whole — selection overhead isn't worth it.
const SMALL_SET: usize = 8;
/// SimHash Hamming distance at or below which two items are near-duplicates.
const NEAR_DUP_DISTANCE: u32 = 3;
/// Cluster counts at or below this mean the set is essentially redundant.
const REDUNDANT_CLUSTERS: usize = 3;
/// Minimum normalised gap for a knee to count (Kneedle sensitivity).
const KNEE_MIN_GAP: f32 = 0.05;
/// Diversity above this raises a knee-derived K to the diversity floor.
const HIGH_DIVERSITY: f32 = 0.7;

/// Compute how many of `items` (in their given order) are worth keeping,
/// clamped to `[min_k, max_k]`. Sets of at most 8 items are returned whole.
pub(crate) fn compute_optimal_k(items: &[&str], min_k: usize, max_k: usize) -> usize {
    let n = items.len();
    if n <= SMALL_SET {
        return n;
    }
    let lo = min_k.min(max_k);

    let hashes: Vec<u64> = items.iter().map(|item| simhash(item)).collect();
    let clusters = cluster_count(&hashes);
    if clusters <= REDUNDANT_CLUSTERS {
        // Near-duplicates all the way down: one exemplar per cluster.
        return clusters.clamp(lo, max_k);
    }

    let diversity = clusters as f32 / n as f32;
    let diversity_floor = (n as f32 * (0.3 + 0.7 * diversity)).round() as usize;
    let k = match knee_index(items) {
        // An early knee on a highly diverse set under-counts (the curve is
        // noisy when almost every item is novel) — respect the diversity floor.
        Some(knee) if diversity > HIGH_DIVERSITY => knee.max(diversity_floor),
        Some(knee) => knee,
        None => diversity_floor,
    };
    k.clamp(lo, max_k)
}

/// 64-bit SimHash over character 4-grams with per-bit voting. Items shorter
/// than one gram hash as a single feature.
fn simhash(text: &str) -> u64 {
    let chars: Vec<char> = text.chars().collect();
    let mut votes = [0i32; 64];
    let mut vote = |feature_hash: u64| {
        for (bit, slot) in votes.iter_mut().enumerate() {
            if feature_hash >> bit & 1 == 1 {
                *slot += 1;
            } else {
                *slot -= 1;
            }
        }
    };
    if chars.len() < 4 {
        vote(hash_feature(&chars));
    } else {
        for gram in chars.windows(4) {
            vote(hash_feature(gram));
        }
    }
    votes
        .iter()
        .enumerate()
        .fold(0u64, |acc, (bit, &v)| acc | (u64::from(v > 0) << bit))
}

fn hash_feature(gram: &[char]) -> u64 {
    let mut hasher = DefaultHasher::new();
    gram.hash(&mut hasher);
    hasher.finish()
}

/// Greedy single-link clustering: hashes within [`NEAR_DUP_DISTANCE`] of an
/// existing representative join that cluster.
fn cluster_count(hashes: &[u64]) -> usize {
    let mut representatives: Vec<u64> = Vec::new();
    for &hash in hashes {
        let near_dup = representatives
            .iter()
            .any(|&rep| (rep ^ hash).count_ones() <= NEAR_DUP_DISTANCE);
        if !near_dup {
            representatives.push(hash);
        }
    }
    representatives.len()
}

/// Kneedle knee detection on the cumulative unique word-bigram coverage
/// curve. Returns the 1-based item count at the knee, or `None` when the
/// curve is too close to the diagonal (information arrives evenly).
fn knee_index(items: &[&str]) -> Option<usize> {
    let n = items.len();
    let mut seen: HashSet<u64> = HashSet::new();
    let mut coverage: Vec<usize> = Vec::with_capacity(n);
    for item in items {
        let words: Vec<&str> = item.split_whitespace().collect();
        if words.len() < 2 {
            // A one-word item still carries information: count it as a
            // degenerate bigram so all-short sets aren't flat-zero.
            if let Some(word) = words.first() {
                seen.insert(hash_bigram(word, ""));
            }
        } else {
            for pair in words.windows(2) {
                seen.insert(hash_bigram(pair[0], pair[1]));
            }
        }
        coverage.push(seen.len());
    }

    let total = *coverage.last()? as f32;
    if total <= 0.0 || n < 2 {
        return None;
    }
    let mut best_gap = 0.0f32;
    let mut best_index = 0usize;
    for (i, &y) in coverage.iter().enumerate() {
        let x = i as f32 / (n - 1) as f32;
        let gap = y as f32 / total - x;
        if gap > best_gap {
            best_gap = gap;
            best_index = i;
        }
    }
    (best_gap > KNEE_MIN_GAP).then_some(best_index + 1)
}

fn hash_bigram(a: &str, b: &str) -> u64 {
    let mut hasher = DefaultHasher::new();
    a.hash(&mut hasher);
    b.hash(&mut hasher);
    hasher.finish()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tiny_sets_are_kept_whole() {
        let items = ["alpha one", "beta two", "gamma three"];
        assert_eq!(compute_optimal_k(&items, 1, 10), 3);
    }

    #[test]
    fn redundant_items_yield_small_k() {
        let items: Vec<String> = (0..24)
            .map(|_| "connection timeout retrying request to upstream host".to_string())
            .collect();
        let refs: Vec<&str> = items.iter().map(String::as_str).collect();
        let k = compute_optimal_k(&refs, 2, 15);
        assert_eq!(k, 2, "identical items collapse to the floor");
    }

    #[test]
    fn diverse_items_yield_k_near_n() {
        let vocab = [
            "database migration applied schema version",
            "user login rejected invalid credential token",
            "cache eviction pressure exceeded memory budget",
            "worker heartbeat missed scheduler requeued job",
            "tls handshake negotiated cipher suite protocol",
            "disk io latency spiked during checkpoint flush",
            "feature flag rollout gated cohort percentage",
            "payment webhook signature verified merchant order",
            "dns resolution fallback secondary nameserver query",
            "kernel oom killer reaped container process group",
            "metrics exporter scraped endpoint histogram bucket",
            "queue backlog drained consumer lag recovered",
            "certificate renewal scheduled expiry threshold reached",
            "replica election promoted follower after partition",
            "rate limiter shed excess burst traffic upstream",
            "search index rebuilt analyzer tokenizer updated",
            "session cookie rotated secure attribute enforced",
            "thread pool saturated queued tasks rejected",
            "object storage multipart upload part etag",
            "grpc deadline exceeded retry budget exhausted",
        ];
        let refs: Vec<&str> = vocab.to_vec();
        let k = compute_optimal_k(&refs, 2, 15);
        assert_eq!(k, 15, "fully diverse items should reach the ceiling");
    }

    #[test]
    fn front_loaded_information_finds_early_knee() {
        // Four information-dense items up front, then a long tail of
        // near-boilerplate variations that add almost no new bigrams.
        let mut items: Vec<String> = vec![
            "FAIL src/auth/session.test.ts expected token refresh to rotate secrets".to_string(),
            "AssertionError expected 401 unauthorized received 200 ok at handler".to_string(),
            "stack trace at validateSession auth/session.ts line 118 column 9".to_string(),
            "caused by expired signing key rotation missed grace window".to_string(),
        ];
        for _ in 0..20 {
            items.push("retry attempt failed with timeout waiting for upstream".to_string());
        }
        let refs: Vec<&str> = items.iter().map(String::as_str).collect();
        let k = compute_optimal_k(&refs, 2, 20);
        assert!(
            (2..=8).contains(&k),
            "knee should land near the dense head, got {k}"
        );
    }

    #[test]
    fn result_is_clamped_to_bounds() {
        let items: Vec<String> = (0..30).map(|_| "same line again".to_string()).collect();
        let refs: Vec<&str> = items.iter().map(String::as_str).collect();
        assert_eq!(compute_optimal_k(&refs, 5, 12), 5);
    }
}
