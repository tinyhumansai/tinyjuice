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
