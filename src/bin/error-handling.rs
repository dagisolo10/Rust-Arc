use std::{
    fs::File,
    io::{self, Read},
};

fn main() {
    let content = read_content_from_file().expect("Failed to read content");

    println!("{}", content);
}

fn read_content_from_file() -> Result<String, io::Error> {
    let path = "hello.txt";
    let f = File::open(path);

    // if file returns Ok, store the file in 'f'
    // if it fails, exit and return the error
    let mut file = match f {
        Ok(file) => file,
        Err(err) => return Err(err),
    };

    // string to store the file content
    let mut s = String::new();

    // read_to_string returns a result
    // so if it succeeds, return the string in Ok, because of return type of the function
    // else the error
    // so this block returns the Result type
    // and notice there is no ';' at the end
    match file.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(err) => Err(err),
    }
}
