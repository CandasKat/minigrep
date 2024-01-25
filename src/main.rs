use std::env;
use minigrep::handle_args;

fn main() {
    let args: Vec<String> = env::args().collect();
    match handle_args(&args) {
        Ok(message) => println!("{}", message),
        Err(e) => println!("Erreur : {}", e),
    }
}
