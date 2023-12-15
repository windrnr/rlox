use std::env;

fn main() {
    rlox::AstPrinter::new().execute();
    let fallo = false;
    if let Err(error) = rlox::start(env::args(), fallo) {
        eprintln!("Error: {error}");
    }
}
