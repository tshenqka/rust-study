fn main() {
    let mut v = vec![1, 2, 3];
    let num: &mut i32 = &mut v[2]; // why must I specify i32 here, cant it auto tell?

    *num += 1;
    println!("Third element is {}", *num);
    println!("Vector is now {:?}", v);

    let x = Box::from(2);
    let y = &x;
    // *y += 2; this will not compile
}