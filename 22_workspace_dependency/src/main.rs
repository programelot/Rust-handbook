// To use outter module, it need to declared with use.
// It also need to denoted at Cargo.toml.
use dependent;

fn main() {
    let v = 1i32;
    
    // Call foo in dependent module
    let m = dependent::foo(v);

    println!("{m}");
    // 11
}
