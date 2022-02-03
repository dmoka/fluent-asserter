extern crate fluent_asserter;
use fluent_asserter::*;

#[cfg(test)]
mod test_number_approx_asserter {
    use super::*;

    #[test]
    fn test_is_equal_to_approximately_for_unsigned() {
        assert_that!(3u32).is_approx_equal_to(2,1);
        assert_that!(3u32).is_approx_equal_to(3,1);
        assert_that!(3u32).is_approx_equal_to(4,1);

        assert_that_code!(||assert_that!(3u32).is_approx_equal_to(5,1)).panics();
    }
    
    #[test]
    fn test_is_equal_to_approximately_for_signed() {
        assert_that!(3i32).is_approx_equal_to(2,1);
        assert_that!(3i32).is_approx_equal_to(3,1);
        assert_that!(3i32).is_approx_equal_to(4,1);

        assert_that_code!(||assert_that!(3i32).is_approx_equal_to(5,1)).panics();
    }
    
    #[test]
    fn test_is_equal_to_approximately_for_f64() {
        assert_that!(6.14f64).is_approx_equal_to(6.16,0.0);
        assert_that!(6.14f64).is_approx_equal_to(6.16,0.02);
        assert_that!(6.14f64).is_approx_equal_to(6.14,0.00);
        assert_that!(6.14159f64).is_approx_equal_to(6.14157,0.00002);

        assert_that_code!(||assert_that!(6.14159f64).is_approx_equal_to(6.14157,0.00001))
            .panics()
            .with_message("The number '6.14159f64' is not approximately equal to '6.14157' within delta '0.00001'");

        let float_var = 6.14159f64;
        assert_that_code!(||assert_that!(float_var).is_approx_equal_to(6.14157,0.00001))
                .panics()
                .with_message("The number 'float_var' is not approximately equal to '6.14157' within delta '0.00001'");    
    }

    #[test]
    fn test_is_equal_to_approximately_for_f32() {
        assert_that!(6.14f32).is_approx_equal_to(6.16,0.0);
        assert_that!(6.14f32).is_approx_equal_to(6.16,0.02);
        assert_that!(6.14f32).is_approx_equal_to(6.14,0.00);
        assert_that!(6.14159f32).is_approx_equal_to(6.14157,0.00002);

        assert_that_code!(||assert_that!(6.14159f32).is_approx_equal_to(6.14157,0.00001)).panics();
    }

}