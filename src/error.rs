use std::{fmt, error};
use LexerErrorKind::*;

#[derive(Debug)]
pub struct LexerError {
    line: usize,
    kind: LexerErrorKind,
}

#[derive(Debug)]
pub enum LexerErrorKind {
    MissingDelim,
    InvalidDelim(String),

    InvalidSplitmode(String),
    NoSplitMode,

    MissingCaptureName,
    InvalidMacro,
}

impl LexerError {
    pub fn from(line: usize, kind: LexerErrorKind) -> LexerError {
        LexerError { line, kind }
    }
}

impl fmt::Display for LexerError {
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

            MissingCaptureName => {
                write!(f, "Missing capture name at line {}", self.line)
            },
            InvalidMacro => {
                write!(f, "Line {}: invalid macro", self.line)
            },
        }
    }
}

impl error::Error for LexerError {}