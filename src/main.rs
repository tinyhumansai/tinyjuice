use std::fs;
use std::io::{self, Read, Write};
use std::path::PathBuf;
use std::process::Command;
use std::process::ExitCode;

use tinyjuice::cache::RangeUnit;
use tinyjuice::{
    LoadRuleOptions, ReduceOptions, ToolExecutionInput, discover_fallback_outputs,
    load_builtin_rules, reduce_execution_with_rules, reduce_json_str, verify_rule_fixtures,
    verify_rules,
};

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
    let mut tokens = Vec::new();
    for entry in fs::read_dir(&store_dir)
        .map_err(|error| format!("failed to read {}: {error}", store_dir.display()))?
    {
        let entry = entry.map_err(|error| format!("failed to read store entry: {error}"))?;
        if !entry
            .file_type()
            .map_err(|error| format!("failed to inspect store entry: {error}"))?
            .is_file()
        {
            continue;
        }
        let name = entry.file_name().to_string_lossy().into_owned();
        if is_cli_ccr_token(&name) {
            tokens.push(name);
        }
    }
    tokens.sort();
    for token in tokens {
        println!("{token}");
    }
    Ok(())
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
