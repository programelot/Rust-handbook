// Function also can be denoted as unsafe.
unsafe fn dangerous() {
    let mut num = 5;
    // It is not medatory to denote unsafe again in the unsafe function.
    let r1 = &num as *const i32;
}

use std::slice;
// Function doesn't need to be denoted as unsafe even if it has unsafe part.
// Below is an implementation of split mutable slice to two seperate mutable slices.
fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = values.len();
    // as_mut_ptr() gests values as mutable raw pointer.
    let ptr = values.as_mut_ptr();

    assert!(mid <= len);

    // Below is safe since it passed mid <= len already.
    unsafe {
        (
            // It generates slice from ptr to ptr + mid
            slice::from_raw_parts_mut(ptr, mid),
            // It generates slice from ptr + mid to ptr + mid + len - mid
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}

// Rust can use function from other langauge.
// extern is used in this case.
// Code below means that it uses abs function from c language.
// programmer must define a function definition in rust for it.
extern "C" {
    fn abs(a: i32) -> i32;
}

// Rust can also export a function made with rust.
// It makes that function can be used in C in this case.
// no_mangle means that compiler doesn't mangle the function name.
// Unless it makes hard to progammer finds a function name which is mangled.
// Notice that exporting doesn't require unsafe.
#[no_mangle]
pub extern "C" fn call_from_c() {
    println!("Just called a Rust function from C!");
}

// Rust doesn't have a global variable.
// Instead of it, rust have a static variable.
// In general static variable uses all uppercase with underbarlike below (SCREAMING_SNAKE_CASE).
static HELLO_WORLD: &str = "Hello, world!";

// static variable can be mutable.
static mut COUNTER: u32 = 0;

// However, chaning mutable static variable's value is unsafe.
fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

// Unsafe trait also can be defined.
unsafe trait Foo {}

// methods will be defined as follows.
unsafe impl Foo for i32 {}

fn main() {
    let mut num = 5;

    // raw pointer is a type that works in the similar way with the pointer in C/C++.
    // It can pointing some place.
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    // It is not required to be speicific address.
    // Code below is highly likely not working but it still programmable.
    let address = 0x012345usize;
    let r = address as *const i32;

    // Raw pointer can be generated in the safe rust yet.
    // However, dereferencing it requires unsafe to wrap it.
    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);

        // Unsafe function must be called in the unsafe block.
        dangerous();
    }

    let mut v = vec![1, 2, 3, 4, 5, 6];

    let r = &mut v[..];

    let (a, b) = split_at_mut(r, 3);

    for e in a {
        print!("{e} ");
    }
    println!("");
    // 1 2 3

    for e in b {
        print!("{e} ");
    }
    println!("");
    // 4 5 6

    // Calling extern function is not safe.
    // Other language may can violate rule of the rust.
    // Therefore, it must be declared with unsafe.
    unsafe {
        println!("|-3| = {0}", abs(-3));
    }
    add_to_count(1);

    // Reading static variable is also unsafe.
    unsafe {
        println!("{COUNTER}");
    }
}
