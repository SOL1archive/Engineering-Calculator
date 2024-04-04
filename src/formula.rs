pub mod term;

use std::collections::LinkedList;

pub struct Formula {
    formula: LinkedList<term::Term>,
}

impl Formula {
    pub fn new() {
        Formula {
            formula: LinkedList::new()
        }
    }

    pub fn new(&mut list: LinkedList<term::Term>) {
        Formula {
            formula: list
        }
    }
}

