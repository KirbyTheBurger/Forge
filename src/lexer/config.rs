use std::collections::{HashMap, HashSet};

pub struct LexerConfig {
    pub split: HashSet<SplitMode>,
    pub delimiters: HashMap<char, char>,
}

#[derive(PartialEq, Eq, Hash, Debug)]
pub enum SplitMode {
    Char,
    Whitespace,
    Other(char),
}

impl LexerConfig {
    pub fn new() -> LexerConfig {
        LexerConfig {
            split: HashSet::new(),
            delimiters: HashMap::new(),
        }
    }
}