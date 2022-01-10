use super::*;

trait StringAsserters { //TODO: use better naming
    fn be_empty(&self);
    
    fn be_equal_to(&self, expected_value: &str);

    fn contain(&self, expected_value_to_be_contained: &str);

    fn start_with(&self, expected_start: &str);

    fn end_with(&self, expected_start: &str);

    fn not_be_empty(&self);

    fn have_length(&self, expected_length: usize);
}

impl<'a> StringAsserters for ShouldRoot<'a, String> {
    
    fn be_empty(&self){
        assert_that!(&self.value).is_empty();
    }

    fn be_equal_to(&self, expected_value: &str) {
        assert_that!(&self.value).is_equal_to(expected_value);
    }

    fn contain(&self, expected_value_to_be_contained: &str) {
        assert_that!(&self.value).contains(expected_value_to_be_contained);
    }

    fn start_with(&self, expected_start: &str) {
        assert_that!(&self.value).starts_with(expected_start);
    }

    fn end_with(&self, expected_start: &str) {
        assert_that!(&self.value).ends_with(expected_start);
    }

    fn not_be_empty(&self){
        assert_that!(&self.value).is_not_empty();
    }

    fn have_length(&self, expected_length: usize){
        assert_that!(&self.value).has_length(expected_length);

    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::panic_asserter_helper::assert_that_panics;

    #[test]
    fn sanity_checks_for_extension_methods() {
        //The corresponding functionality is already tested comprehensively in the string_asserter test module
        //This is just a sanity check if the extension method work properly
        let str = String::from("bitcoin"); 

        str.should().not_be_empty();
        str.should().be_equal_to("bitcoin");
        str.should().contain("coin");
        str.should().start_with("bit");
        str.should().end_with("in");
        str.should().have_length(7);

        assert_that_panics(|| assert_that!(str.should().be_empty()));
    }
}
