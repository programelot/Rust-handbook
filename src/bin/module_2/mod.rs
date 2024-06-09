mod module_21; //has private foo function
mod module_22; //has public foo function
pub mod module_23; //has private foo function
pub mod module_24; //has public foo function

pub fn foo() {
    println!("module_2's foo function.");
    // Impossible since it is private function
    // module_21::foo();
    // module_23::foo();

    
    // Visibility is based on current location.
    // crate::module_2::module_22::foo();
    // module_22's foo Function
    // It works.

    module_22::foo();
    // module_22's foo Function
    module_24::foo();
    // module_24's foo function
}

fn bar(){
    println!("module2's bar");
}
