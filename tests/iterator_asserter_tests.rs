extern crate fluent_asserter;
use fluent_asserter::*;

mod test_number_asserter {
    use super::*;

    #[test]
    fn test_is_equal_to() { 
        assert_that!(vec!["item1"]).is_equal_to(vec!["item1"]);

        let list = vec!["item1"];
        assert_that_code!(||assert_that!(list).is_equal_to(vec!["item2"]))
            .panics()
            .with_message("Expected list to be [\"item2\"], but was [\"item1\"]")
    }
}