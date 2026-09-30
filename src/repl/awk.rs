//! A small awk subset over in-memory text: one `pattern { print terms }` rule.
//!
//! Patterns: `/re/`, `!/re/`, `NR|NF|$N` compared with `== != < <= > >=`, and
//! `$N ~ /re/`. Terms: `$N`, `$NF`, `$0`, `NR`, `NF`, numbers, `"strings"`.
//! A leading `-F<sep>` sets the field separator. No variables, loops, functions
//! or I/O, so a program can only select and project the lines it was given.

use regex::Regex;

use super::types::{Hit, ReplError, ReplLimits};

enum Operand {
    Field(usize),
    LastField,
    Nr,
    Nf,
}

enum Test {
    Re(Regex, bool),
    Cmp(Operand, &'static str, String),
    Match(Operand, Regex),
}

enum Term {
    Operand(Operand),
    Lit(String),
}

fn bad(msg: &str) -> ReplError {
    ReplError::InvalidPattern(format!("awk: {msg}"))
}

fn build(src: &str, limits: &ReplLimits) -> Result<Regex, ReplError> {
    regex::RegexBuilder::new(src)
        .size_limit(limits.regex_size_limit)
        .dfa_size_limit(limits.regex_size_limit)
        .build()
        .map_err(|e| bad(&e.to_string()))
}

fn operand(tok: &str) -> Option<Operand> {
    match tok {
        "NR" => Some(Operand::Nr),
        "NF" => Some(Operand::Nf),
        "$NF" => Some(Operand::LastField),
        _ => tok.strip_prefix('$')?.parse().ok().map(Operand::Field),
    }
}

fn regex_literal(s: &str) -> Option<&str> {
    s.strip_prefix('/')?.strip_suffix('/')
}

fn parse_test(pat: &str, limits: &ReplLimits) -> Result<Option<Test>, ReplError> {
    let pat = pat.trim();
    if pat.is_empty() {
        return Ok(None);
    }
    let (negate, rest) = match pat.strip_prefix('!') {
        Some(r) => (true, r.trim()),
        None => (false, pat),
    };
    if let Some(src) = regex_literal(rest) {
        return Ok(Some(Test::Re(build(src, limits)?, negate)));
    }
    if let Some((lhs, rhs)) = rest.split_once('~') {
        let re = regex_literal(rhs.trim()).ok_or_else(|| bad("~ needs a /regex/"))?;
        let op = operand(lhs.trim()).ok_or_else(|| bad("bad operand before ~"))?;
        return Ok(Some(Test::Match(op, build(re, limits)?)));
    }
    for op in ["==", "!=", "<=", ">=", "<", ">"] {
        if let Some((lhs, rhs)) = rest.split_once(op) {
            let l = operand(lhs.trim()).ok_or_else(|| bad("bad comparison operand"))?;
            let r = rhs.trim().trim_matches('"').to_string();
            return Ok(Some(Test::Cmp(l, op, r)));
        }
    }
    Err(bad("unsupported pattern"))
}

fn parse_terms(body: &str) -> Result<Vec<Term>, ReplError> {
    let body = body.trim();
    let list = body
        .strip_prefix("print")
        .ok_or_else(|| bad("only `print` actions are supported"))?
        .trim();
    if list.is_empty() {
        return Ok(vec![Term::Operand(Operand::Field(0))]);
    }
    list.split(',')
        .map(|t| {
            let t = t.trim();
            if let Some(s) = t.strip_prefix('"').and_then(|s| s.strip_suffix('"')) {
                Ok(Term::Lit(s.to_string()))
            } else if let Some(o) = operand(t) {
                Ok(Term::Operand(o))
            } else if t.parse::<f64>().is_ok() {
                Ok(Term::Lit(t.to_string()))
            } else {
                Err(bad("unsupported term"))
            }
        })
        .collect()
}

fn value(op: &Operand, nr: usize, line: &str, fields: &[&str]) -> String {
    match op {
        Operand::Nr => nr.to_string(),
        Operand::Nf => fields.len().to_string(),
        Operand::LastField => fields.last().copied().unwrap_or("").to_string(),
        Operand::Field(0) => line.to_string(),
        Operand::Field(i) => fields.get(i - 1).copied().unwrap_or("").to_string(),
    }
}

fn compare(l: &str, op: &str, r: &str) -> bool {
    let ord = match (l.parse::<f64>(), r.parse::<f64>()) {
        (Ok(a), Ok(b)) => a.partial_cmp(&b),
        _ => Some(l.cmp(r)),
    };
    let Some(ord) = ord else { return false };
    match op {
        "==" => ord.is_eq(),
        "!=" => ord.is_ne(),
        "<" => ord.is_lt(),
        "<=" => ord.is_le(),
        ">" => ord.is_gt(),
        _ => ord.is_ge(),
    }
}

/// Run `program` over `text`; each hit carries its input line number.
pub fn run(text: &str, program: &str, limits: &ReplLimits) -> Result<(Vec<Hit>, usize), ReplError> {
    let mut program = program.trim();
    let mut sep: Option<Regex> = None;
    if let Some(rest) = program.strip_prefix("-F") {
        let end = rest.find(char::is_whitespace).unwrap_or(rest.len());
        let (s, tail) = rest.split_at(end);
        let s = s.trim_matches(|c| c == '\'' || c == '"');
        if s.is_empty() {
            return Err(bad("-F needs a separator"));
        }
        sep = Some(build(&regex::escape(s), limits)?);
        program = tail.trim();
    }
    let (pattern, action) = match program.find('{') {
        Some(i) => (
            &program[..i],
            Some(
                program[i..]
                    .trim()
                    .trim_start_matches('{')
                    .trim_end_matches('}'),
            ),
        ),
        None => (program, None),
    };
    let test = parse_test(pattern, limits)?;
    let terms = match action {
        Some(body) => parse_terms(body)?,
        None if test.is_some() => vec![Term::Operand(Operand::Field(0))],
        None => return Err(ReplError::EmptyQuery),
    };

    let mut out = Vec::new();
    let mut dropped = 0usize;
    for (idx, line) in text.lines().enumerate() {
        let nr = idx + 1;
        let fields: Vec<&str> = match &sep {
            Some(re) => re.split(line).collect(),
            None => line.split_whitespace().collect(),
        };
        let selected = match &test {
            None => true,
            Some(Test::Re(re, negate)) => re.is_match(line) != *negate,
            Some(Test::Match(op, re)) => re.is_match(&value(op, nr, line, &fields)),
            Some(Test::Cmp(op, cmp, rhs)) => compare(&value(op, nr, line, &fields), cmp, rhs),
        };
        if !selected {
            continue;
        }
        let rendered: Vec<String> = terms
            .iter()
            .map(|t| match t {
                Term::Lit(s) => s.clone(),
                Term::Operand(o) => value(o, nr, line, &fields),
            })
            .collect();
        if out.len() >= limits.max_lines {
            dropped += 1;
        } else {
            let text = rendered.join(" ");
            let text = if text.chars().count() > limits.max_line_chars {
                text.chars().take(limits.max_line_chars).collect::<String>() + "…"
            } else {
                text
            };
            out.push(Hit { line: nr, text });
        }
    }
    Ok((out, dropped))
}
