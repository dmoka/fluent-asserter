use std::panic;

#[macro_export]
macro_rules! assert_that_code {
    ($value:expr) => {
        FunctionAsserter::new($value) //TODO: only restrict it to pass function, and nothing else
    };
}

pub fn assert_that_code<F: FnOnce() -> R + panic::UnwindSafe, R>(f: F) -> FunctionAsserter<F, R> where F: FnOnce() -> R + panic::UnwindSafe {
    FunctionAsserter {
        value: f
    }
}

pub struct FunctionAsserter <F, R> where F: FnOnce() -> R + panic::UnwindSafe {
    value :  F
}

impl<F, R> FunctionAsserter<F, R>  where F: FnOnce() -> R + panic::UnwindSafe{
    pub fn new(f:  F) -> Self {
        FunctionAsserter{
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