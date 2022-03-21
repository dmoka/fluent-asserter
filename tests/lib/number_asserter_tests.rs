extern crate fluent_asserter;
use fluent_asserter::*;

mod number_asserter_tests {
    use super::*;

    #[test]
    fn test_is_smaller_than() {
        assert_that!(3).is_smaller_than(4);
        assert_that!(21.0).is_smaller_than(21.1);

        assert_that_code!(|| assert_that!(5).is_smaller_than(4)).panics();
        assert_that_code!(|| assert_that!(10).is_smaller_than(10)).panics();
    }

    #[test]
    fn test_is_smaller_than_or_equal_to() {
        assert_that!(4).is_smaller_than_or_equal_to(4);
        assert_that!(21.0).is_smaller_than_or_equal_to(21.1);

        assert_that_code!(|| assert_that!(4.01).is_smaller_than_or_equal_to(4.0)).panics();
    }

    #[test]
    fn test_is_greater_than() {
        assert_that!(1).is_greater_than(0);
        assert_that!(15).is_greater_than(14);

        assert_that_code!(|| assert_that!(15).is_greater_than(15)).panics();
        assert_that_code!(|| assert_that!(10).is_greater_than(15)).panics();
    }

    #[test]
    fn test_is_greater_than_or_equal_to() {
        assert_that!(11).is_greater_than_or_equal_to(10);
        assert_that!(10).is_greater_than_or_equal_to(10);

        assert_that_code!(|| assert_that!(9).is_greater_than(10)).panics();
    }

    #[test]
    fn test_is_in_range() {
        assert_that!(3).is_in_range(1, 10);
        assert_that!(1).is_in_range(1, 10);
        assert_that!(10).is_in_range(1, 10);

        assert_that_code!(|| assert_that!(0).is_in_range(1, 10)).panics();
        assert_that_code!(|| assert_that!(11).is_in_range(1, 10)).panics();
    }

    #[test]
    fn test_is_not_in_range() {
        assert_that!(0).is_not_in_range(1, 10);
        assert_that!(11).is_not_in_range(1, 10);

        assert_that_code!(|| assert_that!(1).is_not_in_range(1, 10)).panics();
        assert_that_code!(|| assert_that!(2).is_not_in_range(1, 10)).panics();
        assert_that_code!(|| assert_that!(10).is_not_in_range(1, 10)).panics();
    }

    #[test]
    fn test_that_error_message_contains_variable_name() {
        let variable_name = 1;

        assert_that_code!(|| assert_that!(variable_name).is_equal_to(2))
            .panics()
            .with_message("Expected variable_name to be 2, but was 1.")
    }

    #[test]
    fn test_that_error_message_contains_inline_value() {
        assert_that_code!(|| assert_that!(1).is_equal_to(2))
            .panics()
            .with_message("Expected 1 to be 2, but was 1.");

        assert_that_code!(|| assert_that!(1u32).is_equal_to(2u32))
            .panics()
            .with_message("Expected 1u32 to be 2, but was 1.")
    }
}
