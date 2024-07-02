use std::fmt;
use std::fmt::Display;
use std::ops::Add;

struct Point {
    x: i32,
    y: i32,
}

// Define a way to compare two points by PartialEq trait.
impl PartialEq for Point {
    fn eq(&self, rhs: &Point) -> bool {
        self.x == rhs.x && self.y == rhs.y
    }
}

// Define a way to display Point.
impl Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

// Add trait defined in this way.
// Rhs has been defined as a default Rhs type.
// Notice that it supports Rhs type and Output type that may not the same with the input type.
// Notice that Output defined as a associated type.
// Because, using two generic can result in a situation that having one rhs type with two different output type.
// trait Add<Rhs=Self> {
//     type Output;
//     fn add(self, rhs: Rhs) -> Self::Output;
// }

// Therefore, it is the same with below.
// impl Add<Point> for Point {
impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

// If user want to use another type for rhs, it can be fixed with generic.
impl Add<(i32, i32)> for Point {
    type Output = Point;

    fn add(self, other: (i32, i32)) -> Point {
        Point {
            x: self.x + other.0,
            y: self.y + other.1,
        }
    }
}

fn main() {
    let p1 = Point { x: 1, y: 0 };
    let p2 = Point { x: 2, y: 3 };
    let p3 = Point { x: 3, y: 3 };
    println!("Check whether {0} + {1} = {2}", p1, p2, p3);
    let p4 = p1 + p2;
    println!("Result is {0}, therefore it is {1}", p4, p4 == p3);

    let p1 = Point { x: 1, y: 0 };
    let p2 = Point { x: 2, y: 2 };
    let p3 = Point { x: 3, y: 3 };
    println!("Check whether {0} + {1} = {2}", p1, p2, p3);
    let p3 = Point { x: 3, y: 3 };
    let p4 = p1 + p2;
    println!("Result is {0}, therefore it is {1}", p4, p4 == p3);

    let p1 = Point { x: 1, y: 0 };
    let p2 = (2, 2);
    let p3 = p1 + p2;
    println!("p3 is {p3}");
    // p3 is (3, 2)
}
