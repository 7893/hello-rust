use std::thread;

fn main() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("这是分线程:{:?}", v);
    });

    //drop(v);

    handle.join().unwrap();
}
