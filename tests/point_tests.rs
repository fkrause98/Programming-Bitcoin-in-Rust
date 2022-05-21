#[cfg(test)]
mod point {

    use programming_bitcoin::point::Num;
    use programming_bitcoin::point::Point;
    #[test]
    fn point_inside_curve() {
        let pairs = [(-1, 1), (-1, -1), (2, -5), (3, -7), (3, 7), (18, -77)];
        for pair in pairs{
            let (x, y) = pair;
            let point = Point::new(Num::NonInf(x), Num::NonInf(y), 5, 7);
            assert!(matches!(point, Ok(_)));
        }
    }
    #[test]
    fn point_outside_curve() {
        let pairs = [(-4, 1), (-3, -1), (55, -5), (1, -7), (3, 9), (111, -77)];
        for pair in pairs {
            let (x, y) = pair;
            let point = Point::new(Num::NonInf(x), Num::NonInf(y), 5, 7);
            assert!(matches!(point, Err(_)));
        }
        let point = Point::new(Num::NonInf(-1), Num::NonInf(-2), 5, 7);
        assert!(matches!(point, Err(_)));
    }
    #[test]
    fn add_same_x_different_y() {
        let point_0 = Point::new(Num::NonInf(-1), Num::NonInf(1), 5, 7).unwrap();
        let point_1 = Point::new(Num::NonInf(-1), Num::NonInf(-1), 5, 7).unwrap();
        let infinity_5_7 = Point::new(Num::Inf, Num::Inf, 5, 7).unwrap();
        let must_be_infinity = (point_0 + point_1).unwrap();
        assert_eq!(must_be_infinity, infinity_5_7);
    }
    #[test]
    fn add_infinite(){
        let inf = Num::Inf;
        let point_0 = Point::new(inf, inf, 5, 7).unwrap();
        let point_1 = Point::new(Num::NonInf(2), Num::NonInf(5), 5,7).unwrap();
        assert_eq!(point_0, (point_0.clone() + point_1).unwrap());
    }
    #[test]
    fn add_different_x() {
        let point_0 = Point::new(Num::NonInf(3), Num::NonInf(7), 5, 7).unwrap();
        let point_1 = Point::new(Num::NonInf(-1), Num::NonInf(-1), 5, 7).unwrap();
        let result = (point_0.clone() + point_1.clone()).unwrap();
        assert_eq!(
            result,
            Point::new(Num::NonInf(2), Num::NonInf(-5), 5, 7).unwrap()
        );
    }
}
