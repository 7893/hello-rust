pub fn add_two(a: i32) -> i32 {
    a + 2
}

pub fn gerrting(_name: &str) -> String {
    format!("Hello {}!", _name)
    //format!("Hello")
}

pub struct Guess {
    value: u32
}

impl Guess {
    pub fn new(value: u32) -> Guess {
        if value < 1 {
            panic!(
                "Guess value must be greater than or equal to 1, got {}",
                value
            )
        } else if value > 100 {
            panic!(
                "Guess value must be less than or equal to 100, got {}",
                value
            )
        }
        Guess { value }
    }
}

#[cfg(test)]
mod tests {
    //导入外部模块所有内容
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    //由于有了Err 所以如果发生错误则会直接返回 Err，不应该标注 should_panic 
    fn it_works_two() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }

    #[test]
    fn it_adds_two() {
        assert_eq!(4, add_two(2));
        assert_ne!(5, add_two(2));
    }

    #[test]
    fn greetings_contain_name() {
        let result = gerrting("Carol");
        assert!(
            result.contains("Carol"),
            "Greeting didn't contain name, value was '{}'",
            result
        );
    }

    #[test]
    #[should_panic(expected = "Guess value must be less than or equal to 100")]
    fn greater_than_100() {
        Guess::new(200);
    }
}
