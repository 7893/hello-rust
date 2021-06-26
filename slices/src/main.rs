fn main() {
    println!("Hello, world!");

    let mut s = String::from("hello world");
    let word = first_word(&s);
    s.clear();

    println!("the first word is: {}", word);

    let s1 = String::from("Hello, world!");
    let slice1 = &s1[0..2];
    let slice2 = &s1[..2];
    let len = s1.len();
    let slice3 = &s1[3..len];
    let slice4 = &s1[3..];
    let slice5 = &s1[..];
    let slice6 = &s1[0..len];

    println!("{}", slice1);
    println!("{}", slice2);
    println!("{}", len);
    println!("{}", slice3);
    println!("{}", slice4);
    println!("{}", slice5);
    println!("{}", slice6);
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}
