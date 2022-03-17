pub fn yolo() -> String {
    "yolo cash money".to_string()
}

pub fn panic_time(b: bool) {
    if b {
        panic!("panic time!!!");
    }
}

mod doggo {
    static ARF: &str = "Arf Arf Arf!";

    pub fn bark() ->  &'static str {
        ARF
    }

    pub fn print_bark() {
        println!("{}", bark());
    }
}

pub fn greeting (s: &str) -> String {
    println!("{}", "Hello from the other side!");
    "Hello ".to_string() + s
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_arf() {
        assert_eq!("Arf Arf Arf!", doggo::bark());
    }

    #[test]
    fn yolo_test_suite_1() {
        assert_eq!(yolo(), "yolo cash money");
    }

    #[test]
    #[ignore]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(
            result.contains("Carol"),
            "Greeting did not contain name, value was `{}`",
            result
        );
    }

    #[test]
    #[should_panic(expected = "panic time")]
    fn panic_test_suite_1() {
        panic_time(true)
    }
}
