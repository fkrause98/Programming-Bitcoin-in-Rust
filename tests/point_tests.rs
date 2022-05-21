#[cfg(test)]
mod point {

    use programming_bitcoin::point::Point;
    #[test]
    fn point_inside_curve() {
        let pairs = [(-1, 1), (-1, -1), (2, -5), (3, -7), (3, 7), (18, -77)];
        for pair in pairs {
            let (x, y) = pair;
            let point = Point::new(Some(x), Some(y), 5, 7);
            assert!(matches!(point, Ok(_)));
        }
    }
    #[test]
    fn point_outside_curve() {
        let pairs = [(-4, 1), (-3, -1), (55, -5), (1, -7), (3, 9), (111, -77)];
        for pair in pairs {
            let (x, y) = pair;
            let point = Point::new(Some(x), Some(y), 5, 7);
            assert!(matches!(point, Err(_)));
        }
        let point = Point::new(Some(-1), Some(-2), 5, 7);
        assert!(matches!(point, Err(_)));
    }
    #[test]
    fn add_same_x_different_y() {
        let point_0 = Point::new(Some(-1), Some(1), 5, 7).unwrap();
        let point_1 = Point::new(Some(-1), Some(-1), 5, 7).unwrap();
        let infinity_5_7 = Point::new(None, None, 5, 7).unwrap();
        let must_be_infinity = (point_0 + point_1).unwrap();
        assert_eq!(must_be_infinity, infinity_5_7);
    }
    #[test]
    fn add_infinite() {
        let inf = None;
        let point_0 = Point::new(inf, inf, 5, 7).unwrap();
        let point_1 = Point::new(Some(2), Some(5), 5, 7).unwrap();
        assert_eq!(point_0, (point_0.clone() + point_1).unwrap());
    }
    #[test]
    fn add_different_x() {
        let point_0 = Point::new(Some(3), Some(7), 5, 7).unwrap();
        let point_1 = Point::new(Some(-1), Some(-1), 5, 7).unwrap();
        let result = (point_0.clone() + point_1.clone()).unwrap();
        assert_eq!(
            result,
            Point::new(Some(2), Some(-5), 5, 7).unwrap()
        );
    }
    // Tests taken from the books' repo.
    #[test]
    fn add_equal() {
        let a = Point::new(Some(-1), Some(-1), 5, 7).unwrap();
        let b = a.clone();
        let result = (a + b).unwrap();
        let expected = Point::new(Some(18), Some(77), 5, 7).unwrap();
        assert_eq!(expected, result);
    }
    #[test]
    fn not_equal() {
        let a = Point::new(Some(3), Some(-7), 5, 7);
        let b = Point::new(Some(18), Some(77), 5, 7);
        assert!(a != b);
        assert!(a == a);
    }
}
