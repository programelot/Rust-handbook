// function argument can use pattern either.
fn foo((x, y): &(i32, i32)) {
    println!("x,y = ({x},{y})");
}

struct Point {
    x: i32,
    y: i32,
}

struct Point3D {
    x: i32,
    y: i32,
    z: i32,
}

//Basic enum can be used as enum existed before.
enum Color {
    Red,
    Blue,
}

//Each enum can has own types
enum Query {
    Noop,                  //Empty
    Resize(u8, u8),        //two u8
    Write(String),         // String type
    Move { x: u8, y: u8 }, // two named fields. It starts with a bracket {} but not parenthesis ()
    Die(Color),            //Color enum
    Error,
}

fn main() {
    // match is a pattern that formed in the following way.
    // match VALUE {
    // PATTERN => EXPRESSION,
    // PATTERN => EXPRESSION,
    // }
    let a = Some(32);
    match a {
        Some(v) => println!("{v}"),
        None => println!("None"),
    }
    // 32

    // let is a pattern
    // let PATTERN = EXPRESSION;
    let (x, y, z) = (1, 2, 3);
    println!("x : {x}, y : {y}, z : {z}");
    // x : 1, y : 2, z : 3

    // This is impossible since, pattern doesn't work.
    // Left has a tuple with two elements and right has a tuple with three elements.
    // let (x, y) = (1, 2, 3);

    let a = Some(32);
    // if let works when it matches the pattern.
    if let Some(e) = a {
        println!("a is some and value is {e}");
    }
    // a is some and value is 32

    let mut v = vec![1, 2, 3, 4, 5];
    // while let repeats it untill pattern matchs
    while let Some(e) = v.pop() {
        println!("e = {e}");
    }
    // e = 5
    // e = 4
    // e = 3
    // e = 2
    // e = 1

    // for also have a pattern for it.
    // In the case below, (x, y) is a pattern.
    let v = vec![(1, 2), (3, 4), (5, 6)];
    for (index, (x, y)) in v.iter().enumerate() {
        println!("({index} : {x},{y})");
    }
    // (0 : 1,2)
    // (1 : 3,4)
    // (2 : 5,6)

    let p = (1, 2);
    foo(&p);
    // x,y = (1,2)

    // pattern are consisted by following ways.

    // .. means a range (exclusive)
    // ..= means a range (inclusive)
    // It works with integer and char only.

    // Exclusive (1,2,3,4,5,6)
    for i in 1..7 {
        match i {
            // | means or operation.
            1 | 2 => {
                println!("{i} is One or two.");
            }
            // Inclusive (3, 4)
            3..=4 => println!("{i} is greater or equal than 3 and smaller or equal than 4."),
            _ => println!("{i} is Something other."),
        }
    }
    // 1 is One or two.
    // 2 is One or two.
    // 3 is greater or equal than 3 and smaller or equal than 4.
    // 4 is greater or equal than 3 and smaller or equal than 4.
    // 5 is Something other.
    // 6 is Something other.

    for c in ['H', 'e', 'l', 'W', 'o', '!'] {
        match c {
            'a'..='z' => println!("{c} is lower case."),
            'A'..='Z' => println!("{c} is upper case."),
            _ => println!("{c} is not an alphabet."),
        }
    }
    // H is upper case.
    // e is lower case.
    // l is lower case.
    // W is upper case.
    // o is lower case.
    // ! is not an alphabet.

    // Pattern can be used for disassemble data.

    // Tuple disassemble works as below.
    let p = (10, 5);
    let (x, y) = p;
    println!("x = {x}, y = {y}");
    // x = 10, y = 5

    // Pattern can be applied with complex situation either.
    let nested_tuple = ((20, 30), Point { x: 3, y: 5 });
    let ((a, b), Point { x, y }) = nested_tuple;
    println!("a : {a}, b: {b}, x: {x}, y: {y}");
    // a : 20, b: 30, x: 3, y: 5

    // Struct disassemble works as below.
    let p = Point { x: 3, y: 5 };
    let Point { x: a, y: b } = p;
    println!("x = {a}, y = {b}");
    // x = 3, y = 5

    // It can be shorten in the following way.
    // Notice that it used simillar when making struct by matching the variable name with the field name.
    let p = Point { x: 3, y: 5 };
    let Point { x, y } = p;
    println!("x = {x}, y = {y}");
    // x = 3, y = 5

    // Pattern can be applied with some literals.
    // In the case below, it matches when x is 0 or y is 0 or both.
    // Notice that when matching didn't used as a variable, it doesn't exsits in the scope.
    // For example, Point{x : 0, y : 0} doesn't produce any variable.
    let p = Point { x: 0, y: 5 };
    match p {
        Point { x: 0, y: 0 } => println!("p(0, 0) is on the origin."),
        Point { x, y: 0 } => println!("p({x}, 0) is on x axis."),
        Point { x: 0, y } => println!("p(0, {y}) is on y axis."),
        Point { x, y } => println!("p({x}, {y}) is not on axis."),
    }
    // p(0, 5) is on y axis.

    // Instead of using it as it is, it can be used with restriction.
    // It is useful when pattern is complex.
    match p {
        Point { x, y } if x == 0 && y == 0 => println!("p({x}, {y}) is on the origin."),
        Point { x, y } if y == 0 => println!("p({x}, {y}) is on x axis."),
        Point { x, y } if x == 0 => println!("p({x}, {y}) is on y axis."),
        Point { x, y } => println!("p({x}, {y}) is not on axis."),
    }
    // p(0, 5) is on y axis.

    // It can solve the problem in the case below.
    let a = Some(5);
    let b = 5;
    match a {
        // Programmer may think this will be match case when a has the same value with b.
        // However, it just generates a new variable b and binds it with a's value.
        // Some(b) => println!("{v}"),
        // Therefor, instead of it, match guard can be used.
        Some(v) if v == b => println!("a is {v} which is the same with b"),
        Some(v) => println!("a is {v}"),
        None => println!("None"),
    }
    // a is 5 which is the same with b

    // Impossible, even if programmer covered every case from match, compiler can not undertand it.
    // Therefore, Some need to covered yet.
    // Notice that the case using guard is not considered as cover at all.

    // let a = Some(5);
    // match a {
    //     Some(v) if v % 2 == 0 => println!("a is even"),
    //     Some(v) if v % 2 == 1 => println!("a is odd"),
    //     None => println!("a is None"),
    // }

    // match multiple case also works with if.
    // Notice that it binds with all the cases.
    // In other word, "1 | 2 if f" means "(1 | 2) if f" but not "1 | (2 if f)"
    let f = false;
    for i in 1..3 {
        match i {
            1 | 2 if f => {
                println!("{i} is One or two.");
            }
            3 => println!("{i} is greater or equal than 3 and smaller or equal than 4."),
            _ => println!("{i} is Something other."),
        }
    }
    // 1 is Something other.
    // 2 is Something other.

    // When using range, it may still wants a symbol for it.
    // Then, @ can be used.
    // It is similar with using if instead of it.
    let l = (((10), 8), 6);
    match l.0 .0 {
        v @ 0..=5 => println!("{v} is in the range of 0~5."),
        v @ 6..=10 => println!("{v} is in the range of 6~10."),
        v => println!("{v} is not in the range."),
    }
    // 10 is in the range of 6~10

    // Recap of match for enum.
    // It is the same example from enum except Query::Die.
    let q = Query::Resize(8, 8);
    match q {
        Query::Noop => println!("No operation"),
        Query::Write(s) => println!("Write {0}!", s),
        Query::Resize(x, y) => println!("Resized to {0} by {1}.", x, y),
        Query::Move { x, y } => println!("Move to {0}, {1}", x, y),
        // Enum can be nested or unnested.
        Query::Die(Color::Red) => println!("Color changed to red."),
        Query::Die(Color::Blue) => println!("Color changed to blue."),
        // Below also works.
        // Query::Die(color) => match color {
        //     Color::Red => println!("Color changed to red."),
        //     Color::Blue => println!("Color changed to blue."),
        // },
        Query::Error => (),
    }
    // Resized to 8 by 8.

    // _ means ignoring it.
    // In this case, it only checks whether it is Some or not and ignore the value.
    // Note : variable starts with _ will not generate warning.
    // Rust usually generates a warning for not used variable.
    // However, it doesn't generates a warning for variable starts with _.
    let _a = 10; // It will not generate a warning.
    let mut a = Some(5);
    let mut b = Some(20);
    match (a, b) {
        (Some(_), Some(_)) => println!("Can not overwrite existing value"),
        _ => a = b,
    }
    //Can not overwrite existing value

    println!("{0} {1}", a.unwrap_or(-1), b.unwrap_or(-1));
    // 5 20

    // .. can replace many _.
    // It ignores all left things.
    let p = Point3D {
        x: 10,
        y: 20,
        z: 30,
    };

    if let Point3D { z, .. } = p {
        println!("z of p is {z}");
    }
    // z of p is 30

    // .. can ignore with the order of it.
    // It can not be applied in the middle.
    // In other word, it can only be used in the front and back.
    let p = (1, 2, 3, 4, 5);
    let (first, second, .., last) = p;
    println!("{first} {second} .. {last}");
    // 1 2 .. 5
}
