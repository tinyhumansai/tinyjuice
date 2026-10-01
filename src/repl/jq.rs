//! jq-style queries over a JSON output, via `jaq` (a jq clone).
//!
//! Hardening: programs that could read the host environment or stdin are
//! rejected, output is capped, and evaluation runs under a wall-clock deadline.
//! A filter that loops without producing output cannot be interrupted, so the
//! deadline abandons its worker thread rather than killing it.

use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::{Arc, LazyLock};
use std::time::Duration;

use jaq_core::load::{Arena, File, Loader};
use jaq_core::{Compiler, Ctx, data, unwrap_valr};
use jaq_json::{Val, read};
use regex::Regex;

use super::types::{ReplError, ReplLimits};

const DEADLINE: Duration = Duration::from_secs(2);

/// Most jq workers that may be alive at once. A timed-out worker cannot be
/// killed, so new queries are refused while abandoned ones still occupy slots.
const MAX_LIVE_WORKERS: usize = 4;
static LIVE_WORKERS: AtomicUsize = AtomicUsize::new(0);

/// Holds one worker slot; released when the worker thread exits.
struct WorkerSlot;

impl WorkerSlot {
    fn acquire() -> Option<Self> {
        let mut current = LIVE_WORKERS.load(Ordering::Acquire);
        loop {
            if current >= MAX_LIVE_WORKERS {
                return None;
            }
            match LIVE_WORKERS.compare_exchange_weak(
                current,
                current + 1,
                Ordering::AcqRel,
                Ordering::Acquire,
            ) {
                Ok(_) => return Some(Self),
                Err(observed) => current = observed,
            }
        }
    }
}

impl Drop for WorkerSlot {
    fn drop(&mut self) {
        LIVE_WORKERS.fetch_sub(1, Ordering::AcqRel);
    }
}

/// Builtins that reach outside the value: environment, stdin, stderr, exit.
static FORBIDDEN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r"\b(env|ENV|input|inputs|input_filename|debug|stderr|halt|halt_error|get_search_list)\b",
    )
    .unwrap()
});

/// String literals without interpolation, `.field` accesses and `key:` object
/// keys: names inside these are data, not builtin calls.
static DATA_NAMES: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#""(?:[^"\\]|\\[^(])*"|\.\s*[A-Za-z_][A-Za-z0-9_]*|[A-Za-z_][A-Za-z0-9_]*\s*:"#)
        .unwrap()
});

/// Whether `expr` calls a forbidden builtin, ignoring names used as data.
fn references_forbidden(expr: &str) -> bool {
    FORBIDDEN.is_match(&DATA_NAMES.replace_all(expr, " "))
}

fn bad(msg: impl std::fmt::Display) -> ReplError {
    ReplError::InvalidPattern(format!("jq: {msg}"))
}

/// Evaluate `expr` against the JSON in `text`. Returns the output values
/// (compact JSON strings) and how many were dropped by the cap.
pub fn run(text: &str, expr: &str, limits: &ReplLimits) -> Result<(Vec<String>, usize), ReplError> {
    if expr.trim().is_empty() {
        return Err(ReplError::EmptyQuery);
    }
    if references_forbidden(expr) {
        return Err(bad("environment, input and debug builtins are not allowed"));
    }
    let Some(slot) = WorkerSlot::acquire() else {
        return Err(bad("too many earlier queries are still running; try later"));
    };
    let (text, expr, max_hits, max_chars) = (
        text.to_string(),
        expr.to_string(),
        limits.max_hits,
        limits.max_output_chars,
    );
    let cancel = Arc::new(AtomicBool::new(false));
    let worker_cancel = Arc::clone(&cancel);
    let (tx, rx) = std::sync::mpsc::channel();
    std::thread::spawn(move || {
        let _slot = slot;
        // jaq values are `Rc`-based, so they are built on the worker thread.
        let result = read::parse_single(text.as_bytes())
            .map_err(|_| bad("the output is not valid JSON"))
            .and_then(|input| evaluate(&expr, input, max_hits, max_chars, &worker_cancel));
        let _ = tx.send(result);
    });
    match rx.recv_timeout(DEADLINE) {
        Ok(result) => result,
        Err(_) => {
            cancel.store(true, Ordering::Relaxed);
            Err(bad("query timed out; narrow it"))
        }
    }
}

fn evaluate(
    expr: &str,
    input: Val,
    max_hits: usize,
    max_chars: usize,
    cancel: &AtomicBool,
) -> Result<(Vec<String>, usize), ReplError> {
    let defs = jaq_core::defs()
        .chain(jaq_std::defs())
        .chain(jaq_json::defs());
    let funs = jaq_core::funs()
        .chain(jaq_std::funs())
        .chain(jaq_json::funs());
    let arena = Arena::default();
    let modules = Loader::new(defs)
        .load(
            &arena,
            File {
                code: expr,
                path: (),
            },
        )
        .map_err(|_| bad("syntax error"))?;
    let filter = Compiler::default()
        .with_funs(funs)
        .compile(modules)
        .map_err(|_| bad("unknown function or bad arguments"))?;
    let ctx = Ctx::<data::JustLut<Val>>::new(&filter.lut, jaq_core::Vars::new([]));
    let mut values = Vec::new();
    let mut dropped = 0usize;
    for out in filter.id.run((ctx, input)).map(unwrap_valr) {
        if cancel.load(Ordering::Relaxed) {
            break;
        }
        match out {
            Ok(v) if values.len() < max_hits => {
                let s = v.to_string();
                values.push(if s.chars().count() > max_chars {
                    s.chars().take(max_chars).collect::<String>() + "…"
                } else {
                    s
                });
            }
            Ok(_) => {
                dropped += 1;
                // Stop counting once it is clear the query is too broad.
                if dropped >= 10_000 {
                    break;
                }
            }
            Err(e) => return Err(bad(e)),
        }
    }
    Ok((values, dropped))
}
