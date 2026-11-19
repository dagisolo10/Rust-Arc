fn main() {
    println!();

    let array = [1, 2, 3];
    let first = array[0];

    println!("First element of the array {}", first);

    let mut v: Vec<i32> = Vec::new();

    v.push(1);
    v.push(2);
    v.push(3);

    // stored on the heap
    let v2 = vec![1, 2, 3];

    let index = 20;

    match v2.get(index) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no element at index {}", index),
    }

    for i in &mut v {
        *i *= 2
    }

    for i in v {
        println!("{}", i)
    }

    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(93),
        SpreadsheetCell::Float(10.12),
        SpreadsheetCell::Text(String::from("Blue")),
    ];

    match &row[1] {
        SpreadsheetCell::Int(i) => println!("{}", i),
        _ => println!("Not integer"),
    }
}
