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
        
        loop {
            tokens.extend(
                match self.current().cloned() {
                    Some(l) => match l {
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

                        _ => todo!(),
                    },

                    None => break,
                }
            );
        }

        Ok(tokens)
    }

    fn tokenize_code(&self, code: String) -> Vec<Token> {
        if self.config.split.contains(&SplitMode::Char) {
            return code.chars().filter(|c| {
                !(self.config.split.contains(&SplitMode::Other(*c)) ||
                (c.is_whitespace() && self.config.split.contains(&SplitMode::Whitespace)))
            }).map(|c| Token(c.to_string())).collect::<Vec<Token>>();
        }

        code.split(|c| {
            self.config.split.contains(&SplitMode::Other(c)) ||
            (c.is_whitespace() && self.config.split.contains(&SplitMode::Whitespace))
        })
        .filter(|s| !s.is_empty())
        .map(|s| Token(s.to_string())).collect::<Vec<Token>>()
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