fn main() {
    let age = 22;
    let mut name = "Ayakashi";

    println!("Name before {}", name);

    name = "Bob";

    println!("Name after {}, {}", name, age);

    let x = add(1, 2);
    let y = add(3, 9);
    let z = add(x, y);

    println!("x: {}, y: {}, z:{}", x, y, z);
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}
