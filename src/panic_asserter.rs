use std::panic;
use super::*;
use std::sync::{Arc, Mutex};
/*
TODO: this is non-deterministic and results in failing test due set_hook set the panic handling globally!!!
Idea how to solve: One option could be to make a panic hook that delegates to some thread-local state. Have all of your tests install that hook and then setup the thread local hook to what they want.
*/

pub struct WithMessage {
    actual_panic_message: String,
}

impl WithMessage {
    pub fn new(actual_panic_message: String) -> WithMessage {
        WithMessage {
            actual_panic_message
        }  
    }
    
    pub fn with_message(self, expected_panic_message: &str) {        
        if self.actual_panic_message != expected_panic_message {
            panic!("Expected a panic message '{}', but found '{}'",self.actual_panic_message, expected_panic_message)
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
        let _guard = LOCK_FOR_PANIC_ASSERTER.lock();
        let global_buffer = Arc::new(Mutex::new(String::new()));
        let old_hook = panic::take_hook();
    
        //try to use a thread local storage with RefCell or so 
        register_panic_hook_to_capture_output(&global_buffer);
        let result = panic::catch_unwind(self.value);
        panic::set_hook(old_hook);
        drop(_guard);
    
        let panic_message;
    
        match result {
            Ok(_res) => {
                panic!("There was no panic, but it was expected.")
            },
            Err(_) => {
                panic_message = global_buffer.lock().unwrap();
            }
        }
        

        WithMessage {
            actual_panic_message: panic_message.to_string()
        }
    }
    
    pub fn does_not_panic(self) {
        let result = self.catch_unwind_silent();
        
        if result.is_err() {
            panic!("Expected code to panic, but it does not panic.");//TODO: "did" instead of "does"
        }
    }

    fn catch_unwind_silent(self) -> std::thread::Result<R> {
        let prev_hook = panic::take_hook();
        panic::set_hook(Box::new(|_| {}));
        let result = panic::catch_unwind(self.value);
        panic::set_hook(prev_hook);
        result
    }
}


fn register_panic_hook_to_capture_output(global_buffer: &Arc<Mutex<String>>) {
    panic::set_hook({
        let global_buffer = global_buffer.clone();
        Box::new(move |info| {
            let mut global_buffer = global_buffer.lock().unwrap();

            //Capture for string literal like panic("some string")
            if let Some(s) = info.payload().downcast_ref::<&str>() {
                global_buffer.push_str(s);
            }

            //Check for dynamically created String like panic("some {}", "string")
            if let Some(s) = info.payload().downcast_ref::<String>() {
                global_buffer.push_str(s);
            }
        })
    });
}
