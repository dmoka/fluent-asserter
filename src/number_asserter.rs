use super::*;

impl<T> Asserter<T> where T : Copy + PartialOrd + std::fmt::Debug + std::fmt::Display {
    pub fn is_smaller_than(self, expected: T) {
        if self.value > expected {
            panic!("The value {} is not smaller than {}", self.value, expected)
        }
    }
    //TODO: add further number stuff
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::panic_asserter_helper::assert_that_panics;

    //TODO: We should add these tests to lib.rs
    #[test]
    fn test_is_smaller_than() { 
        assert_that!(3).is_smaller_than(4);
        assert_that!(21.0).is_smaller_than(21.1);

        assert_that_panics(||assert_that!(5).is_smaller_than(4));
    }
}
