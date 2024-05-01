#[derive(Debug, PartialEq)]
pub enum Token {
    Illegal(String),
    EoF(String),
    
    Plus(String),
    Minus(String),
    Multiply(String),
    Divide(String),

    Lparen(String),
    Rparen(String),

    Ident(String),
    
    Log(String),
    Exp(String),
}

pub fn parse(text: String) -> Vec<Token> {
    let mut vec:Vec<Token> = Vec::new();
    for (i, ch) in text.chars().enumerate() {
        if ch.is_whitespace() {
            continue;
        } else if ch == '+' {
            vec.push(Token::Plus(String::from('+')));
        } else if ch == '-' {
            vec.push(Token::Minus(String::from('-')));
        } else if ch == '*' {
            vec.push(Token::Multiply(String::from('*')));
        } else if ch == '/' {
            vec.push(Token::Divide(String::from('/')));
        } else if ch == '(' {
            vec.push(Token::Lparen(String::from('(')));
        } else if ch == ')' {
            vec.push(Token::Rparen(String::from(')')));
        }
    }

    vec
}
