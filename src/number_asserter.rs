use super::*;

impl<T> Asserter<T> where T : Copy + PartialOrd + std::fmt::Debug {
    pub fn is_equal_to2(&self, expected_value: T) {
        assert_eq!(self.value, expected_value);
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::panic_asserter_helper::assert_that_panics;


    #[test]
    fn sanity_check_for_assertions() { 
        assert_that!(3).is_equal_to2(3);
        assert_that!(3.0).is_equal_to2(3.0);
        assert_that!(-5).is_equal_to2(-5);
    }
}
