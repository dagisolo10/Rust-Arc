fn main() {
    let (_x, y) = coordinates();

    if y > 5 {
        println!("y is greater than 5")
    } else if y < 5 {
        println!("y is less than 5")
    } else {
        println!("y is 5")
    }
}

fn coordinates() -> (i32, i32) {
    (1, 5)
}
