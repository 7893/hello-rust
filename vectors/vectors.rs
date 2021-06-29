#![allow(unused_variables)]
fn main() {
    //声明
    let a = vec![1, 2, 3];
    let mut b: Vec<i32> = Vec::new();

    println!("The length of a is : {}", a.len());
    println!("The length of b is : {}", b.len());

    //添加元素
    b.push(5);
    b.push(6);
    b.push(7);
    b.push(8);
    println!("The length of b is : {}", b.len());
    println!("The last of b is : {}", b[3]);

    //删除元素
    b.pop();
    println!("The length of b is : {}", b.len());
    //println!("The last of b is : {}",b[3]);

    //索引访问元素
    let a_third: &i32 = &a[2];
    let b_third: &i32 = &b[2];
    println!("The third element of 'a' is : {}", a_third);
    println!("The third element of 'b' is : {}", b_third);

    //遍历访问
    for i in &b {
        println!("{}", i);
    }

    //遍历修改元素
    //需要遍历元素的引用
    let mut c = vec![100, 101, 102];
    for i in &mut c {
        *i += 50;
    }
    for i in &c {
        println!("{}", i);
    }

    //利用枚举存储不同类型的例子
    /*
    enum C{
        Item1(i32),
        Item2(f64),
        Item3(String),
    }
    let d=vec![
        C::Item1(3),
        C::Item2(String::from(String::from("blue")),
        C::Item3(1.23),
    ];
    */
}
