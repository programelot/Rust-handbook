struct Some_readable {
    data: i8,
}

impl Some_readable {
    fn read_as(&self) -> i8 {
        self.data * 4
    }
}

// associated type makes a type that can be defined later on the implementation side.
// type Target means an associated type in here.
trait Read_associated_type {
    type Target;
    fn read_ass(&self) -> Self::Target;
}

// When using it, the type must be defined like below.
impl Read_associated_type for Some_readable {
    type Target = i8;
    fn read_ass(&self) -> Self::Target {
        self.data * 5
    }
}

// Associated type may work similar with generic.
trait Read_generic<T> {
    fn read_as(&self) -> T;
}

// However, here is the difference that generic trait makes it can be defined in two seperate types.
// Below is the defined Read_generic trait with i8 type for Some_readable.
impl Read_generic<i8> for Some_readable {
    fn read_as(&self) -> i8 {
        self.data
    }
}

// Below is the defined Read_generic trait with i32! not i8 type for Some_readable.
impl Read_generic<i32> for Some_readable {
    fn read_as(&self) -> i32 {
        self.data as i32 * 2
    }
}

trait Read_as_itis {
    fn read_as(&self) -> i8;
}

impl Read_as_itis for Some_readable {
    fn read_as(&self) -> i8 {
        self.data * 3
    }
}

struct Not_method_struct;

trait Not_method_trait {
    fn not_method() -> String;
}

impl Not_method_trait for Not_method_struct {
    fn not_method() -> String {
        String::from("not_method function in Not_method_trait for Not_method_struct")
    }
}

impl Not_method_struct {
    fn not_method() -> String {
        String::from("not_method function in Not_method_struct")
    }
}

fn main() {
    let v = Some_readable { data: 3 };
    // If there are two generic trait for a struct, it can be called as follow.
    println!("read_gen i8 {:#?}", Read_generic::<i8>::read_as(&v));
    // read_gen i8 3

    println!("read_gen i32 {:#?}", Read_generic::<i32>::read_as(&v));
    // read_gen i32 6

    // If it doesn't have a generic, it can be called by using following method.
    println!("reaad_as_it_is {:#?}", Read_as_itis::read_as(&v));
    // reaad_as_it_is 9

    // If struct have a method that has the same name with function of trait, it can be called as follows.
    // In other word, it calls own fundtion as a default.
    println!("read_as (native) {:#?}", v.read_as());
    // read_as (native) 12

    // If programmer want to make it clear, following also do the same thing.
    println!("read_as (native) {:#?}", Some_readable::read_as(&v));
    // read_as (native) 12

    println!("read_ass {:#?}", v.read_ass());
    // read_ass 15

    println!(
        "Not_method_struct::not_method() is {0}",
        Not_method_struct::not_method()
    );
    // Not_method_struct::not_method() is not_method function in Not_method_struct

    // Function below is not working, trait can not call a method by its own.
    // Notice that it doesn't work even it has a default method implementation.
    // println!("Not_method_trait::not_method() is {0}", Not_method_trait::not_method());

    // Therefore, following must be used.
    println!(
        "Not_method_trait::not_method() is {0}",
        <Not_method_struct as Not_method_trait>::not_method()
    );
    // Not_method_trait::not_method() is not_method function in Not_method_trait for Not_method_struct
}
