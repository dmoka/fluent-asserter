use super::*;

//TODO: shall we add these methods to ShouldRoot?

trait PanicAssertions {
    fn should_panic(self);

    fn should_not_panic(self);
}

impl<T, R> PanicAssertions for T where T: FnOnce() -> R + panic::UnwindSafe {

    fn should_panic(self){
        PanicAsserter::<T,R>::new(self).panics()
    }

    fn should_not_panic(self){
        PanicAsserter::<T,R>::new(self).does_not_panic()
    }
}


#[cfg(test)]
mod test {
    use super::*;
    use crate::panic_asserter_helper::assert_that_panics;

    #[test]
    fn sanity_checks() {
        (|| panic!("error")).should_panic();

        (|| println!("gm")).should_not_panic();
        
        assert_that_panics(||(|| println!("gm")).should_panic())
    }

}