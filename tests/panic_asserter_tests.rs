extern crate fluent_asserter;
use fluent_asserter::*;

mod common;

mod test {
    use common::assert_that_panics;

    use super::*;

    #[test]
    fn test_assert_that_panics() {
        assert_that_code!(|| panic!("error")).panics();

        assert_that_code!(|| panic!("validation error")).panics();
    }

    #[test]
    fn test_assert_that_does_not_panics() {
        assert_that_code!(|| println!("gm")).does_not_panic();

        assert_that_code!(|| println!("WAGMI")).does_not_panic();

        let failing_assertion = ||assert_that_code!(|| panic!("WAGMI")).does_not_panic();
        assert_that_code!(failing_assertion).panics().with_message("Expected code to panic, but it does not panic.")
    }

    #[test]
    fn test_panics_with_message() {
        assert_that_code!(|| panic!("specific panic message"))
                                .panics()
                                .with_message("specific panic message");

        assert_that_panics(||assert_that_code!(|| panic!("specific panic message"))
                                    .panics()
                                    .with_message("another expected panic message"));
    }

}