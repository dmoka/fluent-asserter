extern crate fluent_asserter;
use fluent_asserter::*;

mod test_number_asserter {
    use super::*;

    #[test]
    fn test_is_equal_to() { 
        assert_that!(vec!["s"]).is_equal_to(vec!["s"])
    }
}