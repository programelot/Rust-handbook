//Rust program starts with main function
//Function starts with fn
//Function parameter exists in parenthesis.
//Block defined with {} brackets.
fn main() {
    //function ends with exclamation mark is a macro.
    println!("Hello, world!");

    //Immutabale Variable defines with let
    let x = 10;
    //x = 20; //Impossible

    //Mutable variable defines with let mut
    let mut y = 20;
    y = 30;

    //constant need to be defined in compile time.
    //Similar with immutable variable
    //It requires type definition.
    //Constant is better to defined with capital letters
    const THIS_IS_ONE: i32 = 1;

    let spaces = "   ";
    //Variable name can be reused to define other type of variable.
    //It can use old variable.
    //Notice that variable type can not be changed
    let spaces = spaces.len();

    //There are following types

    //signed integers
    //Number(literals) can be represented as follows.
    let t: i8 = 10_0;
    //let t : i8 = 10_000; //Impossible since it is out of range
    let t: i16 = 0b_000_000; //binary
    let t: i32 = 0x_0ff_fea; //hexa
    let t: i64 = 0o_077_777; //octa
    let t: i128 = 0;
    let t: isize = 1;

    //unsigned integers

    //u8 type can represent byte
    let t: u8 = b'A'; //byte
    let t: u16 = 1;
    let t: u32 = 1;
    let t: u64 = 1;
    let t: u128 = 1;
    let t: usize = 1;

    //floating point numbers
    let t: f32 = 1.0;
    let t: f64 = 1.0;

    //Handle overflows
    let a: i8 = 2;
    let b: i8 = 126;

    let c: i8 = 1;
    let d: i8 = 126;

    //let c : i8 = a + b; //Impossible since it is overflow
    //It just overflows when it builds on release mode.
    //It gives panic when it build on debug mode.

    //Do overflow
    let e: i8 = a.wrapping_add(b);
    let f: i8 = c.wrapping_add(d);
    println!("e : {e}, f : {f}");
    //e : -128, f : 127

    //Return None for overflow.
    let e: Option<i8> = a.checked_add(b);
    let f: Option<i8> = c.checked_add(d);
    //If it is value : get value
    //      othersise : get 44
    let x: i8 = e.unwrap_or(44);
    let y: i8 = f.unwrap_or(44);

    let e_is_some = e.is_some();
    let f_is_some = f.is_some();
    println!("x : {x} {e_is_some}, y : {y} {f_is_some}");
    //x : 44 false, y : 127 true

    //Overflows but return with checker whether it overflows or not
    let e = a.overflowing_add(b);
    let f = c.overflowing_add(d);

    let e0 = e.0;
    let e1 = e.1;
    let f0 = f.0;
    let f1 = f.1;

    println!("e : {e0} {e1}, f : {f0} {f1}");
    //e : -128 true, f : 127 false

    //Limits value within maximum and minimum value
    let e = a.saturating_add(b);
    let f = c.saturating_add(d);
    println!("e : {e}, f : {f}");
    //e : 127, f : 127

    //character is 4 byte and supports unicode.
    let c: char = '한';

    println!("Unicode {c}");
    //Unicode 한

    //Tuple can be defined as follows.

    let c: (i8, i32) = (127, 128);
    let a = c.0;
    let b = c.1;
    println!("c.0 {a} c.1 {b}");
    //c.0 127 c.1 128

    //Tuple index can not be variable
    // let d = 0;
    // let e = c.d;
    // println!("c.0 {e}}");

    let c = [1, 2, 3, 4];
    let b = c[0];

    //Array index can be variable
    let d = 0;
    let e = c[d];
    println!("c[0] {b} c[d] where d = 0 {e}");
    //c[0] 1 c[d] where d = 0 1

    //let d = 5;
    //let e = c[d]; //Impossible index is out of range.

    //initialize with the same values
    let c: [i32; 5] = [3; 5];
    let c0 = c[0];
    println!("c0 {c0}");
    //c0 3
    let r = compute_add_and_sub(&10, &20);
    let r0 = r.0;
    let r1 = r.1;
    println!("r0 : {r0}, r1: {r1}");
    //r0 : 30, r1: -10

    //////////////////////////////////////////////////
    // Big Caution with good example
    // If something ends with semi colon (;), it is statement.
    // Otherwise it is expression.
    // It does return some value
    // if is an expression
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
        //It loops until meet break
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
    //It seems like two form's expression returns () which is an empty tuple.
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
    let v = [1,2,3,4,5];
    for e in v{
        println!("for loop 1 {e}");
    };
    // for loop 1 1
    // for loop 1 2
    // for loop 1 3
    // for loop 1 4
    // for loop 1 5

    println!("======================");
    //======================

    //Notice that it is exlusive for end point
    //rev() makes it in the reverse order
    for e in (1..5).rev(){
        println!("for loop 2 {e}");
    };
    // for loop 2 4
    // for loop 2 3
    // for loop 2 2
    // for loop 2 1
    
}

//function name uses under cases
fn compute_add_and_sub(a: &i32, b: &i32) -> (i32, i32) {
    (a + b, a - b)
}
