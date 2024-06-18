#[derive(Debug, PartialEq)]
pub enum Token {
    Illegal(String),
    
    Plus,
    Minus,
    Multiply,
    Divide,

    Lparen,
    Rparen,

    Ident(String),
    
    Log(String),
    Exp(String),
}
