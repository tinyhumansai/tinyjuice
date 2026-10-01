use super::*;

#[test]
fn agent_profile_labels_are_stable() {
    assert_eq!(AgentTokenjuiceCompression::Auto.as_str(), "auto");
    assert_eq!(AgentTokenjuiceCompression::Full.as_str(), "full");
    assert_eq!(AgentTokenjuiceCompression::Light.as_str(), "light");
    assert_eq!(AgentTokenjuiceCompression::Off.as_str(), "off");
}

#[test]
fn content_kind_labels_are_stable() {
    let labels = [
        (ContentKind::Json, "json"),
        (ContentKind::Code, "code"),
        (ContentKind::Log, "log"),
        (ContentKind::Search, "search"),
        (ContentKind::Diff, "diff"),
        (ContentKind::Html, "html"),
        (ContentKind::PlainText, "plain_text"),
    ];

    for (kind, label) in labels {
        assert_eq!(kind.as_str(), label);
    }
}

#[test]
fn compressor_kind_labels_are_stable() {
    let labels = [
        (CompressorKind::SmartCrusher, "smartcrusher"),
        (CompressorKind::Code, "code"),
        (CompressorKind::Log, "log"),
        (CompressorKind::Search, "search"),
        (CompressorKind::Diff, "diff"),
        (CompressorKind::Html, "html"),
        (CompressorKind::MlText, "ml_text"),
        (CompressorKind::Generic, "generic"),
        (CompressorKind::None, "none"),
    ];

    for (kind, label) in labels {
        assert_eq!(kind.as_str(), label);
    }
}

#[test]
fn content_hint_for_tool_sets_only_source_tool() {
    let hint = ContentHint::for_tool("shell");

    assert_eq!(hint.source_tool.as_deref(), Some("shell"));
    assert!(hint.mime.is_none());
    assert!(hint.extension.is_none());
    assert!(hint.query.is_none());
    assert!(hint.explicit.is_none());
}

#[test]
fn compress_output_constructors_set_lossiness_and_facts() {
    let reformatted = CompressOutput::reformatted("{}".into(), CompressorKind::SmartCrusher);
    assert!(!reformatted.lossy);
    assert_eq!(reformatted.kind, CompressorKind::SmartCrusher);
    assert!(reformatted.facts.is_none());

    let lossy = CompressOutput::lossy("partial".into(), CompressorKind::Log);
    assert!(lossy.lossy);
    assert_eq!(lossy.kind, CompressorKind::Log);
    assert!(lossy.facts.is_none());
}

#[test]
fn compressed_output_passthrough_preserves_lengths_and_flags() {
    let output = CompressedOutput::passthrough("alpha".into(), ContentKind::PlainText);

    assert_eq!(output.text, "alpha");
    assert_eq!(output.content_kind, ContentKind::PlainText);
    assert_eq!(output.compressor, CompressorKind::None);
    assert!(!output.lossy);
    assert!(!output.applied);
    assert!(output.ccr_token.is_none());
    assert_eq!(output.original_bytes, 5);
    assert_eq!(output.compacted_bytes, 5);
}
