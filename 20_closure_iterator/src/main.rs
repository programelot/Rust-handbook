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

    let s1 = vec![
        String::from("James"),
        String::from("John"),
        String::from("Lee"),
    ];

    // Generates iterator.
    // Rust iterator is lazy.
    let mut s1_iter = s1.iter();
    // vector can be read with iterator.
    loop {
        let n = s1_iter.next();
        if n.is_none() {
            break;
        }
        // Caution that this is immutable reference.
        let v = n.unwrap();
        print!("{v}, ");
    }
    println!("");
    // James, John, Lee,

    // To make iterator takes ownership, use into_iter().
    let mut s2 = Vec::new();
    let mut s1_iter = s1.into_iter();
    loop {
        let n = s1_iter.next();
        if n.is_none() {
            break;
        }
        // Caution that this is immutable reference.
        let v = n.unwrap();
        s2.push(v);
    }

    // Impossible since iterator takes ownership of s1.
    // let s3 = s2;

    for e in &s2 {
        print!("{e}, ");
    }
    println!("");
    // James, John, Lee,

    // To change value with iterator, use iter_mut()

    let mut s2_iter = s2.iter_mut();
    loop {
        let n = s2_iter.next();
        if n.is_none() {
            break;
        }
        // Caution that this is immutable reference.
        let v = n.unwrap();
        v.push_str(" Jackson");
    }

    for e in &s2 {
        print!("{e}, ");
    }
    println!("");
    // James Jackson, John Jackson, Lee Jackson,

    let v1 = vec![1, 2, 3];

    // Iterator has a map function that generate other iterator.
    // It maps each element of the iterator to a new element by passing it to arguments.
    
    // Notice that function below doesn't work since this iterator is lazy.
    // v1.iter().map(|x| x + 1);

    // Collect function iterates iterator and collects every values inside of it.
    // Notice that it takes x as immutable.
    let mut v2: Vec<_> = v1.iter().map(|x| x + 1).collect();

    for e in &v1 {
        print!("{e}, ");
    }println!("");
    // 1, 2, 3,

    for e in &v2 {
        print!("{e}, ");
    }println!("");
    // 2, 3, 4,

    
    let v3 = vec![String::from("Ada"), String::from("James"), String::from("John"), String::from("Lee")];

    for e in &v3 {
        print!("{e}, ");
    }println!("");
    // Ada, James, John, Lee,

    // filter() filters elements.
    // It requires into_filter;
    let mut v3_iter = v3.iter();

    //Notice that v4 makes a reference of v3.
    let v4: Vec<_> = v3_iter.filter(|x| x.len() >= 4).collect();

    for e in &v4 {
        print!("{e}, ");
    }println!("");
    // James, John,

    let v5 = vec![1,2,3,4,5];
    let v6 = vec![1.0, 2.0, 3.0];
    
    // Zip pairs iterator with target iteratable.
    // If it doesn't have enough element, length will be mached with the smaller one.

    let v7 : Vec<_> = v5.iter().zip(&v6).collect();

    for e in v7{
        print!("{:?} ", e);
    }println!("");
    // (1, 1.0) (2, 2.0) (3, 3.0)
}
