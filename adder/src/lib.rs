pub fn add_two(a: i32) -> i32 {
    a + 2
}

pub fn greeting(name: &str) -> String {
    format!("Hello {}!", name)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn exploration() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    #[ignore]
    fn another() {
        panic!("Make this test fail")
    }

    #[test]
    fn it_adds_two() {
        assert_eq!(4, add_two(2));
        // also eq
        //assert_eq!(add_two(2),4);
    }

    //添加自定义信息
    #[test]
    fn greeting_contain_name() {
        let result = greeting("Carol");
        assert!(result.contains("Carol"));
    }
}
