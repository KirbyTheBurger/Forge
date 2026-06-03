use crate::lexer::config::LexerConfig;

mod config;

pub enum Token {

}

pub struct Lexer {
    config: LexerConfig,
    input: String,
}

impl Lexer {
    pub fn new(input: String) -> Lexer {
        Lexer {
            input,
            config: LexerConfig::new(),
        }
    }
}