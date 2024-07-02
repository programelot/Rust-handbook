// Removed redundant comments from 25_memory_leak.
use std::cell::RefCell;
use std::rc::{Rc, Weak};

enum List {
    Cons_weak(i32, RefCell<Weak<List>>),
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}

impl List {
    fn print(&self, depth: i32) {
        self._print(depth);
        println!("");
    }

    fn _print(&self, depth: i32) {
        match self {
            List::Cons(value, p) => {
                print!("{0} ", value);
                if depth > 0 {
                    let p_list = &**p.borrow();

                    print!(
                        "[Strong: {0}, Weak: {1}]",
                        Rc::strong_count(&*p.borrow()),
                        Rc::weak_count(&*p.borrow())
                    );
                    match p_list {
                        List::Cons(_, _) => {
                            p_list._print(depth - 1);
                        }
                        List::Cons_weak(_, _) => {
                            p_list._print(depth - 1);
                        }
                        List::Nil => (),
                    }
                }
            }
            List::Cons_weak(value, p) => {
                // Print value first
                print!("{0} ", value);
                // If depth is more than zero and there is next, print reference count and recursively print value.
                if depth > 0 {
                    // p.borrow() return Ref<Weak<List>> type.
                    // *p.borrow() return Weak<List> type.
                    // p.borrow().upgrade() returns Option<List> type.(* omitted)
                    // p_rc Will get Rc<List> from Weak<List> if it is possible.
                    let p_weak_upgraded = p.borrow().upgrade();
                    // Notice that p_rc is Rc<List>
                    // It will make a clone of it.
                    // Therefore, it increases reference counter.
                    let mut p_rc;
                    match p_weak_upgraded {
                        Some(v) => {
                            p_rc = v;
                        }
                        None => {
                            println!("Gone.");
                            return;
                        }
                    }

                    print!(
                        "[Strong: {0}, Weak: {1}]",
                        Rc::strong_count(&p_rc),
                        Rc::weak_count(&p_rc)
                    );
                    let p_list = &*p_rc;
                    match p_list {
                        List::Cons(_, _) => {
                            p_list._print(depth - 1);
                        }
                        List::Cons_weak(_, _) => {
                            p_list._print(depth - 1);
                        }
                        List::Nil => (),
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
        // To make a weak link, downgrade can be used to get Weak<T>.
        // It will not claim ownership but it will have an access point.
        // It need to use upgrade() to check whether the value exists or not.
        // If there is no value, it will return None.
        // Other wise it will return Some(Rc<T>).
        let x = List::Cons_weak(10, RefCell::new(Weak::new()));
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
        if let List::Cons_weak(_, next) = &*a {
            *next.borrow_mut() = Rc::downgrade(&b);
        }
        c = Rc::clone(&b);

        // Reference counter below increases as it prints more elements.
        // It happens since Weak::upgrade returns Option<Rc>.
        // Which means it generates new Rc (therefore counter must increase).
        // In other word, Weak tries to acquire a new strong coutner and returns it.

        a.print(5);
        // 10 [Strong: 3, Weak: 1]20 [Strong: 2, Weak: 0]10 [Strong: 4, Weak: 1]20 [Strong: 2, Weak: 0]10 [Strong: 5, Weak: 1]20

        b.print(5);
        // 20 [Strong: 2, Weak: 0]10 [Strong: 3, Weak: 1]20 [Strong: 2, Weak: 0]10 [Strong: 4, Weak: 1]20 [Strong: 2, Weak: 0]10

        // The situation is like belows.
        // [x, 10] =(Weak)=> [y, 20] =(Strong)=> [x, 10] (again)
        // Notice that y is owned by b and c at the same time,
        //             x is owned by a.
    }

    c.print(5);
    // 20 [Strong: 1, Weak: 0]10 [Strong: 2, Weak: 1]20 [Strong: 1, Weak: 0]10 [Strong: 3, Weak: 1]20 [Strong: 1, Weak: 0]10

    // Now situation changed, if c disappear memory will be deallocated.
    // If c releases, it changes reference count of y to zero.
    // Therefore, y will be released and it will release x since y has been released.
}
