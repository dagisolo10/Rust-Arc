use std::env;
use std::error::Error;
use std::fs;
use std::process;

struct Config {
    path: String,
    query: String,
}

impl Config {
    // constructing a new config
    fn new(args: &[String]) -> Result<Config, String> {
        if args.len() < 3 {
            return Err(format!("Expected 2 args, got {}", args.len() - 1));
        }

        // inorder not to take ownership, clone the strings
        Ok(Config {
            path: args[2].clone(),
            query: args[1].clone(),
        })
    }
}

fn main() {
    // need to specify the type so collect knows the type
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        // on fail, print the error and exit the program
        println!("Problem parsing arguments: {}", err);
        process::exit(1)
    });

    println!("Searching for '{}'", config.query);
    println!("In file '{}'", config.path);

    // * shorter version of a match expression but with only what i need, either error or result
    // if let Err(e) = run(config) {
    //     println!("Application error: {}", e);
    //     process::exit(1);
    // };

    run(config).unwrap_or_else(|err| {
        println!("Application error: {}", err);
        process::exit(1);
    })
}

// any type of error
fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(&config.path)?;

    println!("{}", contents);

    Ok(())
}
