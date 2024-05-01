pub mod term;
pub mod parser;

use crate::term::{OperatorType, TermType, Term, Formula};

fn main() {
    
}

#[test]
fn evaluation_test() {
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
fn parser_test() {
    let text = String::from("+-*/()");
    let output = parser::parse(text);
    let expected = vec![
        parser::Token::Plus(String::from('+')),
        parser::Token::Minus(String::from('-')),
        parser::Token::Multiply(String::from('*')),
        parser::Token::Divide(String::from('/')),
        parser::Token::Lparen(String::from('(')),
        parser::Token::Rparen(String::from(')')),
    ];
    
    assert_eq!(output, expected);
}
