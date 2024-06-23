// To create a binary crate, it requires src/bin folder or main.rs at src.
// It compiles every rs file as binary file in src/bin folder or it is "src/main.rs"
//
// To create a library crate, it requires "src/lib.rs".
// It can make many modules by following ways.

// Find module at the same directory with the name of "module_1.rs".
mod module_1;

// Find module at the folder "module_2" in the same directory with the name of "mod.rs".
mod module_2;

// Inline module
mod module_3 {
    // Make function accessible from outside of the module.
    pub fn foo() {
        println!("module_3's foo function.");
    }
}

// Make a shortcut for foo function in module_3
// It makes foo function in the same scope as foo of module_3.
use module_3::foo;

fn main() {
    module_1::foo();
    // module_1's foo function.

    module_3::foo();
    // module_3's foo function.

    // It will call module3's foo function.
    foo();
    // module_3's foo function.

    // It will call sub module function either.
    module_2::foo();
    // module_2's foo function.
    // module_22's foo function
    // module_24's foo function

    // Impossible, module_21, module_22 are private.
    // module_2::module_21::foo();
    // module_2::module_22::foo();

    // Impossible, function foo is private
    // module_2::module_23::foo();
    module_2::module_24::foo();
    // module_24's foo function

    // Module usages above uses relative location.
    // It based on the current module's location.
    // Below is the module usage by absolute location.
    // Root is the crate.
    // Visibility is based on current location even when it uses absolute location.
    // Please check module_2/mod.rs
    crate::module_2::module_24::foo();
    // module_24's foo function

    // super finds parents.

    // Impossible, super can only exists at the start of the path.
    // module_2::super::module_2::module_24::bar();
 
    // Function below calls module2's bar in module_24 using super.
    crate::module_2::module_24::bar();
    // module2's bar
}
