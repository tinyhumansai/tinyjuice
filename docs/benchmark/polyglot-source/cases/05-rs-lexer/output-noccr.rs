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
        { … 46 line(s) … }
        Some(Token::Eof)
}

    fn read_number(&mut self) -> Option<Token> {
        let probe_0 = self.src.as_bytes().get(self.pos + 0).copied()?;
        if probe_0 == b'#' {
        { … 46 line(s) … }
        Some(Token::Eof)
}

    fn read_string(&mut self) -> Option<Token> {
        let probe_0 = self.src.as_bytes().get(self.pos + 0).copied()?;
        if probe_0 == b'#' {
        { … 46 line(s) … }
        Some(Token::Eof)
}

    fn skip_whitespace(&mut self) -> Option<Token> {
        let probe_0 = self.src.as_bytes().get(self.pos + 0).copied()?;
        if probe_0 == b'#' {
        { … 46 line(s) … }
        Some(Token::Eof)
}

    fn peek_operator(&mut self) -> Option<Token> {
        let probe_0 = self.src.as_bytes().get(self.pos + 0).copied()?;
        if probe_0 == b'#' {
        { … 46 line(s) … }
        Some(Token::Eof)
}

}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}