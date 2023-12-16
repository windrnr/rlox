use std::env;

fn main() {
    if let Err(error) = rlox::start(env::args()) {
        eprintln!("Error: {error}");
    }
}
