//Idiomatic rust https://cheats.rs/#idiomatic-rust

//TODO: follow these practices: https://pascalhertleif.de/artikel/good-practices-for-writing-rust-libraries/

//TODO: add theory via annotation

mod panic_asserter_helper;
mod string_asserter;
mod panic_asserter;
mod number_asserter;

use std::{panic};
use std::borrow::Borrow;
use std::fmt::Debug;

#[macro_export] //TODO: place it in number asserter, without export
macro_rules! abs_diff_eq {
    ($x:expr, $y:expr, $d:expr) => {
        if (($x - $y).abs() > $d) { panic!("AssertionError: not equal"); }
    }
}

#[macro_export] //TODO: place it in number asserter, without export
macro_rules! abs_diff_unsigned {
    ($x:expr, $y:expr, $d:expr) => {
        if (if $x > $y {
            $x - $y 
        } else {
            $y - $x
        }) > $d {
            panic!("AssertionError: not equal");
        }
    }
}


#[macro_export] 
//TODO: place it in number asserter, without export
macro_rules! abs_diff {
    ($x:expr, $y:expr) => {
        ($x - $y).abs()
    }
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


#[macro_export]
macro_rules! assert_that {
    ($value:expr) => {
        Asserter::new($value)
    };
}


#[cfg(test)]
mod test {
    use super::*;

    use crate::panic_asserter_helper::assert_that_panics;

    #[test]
    fn sanity_check_for_assertions() {
        Assert::that_code(|| panic!("")).panics();

        Assert::that("value").is_not_empty();
    }

    #[test]
    fn test_is_equal_to_for_numerics() { 
        assert_that!(1u32).is_equal_to(1);
        assert_that!(2i32).is_equal_to(2);
        assert_that!(3.0).is_equal_to(3.0);
        assert_that!(-4).is_equal_to(-4);

        assert_that_panics(|| assert_that!(3.0).is_equal_to(4.0))
    }

    #[test]
    fn test_is_not_equal_to_for_numerics() { 
        assert_that!(1u32).is_not_equal_to(2);
        assert_that!(2i32).is_not_equal_to(3);
        assert_that!(3.0).is_not_equal_to(4.0);
        assert_that!(-4).is_not_equal_to(-5);

        assert_that_panics(|| assert_that!(3.0).is_not_equal_to(3.0))
    }

    #[test]
    fn test_is_equal_to_for_string() {
        assert_that!(&String::from("test string")).is_equal_to(&String::from("test string"));
        assert_that!(&String::from("bitcoin")).is_equal_to(&String::from("bitcoin"));

        assert_that_panics(|| assert_that!(&String::from("test string")).is_equal_to(&String::from("test")));
        assert_that_panics(|| assert_that!(&String::from("bitcoin")).is_equal_to(&String::from("ethereum")));
    }


    #[test]
    fn test_is_equal_to_for_str() {
        assert_that!("test string").is_equal_to("test string");
        assert_that!("bitcoin").is_equal_to("bitcoin");
        
        assert_that_panics(|| assert_that!("test string").is_equal_to("string"));
        assert_that_panics(|| assert_that!("bitcoin").is_equal_to("ethereum"));
    }
}

//TODO: implement normal assert_that method when we have the all asserter implemented implemented
