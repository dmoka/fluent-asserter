extern crate fluent_asserter;
use fluent_asserter::*;

#[cfg(test)]
mod test_option_asserter {
    use super::*;

    #[test]
    fn test_is_some_without_value() {
        let option = Option::Some(3);
        assert_that!(option).is_some();

        let option = Option::<i32>::None;

        assert_that_code!(|| assert_that!(option).is_some())
            .panics()
            .with_message("Expected 'option' to be Some(_), but found None");
    }

    #[test]
    fn test_is_some_with_some() {
        let option = Option::Some(3);
        assert_that!(option).is_some_with_value(3);

        assert_that_code!(|| assert_that!(option).is_some_with_value(4))
            .panics()
            .with_message("Expected 'option' to be Some(4), but found Some(3).");
    }

    #[test]
    fn test_is_some_with_none() {
        let option = Option::None;

        assert_that_code!(|| assert_that!(option).is_some_with_value(4))
            .panics()
            .with_message("Expected 'option' to be Some(4), but found None.");
    }

    #[test]
    fn test_is_none_with_none() {
        let option = Option::<String>::None;
        assert_that!(option).is_none();
    }

    #[test]
    fn test_is_none_with_some() {
        let option = Option::Some(3);

        assert_that_code!(|| assert_that!(option).is_none())
            .panics()
            .with_message("Expected 'option' to be None, but found Some(3).");
    }
}
