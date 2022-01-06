use std::panic;
use super::*;

//TODO: add answer to this SO question: https://stackoverflow.com/questions/26469715/how-do-i-write-a-rust-unit-test-that-ensures-that-a-panic-has-occurred

#[macro_export]
macro_rules! assert_that_code {
    ($value:expr) => {
        PanicAsserter::new($value) //TODO: only restrict it to pass function, and nothing else
    };
}

pub fn assert_that_code<F: FnOnce() -> R + panic::UnwindSafe, R>(f: F) -> PanicAsserter<F, R> where F: FnOnce() -> R + panic::UnwindSafe {
    PanicAsserter {
        value: f
    }
}


impl<F, R> PanicAsserter<F, R>  where F: FnOnce() -> R + panic::UnwindSafe{
    pub fn new(f:  F) -> Self {
        PanicAsserter{
            value: f
        }
    }

    pub fn panics(self) {
        let result = self.catch_unwind_silent();
    
        //TODO: add better panic error message
        assert!(result.is_err())
    }
    
    pub fn does_not_panic(self) {
        let result = self.catch_unwind_silent();
    
        //TODO: add better panic error message
        assert!(result.is_ok())
    }

    fn catch_unwind_silent(self) -> std::thread::Result<R> {
        let prev_hook = panic::take_hook();
        panic::set_hook(Box::new(|_| {}));
        let result = panic::catch_unwind(self.value);
        panic::set_hook(prev_hook);
        result
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn asd() {
        assert_that_code!(|| panic!("error")).panics();
        assert_that_code(|| panic!("error")).panics();
    }

    #[test]
    fn test_assert_that_panics() {
        assert_that_code!(|| panic!("error")).panics();
        assert_that_code(|| panic!("error")).panics();

        assert_that_code!(|| panic!("validation error")).panics();
        assert_that_code(|| panic!("validation error")).panics();
    }

    #[test]
    fn test_assert_that_does_not_panics() {
        assert_that_code!(|| println!("gm")).does_not_panic();
        assert_that_code(|| println!("gm")).does_not_panic();

        assert_that_code!(|| println!("WAGMI")).does_not_panic();
        assert_that_code(|| println!("WAGMI")).does_not_panic();
    }
}