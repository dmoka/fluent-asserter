//Idiomatic rust https://cheats.rs/#idiomatic-rust

use std::panic;

pub struct Asserter<T> {
    value : T
}

pub struct FunctionAsserter <F, R> where F: FnOnce() -> R + panic::UnwindSafe {
    value :  F
}

//TODO: struct assertions with lambda like in c#

#[macro_export]
macro_rules! assert_that {
    ($value:expr) => {
        Asserter::new($value)
    };
}

#[macro_export]
macro_rules! assert_that_code {
    ($value:expr) => {
        FunctionAsserter::new($value)
    };
}

//TODO: implement normal assert_that method when we have the stuff implemented

//TODO: do we need this here?
pub fn assert_that_code<F: FnOnce() -> R + panic::UnwindSafe, R>(f: F) -> FunctionAsserter<F, R> where F: FnOnce() -> R + panic::UnwindSafe {
    FunctionAsserter {
        value: f
    }
}


mod panic_asserter;
mod string_asserter;
mod fn_asserter;