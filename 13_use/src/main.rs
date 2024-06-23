mod module_outside {
    pub fn foo() {
        println!("function foo in module_outside.");
    }
    pub mod module_inside {
        pub fn foo() {
            println!("function foo in module_inside that is in module_outside.");
        }
    }
}

//Use is more typical to be targeting with module.
//If we making a shortcut to a function, it can hide the purpose of the function since module is unknown.

use module_outside::foo; //Not typical since it is a function name.

//Impossible since function name collides.
//use module_outside::module_inside::foo;

//alias, it renames a function.
use module_outside::module_inside::foo as mifoo;

use module_outside::module_inside; //Typical since it is a module name.

mod other_module {
    pub fn bar() {
        // Impossible.
        // use module_outside::foo; has been used outside of the module.
        // Therefore, it doesn't affects to this module.
        // foo();

        //It works.
        super::foo();
    }
    //Alias can be public either.
    //It can be used outside of the scope in this case.
    pub use super::module_outside::module_inside as mi;
}

fn bar() {
    println!("function e");
    foo(); //It doesn't show where this function came from.
    module_inside::foo();
}

// To use outter module, please change Cargo.toml
// Add module name and version below [dependencies].
// For example, this project added random module.
use rand::Rng;

mod many_module_a {
    pub mod module_a {
        pub fn foo() {
            println!("foo in module_a");
        }
    }
    pub mod module_b {
        pub fn foo() {
            println!("foo in module_b");
        }
    }
}

// Can make two shortcuts at once.
// It can also include itself by using self.
// use many_module_a::{self, module_a, module_b};
use many_module_a::{module_a, module_b};

mod many_module_b {
    pub mod module_c {
        pub fn foo() {
            println!("foo in module_c");
        }
    }
    pub mod module_d {
        pub fn foo() {
            println!("foo in module_d");
        }
    }
    pub mod module_e {
        pub fn foo() {
            println!("foo in module_e");
        }
    }
}

//Can make shortcuts for every thing inside of it at once.
//It can lead a problem that reduce redability.
use many_module_b::*;

fn main() {
    bar();
    // function e
    // function foo in module_outside.
    // function foo in module_inside that is in module_outside.

    other_module::bar();
    //function foo in module_outside.

    mifoo();
    // function foo in module_inside that is in module_outside.

    other_module::mi::foo();
    // function foo in module_inside that is in module_outside.

    let randV = rand::thread_rng().gen_range(1..10);
    println!("Random value is {randV}");
    //This is one of possible output from function above.
    //Random value is 4

    module_a::foo();
    // foo in module_a

    module_b::foo();
    // foo in module_b

    module_c::foo();
    // foo in module_c

    module_d::foo();
    // foo in module_d

    module_e::foo();
    // foo in module_e
}
