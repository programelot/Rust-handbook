// Using RefCell and Rc Can cause self loop in memory counter.
// Rc shouldn't be mutable.
// However, changing it to other value can cause this situation.
use std::cell::RefCell;
use std::rc::Rc;

enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}

impl List {
    // Wrapper function to print it clearly.
    fn print(&self, depth: i32) {
        self._print(depth);
        println!("");
    }

    fn _print(&self, depth: i32) {
        match self {
            List::Cons(value, p) => {
                // Print value first
                print!("{0} ", value);
                // If depth is more than zero and there is next, print reference count and recursively print value.
                if depth > 0 {
                    // p.borrow() return Ref<Rc<List>> type.
                    // *p.borrow() return Rc<List> type.
                    // **p.borrow() return List type.
                    // &**p.borrow() return &List type.
                    let p_list = &**p.borrow();

                    // &*p.borrow return &Rc<List> type.
                    // It can be used by &p.borrow() since rust automatically deref things in here.
                    print!("[{0}]", Rc::strong_count(&p.borrow()));
                    match p_list {
                        List::Cons(_, _) => {
                            p_list._print(depth - 1);
                        }
                        List::Nil => {
                            println!("Nil.");
                        }
                    }
                }
            }
            List::Nil => {
                println!("Nil.");
            }
        }
    }
}

fn main() {
    let mut c;
    {
        let x = List::Cons(10, RefCell::new(Rc::new(List::Nil)));
        let y = List::Cons(20, RefCell::new(Rc::new(x)));
        let mut a = Rc::new(List::Nil);
        // Link a to y's next (x).
        // It designed in this way since to clone Rc, it requires Rc and it exists in the list only.
        if let List::Cons(_, next) = &y {
            a = Rc::clone(&next.borrow());
            println!("Success to acquired a.");
        }
        // link b to y.
        let mut b = Rc::new(y);

        if let List::Cons(_, next) = &*a {
            *next.borrow_mut() = Rc::clone(&b);
        }
        // It makes that x have 10 and links to y and
        //               y have 20 and links to x.
        c = Rc::clone(&b);
        // and c get a ownership shared by a.

        a.print(5);
        // 10 [2]20 [3]10 [2]20 [3]10 [2]20

        b.print(5);
        // 20 [3]10 [2]20 [3]10 [2]20 [3]10

        // The situation is like belows.
        // [x, 10] => [y, 20] => [x, 10] (again)
        // Notice that y is owned by b and c at the same time,
        //             x is owned by a.
    }

    c.print(5);
    // 20 [1]10 [2]20 [1]10 [2]20 [1]10

    // Like it shows, memory leak happens.
    // Even with out c(where it pointing 10), it will still exists in the memory like below.
    // 20 [1]10 [1]20 [1]10 [1]20 [1]10
    // Notice that a already gone but
}
