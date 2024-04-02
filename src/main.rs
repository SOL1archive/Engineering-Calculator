use crate::term::Polymomial;

pub mod term;

fn main() {
    
}

#[test]
fn term_copy_test() {
    let term1 = Polymomial::new(1., 1.);
    let term2 = term1;

    assert_eq!(term2, term1);
}
