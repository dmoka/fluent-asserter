//! A fluent library to write clean tests. Writing clean tests is as important as writing clean code.
//! This library contains test asserters with a fluent-like syntax to be used for making clean assertions for most of the standard Rust types.
//! Also ideal for enhancing the Test-Driven Development (TDD) experience to design code.

//Idiomatic rust https://cheats.rs/#idiomatic-rust

//TODO: follow these practices: https://pascalhertleif.de/artikel/good-practices-for-writing-rust-libraries/

//TODO: add path assertions - exists,doesNotExists, is a file, is a directory, hasfile name

//TODO: add our answer here: https://stackoverflow.com/questions/26469715/how-do-i-write-a-rust-unit-test-that-ensures-that-a-panic-has-occurred
//TODO: add hashmap asserter
//And here too± https://stackoverflow.com/questions/60965319/problems-using-paniccatch-unwind-in-a-macro-context-test-for-panics-in-unit-te
//and also to other place
mod string_asserter;
mod panic_asserter;
mod number_asserter;
mod number_approx_asserter;
mod boolean_asserter;
mod iterator_asserter;
mod option_asserter;
mod result_asserter;
mod hashmap_asserter;
pub mod prelude;

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
        create_asserter($value, stringify!($value).to_string())
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

impl<T> Asserter<T> where T: Debug + PartialEq{
    pub fn new(value: T, name: String) -> Asserter<T> {
        Asserter {
            value,
            name
        }
    }

    pub fn is_equal_to(&self, expected_value: T) {
        let expected = expected_value.borrow();
        if &self.value != expected {
            let error_msg = format!("Expected {} to be {:?}, but was {:?}.",self.name,expected,self.value);
            panic!("{}",error_msg)
        }
    }

    pub fn is_not_equal_to(&self, expected_value: T) {
        let expected = expected_value.borrow();
        assert_ne!(&self.value, expected);
    }
}

pub fn create_asserter<T>(value: T, name: String) -> Asserter<T> {
    Asserter {
        value,
        name
    }
}