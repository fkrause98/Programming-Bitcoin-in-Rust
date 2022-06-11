use core::fmt;
use std::ops::{Add, Div, Mul, Sub};
#[derive(Clone, Copy)]
pub struct FieldElement {
    pub num: isize,
    pub prime: isize,
}

impl FieldElement {
    pub fn new(_num: isize, _prime: isize) -> Result<Self, String> {
        if _num > _prime || _num < 0 {
            return Err("A field element's num attribute should be lower than its prime and greater than 0!".to_string());
        }
        return Ok(FieldElement {
            num: _num,
            prime: _prime,
        });
    }
}

fn same_field(_num_0: &FieldElement, _num_1: &FieldElement) -> bool {
    return _num_0.prime == _num_1.prime;
}

impl Add for FieldElement {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        if !same_field(&self, &other) {
            panic!("Can't add elements from different prime fields");
        }
        return FieldElement {
            num: (self.num + other.num) % self.prime,
            prime: self.prime,
        };
    }
}

impl Add<isize> for FieldElement {
    type Output = Self;
    fn add(self, rhs: isize) -> Self {
        return FieldElement {
            num: (self.num + rhs) % self.prime,
            prime: self.prime,
        };
    }
}

impl Sub for FieldElement {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        if !same_field(&self, &other) {
            panic!("Can't substract elements from different prime fields");
        }
        let result = (self.num - other.num) % self.prime;
        return FieldElement {
            num: result,
            prime: self.prime,
        };
    }
}

impl Mul for FieldElement {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        if !same_field(&self, &other) {
            panic!("Can't multiply elements from different prime fields");
        }
        let result = (self.num * other.num) % self.prime;
        return FieldElement {
            num: result,
            prime: (self.prime),
        };
    }
}
impl Div for FieldElement {
    type Output = Self;
    fn div(self, other: Self) -> Self {
        if !same_field(&self, &other) {
            panic!("Cant divide multiple elements from different prime fields");
        }
        let result = (self.num / other.num) % self.prime;
        return FieldElement {
            num: result,
            prime: self.prime,
        };
    }
}
impl PartialEq for FieldElement {
    fn eq(&self, other: &Self) -> bool {
        return self.num == other.num && self.prime == other.prime;
    }
}

impl fmt::Display for FieldElement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} in field {}", self.num, self.prime)
    }
}

impl Mul<isize> for FieldElement {
    type Output = FieldElement;
    fn mul(self, rhs: isize) -> FieldElement {
        let result = (rhs * self.num) % self.prime;
        return FieldElement::new(result, self.prime).unwrap();
    }
}
