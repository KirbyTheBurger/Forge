use std::collections::HashSet;

pub struct LexerConfig {
    split: HashSet<SplitMode>,
    delimiters: HashSet<Delimiter>,
}

enum SplitMode {
    Char,
    Whitespace,
    Other(char),
}

struct Delimiter(char, char);

impl LexerConfig {
    pub fn new() -> LexerConfig {
        LexerConfig {
            split: HashSet::new(),
            delimiters: HashSet::new(),
        }
    }
}