use std::ops::Deref;

struct MyBox<T>(T);

//实现
impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

//实现解引用
impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

enum List {
    Cons(i32, Box<List>),
    Nil,
}

use crate::List::{Cons, Nil};

#[allow(unused_variables)]
fn main() {
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    let x = 5;
    let y = Box::new(x);
    let z = MyBox::new(x);
    //let z=&x;

    assert_eq!(5, x);
    assert_eq!(5, *y);
    assert_eq!(5, *z);
    //*(y.Deref())
}
