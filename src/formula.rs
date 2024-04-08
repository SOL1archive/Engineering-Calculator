use crate::term;

use std::collections::LinkedList;

pub struct Formula {
    formula: LinkedList<term::Term>,
}

impl Formula {
    fn evaluate(&self, x: f64) {
        let mut result: f64 = 0.;
        for term in self.formula.iter() {
            result += term.evaluate(x);
        }
        result
    }

    fn differential(&self, x: f64) {
        (self.evaluate(x + f64::EPSILON) - self.evaluate(x - f64::EPSILON)) / (2 * f64::EPSILON)
    }

    fn integral(&self, start: f64, end: f64) {
        let interval: f64 = (end - start) / f64::EPSILON;
        let n: u64 = (1 / f64::EPSILON) as u64;
        let mut result: f64 = 0.;
        for i in 0..n {
            result += (self.evaluate(start + i * interval) + self.evaluate(start + (i + 1) * interval)) * interval * 2;
        }
        result
    }
}
