use std::{
    fs::{self, File},
    io::{self, Read},
};

fn main() {
    let content = read_content_from_file("hello.txt").expect("Failed to read content");

    println!("{}", content);
}

fn read_content_from_file(path: &str) -> Result<String, io::Error> {
    fs::read_to_string(path)
}
