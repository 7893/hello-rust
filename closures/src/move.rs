fn main() {
    let x = vec![1, 2, 3];

    //把x的所有权移到了函数里面
    let equal_to_x = move |z| z == x;

    //后面再次使用会报错
    //println!("Cann't use x here:{:?}", x);

    let y = vec![1, 2, 3];
    assert!(equal_to_x(y));
}
