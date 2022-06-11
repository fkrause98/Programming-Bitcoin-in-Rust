use crate::finite_field::FieldElement;
use crate::power::Pow;
use std::{
    fmt::Display,
    ops::{Add, Div, Mul, Sub},
};
#[derive(Clone, Debug)]
pub struct Point<T> {
    pub a: T,
    pub b: T,
    pub x: Option<T>,
    pub y: Option<T>,
}
impl<
        T: Pow
            + Mul<T, Output = T>
            + Mul<isize, Output = T>
            + PartialEq
            + Add<T, Output = T>
            + Add<isize, Output = T>
            + Display
            + Copy,
    > Point<T>
{
    pub fn new(x: Option<T>, y: Option<T>, a: T, b: T) -> Result<Point<T>, String> {
        match (x, y) {
            (None, None) => Ok(Point {
                a: a,
                b: a,
                x: None,
                y: None,
            }),
            (Some(x_int), Some(y_int)) => {
                let left_side = y_int.power(2);
                let x_times_a = x_int * a;
                let x_plus_b = x_times_a + b;
                let right_side = (x_int.power(3)) + x_plus_b;
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
impl<
        T: PartialEq<isize>
            + PartialEq<T>
            + Add<T, Output = T>
            + Mul<T, Output = T>
            + Pow
            + Mul<isize, Output = T>
            + Div<T, Output = T>
            + Sub<T, Output = T>
            + Add<isize, Output = T>
            + Display
            + Copy,
    > Add for Point<T>
{
    type Output = Result<Self, String>;
    fn add(self, other: Self) -> Result<Self, String> {
        if self.a != other.a && self.b != other.b {
            return Err(("Points are not on the same curve").to_string());
        }
        return match (self.x, other.x, self.y, other.y) {
            (None, _, _, _) => Point::new(self.x, self.y, self.a, self.b),
            (_, None, _, _) => Point::new(other.x, other.y, self.a, self.b),
            (Some(x_1), Some(x_2), Some(y_1), Some(y_2)) => {
                let same_x = x_1 == x_2;
                let additive_inverses = same_x && y_1 != y_2;
                let equal = (x_1 == x_2) && (y_1 == y_2);
                if additive_inverses {
                    return Point::new(None, None, self.a, self.b);
                }
                if !same_x {
                    let slope = (y_2 - y_1) / (x_2 - x_1);
                    let x_3 = (slope.power(2)) - x_1 - x_2;
                    let y_3 = slope * (x_1 - x_3) - y_1;
                    return Point::new(Some(x_3), Some(y_3), self.a, self.b);
                }
                if equal && y_1 == 0 {
                    return Point::new(None, None, self.a, self.b);
                }
                if equal {
                    let slope = ((x_1.power(2) * 3) + self.a) / (y_1 * 2);
                    let x_3 = (slope.power(2)) - (x_1 * 2);
                    let y_3 = slope * (x_1 - x_3) - y_1;
                    return Point::new(Some(x_3), Some(y_3), self.a, self.b);
                }
                let inf: Result<Point<T>, String> = Point::new(None, None, self.a, self.b);
                return inf;
            }
            _ => unsafe { std::hint::unreachable_unchecked() },
        };
    }
}
impl PartialEq for Point<isize> {
    fn eq(&self, other: &Self) -> bool {
        let equal = (self.a == other.a)
            && (self.b == other.b)
            && (self.x == other.x)
            && (self.y == other.y);
        return equal;
    }
}
