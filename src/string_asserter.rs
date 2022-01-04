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

    //TODO: startsWith, endsWith, isEmpty, isNothEmpty, has length
}

mod panic_asserter;

#[cfg(test)]
mod test {
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
    fn when_string_contains_string_then_no_error() {
        let text = String::from("ring");

        assert_that!(&text).contains("st");
    }

    #[test]
    fn when_string_does_contains_string_then_panics() {
        let text = String::from("test string");

        assert_that_panics(|| assert_that!(&text).contains("asd"));
    }

    //TODO: add different assertion message? check asserteq

}