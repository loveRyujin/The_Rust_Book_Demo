use std::ops::Add;

#[derive(Debug, Copy, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y
        }
    }
}

#[derive(Debug, PartialEq)]
struct Millmeters(i32);

#[derive(Debug, PartialEq)]
struct Meters(i32);

impl Add<Meters> for Millmeters {
    type Output = Millmeters;

    fn add(self, other: Meters) -> Millmeters {
        Millmeters(self.0 + other.0 * 1000)
    }
}

fn main() {
    assert_eq!(
        Point {x: 1, y: 1} + Point {x: 2, y: 2},
        Point {x: 3, y: 3}
    );

    assert_eq!(
        Millmeters(1) + Meters(1),
        Millmeters(1001)
    );

    println!("successfully assert!")
}
