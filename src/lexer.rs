use std::collections::{HashMap, HashSet};

use crate::{error::Error, scanner::Line};

pub enum Token {
    Code(String),

    MacroDef {
        left: Vec<MacroToken>,
        right: Vec<MacroToken>,
    },
    MacroRem(Vec<MacroToken>),

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
    output: LexerOutput,
    input: Vec<Line>,
    pos: usize,
}

pub struct LexerOutput {
    tokens: Vec<Token>,
    gr_delim: HashMap<String, String>,
}

impl Lexer {
    pub fn new(input: Vec<Line>) -> Lexer {
        Lexer {
            config: LexerConfig {
                split: HashSet::new(),
                delim: HashMap::new(),
            },
            input,
            pos: 0,
        }
    }

    pub fn tokenize(&mut self) -> Result<Vec<Token>, Error> {
        let mut tokens = vec![];

        while let Some(l) = self.current() {
            
        }

        Ok(tokens)
    }

    pub fn read_tokens(&mut self, s: String) -> Result<Vec<Token>, Error> {
        todo!()
    }

    fn advance(&mut self) {
        self.pos += 1;
    }

    fn current(&self) -> Option<&Line> {
        self.input.get(self.pos)
    }

    fn next(&mut self) -> Option<&Line> {
        self.advance();
        self.current()
    }
}