use forge::{lexer::Lexer, scanner::scan};

fn main() {
    let lines = scan("

    ^^ { }
    ^^ ( )
    ^^ [[ ]]
    ~~ char
    ~~ whitespace
    ~~ r
    ~~ 54

    ".to_string());
    println!("{:?}", lines);
    let mut lexer = Lexer::new(lines);
    let tokens = lexer.tokenize();
    println!("{:?}", tokens);
}
