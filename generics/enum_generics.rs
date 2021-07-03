enum Option<T> {
    Some(T),
    None,
}

enum Result<T, E> {
    Ok(T),
    Err(E),
}

fn main() {
    println("Hello enum generics!");
}
