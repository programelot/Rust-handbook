#[derive(Debug)]
struct People{
    name : String,
    age : i32
}

//Rust program starts with main function
//Function starts with fn
//Function parameter exists in parenthesis.
//Block defined with {} brackets.
fn main() {
    let james = People{
        name : String::from("James"),
        age : 32
    };

    //Impossible, function for printing is not defined.
    //println!("{0}",james);
    
    //Print function for debugging.
    println!("{:?}",james);
    // People { name: "James", age: 32 }

    println!("{:#?}",james);
    // People {
    //     name: "James",
    //     age: 32,
    // }
    
    //dbg macro gives how it computed.
    //It takes ownership of expression and return it again.
    //Therefore, it can be used as follow.
    //Please, don't send ownership to dbg function if it is used for only printing it out.
    let tommy = People{
        name : String::from("Tommy"),
        age : dbg!(16 * 2)
    };
    // [src\main.rs:30:15] 16 * 2 = 32
    
    //Impossible since if this function takes tommy' ownership, next dbg! function can not be called.
    //dbg!(tommy);
    dbg!(&tommy);
    // [src\main.rs:34:5] &tommy = People {
    //     name: "Tommy",
    //     age: 32,
    // }
}
