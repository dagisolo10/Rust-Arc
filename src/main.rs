use rust_arc::{Config, run};
use std::{env, process};

fn main() {
    let args = env::args();

    let config = Config::new(args).unwrap_or_else(|error| {
        eprintln!("Failed to parse config {}", error);
        process::exit(1);
    });

    run(config);
}
