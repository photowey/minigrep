use std::{env, process};
use std::env::Args;
use minigrep;

fn main() {
    // let args: Vec<String> = env::args().collect();
    let args: Args = env::args();
    let config = minigrep::Config::new(args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = crate::minigrep::run(config) {
        eprintln!("Application error: {:?}", e);
        process::exit(1);
    };
}