fn main() {
    println!("Hello, world!");

    println!("Scalar 标量类型（基本数据类型）：");
    println!("================");

    let a: u8 = 3;
    println!("a:{}", a);

    let x = 2.0;
    let y: f32 = 3.0;

    let sum = 5 + 10;
    let difference = 95.5 - 4.3;
    let quotient = 56.7 / 32.2;
    let remainder = 43 % 5;

    println!("x:{}", x);
    println!("y:{}", y);
    println!("sum:{}", sum);
    println!("difference:{}", difference);
    println!("quotient:{}", quotient);
    println!("remainder:{}", remainder);

    let t = true;
    let f: bool = false;
    println!("t:{}", t);
    println!("f:{}", f);

    let c = 'z';
    let z = 'Z';
    let heart_eyed_cat = '😻';
    let ch = '我';
    println!("c:{}", c);
    println!("z:{}", z);
    println!("heart_eyed_cat:{}", heart_eyed_cat);
    println!("ch:{}", ch);

    println!("Compound types（符合数据类型）：元组和数组");
    println!("================");

    println!("元组例子");
    println!("使用模式匹配（pattern matching）来结构元组值");
    let tup = (500, 6.3, 1);
    let (x, y, z) = tup;
    println!("The value of x is :{}", x);
    println!("The value of y is :{}", y);
    println!("The value of z is :{}", z);

    println!("使用.（点号）来结构元组值");
    let x: (i32, f64, u8) = (500, 6.3, 1);
    let five_hundred = x.0;
    let six_point_three = x.1;
    let one = x.2;
    println!("The value of five_hundred is :{}", five_hundred);
    println!("The value of six_point_three is :{}", six_point_three);
    println!("The value of one is :{}", one);

    println!("数组例子");
    let a = [1, 2, 3, 4, 5];

    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];

    let b: [i32; 5] = [1, 2, 3, 4, 5];
    //let a=[3,3,3,3,3];
    let a = [3, 5];

    let first = months[0];
    let second = months[1];
    println!("first:{}", first);
    println!("second:{}", second);
}
