use crate::scanner::Scanner;

mod lexer;
mod error;
mod scanner;

fn main() {
    let mut scanner = Scanner::new("
^^/ ( )
%% ( )
~~ char
@@ test => test1
test
    ".to_string());
    let lines = scanner.scan();
    println!("{:?}", lines);
}