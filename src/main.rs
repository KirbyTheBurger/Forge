use forge::{lexer::Lexer, scanner::scan};

fn main() {
    let lines = scan("

    hello world
    ~~ whitespace
    hello world
    @@ world => forge
    ~~/ whitespace
    hello world
    @@ hello world => hello forge

    ".to_string());
    println!("{:?}", lines);
    let mut lexer = Lexer::new(lines);
    let tokens = lexer.tokenize();
    println!("{:?}", tokens);
}
