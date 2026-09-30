//! A small sed subset over in-memory text: `p`, `d` and `s///[gip]`, with
//! addresses `N`, `$`, `/re/` and ranges `a,b`. No files, no `w`/`e`/`r`, so a
//! script can only select and rewrite lines it was given.

use regex::Regex;

use super::types::{Hit, ReplError, ReplLimits};

enum Addr {
    Line(usize),
    Last,
    Re(Regex),
}

enum Kind {
    Print,
    Delete,
    Subst {
        re: Regex,
        repl: String,
        global: bool,
        print: bool,
    },
}

struct Cmd {
    a1: Option<Addr>,
    a2: Option<Addr>,
    negate: bool,
    kind: Kind,
    active: bool,
}

fn bad(msg: &str) -> ReplError {
    ReplError::InvalidPattern(format!("sed: {msg}"))
}

struct Parser<'a> {
    s: &'a [u8],
    i: usize,
    limits: &'a ReplLimits,
}

impl Parser<'_> {
    fn peek(&self) -> Option<u8> {
        self.s.get(self.i).copied()
    }

    fn skip_ws(&mut self) {
        while matches!(self.peek(), Some(b' ' | b'\t')) {
            self.i += 1;
        }
    }

    /// Read up to an unescaped `delim`; `\delim` becomes `delim`.
    fn delimited(&mut self, delim: u8) -> Result<String, ReplError> {
        let mut out = Vec::new();
        while let Some(c) = self.peek() {
            self.i += 1;
            if c == delim {
                return String::from_utf8(out).map_err(|_| bad("not utf-8"));
            }
            if c == b'\\' && self.peek() == Some(delim) {
                out.push(delim);
                self.i += 1;
            } else {
                out.push(c);
            }
        }
        Err(bad("unterminated pattern"))
    }

    fn regex(&self, src: &str, icase: bool) -> Result<Regex, ReplError> {
        regex::RegexBuilder::new(src)
            .case_insensitive(icase)
            .size_limit(self.limits.regex_size_limit)
            .dfa_size_limit(self.limits.regex_size_limit)
            .build()
            .map_err(|e| bad(&e.to_string()))
    }

    fn addr(&mut self) -> Result<Option<Addr>, ReplError> {
        match self.peek() {
            Some(b'$') => {
                self.i += 1;
                Ok(Some(Addr::Last))
            }
            Some(b'/') => {
                self.i += 1;
                let src = self.delimited(b'/')?;
                let icase = if self.peek() == Some(b'I') {
                    self.i += 1;
                    true
                } else {
                    false
                };
                Ok(Some(Addr::Re(self.regex(&src, icase)?)))
            }
            Some(c) if c.is_ascii_digit() => {
                let start = self.i;
                while self.peek().is_some_and(|c| c.is_ascii_digit()) {
                    self.i += 1;
                }
                let n = std::str::from_utf8(&self.s[start..self.i])
                    .ok()
                    .and_then(|t| t.parse().ok())
                    .ok_or_else(|| bad("bad line number"))?;
                Ok(Some(Addr::Line(n)))
            }
            _ => Ok(None),
        }
    }

    fn command(&mut self) -> Result<Cmd, ReplError> {
        self.skip_ws();
        let a1 = self.addr()?;
        let a2 = if a1.is_some() && self.peek() == Some(b',') {
            self.i += 1;
            Some(self.addr()?.ok_or_else(|| bad("missing range end"))?)
        } else {
            None
        };
        self.skip_ws();
        let negate = if self.peek() == Some(b'!') {
            self.i += 1;
            true
        } else {
            false
        };
        self.skip_ws();
        let kind = match self.peek().ok_or_else(|| bad("missing command"))? {
            b'p' => {
                self.i += 1;
                Kind::Print
            }
            b'd' => {
                self.i += 1;
                Kind::Delete
            }
            b's' => {
                self.i += 1;
                let delim = self.peek().ok_or_else(|| bad("missing delimiter"))?;
                if delim.is_ascii_alphanumeric() || delim == b'\\' || delim == b'\n' {
                    return Err(bad("bad delimiter"));
                }
                self.i += 1;
                let pat = self.delimited(delim)?;
                let repl = self.delimited(delim)?;
                let (mut global, mut icase, mut print) = (false, false, false);
                while let Some(f) = self.peek() {
                    match f {
                        b'g' => global = true,
                        b'i' | b'I' => icase = true,
                        b'p' => print = true,
                        _ => break,
                    }
                    self.i += 1;
                }
                Kind::Subst {
                    re: self.regex(&pat, icase)?,
                    repl: convert_replacement(&repl),
                    global,
                    print,
                }
            }
            c => return Err(bad(&format!("unsupported command '{}'", c as char))),
        };
        Ok(Cmd {
            a1,
            a2,
            negate,
            kind,
            active: false,
        })
    }
}

/// sed's `\1` and `&` to the regex crate's `${1}` and `${0}`.
fn convert_replacement(repl: &str) -> String {
    let mut out = String::new();
    let mut chars = repl.chars().peekable();
    while let Some(c) = chars.next() {
        match c {
            '\\' => match chars.next() {
                Some(d) if d.is_ascii_digit() => out.push_str(&format!("${{{d}}}")),
                Some('n') => out.push('\n'),
                Some('t') => out.push('\t'),
                Some(o) => out.push(o),
                None => out.push('\\'),
            },
            '&' => out.push_str("${0}"),
            '$' => out.push_str("$$"),
            o => out.push(o),
        }
    }
    out
}

fn parse(script: &str, limits: &ReplLimits) -> Result<Vec<Cmd>, ReplError> {
    let mut p = Parser {
        s: script.as_bytes(),
        i: 0,
        limits,
    };
    let mut cmds = Vec::new();
    loop {
        while matches!(p.peek(), Some(b';' | b'\n' | b' ' | b'\t' | b'}')) {
            p.i += 1;
        }
        if p.peek().is_none() {
            break;
        }
        cmds.push(p.command()?);
        if cmds.len() > 16 {
            return Err(bad("too many commands"));
        }
    }
    if cmds.is_empty() {
        return Err(ReplError::EmptyQuery);
    }
    Ok(cmds)
}

fn addr_matches(a: &Addr, n: usize, last: bool, line: &str) -> bool {
    match a {
        Addr::Line(k) => *k == n,
        Addr::Last => last,
        Addr::Re(re) => re.is_match(line),
    }
}

fn selected(cmd: &mut Cmd, n: usize, last: bool, line: &str) -> bool {
    let hit = match (&cmd.a1, &cmd.a2) {
        (None, _) => true,
        (Some(a1), None) => addr_matches(a1, n, last, line),
        (Some(a1), Some(a2)) => {
            if cmd.active {
                if addr_matches(a2, n, last, line) {
                    cmd.active = false;
                }
                true
            } else if addr_matches(a1, n, last, line) {
                // A numeric end at or before the start selects just this line.
                cmd.active = !matches!(a2, Addr::Line(k) if *k <= n);
                true
            } else {
                false
            }
        }
    };
    hit != cmd.negate
}

/// Run `script` over `text`. With `quiet` (sed's `-n`) only `p` output is
/// returned. Each hit carries the 1-based number of the input line it came from.
pub fn run(
    text: &str,
    script: &str,
    quiet: bool,
    limits: &ReplLimits,
) -> Result<(Vec<Hit>, usize), ReplError> {
    let mut cmds = parse(script, limits)?;
    let lines: Vec<&str> = text.lines().collect();
    let total = lines.len();
    let mut out = Vec::new();
    let mut dropped = 0usize;
    let mut emit = |line: usize, s: &str, out: &mut Vec<Hit>| {
        if out.len() >= limits.max_lines {
            dropped += 1;
        } else {
            out.push(Hit {
                line,
                text: clip_line(s, limits.max_line_chars),
            });
        }
    };
    for (idx, raw) in lines.iter().enumerate() {
        let n = idx + 1;
        let last = n == total;
        let mut space = (*raw).to_string();
        let mut deleted = false;
        for cmd in &mut cmds {
            if !selected(cmd, n, last, &space) {
                continue;
            }
            match &cmd.kind {
                Kind::Print => emit(n, &space, &mut out),
                Kind::Delete => {
                    deleted = true;
                    break;
                }
                Kind::Subst {
                    re,
                    repl,
                    global,
                    print,
                } => {
                    let replaced = if *global {
                        re.replace_all(&space, repl.as_str())
                    } else {
                        re.replace(&space, repl.as_str())
                    };
                    if let std::borrow::Cow::Owned(new) = replaced {
                        space = new;
                        if *print {
                            emit(n, &space, &mut out);
                        }
                    }
                }
            }
        }
        if !quiet && !deleted {
            emit(n, &space, &mut out);
        }
    }
    Ok((out, dropped))
}

fn clip_line(line: &str, max: usize) -> String {
    if line.chars().count() <= max {
        return line.to_string();
    }
    line.chars().take(max).collect::<String>() + "…"
}
