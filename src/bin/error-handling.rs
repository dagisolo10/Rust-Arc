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

    /*
       - This uses closure which will be covered in chapter 10
       - this is like the catch callback in TS where it gives us
       the error and a call back which in this case is called
       'anonymous function'
    */
    let file = File::open(path).unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create(path)
                .unwrap_or_else(|error| panic!("Problem creating the file: {:?}", error))
        } else {
            panic!("Problem opening the file: {:?}", error)
        }
    });

    println!("File: {:?}", file);
}
