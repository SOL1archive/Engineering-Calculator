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
            lt.push_back(Token::Plus);
        } else if ch == '-' {
            lt.push_back(Token::Minus);
        } else if ch == '*' {
            lt.push_back(Token::Multiply);
        } else if ch == '/' {
            lt.push_back(Token::Divide);
        } else if ch == '(' {
            lt.push_back(Token::Lparen);
        } else if ch == ')' {
            lt.push_back(Token::Rparen);
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
    let mut formula = Formula::new();
    for token in token_lt.iter() {
        formula.push(match &token {
            Token::Illegal(token) => return Err(String::from("Unexpected Token: ") + token),
            _ => {},
        })
    }

    Ok(formula)
}
