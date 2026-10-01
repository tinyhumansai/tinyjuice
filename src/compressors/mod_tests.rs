use super::*;

#[test]
fn registry_maps_all_content_kinds_to_expected_compressors() {
    let cases = [
        (ContentKind::Json, CompressorKind::SmartCrusher),
        (ContentKind::Code, CompressorKind::Code),
        (ContentKind::Log, CompressorKind::Log),
        (ContentKind::Search, CompressorKind::Search),
        (ContentKind::Diff, CompressorKind::Diff),
        (ContentKind::Html, CompressorKind::Html),
        (ContentKind::PlainText, CompressorKind::TextCrusher),
    ];

    for (kind, expected) in cases {
        assert_eq!(compressor_for(kind).kind(), expected);
    }
    assert_eq!(generic_compressor().kind(), CompressorKind::Generic);
}
