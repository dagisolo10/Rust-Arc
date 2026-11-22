struct Point<T> {
    x: T,
    y: T,
}

// works for any type
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

// only works for point y of type f64
impl Point<f64> {
    fn y(&self) -> &f64 {
        &self.y
    }
}

fn main() {
    let p1 = Point { x: 1, y: 2 };
    // only x function is available
    println!("x: {}", p1.x());

    let p2 = Point { x: 5.0, y: 10.0 };
    // both x and y are available
    println!("y: {}", p2.y());

    // let p3 = Point { x: 3, y: 6.0 };
}
