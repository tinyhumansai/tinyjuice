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
use serde_json::Value;
use std::collections::BTreeSet;
use std::fmt::Write as _;

use super::Compressor;
use super::signals::has_error_indicators;
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
/// At most this many outlier rows are kept per column (the most extreme
/// first), so a heavy-tailed column can't defeat the point of row-dropping.
pub const MAX_OUTLIERS_PER_COLUMN: usize = 8;

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
        compress(input.content)
    }
}

/// Compress a JSON array-of-objects into a compact table. Returns `None` when
/// the content isn't a uniform-enough array of objects or wouldn't shrink.
pub fn compress(content: &str) -> Option<CompressOutput> {
    let value: Value = serde_json::from_str(content.trim()).ok()?;
    let array = value.as_array()?;
    if array.len() < MIN_ROWS {
        return None;
    }
    if !array.iter().all(Value::is_object) {
        return None;
    }

    // Column order = first-seen key order across all rows (union, stable).
    let mut columns: Vec<String> = Vec::new();
    for item in array {
        if let Some(obj) = item.as_object() {
            for key in obj.keys() {
                if !columns.iter().any(|c| c == key) {
                    columns.push(key.clone());
                }
            }
        }
    }
    if columns.len() < 2 {
        return None;
    }

    // Render every row's cells up front so we can choose full vs. row-dropped.
    let mut rows: Vec<String> = Vec::with_capacity(array.len());
    for item in array {
        let obj = item.as_object()?;
        let cells: Vec<String> = columns
            .iter()
            .map(|col| match obj.get(col) {
                None => String::new(),
                Some(v) => render_cell(v),
            })
            .collect();
        rows.push(render_markdown_row(&cells));
    }

    let lossy = rows.len() > ROW_DROP_THRESHOLD;
    let mut out = String::with_capacity(content.len());
    let _ = writeln!(
        out,
        "[json table: {} rows × {} cols · blank=absent key · exact original via retrieve footer]",
        rows.len(),
        columns.len()
    );
    let header_cells: Vec<String> = columns
        .iter()
        .map(|column| escape_markdown_cell(column))
        .collect();
    let _ = writeln!(out, "{}", render_markdown_row(&header_cells));
    let _ = writeln!(out, "{}", render_markdown_separator(columns.len()));

    if lossy {
        // Keep head + tail PLUS any anomalous rows (errors / numeric outliers)
        // so the signal in a large homogeneous array survives row-dropping.
        let keep = rows_to_keep(array, &columns, rows.len());
        let mut prev: Option<usize> = None;
        for &i in &keep {
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

    let table = out.trim_end().to_string();
    let minified = serde_json::to_string(&value).ok()?;
    let output = if minified.len() < content.len() && minified.len() < table.len() {
        log::debug!(
            "[tokenjuice][json] minified {} rows × {} cols ({} -> {} bytes); markdown table was {} bytes",
            rows.len(),
            columns.len(),
            content.len(),
            minified.len(),
            table.len(),
        );
        return Some(CompressOutput::reformatted(
            minified,
            CompressorKind::SmartCrusher,
        ));
    } else if table.len() < content.len() {
        table
    } else {
        return None;
    };

    log::debug!(
        "[tokenjuice][json] markdown table {} rows × {} cols, lossy={} ({} -> {} bytes)",
        rows.len(),
        columns.len(),
        lossy,
        content.len(),
        output.len(),
    );
    if lossy {
        Some(CompressOutput::lossy(output, CompressorKind::SmartCrusher))
    } else {
        // All values preserved, but the array→table reformat changes layout.
        Some(CompressOutput::reformatted(
            output,
            CompressorKind::SmartCrusher,
        ))
    }
}

/// Pick the row indices to keep when row-dropping: the head/tail windows plus
/// any row flagged as anomalous (error text or a numeric outlier in any
/// column). Returns ascending, de-duplicated indices.
fn rows_to_keep(array: &[Value], columns: &[String], n: usize) -> Vec<usize> {
    let mut keep: BTreeSet<usize> = BTreeSet::new();
    for i in 0..HEAD_ROWS.min(n) {
        keep.insert(i);
    }
    for i in n.saturating_sub(TAIL_ROWS)..n {
        keep.insert(i);
    }

    // Error-text rows: string cells carrying an error indicator. Only string
    // leaves realistically carry error text — stringifying every nested
    // object/number per row just to substring-scan is pure allocation churn.
    for (i, item) in array.iter().enumerate() {
        if let Some(obj) = item.as_object() {
            let row_has_error = obj
                .values()
                .any(|v| matches!(v, Value::String(s) if has_error_indicators(s)));
            if row_has_error {
                keep.insert(i);
            }
        }
    }

    // Numeric outliers: for each column, compute mean/std over numeric cells
    // and keep rows beyond OUTLIER_SIGMA — capped at the most extreme
    // MAX_OUTLIERS_PER_COLUMN so a heavy-tailed distribution (which flags a
    // wide band of rows at |z| ≥ 2) can't keep most of the table.
    for col in columns {
        let nums: Vec<(usize, f64)> = array
            .iter()
            .enumerate()
            .filter_map(|(i, item)| {
                item.as_object()
                    .and_then(|o| o.get(col))
                    .and_then(Value::as_f64)
                    .map(|x| (i, x))
            })
            .collect();
        if nums.len() < 4 {
            continue;
        }
        let mean = nums.iter().map(|(_, x)| x).sum::<f64>() / nums.len() as f64;
        let var = nums.iter().map(|(_, x)| (x - mean).powi(2)).sum::<f64>() / nums.len() as f64;
        let std = var.sqrt();
        if std <= f64::EPSILON {
            continue;
        }
        let mut outliers: Vec<(usize, f64)> = nums
            .into_iter()
            .map(|(i, x)| (i, ((x - mean) / std).abs()))
            .filter(|(_, z)| *z >= OUTLIER_SIGMA)
            .collect();
        outliers.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        for (i, _) in outliers.into_iter().take(MAX_OUTLIERS_PER_COLUMN) {
            keep.insert(i);
        }
    }

    keep.into_iter().collect()
}

/// Render a single cell. Scalars print bare-ish; nested values stay as compact
/// JSON so the table remains lossless.
fn render_cell(v: &Value) -> String {
    match v {
        Value::String(s) if !s.contains('|') && !s.contains('\n') => escape_markdown_cell(s),
        Value::Bool(b) => b.to_string(),
        Value::Number(n) => n.to_string(),
        other => escape_markdown_cell(&serde_json::to_string(other).unwrap_or_default()),
    }
}

fn render_markdown_row(cells: &[String]) -> String {
    format!("| {} |", cells.join(" | "))
}

fn render_markdown_separator(width: usize) -> String {
    let cols = vec!["---"; width];
    format!("| {} |", cols.join(" | "))
}

fn escape_markdown_cell(cell: &str) -> String {
    cell.replace('\\', "\\\\").replace('|', "\\|")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn crushes_uniform_array() {
        let mut rows = Vec::new();
        for i in 0..80 {
            rows.push(format!(
                r#"{{"id":{i},"name":"item number {i}","status":"active","owner":"team-alpha"}}"#
            ));
        }
        let input = format!("[{}]", rows.join(","));
        let out = compress(&input).expect("compresses").text;
        assert_eq!(out.matches("status").count(), 1, "{out}");
        assert!(out.contains("| id | name | owner | status |"), "{out}");
        assert!(out.contains("| --- | --- | --- | --- |"), "{out}");
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

    #[test]
    fn markdown_table_cells_escape_pipes() {
        let mut rows = vec![r#"{"id":0,"text":"alpha | beta","meta":{"note":"x|y"}}"#.to_string()];
        for i in 1..80 {
            rows.push(format!(
                r#"{{"id":{i},"text":"record {i} with repeated detail","meta":{{"note":"z"}}}}"#
            ));
        }
        let input = format!("[{}]", rows.join(","));
        let out = compress(&input).expect("compresses").text;
        assert!(out.contains("| id | meta | text |"), "{out}");
        assert!(out.contains("alpha \\| beta"), "{out}");
        assert!(out.contains(r#"{"note":"x\|y"}"#), "{out}");
    }

    #[test]
    fn falls_back_to_minified_json_when_table_is_larger() {
        let input = r#"[
          {"a": 1, "b": 2},
          {"a": 3, "b": 4},
          {"a": 5, "b": 6}
        ]"#;
        let out = compress(input).expect("minifies").text;
        assert_eq!(out, r#"[{"a":1,"b":2},{"a":3,"b":4},{"a":5,"b":6}]"#);
    }

    #[test]
    fn minified_json_wins_when_table_is_smaller_than_original_but_larger_than_minified() {
        let input = r#"[
          {"enabled": true, "id": 1},
          {"enabled": false, "id": 2},
          {"enabled": true, "id": 3},
          {"enabled": false, "id": 4},
          {"enabled": true, "id": 5}
        ]"#;
        let out = compress(input).expect("minifies").text;
        assert_eq!(
            out,
            r#"[{"enabled":true,"id":1},{"enabled":false,"id":2},{"enabled":true,"id":3},{"enabled":false,"id":4},{"enabled":true,"id":5}]"#
        );
    }
}
