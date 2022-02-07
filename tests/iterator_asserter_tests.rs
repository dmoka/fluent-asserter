extern crate fluent_asserter;
use fluent_asserter::*;
use fluent_asserter::prelude::*;

mod test_iterator_asserter {
    use super::*;

    #[test]
    fn test_is_equal_to() { 
        assert_that!(vec!["item1"]).is_equal_to(vec!["item1"]);

        let list = vec!["item1"];
        assert_that_code!(||assert_that!(list).is_equal_to(vec!["item2"]))
            .panics()
            .with_message("Expected list to be [\"item2\"], but was [\"item1\"]")
    }

    #[test]
    fn test_contains() { 
        assert_that!(vec!["item1"]).contains("item1");

        let list = vec!["item1"];
        assert_that_code!(||assert_that!(list).contains("item2"))
            .panics()
            .with_message("Expected iterator \"list\" to contain \"item2\", but it does not");
    }

    #[test]
    fn test_contains_any() { 
        assert_that!(vec![2,3,4]).contains_any(&[2,4]);

        let list = vec![2,3,4];
        assert_that_code!(||assert_that!(list).contains_any(&[4,5,6]))
            .panics()
            .with_message("Expected iterator \"list\" to contain items [4, 5, 6], but it does not contain [5, 6]");

        let list = vec![2,3,4];
        assert_that_code!(||assert_that!(list).contains_any(&[5,6]))
            .panics()
            .with_message("Expected iterator \"list\" to contain items [5, 6], but it does not contain [5, 6]")
    }

    
    #[test]
    fn test_has_count() { 
        assert_that!(vec![2,3,4]).has_count(3);

        let list = vec![2,3,4];
        assert_that_code!(||assert_that!(list).has_count(4))
            .panics()
            .with_message("Expected iterator \"list\" to have count '4', but it has '3'");
    }
}