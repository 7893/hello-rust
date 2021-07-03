use std::fs::File;
use std::io::ErrorKind;

fn main() {
    //on linux bash shell using "RUST_BACKTRACE=1 lcargo run" to display backtrace info
    //on linux using "RUST_BACKTRACE=full cargo run" to display full backtrace info
    //panic!("carsh and burn");

    //let _f = File::open("hello.txt").unwrap();
    //let _f=File::open("hello.txt").expect("无法打开文件");

    let f = File::open("hello.txt");
    let _f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Error creating file: {:?}", e),
            },
            other_error => panic!("Error opening the file: {:?}", other_error),
        },
    };
}
