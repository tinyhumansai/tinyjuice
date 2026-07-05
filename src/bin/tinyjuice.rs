use std::env;
use std::fs;
use std::io::{self, IsTerminal, Read, Write};
use std::path::{Path, PathBuf};
use std::process::ExitCode;
use std::str::FromStr;

use serde_json::{Map, Value};
use tinyjuice::{
    SdkCompressOptions, TinyJuiceHost, compress_host_hook_payload, compress_request,
    host_install_specs, host_template, request_from_json_value,
};

const SUPPORT_GIT_TRAILER: &str = "Co-Authored-By: TinyJuice <tinyjuice@tinyhumans.ai>";
const CODEX_SUPPORT_ATTRIBUTION: &str = "TinyJuice <tinyjuice@tinyhumans.ai>";
const CLAUDE_DEFAULT_COMMIT_ATTRIBUTION: &str =
    "Co-Authored-By: Claude Code <noreply@anthropic.com>";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SupportGitSignatureChoice {
    Prompt,
    Yes,
    No,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct SupportGitSignatureResult {
    path: PathBuf,
    backup_path: Option<PathBuf>,
    changed: bool,
}

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("tinyjuice: {error}");
            ExitCode::from(1)
        }
    }
}

fn run() -> Result<(), String> {
    let mut args = env::args().skip(1).collect::<Vec<_>>();
    let Some(command) = args.first().cloned() else {
        print_usage();
        return Ok(());
    };
    args.remove(0);

    match command.as_str() {
        "reduce-json" => reduce_json(args),
        "codex-post-tool-use" => post_tool_use_hook(TinyJuiceHost::Codex),
        "claude-code-post-tool-use" => post_tool_use_hook(TinyJuiceHost::ClaudeCode),
        "install" => install_command(args),
        "update" => update_command(args),
        "uninstall" => uninstall_command(args),
        "retrieve" => retrieve_command(args),
        "hosts" => {
            println!(
                "{}",
                serde_json::to_string_pretty(&host_install_specs())
                    .map_err(|error| error.to_string())?
            );
            Ok(())
        }
        "host-template" => host_template_command(args),
        "help" | "--help" | "-h" => {
            print_usage();
            Ok(())
        }
        other => Err(format!("unknown command '{other}'")),
    }
}

fn reduce_json(args: Vec<String>) -> Result<(), String> {
    configure_cli_cache();
    let mut host_override = None;
    let mut path = None;
    let mut index = 0;
    while index < args.len() {
        match args[index].as_str() {
            "--host" => {
                let value = args
                    .get(index + 1)
                    .ok_or_else(|| "--host requires a value".to_string())?;
                host_override = Some(TinyJuiceHost::from_str(value)?);
                index += 2;
            }
            "--help" | "-h" => {
                print_usage();
                return Ok(());
            }
            value if value.starts_with("--host=") => {
                host_override = Some(TinyJuiceHost::from_str(&value["--host=".len()..])?);
                index += 1;
            }
            value if value.starts_with('-') && value != "-" => {
                return Err(format!("unknown reduce-json flag '{value}'"));
            }
            value => {
                if path.replace(value.to_string()).is_some() {
                    return Err("reduce-json accepts at most one path".to_string());
                }
                index += 1;
            }
        }
    }

    let input = read_input(path.as_deref())?;
    let value = serde_json::from_str(&input).map_err(|error| format!("invalid JSON: {error}"))?;
    let mut request = request_from_json_value(value).map_err(|error| error.to_string())?;
    if let Some(host) = host_override {
        request.host = host;
    }

    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .map_err(|error| error.to_string())?;
    let response = runtime.block_on(compress_request(request));
    println!(
        "{}",
        serde_json::to_string_pretty(&response).map_err(|error| error.to_string())?
    );
    Ok(())
}

fn post_tool_use_hook(host: TinyJuiceHost) -> Result<(), String> {
    configure_cli_cache();
    let input = read_input(None)?;
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .map_err(|error| error.to_string())?;
    let result = runtime.block_on(compress_host_hook_payload(
        host,
        &input,
        hook_options_from_env(),
    ));
    if let Ok(Some(value)) = result {
        println!(
            "{}",
            serde_json::to_string(&value).map_err(|error| error.to_string())?
        );
    }
    Ok(())
}

fn install_command(args: Vec<String>) -> Result<(), String> {
    install_or_update_command(args, "install", "installed")
}

fn update_command(args: Vec<String>) -> Result<(), String> {
    install_or_update_command(args, "update", "updated")
}

fn install_or_update_command(
    args: Vec<String>,
    command_name: &str,
    action: &str,
) -> Result<(), String> {
    let mut binary = "tinyjuice".to_string();
    let mut path = None;
    let mut host = None;
    let mut support_git_signature = SupportGitSignatureChoice::Prompt;
    let mut index = 0;
    while index < args.len() {
        match args[index].as_str() {
            "--binary" => {
                binary = args
                    .get(index + 1)
                    .ok_or_else(|| "--binary requires a value".to_string())?
                    .to_string();
                index += 2;
            }
            value if value.starts_with("--binary=") => {
                binary = value["--binary=".len()..].to_string();
                index += 1;
            }
            "--path" => {
                path = Some(PathBuf::from(
                    args.get(index + 1)
                        .ok_or_else(|| "--path requires a value".to_string())?,
                ));
                index += 2;
            }
            value if value.starts_with("--path=") => {
                path = Some(PathBuf::from(&value["--path=".len()..]));
                index += 1;
            }
            "--support-git-signature"
            | "--yes-git-signature"
            | "--support-git-attribution"
            | "--yes-git-attribution" => {
                support_git_signature = SupportGitSignatureChoice::Yes;
                index += 1;
            }
            "--no-support-git-signature"
            | "--no-git-signature"
            | "--no-support-git-attribution"
            | "--no-git-attribution" => {
                support_git_signature = SupportGitSignatureChoice::No;
                index += 1;
            }
            "--help" | "-h" => {
                print_usage();
                return Ok(());
            }
            value if value.starts_with('-') => {
                return Err(format!("unknown {command_name} flag '{value}'"));
            }
            value => {
                if host.replace(TinyJuiceHost::from_str(value)?).is_some() {
                    return Err(format!("{command_name} accepts exactly one host"));
                }
                index += 1;
            }
        }
    }
    let host = host.ok_or_else(|| format!("{command_name} requires a host"))?;
    ensure_hook_host_supported(command_name, host)?;
    let install_path = path.unwrap_or_else(|| default_hook_path(host));
    let backup_path = install_hook_config(host, &install_path, &binary)?;
    println!(
        "{action} {} hook at {}{}",
        host.id(),
        install_path.display(),
        backup_path
            .as_ref()
            .map(|path| format!(" (backup: {})", path.display()))
            .unwrap_or_default()
    );
    if should_install_support_git_signature(host, support_git_signature)? {
        let result = install_support_git_signature(host, &install_path)?;
        if result.changed {
            println!(
                "added TinyJuice support Git attribution at {}{}",
                result.path.display(),
                result
                    .backup_path
                    .as_ref()
                    .map(|path| format!(" (backup: {})", path.display()))
                    .unwrap_or_default()
            );
        } else {
            println!(
                "TinyJuice support Git attribution already present at {}",
                result.path.display()
            );
        }
    }
    Ok(())
}

fn uninstall_command(args: Vec<String>) -> Result<(), String> {
    let mut path = None;
    let mut host = None;
    let mut index = 0;
    while index < args.len() {
        match args[index].as_str() {
            "--path" => {
                path = Some(PathBuf::from(
                    args.get(index + 1)
                        .ok_or_else(|| "--path requires a value".to_string())?,
                ));
                index += 2;
            }
            value if value.starts_with("--path=") => {
                path = Some(PathBuf::from(&value["--path=".len()..]));
                index += 1;
            }
            "--help" | "-h" => {
                print_usage();
                return Ok(());
            }
            value if value.starts_with('-') => {
                return Err(format!("unknown uninstall flag '{value}'"));
            }
            value => {
                if host.replace(TinyJuiceHost::from_str(value)?).is_some() {
                    return Err("uninstall accepts exactly one host".to_string());
                }
                index += 1;
            }
        }
    }
    let host = host.ok_or_else(|| "uninstall requires a host".to_string())?;
    ensure_hook_host_supported("uninstall", host)?;
    let install_path = path.unwrap_or_else(|| default_hook_path(host));
    let (backup_path, removed) = uninstall_hook_config(host, &install_path)?;
    if removed {
        println!(
            "uninstalled {} hook from {}{}",
            host.id(),
            install_path.display(),
            backup_path
                .as_ref()
                .map(|path| format!(" (backup: {})", path.display()))
                .unwrap_or_default()
        );
    } else {
        println!("no {} hook found at {}", host.id(), install_path.display());
    }
    Ok(())
}

fn ensure_hook_host_supported(action: &str, host: TinyJuiceHost) -> Result<(), String> {
    if matches!(host, TinyJuiceHost::Codex | TinyJuiceHost::ClaudeCode) {
        Ok(())
    } else {
        Err(format!("{action} is not implemented for {}", host.id()))
    }
}

fn should_install_support_git_signature(
    host: TinyJuiceHost,
    choice: SupportGitSignatureChoice,
) -> Result<bool, String> {
    match choice {
        SupportGitSignatureChoice::Yes => Ok(true),
        SupportGitSignatureChoice::No => Ok(false),
        SupportGitSignatureChoice::Prompt => prompt_support_git_signature(host),
    }
}

fn prompt_support_git_signature(host: TinyJuiceHost) -> Result<bool, String> {
    if !io::stdin().is_terminal() {
        eprintln!(
            "skipped TinyJuice support Git attribution prompt for {} because stdin is not interactive; pass --support-git-signature to opt in",
            host.id()
        );
        return Ok(false);
    }
    eprint!(
        "Add TinyJuice support Git attribution for {} commits as {SUPPORT_GIT_TRAILER}? [Y/n] ",
        host.label()
    );
    io::stderr().flush().map_err(|error| error.to_string())?;
    let mut answer = String::new();
    io::stdin()
        .read_line(&mut answer)
        .map_err(|error| error.to_string())?;
    match answer.trim().to_ascii_lowercase().as_str() {
        "" | "y" | "yes" => Ok(true),
        "n" | "no" => Ok(false),
        _ => {
            eprintln!("unrecognized answer; leaving Git attribution unchanged");
            Ok(false)
        }
    }
}

fn install_support_git_signature(
    host: TinyJuiceHost,
    hook_path: &Path,
) -> Result<SupportGitSignatureResult, String> {
    match host {
        TinyJuiceHost::Codex => install_codex_support_git_signature(&default_codex_config_path()),
        TinyJuiceHost::ClaudeCode => install_claude_support_git_signature(hook_path),
        TinyJuiceHost::GenericJson | TinyJuiceHost::OpenHuman | TinyJuiceHost::RustHarness => Err(
            format!("Git attribution is not implemented for {}", host.id()),
        ),
    }
}

fn default_codex_config_path() -> PathBuf {
    env::var("CODEX_HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|_| home_dir().join(".codex"))
        .join("config.toml")
}

fn install_codex_support_git_signature(path: &Path) -> Result<SupportGitSignatureResult, String> {
    let existing = match fs::read_to_string(path) {
        Ok(existing) => Some(existing),
        Err(error) if error.kind() == io::ErrorKind::NotFound => None,
        Err(error) => return Err(error.to_string()),
    };
    let content = existing.as_deref().unwrap_or_default();
    let with_attribution =
        set_toml_top_level_string(content, "commit_attribution", CODEX_SUPPORT_ATTRIBUTION);
    let updated = set_toml_table_bool(&with_attribution, "features", "codex_git_commit", true);
    if updated == content {
        return Ok(SupportGitSignatureResult {
            path: path.to_path_buf(),
            backup_path: None,
            changed: false,
        });
    }
    write_text_with_backup(path, existing.as_deref(), &updated)?;
    Ok(SupportGitSignatureResult {
        path: path.to_path_buf(),
        backup_path: existing.as_ref().map(|_| path.with_extension("bak")),
        changed: true,
    })
}

fn install_claude_support_git_signature(path: &Path) -> Result<SupportGitSignatureResult, String> {
    let existing = match fs::read_to_string(path) {
        Ok(existing) => Some(existing),
        Err(error) if error.kind() == io::ErrorKind::NotFound => None,
        Err(error) => return Err(error.to_string()),
    };
    let mut config = existing
        .as_deref()
        .and_then(|content| serde_json::from_str(content).ok())
        .unwrap_or_else(|| Value::Object(Map::new()));
    let attribution = ensure_object_field(&mut config, "attribution")?;
    let current = attribution
        .get("commit")
        .and_then(Value::as_str)
        .unwrap_or(CLAUDE_DEFAULT_COMMIT_ATTRIBUTION);
    if current.contains(SUPPORT_GIT_TRAILER) {
        return Ok(SupportGitSignatureResult {
            path: path.to_path_buf(),
            backup_path: None,
            changed: false,
        });
    }
    let updated_commit = if current.trim().is_empty() {
        SUPPORT_GIT_TRAILER.to_string()
    } else {
        format!("{}\n{}", current.trim_end(), SUPPORT_GIT_TRAILER)
    };
    attribution.insert("commit".to_string(), Value::String(updated_commit));
    let updated = format!(
        "{}\n",
        serde_json::to_string_pretty(&config).map_err(|error| error.to_string())?
    );
    write_text_with_backup(path, existing.as_deref(), &updated)?;
    Ok(SupportGitSignatureResult {
        path: path.to_path_buf(),
        backup_path: existing.as_ref().map(|_| path.with_extension("bak")),
        changed: true,
    })
}

fn write_text_with_backup(
    path: &Path,
    existing: Option<&str>,
    updated: &str,
) -> Result<(), String> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|error| error.to_string())?;
    }
    if let Some(existing) = existing {
        fs::write(path.with_extension("bak"), existing).map_err(|error| error.to_string())?;
    }
    let temp_path = path.with_extension("tmp");
    fs::write(&temp_path, updated).map_err(|error| error.to_string())?;
    fs::rename(&temp_path, path).map_err(|error| error.to_string())
}

fn set_toml_top_level_string(input: &str, key: &str, value: &str) -> String {
    let mut lines = input.lines().map(str::to_string).collect::<Vec<_>>();
    let table_start = lines
        .iter()
        .position(|line| line.trim_start().starts_with('['))
        .unwrap_or(lines.len());
    let rendered = format!(r#"{key} = "{value}""#);
    for line in lines.iter_mut().take(table_start) {
        if toml_line_has_key(line, key) {
            *line = rendered;
            return finish_toml_lines(lines);
        }
    }
    lines.insert(table_start, rendered);
    finish_toml_lines(lines)
}

fn set_toml_table_bool(input: &str, table: &str, key: &str, value: bool) -> String {
    let mut lines = input.lines().map(str::to_string).collect::<Vec<_>>();
    let rendered = format!("{key} = {value}");
    let mut in_table = false;
    let mut table_found = false;
    let mut insert_at = None;
    for (index, line) in lines.iter_mut().enumerate() {
        if let Some(section) = toml_section_name(line) {
            if in_table {
                insert_at = Some(index);
                break;
            }
            in_table = section == table;
            table_found |= in_table;
            continue;
        }
        if in_table && toml_line_has_key(line, key) {
            *line = rendered;
            return finish_toml_lines(lines);
        }
    }
    if table_found {
        lines.insert(insert_at.unwrap_or(lines.len()), rendered);
    } else {
        if !lines.is_empty() && !lines.last().is_some_and(|line| line.trim().is_empty()) {
            lines.push(String::new());
        }
        lines.push(format!("[{table}]"));
        lines.push(rendered);
    }
    finish_toml_lines(lines)
}

fn toml_line_has_key(line: &str, key: &str) -> bool {
    let trimmed = line.trim_start();
    trimmed
        .strip_prefix(key)
        .is_some_and(|rest| rest.trim_start().starts_with('='))
}

fn toml_section_name(line: &str) -> Option<&str> {
    let trimmed = line.trim();
    trimmed.strip_prefix('[')?.strip_suffix(']')
}

fn finish_toml_lines(lines: Vec<String>) -> String {
    if lines.is_empty() {
        String::new()
    } else {
        format!("{}\n", lines.join("\n"))
    }
}

fn retrieve_command(args: Vec<String>) -> Result<(), String> {
    configure_cli_cache();
    let token = args
        .first()
        .ok_or_else(|| "retrieve requires a CCR token".to_string())?;
    if args.len() > 1 {
        return Err("retrieve accepts exactly one CCR token".to_string());
    }
    let Some(original) = tinyjuice::cache::retrieve(token) else {
        return Err(format!("no original found for token {token}"));
    };
    print!("{original}");
    Ok(())
}

fn host_template_command(args: Vec<String>) -> Result<(), String> {
    let mut binary = "tinyjuice".to_string();
    let mut host = None;
    let mut index = 0;
    while index < args.len() {
        match args[index].as_str() {
            "--binary" => {
                binary = args
                    .get(index + 1)
                    .ok_or_else(|| "--binary requires a value".to_string())?
                    .to_string();
                index += 2;
            }
            value if value.starts_with("--binary=") => {
                binary = value["--binary=".len()..].to_string();
                index += 1;
            }
            "--help" | "-h" => {
                print_usage();
                return Ok(());
            }
            value if value.starts_with('-') => {
                return Err(format!("unknown host-template flag '{value}'"));
            }
            value => {
                if host.replace(TinyJuiceHost::from_str(value)?).is_some() {
                    return Err("host-template accepts exactly one host".to_string());
                }
                index += 1;
            }
        }
    }
    let host = host.ok_or_else(|| "host-template requires a host".to_string())?;
    println!("{}", host_template(host, &binary));
    Ok(())
}

fn read_input(path: Option<&str>) -> Result<String, String> {
    match path {
        Some("-") | None => {
            let mut input = String::new();
            io::stdin()
                .read_to_string(&mut input)
                .map_err(|error| error.to_string())?;
            Ok(input)
        }
        Some(path) => fs::read_to_string(path).map_err(|error| error.to_string()),
    }
}

fn configure_cli_cache() {
    tinyjuice::cache::enable_disk_tier(cli_cache_root());
}

fn cli_cache_root() -> PathBuf {
    if let Ok(path) = env::var("TINYJUICE_CCR_DIR")
        && !path.trim().is_empty()
    {
        return PathBuf::from(path);
    }
    dirs::cache_dir()
        .unwrap_or_else(env::temp_dir)
        .join("tinyjuice")
        .join("ccr")
}

fn hook_options_from_env() -> SdkCompressOptions {
    SdkCompressOptions {
        min_bytes_to_compress: read_usize_env("TINYJUICE_MIN_BYTES_TO_COMPRESS"),
        max_inline_chars: read_usize_env("TINYJUICE_MAX_INLINE_CHARS"),
        ccr_min_tokens: read_usize_env("TINYJUICE_CCR_MIN_TOKENS"),
        ccr_enabled: read_bool_env("TINYJUICE_CCR_ENABLED"),
        lossy_without_ccr: read_bool_env("TINYJUICE_LOSSY_WITHOUT_CCR"),
        ..Default::default()
    }
}

fn read_usize_env(name: &str) -> Option<usize> {
    env::var(name)
        .ok()
        .and_then(|value| value.trim().parse::<usize>().ok())
}

fn read_bool_env(name: &str) -> Option<bool> {
    env::var(name)
        .ok()
        .and_then(|value| match value.trim().to_ascii_lowercase().as_str() {
            "1" | "true" | "yes" | "on" => Some(true),
            "0" | "false" | "no" | "off" => Some(false),
            _ => None,
        })
}

fn default_hook_path(host: TinyJuiceHost) -> PathBuf {
    match host {
        TinyJuiceHost::Codex => env::var("CODEX_HOME")
            .map(PathBuf::from)
            .unwrap_or_else(|_| home_dir().join(".codex"))
            .join("hooks.json"),
        TinyJuiceHost::ClaudeCode => env::var("CLAUDE_CONFIG_DIR")
            .or_else(|_| env::var("CLAUDE_HOME"))
            .map(PathBuf::from)
            .unwrap_or_else(|_| home_dir().join(".claude"))
            .join("settings.json"),
        TinyJuiceHost::GenericJson | TinyJuiceHost::OpenHuman | TinyJuiceHost::RustHarness => {
            home_dir()
        }
    }
}

fn home_dir() -> PathBuf {
    dirs::home_dir().unwrap_or_else(env::temp_dir)
}

fn install_hook_config(
    host: TinyJuiceHost,
    path: &Path,
    binary: &str,
) -> Result<Option<PathBuf>, String> {
    let template: Value =
        serde_json::from_str(&host_template(host, binary)).map_err(|error| error.to_string())?;
    let (mut config, backup_path) = load_or_empty_json(path)?;
    merge_template_hooks(&mut config, &template, host)?;
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|error| error.to_string())?;
    }
    write_json_config(path, &config)?;
    Ok(backup_path)
}

fn uninstall_hook_config(
    host: TinyJuiceHost,
    path: &Path,
) -> Result<(Option<PathBuf>, bool), String> {
    let existing = match fs::read_to_string(path) {
        Ok(existing) => existing,
        Err(error) if error.kind() == io::ErrorKind::NotFound => return Ok((None, false)),
        Err(error) => return Err(error.to_string()),
    };
    let mut config = serde_json::from_str(&existing).unwrap_or_else(|_| Value::Object(Map::new()));
    if !remove_tinyjuice_hooks(&mut config, host) {
        return Ok((None, false));
    }
    let backup_path = path.with_extension("bak");
    fs::write(&backup_path, existing).map_err(|error| error.to_string())?;
    write_json_config(path, &config)?;
    Ok((Some(backup_path), true))
}

fn write_json_config(path: &Path, config: &Value) -> Result<(), String> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|error| error.to_string())?;
    }
    let temp_path = path.with_extension("tmp");
    fs::write(
        &temp_path,
        format!(
            "{}\n",
            serde_json::to_string_pretty(&config).map_err(|error| error.to_string())?
        ),
    )
    .map_err(|error| error.to_string())?;
    fs::rename(&temp_path, path).map_err(|error| error.to_string())?;
    Ok(())
}

fn load_or_empty_json(path: &Path) -> Result<(Value, Option<PathBuf>), String> {
    match fs::read_to_string(path) {
        Ok(existing) => {
            let parsed =
                serde_json::from_str(&existing).unwrap_or_else(|_| Value::Object(Map::new()));
            let backup_path = path.with_extension("bak");
            fs::write(&backup_path, existing).map_err(|error| error.to_string())?;
            Ok((parsed, Some(backup_path)))
        }
        Err(error) if error.kind() == io::ErrorKind::NotFound => {
            Ok((Value::Object(Map::new()), None))
        }
        Err(error) => Err(error.to_string()),
    }
}

fn merge_template_hooks(
    config: &mut Value,
    template: &Value,
    host: TinyJuiceHost,
) -> Result<(), String> {
    let hooks = ensure_object_field(config, "hooks")?;
    let template_hooks = template
        .get("hooks")
        .and_then(Value::as_object)
        .ok_or_else(|| "host template did not contain hooks object".to_string())?;
    for (event, groups) in template_hooks {
        let event_groups = ensure_array_field(hooks, event);
        prune_tinyjuice_hook_groups(event_groups, host);
        let template_groups = groups
            .as_array()
            .ok_or_else(|| format!("template hooks.{event} is not an array"))?;
        event_groups.extend(template_groups.iter().cloned());
    }
    Ok(())
}

fn ensure_object_field<'a>(
    value: &'a mut Value,
    key: &str,
) -> Result<&'a mut Map<String, Value>, String> {
    if !value.is_object() {
        *value = Value::Object(Map::new());
    }
    let object = value
        .as_object_mut()
        .ok_or_else(|| "expected JSON object".to_string())?;
    if !object.get(key).is_some_and(Value::is_object) {
        object.insert(key.to_string(), Value::Object(Map::new()));
    }
    object
        .get_mut(key)
        .and_then(Value::as_object_mut)
        .ok_or_else(|| format!("expected {key} object"))
}

fn ensure_array_field<'a>(object: &'a mut Map<String, Value>, key: &str) -> &'a mut Vec<Value> {
    if !object.get(key).is_some_and(Value::is_array) {
        object.insert(key.to_string(), Value::Array(Vec::new()));
    }
    object
        .get_mut(key)
        .and_then(Value::as_array_mut)
        .expect("array field inserted")
}

fn prune_tinyjuice_hook_groups(groups: &mut Vec<Value>, host: TinyJuiceHost) {
    groups.retain_mut(|group| {
        let Some(hooks) = group.get_mut("hooks").and_then(Value::as_array_mut) else {
            return true;
        };
        hooks.retain(|hook| !is_tinyjuice_hook(hook, host));
        !hooks.is_empty()
    });
}

fn remove_tinyjuice_hooks(config: &mut Value, host: TinyJuiceHost) -> bool {
    let Some(hooks) = config.get_mut("hooks").and_then(Value::as_object_mut) else {
        return false;
    };
    let mut removed = false;
    let mut empty_events = Vec::new();
    for (event, value) in hooks.iter_mut() {
        let Some(groups) = value.as_array_mut() else {
            continue;
        };
        groups.retain_mut(|group| {
            let Some(group_hooks) = group.get_mut("hooks").and_then(Value::as_array_mut) else {
                return true;
            };
            let before = group_hooks.len();
            group_hooks.retain(|hook| !is_tinyjuice_hook(hook, host));
            if group_hooks.len() != before {
                removed = true;
            }
            !group_hooks.is_empty()
        });
        if groups.is_empty() {
            empty_events.push(event.clone());
        }
    }
    for event in empty_events {
        hooks.remove(&event);
    }
    removed
}

fn is_tinyjuice_hook(hook: &Value, host: TinyJuiceHost) -> bool {
    let status = hook
        .get("statusMessage")
        .and_then(Value::as_str)
        .unwrap_or_default()
        .to_ascii_lowercase();
    let command = hook
        .get("command")
        .and_then(Value::as_str)
        .unwrap_or_default()
        .to_ascii_lowercase();
    let subcommand = match host {
        TinyJuiceHost::Codex => "codex-post-tool-use",
        TinyJuiceHost::ClaudeCode => "claude-code-post-tool-use",
        TinyJuiceHost::GenericJson | TinyJuiceHost::OpenHuman | TinyJuiceHost::RustHarness => "",
    };
    status.contains("tinyjuice") || (!subcommand.is_empty() && command.contains(subcommand))
}

fn print_usage() {
    eprintln!(
        r#"Usage:
  tinyjuice reduce-json [--host HOST] [payload.json|-]
  tinyjuice codex-post-tool-use
  tinyjuice claude-code-post-tool-use
  tinyjuice install HOST [--path PATH] [--binary PATH] [--support-git-signature|--no-support-git-signature]
  tinyjuice update HOST [--path PATH] [--binary PATH] [--support-git-signature|--no-support-git-signature]
  tinyjuice uninstall HOST [--path PATH]
  tinyjuice retrieve TOKEN
  tinyjuice hosts
  tinyjuice host-template HOST [--binary PATH]

Hosts: generic-json, openhuman, codex, claude-code, rust-harness"#
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn install_merges_without_removing_existing_hooks() {
        let dir = tempfile::tempdir().expect("tempdir");
        let path = dir.path().join("hooks.json");
        fs::write(
            &path,
            r#"{"hooks":{"PostToolUse":[{"matcher":"Edit","hooks":[{"type":"command","command":"echo keep"}]}]}}"#,
        )
        .expect("write");

        let backup =
            install_hook_config(TinyJuiceHost::Codex, &path, "tinyjuice").expect("install");
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
        install_hook_config(TinyJuiceHost::Codex, &path, "/usr/local/bin/tinyjuice")
            .expect("update");

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
        let (backup, removed) =
            uninstall_hook_config(TinyJuiceHost::Codex, &path).expect("uninstall");

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

        let (backup, removed) =
            uninstall_hook_config(TinyJuiceHost::Codex, &path).expect("uninstall");

        assert!(!removed);
        assert!(backup.is_none());
        assert_eq!(fs::read_to_string(path).expect("read"), original);
    }

    #[test]
    fn uninstall_missing_file_is_noop() {
        let dir = tempfile::tempdir().expect("tempdir");
        let path = dir.path().join("hooks.json");

        let (backup, removed) =
            uninstall_hook_config(TinyJuiceHost::Codex, &path).expect("uninstall");

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
}
