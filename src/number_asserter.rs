use super::*;
use std::ops::Neg;
use num::Unsigned;
use num::traits::Pow;
use num::{traits::Zero, Integer, Signed, Bounded, Float}; // 0.2.0

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

//TODO: do we really need macro for this?
macro_rules! abs_diff_unsigned {
    ($x:expr, $y:expr, $d:expr) => {
        if (if $x > $y {
            $x - $y 
        } else {
            $y - $x
        }) > $d {
            panic!("AssertionError: not equal");
        }
    }
}

macro_rules! abs_diff_eq {
    ($x:expr, $y:expr, $d:expr) => {
        if (($x - $y).abs() > $d) { panic!("AssertionError: not equal"); }
    }
}

macro_rules! abs_diff {
    ($x:expr, $y:expr) => {
        ($x - $y).abs()
    }
}

pub trait ApproxEqualMarkerTrait {}
struct UnsignedIntApproxEqual;
struct SignedIntApproxEqual;
struct FloatApproxEqual;

impl ApproxEqualMarkerTrait for SignedIntApproxEqual{}
impl ApproxEqualMarkerTrait for FloatApproxEqual{}
impl ApproxEqualMarkerTrait for UnsignedIntApproxEqual{}

trait ApproximatelyEqual<T, S:ApproxEqualMarkerTrait  > {
    fn is_approx_equal_to(self, expected: T, delta: T);
}

impl<T> ApproximatelyEqual<T, UnsignedIntApproxEqual> for Asserter<T> where T : Unsigned + Integer + Zero  + Bounded + Copy {
    fn is_approx_equal_to(self, expected: T, delta: T) {
        abs_diff_unsigned!(self.value,expected,delta);
    }
}

impl<T> ApproximatelyEqual<T, SignedIntApproxEqual> for Asserter<T> where T : Signed + Integer + Zero + Neg + Bounded + Copy {
    fn is_approx_equal_to(self, expected: T, delta: T) {
        abs_diff_eq!(self.value,expected,delta);
    }
}

impl<T> ApproximatelyEqual<T,FloatApproxEqual> for Asserter<T> where T :Float + Zero + Neg + Copy + std::fmt::Display{
    fn is_approx_equal_to(self, expected: T, delta: T) {
        let rounder = 10f64.pow(get_length_of_rounder(delta));
        
        let diff = abs_diff!(self.value,expected);

        let diff_f64 = round(diff, rounder);
        let delta_f64 =  round(delta, rounder);

        if diff_f64 > delta_f64 {
            panic!("AssertionError: Not equal")
        }
    }
}


//TODO: add abstraction for this, either in a new struct with new or some logic class
fn get_length_of_rounder<T>(delta: T) -> f64 where T: ToString {
    return delta.to_string()
        .split(".")
        .last()
        .unwrap()
        .len()
        .to_string()
        .parse()
        .unwrap();
}

fn round<T>(diff: T, rounder: f64) -> f64 where T: std::fmt::Display {
    let number : f64 = format!("{}",diff).parse().unwrap();
    let number: f64 = (number * rounder).round() / rounder;
    
    number
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
    fn test_is_equal_to_approximately_for_floats() {
        assert_that!(3.14).is_approx_equal_to(3.16,0.02);
        assert_that!(3.14).is_approx_equal_to(3.14,0.00);
        assert_that!(3.14159).is_approx_equal_to(3.14157,0.00002);

        assert_that_panics(||assert_that!(3.14159).is_approx_equal_to(3.14157,0.00001));
    }
}
