#![allow(unused_variables)]

fn main() {
    println!("Hello， World!");

    //创建字符串
    let s1 = String::new();
    let s2 = "hello world from s2.";
    let s3 = s1.to_string();
    let s4 = s2.to_string();
    let s5 = "hello".to_string();

    println!("s1 is : {}", s1);
    println!("s2 is : {}", s2);
    println!("s3 is : {}", s3);
    println!("s4 is : {}", s4);
    println!("s5 is : {}", s5);

    //追加字符串
    //let mut s6=s4.push_str("!");

    //拼接字符串
    let s7 = "hello ".to_string();
    let s8 = "world.".to_string();
    let s9 = s7 + &s8;
    println!("s9 is : {}", s9);

    //遍历字符串
    let s10 = "爱我中华！";
    let s11 = "I love China!";
    let s12 = "안녕하세요";
    let s13 = "Здравствуйте";
    for c in s10.chars() {
        println!("c in s10 are: {}", c);
    }
    for c in s11.chars() {
        println!("c in s10 are: {}", c);
    }
    for c in s12.chars() {
        println!("c in s10 are: {}", c);
    }
    for c in s13.chars() {
        println!("c in s10 are: {}", c);
    }

    //返回byte
    for b in s11.bytes() {
        println!("c in s10 are: {}", b);
    }
}
