fn take_owner_i32(x: i32) {
    println!("{x}");
}

fn take_owner_string(x: String) {
    println!("{x}");
}

fn take_owner_and_return_string(x: String) -> String {
    println!("{x}");
    x
}

fn take_reference(x: &String) {
    //Impossible
    //x.push_str(" mut ref");

    println!("{x}");
}

fn take_mut_reference(x: &mut String) {
    x.push_str(" mut ref");
    println!("{x}");
}

//Rust program starts with main function
//Function starts with fn
//Function parameter exists in parenthesis.
//Block defined with {} brackets.
fn main() {
    let x = 1;
    take_owner_i32(x);
    // 1

    // Possible
    // Function takes ownership.
    // However, it has copy trait(i32).
    // Therefore, instead of take it, it copies the value.
    println!("{x}");
    // 1

    let x = String::from("2");
    take_owner_string(x);
    // 2

    // Impossible
    // Function already took ownership.
    // println!("{x}");

    let x = String::from("3");
    let v = take_owner_and_return_string(x);
    // 3
    println!("{v}");
    // 3

    // Reference doesn't take owner ship.
    // It borrows variables.
    let s = String::from("Hello");
    take_reference(&s);
    // Hello
    println!("{s}");
    // Hello

    let mut s = String::from("Hello");
    take_mut_reference(&mut s);
    // Hello mut ref
    take_reference(&s);
    // Hello mut ref
    println!("{s}");
    // Hello mut ref

    let mut s = String::from("Hello");
    // Reference can be a lot.
    let s1 = &s;
    let s2 = &s;
    println!("{s1} {s2}");
    // Hello Hello

    let mut s = String::from("Hello");
    // Mutable reference need to be only one.
    let mut s1 = &mut s;
    // Impossible (No more than two mutable reference)
    // let mut s2 = &mut s;
    println!("{s1}");
    // Hello

    let mut s = String::from("Hello");
    let s1 = &s;
    // Impossible
    // Mutable reference can not exists with immutable reference.
    // let s2 = &mut s;
    println!("{s1}");
    // Hello

    let mut s = String::from("Hello");
    let s1 = &s;
    println!("{s1}");
    // Hello
    // Mutable reference can exists after immutable reference used for all purpose already.
    let s2 = &mut s;
    println!("{s2}");

    // Summary
    // There must be one mutable reference or multiple immutable reference.

    let mut v: String = String::from("s안녕");
    let i = 1;
    let j = 4;
    let s = &v[i..j];
    // Arrays can be sliced by index.
    // If string is unicode, it must be sliced properly.
    println!("{s}");
    // 안

    let v = [1, 2, 3, 4, 5];
    let x: &[i32] = &v[2..4]; // &[T] is a slice type of T.
    for i in 0..2 {
        println!("{0}", x[i]);
    }
    // 3
    // 4

    let mut x = vec![
        String::from("Hello"),
        String::from("World"),
        String::from("This"),
        String::from("Cool"),
        String::from("World"),
    ]; // &[T] is a slice type of T.
    let sl: Vec<_> = x.iter().step_by(2).collect();
    for e in sl {
        println!("{0}", e);
    }
}
