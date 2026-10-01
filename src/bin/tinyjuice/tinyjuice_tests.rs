use super::*;

#[test]
fn cli_cache_defaults_set_finite_ttl() {
    let (max_entries, max_bytes, ttl) = cli_cache_limits(None, None, None);
    assert_eq!(max_entries, tinyjuice::cache::DEFAULT_MAX_ENTRIES);
    assert_eq!(max_bytes, tinyjuice::cache::DEFAULT_MAX_BYTES);
    assert_eq!(
        ttl,
        Some(DEFAULT_CLI_CCR_TTL_SECS),
        "hook processes are fresh per call — a finite TTL is required or disk entries never expire"
    );
}

#[test]
fn cli_cache_limits_honor_overrides() {
    let (max_entries, max_bytes, ttl) = cli_cache_limits(Some(10), Some(1024), Some(60));
    assert_eq!((max_entries, max_bytes, ttl), (10, 1024, Some(60)));
    // TTL override of 0 means "no expiry".
    let (_, _, ttl) = cli_cache_limits(None, None, Some(0));
    assert_eq!(ttl, None);
}

#[test]
fn install_merges_without_removing_existing_hooks() {
    let dir = tempfile::tempdir().expect("tempdir");
    let path = dir.path().join("hooks.json");
    fs::write(
        &path,
        r#"{"hooks":{"PostToolUse":[{"matcher":"Edit","hooks":[{"type":"command","command":"echo keep"}]}]}}"#,
    )
    .expect("write");

    let backup = install_hook_config(TinyJuiceHost::Codex, &path, "tinyjuice").expect("install");
    assert!(backup.expect("backup").exists());

    let installed: Value =
        serde_json::from_str(&fs::read_to_string(path).expect("read")).expect("json");
    let groups = installed["hooks"]["PostToolUse"]
        .as_array()
        .expect("groups");
    assert_eq!(groups.len(), 2);
    assert!(
        groups
            .iter()
            .any(|group| group["hooks"][0]["command"].as_str() == Some("echo keep"))
    );
    assert!(groups.iter().any(|group| {
        group["hooks"][0]["command"]
            .as_str()
            .is_some_and(|command| command.contains("codex-post-tool-use"))
    }));
}

#[test]
fn reinstall_replaces_tinyjuice_hook() {
    let dir = tempfile::tempdir().expect("tempdir");
    let path = dir.path().join("settings.json");

    install_hook_config(TinyJuiceHost::ClaudeCode, &path, "tinyjuice").expect("install");
    install_hook_config(TinyJuiceHost::ClaudeCode, &path, "/opt/tinyjuice").expect("reinstall");

    let installed: Value =
        serde_json::from_str(&fs::read_to_string(path).expect("read")).expect("json");
    let groups = installed["hooks"]["PostToolUse"]
        .as_array()
        .expect("groups");
    assert_eq!(groups.len(), 1);
    assert_eq!(
        groups[0]["hooks"][0]["command"].as_str(),
        Some("/opt/tinyjuice claude-code-post-tool-use")
    );
}

#[test]
fn update_replaces_existing_tinyjuice_hook_binary() {
    let dir = tempfile::tempdir().expect("tempdir");
    let path = dir.path().join("hooks.json");

    install_hook_config(TinyJuiceHost::Codex, &path, "tinyjuice").expect("install");
    install_hook_config(TinyJuiceHost::Codex, &path, "/usr/local/bin/tinyjuice").expect("update");

    let installed: Value =
        serde_json::from_str(&fs::read_to_string(path).expect("read")).expect("json");
    let groups = installed["hooks"]["PostToolUse"]
        .as_array()
        .expect("groups");
    assert_eq!(groups.len(), 1);
    assert_eq!(
        groups[0]["hooks"][0]["command"].as_str(),
        Some("/usr/local/bin/tinyjuice codex-post-tool-use")
    );
}

#[test]
fn uninstall_removes_tinyjuice_hook_but_preserves_other_hooks() {
    let dir = tempfile::tempdir().expect("tempdir");
    let path = dir.path().join("hooks.json");
    fs::write(
        &path,
        r#"{"hooks":{"PostToolUse":[{"matcher":"Edit","hooks":[{"type":"command","command":"echo keep"}]}]}}"#,
    )
    .expect("write");

    install_hook_config(TinyJuiceHost::Codex, &path, "tinyjuice").expect("install");
    let (backup, removed) = uninstall_hook_config(TinyJuiceHost::Codex, &path).expect("uninstall");

    assert!(removed);
    assert!(backup.expect("backup").exists());
    let installed: Value =
        serde_json::from_str(&fs::read_to_string(path).expect("read")).expect("json");
    let groups = installed["hooks"]["PostToolUse"]
        .as_array()
        .expect("groups");
    assert_eq!(groups.len(), 1);
    assert_eq!(groups[0]["hooks"][0]["command"].as_str(), Some("echo keep"));
}

#[test]
fn uninstall_without_tinyjuice_hook_is_noop() {
    let dir = tempfile::tempdir().expect("tempdir");
    let path = dir.path().join("hooks.json");
    let original = r#"{"hooks":{"PostToolUse":[{"matcher":"Edit","hooks":[{"type":"command","command":"echo keep"}]}]}}"#;
    fs::write(&path, original).expect("write");

    let (backup, removed) = uninstall_hook_config(TinyJuiceHost::Codex, &path).expect("uninstall");

    assert!(!removed);
    assert!(backup.is_none());
    assert_eq!(fs::read_to_string(path).expect("read"), original);
}

#[test]
fn uninstall_missing_file_is_noop() {
    let dir = tempfile::tempdir().expect("tempdir");
    let path = dir.path().join("hooks.json");

    let (backup, removed) = uninstall_hook_config(TinyJuiceHost::Codex, &path).expect("uninstall");

    assert!(!removed);
    assert!(backup.is_none());
    assert!(!path.exists());
}

#[test]
fn toml_top_level_string_is_inserted_before_first_table() {
    let updated = set_toml_top_level_string(
        "[features]\nfoo = true\n",
        "commit_attribution",
        CODEX_SUPPORT_ATTRIBUTION,
    );

    assert_eq!(
        updated,
        "commit_attribution = \"TinyJuice <tinyjuice@tinyhumans.ai>\"\n[features]\nfoo = true\n"
    );
}

#[test]
fn toml_table_bool_is_created_or_replaced() {
    let created = set_toml_table_bool(
        "commit_attribution = \"TinyJuice <tinyjuice@tinyhumans.ai>\"\n",
        "features",
        "codex_git_commit",
        true,
    );
    assert_eq!(
        created,
        "commit_attribution = \"TinyJuice <tinyjuice@tinyhumans.ai>\"\n\n[features]\ncodex_git_commit = true\n"
    );

    let replaced = set_toml_table_bool(
        "[features]\ncodex_git_commit = false\nother = true\n",
        "features",
        "codex_git_commit",
        true,
    );
    assert_eq!(
        replaced,
        "[features]\ncodex_git_commit = true\nother = true\n"
    );
}

#[test]
fn codex_support_git_signature_writes_config_and_backup() {
    let dir = tempfile::tempdir().expect("tempdir");
    let path = dir.path().join("config.toml");
    fs::write(&path, "[features]\nother = true\n").expect("write");

    let result = install_codex_support_git_signature(&path).expect("install attribution");

    assert!(result.changed);
    assert!(result.backup_path.expect("backup").exists());
    let updated = fs::read_to_string(path).expect("read");
    assert!(updated.contains("commit_attribution = \"TinyJuice <tinyjuice@tinyhumans.ai>\""));
    assert!(updated.contains("[features]\n"));
    assert!(updated.contains("other = true\n"));
    assert!(updated.contains("codex_git_commit = true\n"));
}

#[test]
fn codex_support_git_signature_is_noop_when_present() {
    let dir = tempfile::tempdir().expect("tempdir");
    let path = dir.path().join("config.toml");
    fs::write(
        &path,
        "commit_attribution = \"TinyJuice <tinyjuice@tinyhumans.ai>\"\n\n[features]\ncodex_git_commit = true\n",
    )
    .expect("write");

    let result = install_codex_support_git_signature(&path).expect("install attribution");

    assert!(!result.changed);
    assert!(result.backup_path.is_none());
    assert!(!path.with_extension("bak").exists());
}

#[test]
fn claude_support_git_signature_appends_to_existing_attribution() {
    let dir = tempfile::tempdir().expect("tempdir");
    let path = dir.path().join("settings.json");
    fs::write(
        &path,
        r#"{"attribution":{"commit":"Co-Authored-By: Claude Sonnet 5 <noreply@anthropic.com>"},"hooks":{}}"#,
    )
    .expect("write");

    let result = install_claude_support_git_signature(&path).expect("install attribution");

    assert!(result.changed);
    assert!(result.backup_path.expect("backup").exists());
    let updated: Value =
        serde_json::from_str(&fs::read_to_string(path).expect("read")).expect("json");
    assert_eq!(
        updated["attribution"]["commit"].as_str(),
        Some(
            "Co-Authored-By: Claude Sonnet 5 <noreply@anthropic.com>\nCo-Authored-By: TinyJuice <tinyjuice@tinyhumans.ai>"
        )
    );
    assert!(updated.get("hooks").is_some());
}

#[test]
fn claude_support_git_signature_is_noop_when_present() {
    let dir = tempfile::tempdir().expect("tempdir");
    let path = dir.path().join("settings.json");
    fs::write(
        &path,
        r#"{"attribution":{"commit":"Co-Authored-By: TinyJuice <tinyjuice@tinyhumans.ai>"}}"#,
    )
    .expect("write");

    let result = install_claude_support_git_signature(&path).expect("install attribution");

    assert!(!result.changed);
    assert!(result.backup_path.is_none());
    assert!(!path.with_extension("bak").exists());
}
