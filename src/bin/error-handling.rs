use std::{
    fs::File,
    io::{self, Read},
};

fn main() {
    let content = read_content_from_file("hello.txt").expect("Failed to read content");

    println!("{}", content);
}

fn read_content_from_file(path: &str) -> Result<String, io::Error> {
    let mut s = String::new();

    // if it opens, read file, if one fails, return error
    File::open(path)?.read_to_string(&mut s)?;

    Ok(s)
}
