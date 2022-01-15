use std::panic;
use super::*;
use std::{
    sync::{Arc, Mutex},
};
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

fn register_panic_hook_to_capture_output(global_buffer: &Arc<Mutex<String>>) {
    panic::set_hook({
        let global_buffer = global_buffer.clone();
        Box::new(move |info| {
            let mut global_buffer = global_buffer.lock().unwrap();

            if let Some(s) = info.payload().downcast_ref::<&str>() {
                global_buffer.push_str(s);
            }
        })
    });
}

pub struct WithMessage {
    actual_panic_message: String,
}

impl WithMessage {
    pub fn new(self, actual_panic_message: String) -> WithMessage {
        WithMessage {
            actual_panic_message
        }  
    }
    
    pub fn with_message(self, expected_panic_message: &str) {
        if self.actual_panic_message != expected_panic_message {
            panic!("\nAssertionError: The expected panic message is different from the actual one.\n Expected: {} \n Actual: {}",self.actual_panic_message, expected_panic_message)
        }
    }
}


impl<F, R> PanicAsserter<F, R>  where F: FnOnce() -> R + panic::UnwindSafe{
    pub fn new(f:  F) -> Self {
        PanicAsserter{
            value: f
        }
    }

    pub fn panics(self) -> WithMessage {
        let global_buffer = Arc::new(Mutex::new(String::new()));
        let old_hook = panic::take_hook();
    
        register_panic_hook_to_capture_output(&global_buffer);
        let result = panic::catch_unwind(self.value);
        panic::set_hook(old_hook);
    
        let panic_message;
    
        match result {
            Ok(res) => {
                panic!("There was no panic, but it was expected!")
            },
            Err(_) => {
                panic_message = global_buffer.lock().unwrap();
                /*println!("caught panic!");
                println!("I captured:\n\n{}", global_buffer.lock().unwrap())*/
            }
        }
    
        let actual_panic_message : String = panic_message.to_string();
    
        //TODO: add better panic error message
        assert!(result.is_err());
        
        WithMessage {
            actual_panic_message
        }
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
    use crate::panic_asserter_helper::assert_that_panics;

    use super::*;

    #[test]
    fn test_panics_with_message() {
        assert_that_code!(|| panic!("specific panic message"))
                                .panics()
                                .with_message("specific panic message");

        assert_that_panics(||assert_that_code!(|| panic!("specific panic message"))
                                    .panics()
                                    .with_message("another expected panic message"));

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