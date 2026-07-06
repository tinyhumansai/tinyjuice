use std::fs;
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::process::ExitCode;

use tinyjuice::cache::RangeUnit;
use tinyjuice::{
    LoadRuleOptions, ReduceOptions, ToolExecutionInput, discover_fallback_outputs,
    load_builtin_rules, reduce_execution_with_rules, reduce_json_str, verify_rule_fixtures,
    verify_rules,
};

const TINYJUICE_MANAGED_BEGIN: &str = "<!-- tinyjuice-managed:start -->";
const TINYJUICE_MANAGED_END: &str = "<!-- tinyjuice-managed:end -->";

fn main() -> ExitCode {
    match run(std::env::args().skip(1).collect()) {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            let _ = writeln!(io::stderr(), "{error}");
            ExitCode::from(2)
        }
    }
}

fn run(args: Vec<String>) -> Result<(), String> {
    let Some(command) = args.first().map(String::as_str) else {
        print_usage();
        return Err("missing command".to_owned());
    };

    match command {
        "reduce" => run_reduce(&args[1..]),
        "reduce-json" => run_reduce_json(&args[1..]),
        "verify" => run_verify(&args[1..]),
        "discover" => run_discover(&args[1..]),
        "wrap" => run_wrap(&args[1..]),
        "ls" => run_ls(&args[1..]),
        "cat" => run_cat(&args[1..]),
        "stats" => run_stats(&args[1..]),
        "doctor" => run_doctor(&args[1..]),
        "install" => run_install(&args[1..]),
        "uninstall" => run_uninstall(&args[1..]),
        "-h" | "--help" | "help" => {
            print_usage();
            Ok(())
        }
        other => {
            print_usage();
            Err(format!("unknown command: {other}"))
        }
    }
}

fn run_ls(args: &[String]) -> Result<(), String> {
    if args.iter().any(|arg| arg == "-h" || arg == "--help") {
        print_ls_usage();
        return Ok(());
    }
    let store_dir = parse_store_dir_args("ls", args)?;
    let inventory = read_ccr_store_inventory(&store_dir)?;
    for entry in inventory.entries {
        println!("{}", entry.token);
    }
    Ok(())
}

fn run_stats(args: &[String]) -> Result<(), String> {
    let mut store_dir = None;
    let mut pretty = false;
    let mut i = 0;

    while i < args.len() {
        match args[i].as_str() {
            "--store-dir" => {
                i += 1;
                store_dir = Some(PathBuf::from(
                    args.get(i)
                        .ok_or_else(|| "--store-dir requires a value".to_owned())?,
                ));
            }
            "--pretty" => pretty = true,
            "-h" | "--help" => {
                print_stats_usage();
                return Ok(());
            }
            value => return Err(format!("unknown stats option: {value}")),
        }
        i += 1;
    }

    let store_dir = store_dir.ok_or_else(|| "stats requires --store-dir".to_owned())?;
    let inventory = read_ccr_store_inventory(&store_dir)?;
    let bytes = inventory
        .entries
        .iter()
        .map(|entry| entry.bytes)
        .sum::<u64>();
    let stats = serde_json::json!({
        "storeDir": store_dir.to_string_lossy(),
        "tokens": inventory.entries.len(),
        "bytes": bytes,
        "ignoredEntries": inventory.ignored_entries,
    });
    if pretty {
        println!(
            "{}",
            serde_json::to_string_pretty(&stats)
                .map_err(|error| format!("failed to serialize stats: {error}"))?
        );
    } else {
        println!(
            "{}",
            serde_json::to_string(&stats)
                .map_err(|error| format!("failed to serialize stats: {error}"))?
        );
    }
    Ok(())
}

fn run_doctor(args: &[String]) -> Result<(), String> {
    let mut target = None;
    let mut store_dir = None;
    let mut fixtures_dir = PathBuf::from("tests/fixtures");
    let mut codex_home = None;
    let mut openhuman_root = None;
    let mut check_fixtures = false;
    let mut pretty = false;
    let mut i = 0;

    while i < args.len() {
        match args[i].as_str() {
            "--store-dir" => {
                i += 1;
                store_dir = Some(PathBuf::from(
                    args.get(i)
                        .ok_or_else(|| "--store-dir requires a value".to_owned())?,
                ));
            }
            "--codex-home" => {
                i += 1;
                codex_home = Some(PathBuf::from(
                    args.get(i)
                        .ok_or_else(|| "--codex-home requires a value".to_owned())?,
                ));
            }
            "--openhuman-root" => {
                i += 1;
                openhuman_root =
                    Some(PathBuf::from(args.get(i).ok_or_else(|| {
                        "--openhuman-root requires a value".to_owned()
                    })?));
            }
            "--fixtures" => {
                check_fixtures = true;
                if let Some(next) = args.get(i + 1)
                    && !next.starts_with('-')
                {
                    i += 1;
                    fixtures_dir = PathBuf::from(next);
                }
            }
            "--no-fixtures" => check_fixtures = false,
            "--pretty" => pretty = true,
            "-h" | "--help" => {
                print_doctor_usage();
                return Ok(());
            }
            value if value.starts_with('-') => {
                return Err(format!("unknown doctor option: {value}"));
            }
            value => {
                if target.replace(value.to_owned()).is_some() {
                    return Err("doctor accepts at most one host target".to_owned());
                }
            }
        }
        i += 1;
    }

    if let Some(target) = target {
        return run_host_doctor(&target, pretty, codex_home, openhuman_root);
    }

    let mut status = "disabled";
    let mut checks = Vec::new();

    let rules_report = verify_rules(&LoadRuleOptions {
        exclude_user: true,
        exclude_project: true,
        ..Default::default()
    });
    let rules_status = if !rules_report.parse_errors.is_empty()
        || !rules_report.duplicate_ids.is_empty()
    {
        "broken"
    } else if !rules_report.invalid_regexes.is_empty() || !rules_report.shadowed_rules.is_empty() {
        "warn"
    } else {
        "ok"
    };
    status = merge_doctor_status(status, rules_status);
    checks.push(serde_json::json!({
        "name": "rules",
        "status": rules_status,
        "descriptors": rules_report.descriptors_seen,
        "valid": rules_report.valid_rules,
        "final": rules_report.final_rules,
        "parseErrors": rules_report.parse_errors.len(),
        "invalidRegexes": rules_report.invalid_regexes.len(),
        "duplicateIds": rules_report.duplicate_ids.len(),
        "shadowed": rules_report.shadowed_rules.len(),
    }));

    if check_fixtures {
        let rules = load_builtin_rules();
        let fixture_report = verify_rule_fixtures(&fixtures_dir, &rules);
        let fixture_status = if fixture_report.is_clean() {
            "ok"
        } else {
            "broken"
        };
        status = merge_doctor_status(status, fixture_status);
        checks.push(serde_json::json!({
            "name": "fixtures",
            "status": fixture_status,
            "dir": fixtures_dir.to_string_lossy(),
            "seen": fixture_report.fixtures_seen,
            "passed": fixture_report.passed,
            "parseErrors": fixture_report.parse_errors.len(),
            "failures": fixture_report.failures.len(),
        }));
    } else {
        checks.push(serde_json::json!({
            "name": "fixtures",
            "status": "disabled",
        }));
    }

    match store_dir {
        Some(store_dir) => match read_ccr_store_inventory(&store_dir) {
            Ok(inventory) => {
                let bytes = inventory
                    .entries
                    .iter()
                    .map(|entry| entry.bytes)
                    .sum::<u64>();
                status = merge_doctor_status(status, "ok");
                checks.push(serde_json::json!({
                    "name": "ccrDiskStore",
                    "status": "ok",
                    "storeDir": store_dir.to_string_lossy(),
                    "tokens": inventory.entries.len(),
                    "bytes": bytes,
                    "ignoredEntries": inventory.ignored_entries,
                }));
            }
            Err(error) => {
                status = merge_doctor_status(status, "broken");
                checks.push(serde_json::json!({
                    "name": "ccrDiskStore",
                    "status": "broken",
                    "storeDir": store_dir.to_string_lossy(),
                    "error": error,
                }));
            }
        },
        None => checks.push(serde_json::json!({
            "name": "ccrDiskStore",
            "status": "disabled",
        })),
    }

    let report = serde_json::json!({
        "status": status,
        "checks": checks,
    });
    if pretty {
        println!(
            "{}",
            serde_json::to_string_pretty(&report)
                .map_err(|error| format!("failed to serialize doctor report: {error}"))?
        );
    } else {
        println!(
            "{}",
            serde_json::to_string(&report)
                .map_err(|error| format!("failed to serialize doctor report: {error}"))?
        );
    }

    if status == "broken" {
        Err("doctor found broken checks".to_owned())
    } else {
        Ok(())
    }
}

fn run_host_doctor(
    target: &str,
    pretty: bool,
    codex_home: Option<PathBuf>,
    openhuman_root: Option<PathBuf>,
) -> Result<(), String> {
    let checks = match target {
        "codex" => vec![doctor_codex(codex_home)],
        "openhuman" => vec![doctor_openhuman(openhuman_root)],
        "hooks" => vec![doctor_codex(codex_home), doctor_openhuman(openhuman_root)],
        other => return Err(format!("unknown doctor host target: {other}")),
    };
    let status = checks
        .iter()
        .filter_map(|check| check.get("status").and_then(serde_json::Value::as_str))
        .fold("disabled", merge_doctor_status);
    let report = serde_json::json!({
        "status": status,
        "checks": checks,
    });
    if pretty {
        println!(
            "{}",
            serde_json::to_string_pretty(&report)
                .map_err(|error| format!("failed to serialize doctor report: {error}"))?
        );
    } else {
        println!(
            "{}",
            serde_json::to_string(&report)
                .map_err(|error| format!("failed to serialize doctor report: {error}"))?
        );
    }

    if status == "broken" {
        Err("doctor found broken host checks".to_owned())
    } else {
        Ok(())
    }
}

fn run_install(args: &[String]) -> Result<(), String> {
    if args.iter().any(|arg| arg == "-h" || arg == "--help") {
        print_install_usage();
        return Ok(());
    }
    let install = parse_host_mutation_args("install", args, true)?;
    match install.host.as_str() {
        "codex" => {
            let report = install_codex(install.codex_home, install.local_binary)?;
            println!(
                "{}",
                serde_json::to_string(&report)
                    .map_err(|error| format!("failed to serialize install report: {error}"))?
            );
            Ok(())
        }
        other => Err(format!("unknown install host: {other}")),
    }
}

fn run_uninstall(args: &[String]) -> Result<(), String> {
    if args.iter().any(|arg| arg == "-h" || arg == "--help") {
        print_uninstall_usage();
        return Ok(());
    }
    let uninstall = parse_host_mutation_args("uninstall", args, false)?;
    match uninstall.host.as_str() {
        "codex" => {
            let report = uninstall_codex(uninstall.codex_home)?;
            println!(
                "{}",
                serde_json::to_string(&report)
                    .map_err(|error| format!("failed to serialize uninstall report: {error}"))?
            );
            Ok(())
        }
        other => Err(format!("unknown uninstall host: {other}")),
    }
}

struct HostMutationArgs {
    host: String,
    codex_home: Option<PathBuf>,
    local_binary: Option<PathBuf>,
}

fn parse_host_mutation_args(
    command: &str,
    args: &[String],
    allow_local: bool,
) -> Result<HostMutationArgs, String> {
    let mut host = None;
    let mut codex_home = None;
    let mut local_binary = None;
    let mut i = 0;

    while i < args.len() {
        match args[i].as_str() {
            "--codex-home" => {
                i += 1;
                codex_home = Some(PathBuf::from(
                    args.get(i)
                        .ok_or_else(|| "--codex-home requires a value".to_owned())?,
                ));
            }
            "--local" if allow_local => {
                i += 1;
                local_binary =
                    Some(PathBuf::from(args.get(i).ok_or_else(|| {
                        "--local requires a binary path".to_owned()
                    })?));
            }
            value if value.starts_with('-') => {
                return Err(format!("unknown {command} option: {value}"));
            }
            value => {
                if host.replace(value.to_owned()).is_some() {
                    return Err(format!("{command} accepts exactly one host"));
                }
            }
        }
        i += 1;
    }

    Ok(HostMutationArgs {
        host: host.ok_or_else(|| format!("{command} requires a host"))?,
        codex_home,
        local_binary,
    })
}

fn install_codex(
    codex_home: Option<PathBuf>,
    local_binary: Option<PathBuf>,
) -> Result<serde_json::Value, String> {
    let root = codex_home.unwrap_or_else(default_codex_home);
    let instruction_path = codex_instruction_path(&root);
    let binary = local_binary
        .map(|path| path.to_string_lossy().into_owned())
        .unwrap_or_else(|| "tinyjuice".to_owned());
    let block = managed_codex_block(&binary);
    let original = match fs::read_to_string(&instruction_path) {
        Ok(content) => content,
        Err(error) if error.kind() == io::ErrorKind::NotFound => String::new(),
        Err(error) => {
            return Err(format!(
                "failed to read {}: {error}",
                instruction_path.display()
            ));
        }
    };
    let updated = replace_or_insert_managed_block(&original, &block);
    let changed = updated != original;
    if changed {
        write_managed_file(&instruction_path, &original, &updated)?;
    }
    Ok(serde_json::json!({
        "host": "codex",
        "status": "ok",
        "changed": changed,
        "configPath": instruction_path.to_string_lossy(),
        "command": format!("{binary} reduce-json"),
    }))
}

fn uninstall_codex(codex_home: Option<PathBuf>) -> Result<serde_json::Value, String> {
    let root = codex_home.unwrap_or_else(default_codex_home);
    let instruction_path = codex_instruction_path(&root);
    let original = match fs::read_to_string(&instruction_path) {
        Ok(content) => content,
        Err(error) if error.kind() == io::ErrorKind::NotFound => {
            return Ok(serde_json::json!({
                "host": "codex",
                "status": "ok",
                "changed": false,
                "configPath": instruction_path.to_string_lossy(),
            }));
        }
        Err(error) => {
            return Err(format!(
                "failed to read {}: {error}",
                instruction_path.display()
            ));
        }
    };
    let updated = remove_managed_block(&original);
    let changed = updated != original;
    if changed {
        write_managed_file(&instruction_path, &original, &updated)?;
    }
    Ok(serde_json::json!({
        "host": "codex",
        "status": "ok",
        "changed": changed,
        "configPath": instruction_path.to_string_lossy(),
    }))
}

fn doctor_codex(codex_home: Option<PathBuf>) -> serde_json::Value {
    let root = codex_home.unwrap_or_else(default_codex_home);
    let instruction_path = codex_instruction_path(&root);
    let repair_command = "tinyjuice install codex".to_owned();
    let expected_command = "tinyjuice reduce-json".to_owned();

    let content = match fs::read_to_string(&instruction_path) {
        Ok(content) => content,
        Err(error) if error.kind() == io::ErrorKind::NotFound => {
            return serde_json::json!({
                "name": "codex",
                "status": "disabled",
                "configPath": instruction_path.to_string_lossy(),
                "expectedCommand": expected_command,
                "repairCommand": repair_command,
            });
        }
        Err(error) => {
            return serde_json::json!({
                "name": "codex",
                "status": "broken",
                "configPath": instruction_path.to_string_lossy(),
                "expectedCommand": expected_command,
                "repairCommand": repair_command,
                "error": format!("failed to read Codex TinyJuice instructions: {error}"),
            });
        }
    };

    let Some(block) = managed_block(&content) else {
        return serde_json::json!({
            "name": "codex",
            "status": "disabled",
            "configPath": instruction_path.to_string_lossy(),
            "expectedCommand": expected_command,
            "repairCommand": repair_command,
        });
    };
    let detected_command = managed_command(block);
    let status = match detected_command.as_deref().and_then(command_binary_status) {
        Some(false) => "broken",
        _ => "ok",
    };
    let mut check = serde_json::json!({
        "name": "codex",
        "status": status,
        "configPath": instruction_path.to_string_lossy(),
        "expectedCommand": expected_command,
        "detectedCommand": detected_command,
        "repairCommand": repair_command,
    });
    if status == "broken" {
        check["error"] =
            serde_json::Value::String("managed command points at a missing executable".to_owned());
    }
    check
}

fn doctor_openhuman(openhuman_root: Option<PathBuf>) -> serde_json::Value {
    let root = openhuman_root.unwrap_or_else(default_openhuman_root);
    let adapter_path = root
        .join("src")
        .join("openhuman")
        .join("tokenjuice")
        .join("mod.rs");
    if adapter_path.is_file() {
        serde_json::json!({
            "name": "openhuman",
            "status": "ok",
            "configPath": adapter_path.to_string_lossy(),
            "expectedCommand": "direct Rust crate integration",
            "detectedCommand": "src/openhuman/tokenjuice",
            "repairCommand": "tinyjuice doctor openhuman --openhuman-root PATH",
        })
    } else {
        serde_json::json!({
            "name": "openhuman",
            "status": "disabled",
            "configPath": adapter_path.to_string_lossy(),
            "expectedCommand": "direct Rust crate integration",
            "repairCommand": "tinyjuice doctor openhuman --openhuman-root PATH",
        })
    }
}

fn default_codex_home() -> PathBuf {
    std::env::var_os("CODEX_HOME")
        .map(PathBuf::from)
        .or_else(|| std::env::var_os("HOME").map(|home| PathBuf::from(home).join(".codex")))
        .unwrap_or_else(|| PathBuf::from(".codex"))
}

fn default_openhuman_root() -> PathBuf {
    PathBuf::from("../openhuman-4")
}

fn codex_instruction_path(root: &Path) -> PathBuf {
    root.join("instructions").join("tinyjuice.md")
}

fn managed_codex_block(binary: &str) -> String {
    format!(
        "{TINYJUICE_MANAGED_BEGIN}\n\
         # TinyJuice\n\
         Command: {binary} reduce-json\n\
         When tool output is too large for context, prefer passing a structured payload to the command above or using `tinyjuice wrap -- <command>` when no post-tool hook is available.\n\
         {TINYJUICE_MANAGED_END}"
    )
}

fn managed_block(content: &str) -> Option<&str> {
    let start = content.find(TINYJUICE_MANAGED_BEGIN)? + TINYJUICE_MANAGED_BEGIN.len();
    let end = content[start..].find(TINYJUICE_MANAGED_END)? + start;
    Some(content[start..end].trim())
}

fn replace_or_insert_managed_block(content: &str, block: &str) -> String {
    if let Some((start, end)) = managed_block_bounds(content) {
        let mut out = String::with_capacity(content.len() + block.len());
        out.push_str(&content[..start]);
        out.push_str(block);
        out.push_str(&content[end..]);
        out
    } else if content.trim().is_empty() {
        format!("{block}\n")
    } else {
        format!("{}\n\n{block}\n", content.trim_end())
    }
}

fn remove_managed_block(content: &str) -> String {
    if let Some((start, end)) = managed_block_bounds(content) {
        let mut out = String::with_capacity(content.len());
        out.push_str(content[..start].trim_end());
        out.push_str(content[end..].trim_start_matches('\n'));
        if !out.is_empty() && !out.ends_with('\n') {
            out.push('\n');
        }
        out
    } else {
        content.to_owned()
    }
}

fn managed_block_bounds(content: &str) -> Option<(usize, usize)> {
    let start = content.find(TINYJUICE_MANAGED_BEGIN)?;
    let end = content[start..].find(TINYJUICE_MANAGED_END)? + start + TINYJUICE_MANAGED_END.len();
    Some((start, end))
}

fn write_managed_file(path: &Path, original: &str, updated: &str) -> Result<(), String> {
    let parent = path
        .parent()
        .ok_or_else(|| format!("{} has no parent directory", path.display()))?;
    fs::create_dir_all(parent)
        .map_err(|error| format!("failed to create {}: {error}", parent.display()))?;
    if path.exists() && !original.is_empty() {
        let backup = path.with_extension("md.bak");
        if !backup.exists() {
            fs::write(&backup, original)
                .map_err(|error| format!("failed to write backup {}: {error}", backup.display()))?;
        }
    }
    let tmp = path.with_extension("md.tmp");
    fs::write(&tmp, updated)
        .map_err(|error| format!("failed to write temp file {}: {error}", tmp.display()))?;
    fs::rename(&tmp, path).map_err(|error| format!("failed to replace {}: {error}", path.display()))
}

fn managed_command(block: &str) -> Option<String> {
    block.lines().find_map(|line| {
        line.trim()
            .strip_prefix("Command:")
            .map(str::trim)
            .filter(|command| !command.is_empty())
            .map(str::to_owned)
    })
}

fn command_binary_status(command: &str) -> Option<bool> {
    let binary = command.split_whitespace().next()?;
    if binary.contains(std::path::MAIN_SEPARATOR) || Path::new(binary).is_absolute() {
        Some(Path::new(binary).is_file())
    } else {
        None
    }
}

fn run_cat(args: &[String]) -> Result<(), String> {
    let mut store_dir = None;
    let mut token = None;
    let mut range = None;
    let mut i = 0;

    while i < args.len() {
        match args[i].as_str() {
            "--store-dir" => {
                i += 1;
                store_dir = Some(PathBuf::from(
                    args.get(i)
                        .ok_or_else(|| "--store-dir requires a value".to_owned())?,
                ));
            }
            "--lines" => {
                if range.is_some() {
                    return Err("cat accepts at most one range flag".to_owned());
                }
                i += 1;
                range = Some((
                    RangeUnit::Lines,
                    parse_range_arg(
                        args.get(i)
                            .ok_or_else(|| "--lines requires START:END".to_owned())?,
                        "--lines",
                    )?,
                ));
            }
            "--bytes" => {
                if range.is_some() {
                    return Err("cat accepts at most one range flag".to_owned());
                }
                i += 1;
                range = Some((
                    RangeUnit::Bytes,
                    parse_range_arg(
                        args.get(i)
                            .ok_or_else(|| "--bytes requires START:END".to_owned())?,
                        "--bytes",
                    )?,
                ));
            }
            "-h" | "--help" => {
                print_cat_usage();
                return Ok(());
            }
            value if value.starts_with('-') => return Err(format!("unknown cat option: {value}")),
            value => {
                if token.replace(value.to_owned()).is_some() {
                    return Err("cat accepts exactly one token".to_owned());
                }
            }
        }
        i += 1;
    }

    let store_dir = store_dir.ok_or_else(|| "cat requires --store-dir".to_owned())?;
    let token = token.ok_or_else(|| "cat requires a token".to_owned())?;
    if !is_cli_ccr_token(&token) {
        return Err("cat token must be a 32-character hex CCR token".to_owned());
    }

    tinyjuice::cache::enable_disk_tier(store_dir);
    let content = match range {
        Some((unit, (start, end))) => tinyjuice::cache::retrieve_range(&token, start, end, unit),
        None => tinyjuice::cache::retrieve(&token),
    }
    .ok_or_else(|| format!("token not found: {token}"))?;
    print!("{content}");
    Ok(())
}

fn parse_store_dir_args(command: &str, args: &[String]) -> Result<PathBuf, String> {
    let mut store_dir = None;
    let mut i = 0;

    while i < args.len() {
        match args[i].as_str() {
            "--store-dir" => {
                i += 1;
                store_dir = Some(PathBuf::from(
                    args.get(i)
                        .ok_or_else(|| "--store-dir requires a value".to_owned())?,
                ));
            }
            value => return Err(format!("unknown {command} option: {value}")),
        }
        i += 1;
    }

    store_dir.ok_or_else(|| format!("{command} requires --store-dir"))
}

fn parse_range_arg(raw: &str, flag: &str) -> Result<(usize, usize), String> {
    let (start, end) = raw
        .split_once(':')
        .ok_or_else(|| format!("{flag} expects START:END"))?;
    let start = start
        .parse::<usize>()
        .map_err(|_| format!("{flag} start must be a non-negative integer"))?;
    let end = end
        .parse::<usize>()
        .map_err(|_| format!("{flag} end must be a non-negative integer"))?;
    if end < start {
        return Err(format!("{flag} end must be greater than or equal to start"));
    }
    Ok((start, end))
}

fn is_cli_ccr_token(token: &str) -> bool {
    token.len() == 32 && token.bytes().all(|byte| byte.is_ascii_hexdigit())
}

struct CcrStoreInventory {
    entries: Vec<CcrStoreEntry>,
    ignored_entries: usize,
}

struct CcrStoreEntry {
    token: String,
    bytes: u64,
}

fn read_ccr_store_inventory(store_dir: &Path) -> Result<CcrStoreInventory, String> {
    let mut entries = Vec::new();
    let mut ignored_entries = 0;

    for entry in fs::read_dir(store_dir)
        .map_err(|error| format!("failed to read {}: {error}", store_dir.display()))?
    {
        let entry = entry.map_err(|error| format!("failed to read store entry: {error}"))?;
        if !entry
            .file_type()
            .map_err(|error| format!("failed to inspect store entry: {error}"))?
            .is_file()
        {
            ignored_entries += 1;
            continue;
        }
        let name = entry.file_name().to_string_lossy().into_owned();
        if is_cli_ccr_token(&name) {
            let bytes = entry
                .metadata()
                .map_err(|error| format!("failed to inspect store entry {name}: {error}"))?
                .len();
            entries.push(CcrStoreEntry { token: name, bytes });
        } else {
            ignored_entries += 1;
        }
    }

    entries.sort_by(|left, right| left.token.cmp(&right.token));
    Ok(CcrStoreInventory {
        entries,
        ignored_entries,
    })
}

fn merge_doctor_status(current: &str, next: &str) -> &'static str {
    match doctor_status_rank(current).max(doctor_status_rank(next)) {
        3 => "broken",
        2 => "warn",
        1 => "ok",
        _ => "disabled",
    }
}

fn doctor_status_rank(status: &str) -> u8 {
    match status {
        "broken" => 3,
        "warn" => 2,
        "ok" => 1,
        _ => 0,
    }
}

fn run_wrap(args: &[String]) -> Result<(), String> {
    let mut tool_name = "exec".to_owned();
    let mut max_inline_chars = None;
    let mut separator = None;
    let mut i = 0;

    while i < args.len() {
        match args[i].as_str() {
            "--" => {
                separator = Some(i);
                break;
            }
            "--tool-name" => {
                i += 1;
                tool_name = args
                    .get(i)
                    .ok_or_else(|| "--tool-name requires a value".to_owned())?
                    .clone();
            }
            "--max-inline-chars" => {
                i += 1;
                let value = args
                    .get(i)
                    .ok_or_else(|| "--max-inline-chars requires a value".to_owned())?;
                max_inline_chars = Some(
                    value
                        .parse::<usize>()
                        .map_err(|_| "--max-inline-chars must be a positive integer".to_owned())?,
                );
            }
            "-h" | "--help" => {
                print_wrap_usage();
                return Ok(());
            }
            value => return Err(format!("unknown wrap option before --: {value}")),
        }
        i += 1;
    }

    let Some(separator) = separator else {
        return Err("wrap requires -- before the command".to_owned());
    };
    let command_args = &args[separator + 1..];
    let Some(program) = command_args.first() else {
        return Err("wrap requires a command after --".to_owned());
    };

    let output = Command::new(program)
        .args(&command_args[1..])
        .output()
        .map_err(|error| format!("failed to run {program}: {error}"))?;
    let exit_code = output.status.code();
    let stdout = String::from_utf8_lossy(&output.stdout).into_owned();
    let stderr = String::from_utf8_lossy(&output.stderr).into_owned();
    let input = ToolExecutionInput {
        tool_name,
        command: Some(command_args.join(" ")),
        argv: Some(command_args.to_vec()),
        stdout: Some(stdout),
        stderr: Some(stderr),
        exit_code,
        ..Default::default()
    };
    let options = ReduceOptions {
        max_inline_chars,
        ..Default::default()
    };
    let rules = load_builtin_rules();
    let result = reduce_execution_with_rules(input, &rules, &options);
    print!("{}", result.inline_text);

    if output.status.success() {
        Ok(())
    } else {
        std::process::exit(exit_code.unwrap_or(1).clamp(1, 255));
    }
}

fn run_discover(args: &[String]) -> Result<(), String> {
    let mut pretty = false;
    let mut path = None;
    let mut i = 0;

    while i < args.len() {
        match args[i].as_str() {
            "--pretty" => pretty = true,
            "-h" | "--help" => {
                print_discover_usage();
                return Ok(());
            }
            value if value.starts_with('-') && value != "-" => {
                return Err(format!("unknown discover option: {value}"));
            }
            value => {
                if path.replace(value.to_owned()).is_some() {
                    return Err("discover accepts at most one path".to_owned());
                }
            }
        }
        i += 1;
    }

    let payload = read_input(path.as_deref())?;
    let inputs = parse_discovery_inputs(&payload)?;
    let rules = load_builtin_rules();
    let report = discover_fallback_outputs(inputs, &rules);
    if pretty {
        println!(
            "{}",
            serde_json::to_string_pretty(&report)
                .map_err(|error| format!("failed to serialize discovery report: {error}"))?
        );
    } else {
        println!(
            "{}",
            serde_json::to_string(&report)
                .map_err(|error| format!("failed to serialize discovery report: {error}"))?
        );
    }
    Ok(())
}

fn parse_discovery_inputs(payload: &str) -> Result<Vec<ToolExecutionInput>, String> {
    let trimmed = payload.trim();
    if trimmed.is_empty() {
        return Ok(Vec::new());
    }

    if trimmed.starts_with('[') {
        return serde_json::from_str(trimmed)
            .map_err(|error| format!("invalid discovery JSON array: {error}"));
    }

    let mut inputs = Vec::new();
    for (line_index, line) in payload.lines().enumerate() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let input = serde_json::from_str::<ToolExecutionInput>(line)
            .map_err(|error| format!("invalid discovery JSON line {}: {error}", line_index + 1))?;
        inputs.push(input);
    }
    Ok(inputs)
}

fn run_verify(args: &[String]) -> Result<(), String> {
    let mut check_rules = false;
    let mut check_fixtures = false;
    let mut fixtures_dir = PathBuf::from("tests/fixtures");
    let mut i = 0;

    while i < args.len() {
        match args[i].as_str() {
            "--rules" => check_rules = true,
            "--fixtures" => {
                check_fixtures = true;
                if let Some(next) = args.get(i + 1)
                    && !next.starts_with('-')
                {
                    i += 1;
                    fixtures_dir = PathBuf::from(next);
                }
            }
            "-h" | "--help" => {
                print_verify_usage();
                return Ok(());
            }
            value => return Err(format!("unknown verify option: {value}")),
        }
        i += 1;
    }

    if !check_rules && !check_fixtures {
        check_rules = true;
        check_fixtures = true;
    }

    let mut failed = false;
    if check_rules {
        let report = verify_rules(&LoadRuleOptions {
            exclude_user: true,
            exclude_project: true,
            ..Default::default()
        });
        println!(
            "rules: descriptors={} valid={} final={} parse_errors={} invalid_regexes={} duplicate_ids={} shadowed={}",
            report.descriptors_seen,
            report.valid_rules,
            report.final_rules,
            report.parse_errors.len(),
            report.invalid_regexes.len(),
            report.duplicate_ids.len(),
            report.shadowed_rules.len()
        );
        failed |= !report.parse_errors.is_empty() || !report.duplicate_ids.is_empty();
    }

    if check_fixtures {
        let rules = load_builtin_rules();
        let report = verify_rule_fixtures(&fixtures_dir, &rules);
        println!(
            "fixtures: dir={} seen={} passed={} parse_errors={} failures={}",
            fixtures_dir.display(),
            report.fixtures_seen,
            report.passed,
            report.parse_errors.len(),
            report.failures.len()
        );
        failed |= !report.is_clean();
    }

    if failed {
        Err("verification failed".to_owned())
    } else {
        Ok(())
    }
}

fn run_reduce(args: &[String]) -> Result<(), String> {
    let mut tool_name = "stdin".to_owned();
    let mut command = None;
    let mut exit_code = None;
    let mut max_inline_chars = None;
    let mut path = None;
    let mut i = 0;

    while i < args.len() {
        match args[i].as_str() {
            "--tool-name" => {
                i += 1;
                tool_name = args
                    .get(i)
                    .ok_or_else(|| "--tool-name requires a value".to_owned())?
                    .clone();
            }
            "--command" => {
                i += 1;
                command = Some(
                    args.get(i)
                        .ok_or_else(|| "--command requires a value".to_owned())?
                        .clone(),
                );
            }
            "--exit-code" => {
                i += 1;
                let value = args
                    .get(i)
                    .ok_or_else(|| "--exit-code requires a value".to_owned())?;
                exit_code = Some(
                    value
                        .parse::<i32>()
                        .map_err(|_| "--exit-code must be an integer".to_owned())?,
                );
            }
            "--max-inline-chars" => {
                i += 1;
                let value = args
                    .get(i)
                    .ok_or_else(|| "--max-inline-chars requires a value".to_owned())?;
                max_inline_chars = Some(
                    value
                        .parse::<usize>()
                        .map_err(|_| "--max-inline-chars must be a positive integer".to_owned())?,
                );
            }
            "-h" | "--help" => {
                print_reduce_usage();
                return Ok(());
            }
            value if value.starts_with('-') && value != "-" => {
                return Err(format!("unknown reduce option: {value}"));
            }
            value => {
                if path.replace(value.to_owned()).is_some() {
                    return Err("reduce accepts at most one path".to_owned());
                }
            }
        }
        i += 1;
    }

    let stdout = read_input(path.as_deref())?;
    let rules = load_builtin_rules();
    let input = ToolExecutionInput {
        tool_name,
        command,
        exit_code,
        stdout: Some(stdout),
        ..Default::default()
    };
    let options = ReduceOptions {
        max_inline_chars,
        ..Default::default()
    };
    let result = reduce_execution_with_rules(input, &rules, &options);
    print!("{}", result.inline_text);
    Ok(())
}

fn run_reduce_json(args: &[String]) -> Result<(), String> {
    let mut pretty = false;
    let mut path = None;
    let mut i = 0;

    while i < args.len() {
        match args[i].as_str() {
            "--pretty" => pretty = true,
            "-h" | "--help" => {
                print_reduce_json_usage();
                return Ok(());
            }
            value if value.starts_with('-') && value != "-" => {
                return Err(format!("unknown reduce-json option: {value}"));
            }
            value => {
                if path.replace(value.to_owned()).is_some() {
                    return Err("reduce-json accepts at most one path".to_owned());
                }
            }
        }
        i += 1;
    }

    let payload = read_input(path.as_deref())?;
    let rules = load_builtin_rules();
    match reduce_json_str(&payload, &rules) {
        Ok(response) => {
            if pretty {
                println!(
                    "{}",
                    serde_json::to_string_pretty(&response)
                        .map_err(|error| format!("failed to serialize response: {error}"))?
                );
            } else {
                println!(
                    "{}",
                    serde_json::to_string(&response)
                        .map_err(|error| format!("failed to serialize response: {error}"))?
                );
            }
            Ok(())
        }
        Err(error) => {
            let json = serde_json::to_string(&error).unwrap_or_else(|_| {
                r#"{"code":"internal","message":"serialization failed"}"#.to_owned()
            });
            Err(json)
        }
    }
}

fn read_input(path: Option<&str>) -> Result<String, String> {
    match path {
        Some("-") | None => {
            let mut input = String::new();
            io::stdin()
                .read_to_string(&mut input)
                .map_err(|error| format!("failed to read stdin: {error}"))?;
            Ok(input)
        }
        Some(path) => {
            fs::read_to_string(path).map_err(|error| format!("failed to read {path}: {error}"))
        }
    }
}

fn print_usage() {
    println!("Usage: tinyjuice <command> [options] [path|-]");
    println!();
    println!("Commands:");
    println!("  reduce       Reduce plain tool output from a file or stdin");
    println!("  reduce-json  Reduce a JSON protocol payload from a file or stdin");
    println!("  verify       Verify built-in rules and/or reducer fixtures");
    println!("  discover     Report command families still using generic fallback");
    println!("  wrap         Run a command and reduce its captured output");
    println!("  ls           List tokens from an explicit CCR disk store");
    println!("  cat          Print a token from an explicit CCR disk store");
    println!("  stats        Report metadata-only CCR disk store stats");
    println!("  doctor       Report TinyJuice health checks");
    println!("  install      Install TinyJuice integration for a host");
    println!("  uninstall    Remove TinyJuice integration from a host");
}

fn print_reduce_usage() {
    println!(
        "Usage: tinyjuice reduce [--tool-name NAME] [--command CMD] [--exit-code N] [--max-inline-chars N] [path|-]"
    );
}

fn print_reduce_json_usage() {
    println!("Usage: tinyjuice reduce-json [--pretty] [path|-]");
}

fn print_verify_usage() {
    println!("Usage: tinyjuice verify [--rules] [--fixtures [dir]]");
}

fn print_discover_usage() {
    println!("Usage: tinyjuice discover [--pretty] [path|-]");
    println!("Input is a JSON array or newline-delimited ToolExecutionInput objects.");
}

fn print_wrap_usage() {
    println!(
        "Usage: tinyjuice wrap [--tool-name NAME] [--max-inline-chars N] -- command [args...]"
    );
}

fn print_ls_usage() {
    println!("Usage: tinyjuice ls --store-dir DIR");
}

fn print_cat_usage() {
    println!("Usage: tinyjuice cat --store-dir DIR [--lines START:END | --bytes START:END] TOKEN");
}

fn print_stats_usage() {
    println!("Usage: tinyjuice stats --store-dir DIR [--pretty]");
}

fn print_doctor_usage() {
    println!(
        "Usage: tinyjuice doctor [host|hooks] [--pretty] [--fixtures [dir]] [--store-dir DIR] [--codex-home DIR] [--openhuman-root DIR]"
    );
}

fn print_install_usage() {
    println!("Usage: tinyjuice install codex [--codex-home DIR] [--local PATH]");
}

fn print_uninstall_usage() {
    println!("Usage: tinyjuice uninstall codex [--codex-home DIR]");
}
