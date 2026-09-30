//! The host `Generate` callback behind TinyJuice's summary stage, over OpenRouter.
//! The key reaches `curl` through a 0600 config file, never argv or logs.

use std::io::Write as _;
use std::os::unix::fs::OpenOptionsExt as _;
use std::sync::Arc;

use tinyjuice::llm::{GenerateCallback, GenerateRequest};

/// Makes each curl config file name unique within this process.
static CFG_SEQ: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);

pub fn callback(model: String, key: String) -> Arc<GenerateCallback> {
    Arc::new(move |req: GenerateRequest| {
        let (model, key) = (model.clone(), key.clone());
        Box::pin(async move {
            let body = serde_json::json!({
                "model": model,
                "max_tokens": req.max_output_tokens,
                "messages": [
                    { "role": "system", "content": req.system },
                    { "role": "user", "content": req.prompt },
                ],
            });
            tokio::task::spawn_blocking(move || {
                let cfg = std::env::temp_dir().join(format!(
                    "tj-agent-{}-{}.cfg",
                    std::process::id(),
                    CFG_SEQ.fetch_add(1, std::sync::atomic::Ordering::Relaxed)
                ));
                std::fs::OpenOptions::new()
                    .write(true)
                    .create_new(true)
                    .mode(0o600)
                    .open(&cfg)
                    .and_then(|mut f| f.write_all(format!("header = \"Authorization: Bearer {key}\"\n").as_bytes()))
                    .map_err(|e| e.to_string())?;
                let mut child = std::process::Command::new("curl")
                    .args(["-sS", "--max-time", "120", "https://openrouter.ai/api/v1/chat/completions"])
                    .args(["-H", "Content-Type: application/json", "-K"])
                    .arg(&cfg)
                    .args(["-d", "@-"])
                    .stdin(std::process::Stdio::piped())
                    .stdout(std::process::Stdio::piped())
                    .stderr(std::process::Stdio::null())
                    .spawn()
                    .map_err(|e| e.to_string())?;
                child
                    .stdin
                    .take()
                    .ok_or("no stdin")?
                    .write_all(body.to_string().as_bytes())
                    .map_err(|e| e.to_string())?;
                let out = child.wait_with_output().map_err(|e| e.to_string())?;
                let _ = std::fs::remove_file(&cfg);
                let v: serde_json::Value = serde_json::from_slice(&out.stdout).map_err(|e| e.to_string())?;
                // An empty reply (reasoning ate the token cap) is an unusable summary.
                Ok(v["choices"][0]["message"]["content"].as_str().map(str::to_string))
            })
            .await
            .map_err(|e| e.to_string())?
        })
    })
}
