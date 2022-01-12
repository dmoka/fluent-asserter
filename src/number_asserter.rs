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
    fn todo() { 

    }
}
