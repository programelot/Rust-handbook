fn main() {
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
    }
    println!("");
    // 1, 2, 3,

    for e in &v2 {
        print!("{e}, ");
    }
    println!("");
    // 2, 3, 4,

    let v3 = vec![
        String::from("Ada"),
        String::from("James"),
        String::from("John"),
        String::from("Lee"),
    ];

    for e in &v3 {
        print!("{e}, ");
    }
    println!("");
    // Ada, James, John, Lee,

    // filter() filters elements.
    // It requires into_filter;
    let mut v3_iter = v3.iter();

    //Notice that v4 makes a reference of v3.
    let v4: Vec<_> = v3_iter.filter(|x| x.len() >= 4).collect();

    for e in &v4 {
        print!("{e}, ");
    }
    println!("");
    // James, John,

    let v5 = vec![1, 2, 3, 4, 5];
    let v6 = vec![1.0, 2.0, 3.0];

    // Zip pairs iterator with target iteratable.
    // If it doesn't have enough element, length will be mached with the smaller one.

    let v7: Vec<_> = v5.iter().zip(&v6).collect();

    for e in v7 {
        print!("{:?} ", e);
    }
    println!("");
    // (1, 1.0) (2, 2.0) (3, 3.0)
}
