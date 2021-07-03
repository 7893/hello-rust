struct Point<T, U> {
    x: T,
    y: U,
}

fn main() {
    let int = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
    let both = Point { x: 2, y: 3.2 };
}
