use super::*;

impl<T> Asserter<T> where T : Copy + PartialOrd + std::fmt::Debug + std::fmt::Display {
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
}
