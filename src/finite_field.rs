use std::ops::{Add, Div, Mul, Sub};
use core::fmt;
pub struct FieldElement {
    num: i32,
    prime: i32,
}

impl FieldElement {
    pub fn new(_num: i32, _prime: i32) -> FieldElement {
        return FieldElement {
            num: _num,
            prime: _prime,
        };
    }
    pub fn power(self, power: i32) -> Self{
        let n: i32 = power % (self.prime - 1);
        let num = i32::pow(self.num, n as u32) % self.prime;
        return FieldElement { num: num, prime: self.prime };
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
