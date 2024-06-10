// fn stringify_name_with_title(name: &Vec<String>) -> String { // why not mut name?
//     let mut name_clone = name.clone(); // what does this do?
//     name_clone.push(String::from("Esq."));
//     let full = name_clone.join(" ");
//     full
// }

fn stringify_name_with_title(name: &Vec<String>) -> String { // why not mut name?
    let mut full = name.join(" ");
    full.push_str(" Esq.");
    full
}

fn round_in_place(v: &mut Vec<f32>) {
    for n in v {
        *n = n.round();
    }
}

// unsafe program: aliasing and mutating a data struct
// fn add_big_strings(dst: &mut Vec<String>, src: &[String]) {
//     let largest: &String = dst.iter().max_by_key(|s| s.len()).unwrap();

//     for s in src {
//         if s.len() > largest.len() {
//             dst.push(s.clone());
//         }
//     }
// }

fn add_big_strings(dst: &mut Vec<String>, src: &[String]) {
    let largest_len: usize = dst.iter().max_by_key(|s| s.len()).unwrap().len();

    for s in src {
        if s.len() > largest_len {
            dst.push(s.clone());
        }
    }
}



// fn main() {

//     let v: Vec<String> = vec![String::from("Hello world")];
//     let mut s = v[0].clone();
//     s.push('!');
//     println!("{s}");
//     // let s = *s_ref; // error: if this is legal, then we could get double free situation where v and s both think they own data
//     return;
// }

fn get_first(name: &(String, String)) -> &String {
    &name.0 // borrow only in a read only manner
}
fn main() {

    let mut name = (
        String::from("Ferris"),
        String::from("Rustacean")
    );

    let first = get_first(&name);
    // let first = &name.0;
    // name.1.push_str(", Esq"); // problem now is Rust only looks at func signature to conservatively decide
    println!("{first} {}", name.1);


    // let mut a = [0, 1, 2, 3];
    // let x = &mut a[1];
    // let y = &a[2]; // because a gave all index permissions over to x
    // *x += *y

    //could do this
    let mut a = [0, 1, 2, 3];
    let x = &mut a[1] as *mut i32;
    let y = &a[2] as *const i32; // because a gave all index permissions over to x
    unsafe {
        *x += *y
    }
    return;
}