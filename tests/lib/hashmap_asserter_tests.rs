extern crate fluent_asserter;
use fluent_asserter::*;

#[cfg(test)]
mod hashmap_asserter_tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_has_length() {
        let mut hash_map = HashMap::<String, String>::new();
        hash_map.insert(String::from("key"), String::from("value"));

        assert_that!(&hash_map).has_length(1);

        assert_that_code!(|| assert_that!(&hash_map).has_length(2))
            .panics()
            .with_message("Expected &hash_map to have length 2, but it has 1");
    }

    #[test]
    fn test_is_empty() {
        let mut hash_map = HashMap::<String, String>::new();

        assert_that!(&hash_map).is_empty();

        hash_map.insert(String::from("key"), String::from("value"));
        assert_that_code!(|| assert_that!(&hash_map).is_empty())
            .panics()
            .with_message("Expected &hash_map to be empty, but it has length 1.")
    }

    #[test]
    fn test_is_not_empty() {
        let mut hash_map = HashMap::<String, String>::new();
        hash_map.insert(String::from("key"), String::from("value"));

        assert_that!(&hash_map).is_not_empty();

        let empty_hashmap = HashMap::<String, String>::new();
        assert_that_code!(|| assert_that!(&empty_hashmap).is_not_empty())
            .panics()
            .with_message("Expected &empty_hashmap to not to be empty, but it is.")
    }

    #[test]
    fn test_contains_key() {
        let mut hash_map = HashMap::<String, String>::new();
        hash_map.insert(String::from("key"), String::from("value"));

        assert_that!(&hash_map).contains_key(&String::from("key"));
    }

    #[test]
    fn test_contains_key_error_handling() {
        let mut hash_map = HashMap::<String, String>::new();
        hash_map.insert(String::from("key"), String::from("value"));

        assert_that_code!(|| assert_that!(&hash_map)
            .contains_key(&String::from("key2"))
            .with_value(String::from("value")))
        .panics()
        .with_message("Expected &hash_map to contain key2, but it does not.")
    }

    #[test]
    fn test_contains_key_with_value() {
        let mut hash_map = HashMap::<String, String>::new();
        hash_map.insert(String::from("key"), String::from("value"));

        assert_that!(&hash_map)
            .contains_key(&String::from("key"))
            .with_value(String::from("value"));

        assert_that_code!(|| assert_that!(&hash_map)
            .contains_key(&String::from("key"))
            .with_value(String::from("value2")))
        .panics()
        .with_message(
            "Expected &hash_map to contain key with value 'value2', but it has the value 'value'.",
        );
    }

    #[test]
    fn test_does_not_contain_key() {
        let mut hash_map = HashMap::<String, String>::new();
        hash_map.insert(String::from("key"), String::from("value"));

        assert_that!(&hash_map).does_not_contain_key(String::from("key2"));

        assert_that_code!(|| assert_that!(&hash_map).does_not_contain_key(String::from("key")))
            .panics()
            .with_message("Expected &hash_map to not to contain key, but it does.");
    }

    //TODO: normal iterator assertions works with hashmap. Somehow limit it.
}
