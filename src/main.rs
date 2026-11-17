enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn main() {
    let dir: Direction = Direction::Down;

    let go = which_way(&dir);

    println!("{go}");
}

// borrowing will be covered in future topics
fn which_way(dir: &Direction) -> &str {
    match dir {
        Direction::Up => "Up",
        Direction::Down => "Down",
        Direction::Left => "Left",
        Direction::Right => "Right",
    }
}
