use core::fmt;
use programming_bitcoin::point::Point;
use programming_bitcoin::finite_field::FieldElement;

fn main() {
    let test_field_0 = FieldElement::new(4, 4);
    let test_field_1 = FieldElement::new(4, 4);
    let test_field_2 = test_field_1 * test_field_0;
    println!("{}", test_field_2);
    // println!("{}", test_field == test_field);
}
