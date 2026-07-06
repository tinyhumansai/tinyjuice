use std::fs;
use std::io::{self, Read, Write};
use std::process::ExitCode;

use tinyjuice::{
    ReduceOptions, ToolExecutionInput, load_builtin_rules, reduce_execution_with_rules,
    reduce_json_str,
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
}

fn print_reduce_usage() {
    println!(
        "Usage: tinyjuice reduce [--tool-name NAME] [--command CMD] [--exit-code N] [--max-inline-chars N] [path|-]"
    );
}

fn print_reduce_json_usage() {
    println!("Usage: tinyjuice reduce-json [--pretty] [path|-]");
}
