use std::thread;

fn main() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || { // move overrides rust conservative default of borrowing
        println!("Here's a vector: {:?}", v);
    });

    handle.join().unwrap();
}