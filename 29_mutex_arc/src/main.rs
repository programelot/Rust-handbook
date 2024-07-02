use rand::Rng;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    // Mutex can be used as a lock of mutithreading environment.
    // It locks up the value that other thread can not use it unless lock has been released.
    // However, it requires multiple owner of lock to do so.
    // Reacp : Rc, Refcell only supports a single thread environment.
    // Arc is a multithreaded version of Rc.
    // Arc is an acronym of atomically reference counted.
    // It has the same API with Rc.
    // Infact, Mutex is simillar with RefCell for multithreading environment.
    // Even though variable was immutable, Mutex still provides mutable reference for it via lock.
    // Notice that counter and counter_for_thread in the below are immutable.
    let counter = Arc::new(Mutex::new(0));
    let mut handles = Vec::new();

    for _ in 1..10 {
        // Notice that this variable generates from outside of the thread.
        // Then, it will be transfered to the thread right after.
        let counter_for_thread = Arc::clone(&counter);
        handles.push(thread::spawn(move || {
            let mut thread_index: i32;
            {
                // Lock may can fail since thread that holds lock may panic either.
                // lock() returns mutable reference of the target.
                let mut num = counter_for_thread.lock().unwrap();
                *num += 1;
                thread_index = *num;
                // Lock can be released by drop it.
                // Notice that even it didn't releases, it will be release at the end of the scope.
                drop(num);
            }
            println!("This thread got a {0}.", thread_index);

            // rand::thread_rng() is a thread_local random generator.
            let wait_for = rand::thread_rng().gen_range(1..1000);
            println!("Thread {0} started to wait.", thread_index);
            thread::sleep(Duration::from_millis(wait_for));
            println!(
                "Thread {0} waited for {1} millisecs.",
                thread_index, wait_for
            );
        }));
    }
    // This thread got a 1.
    // This thread got a 2.
    // This thread got a 3.
    // This thread got a 4.
    // This thread got a 5.
    // This thread got a 6.
    // This thread got a 7.
    // This thread got a 8.
    // This thread got a 9.
    // Thread 1 started to wait.
    // Thread 5 started to wait.
    // Thread 2 started to wait.
    // Thread 3 started to wait.
    // Thread 4 started to wait.
    // Thread 6 started to wait.
    // Thread 7 started to wait.
    // Thread 8 started to wait.
    // Thread 9 started to wait.
    // Thread 9 waited for 159 millisecs.
    // Thread 8 waited for 196 millisecs.
    // Thread 2 waited for 335 millisecs.
    // Thread 1 waited for 593 millisecs.
    // Thread 4 waited for 599 millisecs.
    // Thread 6 waited for 606 millisecs.
    // Thread 3 waited for 709 millisecs.
    // Thread 7 waited for 815 millisecs.
    // Thread 5 waited for 893 millisecs.
    // counter became 9

    // Notice that 9 lines below may in the different order.
    // It usually sorted with the time it waited.
    // Howver, it may not since thread generation and deletion time.
    for handle in handles {
        handle.join();
    }
    println!("counter became {0}", counter.lock().unwrap());
}

// Extra note
// There are send and sync traits.
// Sync trait denoted as std::marker
// Send trait means that it can transfer the ownership.
// Most of the type has a send trait.
// However, Rc<T> (and probably RefCell<T>) is not.
// It prevented to do so since it may violates in multithreading environment.
// (At the same time, Arc can be used as an alternative.)
// (Notice that Mutex can be an alternative for RefCell either.)
//
// Sync trait means that it can be accessed via many thread.
// Most types are sync but Rc<T> (and probably RefCell<T>) is not either.
// Implementing these traits is not ideal.
// If the components of something is sync or send, it will be considered as sync or send repectively.
// Therefore, it is not mendatory to make it by own.
//
