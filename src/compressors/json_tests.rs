use super::*;
use crate::cache::{CcrStore, MemoryCcrStore};

#[test]
fn crushes_uniform_array() {
    let mut rows = Vec::new();
    for i in 0..20 {
        rows.push(format!(
            r#"{{"id":{i},"name":"item number {i}","status":"active","owner":"team-alpha"}}"#
        ));
    }
    let input = format!("[{}]", rows.join(","));
    let out = compress(&input).expect("compresses").text;
    assert_eq!(out.matches("status").count(), 1, "{out}");
    assert!(out.contains("item number 7"));
    assert!(out.len() < input.len(), "expected shrink");
}

#[test]
fn smartcrusher_table_transform_runs_without_ccr() {
    let mut rows = Vec::new();
    for i in 0..20 {
        rows.push(format!(
            r#"{{"id":{i},"name":"item number {i}","status":"active","owner":"team-alpha"}}"#
        ));
    }
    let input = format!("[{}]", rows.join(","));
    let pipeline_input = PipelineInput {
        content: &input,
        original_content: &input,
        content_kind: ContentKind::Json,
        original_bytes: input.len(),
    };
    let transform = SmartCrusherTableTransform;

    assert!(transform.applies_to(&pipeline_input));
    let out = transform.apply(&pipeline_input).expect("table reformat");

    assert_eq!(out.kind, CompressorKind::SmartCrusher);
    assert!(out.text.contains("item number 7"), "{}", out.text);
    assert!(out.text.len() < input.len(), "{}", out.text);
}

#[test]
fn smartcrusher_rows_transform_requires_retained_ccr() {
    let mut rows = Vec::new();
    for i in 0..140 {
        let note = if i == 71 {
            "contains special needle token"
        } else {
            "ordinary row"
        };
        rows.push(format!(
            r#"{{"id":{i},"name":"record {i}","status":"active","note":"{note}"}}"#
        ));
    }
    let input = format!("[{}]", rows.join(","));
    let pipeline_input = PipelineInput {
        content: &input,
        original_content: &input,
        content_kind: ContentKind::Json,
        original_bytes: input.len(),
    };
    let transform = SmartCrusherRowsTransform::new().with_query("special needle");

    let rejecting_store = MemoryCcrStore::new(1, 1);
    assert!(transform.apply(&pipeline_input, &rejecting_store).is_none());

    let store = MemoryCcrStore::default();
    let out = transform
        .apply(&pipeline_input, &store)
        .expect("retained SmartCrusher rows");

    assert_eq!(out.kind(), CompressorKind::SmartCrusher);
    assert!(
        out.text().contains("special needle token"),
        "{}",
        out.text()
    );
    assert_eq!(store.get(out.token()).as_deref(), Some(input.as_str()));
}

#[test]
fn smartcrusher_rows_transform_skips_reformats_and_non_json_input() {
    let mut rows = Vec::new();
    for i in 0..20 {
        rows.push(format!(
            r#"{{"id":{i},"name":"item number {i}","status":"active","owner":"team-alpha"}}"#
        ));
    }
    let small_json = format!("[{}]", rows.join(","));
    let small_input = PipelineInput {
        content: &small_json,
        original_content: &small_json,
        content_kind: ContentKind::Json,
        original_bytes: small_json.len(),
    };
    let plain_input = PipelineInput {
        content: "plain text",
        original_content: "plain text",
        content_kind: ContentKind::PlainText,
        original_bytes: "plain text".len(),
    };
    let transform = SmartCrusherRowsTransform::new();
    let store = MemoryCcrStore::default();

    assert!(transform.apply(&small_input, &store).is_none());
    assert_eq!(transform.estimate_bloat(&plain_input), 0.0);
    assert!(transform.apply(&plain_input, &store).is_none());
}

#[test]
fn large_array_row_drops_and_is_marked_lossy() {
    let mut rows = Vec::new();
    for i in 0..200 {
        rows.push(format!(
            r#"{{"id":{i},"name":"record number {i}","status":"active","note":"some detail {i}"}}"#
        ));
    }
    let input = format!("[{}]", rows.join(","));
    let c = compress(&input).expect("compresses");
    assert!(c.lossy, "row-dropped output must be lossy");
    assert!(c.text.contains("record number 0"), "{}", c.text);
    assert!(c.text.contains("record number 199"), "{}", c.text);
    assert!(c.text.contains("omitted"));
    assert!(c.text.len() < input.len());
}

#[test]
fn analyzer_reports_constants_sparse_fields_and_numeric_stats() {
    let mut rows = Vec::new();
    for i in 0..20 {
        let extra = if i == 13 {
            r#","debug":"only here""#
        } else {
            ""
        };
        rows.push(format!(
            r#"{{"id":{i},"status":"active","latency":{}{extra}}}"#,
            10 + i
        ));
    }
    let input = format!("[{}]", rows.join(","));

    let analysis = analyze(&input).expect("analysis");
    let status = analysis
        .fields
        .iter()
        .find(|field| field.key == "status")
        .expect("status field");
    let debug = analysis
        .fields
        .iter()
        .find(|field| field.key == "debug")
        .expect("debug field");
    let latency = analysis
        .fields
        .iter()
        .find(|field| field.key == "latency")
        .expect("latency field");

    assert!(status.constant);
    assert_eq!(status.unique_ratio(), 1.0 / 20.0);
    assert!(debug.sparse);
    assert_eq!(debug.present, 1);
    assert_eq!(debug.unique_ratio(), 1.0);
    assert_eq!(latency.numeric.expect("numeric").min, 10.0);
    assert!(latency.unique_ratio() > 0.9);
    assert!(analysis.estimated_table_bytes < input.len());
    assert!(analysis.estimated_reduction_bytes(input.len()) > 0);
    assert!(analysis.estimated_reduction_ratio(input.len()) > 0.0);
}

#[test]
fn constant_columns_are_hoisted_out_of_rows() {
    let mut rows = Vec::new();
    for i in 0..20 {
        rows.push(format!(
            r#"{{"id":{i},"status":"active","region":"us-east","latency":{}}}"#,
            10 + i
        ));
    }
    let input = format!("[{}]", rows.join(","));

    let c = compress(&input).expect("compresses");

    assert!(!c.lossy);
    assert!(c.text.contains("constants: region=us-east | status=active"));
    assert!(c.text.contains("id | latency"), "{}", c.text);
    assert!(!c.text.contains("id | latency | region"), "{}", c.text);
    assert_eq!(c.text.matches("active").count(), 1, "{}", c.text);
    assert_eq!(c.text.matches("us-east").count(), 1, "{}", c.text);
}

#[test]
fn query_anchor_row_survives_dropped_middle() {
    let mut rows = Vec::new();
    for i in 0..140 {
        let note = if i == 71 {
            "contains special needle token"
        } else {
            "ordinary row"
        };
        rows.push(format!(
            r#"{{"id":{i},"name":"record {i}","status":"active","note":"{note}"}}"#
        ));
    }
    let input = format!("[{}]", rows.join(","));

    let c = compress_with_query(&input, Some("special needle")).expect("compresses");

    assert!(c.lossy);
    assert!(c.text.contains("special needle token"), "{}", c.text);
}

#[test]
fn latest_query_extends_tail_anchor_window() {
    let mut rows = Vec::new();
    for i in 0..140 {
        let note = if i == 124 {
            "latest positional context".to_string()
        } else {
            format!("ordinary row {i}")
        };
        rows.push(format!(
            r#"{{"id":{i},"name":"record {i}","status":"active","note":"{note}"}}"#
        ));
    }
    let input = format!("[{}]", rows.join(","));

    let c = compress_with_query(&input, Some("latest records")).expect("compresses");

    assert!(c.lossy);
    assert!(c.text.contains("latest positional context"), "{}", c.text);
}

#[test]
fn oldest_query_extends_head_anchor_window() {
    let mut rows = Vec::new();
    for i in 0..140 {
        let note = if i == 24 {
            "oldest positional context".to_string()
        } else {
            format!("ordinary row {i}")
        };
        rows.push(format!(
            r#"{{"id":{i},"name":"record {i}","status":"active","note":"{note}"}}"#
        ));
    }
    let input = format!("[{}]", rows.join(","));

    let c = compress_with_query(&input, Some("oldest records")).expect("compresses");

    assert!(c.lossy);
    assert!(c.text.contains("oldest positional context"), "{}", c.text);
}

#[test]
fn sparse_structural_row_survives_dropped_middle() {
    let mut rows = Vec::new();
    for i in 0..140 {
        let extra = if i == 72 {
            r#","diagnostic":"rare sparse field""#
        } else {
            ""
        };
        rows.push(format!(
            r#"{{"id":{i},"name":"record {i}","status":"active"{extra}}}"#
        ));
    }
    let input = format!("[{}]", rows.join(","));

    let c = compress(&input).expect("compresses");

    assert!(c.lossy);
    assert!(c.text.contains("rare sparse field"), "{}", c.text);
}

#[test]
fn discriminator_bucket_row_survives_dropped_middle() {
    let mut rows = Vec::new();
    for i in 0..140 {
        let kind = if i == 72 {
            "audit"
        } else if i % 2 == 0 {
            "service"
        } else {
            "worker"
        };
        let message = if i == 72 {
            "audit bucket unique payload"
        } else {
            "ordinary payload"
        };
        rows.push(format!(
            r#"{{"id":{i},"kind":"{kind}","status":"active","message":"{message}"}}"#
        ));
    }
    let input = format!("[{}]", rows.join(","));

    let c = compress(&input).expect("compresses");

    assert!(c.lossy);
    assert!(c.text.contains("audit"), "{}", c.text);
    assert!(c.text.contains("audit bucket unique payload"), "{}", c.text);
}

#[test]
fn duplicate_cluster_representative_survives_dropped_middle() {
    let mut rows = Vec::new();
    for i in 0..140 {
        if (72..=76).contains(&i) {
            rows.push(
                r#"{"id":"retry-batch-17","status":"retry","message":"duplicate cluster payload"}"#
                    .to_string(),
            );
        } else {
            rows.push(format!(
                r#"{{"id":"job-{i}","status":"ok","message":"ordinary payload {i}"}}"#
            ));
        }
    }
    let input = format!("[{}]", rows.join(","));

    let c = compress(&input).expect("compresses");

    assert!(c.lossy);
    assert!(c.text.contains("retry-batch-17"), "{}", c.text);
    assert!(c.text.contains("duplicate cluster payload"), "{}", c.text);
}

#[test]
fn near_duplicate_cluster_representative_survives_dropped_middle() {
    let mut rows = Vec::new();
    for i in 0..140 {
        let message = if (72..=76).contains(&i) {
            "retry backlog service failed"
        } else {
            "routine healthy service stable"
        };
        rows.push(format!(
            r#"{{"id":"job-{i}","status":"active","message":"{message}"}}"#
        ));
    }
    let input = format!("[{}]", rows.join(","));

    let c = compress(&input).expect("compresses");

    assert!(c.lossy);
    assert!(c.text.contains("job-72"), "{}", c.text);
    assert!(
        c.text.contains("retry backlog service failed"),
        "{}",
        c.text
    );
}

#[test]
fn spread_anchor_row_survives_dropped_middle() {
    let mut rows = Vec::new();
    for i in 0..140 {
        let message = if i == 94 {
            "deterministic spread anchor payload".to_string()
        } else {
            format!("ordinary payload {i}")
        };
        rows.push(format!(
            r#"{{"id":{i},"name":"record {i}","status":"active","message":"{message}"}}"#
        ));
    }
    let input = format!("[{}]", rows.join(","));

    let c = compress(&input).expect("compresses");

    assert!(c.lossy);
    assert!(
        c.text.contains("deterministic spread anchor payload"),
        "{}",
        c.text
    );
}

#[test]
fn adaptive_saturation_extends_spread_anchors() {
    let mut rows = Vec::new();
    for i in 0..140 {
        let word = alpha_word(i);
        rows.push(format!(
            r#"{{"id":{i},"status":"active","message":"topic {word} marker alpha beta"}}"#
        ));
    }
    let input = format!("[{}]", rows.join(","));
    let expected = alpha_word(108);

    let c = compress(&input).expect("compresses");

    assert!(c.lossy);
    assert!(c.text.contains(&expected), "{}", c.text);
}

#[test]
fn early_saturation_keeps_base_spread_anchors() {
    let mut rows = Vec::new();
    for i in 0..140 {
        let message = if i < 70 {
            format!("topic {} marker alpha beta", alpha_word(i))
        } else {
            "topic repeated marker alpha beta".to_string()
        };
        rows.push(format!(
            r#"{{"id":"job-{i}","status":"active","message":"{message}"}}"#
        ));
    }
    let input = format!("[{}]", rows.join(","));

    let c = compress(&input).expect("compresses");

    assert!(c.lossy);
    assert!(c.text.contains("job-94"), "{}", c.text);
}

fn alpha_word(mut n: usize) -> String {
    let mut out = String::from("word");
    loop {
        out.push((b'a' + (n % 26) as u8) as char);
        n /= 26;
        if n == 0 {
            break;
        }
    }
    out
}

#[test]
fn numeric_change_point_row_survives_dropped_middle() {
    let mut rows = Vec::new();
    for i in 0..140 {
        let phase = if i < 72 { 10 } else { 30 };
        let message = if i == 72 {
            "phase transition payload".to_string()
        } else {
            format!("ordinary payload {i}")
        };
        rows.push(format!(
            r#"{{"id":{i},"phase_score":{phase},"status":"active","message":"{message}"}}"#
        ));
    }
    let input = format!("[{}]", rows.join(","));

    let c = compress(&input).expect("compresses");

    assert!(c.lossy);
    assert!(c.text.contains("phase transition payload"), "{}", c.text);
}

#[test]
fn information_dense_row_survives_dropped_middle() {
    let mut rows = Vec::new();
    for i in 0..140 {
        let message = if i == 72 {
            "trace alpha beta gamma delta epsilon zeta eta theta iota kappa lambda".to_string()
        } else {
            format!("ordinary payload {i}")
        };
        rows.push(format!(
            r#"{{"id":{i},"status":"active","message":"{message}"}}"#
        ));
    }
    let input = format!("[{}]", rows.join(","));

    let c = compress(&input).expect("compresses");

    assert!(c.lossy);
    assert!(c.text.contains("trace alpha beta gamma"), "{}", c.text);
}

#[test]
fn nested_objects_flatten_to_dotted_columns() {
    let mut rows = Vec::new();
    for i in 0..20 {
        rows.push(format!(
            r#"{{"id":{i},"user":{{"name":"user {i}","team":"core"}},"status":"active"}}"#
        ));
    }
    let input = format!("[{}]", rows.join(","));

    let c = compress(&input).expect("compresses");

    assert!(c.text.contains("user.name"), "{}", c.text);
    assert!(c.text.contains("user.team"), "{}", c.text);
    assert!(c.text.contains("user 7"), "{}", c.text);
}

#[test]
fn stringified_json_objects_flatten_to_dotted_columns() {
    let mut rows = Vec::new();
    for i in 0..20 {
        let metadata = serde_json::json!({
            "owner": format!("team-{i}"),
            "flags": { "retry": i % 2 == 0 }
        })
        .to_string()
        .replace('"', "\\\"");
        rows.push(format!(
            r#"{{"id":{i},"metadata":"{metadata}","status":"active"}}"#
        ));
    }
    let input = format!("[{}]", rows.join(","));

    let c = compress(&input).expect("compresses");

    assert!(c.text.contains("metadata.owner"), "{}", c.text);
    assert!(c.text.contains("metadata.flags.retry"), "{}", c.text);
    assert!(c.text.contains("team-7"), "{}", c.text);
}

#[test]
fn stringified_json_parsing_leaves_scalar_and_invalid_strings_opaque() {
    let input = r#"[
            {"id":1,"payload":"001","note":"{not json}"},
            {"id":2,"payload":"002","note":"[also not json]"},
            {"id":3,"payload":"003","note":"plain"}
        ]"#;

    let c = compress(input).expect("compresses");

    assert!(c.text.contains("payload"), "{}", c.text);
    assert!(c.text.contains("001"), "{}", c.text);
    assert!(c.text.contains("{not json}"), "{}", c.text);
    assert!(!c.text.contains("note."), "{}", c.text);
}

#[test]
fn heterogeneous_shapes_render_smaller_bucket_tables() {
    let mut rows = Vec::new();
    for i in 0..12 {
        rows.push(format!(
            r#"{{"event":"login","user_id":"user-{i}","ip":"10.0.0.{i}","success":true}}"#
        ));
        rows.push(format!(
            r#"{{"event":"deploy","service":"api-{i}","version":"2026.{i}.0","region":"us-east"}}"#
        ));
        rows.push(format!(
            r#"{{"event":"metric","name":"cpu-{i}","value":{},"unit":"pct"}}"#,
            40 + i
        ));
    }
    let input = format!("[{}]", rows.join(","));

    let c = compress(&input).expect("compresses");

    assert!(!c.lossy);
    assert!(c.text.contains("[json bucketed tables:"), "{}", c.text);
    assert!(c.text.contains("[bucket 1:"), "{}", c.text);
    assert!(c.text.contains("[bucket 2:"), "{}", c.text);
    assert!(c.text.contains("[bucket 3:"), "{}", c.text);
    assert!(c.text.contains("constants: event=login | success=true"));
    assert!(c.text.contains("constants: event=deploy | region=us-east"));
    assert!(c.text.contains("constants: event=metric | unit=pct"));
    assert!(c.text.contains("__row | ip | user_id"));
    assert!(c.text.contains("__row | service | version"));
    assert!(c.text.contains("__row | name | value"));
    assert!(c.text.contains("31 | api-10 | 2026.10.0"));
}

#[test]
fn lossy_heterogeneous_bucket_marks_omissions_and_recovery() {
    let mut rows = Vec::new();
    for i in 0..90 {
        rows.push(format!(
            r#"{{"event":"login","user_id":"user-{i}","ip":"10.0.0.{i}","success":true}}"#
        ));
    }
    for i in 0..90 {
        rows.push(format!(
            r#"{{"event":"metric","name":"cpu-{i}","value":{},"unit":"pct"}}"#,
            40 + i
        ));
    }
    let input = format!("[{}]", rows.join(","));

    let value: Value = serde_json::from_str(&input).unwrap();
    let flat_rows = flatten_rows(value.as_array().unwrap()).unwrap();
    let c = compress_shape_buckets(&input, &flat_rows, None).expect("bucketed");

    assert!(c.lossy);
    assert!(c.text.contains("[json bucketed tables:"), "{}", c.text);
    assert!(c.text.contains("exact original via retrieve footer"));
    assert!(c.text.contains("row(s) omitted in bucket"), "{}", c.text);
    assert!(c.text.contains("constants: event=login | success=true"));
    assert!(c.text.contains("0 | 10.0.0.0 | user-0"), "{}", c.text);
    assert!(c.text.contains("constants: event=metric | unit=pct"));
    assert!(c.text.contains("179 | cpu-89 | 129"), "{}", c.text);
}

#[test]
fn keeps_error_row_in_dropped_middle() {
    // A homogeneous array with a single error row buried in the middle: the
    // SmartCrusher must keep that row even though it's in the drop window.
    let mut rows = Vec::new();
    for i in 0..120 {
        let status = if i == 75 { "error: timeout" } else { "ok" };
        rows.push(format!(
            r#"{{"id":{i},"name":"job {i}","status":"{status}","note":"detail {i}"}}"#
        ));
    }
    let input = format!("[{}]", rows.join(","));
    let c = compress(&input).expect("compresses");
    assert!(c.lossy);
    assert!(
        c.text.contains("job 75"),
        "error row must survive:\n{}",
        c.text
    );
    assert!(c.text.contains("error: timeout"));
}

#[test]
fn keeps_numeric_outlier_row() {
    let mut rows = Vec::new();
    for i in 0..120 {
        // Most latencies ~10ms; row 88 is a 9999ms outlier.
        let latency = if i == 88 { 9999 } else { 10 + (i % 3) };
        rows.push(format!(
            r#"{{"id":{i},"endpoint":"/api/{i}","latency_ms":{latency},"region":"us"}}"#
        ));
    }
    let input = format!("[{}]", rows.join(","));
    let c = compress(&input).expect("compresses");
    assert!(
        c.text.contains("9999"),
        "outlier row must survive:\n{}",
        c.text
    );
}

#[test]
fn non_array_returns_none() {
    assert!(compress(r#"{"a":1}"#).is_none());
    assert!(compress("[1,2,3]").is_none());
    assert!(compress(r#"[{"a":1}]"#).is_none());
}
