//! Deterministic synthetic tool outputs, each with a planted fact and its answer.
//! Synthetic on purpose: no third-party content is vendored, and runs reproduce.

pub struct Scenario {
    pub name: &'static str,
    /// The tool the agent must call to get the payload.
    pub tool: &'static str,
    pub tool_description: &'static str,
    pub question: &'static str,
    /// Substring (case-insensitive) the final answer must contain.
    pub expected: &'static str,
    pub payload: fn() -> String,
}

pub const SCENARIOS: &[Scenario] = &[
    Scenario {
        name: "notion_rollback",
        tool: "notion_fetch_blocks",
        tool_description: "Fetch the blocks of the release-notes page.",
        question: "Fetch the release-notes page blocks. What is the exact name of the rollback flag it mentions?",
        expected: "ROLLBACK_SAFE_MODE",
        payload: notion_blocks,
    },
    Scenario {
        name: "github_top_score",
        tool: "github_search_code",
        tool_description: "Search code on GitHub for 'retry budget'.",
        question: "Search the code for 'retry budget'. Which repository has the highest score?",
        expected: "acme/zeta-engine",
        payload: github_search,
    },
    Scenario {
        name: "web_seventh_heading",
        tool: "web_fetch",
        tool_description: "Fetch the docs page https://docs.example.com/guide.",
        question: "Fetch https://docs.example.com/guide. What is the title of its 7th section heading (the 7th h2)?",
        expected: "Quorum Reconfiguration",
        payload: web_page,
    },
    Scenario {
        name: "log_timeout_request",
        tool: "shell",
        tool_description: "Run `docker logs app` and return the output.",
        question: "Run the log command. Which request id failed with an upstream timeout?",
        expected: "req-7f3a91c2",
        payload: app_log,
    },
    Scenario {
        name: "log_final_status",
        tool: "shell",
        tool_description: "Run `docker logs app` and return the output.",
        question: "Run the log command. What was the final shutdown status at the very end of the log? Earlier shutdown lines are from a previous run and do not count.",
        expected: "SHUTDOWN_CLEAN",
        payload: app_log,
    },
];

fn notion_blocks() -> String {
    let blocks: Vec<serde_json::Value> = (0..260)
        .map(|i| {
            let text = if i == 181 {
                "NEEDLE-7731: the rollback flag is ROLLBACK_SAFE_MODE and must be set before deploy.".to_string()
            } else {
                format!(
                    "Finding {i}: module m{} handles request class r{} with retry budget {}; owner team t{}.",
                    i % 23,
                    i % 7,
                    i * 3 % 11,
                    i % 5
                )
            };
            let kind = ["paragraph", "heading_2", "bulleted_list_item", "code"][i % 4];
            serde_json::json!({
                "id": format!("b{i:04}-aaaa-bbbb-cccc-{i:012}"),
                "type": kind,
                "has_children": false,
                "created_time": format!("2026-09-2{}T10:{:02}:00.000Z", i % 9, i % 60),
                "last_edited_by": { "object": "user", "id": "u-1234" },
                "paragraph": { "rich_text": [{
                    "type": "text",
                    "text": { "content": text, "link": null },
                    "annotations": { "bold": false, "italic": false, "code": false, "color": "default" },
                    "plain_text": text,
                    "href": null
                }], "color": "default" }
            })
        })
        .collect();
    serde_json::json!({ "data": { "results": blocks }, "successful": true }).to_string()
}

fn github_search() -> String {
    let items: Vec<serde_json::Value> = (0..110)
        .map(|i| {
            let (repo, score) = if i == 77 {
                ("acme/zeta-engine".to_string(), 99.9)
            } else {
                (format!("org{}/svc-{}", i % 13, i), 40.0 + (i % 37) as f64)
            };
            let fragment = format!(
                "// retry budget handling for upstream {i}\nfn retry_{i}(budget: u32) -> Result<(), Error> {{\n    if budget == 0 {{ return Err(Error::Exhausted) }}\n    // backoff grows with attempt {}\n    Ok(())\n}}\n",
                i % 9
            )
            .repeat(3);
            serde_json::json!({
                "name": format!("retry_{i}.rs"),
                "path": format!("src/net/retry_{i}.rs"),
                "sha": format!("{:040x}", i * 7919),
                "url": format!("https://api.github.com/repositories/{}/contents/src/net/retry_{i}.rs", 1000 + i),
                "repository": { "full_name": repo, "private": false, "fork": i % 5 == 0 },
                "score": score,
                "text_matches": [{ "object_type": "FileContent", "fragment": fragment, "matches": [{ "text": "retry budget", "indices": [3, 15] }] }]
            })
        })
        .collect();
    serde_json::json!({ "total_count": 110, "incomplete_results": false, "items": items }).to_string()
}

fn web_page() -> String {
    let topics = [
        "Overview", "Installation", "Leader Election", "Log Replication", "Snapshots", "Membership Changes",
        "Quorum Reconfiguration", "Failure Detection", "Client Sessions", "Observability", "Upgrades", "Glossary",
    ];
    let mut out = String::from("<html><head><title>Consensus guide</title></head><body><h1>Consensus guide</h1>");
    for (i, topic) in topics.iter().enumerate() {
        out.push_str(&format!("<h2>{topic}</h2>"));
        for p in 0..14 {
            out.push_str(&format!(
                "<p>Paragraph {p} of {topic}: nodes exchange heartbeats and <a href=\"https://docs.example.com/ref/{i}/{p}\">reference {i}.{p}</a> explains term {} handling, with <code>timeout_{p}</code> tuned per cluster.</p>",
                (i * 14 + p) % 17
            ));
        }
    }
    out.push_str("</body></html>");
    out
}

fn app_log() -> String {
    let mut out = String::new();
    for i in 0..5000 {
        let line = match i {
            2000 => "2026-09-30T12:33:20Z INFO worker-2 shutdown: SHUTDOWN_FAILED code=1 (previous run)".to_string(),
            3100 => "2026-09-30T12:51:40Z ERROR worker-5 request req-7f3a91c2 failed: upstream timeout after 30000ms".to_string(),
            4995 => "2026-09-30T13:23:15Z INFO worker-1 draining connections".to_string(),
            4998 => "2026-09-30T13:23:18Z INFO worker-1 shutdown: SHUTDOWN_CLEAN code=0".to_string(),
            _ => format!(
                "2026-09-30T12:{:02}:{:02}Z INFO worker-{} handled request req-{:08x} in {}ms",
                (i / 60) % 60,
                i % 60,
                i % 8,
                (i as u64) * 2_654_435_761 % 0xffff_ffff,
                20 + i % 40
            ),
        };
        out.push_str(&line);
        out.push('\n');
    }
    out
}
