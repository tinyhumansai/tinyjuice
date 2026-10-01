use super::*;
use crate::cache::{CcrStore, MemoryCcrStore};
use crate::types::{CompressOptions, ContentHint, ContentKind};

fn noisy_log() -> String {
    let mut s = String::new();
    for i in 0..200 {
        let _ = writeln!(s, "   Compiling crate_{i} v0.1.0");
    }
    let _ = writeln!(s, "error[E0382]: borrow of moved value `x`");
    let _ = writeln!(s, "  --> src/main.rs:10:5");
    let _ = writeln!(s, "error: aborting due to previous error");
    let _ = writeln!(s, "test result: FAILED. 3 passed; 1 failed");
    s
}

fn repetitive_template_log() -> String {
    let mut s = String::new();
    for i in 0..80 {
        let _ = writeln!(
            s,
            "2026-07-05T12:{:02}:00Z worker-{i} processed item id={} shard={}",
            i % 60,
            10_000 + i,
            i % 8
        );
    }
    s
}

fn reconstruct_template_reformat(output: &str) -> String {
    let mut out = String::new();
    let lines: Vec<&str> = output.lines().collect();
    let mut i = 0usize;
    while i < lines.len() {
        if lines[i].starts_with("[TOKENJUICE LOG TEMPLATE ") {
            let template = lines[i + 1];
            i += 2;
            while i < lines.len() && lines[i] != "[/TOKENJUICE LOG TEMPLATE]" {
                let captures = lines[i]
                    .split('\t')
                    .map(unescape_capture_for_test)
                    .collect::<Vec<_>>();
                let _ = writeln!(out, "{}", apply_template_for_test(template, &captures));
                i += 1;
            }
            i += 1;
        } else {
            let _ = writeln!(out, "{}", lines[i]);
            i += 1;
        }
    }
    out
}

fn unescape_capture_for_test(value: &str) -> String {
    let mut out = String::new();
    let mut chars = value.chars();
    while let Some(ch) = chars.next() {
        if ch == '\\' {
            match chars.next() {
                Some('\\') => out.push('\\'),
                Some('t') => out.push('\t'),
                Some('r') => out.push('\r'),
                Some(other) => {
                    out.push('\\');
                    out.push(other);
                }
                None => out.push('\\'),
            }
        } else {
            out.push(ch);
        }
    }
    out
}

fn apply_template_for_test(template: &str, captures: &[String]) -> String {
    let mut out = String::new();
    let mut rest = template;
    for capture in captures {
        let Some((head, tail)) = rest.split_once("{}") else {
            break;
        };
        out.push_str(head);
        out.push_str(capture);
        rest = tail;
    }
    out.push_str(rest);
    out
}

#[test]
fn template_reformat_is_lossless_and_smaller() {
    let input = repetitive_template_log();
    let out = compress_templates(&input).expect("template reformat");

    assert!(!out.lossy);
    assert!(out.text.contains("[TOKENJUICE LOG TEMPLATE run=80"));
    assert!(out.text.len() < input.len(), "{}", out.text);
    assert_eq!(
        reconstruct_template_reformat(&out.text).trim_end(),
        input.trim_end()
    );
}

#[tokio::test]
async fn template_reformat_routes_without_ccr() {
    let input = repetitive_template_log();
    let hint = ContentHint {
        explicit: Some(ContentKind::Log),
        ..Default::default()
    };
    let opts = CompressOptions {
        min_bytes_to_compress: 64,
        ccr_min_tokens: usize::MAX,
        ..Default::default()
    };

    let out = crate::compress_content(&input, Some(hint), &opts).await;

    assert!(out.applied);
    assert!(!out.lossy);
    assert!(out.ccr_token.is_none());
    assert!(out.text.contains("[TOKENJUICE LOG TEMPLATE"));
}

#[test]
fn template_reformat_transform_runs_without_ccr() {
    let input = repetitive_template_log();
    let pipeline_input = PipelineInput {
        content: &input,
        original_content: &input,
        content_kind: ContentKind::Log,
        original_bytes: input.len(),
    };
    let transform = LogTemplateTransform;

    assert!(transform.applies_to(&pipeline_input));
    let out = transform.apply(&pipeline_input).expect("template reformat");

    assert_eq!(out.kind, CompressorKind::Log);
    assert!(
        out.text.contains("[TOKENJUICE LOG TEMPLATE"),
        "{}",
        out.text
    );
    assert_eq!(
        reconstruct_template_reformat(&out.text).trim_end(),
        input.trim_end()
    );
}

#[test]
fn template_reformat_transform_skips_non_log_input() {
    let input = PipelineInput {
        content: "plain text",
        original_content: "plain text",
        content_kind: ContentKind::PlainText,
        original_bytes: "plain text".len(),
    };
    let transform = LogTemplateTransform;

    assert!(!transform.applies_to(&input));
}

#[test]
fn signal_log_transform_requires_retained_ccr() {
    let input = noisy_log();
    let pipeline_input = PipelineInput {
        content: &input,
        original_content: &input,
        content_kind: ContentKind::Log,
        original_bytes: input.len(),
    };
    let transform = SignalLogTransform;

    let rejecting_store = MemoryCcrStore::new(1, 1);
    assert!(transform.apply(&pipeline_input, &rejecting_store).is_none());

    let store = MemoryCcrStore::default();
    let out = transform
        .apply(&pipeline_input, &store)
        .expect("retained signal log");

    assert_eq!(out.kind(), CompressorKind::Log);
    assert!(out.text().contains("error[E0382]"), "{}", out.text());
    assert_eq!(store.get(out.token()).as_deref(), Some(input.as_str()));
}

#[test]
fn signal_log_transform_skips_non_log_input() {
    let input = PipelineInput {
        content: "plain text",
        original_content: "plain text",
        content_kind: ContentKind::PlainText,
        original_bytes: "plain text".len(),
    };
    let transform = SignalLogTransform;
    let store = MemoryCcrStore::default();

    assert_eq!(transform.estimate_bloat(&input), 0.0);
    assert!(transform.apply(&input, &store).is_none());
}

#[test]
fn template_reformat_declines_without_dynamic_fields() {
    let mut input = String::new();
    for _ in 0..40 {
        let _ = writeln!(input, "plain repeated line without dynamic fields");
    }

    assert!(compress_templates(&input).is_none());
}

#[test]
fn template_reformat_declines_low_context_dynamic_lines() {
    let mut input = String::new();
    for i in 0..40 {
        let _ = writeln!(input, "id={} value={}", 10_000 + i, 20_000 + i);
    }

    assert!(compress_templates(&input).is_none());
}

#[test]
fn signal_keeps_errors_and_summary_drops_noise() {
    let input = noisy_log();
    let out = compress_signal(&input).expect("compresses").text;
    assert!(out.contains("error[E0382]"), "{out}");
    assert!(out.contains("error: aborting"), "{out}");
    assert!(out.contains("test result: FAILED"), "{out}");
    assert!(out.len() < input.len());
    assert!(out.contains("omitted"));
}

#[test]
fn non_log_data_passes_through() {
    let mut s = String::new();
    for i in 0..400 {
        let _ = writeln!(s, "/var/data/file_{i:04}.bin\t{i}\trwxr-xr-x");
    }
    assert!(compress_signal(&s).is_none());
}
