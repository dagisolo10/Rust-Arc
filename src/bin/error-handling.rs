use std::{
    fs::File,
    io::{self, Read},
};

fn main() {
    let content = read_content_from_file("hello.txt").expect("Failed to read content");

    println!("{}", content);
}

fn read_content_from_file(path: &str) -> Result<String, io::Error> {
    // the '?' does what the previous code does
    // it will exit the function by returning the error

    // * the '?' equivalent
    // * either the value or return the error
    // let mut f = match f {
    //     Ok(file) => file,
    //     Err(err) => return Err(err),
    // };

    let mut f = File::open(path)?;

    let mut s = String::new();

    // same here
    f.read_to_string(&mut s)?;

    Ok(s)
}
