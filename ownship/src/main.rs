fn main() {
    println!("Hello, world!");

    let mut s = String::from("hello world.");
    s.push_str("world2!");
    println!("{}", s);

    let s1 = String::from("Rust");
    let s2 = s1;
    //浅拷贝
    //s1不再有效
    //println!("s1:{}",s1);
    println!("s2:{}", s2);

    let s3 = s2.clone();
    println!("s3:{}", s3);
    println!("s2:{}", s2);

    //s是非copy的，在函数体内已经释放，所以不可用
    taken_ownship(s);

    //x是copy的，所以还可以用
    let x = 5;
    makes_copy(x);

    let s4 = gives_ownship();
    let s5 = String::from("hello");
    let s6 = takes_and_gives_back(s5);
    println!("s4:{}", s4);
    //s5被移除作用域并丢弃
    //println!("s5:{}",s5);
    println!("s6:{}", s6);

    let s7 = String::from("hello");
    let (s7, len) = calculate_length(s7);
    println!("The length of '{}' is {}.", s7, len);
}

fn taken_ownship(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

fn gives_ownship() -> String {
    let some_string = String::from("hello");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}
