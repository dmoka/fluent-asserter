extern crate fluent_asserter;
use fluent_asserter::*;

#[cfg(test)]
mod test_option_asserter {
    use super::*;

    #[test]
    pub fn test_is_ok() {
        let result: Result<i32, i32> = Ok(3);
        assert_that!(&result).is_ok();

        let error: Result<i32, i32> = Err(3);

        assert_that_code!(|| assert_that!(&error).is_ok())
            .panics()
            .with_message("Expected '&error' to be Ok, but found Err(3).");
    }
    #[test]
    pub fn test_is_ok_with_value() {
        let result_3: Result<i32, i32> = Ok(3);

        assert_that!(&result_3).is_ok_with_value(3);

        assert_that_code!(|| assert_that!(&result_3).is_ok_with_value(5))
            .panics()
            .with_message("Expected '&result_3' to be Ok(5), but found Ok(3).");
    }

    #[test]
    pub fn test_is_ok_with_value_for_error() {
        let error: Result<i32, i32> = Err(3);

        assert_that_code!(|| assert_that!(&error).is_ok_with_value(3))
            .panics()
            .with_message("Expected '&error' to be Ok(3), but found Err(3).");
    }

    #[test]
    pub fn test_is_error() {
        let error: Result<i32, i32> = Err(3);
        assert_that!(&error).is_error();

        let result: Result<i32, i32> = Ok(3);
        assert_that_code!(|| assert_that!(&result).is_error())
            .panics()
            .with_message("Expected '&result' to be Err(_), but found Ok(3).");
    }

    #[test]
    pub fn test_is_error_with_value() {
        let error: Result<i32, i32> = Err(3);
        assert_that!(&error).is_error_with_value(3);

        assert_that_code!(|| assert_that!(&error).is_error_with_value(5))
            .panics()
            .with_message("Expected '&error' to be Err(5), but found Err(3).");
    }

    #[test]
    pub fn test_is_error_with_value_for_ok_result() {
        let result: Result<i32, i32> = Ok(3);

        assert_that_code!(|| assert_that!(&result).is_error_with_value(5))
            .panics()
            .with_message("Expected '&result' to be Err(5), but found Ok(3).");
    }
}
