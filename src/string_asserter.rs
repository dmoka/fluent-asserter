use super::*;
use std::fmt;

//TODO: add and
impl<T> Asserter<T> where T : Into<String> + Clone{ //TODO: Display is also implemented for int,etc, so it would work
    pub fn contains(&self, expected_value_to_be_contained: &str) {
        let string = self.value.clone().into();
        let is_present = string.contains(expected_value_to_be_contained);
        
        if !is_present {
            panic!("The text {} is not present in string {}", expected_value_to_be_contained, string)
        }
    }

    pub fn starts_with(&self, expected_start: &str) {
        let string = self.value.clone().into();
        let starts_with = string.starts_with(expected_start);

        if !starts_with {
            panic!("The actual text {} does not start with {}", string, expected_start)
        }
    }

    pub fn ends_with(&self, expected_start: &str) {
        let string = self.value.clone().into();
        let ends_with = string.ends_with(expected_start);

        if !ends_with {
            panic!("The actual text {} does not end with {}", string, expected_start)
        }
    }

    pub fn is_empty(&self){
        let string = self.value.clone().into();

        if !string.is_empty() {
            panic!("The text {} is not empty", string)
        }
    }

    pub fn is_not_empty(&self){
        let string = self.value.clone().into();

        if string.is_empty() {
            panic!("The text {} is empty", string)
        }
    }

    pub fn has_length(&self, expected_length: usize){
        let string = self.value.clone().into();
        let len = string.len();

        assert_eq!(len, expected_length); //TODO: use custom panic
    }
}

#[cfg(test)]
mod test {
    use crate::panic_asserter_helper::assert_that_panics;

    use super::*;

    #[test]
    fn test_is_equal_to_for_string() {
        assert_that!(&String::from("test string")).is_equal_to(&String::from("test string"));
        assert_that!(&String::from("bitcoin")).is_equal_to(&String::from("bitcoin"));

        assert_that_panics(|| assert_that!(&String::from("test string")).is_equal_to(&String::from("test")));
        assert_that_panics(|| assert_that!(&String::from("bitcoin")).is_equal_to(&String::from("ethereum")));
    }


    #[test]
    fn test_is_equal_to_for_str() {
        assert_that!("test string").is_equal_to("test string");
        assert_that!("bitcoin").is_equal_to("bitcoin");
        
        assert_that_panics(|| assert_that!("test string").is_equal_to("string"));
        assert_that_panics(|| assert_that!("bitcoin").is_equal_to("ethereum"));
    }

    #[test]
    fn test_string_contains() {
        assert_that!(&String::from("test string")).contains("st");
        assert_that!(&String::from("bitcoin")).contains("co");

        assert_that_panics(|| assert_that!(&String::from("test string")).contains("asd"));
        assert_that_panics(|| assert_that!(&String::from("bitcoin")).contains("eth"));
    }

    #[test]
    fn test_string_starts_with() {
        assert_that!(&String::from("test string")).starts_with("te");
        assert_that!(&String::from("bitcoin")).starts_with("bitcoin");

        assert_that_panics(|| assert_that!(&String::from("test string")).contains("asd"));
        assert_that_panics(|| assert_that!(&String::from("bitcoin")).contains("eth"));
    }

    #[test]
    fn test_string_ends_with() {
        assert_that!(&String::from("test string")).ends_with("ng");
        assert_that!(&String::from("bitcoin")).ends_with("coin");

        assert_that_panics(|| assert_that!(&String::from("test string")).ends_with("asd"));
        assert_that_panics(|| assert_that!(&String::from("bitcoin")).ends_with("eth"));
    }

    #[test]
    fn test_string_is_empty() {
        assert_that!(&String::from("")).is_empty();
        assert_that!("").is_empty();

        assert_that_panics(|| assert_that!(&String::from("test string")).is_empty());
    }

    #[test]
    fn test_has_length() {
        assert_that!(&String::from("bitcoin")).has_length(7);
        assert_that!("ethereum").has_length(8);

        assert_that_panics(|| assert_that!(&String::from("test string")).has_length(1));
    }

    #[test]
    fn test_string_is_not_empty() {
        assert_that!(&String::from("test string")).is_not_empty();
        assert_that!("bitcoin").is_not_empty();

        assert_that_panics(|| assert_that!(&String::from("")).is_not_empty());
    }


    //TODO: add different assertion message? check asserteq

}