use super::*;

impl<T> Asserter<T> where T : Copy + PartialOrd + std::fmt::Debug {
    //TODO: add further number stuff
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::panic_asserter_helper::assert_that_panics;


    //TODO: We should add these tests to lib.rs
    #[test]
    fn test_is_equal_to() { 
        assert_that!(1u32).is_equal_to(1);
        assert_that!(2i32).is_equal_to(2);
        assert_that!(3.0).is_equal_to(3.0);
        assert_that!(-4).is_equal_to(-4);

        assert_that_panics(|| assert_that!(3.0).is_equal_to(4.0))
    }
}
