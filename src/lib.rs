pub mod lexer;
pub mod scanner;

pub struct Span {
    line: usize,
    column: usize,
}