use std::mem;

use crate::{ForgeError, lexer::config::{LexerConfig, SplitMode}, scanner::{Action, Line}};

mod config;

#[derive(Debug)]
pub struct Token(String);

pub struct Lexer {
    config: LexerConfig,
    input: Vec<Line>,
    pos: usize,
}

impl Lexer {
    pub fn new(input: Vec<Line>) -> Lexer {
        Lexer {
            input,
            config: LexerConfig::new(),
            pos: 0,
        }
    }

    pub fn tokenize(&mut self) -> Result<Vec<Token>, ForgeError> {
        let mut tokens = vec![];
        
        while let Some(l) = self.current().cloned() {
            let new = match l {
                Line::Delimiter(action, s) => {
                    let args = parse_args(&s);
                    let (left, right) = match args.get(0) {
                        Some(s) => {
                            match (s.chars().nth(0), s.chars().nth(1)) {
                                (Some(l), Some(r)) => {
                                    (l, r)
                                },
                                _ => {
                                    return Err(ForgeError::from(self.pos,
                                        crate::ForgeErrorKind::InvalidDelim(s.to_string())
                                    ));
                                }
                            }
                        },
                        _ => {
                            self.advance();
                            continue;
                        },
                    };

                    match action {
                        Action::Define => {
                            self.config.delimiters.insert(left, right);
                        },
                        Action::Remove => {
                            if matches!(self.config.delimiters.remove(&right), None) {
                                println!("attempted to remove non-existent delimiter");
                            }
                        }
                    }

                    println!("{:?}", self.config.delimiters);

                    self.advance();
                    continue;
                },

                Line::Split(action, s) => {
                    let split = match parse_args(&s).get(0) {
                        Some(s) => s.trim(),
                        None => continue,
                    };

                    let splitmode = match split {
                        "char" => SplitMode::Char,
                        "whitespace" => SplitMode::Whitespace,
                        s => match s.chars().nth(0) {
                            Some(c) => SplitMode::Other(c),
                            None => {
                                return Err(ForgeError::from(self.pos,
                                    crate::ForgeErrorKind::NoSplitMode
                                ));
                            }
                        },
                    };

                    match action {
                        Action::Define => {
                            self.config.split.insert(splitmode);
                        },
                        Action::Remove => {
                            if !self.config.split.remove(&splitmode) {
                                println!("attempted to remove non existent splitmode");
                            }
                        }
                    }

                    println!("{:?}", self.config.split);

                    self.advance();
                    continue;
                },

                Line::Macro(action, s) => {
                    todo!()
                },

                Line::Code(s) => {
                    let tokens = self.tokenize_code(s);
                    self.advance();
                    tokens
                },
            };

            tokens.extend(new);
        }

        Ok(tokens)
    }

    fn tokenize_code(&self, code: String) -> Vec<Token> {
        let mut tokens = vec![];
        let mut current = String::new();
        let mut chars = code.chars().peekable();

        'outer: while let Some(c) = chars.next() {
            if let Some(&closing) = self.config.delimiters.get(&c) {
                push_clear_current(&mut tokens, &mut current);

                tokens.push(Token(c.to_string()));
                let mut delim = String::new();
                while let Some(&next) = chars.peek() {
                    chars.next();
                    if next == closing {
                        tokens.push(Token(delim));
                        tokens.push(Token(closing.to_string()));
                        continue 'outer;
                    }
                    delim.push(next);
                }

                continue;
            }

            if self.config.split.contains(&SplitMode::Char) {
                tokens.push(Token(c.to_string()));
            } else if self.config.split.contains(&SplitMode::Whitespace) && c.is_whitespace() {
                push_clear_current(&mut tokens, &mut current);
            } else if self.config.split.contains(&SplitMode::Other(c)) {
                push_clear_current(&mut tokens, &mut current);
                tokens.push(Token(c.to_string()));
            } else {
                current.push(c);
            }
        }

        push_clear_current(&mut tokens, &mut current);
        tokens
    }

    fn advance(&mut self) {
        self.pos += 1;
    }

    fn current(&self) -> Option<&Line> {
        self.input.get(self.pos)
    }
}

fn parse_args(s: &String) -> Vec<&str> {
    s.split_whitespace().skip(1).collect()
}

fn push_clear_current(tokens: &mut Vec<Token>, current: &mut String) {
    if !current.is_empty() {
        tokens.push(Token(mem::take(current)));
    }
}