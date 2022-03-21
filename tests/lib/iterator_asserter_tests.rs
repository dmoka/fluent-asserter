extern crate fluent_asserter;
use fluent_asserter::prelude::*;

mod iterator_asserter_tests {
    use super::*;

    #[derive(Clone)]
    struct TestObject {
        name: String,
        age: i32,
    }

    #[test]
    fn test_is_equal_to() {
        assert_that!(vec!["item1"]).is_equal_to(vec!["item1"]);

        let list = vec!["item1"];
        assert_that_code!(|| assert_that!(list).is_equal_to(vec!["item2"]))
            .panics()
            .with_message("Expected list to be [\"item2\"], but was [\"item1\"].")
    }

    #[test]
    fn test_contains() {
        assert_that!(vec!["item1"]).contains("item1");

        let list = vec!["item1"];
        assert_that_code!(|| assert_that!(list).contains("item2"))
            .panics()
            .with_message("Expected iterator \"list\" to contain \"item2\", but it does not.");
    }

    #[test]
    fn test_contains_any() {
        assert_that!(vec![2, 3, 4]).contains_all(&[2, 4]);

        let list = vec![2, 3, 4];
        assert_that_code!(||assert_that!(list).contains_all(&[4,5,6]))
            .panics()
            .with_message("Expected iterator \"list\" to contain items [4, 5, 6], but it does not contain [5, 6].");

        let list = vec![2, 3, 4];
        assert_that_code!(||assert_that!(list).contains_all(&[5,6]))
            .panics()
            .with_message("Expected iterator \"list\" to contain items [5, 6], but it does not contain [5, 6].")
    }

    #[test]
    fn test_has_count() {
        assert_that!(vec![2, 3, 4]).has_count(3);

        let list = vec![2, 3, 4];
        assert_that_code!(|| assert_that!(list).has_count(4))
            .panics()
            .with_message("Expected iterator \"list\" to have count '4', but it has '3'.");
    }

    #[test]
    fn test_does_not_contain_any() {
        assert_that!(vec![2, 3, 4]).does_not_contain_any(&[1, 5]);

        let list = vec![2, 3, 4];
        assert_that_code!(||assert_that!(list).does_not_contain_any(&[3,4,5]))
            .panics()
            .with_message("Expected iterator \"list\" to not contain items [3, 4, 5], but it contains [3, 4].");
    }

    #[test]
    fn test_is_empty() {
        let list = std::vec::Vec::<i32>::new();
        assert_that!(list).is_empty();

        let list_with_items = vec![32];
        assert_that_code!(|| assert_that!(list_with_items).is_empty())
            .panics()
            .with_message("Expected iterator \"list_with_items\" to be empty, but it is not.");
    }

    #[test]
    fn test_is_not_empty() {
        let list = vec![1];
        assert_that!(list).is_not_empty();

        let list = std::vec::Vec::<i32>::new();
        assert_that_code!(|| assert_that!(list).is_not_empty())
            .panics()
            .with_message("Expected iterator \"list\" to be not empty, but it is.");
    }

    #[test]
    fn test_is_equal_respectively_with_simple_single_type() {
        let list = vec![2i32];
        assert_that!(list).satisfies_respectively(vec![Box::new(|item1| {
            assert_that!(item1).is_equal_to(&2);
        })]);
    }

    #[test]
    fn test_satisfies_respectively_with_complex_single_type() {
        let list: Vec<TestObject> = vec![TestObject {
            name: String::from("name"),
            age: 3,
        }];
        assert_that!(list).satisfies_respectively(vec![Box::new(|item1: &TestObject| {
            assert_that!(&item1.name).is_equal_to(&String::from("name"));
            assert_that!(item1.age).is_equal_to(3);
        })]);
    }

    //TODO: S - use more meaningful names in this class test doubles
    #[test]
    fn test_satisfies_respectively_with_complex_multiple_types() {
        let list: Vec<TestObject> = vec![
            TestObject {
                name: String::from("name1"),
                age: 5,
            },
            TestObject {
                name: String::from("name2"),
                age: 11,
            },
        ];

        assert_that!(list).satisfies_respectively(vec![
            Box::new(|item1: &TestObject| {
                assert_that!(&item1.name).is_equal_to(&String::from("name1"));
                assert_that!(&item1.age).is_equal_to(&5);
            }),
            Box::new(|item2: &TestObject| {
                assert_that!(&item2.name).is_equal_to(&String::from("name2"));
                assert_that!(&item2.age).is_equal_to(&11);
            }),
        ]);
    }

    #[test]
    fn test_satisfies_respectively_with_for_multiple_simple_types_by_using_helper_macro() {
        let list = vec![1u32, 5u32];

        assert_that!(list).satisfies_respectively(with_asserters!(
            |item1: &u32| {
                assert_that!(item1).is_equal_to(&1);
                assert_that!(item1).is_smaller_than(&2);
            },
            |item2: &u32| {
                assert_that!(item2).is_equal_to(&5);
                assert_that!(item2).is_greater_than(&4);
            }
        ));
    }

    #[test]
    fn test_satisfies_respectively_with_for_multiple_items_by_using_helper_macro() {
        let list: Vec<TestObject> = vec![
            TestObject {
                name: String::from("name1"),
                age: 5,
            },
            TestObject {
                name: String::from("name2"),
                age: 11,
            },
        ];

        assert_that!(list).satisfies_respectively(with_asserters!(
            |item1: &TestObject| {
                assert_that!(&item1.age).is_equal_to(&5);
                assert_that!(&item1.name).is_equal_to(&String::from("name1"));
            },
            |item2: &TestObject| {
                assert_that!(&item2.name).is_equal_to(&String::from("name2"));
                assert_that!(&item2.age).is_equal_to(&11);
            }
        ));
    }

    #[test]
    fn test_that_satisfies_respectively_fails() {
        let list: Vec<TestObject> = vec![TestObject {
            name: String::from("name1"),
            age: 5,
        }];

        assert_that_code!(
            || assert_that!(list).satisfies_respectively(with_asserters!(|item1: &TestObject| {
                assert_that!(&item1.age).is_equal_to(&6);
            }))
        )
        .panics()
        .with_message("Expected &item1.age to be 6, but was 5.");
    }

    #[test]
    fn test_that_satisfies_respectively_fails_when_different_size_if_asserters_specified() {
        let list: Vec<TestObject> = vec![
            TestObject {
                name: String::from("name1"),
                age: 5,
            },
            TestObject {
                name: String::from("name2"),
                age: 11,
            },
        ];

        assert_that_code!(
            || assert_that!(list).satisfies_respectively(with_asserters!(|item1: &TestObject| {
                assert_that!(&item1.age).is_equal_to(&6);
            }))
        )
        .panics()
        .with_message("Expected number of items to be 1, but was 2.");
    }
}
