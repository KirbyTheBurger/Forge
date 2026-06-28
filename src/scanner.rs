use std::{iter::Peekable, mem, str::Chars};

use Line::*;

#[derive(Debug)]
pub enum Line {
    Code(String),
    Split(bool, String),
    Delim(bool, String, String),
    GrDelim(bool, String, String),
    Macro(bool, String),
}

pub struct Scanner {
    lines: Vec<String>,
}

impl Scanner {
    pub fn new(input: String) -> Scanner {
        let borrowed_lines = input.lines().collect::<Vec<&str>>();
        Scanner {
            lines: borrowed_lines.iter().map(|s| s.to_string()).collect(),
        }
    }

    pub fn scan(&mut self) -> Vec<Line> {
        let mut tokens = vec![];

        for line in mem::take(&mut self.lines).into_iter() {
            if let Some(l) = self.infer_line(line) {
                tokens.push(l);
            }
        }

        tokens
    }

    fn infer_line(&mut self, line: String) -> Option<Line> {
        let mut chars = line.chars().peekable();
        let (c0, c1) = (chars.next(), chars.next());

        if c0 == c1 {
            if line.is_empty() {
                return None;
            }

            let c = c0.unwrap_or_else(|| unreachable!());
            match c {
                '@' => {
                    let def = check_def(&mut chars);
                    skip_whitespace(&mut chars);

                    return Some(Macro(def, chars.collect()));
                },
                '~' => {
                    let def = check_def(&mut chars);
                    skip_whitespace(&mut chars);

                    return Some(Split(def, chars.collect()));
                },
                '^' | '%' => {
                    let def = check_def(&mut chars);
                    skip_whitespace(&mut chars);

                    let (l, r) = (read_word(&mut chars), read_word(&mut chars));

                    return Some(match c {
                        '^' => Delim(def, l, r),
                        '%' => GrDelim(def, l, r),
                        _ => unreachable!()
                    });
                },
                _ => {}
            }
        }

        Some(Code(line))
    }
}

fn check_def(chars: &mut Peekable<Chars>) -> bool {
    if matches!(chars.next(), Some('/')) {
        return false;
    }
    true
}

fn read_word(chars: &mut Peekable<Chars>) -> String {
    let mut s = String::new();

    while let Some(c) = chars.next() {
        if !c.is_whitespace() {
            s.push(c);
        } else {
            break;
        }
    }

    s
}

fn skip_whitespace(chars: &mut Peekable<Chars>) {
    while let Some(c) = chars.peek() {
        if c.is_whitespace() {
            chars.next();
        } else {
            break;
        }
    }
}
