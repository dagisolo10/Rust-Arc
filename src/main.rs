use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = rust_arc::Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1)
    });

    rust_arc::run(config).unwrap_or_else(|err| {
        eprintln!("Application error: {}", err);
        process::exit(1);
    })
}
