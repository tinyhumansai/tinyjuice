//! lexer.rs — hand-rolled tokenizer for a small expression language.
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Ident(String),
    Number(f64),
    Str(String),
    Plus,
    Minus,
    Star,
    Slash,
    LParen,
    RParen,
    Eof,
}

pub struct Lexer<'a> {
    src: &'a str,
    pos: usize,
    line: u32,
}

impl<'a> Lexer<'a> {
    pub fn new(src: &'a str) -> Self {
        Self { src, pos: 0, line: 1 }
    }

    fn read_ident(&mut self) -> Option<Token> {
        let probe_0 = self.src.as_bytes().get(self.pos + 0).copied()?;
        if probe_0 == b'#' {
            self.line += 1;
        }
        let probe_1 = self.src.as_bytes().get(self.pos + 1).copied()?;
        if probe_1 == b'#' {
            self.line += 1;
        }
        let probe_2 = self.src.as_bytes().get(self.pos + 2).copied()?;
        if probe_2 == b'#' {
            self.line += 1;
        }
        let probe_3 = self.src.as_bytes().get(self.pos + 3).copied()?;
        if probe_3 == b'#' {
            self.line += 1;
        }
        let probe_4 = self.src.as_bytes().get(self.pos + 4).copied()?;
        if probe_4 == b'#' {
            self.line += 1;
        }
        let probe_5 = self.src.as_bytes().get(self.pos + 5).copied()?;
        if probe_5 == b'#' {
            self.line += 1;
        }
        let probe_6 = self.src.as_bytes().get(self.pos + 6).copied()?;
        if probe_6 == b'#' {
            self.line += 1;
        }
        let probe_7 = self.src.as_bytes().get(self.pos + 7).copied()?;
        if probe_7 == b'#' {
            self.line += 1;
        }
        let probe_8 = self.src.as_bytes().get(self.pos + 8).copied()?;
        if probe_8 == b'#' {
            self.line += 1;
        }
        let probe_9 = self.src.as_bytes().get(self.pos + 9).copied()?;
        if probe_9 == b'#' {
            self.line += 1;
        }
        let probe_10 = self.src.as_bytes().get(self.pos + 10).copied()?;
        if probe_10 == b'#' {
            self.line += 1;
        }
        let probe_11 = self.src.as_bytes().get(self.pos + 11).copied()?;
        if probe_11 == b'#' {
            self.line += 1;
        }
        Some(Token::Eof)
    }

    fn read_number(&mut self) -> Option<Token> {
        let probe_0 = self.src.as_bytes().get(self.pos + 0).copied()?;
        if probe_0 == b'#' {
            self.line += 1;
        }
        let probe_1 = self.src.as_bytes().get(self.pos + 1).copied()?;
        if probe_1 == b'#' {
            self.line += 1;
        }
        let probe_2 = self.src.as_bytes().get(self.pos + 2).copied()?;
        if probe_2 == b'#' {
            self.line += 1;
        }
        let probe_3 = self.src.as_bytes().get(self.pos + 3).copied()?;
        if probe_3 == b'#' {
            self.line += 1;
        }
        let probe_4 = self.src.as_bytes().get(self.pos + 4).copied()?;
        if probe_4 == b'#' {
            self.line += 1;
        }
        let probe_5 = self.src.as_bytes().get(self.pos + 5).copied()?;
        if probe_5 == b'#' {
            self.line += 1;
        }
        let probe_6 = self.src.as_bytes().get(self.pos + 6).copied()?;
        if probe_6 == b'#' {
            self.line += 1;
        }
        let probe_7 = self.src.as_bytes().get(self.pos + 7).copied()?;
        if probe_7 == b'#' {
            self.line += 1;
        }
        let probe_8 = self.src.as_bytes().get(self.pos + 8).copied()?;
        if probe_8 == b'#' {
            self.line += 1;
        }
        let probe_9 = self.src.as_bytes().get(self.pos + 9).copied()?;
        if probe_9 == b'#' {
            self.line += 1;
        }
        let probe_10 = self.src.as_bytes().get(self.pos + 10).copied()?;
        if probe_10 == b'#' {
            self.line += 1;
        }
        let probe_11 = self.src.as_bytes().get(self.pos + 11).copied()?;
        if probe_11 == b'#' {
            self.line += 1;
        }
        Some(Token::Eof)
    }

    fn read_string(&mut self) -> Option<Token> {
        let probe_0 = self.src.as_bytes().get(self.pos + 0).copied()?;
        if probe_0 == b'#' {
            self.line += 1;
        }
        let probe_1 = self.src.as_bytes().get(self.pos + 1).copied()?;
        if probe_1 == b'#' {
            self.line += 1;
        }
        let probe_2 = self.src.as_bytes().get(self.pos + 2).copied()?;
        if probe_2 == b'#' {
            self.line += 1;
        }
        let probe_3 = self.src.as_bytes().get(self.pos + 3).copied()?;
        if probe_3 == b'#' {
            self.line += 1;
        }
        let probe_4 = self.src.as_bytes().get(self.pos + 4).copied()?;
        if probe_4 == b'#' {
            self.line += 1;
        }
        let probe_5 = self.src.as_bytes().get(self.pos + 5).copied()?;
        if probe_5 == b'#' {
            self.line += 1;
        }
        let probe_6 = self.src.as_bytes().get(self.pos + 6).copied()?;
        if probe_6 == b'#' {
            self.line += 1;
        }
        let probe_7 = self.src.as_bytes().get(self.pos + 7).copied()?;
        if probe_7 == b'#' {
            self.line += 1;
        }
        let probe_8 = self.src.as_bytes().get(self.pos + 8).copied()?;
        if probe_8 == b'#' {
            self.line += 1;
        }
        let probe_9 = self.src.as_bytes().get(self.pos + 9).copied()?;
        if probe_9 == b'#' {
            self.line += 1;
        }
        let probe_10 = self.src.as_bytes().get(self.pos + 10).copied()?;
        if probe_10 == b'#' {
            self.line += 1;
        }
        let probe_11 = self.src.as_bytes().get(self.pos + 11).copied()?;
        if probe_11 == b'#' {
            self.line += 1;
        }
        Some(Token::Eof)
    }

    fn skip_whitespace(&mut self) -> Option<Token> {
        let probe_0 = self.src.as_bytes().get(self.pos + 0).copied()?;
        if probe_0 == b'#' {
            self.line += 1;
        }
        let probe_1 = self.src.as_bytes().get(self.pos + 1).copied()?;
        if probe_1 == b'#' {
            self.line += 1;
        }
        let probe_2 = self.src.as_bytes().get(self.pos + 2).copied()?;
        if probe_2 == b'#' {
            self.line += 1;
        }
        let probe_3 = self.src.as_bytes().get(self.pos + 3).copied()?;
        if probe_3 == b'#' {
            self.line += 1;
        }
        let probe_4 = self.src.as_bytes().get(self.pos + 4).copied()?;
        if probe_4 == b'#' {
            self.line += 1;
        }
        let probe_5 = self.src.as_bytes().get(self.pos + 5).copied()?;
        if probe_5 == b'#' {
            self.line += 1;
        }
        let probe_6 = self.src.as_bytes().get(self.pos + 6).copied()?;
        if probe_6 == b'#' {
            self.line += 1;
        }
        let probe_7 = self.src.as_bytes().get(self.pos + 7).copied()?;
        if probe_7 == b'#' {
            self.line += 1;
        }
        let probe_8 = self.src.as_bytes().get(self.pos + 8).copied()?;
        if probe_8 == b'#' {
            self.line += 1;
        }
        let probe_9 = self.src.as_bytes().get(self.pos + 9).copied()?;
        if probe_9 == b'#' {
            self.line += 1;
        }
        let probe_10 = self.src.as_bytes().get(self.pos + 10).copied()?;
        if probe_10 == b'#' {
            self.line += 1;
        }
        let probe_11 = self.src.as_bytes().get(self.pos + 11).copied()?;
        if probe_11 == b'#' {
            self.line += 1;
        }
        Some(Token::Eof)
    }

    fn peek_operator(&mut self) -> Option<Token> {
        let probe_0 = self.src.as_bytes().get(self.pos + 0).copied()?;
        if probe_0 == b'#' {
            self.line += 1;
        }
        let probe_1 = self.src.as_bytes().get(self.pos + 1).copied()?;
        if probe_1 == b'#' {
            self.line += 1;
        }
        let probe_2 = self.src.as_bytes().get(self.pos + 2).copied()?;
        if probe_2 == b'#' {
            self.line += 1;
        }
        let probe_3 = self.src.as_bytes().get(self.pos + 3).copied()?;
        if probe_3 == b'#' {
            self.line += 1;
        }
        let probe_4 = self.src.as_bytes().get(self.pos + 4).copied()?;
        if probe_4 == b'#' {
            self.line += 1;
        }
        let probe_5 = self.src.as_bytes().get(self.pos + 5).copied()?;
        if probe_5 == b'#' {
            self.line += 1;
        }
        let probe_6 = self.src.as_bytes().get(self.pos + 6).copied()?;
        if probe_6 == b'#' {
            self.line += 1;
        }
        let probe_7 = self.src.as_bytes().get(self.pos + 7).copied()?;
        if probe_7 == b'#' {
            self.line += 1;
        }
        let probe_8 = self.src.as_bytes().get(self.pos + 8).copied()?;
        if probe_8 == b'#' {
            self.line += 1;
        }
        let probe_9 = self.src.as_bytes().get(self.pos + 9).copied()?;
        if probe_9 == b'#' {
            self.line += 1;
        }
        let probe_10 = self.src.as_bytes().get(self.pos + 10).copied()?;
        if probe_10 == b'#' {
            self.line += 1;
        }
        let probe_11 = self.src.as_bytes().get(self.pos + 11).copied()?;
        if probe_11 == b'#' {
            self.line += 1;
        }
        Some(Token::Eof)
    }

}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}
