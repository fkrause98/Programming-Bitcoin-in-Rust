use crate::finite_field::FieldElement;

pub trait Pow {
    type Output;
    fn power(self, power: isize) -> Self;
}

impl Pow for isize {
    type Output = Self;
    fn power(self, power: isize) -> Self {
        return isize::pow(self, power as u32);
    }
}

impl Pow for FieldElement {
    type Output = Self;
    fn power(self, power: isize) -> Self {
        let n: isize = power % (self.prime - 1);
        let num = isize::pow(self.num, n as u32) % self.prime;
        return FieldElement {
            num,
            prime: self.prime,
        };
    }
}
