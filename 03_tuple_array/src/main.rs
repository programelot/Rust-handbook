//Rust program starts with main function
//Function starts with fn
//Function parameter exists in parenthesis.
//Block defined with {} brackets.
fn main() {
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
    let r = compute_add_and_sub(10, 20);
    let r0 = r.0;
    let r1 = r.1;
    println!("r0 : {r0}, r1: {r1}");
    //r0 : 30, r1: -10
}

//function name uses under cases
fn compute_add_and_sub(a: i32, b: i32) -> (i32, i32) {
    (a + b, a - b)
}
