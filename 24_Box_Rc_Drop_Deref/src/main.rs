// One possible implementation of list.
// However, it is not possible since rust requires exact type size.
// Size below has an infinite size.
// size of List<T> = size of T + size of List<T>
// enum List<T> {
//     Cons(T, List<T>),
//     Nil,
// }

// This is possible since Box has known size (pointer, usize)
enum List_Box<T> {
    Cons(T, Box<List_Box<T>>),
    Nil,
}

use std::rc::Rc;

// Reference count doesn't take ownership.
// This list below didn't used template for sake of easy.
enum List_Rc {
    Cons(i32, Rc<List_Rc>),
    Nil,
}

fn print_list_rc(l: &List_Rc) {
    print!("(");
    print_list_rc_recursive(l);
    println!(")");
}
fn print_list_rc_recursive(l: &List_Rc) {
    match l {
        Cons(v, next) => {
            print!("{v},");
            // reference coutner can be known from function.
            // Rc::strong_count returns how many references exists.
            print!("[{0}]", Rc::strong_count(&next));
            print_list_rc_recursive(next);
        }
        Nil => {
            print!("Nil");
        }
    };
}

// Shortcut made for make it shorter on the code.
// It can be compared with List_Box
use crate::List_Rc::{Cons, Nil};

// Own box struct implementation
// templated tuple struct
struct MyBox<T>(T);

// constructor
impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

// use Deref trait
use std::ops::{Deref, DerefMut};

// Deref trait makes that struct can be dereferenced.
impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

struct Student {
    pub name: String,
    pub age: i32,
}

// It has a dereference function from student to i32.
impl Deref for Student {
    type Target = i32;
    fn deref(&self) -> &Self::Target {
        &self.age
    }
}

// This function requires i32 as an argument.
fn print_age(age: &i32) {
    println!("age is {0}", age);
}

//Drop trait have a function that will be called when it dies.
impl Drop for Student {
    fn drop(&mut self) {
        // Can customize drop function for this.
        println!("Dropped student");
    }
}

fn main() {
    // Box wraps 10i32 and save it on the heap memory.
    // Box has deref trait and drop trait.
    // Therefore, it removes memory space when it escape from own scope.
    let i32_box = Box::new(10i32);

    // It also works.
    // let i32_box = Box::new(Box::new(Box::new(10i32)));

    // It can be printed since it has deref trait.
    println!("{i32_box}");
    // 10

    let s = String::from("String");
    let string_box = Box::new(s);

    // Impossible since string_box takes ownership of s.
    // println!("{s}");

    // Unfortunately, format below doesn't work since it need to be Box but not List_Box.
    // let l = List_Box::Cons(1i32, List_Box::Cons(2i32, List_Box::Cons(3i32, List_Box::Nil)));

    let a = List_Box::Cons(
        2i32,
        Box::new(List_Box::Cons(3i32, Box::new(List_Box::Nil))),
    );

    let b = List_Box::Cons(1i32, Box::new(a));

    // Impossible since b takes ownership of a.
    // let c = List_Box::Cons(1i32, Box::new(a));

    let a = List_Rc::Cons(2i32, Rc::new(List_Rc::Cons(3i32, Rc::new(List_Rc::Nil))));

    let a_rc = Rc::new(a);
    let b = List_Rc::Cons(1i32, Rc::clone(&a_rc));
    print_list_rc(&b);
    // (1,[2]2,[1]3,[1]Nil)

    // Possible since reference counter can handle multiple ownership.
    // When it uses Rc::clone, it doesn't take ownership.
    // It increases reference counter and get a control of it.
    let c = List_Rc::Cons(1i32, Rc::clone(&a_rc));

    //All reference counter can be read.
    print_list_rc(&b);
    // (1,[3]2,[1]3,[1])
    // Notice that [2] changed to [3] since c increased reference counter.

    print_list_rc(&c);
    // (1,[3]2,[1]3,[1])

    let mut c: List_Rc;
    {
        let a = List_Rc::Cons(2i32, Rc::new(List_Rc::Cons(3i32, Rc::new(List_Rc::Nil))));
        let a_rc = Rc::new(a);
        c = List_Rc::Cons(1i32, Rc::clone(&a_rc));
        print_list_rc(&c);
        // (1,[2]2,[1]3,[1]Nil)
    }
    // a already lost its ownership but c can handle its ownership since it has reference counter.
    print_list_rc(&c);
    // (1,[1]2,[1]3,[1]Nil)
    // It decreased reference count for the second element.

    // Dereference get values from a reference.
    let a = 1;
    let b = &a;
    assert_eq!(a, 1);

    // Impossible since b is a reference.
    // assert_eq!(b, 1);

    // * derefer the reference so it can be compared.
    assert_eq!(*b, 1);

    let s = String::from("String");
    assert_eq!(s, String::from("String"));

    let string_box = Box::new(s);
    assert_eq!(*string_box, String::from("String"));
    // Box can be compared with String by dereferencing it.

    let s = String::from("String");
    let string_box = MyBox::new(s);

    // MyBox can be compared since it has deref function.
    assert_eq!(*string_box, String::from("String"));

    // This function is the same with the above.
    assert_eq!(*(string_box.deref()), String::from("String"));

    let james = Student {
        name: String::from("James"),
        age: 32,
    };

    // it will call deref function in james.
    println!("{0}", *james);
    // 32

    println!("{0}", *(james.deref()));
    // 32

    let james_ref = &james;
    // This function will map james_ref to &i32
    // Rust uses deref as many as possible if there is a matching type by doing so.
    // In this case, james_ref (&Student) => *james_ref (Student) => **james_ref (&i32)
    // Notice that it only happens when using reference as a vairable(argument).
    print_age(james_ref);
    // age is 32

    // Unfortunately, variable can not be dropped ahead of time.
    // Therefore following is impossible.
    // james.drop();
    // Please use drop function like below if it is necessary.
    drop(james);
}
