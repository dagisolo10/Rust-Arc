#[derive(Debug)]
enum Access {
    Full,
}

fn main() {
    let numbers = one_two_three();
    let (x, y, z) = one_two_three();

    println!("{}, {}", x, numbers.0);
    println!("{}, {}", y, numbers.1);
    println!("{}, {}", z, numbers.2);

    let (name, access) = ("Jake", Access::Full);

    println!("{} has {:?} access", name, access);
}

fn one_two_three() -> (i32, i32, i32) {
    (1, 2, 3)
}
