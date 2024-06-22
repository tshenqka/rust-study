use std::thread::sleep;
use std::time::Duration;

fn main() {
    for i in 0..=100 {
        print!("\rProgress: {:3}%", i); // \r returns to the start of the line
        sleep(Duration::from_millis(300));
    }
    println!("\nDone!");
}
