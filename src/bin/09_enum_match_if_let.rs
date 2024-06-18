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

//Enum can have methods.
impl Query {
    fn call(&self) {
        //It defined in the same with normal methods.
        match self {
            //Math works with patterns.
            //It has following forms.
            //Pattern => Code,
            Query::Noop => println!("No operation"),
            //It can binds variables with value of it.
            Query::Write(s) => println!("Write {0}!", s),
            //If it has more than one variable, it need corresponding number of variables to be binded.
            Query::Resize(x, y) => println!("Resized to {0} by {1}.", x, y),
            //Structs binds in following ways.
            //Notice that it is bracket {} but not parenthesis ().
            Query::Move { x: px, y: py } => println!("Move to {0}, {1}", px, py),
            //Enum can be binded either.
            Query::Die(color) => match color {
                //It can be rebinded.
                Color::Red => println!("Color changed to red."),
                Color::Blue => println!("Color changed to blue."),
            },
            //Wild card cares everything except that has been handled.
            //Be caution that it need to be place at the end.
            //Otherwise, it will ignore other cases.
            //Match starts from the first case to the end.

            //One of above(wild card with/without binding) or perfect match is required.
            //wild card with binding
            other => println!("other"),
            //wild card without binding
            _ => println!("_"),
            //Match doesn't compiled when it is not complete.
            //It can be checked by commenting out all of these three options.
            Query::Error => (), //It can be just ignored by using ().
        }
    }
}

//Rust program starts with main function
//Function starts with fn
//Function parameter exists in parenthesis.
//Block defined with {} brackets.
fn main() {
    let color = Color::Red;
    let color = Color::Blue;

    let query = Query::Noop;
    query.call();
    // No operation
    let query = Query::Resize(8, 8);
    query.call();
    // Resized to 8 by 8.
    let query = Query::Write(String::from("Hello, world"));
    query.call();
    // Write Hello, world!
    let query = Query::Move { x: 5, y: 0 };
    query.call();
    // Move to 5, 0
    let query = Query::Die(Color::Red);
    query.call();
    // Color changed to red.
    let query = Query::Die(Color::Blue);
    query.call();
    // Color changed to blue.
    let query = Query::Error;
    query.call();
    // other

    let query = Query::Write(String::from("Hello, world"));
    //if let works when it is the matching pattern only.
    if let Query::Write(s) = query {
        println!("{s}");
    } else {
        //Else case can be used for all other cases.
        println!("Not a write query.");
    }
    // Hello, world

    let query = Query::Error;
    //if let works when it is the matching pattern only.
    if let Query::Write(s) = query {
        println!("{s}");
    } else {
        //Else case can be used for all other cases.
        println!("Not a write query.");
    }
    // Not a write query.
}
