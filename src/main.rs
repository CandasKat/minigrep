use std::env;
use minigrep::{handle_args};

fn main() {
    let args: Vec<String> = env::args().collect();
    let result = handle_args(&args);
    println!("{}", result)
}
