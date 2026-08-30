//! Tests that pin the shared `TinyJuice` vocabulary and compatibility rule.

use super::{
    AgentTokenjuiceCompression, CONTRACT_VERSION, CacheStats, CompressOptions, CompressorKind,
    ContentKind, RangeUnit, RetrieveRange, is_compatible,
};

#[test]
fn the_contract_accepts_its_own_version_and_newer_minors() {
    assert!(is_compatible(CONTRACT_VERSION));
    assert!(is_compatible((CONTRACT_VERSION.0, CONTRACT_VERSION.1 + 1)));
    assert!(!is_compatible((CONTRACT_VERSION.0 + 1, 0)));
}

#[test]
fn the_agent_profile_keeps_its_snake_case_spelling() {
    // A host writes this into its own config file, so a rename here is a
    // config value that stops parsing on an existing installation.
    for (value, json) in [
        (AgentTokenjuiceCompression::Auto, r#""auto""#),
        (AgentTokenjuiceCompression::Full, r#""full""#),
        (AgentTokenjuiceCompression::Light, r#""light""#),
        (AgentTokenjuiceCompression::Off, r#""off""#),
    ] {
        assert_eq!(serde_json::to_string(&value).unwrap(), json);
        assert_eq!(
            serde_json::from_str::<AgentTokenjuiceCompression>(json).unwrap(),
            value
        );
    }
}

#[test]
fn content_kinds_and_compressors_keep_their_camel_case_spellings() {
    // These two cross the wire inside every `Compress` response, and the
    // compaction path additionally flattens them to strings — so a rename is a
    // response a host decodes as something else, or not at all.
    assert_eq!(
        serde_json::to_string(&ContentKind::PlainText).unwrap(),
        r#""plainText""#
    );
    assert_eq!(
        serde_json::to_string(&CompressorKind::SmartCrusher).unwrap(),
        r#""smartCrusher""#
    );
}

#[test]
fn the_wire_envelopes_keep_their_json_contract() {
    let range = RetrieveRange {
        start: 0,
        end: 10,
        unit: RangeUnit::Lines,
    };
    assert_eq!(
        serde_json::to_string(&range).unwrap(),
        r#"{"start":0,"end":10,"unit":"lines"}"#
    );
    let stats = CacheStats {
        entries: 2,
        bytes: 4096,
    };
    assert_eq!(
        serde_json::to_string(&stats).unwrap(),
        r#"{"entries":2,"bytes":4096}"#
    );
}

#[test]
fn compress_options_round_trip_through_their_defaults() {
    // `CompressOptions` is `#[serde(default)]`, which is what lets a host send
    // only the knobs it cares about. That only holds if every field has a
    // default, and this is what notices when one stops having it.
    let decoded: CompressOptions = serde_json::from_str("{}").unwrap();
    let defaults = CompressOptions::default();
    assert_eq!(
        serde_json::to_value(&decoded).unwrap(),
        serde_json::to_value(&defaults).unwrap()
    );
}

/// Every variant survives `as_str` and back.
///
/// The two directions are what `Compact` needs: it flattens both enums to their
/// `as_str` spelling on the way out, and a caller parses that back. A variant
/// added to one table and not the other is a value that arrives and cannot be
/// read, so the round trip is checked over the whole set rather than a sample.
#[test]
fn the_flattened_spellings_round_trip_for_every_variant() {
    use std::str::FromStr as _;

    for kind in [
        ContentKind::Json,
        ContentKind::Code,
        ContentKind::Log,
        ContentKind::Search,
        ContentKind::Diff,
        ContentKind::Html,
        ContentKind::PlainText,
    ] {
        assert_eq!(ContentKind::from_str(kind.as_str()).unwrap(), kind);
    }

    for compressor in [
        CompressorKind::SmartCrusher,
        CompressorKind::Code,
        CompressorKind::Log,
        CompressorKind::Search,
        CompressorKind::Diff,
        CompressorKind::Html,
        CompressorKind::MlText,
        CompressorKind::Generic,
        CompressorKind::None,
    ] {
        assert_eq!(
            CompressorKind::from_str(compressor.as_str()).unwrap(),
            compressor
        );
    }

    // The flattened spelling is deliberately not the serde one. `plain_text`
    // parses; `plainText` does not, and a host that mixed the two would get a
    // silent `PlainText` fallback on every response.
    assert!(ContentKind::from_str("plainText").is_err());
    assert!(CompressorKind::from_str("smartCrusher").is_err());
}
