use std::{fs::File, io::ErrorKind};

fn main() {
    let path = "hello.txt";
    let file = File::open(path);

    /*
     checks if the file exists
     - if the error kind is not found, creates the file
     - else if its any kind of error other than not found, panic
    */
    let file = match file {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create(path) {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            // ! catch all arm
            other_error => panic!("Problem opening the file: {:?}", other_error),
        },
    };

    println!("File: {:?}", file);
}
