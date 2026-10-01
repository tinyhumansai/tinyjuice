//! ANSI / VT escape-sequence stripping.
//!
//! Port of `src/core/text.ts` strip logic.

use once_cell::sync::Lazy;
use regex::Regex;

// CSI: ESC [ … final-byte
static ANSI_CSI: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"\x1b\[[0-?]*[ -/]*[@-~]").expect("ansi csi regex"));

// OSC: ESC ] … BEL or ESC backslash
static ANSI_OSC: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"\x1b\][^\x07\x1b]*(?:\x07|\x1b\\)").expect("ansi osc regex"));

// Incomplete CSI at end of string
static ANSI_CSI_INCOMPLETE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"\x1b\[[0-?]*[ -/]*$").expect("ansi csi incomplete regex"));

// Incomplete OSC at end of string
static ANSI_OSC_INCOMPLETE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"\x1b\][^\x07\x1b]*$").expect("ansi osc incomplete regex"));

// Single-char escapes: ESC followed by @-_
static ANSI_SINGLE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"\x1b[@-_]").expect("ansi single regex"));

/// Strip all ANSI/VT escape sequences from `text`.
pub fn strip_ansi(text: &str) -> String {
    let input_len = text.len();
    let s = ANSI_OSC.replace_all(text, "");
    let s = ANSI_CSI.replace_all(&s, "");
    let s = ANSI_OSC_INCOMPLETE.replace_all(&s, "");
    let s = ANSI_CSI_INCOMPLETE.replace_all(&s, "");
    let s = ANSI_SINGLE.replace_all(&s, "");
    // Remove any lone ESC bytes that slipped through
    let out = s.replace('\x1b', "");
    log::trace!(
        "[tinyjuice] strip_ansi in_len={} out_len={}",
        input_len,
        out.len()
    );
    out
}

#[cfg(test)]
#[path = "ansi_tests.rs"]
mod tests;
