//Rust program starts with main function
//Function starts with fn
//Function parameter exists in parenthesis.
//Block defined with {} brackets.
fn main() {
    //////////////////////////////////////////////////
    // Big Caution with good example
    // If something ends with semi colon (;), it is statement.
    // Otherwise it is expression.
    // It return some values if is an expression
    //////////////////////////////////////////////////////
    let a = 25;
    let b = if a < 20 {
        println!("{a} < 20");
        0
    } else if a < 30 {
        println!("20 <= {a} < 30");
        1
    } else {
        println!("{a} >= 30");
        2
    };
    //20 <= 25 < 30
    println!("b : {b}");
    //b : 1

    let mut c = 0;
    let d = loop {
        //It loops until meet the break
        println!("loop {c}");
        c = c + 1;
        if c > 5 {
            break 10; //Loop is an expression and it returns value of break function
        }
    };
    // loop 0
    // loop 1
    // loop 2
    // loop 3
    // loop 4
    // loop 5
    println!("d : {d}");
    // d : 10

    //Loop can be nested and below is a break at once example.
    let mut c = 0;
    let r = 'out_most_loop: loop {
        println!("loop {c}");
        let mut d = 0;
        let mut h = loop {
            println!("loop {c} {d}");
            d = d + 1;
            if d > 2 && c > 2 {
                //It can break more than one loop
                //In this case, it can still return value by writing like belows.
                break 'out_most_loop 25;
            }
            if d > 5 {
                //In this case, break above is not belongs to this loop.
                //Therefore, type doesn't need to the same.
                break "hey";
            }
        };
        c = c + 1;
    };
    // loop 0
    // loop 0 0
    // loop 0 1
    // loop 0 2
    // loop 0 3
    // loop 0 4
    // loop 0 5
    // loop 1
    // loop 1 0
    // loop 1 1
    // loop 1 2
    // loop 1 3
    // loop 1 4
    // loop 1 5
    // loop 2
    // loop 2 0
    // loop 2 1
    // loop 2 2
    // loop 2 3
    // loop 2 4
    // loop 2 5
    // loop 3
    // loop 3 0
    // loop 3 1
    // loop 3 2
    println!("r {r}");
    //r 25

    let mut a = 5;

    //While and for looks like an expression but doesn't return value.
    // Error message : you can't `break` with a value in a `while` loop
    //It seems like these two forms' expression returns () which is an empty tuple.
    //It looks like designed so because there is no way to analyze all break points in an expression.
    while a > 0 {
        println!("a {a}");
        a = a - 1;
    }
    // a 5
    // a 4
    // a 3
    // a 2
    // a 1
    let v = [1, 2, 3, 4, 5];
    for e in v {
        println!("for loop 1 {e}");
    }
    // for loop 1 1
    // for loop 1 2
    // for loop 1 3
    // for loop 1 4
    // for loop 1 5

    println!("======================");
    //======================

    //Notice that it is exlusive for end point
    //rev() makes it in the reverse order
    for e in (1..5).rev() {
        println!("for loop 2 {e}");
    }
    // for loop 2 4
    // for loop 2 3
    // for loop 2 2
    // for loop 2 1
}
