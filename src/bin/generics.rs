#[derive(Debug)]
struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn mix_up<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    let pt1 = Point { x: 1, y: 2 };
    let pt2 = Point {
        x: "hello",
        y: 10.4,
    };

    let pt3 = pt2.mix_up(pt1);

    println!("pt3.x = {}", pt3.x);
    println!("pt3.y = {}", pt3.y);
}
