pub mod term;
pub mod token;
pub mod lexer;

use std::collections::LinkedList;

use crate::term::{OperatorType, TermType, Term, Formula};

fn main() {
    
}

#[test]
fn term_evaluation_test() {
    let linear = Term::new(OperatorType::Plus, TermType::Polymomial, 2., 0.);
    let x2 = Term::new(OperatorType::Plus, TermType::Polymomial, 1., 2.);
    let log10 = Term::new(OperatorType::Plus, TermType::Log { base: 10.}, 1., 1.);
    let exp2 = Term::new(OperatorType::Plus, TermType::Exp { base: 2. }, 1., 1.);

    assert_eq!(linear.evaluate(1.), 2.);
    assert_eq!(x2.evaluate(2.), 4.);
    assert_eq!(log10.evaluate(10.), 1.);
    assert_eq!(exp2.evaluate(2.), 4.);
}

#[test]
fn formula_evaluation_test() {
    let linear = Term::new(OperatorType::Plus, TermType::Polymomial, 2., 0.);
    let x2 = Term::new(OperatorType::Plus, TermType::Polymomial, 1., 2.);
    let log10 = Term::new(OperatorType::Plus, TermType::Log { base: 10.}, 1., 1.);
    let exp2 = Term::new(OperatorType::Plus, TermType::Exp { base: 2. }, 1., 1.);
    
    let mut formula1 = Formula::new();
    formula1.push(linear);
    formula1.push(x2);

    let mut formula2 = Formula::new();
    formula2.push(log10);
    formula2.push(exp2);

    assert_eq!(formula1.evaluate(1.), 3.);
    assert_eq!(formula2.evaluate(1.), 2.);

    let linear = Term::new(OperatorType::Plus, TermType::Polymomial, 2., 1.);
    let x2 = Term::new(OperatorType::Multiply, TermType::Polymomial, 1., 2.);
    let mut multipy_formula = Formula::new();
    multipy_formula.push(linear);
    multipy_formula.push(x2);

    assert_eq!(multipy_formula.evaluate(2.), 16.);
}

#[test]
fn lexer_test() {
    let text1 = String::from("+-*/()");
    let output1 = lexer::lexer(text1);
    let expected1 = LinkedList::from([
        token::Token::Plus(String::from('+')),
        token::Token::Minus(String::from('-')),
        token::Token::Multiply(String::from('*')),
        token::Token::Divide(String::from('/')),
        token::Token::Lparen(String::from('(')),
        token::Token::Rparen(String::from(')')),
    ]);
    assert_eq!(output1, expected1);

    let text2 = String::from("x + 1");
    let output2 = lexer::lexer(text2);
    let expected2 = LinkedList::from([
        token::Token::Ident(String::from('x')),
        token::Token::Plus(String::from('+')),
        token::Token::Ident(String::from('1')),
    ]);
    assert_eq!(output2, expected2);

    let text3 = String::from("sin+ 1");
    let output3 = lexer::lexer(text3);
    let expected3 = LinkedList::from([
        token::Token::Ident(String::from("sin")),
        token::Token::Plus(String::from('+')),
        token::Token::Ident(String::from('1')),
    ]);
    assert_eq!(output3, expected3);
}
