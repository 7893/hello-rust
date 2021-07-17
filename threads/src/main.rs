use std::thread;
use std::time::Duration;

fn main() {
    println!("Hello, world!");

    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("第 {} 次执行，来自分线程", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("第 {} 次执行，来自主线程", i);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();
}
