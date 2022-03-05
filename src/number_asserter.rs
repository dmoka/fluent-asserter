use super::*;

//TODO: make trait and implement it

impl<T> Asserter<T>
where
    T: Copy + PartialOrd + std::fmt::Debug + std::fmt::Display,
{
    pub fn is_smaller_than(self, expected: T) {
        if self.value >= expected {
            panic!("The value {} is not smaller than {}", self.value, expected)
        }
    }

    pub fn is_smaller_than_or_equal_to(self, expected: T) {
        if self.value > expected {
            panic!(
                "The value {} is not smaller than or equal to {}",
                self.value, expected
            )
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

    pub fn is_in_range(self, expected_lower_range: T, expected_upper_range: T) {
        if self.value < expected_lower_range || self.value > expected_upper_range {
            panic!(
                "The value {} is not in range [{},{}]",
                self.value, expected_lower_range, expected_upper_range
            );
        }
    }

    pub fn is_not_in_range(self, expected_lower_range: T, expected_upper_range: T) {
        if self.value >= expected_lower_range && self.value <= expected_upper_range {
            panic!(
                "The value {} is unexpectedly in range [{},{}]",
                self.value, expected_lower_range, expected_upper_range
            );
        }
    }
}
