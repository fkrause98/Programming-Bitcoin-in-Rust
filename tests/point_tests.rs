#[cfg(test)]
mod PointTests {

    use programming_bitcoin::point::Point;
    use programming_bitcoin::point::Number;
    #[test]
    fn point_in_curve() {
        let point = Point::new(Number::NonInf(-1),Number::NonInf(-1) , 5, 7);
        assert!(matches!(point, Ok(_)));
    }
    #[test]
    fn point_outside_curve() {
        let point = Point::new(Number::NonInf(-1), Number::NonInf(-2), 5, 7);
        assert!(matches!(point, Err(_)));
    }
}
