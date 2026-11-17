fn main() {
    let mut a = 0;

    loop {
        if a == 5 {
            break;
        }

        println!("{a}");
        a += 1;
    }

    println!("a end");

    let mut b = 0;

    while b != 5 {
        println!("{b}");
        b += 1;
    }

    println!("b end");
}
