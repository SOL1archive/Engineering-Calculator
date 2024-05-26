use std::collections::LinkedList;
use crate::token::Token;
use crate::term::{Term, Formula};

pub fn tokenize(text: String) -> LinkedList<Token> {
    let mut lt :LinkedList<Token> = LinkedList::new();
    let mut start_pos: usize = text.len();
    for (i, ch) in text.chars().enumerate() {
        if ch.is_whitespace() {
            continue;
        } else if ch == '+' {
            lt.push_back(Token::Plus(String::from('+')));
        } else if ch == '-' {
            lt.push_back(Token::Minus(String::from('-')));
        } else if ch == '*' {
            lt.push_back(Token::Multiply(String::from('*')));
        } else if ch == '/' {
            lt.push_back(Token::Divide(String::from('/')));
        } else if ch == '(' {
            lt.push_back(Token::Lparen(String::from('(')));
        } else if ch == ')' {
            lt.push_back(Token::Rparen(String::from(')')));
        } else if ch.is_alphanumeric() {
            if start_pos == text.len() {
                start_pos = i;
            }

            if i + 1 == text.len() || (! text.as_bytes()[i + 1].is_ascii_alphanumeric()) {
                lt.push_back(Token::Ident(String::from(&text[start_pos..i + 1])));
                start_pos = text.len();
            } 
        }
    }

    lt
}

fn parse(token_lt: LinkedList<Token>) -> Result<Formula, String> {
    let formula = Formula::new();
    for token in token_lt.iter() {
        formula.push(match &token {
            Token::Illegal(token) => return Err(String::from("Unexpected Token: ") + token),
            
        })
    }

    Ok(formula)
}
