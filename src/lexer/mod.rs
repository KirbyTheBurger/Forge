use crate::{lexer::config::{Delimiter, LexerConfig, SplitMode}, scanner::{Action, Line}};

mod config;

#[derive(Debug)]
pub enum Token {

}

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

    pub fn tokenize(& mut self) -> Vec<Token> {
        let mut tokens = vec![];
        
        loop {
            tokens.push(
                match self.current().cloned() {
                    Some(l) => match l {
                        Line::Delimiter(action, s) => {
                            let args = parse_args(&s);
                            let (left, right) = match (args.get(0), args.get(1)) {
                                (Some(l), Some(r)) => {
                                    (l.trim().to_string(), r.trim().to_string())
                                },
                                _ => continue,
                            };

                            let delim = Delimiter(left, right);
                            match action {
                                Action::Define => {
                                    self.config.delimiters.insert(delim);
                                },
                                Action::Remove => {
                                    if !self.config.delimiters.remove(&delim) {
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
                                s => SplitMode::Other(s.to_string()),
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
                        }

                        _ => todo!(),
                    },

                    None => break,
                }
            );
        }

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