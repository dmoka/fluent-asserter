//Idiomatic rust https://cheats.rs/#idiomatic-rust

//TODO: follow these practices: https://pascalhertleif.de/artikel/good-practices-for-writing-rust-libraries/

mod panic_asserter_helper;
mod string_asserter;
mod panic_asserter;

use std::panic;

pub struct Asserter<T> {
    value : T
}

pub struct PanicAsserter <F, R> where F: FnOnce() -> R + panic::UnwindSafe {
    value :  F
}

//TODO: add this?

impl<T> Asserter<T> {
    pub fn new(value: T) -> Asserter<T> {
        Asserter {
            value
        }
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


//TODO: struct assertions with lambda like in c#

#[macro_export]
macro_rules! assert_that {
    ($value:expr) => {
        Asserter::new($value)
    };
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn TestRootAssert() {
        Assert::that_code(|| panic!("")).panics();

        Assert::that("value").is_not_empty();
    }
}

//TODO: implement normal assert_that method when we have the all asserter implemented implemented
