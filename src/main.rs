use std::env;
use term_colr::red;
use xit;

fn main() {
    let args: Vec<String> = env::args().collect();
    if let Err(e) = xit::run_command(&args) {
        eprintln!("{}", red!("Application error: {}", e));
    }
}