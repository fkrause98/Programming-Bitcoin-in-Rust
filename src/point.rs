use core::ops::Add;
#[derive(Clone, Debug)]
pub struct Point {
    pub a: isize,
    pub b: isize,
    pub x: Option<isize>,
    pub y: Option<isize>,
}
impl Point {
    pub fn new(x: Option<isize>, y: Option<isize>, a: isize, b: isize) -> Result<Point, String> {
        match (&x, &y) {
            (None, None) => Ok(Point { a, b, x, y }),
            (Some(x_int), Some(y_int)) => {
                let left_side = y_int.pow(2);
                let right_side = (x_int.pow(3)) + (a * x_int) + b;
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
            (None, _, _, _) => self,
            (_, None, _, _) => other,
            (Some(x_1), Some(x_2), Some(y_1), Some(y_2)) => {
                let (new_x, new_y) = do_addition(x_1, x_2, y_1, y_2, self.a);
                return Point::new(new_x, new_y, self.a, self.b);
            }
            _ => unsafe { std::hint::unreachable_unchecked() },
        };
        return Ok(result);
    }
}
fn do_addition(x_1: isize, x_2: isize, y_1: isize, y_2: isize, a: isize) -> (Option<isize>, Option<isize>) {
    let same_x = x_1 == x_2;
    let additive_inverses = same_x && y_1 != y_2;
    let equal = (x_1 == x_2) && (y_1 == y_2);
    if additive_inverses {
        return (None, None);
    }
    if !same_x{
        let slope = (y_2 - y_1)/(x_2 - x_1);
        let x_3 = (slope.pow(2)) - x_1 - x_2;
        let y_3 = slope*(x_1 - x_3) - y_1;
        return (Some(x_3), Some(y_3))
    }
    if equal && y_1 == 0 {
       return (None, None)
    }
    if equal{
        let slope = (3*(x_1.pow(2)) + a)/(2*y_1);
        let x_3 = (slope.pow(2)) - (2 * x_1);
        let y_3 = slope*(x_1 - x_3) - y_1;
        return (Some(x_3), Some(y_3))
    }
    return (None, None)
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
