#[derive(Debug)]

enum Operation {
    Move { x: i32, y: i32 },
    Jump(u32),
    Attack(i32),
    Talk(String),
}

fn main() {
    let opt_talk = Operation::Talk(String::from("hello"));
    let opt_move = Operation::Move { x: 10, y: 20 };

    match opt_move {
        Operation::Talk(ref value) => {
            println!("Talk: {:?}", value);
        }
        Operation::Move { x, y } => {
            println!("Move, x: {}, y: {}", x, y);
        }
        _ => {
            // nothing
        }
    }
}
