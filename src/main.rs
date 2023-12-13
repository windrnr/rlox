use std::env;

fn main() {
    let lox = rlox::Lox { had_error: false };
    if let Err(error) = rlox::start(lox, env::args()) {
        eprintln!("Error: {error}");
    }
}
