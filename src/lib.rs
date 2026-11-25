use std::{
    collections::HashMap,
    env::Args,
    fs::{self, read_to_string},
};

use colored::{self, Colorize};

#[allow(dead_code)]
pub struct Config {
    query: String,
    directory: String,
    case_insensitive: bool,
    skipped_folders: Vec<String>,
}

impl Config {
    pub fn new(args: Args) -> Result<Config, &'static str> {
        let args: Vec<String> = args.collect();

        let query = args.get(1).ok_or("Missing query argument")?.clone();
        let directory = args.get(2).ok_or("Missing directory argument")?.clone();

        let mut case_insensitive = false;
        let mut skipped_folders: Vec<String> = vec![];

        let mut index = 3;

        while index < args.len() {
            match args[index].as_str() {
                "-i" => {
                    case_insensitive = true;
                }
                "-s" => {
                    let skipped_arg = args.get(index + 1).ok_or("Missing folders after -s")?;

                    skipped_folders.extend(skipped_arg.split('|').map(String::from));

                    index += 1;
                }
                _ => return Err("Unknown argument"),
            }

            index += 1;
        }

        Ok(Config {
            query,
            directory,
            skipped_folders,
            case_insensitive,
        })
    }
}

pub fn run(config: Config) {
    println!(
        "\n{} \"{}\" in \"{}\" (including subdirectories).",
        "Searching for",
        config.query.yellow().bold(),
        config.directory.cyan()
    );

    println!(
        "{} {}",
        "Case-insensitive matching:".bold(),
        if config.case_insensitive {
            "enabled".green()
        } else {
            "disabled".red()
        }
    );

    if config.skipped_folders.is_empty() {
        println!("{} {}", "Excluded folders:".bold(), "none".dimmed());
    } else {
        println!(
            "{} {}",
            "Skipping folders:".bold(),
            config.skipped_folders.join(", ").red()
        );
    }

    let paths = scan_dir(&config.directory, &config.skipped_folders);

    for path in paths {
        search(&path, &config);
    }
}

fn scan_dir(directory: &str, skipped_folders: &Vec<String>) -> Vec<String> {
    let mut discovered = Vec::new();
    let entries = match fs::read_dir(directory) {
        Ok(entries) => entries,
        Err(err) => {
            eprintln!("Failed to scan {}: {}", directory, err);
            return discovered;
        }
    };

    for entry in entries {
        let entry = match entry {
            Ok(entry) => entry,
            Err(err) => {
                eprintln!("Failed to read directory entry: {}", err);
                continue;
            }
        };

        let path = entry.path();

        let full_path = match path.into_os_string().into_string() {
            Ok(path) => path,
            Err(_) => {
                eprintln!("Skipping entry: path contains invalid UTF-8");
                continue;
            }
        };

        let file_type = match entry.file_type() {
            Ok(file_type) => file_type,
            Err(err) => {
                eprintln!("Failed to determine file type: {}", err);
                continue;
            }
        };

        if file_type.is_dir() {
            let mut is_exclueded: bool = false;
            for ex in skipped_folders {
                let path = entry.path();
                let full_path_foldername = path.file_name().unwrap().to_str().unwrap();

                if full_path_foldername == ex {
                    is_exclueded = true;
                    break;
                }
            }

            if is_exclueded {
                continue;
            }

            discovered.extend(scan_dir(&full_path, skipped_folders));
        } else if file_type.is_file() {
            discovered.push(full_path);
        }
    }

    discovered
}

#[allow(dead_code)]
fn search(full_path: &str, config: &Config) {
    let contents: String = match read_to_string(full_path) {
        Ok(contents) => contents,
        Err(err) => {
            eprintln!("Failed to read {}: {}", full_path, err);
            return;
        }
    };
    let normoalized_query = match config.case_insensitive {
        false => config.query.clone(),
        true => config.query.to_lowercase(),
    };

    let filtered: Vec<&str> = contents
        .lines()
        .filter(|line| {
            let normalized_line = match config.case_insensitive {
                false => *line,
                true => &line.to_lowercase(),
            };

            normalized_line.contains(&normoalized_query)
        })
        .collect();

    if filtered.len() == 0 {
        return;
    }

    println!("\n{}", full_path.purple());

    let mut occ_map: HashMap<usize, Vec<(usize, usize)>> = HashMap::new();

    for (occurrence_line, original_line) in contents.lines().enumerate() {
        let normalized_line = match config.case_insensitive {
            false => original_line,
            true => &original_line.to_lowercase(),
        };

        if !normalized_line.contains(&normoalized_query) {
            continue;
        }

        for (start_index, matched) in normalized_line.match_indices(&normoalized_query) {
            let end_index = start_index + matched.len();

            let line_map = occ_map.entry(occurrence_line).or_insert(vec![]);

            line_map.push((start_index, end_index));
        }

        let occurrence = occ_map.get(&occurrence_line).unwrap();

        let mut cursor: usize = 0;

        print!("{}. ", (occurrence_line + 1).to_string().green());

        for (start_index, end_index) in occurrence {
            let before = &original_line[cursor..*start_index];
            let query = &original_line[*start_index..*end_index];

            print!("{}{}", before, query.red());

            cursor = *end_index;
        }

        println!("{}", &original_line[cursor..]);
    }
}
