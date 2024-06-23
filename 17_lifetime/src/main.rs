// The function below can not be defined.
// Function below can not decide the lifetime.
// (Whether a will live or b will live)

// fn longer_between(a : &String, b : &String) -> &String{
//     if a.len() > b.len(){
//         return a;
//     }
//     b
// }

// 'a denotes lifetime.
// To denote lifetime, it must starts with quotation mark (').
// This type is not an actual type, therefore it must be templated.

// There are two types of lifetimes.
// Incoming and out going lifetime.
// In the following case, all life times works in the same way.
// Therefore, it denoted by 'a.
// Be caution where lifetime type is denoted.
fn longer_between<'a>(a: &'a String, b: &'a String) -> &'a String {
    if a.len() > b.len() {
        return a;
    }
    b
}

// Life time can be used more than one.
fn slice_two_string<'a, 'b>(a: &'a str, b: &'b str) -> (&'a str, &'b str) {
    (&a[0..2], &b[2..5])
}

// Lifetime can be used with templates and traits.
// Function below compare two references that can be compared.
// It return bigger one.
fn find_big_ref<'a, T: std::cmp::PartialOrd>(a: &'a T, b: &'a T) -> &'a T {
    if a > b {
        return a;
    }
    b
}

// struct also can have lifetime template.
// It used when some variables are references.
struct User<'a> {
    name: &'a str,
}

fn main() {
    let s1 = String::from("Hello");
    let s2 = String::from("World!");
    let s3 = longer_between(&s1, &s2);
    println!("{0}", s3);
    //World!

    let mut p0;
    let mut p1;
    let a = String::from("Hello");
    {
        let b = String::from("World");
        let p = slice_two_string(&a, &b);
        p0 = p.0;
        p1 = p.1;
        println!("{0}{1}", p.0, p.1);
        //Herld
    }
    println!("p0 : {0}", p0);
    // p0 : He

    // Impossible, borrow checker will check it originates from slice_two_string.
    // Its life time corresponds with b therefore, it can not be used in here.
    // println!("p1 : {0}", p1);

    let a = 10;
    let b = 20;
    let c = &a;
    let d = &b;
    let e = find_big_ref(c, d);
    println!("{0} is bigger between {1} and {2}.", e, c, d);
    // 20 is bigger between 10 and 20.

    let james = User { name: "James" };

    println!("This user's name is {0}.", james.name);
    // This user's name is James.
}
