use std::collections::LinkedList;
use crate::token::Token;

pub fn lexer(text: String) -> LinkedList<Token> {
    let mut vec :LinkedList<Token> = LinkedList::new();
    let mut start_pos: usize = text.len();
    for (i, ch) in text.chars().enumerate() {
        if ch.is_whitespace() {
            if start_pos != text.len() {
                vec.push_back(Token::Ident(String::from(&text[start_pos..i])));
                start_pos = text.len();
            }
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
        } else if ch.is_alphanumeric() {
            if start_pos == text.len() {
                start_pos = i;
            }
        }
    }

    vec
}
