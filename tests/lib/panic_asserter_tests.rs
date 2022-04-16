extern crate fluent_asserter;
use fluent_asserter::*;

mod panic_asserter_tests {
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

        let failing_assertion = || assert_that_code!(|| panic!("WAGMI")).does_not_panic();
        assert_that_code!(failing_assertion)
            .panics()
            .with_message("Expected code to panic, but it does not panic.")
    }

    #[test]
    fn test_panics_with_message() {
        assert_that_code!(|| panic!("specific panic message"))
            .panics()
            .with_message("specific panic message");
    }

    #[test]
    #[should_panic(
        expected = "Expected a panic message 'expected panic message', but found 'specific panic message'"
    )]
    fn test_panics_when_panic_assertion_fails() {
        assert_that_code!(|| panic!("specific panic message"))
            .panics()
            .with_message("expected panic message");
    }

    #[test]
    #[should_panic(expected = "There was no panic, but it was expected.")]
    fn test_that_code_panics_whereas_not() {
        assert_that_code!(|| println!("specific panic message"))
            .panics()
            .with_message("another expected panic message");
    }

    #[test]
    fn test_that_panic_contains_panic_message() {
        assert_that_code!(|| panic!("specific panic message"))
            .panics()
            .with_having_message("panic message");
    }

    #[test]
    #[should_panic(
        expected = "The text 'bitcoin' is not present in the panic message 'specific panic message'"
    )]
    fn test_that_panic_contains_message_whereas_not() {
        assert_that_code!(|| panic!("specific panic message"))
            .panics()
            .with_having_message("bitcoin");
    }

    #[allow(unreachable_code)]
    #[test]
    fn test_that_provider_closure_panics() {
        let panicking_provider = || {
            panic!("Provider panicked");
            return 3;
        };

        assert_that_code!(|| panicking_provider())
            .panics()
            .with_message("Provider panicked");
    }
}
