use crate::*;
use std::fmt::Display;

//TODO: make trait and implement it

impl<T, K> Asserter<&Result<T, K>>
where
    T: PartialEq + Display,
    K: PartialEq + Display,
{
    pub fn is_ok(&self) {
        match &self.value {
            Ok(_) => {}
            Err(val) => {
                panic!(
                    "Expected '{}' to be Ok, but found Err({}).",
                    &self.name, val
                )
            }
        }
    }

    pub fn is_error(&self) {
        match &self.value {
            Ok(val) => {
                panic!(
                    "Expected '{}' to be Err(_), but found Ok({}).",
                    &self.name, val
                )
            }
            Err(_) => {}
        }
    }

    pub fn is_ok_with_value(&self, expected_value: T) {
        match &self.value {
            Ok(val) => {
                if *val != expected_value {
                    panic!(
                        "Expected '{}' to be Ok({}), but found Ok({}).",
                        &self.name, expected_value, val
                    )
                }
            }
            Err(val) => {
                panic!(
                    "Expected '{}' to be Ok({}), but found Err({}).",
                    &self.name, expected_value, val
                )
            }
        }
    }

    pub fn is_error_with_value(&self, expected_value: K) {
        match &self.value {
            Ok(val) => {
                panic!(
                    "Expected '{}' to be Err({}), but found Ok({}).",
                    &self.name, expected_value, val
                )
            }
            Err(val) => {
                if *val != expected_value {
                    panic!(
                        "Expected '{}' to be Err({}), but found Err({}).",
                        &self.name, expected_value, val
                    )
                }
            }
        }
    }
}
