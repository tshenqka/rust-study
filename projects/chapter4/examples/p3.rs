fn main() {
    let m1 = String::from("Hello");
    let m2 = String::from("world");
    greet(&m1, &m2);
    let s = format!("{} {}", m1, m2);
}

fn greet(g1: &String, g2: String) {
    println!("{} {}", g1, g2); // automatic dereferencing: deref coercion
}