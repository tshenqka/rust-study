fn main() {

    let mut x = Box::new(1); // returns an instance of Box which is a smart pointer
    let a = *x;
    *x += 1;

    let r1 = &x;
    let b = **r1;

    let r2 = &*x;
    let c = *r2; 

}

// fn main() {
//     let x = 5;
//     let r1 = &x as *const i32; // Raw pointer to immutable data
//     let r2 = &x as *mut i32;   // Raw pointer to mutable data (note: this is usually unsafe)

//     unsafe {
//         println!("r1 points to: {}", *r1); // Dereferencing raw pointers requires an unsafe block
//         println!("r2 points to: {}", *r2); // Dereferencing raw pointers requires an unsafe block
//     }
// }
