//Rust program starts with main function
//Function starts with fn
//Function parameter exists in parenthesis.
//Block defined with {} brackets.
fn main() {
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
    let t: u16 = 1u16; //Type can be denoted at the end of the number.
    let t: u32 = 1;
    let t: u64 = 1;
    let t: u128 = 1;
    let t: usize = 1;

    //floating point numbers
    let t: f32 = 1.0;
    let t: f64 = 1.0;

    //character is 4 byte and supports unicode.
    let c: char = '한';

    println!("Unicode {c}");
    //Unicode 한
}
