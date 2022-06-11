#[cfg(test)]
mod field_element_tests {
    use programming_bitcoin::power::Pow;
    use programming_bitcoin::finite_field::FieldElement;

    #[test]
    fn addition_simple() {
        let elem_0 = FieldElement::new(1, 2).unwrap();
        let elem_1 = FieldElement::new(1, 2).unwrap();
        let addition: FieldElement = (elem_0 + elem_1).unwrap();
        assert_eq!(addition.num, 0)
    }
    #[test]
    fn addition_greater_than_field() {
        let elem_0 = FieldElement::new(4, 5).unwrap();
        let elem_1 = FieldElement::new(3, 5).unwrap();
        let addition: FieldElement = (elem_0 + elem_1).unwrap();
        assert_eq!(addition.num, 2)
    }
    #[test]
    fn substraction_simple() {
        let elem_0 = FieldElement::new(6, 7).unwrap();
        let elem_1 = FieldElement::new(1, 7).unwrap();
        let addition: FieldElement = (elem_0 - elem_1).unwrap();
        assert_eq!(addition.num, 5)
    }
    #[test]
    fn multiplication_simple() {
        let elem_0 = FieldElement::new(6, 7).unwrap();
        let elem_1 = FieldElement::new(2, 7).unwrap();
        let mul: FieldElement = (elem_0 * elem_1).unwrap();
        assert_eq!(mul.num, 5);
    }
    #[test]
    #[should_panic]
    fn adding_distinct_fields_panics() {
        let elem_0 = FieldElement::new(2147483647, 7).unwrap();
        let elem_1 = FieldElement::new(2147483644, 5).unwrap();
        let _ = elem_0 + elem_1;
    }
    #[test]
    fn div_simple() {
        let elem_0 = FieldElement::new(6, 7).unwrap();
        let elem_1 = FieldElement::new(4, 7).unwrap();
        let one = (elem_0 / elem_1).unwrap();
        assert_eq!(one.num, 1);
    }
    #[test]
    fn power_test() {
        let elem_0 = FieldElement::new(3, 5).unwrap();
        assert_eq!(elem_0.power(5).num, 3);
    }
}
