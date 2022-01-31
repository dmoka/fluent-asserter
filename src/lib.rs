//Idiomatic rust https://cheats.rs/#idiomatic-rust

//TODO: follow these practices: https://pascalhertleif.de/artikel/good-practices-for-writing-rust-libraries/

//TODO: add theory via annotation

//TODO: add our answer here: https://stackoverflow.com/questions/26469715/how-do-i-write-a-rust-unit-test-that-ensures-that-a-panic-has-occurred
//And here too± https://stackoverflow.com/questions/60965319/problems-using-paniccatch-unwind-in-a-macro-context-test-for-panics-in-unit-te
//and also to other place
mod string_asserter;
mod panic_asserter;
mod number_asserter;
mod number_approx_asserter;
mod assertion_failure_message;
mod boolean_asserter;

use std::{panic};
use std::borrow::Borrow;
use std::fmt::Debug;
use lazy_static::lazy_static;
use std::sync::Mutex;

lazy_static! {
    static ref LOCK_FOR_PANIC_ASSERTER: std::sync::Mutex<()> = Mutex::new(());
}

#[macro_export]
macro_rules! assert_that {
    ($value:expr) => {
        Asserter::new($value, stringify!($value).to_string())
    };
}

#[macro_export]
macro_rules! assert_that_code {
    ($value:expr) => {
        PanicAsserter::new($value) //TODO: only restrict it to pass function, and nothing else
    };
}

pub struct Asserter<T> {
    value : T,
    name: String
}

pub struct PanicAsserter <F, R> where F: FnOnce() -> R + panic::UnwindSafe {
    value :  F
}

impl<T> Asserter<T> where T: Debug + PartialEq + ToString {
    pub fn new(value: T, name: String) -> Asserter<T> {
        Asserter {
            value,
            name
        }
    }

    pub fn is_equal_to(&self, expected_value: T) {
        let expected = expected_value.borrow();
        if &self.value != expected {
            let error_msg = format!("Expected {} to be '{}', but was '{}'",self.name,expected_value.to_string(),self.value.to_string());
            panic!("{}",error_msg)
        }
    }

    pub fn is_not_equal_to(&self, expected_value: T) {
        let expected = expected_value.borrow();
        assert_ne!(&self.value, expected);
    }
}

//TODO: can we put these to approx class? If we put there, we can not use is_approx_equal_to method in test project
pub trait ApproxEqualMarkerTrait {}

pub struct UnsignedIntApproxEqual;
pub struct SignedIntApproxEqual;
pub struct FloatApproxEqual;

impl ApproxEqualMarkerTrait for SignedIntApproxEqual{}
impl ApproxEqualMarkerTrait for FloatApproxEqual{}
impl ApproxEqualMarkerTrait for UnsignedIntApproxEqual{}

#[allow(clippy::wrong_self_convention)] 
pub trait ApproximatelyEqual<T, S:ApproxEqualMarkerTrait  > {
    fn is_approx_equal_to(self, expected: T, delta: T);
}

//TODO: S - add this to tests folder
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_macro_syntax() {
        assert_that_code!(|| panic!("")).panics();

        assert_that!("value").is_not_empty();
    }

}