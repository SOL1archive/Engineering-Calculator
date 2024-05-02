use std::collections::LinkedList;

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

pub fn parse(text: String) -> LinkedList<Token> {
    let mut vec :LinkedList<Token> = LinkedList::new();
    for (i, ch) in text.chars().enumerate() {
        if ch.is_whitespace() {
            continue;
        } else if ch == '+' {
            vec.push_back(Token::Plus(String::from('+')));
        } else if ch == '-' {
            vec.push_back(Token::Minus(String::from('-')));
        } else if ch == '*' {
            vec.push_back(Token::Multiply(String::from('*')));
        } else if ch == '/' {
            vec.push_back(Token::Divide(String::from('/')));
        } else if ch == '(' {
            vec.push_back(Token::Lparen(String::from('(')));
        } else if ch == ')' {
            vec.push_back(Token::Rparen(String::from(')')));
        }
    }

    vec
}
