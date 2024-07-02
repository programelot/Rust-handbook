// fn is a function pointer type that denotes a function.
// it denoted in a following way.
// f1 has a &String as an argument.
// f2 has two i32 arguments and return String.
fn call_function_with_arg(f1: fn(&String), f2: fn(i32, i32) -> String, a: i32, b: i32) {
    f1(&f2(a, b));
}

fn i32_pair_to_string(a: i32, b: i32) -> String {
    format!("a : {a}, b : {b}")
}

fn print_string(s: &String) {
    println!("String was \"{s}\"");
}

// function pointer has a Fn, FnOnce, FnMut trait so it can be used with it.
fn call_closure<T: Fn(&String)>(f: T, s: &String) {
    f(s);
}

fn generate_color_from_i32(f: fn(i32) -> Color, a: i32) -> Color {
    Color::Red(a)
}

// Notice that enum's each name is a initializer function.
// Therefore it can be passed as a function pointer.
#[derive(Debug)]
enum Color {
    Red(i32),
    Blue,
}

// function pointer may can be used for return
// However it can only return what it got since there are no way to generate function in side of the function again.
fn returns_function_pointer(f: fn()) -> fn() {
    f
}

// Returning a closure is a bit different.
// Since Fn is a trait, it can not be sized.
// There are two possible ways in this case.

// Return Fn trait by using impl Fn.
// However, it requires a programmer to not have any branch.

// This is possible.
// fn returns_closure(a : bool) -> impl Fn(i32) -> i32 {
//     |x| x + 1
// }

// This is impossible since there is a branch.
// fn returns_closure(a : i32) -> impl Fn(i32) -> i32 {
//     if a > 0 {
//         return |x| x + 1;
//     }
//     else{
//         return |x| x + 2;
//     }
// }


// // Return closure that wrapped with Box.
fn returns_closure(a: i32) -> Box<dyn Fn(i32) -> i32> {
    if a > 0 {
        return Box::new(|x| x + 1);
    }
    else{
        return Box::new(|x| x + 2);
    }
}

// Both case works.

fn main() {
    // Closure is a type of functor.
    // it can have arguments and return types (or can be omitted)
    // There are many types of closures.
    let hello = String::from("Hello!");

    // This is FnOnce type of closure.
    // It can be called only once.
    // Notice that it can't be called twice since it takes ownership of hello.
    let c1 = || {
        //Doesn't get and return any type
        println!("Hello in closure c1 {0}", hello);
        hello
    };
    let s = c1();
    // Hello in closure c1 Hello!

    // This is impossible since c1 takes ownership of hello.
    // let s = c1();

    println!("{s}");
    // Hello!

    // This is Fn type of closure that doesn't take ownership and change values.
    // It can be called multiple times.
    let c2 = |n: u32| -> u32 {
        //get one argument n as n32 type and return n32
        n + 1
    };

    let v = 2;
    let mut w = c2(v);
    w = c2(w);
    println!("{v} + 1 + 1 = {w}");
    // 2 + 1 = 3

    let c1 = |n: u32| -> u32 { n + 1 };
    //Type can be omitted if it can be gueesed.
    let c2 = |n| n + 1;
    println!("3 + 1 = {0}", c2(3u32));
    // 3 + 1 = 4

    // Impossible, closure is not a template.
    // It just guess type and make it proper.
    // println!("3 + 1 = {0}", c2(3u16));

    // it doesn't need to be binded with {}
    let c3 = |n| n + 1;
    println!("4 + 1 = {0}", c3(4));
    // 4 + 1 = 5

    let mut v = 10;
    println!("Before call closure : {v}");

    // This is FnMut type of closure.
    // If closure changes captured variable, it must be defined as mut.
    // It can be called a lot of time since it doesn't take ownership.
    let mut c = || {
        v += 1;
    };

    // Impossible since v has been borrowed by closure.
    // println!("Before call closure {v}");

    c();

    println!("After call closure : {v}");

    let (a, b) = (10, 20);
    call_function_with_arg(print_string, i32_pair_to_string, a, b);
    // String was "a : 10, b : 20"

    call_closure(
        |s: &String| {
            println!("Print function by closure : \"{s}\"");
        },
        &String::from("Call_closure"),
    );
    // Print function by closure : "Call_closure"

    call_closure(print_string, &String::from("Call_closure"));
    // String was "Call_closure"

    let v = generate_color_from_i32(Color::Red, 10);
    println!("{:?}", v);
    // Red(10)

    let c = returns_closure(2);
    let v = 3;
    println!("{0}", c(v));
    // 4
}
