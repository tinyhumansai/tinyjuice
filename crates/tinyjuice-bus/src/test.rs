//! Tests that pin the shared `TinyJuice` vocabulary and compatibility rule.

use super::{
    AgentTokenjuiceCompression, CacheStats, CompressOptions, CompressorKind, ContentKind,
    RangeUnit, RetrieveRange, CONTRACT_VERSION, is_compatible,
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
