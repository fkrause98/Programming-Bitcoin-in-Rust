#[cfg(test)]
mod point{

    use programming_bitcoin::point::Point;
    use programming_bitcoin::point::Num;
    #[test]
    fn point_in_curve() {
        let point = Point::new(Num::NonInf(-1),Num::NonInf(-1) , 5, 7);
        assert!(matches!(point, Ok(_)));
    }
    #[test]
    fn point_outside_curve() {
        let point = Point::new(Num::NonInf(-1), Num::NonInf(-2), 5, 7);
        assert!(matches!(point, Err(_)));
    }
    #[test]
    fn add_same_x_different_y() {
        let point_0 = Point::new(Num::NonInf(-1),Num::NonInf(1) , 5, 7).unwrap();
        let point_1 = Point::new(Num::NonInf(-1),Num::NonInf(-1) , 5, 7).unwrap();
        let infinity_5_7 = Point::new(Num::Inf, Num::Inf, 5, 7).unwrap();
        let must_be_infinity = (point_0 + point_1).unwrap();
        assert_eq!(must_be_infinity, infinity_5_7);
    }
}
