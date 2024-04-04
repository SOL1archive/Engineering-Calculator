use crate::term;

use std::collections::LinkedList;

pub struct Formula {
    formula: LinkedList<term::Term>,
}
