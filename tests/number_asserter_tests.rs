extern crate fluent_asserter;
use fluent_asserter::*;

mod common;

mod test_number_asserter {
    use super::*;
    use common::assert_that_panics;

    #[test]
    fn test_is_smaller_than() { 
        assert_that!(3).is_smaller_than(4);
        assert_that!(21.0).is_smaller_than(21.1);

        assert_that_panics(||assert_that!(5).is_smaller_than(4));
        assert_that_panics(||assert_that!(10).is_smaller_than(10));
    }

    #[test]
    fn test_is_smaller_than_or_equal_to() { 
        assert_that!(4).is_smaller_than_or_equal_to(4);
        assert_that!(21.0).is_smaller_than_or_equal_to(21.1);

        assert_that_panics(||assert_that!(4.01).is_smaller_than_or_equal_to(4.0));
    }

    #[test]
    fn test_is_greater_than() {
        assert_that!(1).is_greater_than(0);
        assert_that!(15).is_greater_than(14);

        assert_that_panics(||assert_that!(15).is_greater_than(15));
        assert_that_panics(||assert_that!(10).is_greater_than(15));
    }

    #[test]
    fn test_is_greater_than_or_equal_to() {
        assert_that!(11).is_greater_than_or_equal_to(10);
        assert_that!(10).is_greater_than_or_equal_to(10);

        assert_that_panics(||assert_that!(9).is_greater_than(10));
    }

    #[test]
    fn test_is_in_range() {
        assert_that!(3).is_in_range(1,10);
        assert_that!(1).is_in_range(1,10);
        assert_that!(10).is_in_range(1,10);

        assert_that_panics(||assert_that!(0).is_in_range(1,10));
        assert_that_panics(||assert_that!(11).is_in_range(1,10));
    }

    #[test]
    fn test_is_not_in_range() {
        assert_that!(0).is_not_in_range(1,10);
        assert_that!(11).is_not_in_range(1,10);

        assert_that_panics(||assert_that!(1).is_not_in_range(1,10));
        assert_that_panics(||assert_that!(2).is_not_in_range(1,10));
        assert_that_panics(||assert_that!(10).is_not_in_range(1,10));
    }
}