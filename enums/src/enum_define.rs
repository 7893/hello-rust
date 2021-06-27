#[derive(Debug)]
enum Week {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}

fn main() {
    let today = Week::Saturday; // 使用枚举
    let tomorrow = Week::Sunday;

    println!("{:?}", today);
    println!("{:?}", tomorrow);
}
