pub struct Point {
    a: isize,
    b: isize,
    x: isize,
    y: isize
}
impl Point {
    fn new(a: isize, b: isize, x: isize, y: isize) -> Result<Point, String>{
        let left_side = y.pow(2);
        let right_side = (x.pow(3)) + (a*x) + b;
        if left_side != right_side {
            Err(format!("({}, {}) is outside the curve!", left_side, right_side))
        } else {
            Ok(Point {a: a, b: b, x: x, y: y})
        }
    }
}
impl PartialEq for Point{
    fn eq(&self, other: &Self) -> bool{
        let equal = (self.a == other.a) && (self.b == other.b) && (self.x == other.x) && (self.y == other.y);
        return equal;
    }
}
