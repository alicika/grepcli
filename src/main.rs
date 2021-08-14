extern crate grepcli;

use std::env;
use std::process;
use grepcli::{Config, run};

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = run(config) {
        eprintln!("Application Error: {}", e);
        process::exit(1);
    }
}