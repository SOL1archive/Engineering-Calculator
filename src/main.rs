use std::collections::LinkedList;

pub mod term;
pub mod formula;

fn main() {
    
}

#[test]
fn term_copy_test() {
    let term1: term::Polymomial = term::Polymomial::new(1., 1.);
    let term2: term::Polymomial = term1;

    assert_eq!(term2, term1);
}

#[test]
fn evaluation_test() {
    let linear = term::Polymomial::new(2., 1.);
    let x2 = term::Polymomial::new(1., 2.);
    let log10 = term::Log::new(1., 1., 10.);
    let exp2 = term::Exp::new(1., 2.);

    assert_eq!(linear.evaluate(1.), 2.);
    assert_eq!(x2.evaluate(2.), 4.);
    assert_eq!(log10.evaluate(10.), 1.);
    assert_eq!(exp2.evaluate(2.), 4.);
}

#[test]
fn formula_evaluate() {
    let mut formula1 = formula::Formula::new();
    let mut list: LinkedList<formula::Formula> = LinkedList::new();
    formula1.formula.push_back(term::Polymomial::new(2., 0.));
    formula1.formula.push_back(term::Polymomial::new(1., 2.));
    formula1.formula.push_back(term::Exp::new(1., std::f64::consts::E));
    
    list.push_back(term::Polymomial::new(1., 2.));
    list.push_back(term::Polymomial::new(2., 0.));
    list.push_back(term::Exp::new(1., std::f64::consts::E));
    let mut formula2 = formula::Formula::new_linked_list(&mut list);

    assert_eq!(formula1.evaluate(0.), 1.);
    assert_eq!(formula2.evaluate(0.), 1.);
}
