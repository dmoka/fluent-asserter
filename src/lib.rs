pub struct Checker<T> {
    value : T
}

impl Checker<&str> {

    pub fn new(value: &str) -> Checker<&str> {
        Checker {
            value
        }
    }

    pub fn is_equal_to(self, expected_value: &str) {
        println!("hello from is is equal method");
    }
}

#[macro_export]
macro_rules! assert_that {
    ($value:expr) => {
        Checker::new($value)
    };
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn given_string_when_is_equal_to_called_then_works() {
        let text =String::from("test string");

        assert_that!(&text).is_equal_to("test string");
    }

    #[test]
    fn given_str_when_is_equal_to_called_then_works() {
        let text = "test string";

        assert_that!(text).is_equal_to("test string");
    }
}
