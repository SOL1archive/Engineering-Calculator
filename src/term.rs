pub trait Term : Copy + Clone {
    fn get_coefficient(&self) -> f64;
    fn get_exponent(&self) -> f64;
    fn evaluate(&self, x: f64) -> f64;
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
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

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Log {
    coefficient: f64,
    exponent: f64,
    base: f64,
}

impl Log {
    pub fn new(coefficient: f64, exponent: f64, base: f64) -> Log {
        Log {
            coefficient: coefficient,
            exponent: exponent,
            base: base
        }
    }
}

impl Term for Log {
    fn get_coefficient(&self) -> f64 {
        self.coefficient
    }

    fn get_exponent(&self) -> f64 {
        self.exponent
    }

    fn evaluate(&self, x: f64) -> f64 {
        self.coefficient * x.log(self.base)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Exp {
    coefficient: f64,
    base: f64,
}

impl Exp {
    pub fn new(coefficient: f64, base: f64) -> Exp {
        Exp {
            coefficient: coefficient,
            base: base,
        }
    }
}

impl Term for Exp {
    fn get_coefficient(&self) -> f64 {
        self.coefficient
    }

    fn get_exponent(&self) -> f64 {
        self.base
    }

    fn evaluate(&self, x: f64) -> f64 {
        self.coefficient * self.base.powf(x)
    }
}
