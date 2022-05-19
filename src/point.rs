use core::ops::Add;
#[derive(PartialEq)]
#[derive(Clone)]
pub enum Number {
    NonInf(isize),
    Infinity,
}
pub struct Point {
    pub a: isize,
    pub b: isize,
    pub x: Number,
    pub y: Number,
}
impl Point {
    pub fn new(x: Number, y: Number, a: isize, b: isize) -> Result<Point, String> {
        match (&x, &y) {
            (Number::Infinity, Number::Infinity) => Ok(Point { a, b, x, y }),
            (Number::NonInf(x_int), Number::NonInf(y_int)) => {
                let left_side = y_int.pow(2);
                println!("Left side {}", left_side);
                let right_side = (x_int.pow(3)) + (a * x_int) + b;
                println!("Right side {}", right_side);
                let not_in_curve = left_side != right_side;
                if not_in_curve {
                    Err(format!(
                        "({}, {}) is outside the curve!",
                        left_side, right_side
                    ))
                } else {
                    Ok(Point { a, b, x, y })
                }
            }
            (_, _) => Err(("Unexpected combination of x and y").to_string()),
        }
    }
}
// impl Add for Point{
//     type Output = Self;
//     // fn do_addition(self, other: Self) -> (Number, Number){
//     //     // match (self.x, other.x)
//     // }
//     // fn add(self, other: Self) -> Result<Self, String>{
//     //     if self.a != other.a && self.b != other.b {
//     //         return Err("Points are not on the same curve").to_string();
//     //     }
//     //     let result = match (self, other) {
//     //         (Number::Infinity, _) => self,
//     //         (_, Number::Infinity) => other,
//     //         (_, _) => (1, 1)
//     //             // do_addition(self, other: Self)
//     //     };
//     //     return Ok(result);
//     // }
// }
impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        let equal = (self.a == other.a)
            && (self.b == other.b)
            && (self.x == other.x)
            && (self.y == other.y);
        return equal;
    }
}
