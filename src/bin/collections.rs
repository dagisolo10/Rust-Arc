fn main() {
    let s1 = String::from("My ");
    let s2 = String::from("boy");

    // format doesn't take ownership so both s1 and s2 can be used afterwards
    let s3 = format!("{}{}", s1, s2);

    println!("{}", s3);
}
