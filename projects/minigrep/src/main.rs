use std::env;
use std::process;

use minigrep::Config;


fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err| { // between vertical pipes
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    if let Err(e) = minigrep::run(config) {
        println!("Applications error: {e}");
        process::exit(1);
    }
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
