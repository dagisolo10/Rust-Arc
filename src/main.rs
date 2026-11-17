fn main() {
    let some_bool = false;

    match some_bool {
        true => println!("TRUE"),
        false => println!("FALSE"),
    }

    let some_int = 3;

    match some_int {
        1 => println!("1"),
        2 => println!("2"),
        3 => println!("3"),
        _ => println!("Something else"),
    }
}
