//! JSON-array crusher (SmartCrusher).
//!
//! Clean-room port of Headroom's `SmartCrusher` (Apache-2.0), extended with
//! variance-aware row preservation. An array of objects that repeat the same
//! keys is the single most common bloated tool output (API list responses, DB
//! rows, search manifests). Re-rendering it as a table emits each key **once**
//! instead of per row.
//!
//! Up to [`ROW_DROP_THRESHOLD`] rows every value is preserved (faithful
//! reformat). Above the threshold the table is row-dropped — but rather than a
//! blind head/tail window, rows carrying **error indicators** or **numeric
//! outliers** are always kept, so anomalies survive even when the bulk of a
//! homogeneous array is dropped. The router offloads the full original to CCR,
//! so the dropped rows stay recoverable.

use async_trait::async_trait;
use serde_json::{Map, Value};
use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::fmt::Write as _;

use super::Compressor;
use super::signals::has_error_indicators;
use crate::relevance::Bm25Corpus;
use crate::types::{CompressInput, CompressOptions, CompressOutput, CompressorKind};

/// Minimum rows before tabular rendering is worth the header overhead.
pub const MIN_ROWS: usize = 3;
/// Above this many rows the table is additionally row-dropped.
pub const ROW_DROP_THRESHOLD: usize = 40;
/// Rows kept from the head when row-dropping.
pub const HEAD_ROWS: usize = 20;
/// Rows kept from the tail when row-dropping.
pub const TAIL_ROWS: usize = 10;
/// Z-score beyond which a numeric cell is treated as an outlier worth keeping.
pub const OUTLIER_SIGMA: f64 = 2.0;
/// Maximum distinct values for a full-present field to be treated as a
/// discriminator bucket anchor.
pub const MAX_DISCRIMINATOR_BUCKETS: usize = 12;
/// Maximum exact duplicate row clusters to anchor while row-dropping.
pub const MAX_DUPLICATE_CLUSTER_ANCHORS: usize = 20;
/// Maximum evenly-spaced middle anchors to add for large generic arrays.
pub const MAX_SPREAD_ANCHORS: usize = 8;
/// Do not spend unbounded parse effort on huge string cells.
const MAX_STRINGIFIED_JSON_BYTES: usize = 16 * 1024;

pub struct JsonCompressor;

#[async_trait]
impl Compressor for JsonCompressor {
    fn kind(&self) -> CompressorKind {
        CompressorKind::SmartCrusher
    }

    async fn compress(
        &self,
        input: &CompressInput<'_>,
        _opts: &CompressOptions,
    ) -> Option<CompressOutput> {
        compress_with_query(input.content, input.hint.query.as_deref())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct JsonAnalysis {
    pub row_count: usize,
    pub columns: Vec<String>,
    pub fields: Vec<JsonFieldAnalysis>,
    pub estimated_table_bytes: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub struct JsonFieldAnalysis {
    pub key: String,
    pub present: usize,
    pub types: BTreeSet<&'static str>,
    pub unique_count: usize,
    pub constant: bool,
    pub sparse: bool,
    pub numeric: Option<JsonNumericStats>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct JsonNumericStats {
    pub min: f64,
    pub max: f64,
    pub mean: f64,
    pub stddev: f64,
}

#[derive(Debug, Clone)]
struct JsonPlan {
    lossy: bool,
    keep_rows: Vec<usize>,
}

/// Compress a JSON array-of-objects into a compact table. Returns `None` when
/// the content isn't a uniform-enough array of objects or wouldn't shrink.
pub fn compress(content: &str) -> Option<CompressOutput> {
    compress_with_query(content, None)
}

/// Compress a JSON array-of-objects with optional query context for row anchors.
pub fn compress_with_query(content: &str, query: Option<&str>) -> Option<CompressOutput> {
    let value: Value = serde_json::from_str(content.trim()).ok()?;
    let array = value.as_array()?;
    if array.len() < MIN_ROWS {
        return None;
    }
    if !array.iter().all(Value::is_object) {
        return None;
    }

    let flat_rows = flatten_rows(array)?;
    let analysis = analyze_flat_rows(&flat_rows);
    let columns = analysis.columns.clone();
    if columns.len() < 2 {
        return None;
    }

    // Render every row's cells up front so we can choose full vs. row-dropped.
    let mut rows: Vec<String> = Vec::with_capacity(flat_rows.len());
    for obj in &flat_rows {
        let cells: Vec<String> = columns
            .iter()
            .map(|col| match obj.values.get(col) {
                None => String::new(),
                Some(v) => render_cell(v),
            })
            .collect();
        rows.push(cells.join(" | "));
    }

    let plan = plan_rows(&flat_rows, &analysis, query);
    let mut out = String::with_capacity(content.len());
    let _ = writeln!(
        out,
        "[json table: {} rows × {} cols · blank=absent key · exact original via retrieve footer]",
        rows.len(),
        columns.len()
    );
    let _ = writeln!(out, "{}", columns.join(" | "));

    if plan.lossy {
        // Keep head + tail PLUS any anomalous rows (errors / numeric outliers)
        // so the signal in a large homogeneous array survives row-dropping.
        let mut prev: Option<usize> = None;
        for &i in &plan.keep_rows {
            if let Some(p) = prev {
                let gap = i - p - 1;
                if gap > 0 {
                    let _ = writeln!(out, "[... {gap} row(s) omitted ...]");
                }
            } else if i > 0 {
                let _ = writeln!(out, "[... {i} row(s) omitted ...]");
            }
            let _ = writeln!(out, "{}", rows[i]);
            prev = Some(i);
        }
        if let Some(p) = prev {
            let tail = rows.len().saturating_sub(p + 1);
            if tail > 0 {
                let _ = writeln!(out, "[... {tail} row(s) omitted ...]");
            }
        }
    } else {
        for row in &rows {
            let _ = writeln!(out, "{row}");
        }
    }

    let out = out.trim_end().to_string();
    if out.len() >= content.len() {
        return None;
    }
    log::debug!(
        "[tokenjuice][json] {} rows × {} cols, lossy={} ({} -> {} bytes)",
        rows.len(),
        columns.len(),
        plan.lossy,
        content.len(),
        out.len(),
    );
    if plan.lossy {
        Some(CompressOutput::lossy(out, CompressorKind::SmartCrusher))
    } else {
        // All values preserved, but the array→table reformat changes layout.
        Some(CompressOutput::reformatted(
            out,
            CompressorKind::SmartCrusher,
        ))
    }
}

/// Analyze a JSON array-of-objects without rendering it.
pub fn analyze(content: &str) -> Option<JsonAnalysis> {
    let value: Value = serde_json::from_str(content.trim()).ok()?;
    let array = value.as_array()?;
    if array.len() < MIN_ROWS || !array.iter().all(Value::is_object) {
        return None;
    }
    let flat_rows = flatten_rows(array)?;
    Some(analyze_flat_rows(&flat_rows))
}

/// Pick the row indices to keep when row-dropping: the head/tail windows plus
/// rows flagged as anomalous, duplicate-cluster representatives, spread
/// anchors, representative discriminator buckets, or query-relevant. Returns
/// ascending, de-duplicated indices.
fn plan_rows(rows: &[FlatRow], analysis: &JsonAnalysis, query: Option<&str>) -> JsonPlan {
    let n = rows.len();
    let lossy = n > ROW_DROP_THRESHOLD;
    if !lossy {
        return JsonPlan {
            lossy: false,
            keep_rows: (0..n).collect(),
        };
    }

    let mut keep: BTreeSet<usize> = BTreeSet::new();
    for i in 0..dynamic_head_rows(n) {
        keep.insert(i);
    }
    for i in n.saturating_sub(dynamic_tail_rows(n))..n {
        keep.insert(i);
    }
    for i in spread_anchor_rows(rows, &keep) {
        keep.insert(i);
    }
    for i in duplicate_cluster_representatives(rows) {
        keep.insert(i);
    }

    // Error-text rows: any string/scalar cell carrying an error indicator.
    for (i, row) in rows.iter().enumerate() {
        let row_has_error = row.values.values().any(|v| match v {
            Value::String(s) => has_error_indicators(s),
            other => has_error_indicators(&other.to_string()),
        });
        if row_has_error {
            keep.insert(i);
        }
    }

    // Numeric outliers: for each column, compute mean/std over numeric cells and
    // keep rows whose value is beyond OUTLIER_SIGMA. Bounded by a cap so a wide
    // anomalous tail can't defeat the point of dropping.
    for field in &analysis.fields {
        if let Some(stats) = field.numeric
            && stats.stddev > f64::EPSILON
        {
            for (i, row) in rows.iter().enumerate() {
                if let Some(x) = row.values.get(&field.key).and_then(Value::as_f64)
                    && ((x - stats.mean) / stats.stddev).abs() >= OUTLIER_SIGMA
                {
                    keep.insert(i);
                }
            }
        }
        if field.sparse {
            for (i, row) in rows.iter().enumerate() {
                if row.values.contains_key(&field.key) {
                    keep.insert(i);
                }
            }
        }
        if is_discriminator_field(field, rows.len()) {
            let mut seen_values = BTreeSet::new();
            for (i, row) in rows.iter().enumerate() {
                if let Some(value) = row.values.get(&field.key)
                    && seen_values.insert(render_cell(value))
                {
                    keep.insert(i);
                }
            }
        }
    }

    if let Some(query) = query.filter(|q| !q.trim().is_empty()) {
        let docs: Vec<String> = rows.iter().map(row_text).collect();
        let corpus = Bm25Corpus::new(docs.iter().map(String::as_str));
        let mut scored = corpus.score_all(query);
        scored.sort_by(|a, b| {
            b.score
                .partial_cmp(&a.score)
                .unwrap_or(std::cmp::Ordering::Equal)
                .then_with(|| a.index.cmp(&b.index))
        });
        for score in scored
            .into_iter()
            .take(20)
            .filter(|score| score.score > 0.0)
        {
            keep.insert(score.index);
        }
    }

    JsonPlan {
        lossy,
        keep_rows: keep.into_iter().collect(),
    }
}

fn is_discriminator_field(field: &JsonFieldAnalysis, row_count: usize) -> bool {
    field.present == row_count
        && field.numeric.is_none()
        && (2..=MAX_DISCRIMINATOR_BUCKETS).contains(&field.unique_count)
}

fn duplicate_cluster_representatives(rows: &[FlatRow]) -> Vec<usize> {
    let mut clusters: BTreeMap<String, (usize, usize)> = BTreeMap::new();
    for (i, row) in rows.iter().enumerate() {
        let entry = clusters.entry(row_text(row)).or_insert((i, 0));
        entry.1 += 1;
    }

    let mut repeated = clusters
        .into_values()
        .filter(|(_, count)| *count > 1)
        .collect::<Vec<_>>();
    repeated.sort_by(|(a_first, a_count), (b_first, b_count)| {
        b_count.cmp(a_count).then_with(|| a_first.cmp(b_first))
    });
    repeated
        .into_iter()
        .take(MAX_DUPLICATE_CLUSTER_ANCHORS)
        .map(|(first, _)| first)
        .collect()
}

fn spread_anchor_rows(rows: &[FlatRow], existing_keep: &BTreeSet<usize>) -> Vec<usize> {
    let n = rows.len();
    let start = dynamic_head_rows(n).min(n);
    let end = n.saturating_sub(dynamic_tail_rows(n));
    if start >= end {
        return Vec::new();
    }

    let middle_len = end - start;
    let anchor_count = (middle_len / ROW_DROP_THRESHOLD).min(MAX_SPREAD_ANCHORS);
    if anchor_count == 0 {
        return Vec::new();
    }

    let mut seen = existing_keep
        .iter()
        .filter_map(|&i| rows.get(i).map(row_text))
        .collect::<BTreeSet<_>>();
    let mut anchors = Vec::new();
    for rank in 1..=anchor_count {
        let target = start + (rank * (middle_len + 1)) / (anchor_count + 1);
        if let Some(index) = nearest_unseen_row(rows, start, end, target, &mut seen) {
            anchors.push(index);
        }
    }
    anchors.sort_unstable();
    anchors
}

fn nearest_unseen_row(
    rows: &[FlatRow],
    start: usize,
    end: usize,
    target: usize,
    seen: &mut BTreeSet<String>,
) -> Option<usize> {
    for offset in 0.. {
        let left = target.checked_sub(offset).filter(|&i| i >= start);
        if let Some(i) = left
            && seen.insert(row_text(&rows[i]))
        {
            return Some(i);
        }

        let right = target + offset;
        if right < end && Some(right) != left && seen.insert(row_text(&rows[right])) {
            return Some(right);
        }

        if left.is_none() && right >= end {
            break;
        }
    }
    None
}

#[derive(Debug, Clone)]
struct FlatRow {
    values: BTreeMap<String, Value>,
}

fn flatten_rows(array: &[Value]) -> Option<Vec<FlatRow>> {
    array
        .iter()
        .map(|item| {
            let obj = item.as_object()?;
            Some(FlatRow {
                values: flatten_object(obj),
            })
        })
        .collect()
}

fn flatten_object(obj: &Map<String, Value>) -> BTreeMap<String, Value> {
    let mut out = BTreeMap::new();
    for (key, value) in obj {
        flatten_value(key, value, &mut out);
    }
    out
}

fn flatten_value(prefix: &str, value: &Value, out: &mut BTreeMap<String, Value>) {
    match value {
        Value::Object(obj) if !obj.is_empty() => {
            for (key, child) in obj {
                flatten_value(&format!("{prefix}.{key}"), child, out);
            }
        }
        Value::String(text) => {
            if let Some(parsed) = parse_stringified_json(text) {
                match parsed {
                    Value::Object(obj) if !obj.is_empty() => {
                        for (key, child) in &obj {
                            flatten_value(&format!("{prefix}.{key}"), child, out);
                        }
                    }
                    Value::Array(_) => {
                        out.insert(prefix.to_string(), parsed);
                    }
                    _ => {
                        out.insert(prefix.to_string(), value.clone());
                    }
                }
            } else {
                out.insert(prefix.to_string(), value.clone());
            }
        }
        _ => {
            out.insert(prefix.to_string(), value.clone());
        }
    }
}

fn parse_stringified_json(text: &str) -> Option<Value> {
    let trimmed = text.trim();
    if trimmed.len() > MAX_STRINGIFIED_JSON_BYTES {
        return None;
    }
    if !(trimmed.starts_with('{') || trimmed.starts_with('[')) {
        return None;
    }
    serde_json::from_str(trimmed).ok()
}

fn analyze_flat_rows(rows: &[FlatRow]) -> JsonAnalysis {
    let mut present: HashMap<String, usize> = HashMap::new();
    let mut types: HashMap<String, BTreeSet<&'static str>> = HashMap::new();
    let mut uniques: HashMap<String, BTreeSet<String>> = HashMap::new();
    let mut numeric_values: HashMap<String, Vec<f64>> = HashMap::new();

    for row in rows {
        for (key, value) in &row.values {
            *present.entry(key.clone()).or_insert(0) += 1;
            types
                .entry(key.clone())
                .or_default()
                .insert(value_type(value));
            uniques
                .entry(key.clone())
                .or_default()
                .insert(render_cell(value));
            if let Some(x) = value.as_f64() {
                numeric_values.entry(key.clone()).or_default().push(x);
            }
        }
    }

    let mut fields: Vec<JsonFieldAnalysis> = present
        .into_iter()
        .map(|(key, present)| {
            let unique_count = uniques.get(&key).map_or(0, BTreeSet::len);
            JsonFieldAnalysis {
                key: key.clone(),
                present,
                types: types.remove(&key).unwrap_or_default(),
                unique_count,
                constant: unique_count == 1 && present == rows.len(),
                sparse: present <= sparse_threshold(rows.len()),
                numeric: numeric_values.get(&key).map(|values| numeric_stats(values)),
            }
        })
        .collect();

    fields.sort_by(|a, b| b.present.cmp(&a.present).then_with(|| a.key.cmp(&b.key)));
    let columns = fields
        .iter()
        .map(|field| field.key.clone())
        .collect::<Vec<_>>();
    let estimated_table_bytes = rows
        .iter()
        .map(|row| {
            row.values
                .values()
                .map(render_cell)
                .map(|s| s.len() + 3)
                .sum::<usize>()
        })
        .sum::<usize>()
        + columns.iter().map(String::len).sum::<usize>();

    JsonAnalysis {
        row_count: rows.len(),
        columns,
        fields,
        estimated_table_bytes,
    }
}

fn value_type(value: &Value) -> &'static str {
    match value {
        Value::Null => "null",
        Value::Bool(_) => "bool",
        Value::Number(_) => "number",
        Value::String(_) => "string",
        Value::Array(_) => "array",
        Value::Object(_) => "object",
    }
}

fn numeric_stats(values: &[f64]) -> JsonNumericStats {
    let min = values.iter().copied().fold(f64::INFINITY, f64::min);
    let max = values.iter().copied().fold(f64::NEG_INFINITY, f64::max);
    let mean = values.iter().sum::<f64>() / values.len() as f64;
    let variance = values.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / values.len() as f64;
    JsonNumericStats {
        min,
        max,
        mean,
        stddev: variance.sqrt(),
    }
}

fn sparse_threshold(row_count: usize) -> usize {
    3.max(row_count / 10)
}

fn dynamic_head_rows(row_count: usize) -> usize {
    HEAD_ROWS.min((row_count / 4).max(5))
}

fn dynamic_tail_rows(row_count: usize) -> usize {
    TAIL_ROWS.min((row_count / 8).max(3))
}

fn row_text(row: &FlatRow) -> String {
    row.values
        .iter()
        .map(|(key, value)| format!("{key}={}", render_cell(value)))
        .collect::<Vec<_>>()
        .join(" ")
}

/// Render a single cell. Scalars print bare-ish; nested values stay as compact
/// JSON so the table remains lossless.
fn render_cell(v: &Value) -> String {
    match v {
        Value::String(s) if !s.contains('|') && !s.contains('\n') => s.clone(),
        Value::Bool(b) => b.to_string(),
        Value::Number(n) => n.to_string(),
        other => serde_json::to_string(other).unwrap_or_default(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
        assert!(debug.sparse);
        assert_eq!(debug.present, 1);
        assert_eq!(latency.numeric.expect("numeric").min, 10.0);
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
}
