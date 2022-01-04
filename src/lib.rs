//Idiomatic rust https://cheats.rs/#idiomatic-rust

//TODO: implement assert_that method when we have the stuff implemented


pub struct Asserter<T> {
    value : T
}


impl Asserter<&str> {

    pub fn new(value: &str) -> Asserter<&str> {
        Asserter {
            value
        }
    }

    //TODO: add doc for all the functions
    pub fn is_equal_to(self, expected_value: &str) {
        assert_eq!(self.value, expected_value);
    }

    pub fn contains(self, expected_value_to_be_contained: &str) {
        let is_present = self.value.contains(expected_value_to_be_contained);
        if !is_present {
            panic!("The text {} is not present in string {}", expected_value_to_be_contained, self.value)
        }
    }

    //TODO: startsWith, endsWith, isEmpty, isNothEmpty
    
    
    //TODO: has length
}

//TODO: struct assertions with lambda like in c#

#[macro_export]
macro_rules! assert_that {
    ($value:expr) => {
        Asserter::new($value)
    };
}

mod panic_asserter;

#[cfg(test)]
mod tests {
    use crate::panic_asserter::assert_that_panics;

    use super::*;


    #[test]
    fn test_is_equal_to_for_string() {
        let text = String::from("test string");

        assert_that!(&text).is_equal_to("test string");
    }

    #[test]
    fn test_is_equal_to_for_str() {
        let text = "test string";

        assert_that!(text).is_equal_to("test string");
    }

    #[test]
    fn when_string_contains_string_then_no_error() {
        let text = String::from("test string");

        assert_that!(&text).contains("st");
    }

    #[test]
    fn when_string_does_contains_string_then_panics() {
        let text = String::from("test string");

        assert_that_panics(|| assert_that!(&text).contains("asd"));
    }

    //TODO: add different assertion message? check asserteq

}

