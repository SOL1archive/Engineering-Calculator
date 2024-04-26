pub mod term;
pub mod formula;

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
fn formula_evaluate() {
    
}
