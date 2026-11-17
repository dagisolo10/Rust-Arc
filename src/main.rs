fn main() {
    let my_num = 3;

    // message is a string slice
    let message: &str = match my_num {
        1 => "hello",
        _ => "good bye",
    };

    println!("{}", message);
}
