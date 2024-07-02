use std::sync::mpsc;
use std::thread;
use std::time::Duration;
// mpsc is an acronym of multiple producer single consumer.
// rust supports multiple producer but allows one consumer.

fn main() {
    let s = String::from("Thread work has been done!");

    // mpsc::channel function used to communicate between threads.
    // It returns a tuple that consisted by (transmitter and receiver)
    let channel = mpsc::channel();
    let tx = channel.0;
    let rx = channel.1;

    // Like the name of mpsc, it can have many producer.
    // It can be achieved by making a clone of tx.
    let tx_other = tx.clone();

    // Thread can be generated with thread:;spawn function.
    // It gets a function as an input.
    // thread runs asyncronized time with the main thread.
    // Therefore, it mostly requires a move to take variable's ownership.
    let handle = thread::spawn(move || {
        for i in 1..5 {
            println!("hi number {} from the spawned thread!", i);
            // thread::sleep function makes a thread fall a sleep.
            thread::sleep(Duration::from_millis(1));
        }
        // hi number 1 from the spawned thread!
        // hi number 2 from the spawned thread!
        // hi number 3 from the spawned thread!
        // hi number 4 from the spawned thread!

        println!("{s}");
        // Thread work has been done!

        // transmitter may return failure since no receiver left.
        tx.send(s).unwrap();

        // Impossible, mpsc can send any type of message but only one type is allowed.
        // let v = 32;
        // tx.send(v).unwrap();

        // Impossible since this value has been sent via transmitter.
        // println!("{s}");
    });

    let handle = thread::spawn(move || {
        thread::sleep(Duration::from_millis(1));
        let s = String::from("message from other.");
        tx_other.send(s).unwrap();
    });

    // Thread message will be printed simultaneously with main thread.

    // rx = receiver
    // try_recv() is not waiting for message to be arrive.
    // Instead of it it tries to get message, if it has a message income, it returns Ok
    // Otherwise, it returnsi Err.
    // It is highly likely to not receve message yet in this point.
    let tr = rx.try_recv();
    // Therefore, this message will be printed in most of cases.
    if let Err(_) = tr {
        println!("No message arrived");
    }
    // No message arrived

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
    }
    // hi number 1 from the main thread!
    // hi number 2 from the main thread!
    // hi number 3 from the main thread!
    // hi number 4 from the main thread!

    // recv still have a possibility to return err.
    // If transmitter dropped ahead of time before receiver gets everything.
    // Notice that two message below may change their order.
    let receive = rx.recv().unwrap();
    println!("Message received : {receive}");
    // Message received : message from other.

    let receive = rx.recv().unwrap();
    println!("Message received : {receive}");
    // Message received : Thread work has been done!

    handle.join().unwrap();
}
