//! Tests that pin the shared `TinyJuice` vocabulary and compatibility rule.

use super::*;

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

#[test]
fn the_llm_summary_defaults_to_on_demand() {
    // Ingest must never pay for a model call unless a host opts back in.
    assert_eq!(
        CompressOptions::default().llm_summary_mode,
        LlmSummaryMode::OnDemand
    );
    // A host built before the field existed sends no mode at all.
    let decoded: CompressOptions = serde_json::from_str(r#"{"llmSummaryEnabled":true}"#).unwrap();
    assert_eq!(decoded.llm_summary_mode, LlmSummaryMode::OnDemand);
    for (mode, json) in [
        (LlmSummaryMode::Auto, r#""auto""#),
        (LlmSummaryMode::OnDemand, r#""onDemand""#),
    ] {
        assert_eq!(serde_json::to_string(&mode).unwrap(), json);
        assert_eq!(serde_json::from_str::<LlmSummaryMode>(json).unwrap(), mode);
    }
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
        CompressorKind::TextCrusher,
        CompressorKind::Generic,
        CompressorKind::LlmSummary,
        CompressorKind::Repl,
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

#[test]
fn a_compact_request_needs_only_content_and_tool_name() {
    // Everything past the two positional `Compact` arguments is optional, so a
    // host can move from `Compact` to `CompactWith` without learning the rest.
    let request: CompactRequest =
        serde_json::from_str(r#"{"content":"body","toolName":"web_fetch"}"#).unwrap();
    assert!(request.enabled);
    assert_eq!(request.profile, AgentTokenjuiceCompression::Full);
    assert!(request.focus.is_none());
    assert!(request.context_token.is_none());
    assert!(request.arguments.is_none());
    assert!(request.scope.is_none());
}

#[test]
fn a_generate_request_keeps_its_camel_case_fields() {
    let request = GenerateRequest {
        context_token: "t".into(),
        purpose: "tool_output_summary".into(),
        system: "s".into(),
        prompt: "p".into(),
        max_output_tokens: 7,
    };
    assert_eq!(
        serde_json::to_string(&request).unwrap(),
        r#"{"contextToken":"t","purpose":"tool_output_summary","system":"s","prompt":"p","maxOutputTokens":7}"#
    );
}

#[test]
fn typed_queries_preserve_the_legacy_operation_wire_shape() {
    use crate::repl::{FindMode, ReplOp, ScopeUnit};
    use crate::wire::{QueryRequest, QueryTarget};
    let value = serde_json::json!({
        "target": {"kind": "handle", "token": "abc"},
        "op": {"op": "find", "query": "needle"}
    });
    let request: QueryRequest = serde_json::from_value(value).unwrap();
    assert_eq!(
        request.target,
        QueryTarget::Handle {
            token: "abc".into()
        }
    );
    assert_eq!(request.limits, crate::repl::ReplLimits::default());
    assert_eq!(
        request.op,
        ReplOp::Find {
            query: "needle".into(),
            mode: FindMode::Text,
            ignore_case: false,
            context: 0,
            top_k: None,
            scope: None,
            unit: ScopeUnit::Lines,
        }
    );
    let round_trip: QueryRequest =
        serde_json::from_value(serde_json::to_value(&request).unwrap()).unwrap();
    assert_eq!(round_trip, request);
    assert!(
        !is_compatible((1, 1)),
        "typed query hosts need contract 1.2"
    );
}

#[test]
fn query_replies_distinguish_operation_errors_from_success() {
    use crate::wire::{QueryError, QueryResponse};
    let reply: QueryResponse = Err(QueryError::HandleNotFound);
    assert_eq!(
        serde_json::to_value(&reply).unwrap(),
        serde_json::json!({"Err": {"kind": "handle_not_found"}})
    );
    let parsed: QueryResponse =
        serde_json::from_value(serde_json::to_value(&reply).unwrap()).unwrap();
    assert_eq!(reply, parsed);
    let reply: QueryResponse = Ok(crate::repl::ReplOutput::Text {
        text: "overview".into(),
    });
    assert_eq!(
        serde_json::to_value(reply).unwrap(),
        serde_json::json!({"Ok": {"kind": "text", "text": "overview"}})
    );
}

#[test]
fn declarations_round_trip_and_keep_the_existing_tool_names() {
    let declarations = crate::tools::repl_tool_declarations();
    assert_eq!(
        declarations
            .iter()
            .map(|tool| tool.name.as_str())
            .collect::<Vec<_>>(),
        ["juice_find", "juice_extract", "juice_summarize"]
    );
    for declaration in declarations {
        let decoded: crate::tools::ReplToolDeclaration =
            serde_json::from_value(serde_json::to_value(&declaration).unwrap()).unwrap();
        assert_eq!(decoded, declaration);
        assert_eq!(declaration.parameters["required"][0], "handle");
    }
}
