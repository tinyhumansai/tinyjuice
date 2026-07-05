//! JSON crusher (SmartCrusher).
//!
//! Clean-room port of Headroom's `SmartCrusher` (Apache-2.0), extended with
//! variance-aware row preservation, nested-array discovery, column factoring and
//! query anchoring. An array of objects that repeat the same keys is the single
//! most common bloated tool output (API list responses, DB rows, search
//! manifests). Re-rendering it as a table emits each key **once** instead of per
//! row.
//!
//! On top of the base table:
//!   * a top-level **object** is scanned breadth-first (depth ≤ 2) for the
//!     largest uniform-object array, which is tabled under its JSON path with the
//!     sibling scalars rendered as a preamble;
//!   * **constant columns** (identical across every row) are hoisted into a
//!     one-line note and dropped from the table (lossless);
//!   * columns whose values are uniformly objects are **flattened** — the
//!     name/description-like sub-keys are promoted to their own columns instead
//!     of dumping an escaped-JSON blob per cell;
//!   * plain **number/string arrays** collapse to distributional stats /
//!     deduped counts;
//!   * rows carrying **error text (nested), categorical rarities (Pareto),
//!     rare structural fields, numeric outliers, or a query anchor** are always
//!     kept when the table is row-dropped.
//!
//! The router offloads the full original to CCR, so everything dropped stays
//! recoverable.

use async_trait::async_trait;
use serde_json::{Map, Value};
use std::collections::{BTreeSet, HashMap};
use std::fmt::Write as _;

use super::adaptive::compute_optimal_k;
use super::anchors::extract_anchors;
use super::signals::has_error_indicators;
use super::{BLOCK_NOTE, Compressor, block_token};
use crate::types::{CompressInput, CompressOptions, CompressOutput, CompressorKind};

/// Minimum rows before tabular rendering is worth the header overhead.
pub const MIN_ROWS: usize = 3;
/// Above this many rows the table is additionally row-dropped.
pub const ROW_DROP_THRESHOLD: usize = 40;
/// At most this many rows are kept from the head when row-dropping (the
/// adaptive selector may keep fewer for redundant tables).
pub const HEAD_ROWS: usize = 20;
/// At most this many rows are kept from the tail when row-dropping (the
/// adaptive selector may keep fewer for redundant tables).
pub const TAIL_ROWS: usize = 10;
/// Adaptive floor: never keep fewer head+tail rows than this when the table
/// has more rows than it.
pub const MIN_KEPT_ROWS: usize = 8;
/// Z-score beyond which a numeric cell is treated as an outlier worth keeping.
pub const OUTLIER_SIGMA: f64 = 2.0;
/// At most this many outlier rows are kept per column (the most extreme
/// first), so a heavy-tailed column can't defeat the point of row-dropping.
pub const MAX_OUTLIERS_PER_COLUMN: usize = 8;
/// Cells longer than this are truncated (marking the table lossy).
pub const CELL_MAX: usize = 240;
/// Minimum elements before a plain number/string array is worth crushing.
pub const SCALAR_ARRAY_MIN: usize = 8;
/// A categorical column with more distinct values than this isn't Pareto-scanned.
pub const MAX_CARDINALITY: usize = 50;
/// Pareto coverage target: the smallest value set covering this fraction of rows.
pub const PARETO_COVER: f64 = 0.80;
/// Only rare-status rows are kept when the covering set is at most this size.
pub const PARETO_MAX_K: usize = 5;
/// Rows carrying a field present in fewer than this fraction of rows are kept.
pub const RARE_FIELD_RATIO: f64 = 0.20;

pub struct JsonCompressor;

#[async_trait]
impl Compressor for JsonCompressor {
    fn kind(&self) -> CompressorKind {
        CompressorKind::SmartCrusher
    }

    async fn compress(
        &self,
        input: &CompressInput<'_>,
        opts: &CompressOptions,
    ) -> Option<CompressOutput> {
        compress(input.content, opts.ccr_enabled, input.hint.query.as_deref())
    }
}

/// Compress a JSON payload. `block_tokens` offloads each omitted block to CCR so
/// its marker retrieves exactly that block. `query` (when present) yields exact-
/// match anchors that force any row mentioning them to survive row-dropping.
/// Returns `None` when there's nothing tabular/statistical to do or it wouldn't
/// shrink.
pub fn compress(content: &str, block_tokens: bool, query: Option<&str>) -> Option<CompressOutput> {
    let value: Value = serde_json::from_str(content.trim()).ok()?;
    let anchors = extract_anchors(query.unwrap_or_default());

    match &value {
        Value::Array(array) => compress_top_array(&value, array, content, block_tokens, &anchors),
        Value::Object(map) => compress_object(map, content, block_tokens, &anchors),
        _ => None,
    }
}

/// Dispatch a top-level array to the object-table, number-stats or string-dedupe
/// crusher depending on its element kind.
fn compress_top_array(
    value: &Value,
    array: &[Value],
    content: &str,
    block_tokens: bool,
    anchors: &[String],
) -> Option<CompressOutput> {
    if array.len() >= MIN_ROWS && array.iter().all(Value::is_object) {
        return object_table_output(value, array, content, block_tokens, anchors, None, "");
    }
    if array.len() >= SCALAR_ARRAY_MIN && array.iter().all(is_plain_number) {
        return crush_number_array(array, content, block_tokens, "");
    }
    if array.len() >= SCALAR_ARRAY_MIN && array.iter().all(Value::is_string) {
        return crush_string_array(array, content, block_tokens, "");
    }
    None
}

/// A top-level object: find the largest uniform-object array nested within
/// (depth ≤ 2) and table it under its JSON path, else fall back to the largest
/// plain scalar array. Sibling scalars of the container become a preamble.
fn compress_object(
    root: &Map<String, Value>,
    content: &str,
    block_tokens: bool,
    anchors: &[String],
) -> Option<CompressOutput> {
    let found = find_best_nested(root)?;
    let container = found.container;
    let key = found.path.rsplit('.').next().unwrap_or(&found.path);
    let preamble = sibling_preamble(container, key);

    match found.kind {
        CandidateKind::Objects => {
            let arr = container.get(key)?.as_array()?;
            object_table_output(
                &Value::Null,
                arr,
                content,
                block_tokens,
                anchors,
                Some(&found.path),
                &preamble,
            )
        }
        CandidateKind::Numbers => {
            let arr = container.get(key)?.as_array()?;
            crush_number_array(arr, content, block_tokens, &found.path)
        }
        CandidateKind::Strings => {
            let arr = container.get(key)?.as_array()?;
            crush_string_array(arr, content, block_tokens, &found.path)
        }
    }
}

// ---------------------------------------------------------------------------
// Nested-array discovery
// ---------------------------------------------------------------------------

#[derive(Clone, Copy, PartialEq)]
enum CandidateKind {
    Objects,
    Numbers,
    Strings,
}

struct Candidate<'a> {
    path: String,
    container: &'a Map<String, Value>,
    kind: CandidateKind,
    rows: usize,
}

/// Breadth-first (depth ≤ 2) scan of `root` for the best crushable array. Prefers
/// the largest uniform-object array; failing that, the largest plain scalar
/// array. Returns `None` when nothing qualifies.
fn find_best_nested(root: &Map<String, Value>) -> Option<Candidate<'_>> {
    let mut candidates: Vec<Candidate> = Vec::new();
    collect_candidates(root, String::new(), 1, &mut candidates);

    // Object arrays win outright (most rows); scalar arrays are the fallback.
    candidates
        .iter()
        .filter(|c| c.kind == CandidateKind::Objects)
        .max_by_key(|c| c.rows)
        .or_else(|| {
            candidates
                .iter()
                .filter(|c| c.kind != CandidateKind::Objects)
                .max_by_key(|c| c.rows)
        })
        .map(|c| Candidate {
            path: c.path.clone(),
            container: c.container,
            kind: c.kind,
            rows: c.rows,
        })
}

/// Visit the entries of `obj` at `depth`, recording any qualifying array and
/// recursing one level into nested objects (until `depth` exceeds 2).
fn collect_candidates<'a>(
    obj: &'a Map<String, Value>,
    prefix: String,
    depth: usize,
    out: &mut Vec<Candidate<'a>>,
) {
    for (key, val) in obj {
        let path = if prefix.is_empty() {
            key.clone()
        } else {
            format!("{prefix}.{key}")
        };
        match val {
            Value::Array(arr) => {
                if let Some(kind) = classify_array(arr) {
                    out.push(Candidate {
                        path,
                        container: obj,
                        kind,
                        rows: arr.len(),
                    });
                }
            }
            Value::Object(inner) if depth < 2 => {
                collect_candidates(inner, path, depth + 1, out);
            }
            _ => {}
        }
    }
}

/// Classify an array as a crushable object/number/string array, or `None`.
fn classify_array(arr: &[Value]) -> Option<CandidateKind> {
    if arr.len() >= MIN_ROWS && arr.iter().all(Value::is_object) && union_columns(arr).len() >= 2 {
        Some(CandidateKind::Objects)
    } else if arr.len() >= SCALAR_ARRAY_MIN && arr.iter().all(is_plain_number) {
        Some(CandidateKind::Numbers)
    } else if arr.len() >= SCALAR_ARRAY_MIN && arr.iter().all(Value::is_string) {
        Some(CandidateKind::Strings)
    } else {
        None
    }
}

/// Render the scalar siblings of the tabled array key as `key: value` preamble
/// lines (truncated). Non-scalar siblings are skipped — the table/preamble is a
/// summary, and the router keeps the original recoverable.
fn sibling_preamble(container: &Map<String, Value>, array_key: &str) -> String {
    let mut out = String::new();
    for (key, val) in container {
        if key == array_key {
            continue;
        }
        let rendered = match val {
            Value::String(s) => truncate_plain(s, CELL_MAX),
            Value::Bool(_) | Value::Number(_) | Value::Null => val.to_string(),
            _ => continue,
        };
        let _ = writeln!(out, "{key}: {rendered}");
    }
    out
}

// ---------------------------------------------------------------------------
// Object-array table
// ---------------------------------------------------------------------------

/// Build the table and wrap it in a [`CompressOutput`], applying the minified-
/// JSON fallback only for a genuine top-level array (`minify_value` non-null).
#[allow(clippy::too_many_arguments)]
fn object_table_output(
    minify_value: &Value,
    array: &[Value],
    content: &str,
    block_tokens: bool,
    anchors: &[String],
    path: Option<&str>,
    preamble: &str,
) -> Option<CompressOutput> {
    let (table, lossy) = build_object_table(array, block_tokens, anchors, path, preamble)?;

    // Top-level arrays may still be better served by a straight minify; nested
    // discovery always prefers the table (minifying the whole object defeats it).
    if path.is_none()
        && let Ok(minified) = serde_json::to_string(minify_value)
        && minified.len() < content.len()
        && minified.len() < table.len()
    {
        return Some(CompressOutput::reformatted(
            minified,
            CompressorKind::SmartCrusher,
        ));
    }

    if table.len() >= content.len() {
        return None;
    }
    log::debug!(
        "[tinyjuice][json] table {} rows, lossy={} ({} -> {} bytes)",
        array.len(),
        lossy,
        content.len(),
        table.len(),
    );
    Some(if lossy {
        CompressOutput::lossy(table, CompressorKind::SmartCrusher)
    } else {
        CompressOutput::reformatted(table, CompressorKind::SmartCrusher)
    })
}

/// The output column plan for one source column.
enum ColPlan {
    /// Identical across every row — hoisted to a preamble note, dropped.
    Constant(String),
    /// Rendered verbatim.
    Plain,
    /// Uniform-object column: promote `promoted` sub-keys to their own columns,
    /// dump the rest as one compact-JSON blob column (when `remainder` non-empty).
    Flat {
        promoted: Vec<String>,
        remainder: Vec<String>,
    },
}

/// An output column resolved from the plans (owns its labels).
enum OutCol {
    Plain(String),
    FlatSub(String, String),
    FlatBlob(String, Vec<String>),
}

/// Render an object-array as a markdown table (constant columns hoisted, uniform-
/// object columns flattened, cells truncated). Returns `(text, lossy)` or `None`
/// when there aren't ≥ 2 columns / any non-constant column left.
fn build_object_table(
    array: &[Value],
    block_tokens: bool,
    anchors: &[String],
    path: Option<&str>,
    preamble: &str,
) -> Option<(String, bool)> {
    let columns = union_columns(array);
    if columns.len() < 2 {
        return None;
    }
    let n = array.len();

    // Plan every source column, collecting constant-column notes and the ordered
    // list of concrete output columns.
    let mut notes: Vec<(String, String)> = Vec::new();
    let mut out_cols: Vec<OutCol> = Vec::new();
    for col in &columns {
        match analyze_column(array, col, n) {
            ColPlan::Constant(v) => notes.push((col.clone(), v)),
            ColPlan::Plain => out_cols.push(OutCol::Plain(col.clone())),
            ColPlan::Flat {
                promoted,
                remainder,
            } => {
                for sub in promoted {
                    out_cols.push(OutCol::FlatSub(col.clone(), sub));
                }
                if !remainder.is_empty() {
                    out_cols.push(OutCol::FlatBlob(col.clone(), remainder));
                }
            }
        }
    }
    if out_cols.is_empty() {
        return None;
    }

    // Header labels.
    let headers: Vec<String> = out_cols
        .iter()
        .map(|oc| match oc {
            OutCol::Plain(c) => escape_markdown_cell(c),
            OutCol::FlatSub(c, s) => escape_markdown_cell(&format!("{c}.{s}")),
            OutCol::FlatBlob(c, _) => escape_markdown_cell(&format!("{c}.\u{2026}")),
        })
        .collect();
    let width = headers.len();

    // Render each row's cells (tracking whether any cell was truncated).
    let mut truncated = false;
    let mut rows: Vec<String> = Vec::with_capacity(n);
    for item in array {
        let obj = item.as_object()?;
        let cells: Vec<String> = out_cols
            .iter()
            .map(|oc| render_out_cell(obj, oc, &mut truncated))
            .collect();
        rows.push(render_markdown_row(&cells));
    }

    let row_dropped = n > ROW_DROP_THRESHOLD;
    let lossy = row_dropped || truncated;

    // Assemble output: sibling preamble, constant notes, header, body.
    let mut out =
        String::with_capacity(preamble.len() + rows.iter().map(String::len).sum::<usize>());
    if !preamble.is_empty() {
        out.push_str(preamble);
    }
    if !notes.is_empty() {
        let joined = notes
            .iter()
            .map(|(k, v)| format!("{k}={v}"))
            .collect::<Vec<_>>()
            .join(", ");
        let _ = writeln!(out, "[all rows: {joined}]");
    }
    match path {
        Some(p) => {
            let _ = writeln!(
                out,
                "[json table: {p} — {n} rows × {width} cols · blank=absent key · exact original via retrieve footer]"
            );
        }
        None => {
            let _ = writeln!(
                out,
                "[json table: {n} rows × {width} cols · blank=absent key · exact original via retrieve footer]"
            );
        }
    }
    let _ = writeln!(out, "{}", render_markdown_row(&headers));
    let _ = writeln!(out, "{}", render_markdown_separator(width));

    let mut any_token = false;
    if row_dropped {
        let keep = rows_to_keep(array, &columns, n, anchors);
        let mut gap_marker = |out: &mut String, start: usize, end: usize| {
            let slice = serde_json::to_string(&array[start..end]).unwrap_or_default();
            let token = block_token(&slice, block_tokens && !slice.is_empty());
            any_token |= !token.is_empty();
            let _ = writeln!(out, "[... {} row(s) omitted ...{token}]", end - start);
        };
        let mut prev: Option<usize> = None;
        for &i in &keep {
            if let Some(p) = prev {
                if i > p + 1 {
                    gap_marker(&mut out, p + 1, i);
                }
            } else if i > 0 {
                gap_marker(&mut out, 0, i);
            }
            let _ = writeln!(out, "{}", rows[i]);
            prev = Some(i);
        }
        if let Some(p) = prev
            && p + 1 < n
        {
            gap_marker(&mut out, p + 1, n);
        }
    } else {
        for row in &rows {
            let _ = writeln!(out, "{row}");
        }
    }

    let mut table = out.trim_end().to_string();
    if any_token {
        table.push_str(BLOCK_NOTE);
    }
    Some((table, lossy))
}

/// Plan a single source column: constant → hoisted, uniform-object with an
/// informative key → flattened, else plain.
fn analyze_column(array: &[Value], col: &str, n: usize) -> ColPlan {
    let present: Vec<&Value> = array
        .iter()
        .filter_map(|item| item.as_object().and_then(|o| o.get(col)))
        .collect();

    // Constant: present in every row and all equal.
    if present.len() == n && n > 0 {
        let first = &present[0];
        if present.iter().all(|v| *v == *first) {
            return ColPlan::Constant(compact_json(first));
        }
    }

    // Uniform object → maybe flatten.
    if !present.is_empty() && present.iter().all(|v| v.is_object()) {
        let subkeys = union_subkeys(&present);
        let (promoted, remainder) = plan_flatten(&subkeys, &present);
        if promoted.iter().any(|k| is_name_like(k) || is_desc_like(k)) {
            return ColPlan::Flat {
                promoted,
                remainder,
            };
        }
    }

    ColPlan::Plain
}

/// Choose the promoted sub-keys (name/id/title-like first, description-like
/// next, then up to three total filled with scalar-valued keys) and the
/// remainder to keep as a blob.
fn plan_flatten(subkeys: &[String], present: &[&Value]) -> (Vec<String>, Vec<String>) {
    let mut promoted: Vec<String> = Vec::new();
    if let Some(k) = subkeys.iter().find(|k| is_name_like(k)) {
        promoted.push(k.clone());
    }
    if let Some(k) = subkeys.iter().find(|k| is_desc_like(k)) {
        promoted.push(k.clone());
    }
    for k in subkeys {
        if promoted.len() >= 3 {
            break;
        }
        if !promoted.contains(k) && subkey_is_scalar(k, present) {
            promoted.push(k.clone());
        }
    }
    let remainder: Vec<String> = subkeys
        .iter()
        .filter(|k| !promoted.contains(k))
        .cloned()
        .collect();
    (promoted, remainder)
}

/// True if the sub-key's value is scalar in the first object that carries it.
fn subkey_is_scalar(sub: &str, present: &[&Value]) -> bool {
    for v in present {
        if let Some(obj) = v.as_object()
            && let Some(inner) = obj.get(sub)
        {
            return !matches!(inner, Value::Object(_) | Value::Array(_));
        }
    }
    false
}

/// Render one output cell for a row, applying truncation.
fn render_out_cell(obj: &Map<String, Value>, oc: &OutCol, truncated: &mut bool) -> String {
    let raw = match oc {
        OutCol::Plain(col) => obj.get(col).map(render_cell).unwrap_or_default(),
        OutCol::FlatSub(col, sub) => obj
            .get(col)
            .and_then(Value::as_object)
            .and_then(|o| o.get(sub))
            .map(render_cell)
            .unwrap_or_default(),
        OutCol::FlatBlob(col, subs) => match obj.get(col).and_then(Value::as_object) {
            Some(o) => {
                let mut kept = Map::new();
                for s in subs {
                    if let Some(v) = o.get(s) {
                        kept.insert(s.clone(), v.clone());
                    }
                }
                if kept.is_empty() {
                    String::new()
                } else {
                    escape_markdown_cell(&compact_json(&Value::Object(kept)))
                }
            }
            None => obj.get(col).map(render_cell).unwrap_or_default(),
        },
    };
    let (cell, cut) = truncate_cell(&raw);
    *truncated |= cut;
    cell
}

// ---------------------------------------------------------------------------
// Row-keeping (row-drop must-keeps)
// ---------------------------------------------------------------------------

/// Pick the row indices to keep when row-dropping: adaptively sized head/tail
/// windows plus every anomalous row — nested error text, numeric outliers,
/// Pareto-rare categorical values, rare structural fields, and query-anchor
/// hits. Ascending, de-duped.
fn rows_to_keep(array: &[Value], columns: &[String], n: usize, anchors: &[String]) -> Vec<usize> {
    let (head, tail) = adaptive_head_tail(array);
    let mut keep: BTreeSet<usize> = BTreeSet::new();
    for i in 0..head.min(n) {
        keep.insert(i);
    }
    for i in n.saturating_sub(tail)..n {
        keep.insert(i);
    }

    // (2) Error text, scanning nested object/array values to depth 2.
    for (i, item) in array.iter().enumerate() {
        if value_has_error(item, 2) {
            keep.insert(i);
        }
    }

    // (8) Query anchors: keep any row whose serialized form contains an anchor.
    if !anchors.is_empty() {
        for (i, item) in array.iter().enumerate() {
            let serialized = compact_json(item).to_ascii_lowercase();
            if anchors.iter().any(|a| serialized.contains(a)) {
                keep.insert(i);
            }
        }
    }

    for col in columns {
        keep_numeric_outliers(array, col, &mut keep);
        keep_pareto_rare(array, col, n, &mut keep);
        keep_rare_field(array, col, n, &mut keep);
    }

    keep.into_iter().collect()
}

/// Size the head/tail keep windows from the rows themselves: highly redundant
/// arrays collapse toward [`MIN_KEPT_ROWS`], diverse ones keep up to the full
/// [`HEAD_ROWS`] + [`TAIL_ROWS`] budget. The total splits ~2/3 head (first
/// rows) and ~1/3 tail (last rows).
fn adaptive_head_tail(array: &[Value]) -> (usize, usize) {
    let serialized: Vec<String> = array.iter().map(compact_json).collect();
    let refs: Vec<&str> = serialized.iter().map(String::as_str).collect();
    let k = compute_optimal_k(&refs, MIN_KEPT_ROWS, HEAD_ROWS + TAIL_ROWS);
    let tail = (k / 3).max(1);
    (k - tail, tail)
}

/// Recursively test for error indicators in string leaves down to `depth`.
fn value_has_error(v: &Value, depth: usize) -> bool {
    match v {
        Value::String(s) => has_error_indicators(s),
        Value::Array(a) if depth > 0 => a.iter().any(|x| value_has_error(x, depth - 1)),
        Value::Object(m) if depth > 0 => m.values().any(|x| value_has_error(x, depth - 1)),
        _ => false,
    }
}

/// (existing) Keep the most extreme numeric outliers in `col`.
fn keep_numeric_outliers(array: &[Value], col: &str, keep: &mut BTreeSet<usize>) {
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
        return;
    }
    let mean = nums.iter().map(|(_, x)| x).sum::<f64>() / nums.len() as f64;
    let var = nums.iter().map(|(_, x)| (x - mean).powi(2)).sum::<f64>() / nums.len() as f64;
    let std = var.sqrt();
    if std <= f64::EPSILON {
        return;
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

/// (5) Pareto rare-status: for a low-cardinality string column, keep rows whose
/// value falls outside the smallest ≥ 80%-covering set when that set is tiny.
fn keep_pareto_rare(array: &[Value], col: &str, n: usize, keep: &mut BTreeSet<usize>) {
    let mut freq: HashMap<&str, usize> = HashMap::new();
    let mut total = 0usize;
    for item in array {
        if let Some(Value::String(s)) = item.as_object().and_then(|o| o.get(col)) {
            *freq.entry(s.as_str()).or_default() += 1;
            total += 1;
        }
    }
    // Require a genuinely categorical column that most rows participate in.
    if total < n / 2 || freq.len() < 2 || freq.len() > MAX_CARDINALITY {
        return;
    }
    let mut counts: Vec<(&str, usize)> = freq.into_iter().collect();
    counts.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(b.0)));
    let target = (PARETO_COVER * total as f64).ceil() as usize;
    let mut covered = 0usize;
    let mut k = 0usize;
    let mut common: BTreeSet<&str> = BTreeSet::new();
    for (val, c) in &counts {
        if covered >= target {
            break;
        }
        common.insert(val);
        covered += c;
        k += 1;
    }
    // Only fire when a tiny head covers the bulk AND rare values remain.
    if k > PARETO_MAX_K || common.len() >= counts.len() {
        return;
    }
    for (i, item) in array.iter().enumerate() {
        if let Some(Value::String(s)) = item.as_object().and_then(|o| o.get(col))
            && !common.contains(s.as_str())
        {
            keep.insert(i);
        }
    }
}

/// (6) Rare structural field: if `col` appears in < 20% of rows, keep the rows
/// that carry it.
fn keep_rare_field(array: &[Value], col: &str, n: usize, keep: &mut BTreeSet<usize>) {
    let present: Vec<usize> = array
        .iter()
        .enumerate()
        .filter(|(_, item)| item.as_object().is_some_and(|o| o.contains_key(col)))
        .map(|(i, _)| i)
        .collect();
    let threshold = (RARE_FIELD_RATIO * n as f64).floor() as usize;
    if present.is_empty() || present.len() >= threshold.max(1) {
        return;
    }
    for i in present {
        keep.insert(i);
    }
}

// ---------------------------------------------------------------------------
// Scalar-array crushers
// ---------------------------------------------------------------------------

/// Collapse a plain-number array to distributional stats + head/tail samples.
fn crush_number_array(
    arr: &[Value],
    content: &str,
    block_tokens: bool,
    path: &str,
) -> Option<CompressOutput> {
    let mut nums: Vec<f64> = arr.iter().filter_map(Value::as_f64).collect();
    if nums.len() < SCALAR_ARRAY_MIN {
        return None;
    }
    let n = nums.len();
    nums.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    let mean = nums.iter().sum::<f64>() / n as f64;
    let pct = |q: f64| nums[((q * (n - 1) as f64).round() as usize).min(n - 1)];

    let head = 5.min(n);
    let tail = 3.min(n.saturating_sub(head));
    let mut out = String::new();
    let label = if path.is_empty() {
        String::new()
    } else {
        format!(" {path}")
    };
    let _ = writeln!(
        out,
        "[json number array{label}: {n} values · min={} p25={} median={} p75={} max={} mean={}]",
        fmt_num(pct(0.0)),
        fmt_num(pct(0.25)),
        fmt_num(pct(0.5)),
        fmt_num(pct(0.75)),
        fmt_num(pct(1.0)),
        fmt_num(mean),
    );
    let head_vals: Vec<String> = arr[..head].iter().map(compact_json).collect();
    let _ = writeln!(out, "head: {}", head_vals.join(", "));
    if head + tail < arr.len() {
        let slice = serde_json::to_string(&arr[head..arr.len() - tail]).unwrap_or_default();
        let token = block_token(&slice, block_tokens && !slice.is_empty());
        let _ = writeln!(
            out,
            "[... {} value(s) omitted ...{token}]",
            arr.len() - head - tail
        );
        if !token.is_empty() {
            out.push_str(BLOCK_NOTE);
            out.push('\n');
        }
    }
    if tail > 0 {
        let tail_vals: Vec<String> = arr[arr.len() - tail..].iter().map(compact_json).collect();
        let _ = writeln!(out, "tail: {}", tail_vals.join(", "));
    }

    let text = out.trim_end().to_string();
    if text.len() >= content.len() {
        return None;
    }
    Some(CompressOutput::lossy(text, CompressorKind::SmartCrusher))
}

/// Collapse a plain-string array to deduped value/count pairs + head/tail.
fn crush_string_array(
    arr: &[Value],
    content: &str,
    block_tokens: bool,
    path: &str,
) -> Option<CompressOutput> {
    let strings: Vec<&str> = arr.iter().filter_map(Value::as_str).collect();
    if strings.len() < SCALAR_ARRAY_MIN {
        return None;
    }
    let n = strings.len();
    let mut freq: HashMap<&str, usize> = HashMap::new();
    let mut order: Vec<&str> = Vec::new();
    for s in &strings {
        if !freq.contains_key(s) {
            order.push(s);
        }
        *freq.entry(s).or_default() += 1;
    }
    let unique = order.len();

    let mut counts: Vec<(&str, usize)> = order.iter().map(|s| (*s, freq[s])).collect();
    counts.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(b.0)));
    const MAX_LISTED: usize = 12;

    let label = if path.is_empty() {
        String::new()
    } else {
        format!(" {path}")
    };
    let mut out = String::new();
    let _ = writeln!(
        out,
        "[json string array{label}: {n} values · {unique} unique]"
    );
    for (val, c) in counts.iter().take(MAX_LISTED) {
        let (cell, _) = truncate_cell(val);
        let _ = writeln!(out, "{} ×{c}", compact_json(&Value::String(cell)));
    }
    if unique > MAX_LISTED {
        let slice = serde_json::to_string(arr).unwrap_or_default();
        let token = block_token(&slice, block_tokens && !slice.is_empty());
        let _ = writeln!(
            out,
            "[... {} more unique value(s) ...{token}]",
            unique - MAX_LISTED
        );
        if !token.is_empty() {
            out.push_str(BLOCK_NOTE);
            out.push('\n');
        }
    }

    let text = out.trim_end().to_string();
    if text.len() >= content.len() {
        return None;
    }
    Some(CompressOutput::lossy(text, CompressorKind::SmartCrusher))
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/// First-seen union of object keys across an array of objects (stable order).
fn union_columns(array: &[Value]) -> Vec<String> {
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
    columns
}

/// First-seen union of sub-keys across a set of object values (stable order).
fn union_subkeys(values: &[&Value]) -> Vec<String> {
    let mut keys: Vec<String> = Vec::new();
    for v in values {
        if let Some(obj) = v.as_object() {
            for key in obj.keys() {
                if !keys.iter().any(|c| c == key) {
                    keys.push(key.clone());
                }
            }
        }
    }
    keys
}

fn is_name_like(key: &str) -> bool {
    let k = key.to_ascii_lowercase();
    matches!(
        k.as_str(),
        "name" | "id" | "title" | "key" | "slug" | "label" | "identifier"
    )
}

fn is_desc_like(key: &str) -> bool {
    let k = key.to_ascii_lowercase();
    matches!(
        k.as_str(),
        "description" | "desc" | "summary" | "comment" | "text" | "message" | "detail"
    )
}

fn is_plain_number(v: &Value) -> bool {
    v.is_number()
}

fn compact_json(v: &Value) -> String {
    serde_json::to_string(v).unwrap_or_default()
}

/// Format a float without a trailing `.0` for whole numbers.
fn fmt_num(x: f64) -> String {
    if x.fract() == 0.0 && x.abs() < 1e15 {
        format!("{}", x as i64)
    } else {
        let s = format!("{x:.3}");
        s.trim_end_matches('0').trim_end_matches('.').to_string()
    }
}

/// Render a single table cell. Scalars print bare-ish; nested values stay as
/// compact JSON so the table stays lossless (before any truncation).
fn render_cell(v: &Value) -> String {
    match v {
        Value::String(s) if !s.contains('|') && !s.contains('\n') => escape_markdown_cell(s),
        Value::Bool(b) => b.to_string(),
        Value::Number(n) => n.to_string(),
        other => escape_markdown_cell(&serde_json::to_string(other).unwrap_or_default()),
    }
}

/// Truncate an already-escaped cell to [`CELL_MAX`] chars, avoiding a dangling
/// escape backslash. Returns `(cell, was_truncated)`.
fn truncate_cell(s: &str) -> (String, bool) {
    if s.chars().count() <= CELL_MAX {
        return (s.to_string(), false);
    }
    let mut cut: String = s.chars().take(CELL_MAX).collect();
    while cut.ends_with('\\') {
        cut.pop();
    }
    cut.push('…');
    (cut, true)
}

/// Truncate a plain (unescaped) string for preamble rendering.
fn truncate_plain(s: &str, max: usize) -> String {
    if s.chars().count() <= max {
        s.to_string()
    } else {
        let mut cut: String = s.chars().take(max).collect();
        cut.push('…');
        cut
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
    fn crushes_uniform_array_hoists_constants() {
        // `status` and `owner` are identical across all rows → hoisted to a
        // preamble note and dropped from the table (feature 3).
        let mut rows = Vec::new();
        for i in 0..80 {
            rows.push(format!(
                r#"{{"id":{i},"name":"item number {i}","status":"active","owner":"team-alpha"}}"#
            ));
        }
        let input = format!("[{}]", rows.join(","));
        let out = compress(&input, false, None).expect("compresses").text;
        assert!(out.contains("[all rows:"), "{out}");
        assert!(out.contains(r#"status="active""#), "{out}");
        assert!(out.contains(r#"owner="team-alpha""#), "{out}");
        assert!(out.contains("| id | name |"), "{out}");
        assert_eq!(out.matches("| --- | --- |").count(), 1, "{out}");
        assert!(out.contains("item number 7"));
        assert!(out.len() < input.len(), "expected shrink");
    }

    #[test]
    fn large_array_row_drops_and_is_marked_lossy() {
        let mut rows = Vec::new();
        for i in 0..200 {
            rows.push(format!(
                r#"{{"id":{i},"name":"record number {i}","status":"s{i}","note":"some detail {i}"}}"#
            ));
        }
        let input = format!("[{}]", rows.join(","));
        let c = compress(&input, false, None).expect("compresses");
        assert!(c.lossy, "row-dropped output must be lossy");
        assert!(c.text.contains("record number 0"), "{}", c.text);
        assert!(c.text.contains("record number 199"), "{}", c.text);
        assert!(c.text.contains("omitted"));
        assert!(c.text.len() < input.len());
    }

    #[test]
    fn keeps_error_row_in_dropped_middle() {
        let mut rows = Vec::new();
        for i in 0..120 {
            let status = if i == 75 { "error: timeout" } else { "ok" };
            rows.push(format!(
                r#"{{"id":{i},"name":"job {i}","status":"{status}","note":"detail {i}"}}"#
            ));
        }
        let input = format!("[{}]", rows.join(","));
        let c = compress(&input, false, None).expect("compresses");
        assert!(c.lossy);
        assert!(
            c.text.contains("job 75"),
            "error row must survive:\n{}",
            c.text
        );
        assert!(c.text.contains("error: timeout"));
    }

    #[test]
    fn keeps_nested_error_row() {
        // Feature 2: error text lives inside a nested object, not a top-level
        // string. The row must still be kept.
        let mut rows = Vec::new();
        for i in 0..120 {
            let detail = if i == 63 {
                r#"{"code":"CONFLICT_PACKAGES","message":"resolution failed"}"#
            } else {
                r#"{"code":"OK","message":"done"}"#
            };
            rows.push(format!(
                r#"{{"id":{i},"name":"pkg {i}","result":{detail}}}"#
            ));
        }
        let input = format!("[{}]", rows.join(","));
        let c = compress(&input, false, None).expect("compresses");
        assert!(
            c.text.contains("pkg 63") || c.text.contains("CONFLICT_PACKAGES"),
            "nested error row must survive:\n{}",
            c.text
        );
    }

    #[test]
    fn keeps_numeric_outlier_row() {
        let mut rows = Vec::new();
        for i in 0..120 {
            let latency = if i == 88 { 9999 } else { 10 + (i % 3) };
            rows.push(format!(
                r#"{{"id":{i},"endpoint":"/api/{i}","latency_ms":{latency},"region":"r{i}"}}"#
            ));
        }
        let input = format!("[{}]", rows.join(","));
        let c = compress(&input, false, None).expect("compresses");
        assert!(
            c.text.contains("9999"),
            "outlier row must survive:\n{}",
            c.text
        );
    }

    #[test]
    fn keeps_pareto_rare_status_row() {
        // Feature 5: `status` is 99% "ok"; the single "degraded" row is a
        // categorical rarity that must be kept even in the drop window.
        let mut rows = Vec::new();
        for i in 0..120 {
            let status = if i == 70 { "degraded" } else { "ok" };
            rows.push(format!(
                r#"{{"id":{i},"endpoint":"/api/{i}","status":"{status}"}}"#
            ));
        }
        let input = format!("[{}]", rows.join(","));
        let c = compress(&input, false, None).expect("compresses");
        assert!(
            c.text.contains("degraded"),
            "rare categorical value must survive:\n{}",
            c.text
        );
    }

    #[test]
    fn keeps_rare_structural_field_row() {
        // Feature 6: only one row carries `escalation` → kept.
        let mut rows = Vec::new();
        for i in 0..120 {
            if i == 55 {
                rows.push(format!(
                    r#"{{"id":{i},"name":"t{i}","escalation":"paged oncall"}}"#
                ));
            } else {
                rows.push(format!(r#"{{"id":{i},"name":"t{i}"}}"#));
            }
        }
        let input = format!("[{}]", rows.join(","));
        let c = compress(&input, false, None).expect("compresses");
        assert!(
            c.text.contains("paged oncall") || c.text.contains("t55"),
            "rare-field row must survive:\n{}",
            c.text
        );
    }

    #[test]
    fn keeps_query_anchor_row() {
        // Feature 8: a distinctive id in the query pins the matching row.
        let mut rows = Vec::new();
        for i in 0..120 {
            rows.push(format!(
                r#"{{"id":{i},"name":"widget {i}","ticket":{}}}"#,
                5000 + i
            ));
        }
        let input = format!("[{}]", rows.join(","));
        let c =
            compress(&input, false, Some("investigate ticket 5087 failure")).expect("compresses");
        assert!(
            c.text.contains("widget 87"),
            "anchored row must survive:\n{}",
            c.text
        );
    }

    #[test]
    fn flattens_uniform_object_column() {
        // Feature 4: the `function` column is a uniform object; name/description
        // are promoted to columns instead of a per-row JSON blob.
        let mut rows = Vec::new();
        for i in 0..30 {
            rows.push(format!(
                r#"{{"type":"function","function":{{"name":"TOOL_{i}","description":"does thing {i}","parameters":{{"type":"object"}}}}}}"#
            ));
        }
        let input = format!("[{}]", rows.join(","));
        let out = compress(&input, false, None).expect("compresses").text;
        assert!(out.contains("function.name"), "{out}");
        assert!(out.contains("function.description"), "{out}");
        assert!(out.contains("TOOL_3"), "{out}");
        // `type` is constant → hoisted, not a column.
        assert!(out.contains(r#"type="function""#), "{out}");
    }

    #[test]
    fn tables_nested_array_with_path_prefix() {
        // Feature 1: top-level object whose largest uniform-object array is
        // nested under `methods`.
        let mut methods = Vec::new();
        for i in 0..69 {
            methods.push(format!(
                r#"{{"method":"core.m{i}","namespace":"core","description":"method {i}"}}"#
            ));
        }
        let input = format!(r#"{{"version":"1.0","methods":[{}]}}"#, methods.join(","));
        let out = compress(&input, false, None).expect("compresses").text;
        assert!(out.contains("[json table: methods —"), "{out}");
        assert!(out.contains("69 rows"), "{out}");
        // Sibling scalar rendered as preamble.
        assert!(out.contains("version: 1.0"), "{out}");
    }

    #[test]
    fn crushes_number_array() {
        // Feature 7: plain-number array → distributional stats.
        let nums: Vec<String> = (0..200).map(|i| (i * 3).to_string()).collect();
        let input = format!("[{}]", nums.join(","));
        let c = compress(&input, false, None).expect("compresses");
        assert!(c.text.contains("json number array"), "{}", c.text);
        assert!(c.text.contains("min=0"), "{}", c.text);
        assert!(c.text.contains("max=597"), "{}", c.text);
        assert!(c.text.contains("median="), "{}", c.text);
        assert!(c.lossy);
        assert!(c.text.len() < input.len());
    }

    #[test]
    fn crushes_string_array_with_dedupe() {
        // Feature 7: plain-string array → deduped counts.
        let mut vals = Vec::new();
        for _ in 0..50 {
            vals.push("\"apple\"".to_string());
        }
        for _ in 0..30 {
            vals.push("\"banana\"".to_string());
        }
        let input = format!("[{}]", vals.join(","));
        let c = compress(&input, false, None).expect("compresses");
        assert!(c.text.contains("json string array"), "{}", c.text);
        assert!(c.text.contains("2 unique"), "{}", c.text);
        assert!(c.text.contains("×50"), "{}", c.text);
        assert!(c.text.len() < input.len());
    }

    #[test]
    fn non_crushable_returns_none() {
        assert!(compress(r#"{"a":1}"#, false, None).is_none());
        assert!(compress("[1,2,3]", false, None).is_none());
        assert!(compress(r#"[{"a":1}]"#, false, None).is_none());
    }

    #[test]
    fn markdown_table_cells_escape_pipes() {
        // `meta` has only a non-informative `note` sub-key → NOT flattened, so
        // the nested-JSON escaping path stays exercised.
        let mut rows = vec![r#"{"id":0,"text":"alpha | beta","meta":{"note":"x|y"}}"#.to_string()];
        for i in 1..80 {
            rows.push(format!(
                r#"{{"id":{i},"text":"record {i} with repeated detail","meta":{{"note":"z"}}}}"#
            ));
        }
        let input = format!("[{}]", rows.join(","));
        let out = compress(&input, false, None).expect("compresses").text;
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
        let out = compress(input, false, None).expect("minifies").text;
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
        let out = compress(input, false, None).expect("minifies").text;
        assert_eq!(
            out,
            r#"[{"enabled":true,"id":1},{"enabled":false,"id":2},{"enabled":true,"id":3},{"enabled":false,"id":4},{"enabled":true,"id":5}]"#
        );
    }

    #[test]
    fn omitted_rows_token_retrieves_json_slice() {
        use crate::cache;
        let mut items = Vec::new();
        for i in 0..120 {
            items.push(format!(r#"{{"id":{i},"name":"row_{i}","status":"st{i}"}}"#));
        }
        let input = format!("[{}]", items.join(","));
        let out = compress(&input, true, None).expect("compresses").text;
        let tokens = cache::parse_markers(&out);
        assert!(!tokens.is_empty(), "row gaps should carry tokens: {out}");
        let block = cache::retrieve(&tokens[0]).expect("stored");
        let parsed: serde_json::Value = serde_json::from_str(&block).expect("valid JSON slice");
        let arr = parsed.as_array().expect("array slice");
        assert!(!arr.is_empty());
        // The slice starts exactly where the adaptive head window ends, and
        // none of its rows also appear in the rendered table.
        let first_id = arr[0]["id"].as_u64().expect("id") as usize;
        assert!(first_id > 0, "head must keep at least one row");
        assert_eq!(arr[0]["name"], format!("row_{first_id}"));
        assert!(
            out.contains(&format!("row_{}", first_id - 1)),
            "row before the gap must be shown:\n{out}"
        );
        for row in arr {
            let name = row["name"].as_str().expect("name");
            assert!(
                !out.contains(&format!("| {name} |")),
                "omitted row {name} must not also be rendered:\n{out}"
            );
        }
    }

    /// Count rendered table body rows (data rows, excluding header/separator).
    fn body_row_count(text: &str) -> usize {
        text.lines()
            .filter(|l| l.starts_with("| ") && !l.starts_with("| ---"))
            .count()
            .saturating_sub(1) // header
    }

    #[test]
    fn redundant_rows_collapse_toward_floor() {
        // 100 near-identical rows: adaptive selection should keep close to
        // MIN_KEPT_ROWS, far below the fixed HEAD_ROWS+TAIL_ROWS budget.
        let mut rows = Vec::new();
        for i in 0..100 {
            rows.push(format!(
                r#"{{"id":{i},"msg":"connection timeout retrying request to upstream host","status":"ok"}}"#
            ));
        }
        let input = format!("[{}]", rows.join(","));
        let c = compress(&input, false, None).expect("compresses");
        let kept = body_row_count(&c.text);
        assert!(
            kept >= MIN_KEPT_ROWS,
            "never fewer than the floor, got {kept}:\n{}",
            c.text
        );
        assert!(
            kept <= MIN_KEPT_ROWS + 4,
            "redundant rows must stay near the floor, got {kept}:\n{}",
            c.text
        );
        assert!(c.text.contains("omitted"), "{}", c.text);
    }

    #[test]
    fn diverse_rows_keep_full_budget() {
        // 100 fully-diverse rows: adaptive selection should keep the full
        // HEAD_ROWS + TAIL_ROWS budget.
        let words = [
            "database",
            "login",
            "cache",
            "worker",
            "handshake",
            "latency",
            "rollout",
            "webhook",
            "resolver",
            "kernel",
            "exporter",
            "backlog",
            "renewal",
            "election",
            "limiter",
            "index",
            "cookie",
            "saturated",
            "multipart",
            "deadline",
        ];
        let mut rows = Vec::new();
        for i in 0..100 {
            rows.push(format!(
                r#"{{"event":"{} {} {} incident {i}","code":"{}{}","weight":{}}}"#,
                words[i % 20],
                words[(i * 7 + 3) % 20],
                words[(i * 13 + 11) % 20],
                words[(i * 3 + 5) % 20],
                i * 977,
                i * i + 17,
            ));
        }
        let input = format!("[{}]", rows.join(","));
        let c = compress(&input, false, None).expect("compresses");
        let kept = body_row_count(&c.text);
        assert!(
            kept >= HEAD_ROWS + TAIL_ROWS,
            "diverse rows should keep the full budget, got {kept}:\n{}",
            c.text
        );
    }

    #[test]
    fn must_keep_rows_are_additive_over_adaptive_window() {
        // Redundant array (small adaptive window) with an error row buried in
        // the dropped middle: the error row is kept ON TOP of the window.
        let mut rows = Vec::new();
        for i in 0..100 {
            let msg = if i == 60 {
                "error: upstream exploded"
            } else {
                "connection timeout retrying request to upstream host"
            };
            rows.push(format!(r#"{{"id":{i},"msg":"{msg}","status":"ok"}}"#));
        }
        let input = format!("[{}]", rows.join(","));
        let c = compress(&input, false, None).expect("compresses");
        assert!(
            c.text.contains("error: upstream exploded"),
            "must-keep error row must survive the adaptive window:\n{}",
            c.text
        );
        let kept = body_row_count(&c.text);
        assert!(
            kept <= MIN_KEPT_ROWS + 5,
            "window should stay small even with the extra must-keep, got {kept}:\n{}",
            c.text
        );
    }
}
