//! A library to write test assertions in a fluent syntax. Writing clean tests is as important as writing clean code.
//! This library contains test asserters to be used to make clean assertions in our automated tests.
//! It also helps to enhance the Test-Driven Development (TDD) experience, resulting in clean, readable and maintainable tests.
//!
//! ## Usage
//!
//! Add the dependency to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! fluent-asserter = "0.1.0"
//! ```
//!
//! Then add this to your crate:
//!
//! ```rust
//! extern crate fluent_asserter;
//! ```
//! And finally, import the the predefined traits from the prelude
//! ```rust
//! use fluent_asserter::prelude::*;
//! ```
//!
//! Now you should be able to write test assertions with a fluent syntax in your tests.
//!
//! ## String and string slice assertions
//! You can write string assertions for both String and str slices
//! ```rust
//!#[test]
//!fn string_assertions() {
//!    assert_that!("Life tastes great!").is_equal_to("Life tastes great!");
//!    assert_that!("Life tastes great!").contains("great");
//!    assert_that!("Life tastes great!").starts_with("Life");
//!    assert_that!("Life tastes great!").ends_with("!");
//!    assert_that!("Life tastes great!").is_not_empty();
//!    assert_that!("Life tastes great!").has_length(18);
//!    assert_that!("Life tastes great!").contains_any(&["Life", "awesome"]);
//!    assert_that!("Life tastes great!").contains_all(&["Life", "tastes", "great!"]);
//!}
//! ```
//!
//! ## Number assertions
//!
//! ```rust
//!#[test]
//!fn number_assertions() {
//!    assert_that!(21).is_equal_to(21);
//!    assert_that!(21).is_smaller_than(22);
//!    assert_that!(21).is_smaller_than_or_equal_to(21);
//!    assert_that!(21).is_greater_than(20);
//!    assert_that!(21).is_in_range(21,31);
//!    assert_that!(21).is_not_in_range(10,20);
//!    assert_that!(3.14159).is_approx_equal(3.142, 0.001);
//!}
//! ```
//!
//! ## Boolean assertions
//!
//! ```rust
//!#[test]
//!fn boolean_assertions() {
//!    assert_that!(true).is_true();
//!    assert_that!(false).is_false();
//!}
//! ```
//!
//! ## Panic assertions
//!
//! ```rust
//! #[test]
//! fn panic_assertions() {
//!     assert_that_code!(|| panic!("An error occurred!"))
//!         .panics()
//!         .with_message("An error occurred!");
//!
//!     assert_that_code!(|| println!("Life tastes great!")).does_not_panic();
//! }
//! ```
//!
//! ## Iterator assertions
//!
//! ```rust
//! #[test]
//! fn iterator_assertions() {
//!     assert_that!(vec!["tasty", "delicious", "lovely"]).is_equal_to(vec!["tasty", "delicious", "lovely"]);
//!     assert_that!(vec!["tasty", "delicious", "lovely"]).contains("delicious");
//!     assert_that!(vec!["tasty", "delicious", "lovely"]).contains_all(&["tasty", "delicious", "lovely"]);
//!     assert_that!(vec!["tasty", "delicious", "lovely"]).has_count(3);
//!     assert_that!(vec!["tasty", "delicious", "lovely"]).does_not_contain_any(&["awesome", "amazing"]);
//!     assert_that!(vec!["tasty", "delicious", "lovely"]).is_not_empty();
//! }
//! ```
//!
//! ## Iterator assertion for structs
//!
//! ```rust
//! #[derive(Clone)]
//! struct Person {
//!     name: String,
//!     age: i32,
//! }
//!
//! #[test]
//! fn iterator_assertion_for_struct() {
//!     let people: Vec<Person> = vec![
//!         Person {
//!             name: String::from("Daniel"),
//!             age: 32,
//!         },
//!         Person {
//!             name: String::from("Jimmy"),
//!             age: 45,
//!         },
//!     ];
//!
//!     assert_that!(people).satisfies_respectively(with_asserters!(
//!             |person1: &Person| {
//!                 assert_that!(&person1.name).is_equal_to(&String::from("Daniel"));
//!                 assert_that!(&person1.age).is_equal_to(&32);
//!             },
//!             |person2: &Person| {
//!                 assert_that!(&person2.name).is_equal_to(&String::from("Jimmy"));
//!                 assert_that!(&person2.age).is_equal_to(&45);
//!             }
//!         ));
//! }
//! ```
//!
//! ## Clear and concise error messages
//!
//! In case of a failing assertion, the error message is clear and on the point, containing all the information relating to the domain subject.
//!
//! ```rust
//! #[test]
//! fn test() {
//!    let string_variable = String::from("Hello Rust!");
//!
//!    assert_that!(string_variable).is_equal_to(String::from("Hello C#!"));
//! }
//! ```
//!
//! This test produces the assertion error message:
//!
//! ```doc
//! Expected string_variable to be "Hello C#!", but was "Hello Rust!".
//! ```
//!
//!
//TODO: follow these practices: https://pascalhertleif.de/artikel/good-practices-for-writing-rust-libraries/
//Idiomatic rust https://cheats.rs/#idiomatic-rust

//TODO: add path assertions - exists,doesNotExists, is a file, is a directory, hasfile name
//TODO: add Result asserter
//TODO: add our answer here: https://stackoverflow.com/questions/26469715/how-do-i-write-a-rust-unit-test-that-ensures-that-a-panic-has-occurred
//And here too± https://stackoverflow.com/questions/60965319/problems-using-paniccatch-unwind-in-a-macro-context-test-for-panics-in-unit-te
//and also to other place
mod boolean_asserter;
mod hashmap_asserter;
mod iterator_asserter;
mod number_approx_asserter;
mod number_asserter;
mod option_asserter;
mod panic_asserter;
pub mod prelude;
mod result_asserter;
mod string_asserter;

use lazy_static::lazy_static;
use std::borrow::Borrow;
use std::fmt::Debug;
use std::panic;

/// Creating fluent assertion for the specified type.
/// Depending on the specified type, there are different assertion methods available.
///
/// # Examples
/// ```rust
/// # #[macro_use] extern crate fluent_asserter;use fluent_asserter::*; fn main() {
/// assert_that!("awesome").is_equal_to("awesome");
/// assert_that!(3.14).is_smaller_than(3.15);
/// assert_that!(true).is_true();
/// # }
/// ```
///
#[macro_export]
macro_rules! assert_that {
    ($value:expr) => {
        create_asserter($value, stringify!($value).to_string())
    };
}

/// Creating fluent assertion to check if a closure panics
///
///  NOTE: Do not use this and the native #\[should_panic\] attribute at the same time while executing the tests parallel, as it can have non-deterministic behaviour
/// # Examples
/// ```rust
/// # #[macro_use] extern crate fluent_asserter;use fluent_asserter::*; fn main() {
///     assert_that_code!(|| panic!("An error occurred!"))
///         .panics()
///         .with_message("An error occurred!");
///
///     assert_that_code!(|| println!("Life tastes great!"))
///         .does_not_panic();
/// # }
/// ```
#[macro_export]
macro_rules! assert_that_code {
    ($value:expr) => {
        PanicAsserter::new($value) //TODO: only restrict it to pass function, and nothing else
    };
}

pub struct Asserter<T> {
    value: T,
    name: String,
}

pub struct PanicAsserter<F, R>
where
    F: FnOnce() -> R + panic::UnwindSafe,
{
    value: F,
}

impl<T> Asserter<T>
where
    T: Debug + PartialEq,
{
    pub fn new(value: T, name: String) -> Asserter<T> {
        Asserter { value, name }
    }

    pub fn is_equal_to(&self, expected_value: T) {
        let expected = expected_value.borrow();
        if &self.value != expected {
            let error_msg = format!(
                "Expected {} to be {:?}, but was {:?}.",
                self.name, expected, self.value
            );
            panic!("{}", error_msg)
        }
    }

    pub fn is_not_equal_to(&self, expected_value: T) {
        let expected = expected_value.borrow();
        assert_ne!(&self.value, expected);
    }
}

pub fn create_asserter<T>(value: T, name: String) -> Asserter<T> {
    Asserter { value, name }
}
