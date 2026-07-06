use std::fs;
use std::io::Write;
use std::process::{Command, Stdio};

fn tinyjuice() -> Command {
    Command::new(env!("CARGO_BIN_EXE_tinyjuice"))
}

#[test]
fn reduce_compacts_stdin_with_command_metadata() {
    let mut child = tinyjuice()
        .args(["reduce", "--tool-name", "bash", "--command", "git status"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("spawn tinyjuice reduce");

    child
        .stdin
        .as_mut()
        .expect("stdin")
        .write_all(b"On branch main\n\nChanges not staged for commit:\n\tmodified:   src/lib.rs\n")
        .expect("write stdin");

    let output = child.wait_with_output().expect("wait tinyjuice reduce");
    assert!(output.status.success(), "{output:#?}");
    assert_eq!(
        String::from_utf8(output.stdout).expect("utf8 stdout"),
        "Changes not staged:\nM: src/lib.rs"
    );
}

#[test]
fn reduce_json_prints_response_json() {
    let mut child = tinyjuice()
        .args(["reduce-json"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("spawn tinyjuice reduce-json");

    child
        .stdin
        .as_mut()
        .expect("stdin")
        .write_all(
            br#"{
                "toolName": "bash",
                "argv": ["git", "status"],
                "stdout": "On branch main\n\nChanges not staged for commit:\n\tmodified:   src/lib.rs\n"
            }"#,
        )
        .expect("write stdin");

    let output = child
        .wait_with_output()
        .expect("wait tinyjuice reduce-json");
    assert!(output.status.success(), "{output:#?}");
    let response: serde_json::Value =
        serde_json::from_slice(&output.stdout).expect("response json");
    assert_eq!(response["inlineText"], "Changes not staged:\nM: src/lib.rs");
    assert_eq!(response["classification"]["matchedReducer"], "git/status");
}

#[test]
fn reduce_json_rejects_malformed_payload_with_structured_error() {
    let mut child = tinyjuice()
        .args(["reduce-json"])
        .stdin(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("spawn tinyjuice reduce-json");

    child
        .stdin
        .as_mut()
        .expect("stdin")
        .write_all(br#"{"toolName":"bash","stdout":"secret""#)
        .expect("write stdin");

    let output = child
        .wait_with_output()
        .expect("wait tinyjuice reduce-json");
    assert!(!output.status.success(), "{output:#?}");
    let error: serde_json::Value = serde_json::from_slice(&output.stderr).expect("error json");
    assert_eq!(error["code"], "invalid_json");
}

#[test]
fn verify_rules_and_fixtures_reports_counts() {
    let output = tinyjuice()
        .args(["verify", "--rules", "--fixtures"])
        .output()
        .expect("run tinyjuice verify");

    assert!(output.status.success(), "{output:#?}");
    let stdout = String::from_utf8(output.stdout).expect("utf8 stdout");
    assert!(
        stdout.contains("rules: descriptors=101 valid=101 final=101"),
        "{stdout}"
    );
    assert!(
        stdout.contains("fixtures: dir=tests/fixtures seen=5 passed=5"),
        "{stdout}"
    );
}

#[test]
fn discover_reports_fallback_families_without_raw_output() {
    let mut child = tinyjuice()
        .args(["discover"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("spawn tinyjuice discover");

    child
        .stdin
        .as_mut()
        .expect("stdin")
        .write_all(
            br#"{"toolName":"bash","argv":["unknown_tool"],"stdout":"raw secret output"}
{"toolName":"bash","argv":["git","status"],"stdout":"On branch main"}
"#,
        )
        .expect("write stdin");

    let output = child.wait_with_output().expect("wait tinyjuice discover");
    assert!(output.status.success(), "{output:#?}");
    let response: serde_json::Value =
        serde_json::from_slice(&output.stdout).expect("discovery json");
    assert_eq!(response["inputsSeen"], 2);
    assert_eq!(response["fallbackOutputs"], 1);
    assert_eq!(response["families"][0]["toolName"], "bash");
    assert_eq!(response["families"][0]["argv0"], "unknown_tool");
    let stdout = String::from_utf8(output.stdout).expect("utf8 stdout");
    assert!(!stdout.contains("raw secret output"), "{stdout}");
}

#[test]
fn wrap_runs_command_and_reduces_output() {
    let mut script = tempfile::NamedTempFile::new().expect("temp script");
    writeln!(
        script,
        "i=1; while [ $i -le 80 ]; do echo line $i; i=$((i + 1)); done"
    )
    .expect("write script");
    let script_path = script.path().to_string_lossy().into_owned();

    let output = tinyjuice()
        .args([
            "wrap",
            "--max-inline-chars",
            "120",
            "--",
            "sh",
            &script_path,
        ])
        .output()
        .expect("run tinyjuice wrap");

    assert!(output.status.success(), "{output:#?}");
    let stdout = String::from_utf8(output.stdout).expect("utf8 stdout");
    assert!(stdout.len() <= 120, "{stdout}");
    assert!(stdout.contains("omitted"), "{stdout}");
    assert!(!stdout.contains("line 40\nline 41"), "{stdout}");
}

#[test]
fn wrap_preserves_wrapped_exit_code() {
    let output = tinyjuice()
        .args(["wrap", "--", "sh", "-c", "printf 'line 1\\n'; exit 7"])
        .output()
        .expect("run tinyjuice wrap failure");

    assert_eq!(output.status.code(), Some(7), "{output:#?}");
    assert_eq!(
        String::from_utf8(output.stdout).expect("utf8 stdout"),
        "line 1\n"
    );
}

#[test]
fn ls_lists_only_token_shaped_store_entries() {
    let dir = tempfile::tempdir().expect("temp store");
    fs::write(dir.path().join("0123456789abcdef0123456789abcdef"), "one").expect("write token");
    fs::write(dir.path().join("not-a-token"), "ignored").expect("write ignored");

    let output = tinyjuice()
        .args(["ls", "--store-dir", &dir.path().to_string_lossy()])
        .output()
        .expect("run tinyjuice ls");

    assert!(output.status.success(), "{output:#?}");
    assert_eq!(
        String::from_utf8(output.stdout).expect("utf8 stdout"),
        "0123456789abcdef0123456789abcdef\n"
    );
}

#[test]
fn cat_reads_token_from_explicit_store_dir() {
    let dir = tempfile::tempdir().expect("temp store");
    let token = "0123456789abcdef0123456789abcdef";
    fs::write(dir.path().join(token), "alpha\nbeta\ngamma\n").expect("write token");

    let output = tinyjuice()
        .args(["cat", "--store-dir", &dir.path().to_string_lossy(), token])
        .output()
        .expect("run tinyjuice cat");

    assert!(output.status.success(), "{output:#?}");
    assert_eq!(
        String::from_utf8(output.stdout).expect("utf8 stdout"),
        "alpha\nbeta\ngamma\n"
    );
}

#[test]
fn cat_reads_line_range_from_explicit_store_dir() {
    let dir = tempfile::tempdir().expect("temp store");
    let token = "0123456789abcdef0123456789abcdef";
    fs::write(dir.path().join(token), "alpha\nbeta\ngamma\ndelta\n").expect("write token");

    let output = tinyjuice()
        .args([
            "cat",
            "--store-dir",
            &dir.path().to_string_lossy(),
            "--lines",
            "1:3",
            token,
        ])
        .output()
        .expect("run tinyjuice cat lines");

    assert!(output.status.success(), "{output:#?}");
    assert_eq!(
        String::from_utf8(output.stdout).expect("utf8 stdout"),
        "beta\ngamma"
    );
}

#[test]
fn cat_reads_byte_range_from_explicit_store_dir() {
    let dir = tempfile::tempdir().expect("temp store");
    let token = "0123456789abcdef0123456789abcdef";
    fs::write(dir.path().join(token), "alpha beta gamma").expect("write token");

    let output = tinyjuice()
        .args([
            "cat",
            "--store-dir",
            &dir.path().to_string_lossy(),
            "--bytes",
            "6:10",
            token,
        ])
        .output()
        .expect("run tinyjuice cat bytes");

    assert!(output.status.success(), "{output:#?}");
    assert_eq!(
        String::from_utf8(output.stdout).expect("utf8 stdout"),
        "beta"
    );
}

#[test]
fn cat_rejects_multiple_range_flags() {
    let dir = tempfile::tempdir().expect("temp store");
    let token = "0123456789abcdef0123456789abcdef";
    fs::write(dir.path().join(token), "alpha\nbeta\n").expect("write token");

    let output = tinyjuice()
        .args([
            "cat",
            "--store-dir",
            &dir.path().to_string_lossy(),
            "--lines",
            "0:1",
            "--bytes",
            "0:5",
            token,
        ])
        .output()
        .expect("run tinyjuice cat conflicting ranges");

    assert!(!output.status.success(), "{output:#?}");
    assert!(
        String::from_utf8(output.stderr)
            .expect("utf8 stderr")
            .contains("cat accepts at most one range flag")
    );
}

#[test]
fn stats_reports_metadata_for_explicit_store_dir() {
    let dir = tempfile::tempdir().expect("temp store");
    fs::write(dir.path().join("0123456789abcdef0123456789abcdef"), "one").expect("write token");
    fs::write(dir.path().join("fedcba9876543210fedcba9876543210"), "three")
        .expect("write second token");
    fs::write(dir.path().join("not-a-token"), "ignored raw content").expect("write ignored");
    fs::create_dir(dir.path().join("subdir")).expect("create ignored dir");

    let output = tinyjuice()
        .args(["stats", "--store-dir", &dir.path().to_string_lossy()])
        .output()
        .expect("run tinyjuice stats");

    assert!(output.status.success(), "{output:#?}");
    let response: serde_json::Value = serde_json::from_slice(&output.stdout).expect("stats json");
    assert_eq!(response["tokens"], 2);
    assert_eq!(response["bytes"], 8);
    assert_eq!(response["ignoredEntries"], 2);
    assert_eq!(response["storeDir"], dir.path().to_string_lossy().as_ref());

    let stdout = String::from_utf8(output.stdout).expect("utf8 stdout");
    assert!(!stdout.contains("ignored raw content"), "{stdout}");
}

#[test]
fn doctor_reports_health_checks_without_fixtures_by_default() {
    let output = tinyjuice()
        .args(["doctor"])
        .output()
        .expect("run tinyjuice doctor");

    assert!(output.status.success(), "{output:#?}");
    let response: serde_json::Value = serde_json::from_slice(&output.stdout).expect("doctor json");
    assert!(
        matches!(response["status"].as_str(), Some("ok" | "warn")),
        "{response:#?}"
    );

    let checks = response["checks"].as_array().expect("checks array");
    let rules = checks
        .iter()
        .find(|check| check["name"] == "rules")
        .expect("rules check");
    assert!(matches!(rules["status"].as_str(), Some("ok" | "warn")));
    assert_eq!(rules["descriptors"], 101);

    let fixtures = checks
        .iter()
        .find(|check| check["name"] == "fixtures")
        .expect("fixtures check");
    assert_eq!(fixtures["status"], "disabled");
}

#[test]
fn doctor_reports_broken_store_dir() {
    let missing_dir = tempfile::tempdir()
        .expect("temp parent")
        .path()
        .join("missing");
    let missing_dir = missing_dir.to_string_lossy().into_owned();

    let output = tinyjuice()
        .args(["doctor", "--store-dir", &missing_dir])
        .output()
        .expect("run tinyjuice doctor");

    assert!(!output.status.success(), "{output:#?}");
    let response: serde_json::Value = serde_json::from_slice(&output.stdout).expect("doctor json");
    assert_eq!(response["status"], "broken");
    let checks = response["checks"].as_array().expect("checks array");
    let store = checks
        .iter()
        .find(|check| check["name"] == "ccrDiskStore")
        .expect("store check");
    assert_eq!(store["status"], "broken");
    assert_eq!(store["storeDir"], missing_dir);
}
