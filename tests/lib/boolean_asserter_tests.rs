extern crate fluent_asserter;
use fluent_asserter::*;

#[cfg(test)]
mod boolean_asserter_tests {
    use super::*;

    #[test]
    fn test_is_true() {
        assert_that!(true).is_true();

        assert_that_code!(|| assert_that!(false).is_true()).panics();
    }

    #[test]
    fn test_is_false() {
        assert_that!(false).is_false();

        assert_that_code!(|| assert_that!(true).is_false()).panics();
    }

    #[test]
    fn test_is_equal_to() {
        assert_that!(false).is_equal_to(false);
        assert_that!(true).is_equal_to(true);

        assert_that_code!(|| assert_that!(true).is_equal_to(false)).panics();
        assert_that_code!(|| assert_that!(false).is_equal_to(true)).panics();
    }

    #[test]
    fn test_error_message() {
        let false_var = false;
        assert_that_code!(|| assert_that!(false_var).is_true())
            .panics()
            .with_message("Expected false_var to be true, but was false");

        let true_var = true;
        assert_that_code!(|| assert_that!(true_var).is_false())
            .panics()
            .with_message("Expected true_var to be false, but was true");
    }
}
