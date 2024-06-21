use std::env;
use std::fs;


fn main() {
    let args: Vec<String> = env::args().collect();
    let config = parse_config(&args);
    let contents = fs::read_to_string(config.file_path).expect("Should have been able to read the file");
}

struct Config {
    query: String,
    file_path: String
}

fn parse_config(args: &[String]) -> Config {
    let query = args[1].clone();
    let file_path = args[2].clone();

    Config {query, file_path}
}

// // old version
// fn main() {
//     let args: Vec<String> = env::args().collect();
    
//     let query = &args[1];
//     let file_path = &args[2];

//     println!("In file {}", file_path);

//     let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
//     println!("With text:\n{contents}");
// }
