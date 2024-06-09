fn main() {
    let first = String::from("Ferris"); // boxes are used by Rust data structures
    let first_clone = first.clone(); // clone makes a deep copy
    let full = add_suffix(first_clone);
    println!("{full}, originally {first}");
}

fn add_suffix(mut name: String) -> String {
    name.push_str(" Jr.");
    name
}

//ownership is discipline of heap management
// all heap data is owned by one variable, deallocates heap after owner out of scope, ownership happens by moves, on assignments and func calls