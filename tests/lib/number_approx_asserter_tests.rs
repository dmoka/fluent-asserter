extern crate fluent_asserter;
use fluent_asserter::*;

#[cfg(test)]
mod number_approx_asserter_tests {
    use fluent_asserter::prelude::IsApproxEqual;

    use super::*;

    #[test]
    fn test_is_equal_to_approximately_for_unsigned() {
        assert_that!(3u8).is_approx_equal(2, 1);
        assert_that!(3u16).is_approx_equal(2, 1);
        assert_that!(3u32).is_approx_equal(2, 1);
        assert_that!(3u32).is_approx_equal(3, 1);
        assert_that!(3u32).is_approx_equal(4, 1);
        assert_that!(3u64).is_approx_equal(4, 1);
        assert_that!(3u128).is_approx_equal(4, 1);

        assert_that_code!(|| assert_that!(3u32).is_approx_equal(5, 1)).panics();
    }

    #[test]
    fn test_is_equal_to_approximately_for_signed() {
        assert_that!(3i8).is_approx_equal(2, 1);
        assert_that!(3i16).is_approx_equal(2, 1);
        assert_that!(3i32).is_approx_equal(2, 1);
        assert_that!(3i32).is_approx_equal(3, 1);
        assert_that!(3i32).is_approx_equal(4, 1);
        assert_that!(3i64).is_approx_equal(4, 1);
        assert_that!(3i128).is_approx_equal(4, 1);

        assert_that_code!(|| assert_that!(3i32).is_approx_equal(5, 1)).panics();
    }

    #[test]
    fn test_is_equal_to_approximately_for_f64() {
        assert_that!(6.14f64).is_approx_equal(6.16, 0.0);
        assert_that!(6.14f64).is_approx_equal(6.16, 0.02);
        assert_that!(6.14f64).is_approx_equal(6.14, 0.00);
        assert_that!(6.14159f64).is_approx_equal(6.14157, 0.00002);

        assert_that_code!(||assert_that!(6.14159f64).is_approx_equal(6.14157,0.00001))
            .panics()
            .with_message("The number '6.14159f64' is not approximately equal to '6.14157' within delta '0.00001'");

        let float_var = 6.14159f64;
        assert_that_code!(||assert_that!(float_var).is_approx_equal(6.14157,0.00001))
                .panics()
                .with_message("The number 'float_var' is not approximately equal to '6.14157' within delta '0.00001'");
    }

    #[test]
    fn test_is_equal_to_approximately_for_f32() {
        assert_that!(6.14f32).is_approx_equal(6.16, 0.0);
        assert_that!(6.14f32).is_approx_equal(6.16, 0.02);
        assert_that!(6.14f32).is_approx_equal(6.14, 0.00);
        assert_that!(6.14159f32).is_approx_equal(6.14157, 0.00002);

        assert_that_code!(|| assert_that!(6.14159f32).is_approx_equal(6.14157, 0.00001)).panics();
    }
}
