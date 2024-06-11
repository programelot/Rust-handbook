use std::collections::HashMap;

fn main() {
    //Type must be annotied here since there is no data inside of it yet.
    //However in most of cases, it can be omitted since type can be guessed.
    let v: Vec<i32> = Vec::new();

    //This is the case when type can be guessed.
    let mut v = Vec::new();
    v.push(1);
    v.push(2);

    for e in &v {
        print!("{e} ");
    }
    print!("\n");
    // 1 2

    //vec! macro generates vector.
    let mut v = vec![1, 2, 3, 4, 5];

    //Vector can be read by two methods.
    //Fist is an indexing.
    let mut i = 0;
    while i < v.len() {
        let g = &v[i];
        print!("{0} ", g);
        i += 1;
    }
    print!("\n");
    // 1 2 3 4 5

    //It will crash the program since it is out of the range.
    //print!("{0} ", v[5]);

    //The second method is get function.
    //It returns an option value of vector.
    let mut i = 0;
    loop {
        let g = v.get(i);
        match g {
            Some(v) => {
                print!("{0} ", v);
                i += 1;
            }
            None => break,
        }
    }
    print!("\n");
    // 1 2 3 4 5

    let mut v = vec![1, 2, 3, 4, 5];
    for e in &mut v {
        *e += 10;
    }

    for e in &v {
        println!("{e}");
    }

    //Many ways to generate strings
    let mut s = String::new(); //empty string
    s.push_str("Hello world 1");
    println! {"The string is \"{0}\"", s};
    // The string is "Hello world 1"

    //String::from and to_string() works the same.
    //It can be used for any of two but decides by considering readability.
    let s = String::from("Hello world 2");
    println! {"The string is \"{0}\"", s};
    // The string is "Hello world 2"

    let s = "Hello world 3".to_string();
    println! {"The string is \"{0}\"", s};
    // The string is "Hello world 3"

    // ===========================================================
    //push_str doesn't take ownership.
    let mut s1 = String::from("Hello ");
    let s2 = "world ";
    s1.push_str(s2); //push_str pushes str at the end of the string.
    s1.push('4'); //push add one byte at the end of the string
    println! {"The string is \"{0}\"", s1};
    // The string is "Hello world 4"
    println!("Still can use string \"{0}\"", s2);
    // Still can use string "world "

    // ===========================================================
    let s1 = String::from("Hello ");
    let s2 = String::from("world 5");
    let s3 = s1 + &s2; //S1 will lost its ownership but S2 will not.
    println! {"The string is \"{0}\"", s3};
    // The string is "Hello world 5"

    //Impossible since s3 takes s1's ownership.
    // println!("Still can use string \"{0}\"", s1);

    println!("Still can use string \"{0}\"", s2);
    // Still can use string "world 5"

    // ===========================================================
    let s1 = String::from("Hello");
    let s2 = String::from("world 6");
    let s3 = format!("{0} {1}", s1, s2); //format! doesn't take ownership.

    println! {"The string is \"{0}\"", s3};
    // The string is "Hello world 6"

    println!("Still can use string \"{0}\"", s1);
    // Still can use string "Hello"

    println!("Still can use string \"{0}\"", s2);
    // Still can use string "world 6"

    // ===========================================================

    //String can be read by charcaters or bytes.
    //Korean character uses three bytes and englush and space uses only a byte.
    //Following are the ways to use it.

    let s = "안녕 James";

    for c in s.chars() {
        print!("\"{c}\" ");
    }
    print!("\n");
    // "안" "녕" " " "J" "a" "m" "e" "s"

    for b in s.bytes() {
        print!("\"{b}\" ");
    }
    print!("\n");
    // "236" "149" "136" "235" "133" "149" "32" "74" "97" "109" "101" "115"
    //It works in follows.
    // 안 ["236" "149" "136"]
    // 녕 ["235" "133" "149"]
    //   ["32"]
    // james [ "74" "97" "109" "101" "115"]

    let mut hash_map = HashMap::new();

    let james = String::from("James");
    let john = String::from("John");
    //hash_map takes ownership of key.
    hash_map.insert(james, john);

    // key and value must ne regenerated since it lost ownership already.
    let james = String::from("James");
    let tom = String::from("Tom");
    //hash_map overwrites the value if it uses the same key.
    hash_map.insert(james, tom);

    //entry can be used when it wants to be added only if there is no element with the same key.
    let james = String::from("James");
    let john = String::from("John");
    let v = hash_map.entry(james).or_insert(john);
    println!("James value is {v}");
    //James value is Tom

    let jack = String::from("Jack");
    let john = String::from("John");
    let v = hash_map.entry(jack).or_insert(john);
    println!("Jack value is {v}");
    //Jack value is John

    let james = String::from("James");

    let v = hash_map.get(&james);
    match v {
        Some(x) => println!("{x}"),
        None => println!("Not found"),
    }
    //Tom

    for (key, value) in &hash_map {
        println!("{key}: {value}");
    }
    // James: Tom
    // Jack: John
}
