//Rust program starts with main function
//Function starts with fn
//Function parameter exists in parenthesis.
//Block defined with {} brackets.
fn main() {
    let a: i8 = 10;
    let b: i8 = 3;
    let e: i8 = a % b;
    println!("{a} % {b} = {e}");
    // 10 % 3 = 1
    
    let a: i8 = -10;
    let b: i8 = 3;
    let e: i8 = a % b;
    println!("{a} % {b} = {e}");
    // -10 % 3 = -1

    let a: i8 = 10;
    let b: i8 = 110;
    let e: i8 = a + b;
    println!("{a} + {b} = {e}");
    // 10 + 110 = 120

    let a: i8 = 100;
    let b: i8 = 120;
    let e: i8 = a - b;
    println!("{a} - {b} = {e}");
    // 100 - 120 = -20

    let a: i8 = 125;
    let b: i8 = 10;
    let e: i8 = a / b;
    println!("{a} / {b} = {e}");
    // 125 / 10 = 12
    
    let a: f32 = 4.5;
    let b: f32 = 1.8;
    let e: f32 = a / b;
    println!("{a} / {b} = {e}");
    // 4.5 / 1.8 = 2.5

    let a: i8 = 5;
    let b: i8 = 6;
    let e: i8 = a * b;
    println!("{a} * {b} = {e}");
    // 5 * 6 = 30

    //Handle overflows

    let a: i8 = 2;
    let b: i8 = 126;
    //let c : i8 = a + b; //Impossible since it is overflow
    //It just overflows when it builds on release mode.
    //It gives panic when it build on debug mode.

    let a: i8 = 2;
    let b: i8 = 126;
    let c: i8 = 1;
    let d: i8 = 126;

    let e: i8 = a.wrapping_add(b); //Do overflow
    let f: i8 = c.wrapping_add(d);
    println!("e : {e}, f : {f}");
    //e : -128, f : 127

    let e: Option<i8> = a.checked_add(b); //Return None for overflow.
    let f: Option<i8> = c.checked_add(d);
    //If it is value : get value
    //      othersise : get 44
    let x: i8 = e.unwrap_or(44);
    let y: i8 = f.unwrap_or(44);

    let e_is_some = e.is_some();
    let f_is_some = f.is_some();
    println!("x : {x} {e_is_some}, y : {y} {f_is_some}");
    //x : 44 false, y : 127 true

    let e = a.overflowing_add(b); //Overflows but return with checker whether it overflows or not
    let f = c.overflowing_add(d);

    let e0 = e.0;
    let e1 = e.1;
    let f0 = f.0;
    let f1 = f.1;

    println!("e : {e0} {e1}, f : {f0} {f1}");
    //e : -128 true, f : 127 false

    let e = a.saturating_add(b);  //Limits value within maximum and minimum value
    let f = c.saturating_add(d);
    println!("e : {e}, f : {f}");
    //e : 127, f : 127
}
