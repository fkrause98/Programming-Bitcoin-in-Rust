use core::ops::Add;
#[derive(PartialEq, Clone, Copy, Debug)]
pub enum Num {
    NonInf(isize),
    Inf,
}
#[derive(Clone, Debug)]
pub struct Point {
    pub a: isize,
    pub b: isize,
    pub x: Num,
    pub y: Num,
}
impl Point {
    pub fn new(x: Num, y: Num, a: isize, b: isize) -> Result<Point, String> {
        match (&x, &y) {
            (Num::Inf, Num::Inf) => Ok(Point { a, b, x, y }),
            (Num::NonInf(x_int), Num::NonInf(y_int)) => {
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
impl Add for Point {
    type Output = Result<Self, String>;
    fn add(self, other: Self) -> Result<Self, String> {
        if self.a != other.a && self.b != other.b {
            return Err(("Points are not on the same curve").to_string());
        }
        let result = match (self.x, other.x, self.y, other.y) {
            (Num::Inf, _, _, _) => self,
            (_, Num::Inf, _, _) => other,
            (Num::NonInf(x_1), Num::NonInf(x_2), Num::NonInf(y_1), Num::NonInf(y_2)) => {
                let (new_x, new_y) = do_addition(x_1, x_2, y_1, y_2);
                return Point::new(new_x, new_y, self.a, self.b);
            }
            _ => unsafe { std::hint::unreachable_unchecked() },
        };
        return Ok(result);
    }
}
fn do_addition(x_1: isize, x_2: isize, y_1: isize, y_2: isize) -> (Num, Num) {
    let same_x = x_1 == x_2;
    let additive_inverses = same_x && y_1 != y_2;
    let _result = if additive_inverses {
        return (Num::Inf, Num::Inf);
    } else {
        let slope = (y_2 - y_1) / (x_2 - x_1);
        let x_3 = slope.pow(2) - x_1 - x_2;
        let y_3 = (slope * (x_1 - x_3)) - y_1;
        println!("new x {} , new y {}", x_3, y_3);
        return (Num::NonInf(x_3), Num::NonInf(y_3));
    };
}
impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        let equal = (self.a == other.a)
            && (self.b == other.b)
            && (self.x == other.x)
            && (self.y == other.y);
        return equal;
    }
}
