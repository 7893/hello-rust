fn func(y: Option<i8>) -> i8 {
    match y {
        Option::Some(i8) => 5,
        Option::None => 0,
    }
}

fn main() {
    let some_num = Some(5);
    let some_string = Some("a string");

    let absent_num: Option<i32> = None;

    println!("{:?}", some_string);

    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    let y1 = func(y);
    println!("{}", x + y1);
}
