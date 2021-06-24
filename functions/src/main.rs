fn main() {
    println!("Hello, function!");

    another_function(6, 3.2);

    let x = 6;

    let y = {
        let x = 3;
        //表达式结尾没有分号
        //有分号结尾的是语句
        x + 1
    };

    println!("The value of x is :{}", x);
    println!("The value of y is :{}", y);

    let z = five();
    println!("The value of z is:{}", z);

    let x = plus_one(5);
    println!("The value of x is:{}", x);
}

fn another_function(x: i32, y: f32) {
    println!("another function.");
    println!("The value of x,y is:{},{}", x, y);
}

fn five() -> i32 {
    //函数的返回值等于最后一个表达式的值
    //没有分号的是表达式，所以5后面没有分号
    5
}

fn plus_one(x: i32) -> i32 {
    x + 2
}
