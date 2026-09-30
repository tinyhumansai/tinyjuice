use tinyjuice::repl::{handle_view, ops, ReplLimits};
use tinyjuice::types::ContentKind;

fn main() {
    let items: Vec<String> = (0..60)
        .map(|i| format!(r#"{{"id":{i},"name":"repo-{i}","stars":{},"owner":{{"login":"org{}"}},"description":"A sample repository number {i} with a long description"}}"#, i * 13, i % 7))
        .collect();
    let json = format!(r#"{{"total_count":60,"incomplete_results":false,"items":[{}]}}"#, items.join(","));
    let old = ops::summarize(&json, 1200, &ReplLimits::default());
    let (body, footer, _) = handle_view(&json, "abc123def456", ContentKind::Json, 4.0, 1200, Some(std::path::Path::new("/work/.tinyjuice/repl/abc123def456.txt")));
    println!("INPUT_BYTES {}", json.len());
    println!("=====OLD\n{old}");
    println!("=====NEW\n{body}{footer}");
}
