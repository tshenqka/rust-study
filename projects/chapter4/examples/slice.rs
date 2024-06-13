// fn first_word(s: &String) -> usize {
//     let bytes = s.as_bytes(); // need to go through element by element
//     // not tied to state might cause bug
    
//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return i;
//         }
//     }

//     return s.len();
// }


fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}


fn main() {
    let mut s = String::from("hello world");
    let word = first_word(&s);

    s.clear(); // compiler error
    println!("The first word is: {}", word);
}