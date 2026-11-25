use rust_arc;
use std::{env, process};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.iter().any(|arg| arg == "--help" || arg == "-h") {
        rust_arc::print_help();
        return;
    }

    let config = rust_arc::Config::new(&args).unwrap_or_else(|error| {
        eprintln!("Failed to parse config {}", error);
        process::exit(1);
    });

    rust_arc::run(config);
}
