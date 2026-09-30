//! Prints the model-facing spec of each REPL tool and its size.
//! `cargo run --example repl_tool_schema --features tinytools`

use std::sync::Arc;

use tinyjuice::cache::MemoryCcrStore;
use tinyjuice::repl::{ReplLimits, tools::repl_tools};

fn main() {
    let tools = repl_tools(Arc::new(MemoryCcrStore::default()), ReplLimits::default());
    let mut total = 0;
    for tool in &tools {
        let spec = serde_json::to_string(&tool.spec()).unwrap();
        total += spec.len();
        println!("{:<24} {:>5} bytes", tool.name(), spec.len());
        if std::env::args().any(|a| a == "--show") {
            println!("{spec}");
        }
    }
    println!(
        "{:<24} {:>5} bytes (~{} tokens at 4 chars/token)",
        "TOTAL",
        total,
        total / 4
    );
}
