use std::{error, fmt};
use ForgeErrorKind::*;

pub mod lexer;
pub mod scanner;

#[derive(Debug)]
pub struct ForgeError {
    line: usize,
    kind: ForgeErrorKind,
}

#[derive(Debug)]
pub enum ForgeErrorKind {
    MissingDelim,
    InvalidDelim(String),

    InvalidSplitmode(String),
    NoSplitMode,
}

impl ForgeError {
    pub fn from(line: usize, kind: ForgeErrorKind) -> ForgeError {
        ForgeError { line, kind }
    }
}

impl fmt::Display for ForgeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.kind {
            MissingDelim => {
                write!(f, "Missing a delimiter pair at line {}", self.line)
            },
            InvalidDelim(s) => {
                write!(f, "line {}: `{s}` is not a valid delimiter pair", self.line)
            },

            InvalidSplitmode(s) => {
                write!(f, "Invalid splitmode {s} at line {}", self.line)
            },
            NoSplitMode => {
                write!(f, "Missing splitmode at line {}", self.line)
            },
        }
    }
}

impl error::Error for ForgeError {}