fn main() {
    let largest = get_largest(vec![4, 5, 23, 64, 2356, 346]);
    println!("The largest number is {}", largest);

    let largest = get_largest(vec!['z', 'g', 'l', 's', 'f', 'u', 'i', 'o']);
    println!("The largest number is {}", largest);
}

fn get_largest<T: PartialOrd + Copy>(list: Vec<T>) -> T {
    let mut largest = list[0];

    for item in list {
        // or item > largest, both work
        if item.gt(&largest) {
            largest = item
        }
    }

    largest
}
