use std::fmt;
use std::fmt::Display;
use std::ops::Add;

struct Point {
    x: i32,
    y: i32,
}

// Define a way to display Point.
impl Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

// some trait can depend on other trait.
// This trait requires to implementation of
// Point has a Display trait to use it.
trait BoxDisplay: Display {
    fn print_box(&self);
}

// Point has a Display trait so it works.
impl BoxDisplay for Point {
    fn print_box(&self) {
        println!("======");
        println!("{0}", self);
        println!("======");
    }
}

struct Not_Dispaly_Struct;

// It is impossible to compile unless Not_Dispaly_Struct have a Display trait.
// impl BoxDisplay for Not_Dispaly_Struct {
//     fn print_box(&self) {}
// }

fn main() {
    let p1 = Point { x: 1, y: 0 };
    p1.print_box();
    // ======
    // (1, 0)
    // ======
}
