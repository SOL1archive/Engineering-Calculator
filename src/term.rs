use std::cmp;

pub trait Term : Copy + Clone + Eq {
    fn get_coefficient(&self) -> f64;
    fn get_exponent(&self) -> f64;
    fn evaluate(&self, x: f64) -> f64;
}

#[derive(Debug, Clone, Copy, Eq)]
pub struct Polymomial {
    coefficient: f64,
    exponent: f64,
}

impl Polymomial {
    pub fn new(coefficient: f64, exponent: f64) -> Polymomial {
        Polymomial {
            coefficient: coefficient,
            exponent: exponent
        }
    }
}

impl Term for Polymomial {
    fn get_coefficient(&self) -> f64 {
        self.coefficient
    }

    fn get_exponent(&self) -> f64 {
        self.exponent
    }

    fn evaluate(&self, x: f64) -> f64 {
        self.coefficient * x.powf(self.exponent)
    }
}

impl PartialEq for Polymomial {
    fn eq(&self, other: &Self) -> bool {
        self.coefficient == other.coefficient &&
        self.exponent == other.exponent
    }
}
