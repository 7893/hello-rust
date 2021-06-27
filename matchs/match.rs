enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => {
            println!("Luck Boy.");
            5
        }
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn main() {
    let yijiao = Coin::Penny;
    let wumao = Coin::Nickel;

    println!("{:?},{:?}", value_in_cents(yijiao), value_in_cents(wumao));
    println!("Hello, world!");
}
