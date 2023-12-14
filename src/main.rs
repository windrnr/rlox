use std::env;

fn main() {
    let fallo = false;
    if let Err(error) = rlox::start(env::args(), fallo) {
        eprintln!("Error: {error}");
    }
}
