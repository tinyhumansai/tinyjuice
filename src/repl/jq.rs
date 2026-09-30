//! jq-style queries over a JSON output, via `jaq` (a jq clone).
//!
//! Hardening: programs that could read the host environment or stdin are
//! rejected, output is capped, and evaluation runs under a wall-clock deadline.
//! A filter that loops without producing output cannot be interrupted, so the
//! deadline abandons its worker thread rather than killing it.

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, LazyLock};
use std::time::Duration;

use jaq_core::load::{Arena, File, Loader};
use jaq_core::{Compiler, Ctx, data, unwrap_valr};
use jaq_json::{Val, read};
use regex::Regex;

use super::types::{ReplError, ReplLimits};

const DEADLINE: Duration = Duration::from_secs(2);

/// Builtins that reach outside the value: environment, stdin, stderr, exit.
static FORBIDDEN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r"\b(env|ENV|input|inputs|input_filename|debug|stderr|halt|halt_error|get_search_list)\b",
    )
    .unwrap()
});

fn bad(msg: impl std::fmt::Display) -> ReplError {
    ReplError::InvalidPattern(format!("jq: {msg}"))
}

/// Evaluate `expr` against the JSON in `text`. Returns the output values
/// (compact JSON strings) and how many were dropped by the cap.
pub fn run(text: &str, expr: &str, limits: &ReplLimits) -> Result<(Vec<String>, usize), ReplError> {
    if expr.trim().is_empty() {
        return Err(ReplError::EmptyQuery);
    }
    if FORBIDDEN.is_match(expr) {
        return Err(bad("environment, input and debug builtins are not allowed"));
    }
    let (text, expr, max_hits) = (text.to_string(), expr.to_string(), limits.max_hits);
    let cancel = Arc::new(AtomicBool::new(false));
    let worker_cancel = Arc::clone(&cancel);
    let (tx, rx) = std::sync::mpsc::channel();
    std::thread::spawn(move || {
        // jaq values are `Rc`-based, so they are built on the worker thread.
        let result = read::parse_single(text.as_bytes())
            .map_err(|_| bad("the output is not valid JSON"))
            .and_then(|input| evaluate(&expr, input, max_hits, &worker_cancel));
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
            Ok(v) if values.len() < max_hits => values.push(v.to_string()),
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
