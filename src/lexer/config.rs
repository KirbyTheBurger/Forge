use std::collections::HashSet;

pub struct LexerConfig {
    pub split: HashSet<SplitMode>,
    pub delimiters: HashSet<Delimiter>,
}

#[derive(PartialEq, Eq, Hash, Debug)]
pub enum SplitMode {
    Char,
    Whitespace,
    Other(char),
}

#[derive(PartialEq, Eq, Hash, Debug)]
pub struct Delimiter(pub String, pub String);

impl LexerConfig {
    pub fn new() -> LexerConfig {
        LexerConfig {
            split: HashSet::new(),
            delimiters: HashSet::new(),
        }
    }
}