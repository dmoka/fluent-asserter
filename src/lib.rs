//Idiomatic rust https://cheats.rs/#idiomatic-rust

//TODO: follow these practices: https://pascalhertleif.de/artikel/good-practices-for-writing-rust-libraries/

//TODO: add theory via annotation

mod string_asserter;
mod panic_asserter;
mod number_asserter;
mod number_approx_asserter;

use std::{panic};
use std::borrow::Borrow;
use std::fmt::Debug;

#[macro_export]
macro_rules! assert_that {
    ($value:expr) => {
        Asserter::new($value)
    };
}

#[macro_export]
macro_rules! assert_that_code {
    ($value:expr) => {
        PanicAsserter::new($value) //TODO: only restrict it to pass function, and nothing else
    };
}

pub struct Asserter<T> {
    value : T
}

pub struct PanicAsserter <F, R> where F: FnOnce() -> R + panic::UnwindSafe {
    value :  F
}

impl<T> Asserter<T> where T: Debug + PartialEq {
    pub fn new(value: T) -> Asserter<T> {
        Asserter {
            value
        }
    }

    pub fn is_equal_to(&self, expected_value: T) {
        let expected = expected_value.borrow();
        assert_eq!(&self.value, expected);
    }

    pub fn is_not_equal_to(&self, expected_value: T) {
        let expected = expected_value.borrow();
        assert_ne!(&self.value, expected);
    }
}

struct Assert;

trait GenericAssert<TValue> {
    fn that(value: TValue) -> Asserter<TValue>;
}

trait PanicAssert<TFunction, TCatchPanicResult> {
    fn that_code(f: TFunction) -> PanicAsserter<TFunction, TCatchPanicResult> where TFunction: FnOnce() -> TCatchPanicResult + panic::UnwindSafe;
}

impl<TValue> GenericAssert<TValue> for Assert {
    fn that(value: TValue) -> Asserter<TValue> {
        Asserter {
            value
        }
    }
} 

impl<TFunction, TCatchPanicResult>  PanicAssert<TFunction, TCatchPanicResult> for Assert {
    fn that_code(f: TFunction) -> PanicAsserter<TFunction, TCatchPanicResult> where TFunction: FnOnce() -> TCatchPanicResult + panic::UnwindSafe {
        PanicAsserter::new(f)
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
    fn test_basic_syntax() {
        Assert::that_code(|| panic!("")).panics();

        Assert::that("value").is_not_empty();
    }

    #[test]
    fn test_macro_syntax() {
        assert_that_code!(|| panic!("")).panics();

        assert_that!("value").is_not_empty();
    }
}