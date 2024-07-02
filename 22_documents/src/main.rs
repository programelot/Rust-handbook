//! # Crate name
//!
//! This is an area for crate explanation
//! It will be locate at the root of the crate.

// Rust has a functionality to generate documents for programmers.
// It can be generated with cargo doc
// It also has functionality for both build and open
// cargo doc --open
// To add document for function it need to use /// instead of //
// It follows mark down grammar.

// It starts with description.
// Then, it usually includes examples.
// It has a strong good point that rust will test example in the code automatically.
// It must be in lib.rs to be so.
// Panic, Error, Safety can be the other candidate.

/// Function description
///
/// # Examples
///
/// ```
/// let arg = 5;
/// let answer = 21_documents::function_to_be_explained(arg);
///
/// assert_eq!(6, answer);
/// ```
/// # Panic
/// It doesn't panic.
/// # Errors
// It doesn't have error.
/// # Safety
/// It is safe.
pub fn function_to_be_explained(x: i32) -> i32 {
    x + 1
}

// Public shortcut also recorded in the documents.
// It will be displayed on the Re-exports.
// It helps programmer to program easiler and finds a good point to look at.
pub use a::b;

pub mod a {
    pub mod b {
        pub fn foo() {
            println!("foo");
        }
    }
}

fn main() {}
