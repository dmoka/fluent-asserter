use super::*;

//TODO: add and

impl Asserter<&str> {

    pub fn new(value: &str) -> Asserter<&str> {
        Asserter {
            value
        }
    }

    //TODO: add doc for all the functions
    pub fn is_equal_to(&self, expected_value: &str) {
        assert_eq!(self.value, expected_value);
    }

    pub fn contains(&self, expected_value_to_be_contained: &str) {
        let is_present = self.value.contains(expected_value_to_be_contained);
        if !is_present {
            panic!("The text {} is not present in string {}", expected_value_to_be_contained, self.value)
        }
    }

    pub fn starts_with(&self, expected_start: &str) {
        let starts_with = self.value.starts_with(expected_start);

        if !starts_with {
            panic!("The actual text {} does not start with {}", self.value, expected_start)
        }
    }

    pub fn ends_with(&self, expected_start: &str) {
        let ends_with = self.value.ends_with(expected_start);

        if !ends_with {
            panic!("The actual text {} does not end with {}", self.value, expected_start)
        }
    }

    pub fn is_empty(&self){
        if !self.value.is_empty() {
            panic!("The text {} is not empty", self.value)
        }
    }

    pub fn is_not_empty(&self){
        if self.value.is_empty() {
            panic!("The text {} is empty", self.value)
        }
    }

    pub fn has_length(&self, expected_length: usize){
        let len = self.value.len();

        assert_eq!(len, expected_length); //TODO: use custom panic
    }
}

#[cfg(test)]
mod test {
    use crate::panic_asserter_helper::assert_that_panics;

    use super::*;

    #[test]
    fn test_is_equal_to_for_string() {
        assert_that!(&String::from("test string")).is_equal_to("test string");
        assert_that!(&String::from("bitcoin")).is_equal_to("bitcoin");

        assert_that_panics(|| assert_that!(&String::from("test string")).is_equal_to("test"));
        assert_that_panics(|| assert_that!(&String::from("bitcoin")).is_equal_to("ethereum"));
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