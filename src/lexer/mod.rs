use crate::{lexer::config::{Delimiter, LexerConfig}, scanner::{Action, Line}};

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
                                    (l.to_string(), r.to_string())
                                },
                                _ => continue,
                            };

                            let delim = Delimiter(left, right);
                            match action {
                                Action::Define => {
                                    self.config.delimiters.insert(delim);
                                },
                                Action::Remove => {
                                    if self.config.delimiters.remove(&delim) == false {
                                        println!("attempted to remove non-existent delimiter");
                                    }
                                }
                            }

                            println!("{:?}", self.config.delimiters);

                            self.advance();
                            continue;
                        },

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