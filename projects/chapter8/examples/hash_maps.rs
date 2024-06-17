use std::collections::HashMap; // bring into scope, not auto in prelude

fn main() {

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 20);
    scores.insert(String::from("Yellow"), 50);
    scores.insert(String::from("Yellow"), 55); // original overwritten
    
    scores.entry(String::from("Blue")).or_insert(50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);

    for (key, value) in &scores {
        println!("{key}: {value}");
    }


    // updating value based on old value
    let mut map = HashMap::new();
    let text = "hello world wonderful world!";
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}