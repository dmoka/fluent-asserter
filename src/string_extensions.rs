

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

impl StringAsserters for String {
    
    fn be_empty(&self){
        assert_that!(&self).is_empty();
    }

    fn be_equal_to(&self, expected_value: &str) {
        assert_that!(self).is_equal_to(expected_value);
    }

    fn contain(&self, expected_value_to_be_contained: &str) {
        assert_that!(self).contains(expected_value_to_be_contained);
    }

    fn start_with(&self, expected_start: &str) {
        assert_that!(self).starts_with(expected_start);
    }

    fn end_with(&self, expected_start: &str) {
        assert_that!(self).ends_with(expected_start);
    }

    fn not_be_empty(&self){
        assert_that!(self).is_not_empty();
    }

    fn have_length(&self, expected_length: usize){
        assert_that!(self).has_length(expected_length);

    }
}

#[cfg(test)]
mod test {
    use crate::panic_asserter_helper::assert_that_panics;

    use super::*;

    #[test]
    fn sanity_check() {
        let str = String::from(""); 
        &str.be_empty();

        &str.have_length(0);


        /*to problems to fix:
        A -> be_empty should be able to be called after Should().
        B -> the function can not be called second time because it has been moved. 
        */

    }
}
