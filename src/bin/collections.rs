use std::collections::HashMap;

fn main() {
    let blue = String::from("Blue");
    let yellow = String::from("Yellow");

    let mut scores = HashMap::new();

    scores.insert(blue, 10);
    scores.insert(yellow, 7);

    let team_name = String::from("Blue");
    let team_score = scores.get(&team_name);

    println!("Score for team {} is {:?}", team_name, team_score);

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    let mut another_scores = HashMap::new();

    another_scores.insert(String::from("Red"), 12);
    // overwrites the previous insert
    another_scores.insert(String::from("Red"), 16);

    // if key exists, ignore
    // else create a map using the value provided

    // creates a map with key Green and value 30.
    another_scores.entry(String::from("Green")).or_insert(30);
    // since key Green exists, ignore
    // * this is used to create a map with a default value if it doesn't exist
    another_scores.entry(String::from("Green")).or_insert(40);

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);

        // * dereferencing
        // since count is a pointer to the count value,
        // using the dereference operator, we can access the underlying value
        // and modify it
        *count += 1;
    }

    println!("Map: {:?}", map);
}
