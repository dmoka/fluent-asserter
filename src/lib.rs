//Idiomatic rust https://cheats.rs/#idiomatic-rust

//TODO: implement assert_that method when we have the stuff implemented

pub struct Asserter<T> {
    value : T
}

//TODO: struct assertions with lambda like in c#

#[macro_export]
macro_rules! assert_that {
    ($value:expr) => {
        Asserter::new($value)
    };
}

mod panic_asserter;
mod string_asserter;