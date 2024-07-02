// ! can be denote that function never returns a thing.
// It can be unlimited loop or panic or sort of.
// It can denote compiler that this type doesn't need to be considered as a return type at all.
fn foo() -> ! {
    loop {}
    panic!("Do")
}

fn main() {
    let v = Some("Do");

    // Notice that match requires the same type for return type in each case.
    // However, foo has ! return type which means it never return a value.
    // Therefore, compiler just accepts it to be a part of match now.
    // It allows compiler to add panic function in any case of match since it ignore the type match.
    match v {
        Some(_) => println!("It is some value."),
        None => foo(),
    }
}
