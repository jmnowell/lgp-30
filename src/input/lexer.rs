use crate::common::error::*;
use crate::input::tokens::*;

use std::io::{self, BufRead, BufReader};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct LexerLine {
    pub line_number: usize,
    tokens: Vec<ASMToken>,
}

impl LexerLine {
    pub fn new(line_number: usize, tokens: Vec<ASMToken>) -> Self {
        LexerLine {
            line_number: line_number,
            tokens: tokens,
        }
    }

    pub fn push(&mut self, token: ASMToken) {
        self.tokens.push(token);
    }

    pub fn len(self) -> usize {
        self.tokens.len()
    }

    pub fn token_at(&self, index: usize) -> Option<ASMToken> {
        if index > self.tokens.len() {
            return None;
        }

        Some(self.tokens[index])
    }

    pub fn tokens(self) -> Vec<ASMToken> {
        return self.tokens.clone();
    }
}

pub struct Lexer {
    pub lines: Vec<LexerLine>,
    raw_lines: Vec<String>,
}

impl Lexer {
    pub fn read<R: BufRead>(&mut self, mut reader: R) -> Result<usize, Error> {
        let mut line = String::new();

        match reader.read_line(&mut line) {
            Ok(0) => return Ok(0),
            Ok(n) => {
                self.raw_lines.push(line);
                return Ok(n);
            }
            Err(_) => return Err(Error::LexerReadFailed),
        }
    }

    pub fn tokenize(&mut self) -> Result<(), Error> {}
}
