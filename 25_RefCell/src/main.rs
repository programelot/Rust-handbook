// RefCell is very wierd thing in rust.
// It gets mutable reference from immutable thing.
// It can be used in the situation follows.
// If there is a trait that uses immutable self and no returns.
// However, it need to tested.
// Then, it need to store result at something.
// However, it can't be happen with rust since it only takes immutable self.
// Therefore, programmer can use RefCell in this case.
trait Speaker {
    fn talk(&self);
}

struct People();

impl Speaker for People {
    fn talk(&self) {
        println!("I am a people.");
    }
}

// To check this, use the command follows.
// cargo test -p RefCell
#[cfg(test)]
mod tests {
    use super::Speaker;
    use std::cell::RefCell;

    // To test Speaker, Mannequin has been made.
    // Howver it doesn't work with form below since self is immutable.
    // struct Mannequin{
    //     message : Vec<String>,
    // }

    // impl Speaker for Mannequin {
    //     fn talk(&self){
    //         self.message.push(String::from("I am a people."));
    //     }
    // }

    // Instead of using Vec as it is, RefCell can be used.
    struct Mannequin {
        message: RefCell<Vec<String>>,
    }

    impl Speaker for Mannequin {
        fn talk(&self) {
            // borrow_mut generates mutable reference of it.
            // it returns RefMut<T> which has a Deref trait.
            self.message
                .borrow_mut()
                .push(String::from("I am a people."));
        }
    }

    #[test]
    fn test_with_mannequin1() {
        let mannequin = Mannequin {
            message: RefCell::new(vec![]),
        };
        mannequin.talk();
        mannequin.talk();
        mannequin.talk();

        // borrow generates immutable reference of it.
        // it returns Ref<T> which has a Deref trait.
        for e in mannequin.message.borrow().iter() {
            if e != "I am a people." {
                panic!("Wrong message while verifying with mannequin.");
            }
        }

        assert_eq!(mannequin.message.borrow().len(), 3);
    }

    #[test]
    #[should_panic]
    fn test_with_mannequin2() {
        // Each of them still have the same rule.
        // It can only have one of following two conditions.
        // One mutable reference or many(could be one) immutable reference.
        let mannequin = Mannequin {
            message: RefCell::new(vec![]),
        };
        let mut mutable_ref1 = mannequin.message.borrow_mut();
        let mut mutable_ref2 = mannequin.message.borrow_mut();
        mutable_ref1.push(String::from("I am people."));
        mutable_ref2.push(String::from("I am people."));
        // two mutable reference can not exist.
        // Therefore, this test must fail.
    }
}

// Using Rc with RefCell makes a variable that can be owned by many and mutable.
use std::cell::RefCell;
use std::rc::Rc;

// To check the following part, use the command follows.
//  cargo run -p RefCell
fn main() {
    let s = String::from("Hello");
    let a = Rc::new(RefCell::new(s));
    let b = Rc::clone(&a);
    let c = Rc::clone(&a);

    // Notice that still it can not have multiple mutable reference.
    // Therefore, it only used one by one at here.
    a.borrow_mut().push_str(", ");
    b.borrow_mut().push_str("World");
    c.borrow_mut().push_str("!");

    println!("{0}", *a.borrow());
    // Hello, World!
    println!("{0}", *b.borrow());
    // Hello, World!
    println!("{0}", *c.borrow());
    // Hello, World!

    let mut c: Rc<RefCell<String>>;
    {
        let s = String::from("Hello");
        let a = Rc::new(RefCell::new(s));
        let b = Rc::clone(&a);
        c = Rc::clone(&a);

        // Notice that still it can not have multiple mutable reference.
        // Therefore, it only used one by one at here.
        a.borrow_mut().push_str(", ");
        b.borrow_mut().push_str("New World");

        println!("{0}", *a.borrow());
        // Hello, New World
        println!("{0}", *b.borrow());
        // Hello, New World
    }
    c.borrow_mut().push_str("!");
    println!("{0}", *c.borrow());
    // Hello, New World!
}
