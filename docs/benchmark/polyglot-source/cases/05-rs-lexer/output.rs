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

    fn read_ident(&mut self) -> Option<Token> { … 51 line(s) … ⟦tj:d9ff5e034ffcfd344f4a86f771103ba7⟧ }

    fn read_number(&mut self) -> Option<Token> { … 51 line(s) … ⟦tj:d9ff5e034ffcfd344f4a86f771103ba7⟧ }

    fn read_string(&mut self) -> Option<Token> { … 51 line(s) … ⟦tj:d9ff5e034ffcfd344f4a86f771103ba7⟧ }

    fn skip_whitespace(&mut self) -> Option<Token> { … 51 line(s) … ⟦tj:d9ff5e034ffcfd344f4a86f771103ba7⟧ }

    fn peek_operator(&mut self) -> Option<Token> { … 51 line(s) … ⟦tj:d9ff5e034ffcfd344f4a86f771103ba7⟧ }

}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}
[omitted blocks are individually retrievable: call tinyjuice_retrieve with the token inside an omission marker to expand just that block]

[compacted tool output — this is a PARTIAL view; the full original (9304 bytes) is available by calling tinyjuice_retrieve with token "6eb91cf87fac07d371febfef2c2e7b77" (marker ⟦tj:6eb91cf87fac07d371febfef2c2e7b77⟧)]