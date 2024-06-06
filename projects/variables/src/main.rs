use std::io;

fn main() { // main is always entry point to rust programs
    // statically typed means rust must know types of all variables at compile time
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3; // constant must be typed, and assigned a value that can be evaled at compile time
    let mut x = 5;
    println!("The value of x is {x}");
    x = 6;
    println!("The value of x is {x}");

    // rust scalar types rep a single value: integers, floating point numbers, booleans, and chars

    let test: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = test.0;
    let six_point_four  = test.1;
    let one = test.2;

    let a = [1, 2, 3, 4, 5];
    println!("Please enter an index");
    let mut index = String::new();

    io::stdin()
        .read_line(&mut index) // &mut keyword creates a mutable reference
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index]; // rust checks the index to be in range, mem safety in action
}
