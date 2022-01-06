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


//TODO: implement normal assert_that method when we have the all asserter implemented implemented
mod panic_asserter_helper;
mod string_asserter;
mod panic_asserter;