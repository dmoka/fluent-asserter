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
            .with_message("Expected list to be [\"item2\"], but was [\"item1\"].")
    }

    #[test]
    fn test_contains() { 
        assert_that!(vec!["item1"]).contains("item1");

        let list = vec!["item1"];
        assert_that_code!(||assert_that!(list).contains("item2"))
            .panics()
            .with_message("Expected iterator \"list\" to contain \"item2\", but it does not.");
    }

    #[test]
    fn test_contains_any() { 
        assert_that!(vec![2,3,4]).contains_all_of(&[2,4]);

        let list = vec![2,3,4];
        assert_that_code!(||assert_that!(list).contains_all_of(&[4,5,6]))
            .panics()
            .with_message("Expected iterator \"list\" to contain items [4, 5, 6], but it does not contain [5, 6].");

        let list = vec![2,3,4];
        assert_that_code!(||assert_that!(list).contains_all_of(&[5,6]))
            .panics()
            .with_message("Expected iterator \"list\" to contain items [5, 6], but it does not contain [5, 6].")
    }

    
    #[test]
    fn test_has_count() { 
        assert_that!(vec![2,3,4]).has_count(3);

        let list = vec![2,3,4];
        assert_that_code!(||assert_that!(list).has_count(4))
            .panics()
            .with_message("Expected iterator \"list\" to have count '4', but it has '3'.");
    }

    
    #[test]
    fn test_does_not_contain_any() { 
        assert_that!(vec![2,3,4]).does_not_contain_any(&[1,5]);

        let list = vec![2,3,4];
        assert_that_code!(||assert_that!(list).does_not_contain_any(&[3,4,5]))
            .panics()
            .with_message("Expected iterator \"list\" to not contain items [3, 4, 5], but it contains [3, 4].");
    }

    #[test]
    fn test_is_empty() { 
        let list = std::vec::Vec::<i32>::new();
        assert_that!(list).is_empty();

        let list_with_items = vec![32];
        assert_that_code!(||assert_that!(list_with_items).is_empty())
            .panics()
            .with_message("Expected iterator \"list_with_items\" to be empty, but it is not.");
    }

    #[test]
    fn test_is_not_empty() { 
        let list = vec![1];
        assert_that!(list).is_not_empty();

        let list = std::vec::Vec::<i32>::new();
        assert_that_code!(||assert_that!(list).is_not_empty())
            .panics()
            .with_message("Expected iterator \"list\" to be not empty, but it is.");
    }
}