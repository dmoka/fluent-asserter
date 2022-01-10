

use super::*;

trait StringAsserters {
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
    use crate::panic_asserter_helper::assert_that_panics;

    use super::*;

    #[test]
    fn sanity_check() {
        let str = String::from(""); 
        &str.should().be_empty();
        let s = &str.should().be_empty();
        let s = str.should().be_empty();

        //&str.have_length(0);

        /*to problems to fix:
        A -> be_empty should be able to be called after Should().
        */

    }
}
