use once_cell::sync::Lazy;
use regex::Regex;
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// UTF-8 byte range in a provider request or rendered prompt buffer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ByteRange {
    pub start: usize,
    pub end: usize,
}

impl ByteRange {
    pub fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }

    pub fn len(self) -> usize {
        self.end.saturating_sub(self.start)
    }

    pub fn is_empty(self) -> bool {
        self.start >= self.end
    }

    fn contains(self, other: ByteRange) -> bool {
        other.start >= self.start && other.end <= self.end
    }

    fn overlaps(self, other: ByteRange) -> bool {
        self.start < other.end && other.start < self.end
    }

    fn is_valid_for(self, input: &str) -> bool {
        self.start <= self.end
            && self.end <= input.len()
            && input.is_char_boundary(self.start)
            && input.is_char_boundary(self.end)
    }
}

/// Provider-neutral live-zone description. Hosts decide these ranges while
/// TinyJuice validates that replacement splicing does not rewrite frozen bytes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LiveZone {
    pub provider: String,
    pub source: String,
    pub frozen_prefix: ByteRange,
    pub frozen_block_count: usize,
    #[serde(default)]
    pub mutable_blocks: Vec<ByteRange>,
}

impl LiveZone {
    pub fn new(
        provider: impl Into<String>,
        source: impl Into<String>,
        frozen_prefix: ByteRange,
        mutable_blocks: Vec<ByteRange>,
    ) -> Self {
        Self {
            provider: provider.into(),
            source: source.into(),
            frozen_prefix,
            frozen_block_count: 0,
            mutable_blocks,
        }
    }

    pub fn with_frozen_block_count(mut self, frozen_block_count: usize) -> Self {
        self.frozen_block_count = frozen_block_count;
        self
    }
}

/// Replacement for a host-designated mutable byte range.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LiveZoneReplacement {
    pub range: ByteRange,
    pub text: String,
}

impl LiveZoneReplacement {
    pub fn new(range: ByteRange, text: impl Into<String>) -> Self {
        Self {
            range,
            text: text.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LiveZoneRewrite {
    pub text: String,
    pub replaced_blocks: usize,
    pub frozen_prefix_preserved: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum LiveZoneError {
    #[error("live-zone range is outside the input or splits a UTF-8 character")]
    InvalidRange,
    #[error("live-zone replacement overlaps the frozen prefix")]
    FrozenRangeModified,
    #[error("live-zone replacement is outside host-declared mutable blocks")]
    ReplacementOutsideMutableBlock,
    #[error("live-zone replacements overlap each other")]
    OverlappingReplacements,
}

/// Splice host-supplied replacements into mutable ranges while preserving the
/// exact bytes of the frozen prefix.
pub fn splice_live_zone_replacements(
    input: &str,
    zone: &LiveZone,
    replacements: &[LiveZoneReplacement],
) -> Result<LiveZoneRewrite, LiveZoneError> {
    if !zone.frozen_prefix.is_valid_for(input)
        || zone
            .mutable_blocks
            .iter()
            .any(|range| !range.is_valid_for(input))
    {
        return Err(LiveZoneError::InvalidRange);
    }

    let mut replacements = replacements.to_vec();
    replacements.sort_by_key(|replacement| replacement.range.start);

    let mut previous: Option<ByteRange> = None;
    for replacement in &replacements {
        if !replacement.range.is_valid_for(input) {
            return Err(LiveZoneError::InvalidRange);
        }
        if replacement.range.overlaps(zone.frozen_prefix) {
            return Err(LiveZoneError::FrozenRangeModified);
        }
        if !zone
            .mutable_blocks
            .iter()
            .any(|block| block.contains(replacement.range))
        {
            return Err(LiveZoneError::ReplacementOutsideMutableBlock);
        }
        if previous.is_some_and(|range| range.overlaps(replacement.range)) {
            return Err(LiveZoneError::OverlappingReplacements);
        }
        previous = Some(replacement.range);
    }

    let mut out = String::with_capacity(input.len());
    let mut cursor = 0usize;
    for replacement in &replacements {
        out.push_str(&input[cursor..replacement.range.start]);
        out.push_str(&replacement.text);
        cursor = replacement.range.end;
    }
    out.push_str(&input[cursor..]);

    let frozen_prefix_preserved = input
        .as_bytes()
        .get(zone.frozen_prefix.start..zone.frozen_prefix.end)
        == out
            .as_bytes()
            .get(zone.frozen_prefix.start..zone.frozen_prefix.end);

    Ok(LiveZoneRewrite {
        text: out,
        replaced_blocks: replacements.len(),
        frozen_prefix_preserved,
    })
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum VolatileValueKind {
    Uuid,
    IsoTimestamp,
    Jwt,
    Md5Hex,
    Sha1Hex,
    Sha256Hex,
}

impl VolatileValueKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Uuid => "uuid",
            Self::IsoTimestamp => "iso_timestamp",
            Self::Jwt => "jwt",
            Self::Md5Hex => "md5_hex",
            Self::Sha1Hex => "sha1_hex",
            Self::Sha256Hex => "sha256_hex",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VolatileCacheFinding {
    pub kind: VolatileValueKind,
    pub range: ByteRange,
    pub redacted_sample: String,
}

/// Detect cache-hostile volatile values inside a frozen range. Findings are
/// redacted labels only; this helper never returns raw matched text.
pub fn detect_volatile_cache_values(
    input: &str,
    frozen_range: ByteRange,
) -> Vec<VolatileCacheFinding> {
    if !frozen_range.is_valid_for(input) {
        return Vec::new();
    }
    let haystack = &input[frozen_range.start..frozen_range.end];
    let mut findings = Vec::new();

    collect_regex_findings(
        haystack,
        frozen_range.start,
        &JWT_RE,
        VolatileValueKind::Jwt,
        &mut findings,
    );
    collect_regex_findings(
        haystack,
        frozen_range.start,
        &UUID_RE,
        VolatileValueKind::Uuid,
        &mut findings,
    );
    collect_regex_findings(
        haystack,
        frozen_range.start,
        &ISO_TIMESTAMP_RE,
        VolatileValueKind::IsoTimestamp,
        &mut findings,
    );
    collect_hash_findings(haystack, frozen_range.start, &mut findings);

    findings.sort_by_key(|finding| finding.range.start);
    findings
}

static UUID_RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"(?i)\b[0-9a-f]{8}-[0-9a-f]{4}-[1-5][0-9a-f]{3}-[89ab][0-9a-f]{3}-[0-9a-f]{12}\b")
        .expect("valid uuid regex")
});

static ISO_TIMESTAMP_RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"\b\d{4}-\d{2}-\d{2}[T ][0-2]\d:[0-5]\d(?::[0-5]\d(?:\.\d{1,9})?)?(?:Z|[+-][0-2]\d:[0-5]\d)?\b")
        .expect("valid iso timestamp regex")
});

static JWT_RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"\b[A-Za-z0-9_-]{10,}\.[A-Za-z0-9_-]{10,}\.[A-Za-z0-9_-]{10,}\b")
        .expect("valid jwt regex")
});

static HASH_RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"(?i)\b[0-9a-f]{32}\b|\b[0-9a-f]{40}\b|\b[0-9a-f]{64}\b").expect("valid hash regex")
});

fn collect_regex_findings(
    haystack: &str,
    offset: usize,
    regex: &Regex,
    kind: VolatileValueKind,
    findings: &mut Vec<VolatileCacheFinding>,
) {
    for matched in regex.find_iter(haystack) {
        let range = ByteRange::new(offset + matched.start(), offset + matched.end());
        if overlaps_existing(range, findings) {
            continue;
        }
        findings.push(VolatileCacheFinding {
            kind,
            range,
            redacted_sample: redacted_sample(kind),
        });
    }
}

fn collect_hash_findings(haystack: &str, offset: usize, findings: &mut Vec<VolatileCacheFinding>) {
    for matched in HASH_RE.find_iter(haystack) {
        let range = ByteRange::new(offset + matched.start(), offset + matched.end());
        if overlaps_existing(range, findings) {
            continue;
        }
        let kind = match matched.as_str().len() {
            32 => VolatileValueKind::Md5Hex,
            40 => VolatileValueKind::Sha1Hex,
            64 => VolatileValueKind::Sha256Hex,
            _ => continue,
        };
        findings.push(VolatileCacheFinding {
            kind,
            range,
            redacted_sample: redacted_sample(kind),
        });
    }
}

fn overlaps_existing(range: ByteRange, findings: &[VolatileCacheFinding]) -> bool {
    findings.iter().any(|finding| finding.range.overlaps(range))
}

fn redacted_sample(kind: VolatileValueKind) -> String {
    format!("[{}]", kind.as_str())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn splices_mutable_blocks_without_rewriting_frozen_prefix() {
        let input = r#"{"system":"stable","messages":[{"role":"user","content":"hello world"}]}"#;
        let frozen_end = input.find("\"messages\"").expect("messages key");
        let content_start = input.find("hello world").expect("content");
        let content_end = content_start + "hello world".len();
        let zone = LiveZone::new(
            "anthropic",
            "test",
            ByteRange::new(0, frozen_end),
            vec![ByteRange::new(frozen_end, input.len())],
        );

        let rewrite = splice_live_zone_replacements(
            input,
            &zone,
            &[LiveZoneReplacement::new(
                ByteRange::new(content_start, content_end),
                "hi",
            )],
        )
        .expect("splice succeeds");

        assert_eq!(&rewrite.text[..frozen_end], &input[..frozen_end]);
        assert!(rewrite.frozen_prefix_preserved);
        assert_eq!(rewrite.replaced_blocks, 1);
        assert!(rewrite.text.contains(r#""content":"hi""#));
    }

    #[test]
    fn rejects_replacements_that_touch_frozen_prefix() {
        let input = "frozen mutable";
        let zone = LiveZone::new(
            "provider",
            "test",
            ByteRange::new(0, 6),
            vec![ByteRange::new(7, input.len())],
        );

        let error = splice_live_zone_replacements(
            input,
            &zone,
            &[LiveZoneReplacement::new(ByteRange::new(1, 4), "xxx")],
        )
        .expect_err("frozen replacement rejected");

        assert_eq!(error, LiveZoneError::FrozenRangeModified);
    }

    #[test]
    fn byte_splicing_does_not_require_valid_json() {
        let input = r#"{"messages":[{"content":"alpha"}"#;
        let start = input.find("alpha").expect("alpha");
        let end = start + "alpha".len();
        let zone = LiveZone::new(
            "provider",
            "malformed_json_fixture",
            ByteRange::new(0, 0),
            vec![ByteRange::new(0, input.len())],
        );

        let rewrite = splice_live_zone_replacements(
            input,
            &zone,
            &[LiveZoneReplacement::new(ByteRange::new(start, end), "beta")],
        )
        .expect("byte splice succeeds");

        assert_eq!(rewrite.text, r#"{"messages":[{"content":"beta"}"#);
    }

    #[test]
    fn no_mutable_blocks_and_no_replacements_is_noop() {
        let input = "fully frozen prompt";
        let zone = LiveZone::new(
            "provider",
            "no_marker",
            ByteRange::new(0, input.len()),
            Vec::new(),
        );

        let rewrite = splice_live_zone_replacements(input, &zone, &[]).expect("noop succeeds");

        assert_eq!(rewrite.text, input);
        assert!(rewrite.frozen_prefix_preserved);
        assert_eq!(rewrite.replaced_blocks, 0);
    }

    #[test]
    fn volatile_detector_reports_redacted_findings_only() {
        let uuid = "550e8400-e29b-41d4-a716-446655440000";
        let jwt = "eyJhbGciOiJIUzI1NiJ9.eyJzdWIiOiIxMjM0NTY3ODkwIn0.K7gNU3sdo-OL0wNhqoVWhr3g6s1xYv72ol_7IrW0hFQ";
        let hash = "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef";
        let input = format!("id={uuid} ts=2026-07-06T12:34:56Z jwt={jwt} hash={hash}");

        let findings = detect_volatile_cache_values(&input, ByteRange::new(0, input.len()));

        assert_eq!(findings.len(), 4);
        assert!(findings.iter().any(|f| f.kind == VolatileValueKind::Uuid));
        assert!(
            findings
                .iter()
                .any(|f| f.kind == VolatileValueKind::IsoTimestamp)
        );
        assert!(findings.iter().any(|f| f.kind == VolatileValueKind::Jwt));
        assert!(
            findings
                .iter()
                .any(|f| f.kind == VolatileValueKind::Sha256Hex)
        );
        for finding in findings {
            assert!(finding.redacted_sample.starts_with('['));
            assert!(!finding.redacted_sample.contains(uuid));
            assert!(!finding.redacted_sample.contains(jwt));
            assert!(!finding.redacted_sample.contains(hash));
        }
    }
}
