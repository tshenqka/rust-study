// UTF8 is Unicode Transformation Format - 8 bit

fn main() {

    let mut s = String::from("foo");
    s.push_str("bar"); // param is string slice to not take ownership

    let mut s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    // let s1 = s1 + &s2; // this works. 

    let s = format!("{s1}{s2}");
    println!("{s}");



    // NOTE: Rust doesnt allow indexing cus multi byte utf

    for c in "Зд".chars() { // automatically handles calling next method
        println!("{c}");
    }

    // check out contains and replace
}