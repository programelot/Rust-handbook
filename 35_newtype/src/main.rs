use std::fmt;
use std::ops::{Deref, DerefMut};

// To implement something with a struct, one of following must be in the code.
// 1. definition of the struct
// 2. definition of the trait
// Therefore, programmer can not re-invent a Display trait for String.
// New type is a detour for this situation.
// It wrapps a type with certain name and implement a trait with it like belows.
struct Meter(u32);

// alias can be achieved with the following command.
// However it can not provent Meter used as u32 or u32 as a meter.
// It consdier Meter and u32 as the same thing in this case.
// type Meter = u32;

impl fmt::Display for Meter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} m", self.0)
    }
}

// Implementing a deref for it is useful since it makes struct can be used like unwrapped type.
impl Deref for Meter {
    type Target = u32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// DerefMut depend on Deref.
// Notice that Target don't need to defined again.
impl DerefMut for Meter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

fn foo_with_u32(v: &u32) {
    println!("foo_with_u32 : {0}", v);
}

fn add_with_u32(v: &mut u32) {
    *v += 10;
}

fn foo_with_meter(v: &Meter) {
    println!("foo_with_Meter : {0}", v);
}

fn main() {
    let mut ten_meter = 10;
    println!("ten_meter without wrapper : {}", ten_meter);
    // ten_meter without wrapper : 10
    add_with_u32(&mut ten_meter);
    // foo_with_u32 : 20
    foo_with_u32(&ten_meter);
    // foo_with_u32 : 20
    // Impossible u32 can not be dereferenced to Meter.
    // foo_with_meter(&ten_meter);

    let mut ten_meter = Meter(10);
    println!("ten_mteter with wrapper : {}", ten_meter);
    // ten_mteter with wrapper : 10 m
    add_with_u32(&mut ten_meter);
    // foo_with_u32 : 20
    foo_with_u32(&ten_meter);
    // foo_with_u32 : 20
    foo_with_meter(&ten_meter);
    // foo_with_Meter : 20 m
}
