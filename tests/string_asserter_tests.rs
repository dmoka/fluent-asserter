extern crate fluent_asserter;
use fluent_asserter::*;

mod common;

mod test_string_asserter {
    use super::*;
    use common::assert_that_panics;

    #[test]
    fn test_is_equal_to_for_string() {
        assert_that!(&String::from("test string")).is_equal_to(&String::from("test string"));
        assert_that!(&String::from("bitcoin")).is_equal_to(&String::from("bitcoin"));

        assert_that_panics(|| assert_that!(&String::from("test string")).is_equal_to(&String::from("test")));
        assert_that_panics(|| assert_that!(&String::from("bitcoin")).is_equal_to(&String::from("ethereum")));
    }


    #[test]
    fn test_is_equal_to_for_str() {
        assert_that!("test string").is_equal_to("test string");
        assert_that!("bitcoin").is_equal_to("bitcoin");
        
        assert_that_panics(|| assert_that!("test string").is_equal_to("string"));
        assert_that_panics(|| assert_that!("bitcoin").is_equal_to("ethereum"));
    }

    #[test]
    fn test_string_contains() {
        assert_that!(&String::from("test string")).contains("st");
        assert_that!(&String::from("bitcoin")).contains("co");

        assert_that_panics(|| assert_that!(&String::from("test string")).contains("asd"));
        assert_that_panics(|| assert_that!(&String::from("bitcoin")).contains("eth"));
    }

    #[test]
    fn test_string_starts_with() {
        assert_that!(&String::from("test string")).starts_with("te");
        assert_that!(&String::from("bitcoin")).starts_with("bitcoin");

        assert_that_panics(|| assert_that!(&String::from("test string")).contains("asd"));
        assert_that_panics(|| assert_that!(&String::from("bitcoin")).contains("eth"));
    }

    #[test]
    fn test_string_ends_with() {
        assert_that!(&String::from("test string")).ends_with("ng");
        assert_that!(&String::from("bitcoin")).ends_with("coin");

        assert_that_panics(|| assert_that!(&String::from("test string")).ends_with("asd"));
        assert_that_panics(|| assert_that!(&String::from("bitcoin")).ends_with("eth"));
    }

    #[test]
    fn test_string_is_empty() {
        assert_that!(&String::from("")).is_empty();
        assert_that!("").is_empty();

        assert_that_panics(|| assert_that!(&String::from("test string")).is_empty());
    }

    #[test]
    fn test_has_length() {
        assert_that!(&String::from("bitcoin")).has_length(7);
        assert_that!("ethereum").has_length(8);

        assert_that_panics(|| assert_that!(&String::from("test string")).has_length(1));
    }

    #[test]
    fn test_string_is_not_empty() {
        assert_that!(&String::from("test string")).is_not_empty();
        assert_that!("bitcoin").is_not_empty();

        assert_that_panics(|| assert_that!(&String::from("")).is_not_empty());
    }

    #[test]
    fn test_contains_all() {
        assert_that!("bitcoin ethereum solana").contains_all(&["ethereum", "bitcoin", "solana"]);

        assert_that_panics(|| assert_that!("bitcoin ethereum solana").contains_all(&["ethereum", "bitcoin", "solana", "polygon"]));
    }

    #[test]
    fn test_contains_any() {
        assert_that!("bitcoin ethereum solana").contains_any(&["solana"]);

        assert_that_panics(|| assert_that!("bitcoin ethereum solana").contains_any(&["tezos", "litecoin", "luna"]));
    }

    #[test]
    fn test_is_equal_to_panics_with_message() {
        let code = ||assert_that!("test1").is_equal_to("test2");

        assert_that_code!(code)
            .panics()
            .with_message(
                "Expected \"test1\" to be 'test2', but was 'test1'");
    }

    #[test]
    fn test_is_equal_to_panics_with_message_containing_variable_name() {
        let string_var = "test1";
        let code = ||assert_that!(string_var).is_equal_to("test2");

        assert_that_code!(code)
            .panics()
            .with_message(
                "Expected string_var to be 'test2', but was 'test1'");
    }
}
