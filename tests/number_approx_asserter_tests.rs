extern crate fluent_asserter;
use fluent_asserter::*;

mod common;

#[cfg(test)]
mod test_number_approx_asserter {
    use super::*;
    use common::assert_that_panics;

    #[test]
    fn test_is_equal_to_approximately_for_unsigned() {
        assert_that!(3u32).is_approx_equal_to(2,1);
        assert_that!(3u32).is_approx_equal_to(3,1);
        assert_that!(3u32).is_approx_equal_to(4,1);

        assert_that_panics(||assert_that!(3u32).is_approx_equal_to(5,1));
    }
    
    #[test]
    fn test_is_equal_to_approximately_for_signed() {
        assert_that!(3i32).is_approx_equal_to(2,1);
        assert_that!(3i32).is_approx_equal_to(3,1);
        assert_that!(3i32).is_approx_equal_to(4,1);

        assert_that_panics(||assert_that!(3i32).is_approx_equal_to(5,1));
    }
    
    #[test]
    fn test_is_equal_to_approximately_for_f64() {
        assert_that!(3.14f64).is_approx_equal_to(3.16,0.0);
        assert_that!(3.14f64).is_approx_equal_to(3.16,0.02);
        assert_that!(3.14f64).is_approx_equal_to(3.14,0.00);
        assert_that!(3.14159f64).is_approx_equal_to(3.14157,0.00002);

        assert_that_panics(||assert_that!(3.14159f64).is_approx_equal_to(3.14157,0.00001));
    }

    #[test]
    fn test_is_equal_to_approximately_for_f32() {
        assert_that!(3.14f32).is_approx_equal_to(3.16,0.0);
        assert_that!(3.14f32).is_approx_equal_to(3.16,0.02);
        assert_that!(3.14f32).is_approx_equal_to(3.14,0.00);
        assert_that!(3.14159f32).is_approx_equal_to(3.14157,0.00002);

        assert_that_panics(||assert_that!(3.14159f32).is_approx_equal_to(3.14157,0.00001));
    }

}