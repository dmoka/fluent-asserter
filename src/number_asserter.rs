use super::*;
use std::ops::Neg;
use num::{traits::Zero, Integer, Signed, Bounded, Float}; // 0.2.0

//TODO: here use the trait from num crate, like in the approximately method

//approx: Signed + Integer + Zero + Neg + Bounded + Copy
//old asserter: Copy + PartialOrd + std::ops::Sub  + std::fmt::Debug + std::fmt::Display
//mixed:Zero + Neg + Signed + Bounded + Copy + PartialOrd + std::fmt::Debug + std::fmt::Display

impl<T> Asserter<T> where T : Copy + PartialOrd + std::ops::Sub  + Default + std::fmt::Debug + std::fmt::Display {
    pub fn is_smaller_than(self, expected: T) {
        if self.value >= expected {
            panic!("The value {} is not smaller than {}", self.value, expected)
        }
    }

    pub fn is_smaller_than_or_equal_to(self, expected: T) {
        if self.value > expected {
            panic!("The value {} is not smaller than or equal to {}", self.value, expected)
        }
    }

    pub fn is_greater_than(self, expected: T) {
        if self.value <= expected {
            panic!("The value {} is not greater than {}", self.value, expected)
        }
    }

    pub fn is_greater_than_or_equal_to(self, expected: T) {
        if self.value < expected {
            panic!("The value {} is not greater than {}", self.value, expected)
        }
    }

    pub fn is_in_range(self, expected_lower_range: T, expected_upper_range : T) {
        if self.value < expected_lower_range || self.value > expected_upper_range {
            panic!("The value {} is not in range [{},{}]", self.value, expected_lower_range,expected_upper_range);
        }
    }

    pub fn is_not_in_range(self, expected_lower_range: T, expected_upper_range : T) {
        if self.value >= expected_lower_range && self.value <= expected_upper_range {
            panic!("The value {} is unexpectedly in range [{},{}]", self.value, expected_lower_range,expected_upper_range);
        }
    }

}

pub trait ApproximatelyEqualMarker {}
struct IntegerApproximatelyEqual;
struct FloatApproximatelyEqual;

impl ApproximatelyEqualMarker for IntegerApproximatelyEqual{}
impl ApproximatelyEqualMarker for FloatApproximatelyEqual{}

trait ApproximatelyEqual<T, S:ApproximatelyEqualMarker  > {
    fn is_approx_equal_to(self, expected: T, delta: T);
}

impl<T> ApproximatelyEqual<T, IntegerApproximatelyEqual> for Asserter<T> where T : Signed + Integer + Zero + Neg + Bounded + Copy {
    fn is_approx_equal_to(self, expected: T, delta: T) {
        diff_eq!(self.value,expected,delta);
    }
}

impl<T> ApproximatelyEqual<T,FloatApproximatelyEqual> for Asserter<T> where T : Float + Zero + Neg + Copy + std::fmt::Display{
    fn is_approx_equal_to(self, expected: T, delta: T) {
        diff_eq!(self.value,expected,delta)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::panic_asserter_helper::assert_that_panics;

    #[test]
    fn test_is_smaller_than() { 
        assert_that!(3).is_smaller_than(4);
        assert_that!(21.0).is_smaller_than(21.1);

        assert_that_panics(||assert_that!(5).is_smaller_than(4));
        assert_that_panics(||assert_that!(10).is_smaller_than(10));
    }

    #[test]
    fn test_is_smaller_than_or_equal_to() { 
        assert_that!(4).is_smaller_than_or_equal_to(4);
        assert_that!(21.0).is_smaller_than_or_equal_to(21.1);

        assert_that_panics(||assert_that!(4.01).is_smaller_than_or_equal_to(4.0));
    }

    #[test]
    fn test_is_greater_than() {
        assert_that!(1).is_greater_than(0);
        assert_that!(15).is_greater_than(14);

        assert_that_panics(||assert_that!(15).is_greater_than(15));
        assert_that_panics(||assert_that!(10).is_greater_than(15));
    }

    #[test]
    fn test_is_greater_than_or_equal_to() {
        assert_that!(11).is_greater_than_or_equal_to(10);
        assert_that!(10).is_greater_than_or_equal_to(10);

        assert_that_panics(||assert_that!(9).is_greater_than(10));
    }

    #[test]
    fn test_is_in_range() {
        assert_that!(3).is_in_range(1,10);
        assert_that!(1).is_in_range(1,10);
        assert_that!(10).is_in_range(1,10);

        assert_that_panics(||assert_that!(0).is_in_range(1,10));
        assert_that_panics(||assert_that!(11).is_in_range(1,10));
    }

    #[test]
    fn test_is_not_in_range() {
        assert_that!(0).is_not_in_range(1,10);
        assert_that!(11).is_not_in_range(1,10);

        assert_that_panics(||assert_that!(1).is_not_in_range(1,10));
        assert_that_panics(||assert_that!(2).is_not_in_range(1,10));
        assert_that_panics(||assert_that!(10).is_not_in_range(1,10));
    }

    
    #[test]
    fn test_is_equal_to_approximately_for_signed() {
        assert_that!(3).is_approx_equal_to(2,1);
        assert_that!(3).is_approx_equal_to(3,1);
        assert_that!(3).is_approx_equal_to(4,1);

        assert_that_panics(||assert_that!(3).is_approx_equal_to(5,1));
    }

    /*
    #[test]
    fn test_is_equal_to_approximately_for_floats() {
        assert_that!(3.14).is_equal_to_approximately(3.16,0.02);
       // assert_that!(3.14).is_equal_to_approximately(3.14,0.00);
        //assert_that!(3.14159).is_equal_to_approximately(3.14157,0.00002);
    }*/
}
