use std::collections::{HashMap, HashSet};

use crate::error::Error;

pub enum Token {
    Code(String),

    MacroDef {
        left: Vec<Token>,
        right: Vec<Token>,
    },
    MacroRem(Vec<Token>),

    GroupDelim {
        def: bool,
        left: String,
        right: String,
    },
}

pub enum MacroToken {
    Token(Token),
    Capture(char),
}

pub struct LexerConfig {
    split: HashSet<String>,
    delim: HashMap<String, String>,
}

pub struct Lexer {
    config: LexerConfig,
    input: Vec<char>,
    pos: usize,
}

impl Lexer {
    pub fn new(input: String) -> Lexer {
        Lexer {
            config: LexerConfig {
                split: HashSet::new(),
                delim: HashMap::new(),
            },
            input: input.chars().collect(),
            pos: 0,
        }
    }

    pub fn tokenize(&mut self) -> Result<Vec<Token>, Error> {
        let mut tokens = vec![];

        while let Some(&c) = self.current() {
            tokens.push(self.read_token(c)?);
            self.advance();
        }

        Ok(tokens)
    }

    pub fn read_token(&mut self, c: char) -> Result<Token, Error> {
        todo!()
    }

    fn advance(&mut self) {
        self.pos += 1;
    }

    fn current(&self) -> Option<&char> {
        self.input.get(self.pos)
    }

    fn next(&mut self) -> Option<&char> {
        self.advance();
        self.current()
    }
}