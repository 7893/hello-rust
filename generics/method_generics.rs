struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    //返回的类型中第一个T来自于 Point 的T，第二个W来自于mixup的W，是一种混合返回
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    println!("Hello method generics!");
    let p1 = Point { x: 6, y: 2 };
    let p2 = Point { x: "hello", y: 'c' };
    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}
