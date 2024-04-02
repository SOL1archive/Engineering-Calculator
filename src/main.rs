use crate::term::{Polymomial, Term};

pub mod term;

fn main() {
    
}

#[test]
fn term_copy_test() {
    let term1 = Polymomial::new(1., 1.);
    let term2 = term1;

    assert_eq!(term2, term1);
}

#[test]
fn evaluation_test() {
    let linear = Polymomial::new(2., 1.);
    let x2 = Polymomial::new(1., 2.);
    assert_eq!(linear.evaluate(1.), 2.);
    assert_eq!(x2.evaluate(2.), 4.);
}
