//闭包：可以捕获其所在环境的匿名函数

// 是匿名函数
// 保存为变量、作为参数
// 可以在一个地方创建闭包，然后在另一个上下文中调用闭包来完成运算
// 可从其定义的作用域捕获值

use std::thread;
use std::time::Duration;

//让结构体持有闭包，每个字段的类型都需要指明
//每个闭包的实例都有自己的一个唯一类型，即便两个闭包签名完全一样

struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    calculation: T,
    value: Option<u32>,
}

impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}

fn main() {
    println!("Hello, world!");

    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(simulated_user_specified_value, simulated_random_number);

    println!("Hello, world!");
}

// fn simulated_expensive_calculation(intensity:u32)->u32{
//     print!("Calculating slowly ...");
//     thread::sleep(Duration::from_secs(2));
//     intensity
// }

//通畅不需要显示标准参数和返回值类型，编译器可以推断出来，但是标注也是可以的
//本例省略了参数类型和返回值类型
fn generate_workout(intensity: u32, random_number: u32) {
    let mut expensive_closure = Cacher::new(|num| {
        print!("Calculating slowly ...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    //let expensive_result=simulated_expensive_calculation(intensity);

    if intensity < 25 {
        println!("Today , do {} pushups!", expensive_closure.value(intensity));
        println!("Next, do {} situps!", expensive_closure.value(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_closure.value(intensity)
            );
        }
    }
}
