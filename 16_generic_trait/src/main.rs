use std::io;

// Generic struct can be defined as follow.
// T denotes the templated type.
struct Point<T> {
    x: T,
    y: T,
}

// Generic function defined in the following way.
// T denotes the templated type.
fn generate_point<T>(x: T, y: T) -> Point<T> {
    Point { x, y }
}

// Method for generic struct can be defined as follow.
// The reason why <T> mentioned at impl is to denote T next to Point (Point<T>) doesn't mean the name of already exsiting type.
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
    fn y(&self) -> &T {
        &self.y
    }
}

// Type can be specified.
// In this case, function is defined only for this type.
impl Point<f32> {
    fn distance(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

// Generic can use more than two other types.
struct Pair<X, Y> {
    x: X,
    y: Y,
}

// Generic method can use other generic types.
impl<X1, Y1> Pair<X1, Y1> {
    fn mix<X2, Y2>(self, other: Pair<X2, Y2>) -> Pair<X1, Y2> {
        Pair {
            x: self.x,
            y: other.y,
        }
    }
}

// trait is simillar with the interface.
// It makes a common function that can be used across.
trait Summary {
    // Method below only defines types.
    // It forces that user to implement the function for struct.
    fn summarize1(&self) -> String;

    // It has default implementation.
    // It left user to decision to implement it or not.
    fn summarize2(&self) -> String {
        String::from("Unknown")
    }
}

// Trait implementation works as below.
// Trait name goes after impl and for indicates which struct is it for.
impl Summary for Pair<u32, u32> {
    fn summarize1(&self) -> String {
        format!("({0}, {1})", self.x, self.y)
    }
}

impl Summary for Pair<f32, f32> {
    fn summarize1(&self) -> String {
        format!("({0}, {1})", self.x, self.y)
    }

    // Overriding function can be done by the same way to implement a method.
    fn summarize2(&self) -> String {
        format!("(x = {0}, y = {1})", self.x, self.y)
    }
}

// Function that uses trait type as variable
// To use trait as type, variable type must be denoted with impl ahead of the type.
// Recap, &[T] is a slice of T type.
fn print_summaries1(list: &[impl Summary]) {
    for e in list {
        print!("{0} ", e.summarize1());
    }
    println!("");
}

// This function is the same with above.
// Above type is a syntatic sugar.
// It bounds a type of variable to have Summary trait.
// It has advance when many variables need to be the same trait and the same type.
fn print_summaries2<T: Summary>(list: &[T]) {
    for e in list {
        print!("{0} ", e.summarize1());
    }
    println!("");
}

// Many trait can be forced at once.
// It can be added with + oeprator.
// Below is two types of forcing way.
fn print_summaries3(list: &[impl Summary + std::fmt::Display]) {
    for e in list {
        print!("{0} ", e.summarize1());
    }
    println!("");
}
fn print_summaries4<T: Summary + std::fmt::Display>(list: &[T]) {
    for e in list {
        print!("{0} ", e.summarize1());
    }
    println!("");
}

// Many trait can spoil the readability.
// Therefore, it can be written with where later.
fn multi_trait_function1<
    T: Summary + std::fmt::Display,
    U: std::marker::Copy + std::fmt::Display,
>(
    t: T,
    u: U,
) {
}

//This function is the same with the function above.
//However, it is better to read.
fn multi_trait_function2<T, U>(t: T, u: U)
where
    T: Summary + std::fmt::Display,
    U: std::marker::Copy + std::fmt::Display,
{
}

// Function can return trait either
fn max1<T: std::cmp::PartialOrd>(a: T, b: T) -> T {
    if a > b {
        return a;
    }
    b
}

// Notice that generic doesn't support type variance anyway.
// It will generate actual type when it compiles.
// Therefore, it can not support many types at once.
// Therefore, function below is impossible since a is u32, b is f32.
// fn max2(a: u32, b: f32) -> impl std::cmp::PartialOrd{
//     if a > b{
//         return a;
//     }
//     b
// }

// programmer can define function that only works when it only has certain traits.
// It can be defined along with methods that requires no such a thing.
impl<T: std::fmt::Display> Point<T> {
    fn to_string(&self) -> String {
        format! {"({0}, {1})", self.x, self.y}
    }
    //It actually included in the ToString trait.
    //Therefore, use it to implement it.
}

// To use trait, it must be included with use.
// For example, below code is needed.
// use 16_generic_trait::Summary;

fn main() {
    let p1 = Point { x: 1u32, y: 2u32 };
    let p2 = generate_point(3.0f32, 4.0f32);

    println!("p1 = ({0},{1})", p1.x(), p1.y());
    //p1 = (1,2)

    //Impossible, distance function not defined for Point<u32>.
    // println!("Distance to p1 is {0}",p1.distance());

    println!("p2 = ({0},{1})", p2.x(), p2.y());
    //p2 = (3,4)

    println!("Distance to p2 is {0}", p2.distance());
    //Distance to p2 is 5

    let p1 = Pair { x: 1u32, y: 'C' };
    let p2 = Pair {
        x: String::from("V"),
        y: 2u32,
    };

    println!("p1 = ({0},{1})", p1.x, p1.y);
    // p1 = (1,C)

    println!("p2 = ({0},{1})", p2.x, p2.y);
    // p2 = (V,3)

    let p3 = p1.mix(p2);
    println!("p3 = ({0},{1})", p3.x, p3.y);
    // p3 = (1,3)

    // Impossible, Summary trait didn't implemented for this type.
    // println!("P1 is {0}", p1.summarize());

    let p1 = Pair { x: 1u32, y: 2u32 };
    let p2 = Pair {
        x: 1.0f32,
        y: 2.0f32,
    };

    println!("P1 is {0}", p1.summarize1());
    // P1 is (1, 2)

    println!("P2 is {0}", p2.summarize1());
    // P2 is (1, 2)

    println!("P1 is {0}", p1.summarize2());
    // P1 is Unknown

    println!("P2 is {0}", p2.summarize2());
    // P2 is (x = 1, y = 2)

    let mut pairs = Vec::new();
    pairs.push(Pair { x: 1u32, y: 2u32 });
    pairs.push(Pair { x: 3u32, y: 4u32 });
    pairs.push(Pair { x: 5u32, y: 6u32 });

    print_summaries1(&pairs);
    //(1, 2) (3, 4) (5, 6)

    print_summaries2(&pairs);
    //(1, 2) (3, 4) (5, 6)

    let p = Point { x: 'c', y: 'd' };

    println!("{0}", p.to_string());
}
