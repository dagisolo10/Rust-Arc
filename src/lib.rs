use std::env;
use std::error::Error;
use std::fs;

#[derive(Debug)]
pub struct Config {
    pub path: String,
    pub query: String,
    pub case_insensitive: bool,
}

impl Config {
    pub fn new(args: &Vec<String>) -> Result<Config, String> {
        if args.len() < 3 {
            return Err(format!("Expected 2 args, got {}", args.len() - 1));
        }

        let mut path: Option<String> = None;
        let mut query: Option<String> = None;

        let mut flags: Vec<&str> = Vec::new();

        for arg in args.iter().skip(1) {
            if arg.starts_with("-") {
                flags.push(arg.trim());
                continue;
            }

            if path.is_none() {
                path = Some(arg.trim().to_string().clone());
            } else if query.is_none() {
                query = Some(arg.trim().to_string().clone());
            }
        }

        let case_insensitive = flags.contains(&"-i") || env::var("CASE_INSENSITIVE").is_ok();

        // ok_or returns a result so using '?', exit if its Err
        let path = path.ok_or("Missing file path")?;
        let query = query.ok_or("Missing search query")?;

        Ok(Config {
            path,
            query,
            case_insensitive,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(&config.path)?;

    let search_result = search(&config.query, &contents, config.case_insensitive);

    for line in search_result {
        println!("{}", line);
    }

    Ok(())
}

// * One thing to remember: We don't choose a lifetime
// * based on which parameter is more important to the function.
// * We choose it based on which borrowed data the returned
// * references actually depend on.

// everytime we return a reference from a function, the LT must be
// either one of the LT of the parameters
// in this case, we want the LT of the return value to be tied to the contents
// because the return strings will be lines from the contents
pub fn search<'a>(query: &str, contents: &'a str, case_insensitive: bool) -> Vec<&'a str> {
    let query = if case_insensitive {
        &query.to_lowercase()
    } else {
        query
    };

    let mut results: Vec<&str> = Vec::new();

    for line in contents.lines() {
        let line_search = if case_insensitive {
            &line.to_lowercase()
        } else {
            line
        };

        if line_search.contains(query) {
            results.push(line);
        }
    }

    results
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, contents, false)
        );
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(vec!["Rust:", "Trust me."], search(query, contents, true));
    }
}
