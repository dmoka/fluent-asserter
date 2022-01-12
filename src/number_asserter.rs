use super::*;

impl<T> Asserter<T> where T : Copy + PartialOrd + std::fmt::Debug {
    //TODO: add further number stuff
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::panic_asserter_helper::assert_that_panics;


    #[test]
    fn sanity_check_for_assertions() { 
        assert_that!(3).is_equal_to(3);
        assert_that!(3.0).is_equal_to(3.0);
        assert_that!(-5).is_equal_to(-5);
    }
}
