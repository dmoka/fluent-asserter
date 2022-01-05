use super::*;

//TODO: add and

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

    pub fn starts_with(self, expected_start: &str) {
        let starts_with = self.value.starts_with(expected_start);

        if !starts_with {
            panic!("The actual text {} does not start with {}", self.value, expected_start)
        }
    }

    pub fn ends_with(self, expected_start: &str) {
        let starts_with = self.value.ends_with(expected_start);

        if !starts_with {
            panic!("The actual text {} does not end with {}", self.value, expected_start)
        }

    }

    //TODO: isEmpty, isNothEmpty, has length
}

#[cfg(test)]
mod test {
    use crate::panic_asserter::assert_that_panics;

    use super::*;


    #[test]
    fn test_is_equal_to_for_string() {
        assert_that!(&String::from("test string")).is_equal_to("test string");

        assert_that!(&String::from("bitcoin")).is_equal_to("bitcoin");
    }


    #[test]
    fn test_is_equal_to_for_str() {
        assert_that!("test string").is_equal_to("test string");
        
        assert_that!("bitcoin").is_equal_to("bitcoin");
    }

    #[test]
    fn when_string_contains_string_then_no_error() {
        assert_that!(&String::from("test string")).contains("st");

        assert_that!(&String::from("test string")).contains("ing");
    }


    #[test]
    fn when_string_does_not_contain_string_then_panics() {
        assert_that_panics(|| assert_that!(&String::from("test string")).contains("asd"));

        assert_that_panics(|| assert_that!(&String::from("bitcoin")).contains("ethereum"));
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

    //TODO: add different assertion message? check asserteq

}