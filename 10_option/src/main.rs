fn read(val: &Option<i32>) {
    match val {
        Some(v) => println!("{0}", v),
        None => println!("None"),
    }
}

//Rust program starts with main function
//Function starts with fn
//Function parameter exists in parenthesis.
//Block defined with {} brackets.
fn main() {
    //Option is an enum that can be used to represent nullable variable.
    let some_number1: Option<i32> = Some(5);
    let absent_number: Option<i32> = None;

    let some_char: Option<char> = Some('A');
    let absent_char: Option<char> = None;

    //It prevents following cases.
    //Options can not be added.
    let some_number1: Option<i32> = Some(5);
    let some_number2: Option<i32> = Some(8);
    //let some_number3 = some_number1 + some_number2;

    //Options can not be added with existing number neither.
    let some_number1: Option<i32> = Some(5);
    let some_number2: i32 = 8;
    //let some_number3 = some_number1 + some_number2;

    let some_number1: Option<i32> = Some(5);
    let absent_number: Option<i32> = None;

    read(&some_number1);
    //5
    read(&absent_number);
    //None

    // Instead of taking ownership of s, v can just take the value of option and change it to None.
    let mut s = Some(String::from("Text"));
    let mut v = s.take();
    println!("s : {:#?}", s);
    // s : None
    println!("v : {:#?}", v);
    // v : Some(
    //     "Text",
    // )

    let a = Some(String::from("Hello"));
    // a is Option<String>
    // as_ref reutnrs Option<&String> in this case.
    let b = a.as_ref();
    println!("a : {:#?}", a);
    // a : Some(
    //     "Hello",
    // )

    // Notice that b has been dereferenced in here.
    println!("b : {:#?}", b);
    // b : Some(
    //     "Hello",
    // )
}
