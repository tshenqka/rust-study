use std::env;
use std::process;

use minigrep::Config;


fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| { // between vertical pipes
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        eprintln!("Applications error: {e}");
        process::exit(1);
    }
}