use std::fmt;
use std::ops::{Add, AddAssign, Mul, Neg, Sub, SubAssign};
use std::str::FromStr;

/// 2D point / direction vector. `x` = column, `y` = row, `y` grows downwards.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct Point {
    pub x: i64,
    pub y: i64,
}

/// Orthogonal directions, clockwise from up.
pub const DIRS4: [Point; 4] = [Point::UP, Point::RIGHT, Point::DOWN, Point::LEFT];

/// All eight directions, clockwise from up.
pub const DIRS8: [Point; 8] = [
    Point::UP,
    Point::new(1, -1),
    Point::RIGHT,
    Point::new(1, 1),
    Point::DOWN,
    Point::new(-1, 1),
    Point::LEFT,
    Point::new(-1, -1),
];

impl Point {
    pub const ORIGIN: Point = Point::new(0, 0);
    pub const UP: Point = Point::new(0, -1);
    pub const DOWN: Point = Point::new(0, 1);
    pub const LEFT: Point = Point::new(-1, 0);
    pub const RIGHT: Point = Point::new(1, 0);

    pub const fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }

    pub fn manhattan(self, other: Point) -> i64 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }

    /// Rotate a direction 90° clockwise (as seen on screen, y down).
    pub fn turn_right(self) -> Self {
        Self::new(-self.y, self.x)
    }

    /// Rotate a direction 90° anticlockwise (as seen on screen, y down).
    pub fn turn_left(self) -> Self {
        Self::new(self.y, -self.x)
    }

    pub fn neighbors4(self) -> impl Iterator<Item = Point> {
        DIRS4.into_iter().map(move |d| self + d)
    }

    pub fn neighbors8(self) -> impl Iterator<Item = Point> {
        DIRS8.into_iter().map(move |d| self + d)
    }
}

impl Add for Point {
    type Output = Point;
    fn add(self, o: Point) -> Point {
        Point::new(self.x + o.x, self.y + o.y)
    }
}

impl Sub for Point {
    type Output = Point;
    fn sub(self, o: Point) -> Point {
        Point::new(self.x - o.x, self.y - o.y)
    }
}

impl AddAssign for Point {
    fn add_assign(&mut self, o: Point) {
        *self = *self + o;
    }
}

impl SubAssign for Point {
    fn sub_assign(&mut self, o: Point) {
        *self = *self - o;
    }
}

impl Mul<i64> for Point {
    type Output = Point;
    fn mul(self, k: i64) -> Point {
        Point::new(self.x * k, self.y * k)
    }
}

impl Neg for Point {
    type Output = Point;
    fn neg(self) -> Point {
        Point::new(-self.x, -self.y)
    }
}

/// Parses the first two integers in the string, e.g. `"3,4"`, `"3 4"`, `"x=3, y=-4"`.
impl FromStr for Point {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match crate::nums::<i64>(s)[..] {
            [x, y] => Ok(Point::new(x, y)),
            _ => Err(format!("expected two coordinates in {s:?}")),
        }
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ops() {
        let a = Point::new(3, 4);
        let b = Point::new(1, 2);
        assert_eq!(a + b, Point::new(4, 6));
        assert_eq!(a - b, Point::new(2, 2));
        assert_eq!(a * 2, Point::new(6, 8));
        assert_eq!(-a, Point::new(-3, -4));
        assert_eq!(a.manhattan(Point::ORIGIN), 7);
    }

    #[test]
    fn turning() {
        assert_eq!(Point::UP.turn_right(), Point::RIGHT);
        assert_eq!(Point::RIGHT.turn_right(), Point::DOWN);
        assert_eq!(Point::UP.turn_left(), Point::LEFT);
    }

    #[test]
    fn parse() {
        assert_eq!("3,4".parse::<Point>().unwrap(), Point::new(3, 4));
        assert_eq!("-5 10".parse::<Point>().unwrap(), Point::new(-5, 10));
        assert!("1,2,3".parse::<Point>().is_err());
    }
}
