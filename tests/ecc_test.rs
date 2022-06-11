#[cfg(test)]
mod point {
    use programming_bitcoin::{point::Point, finite_field::FieldElement};
    #[test]
    fn on_curve() {
        let prime: isize = 223;
        let a = FieldElement::new(0, prime).unwrap();
        let b = FieldElement::new(7, prime).unwrap();
        let valid_points  = [(192, 105), (17, 56), (1, 193)];
        for (x_raw, y_raw) in valid_points {
            let x: FieldElement = FieldElement::new(x_raw, prime).unwrap();
            let y: FieldElement = FieldElement::new(y_raw, prime).unwrap();
            let point = Point::new(Some(x), Some(y), a, b);
            assert!(matches!(point, Ok(_)));
        }
    }
    #[test]
    fn outside_curve() {
        let prime: isize = 223;
        let a = FieldElement::new(0, prime).unwrap();
        let b = FieldElement::new(7, prime).unwrap();
        let invalid_points  = [(200, 119), (42, 99)];
        for (x_raw, y_raw) in invalid_points {
            let x: FieldElement = FieldElement::new(x_raw, prime).unwrap();
            let y: FieldElement = FieldElement::new(y_raw, prime).unwrap();
            let point = Point::new(Some(x), Some(y), a, b);
            assert!(matches!(point, Err(_)));
        }
    }
}
