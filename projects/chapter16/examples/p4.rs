use std::sync::mpsc; // multiple producer, single consumer
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();  // transmitter and receiver
    
    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
        println!("val is {}", val); // now the thread still owns val, but val is an empty binding that can't be used
    });

    let received = rx.recv().unwrap(); // this will block the main threads execution and wait until a value is sent down
    println!("Got: {}", received);
}