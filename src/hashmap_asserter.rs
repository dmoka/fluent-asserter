use super::*;
use std::collections::HashMap;
use std::fmt::Display;
use std::hash::Hash;

//TODO: add trait

pub struct ValueAssertions<'a, K, V> {
    key: &'a K,
    value: &'a V,
    hash_map_name: String,
}

impl<'a, K, V> ValueAssertions<'a, K, V>
where
    V: PartialEq + Display,
    K: Display,
{
    pub fn with_value(&'a self, expected_value: V) {
        if *self.value != expected_value {
            panic!(
                "Expected {} to contain {} with value '{}', but it has the value '{}'.",
                &self.hash_map_name, &self.key, expected_value, &self.value
            )
        }
    }
}

impl<'a, K, V> Asserter<&HashMap<K, V>>
where
    K: Eq + Hash + Display,
{
    /// Checks the length of the HashMap
    pub fn has_length(&self, expected_length: usize) {
        if self.value.len() != expected_length {
            panic!(
                "Expected {} to have length {}, but it has {}",
                &self.name,
                expected_length,
                self.value.len()
            );
        }
    }

    /// Checks if the HashMap is empty
    pub fn is_empty(&self) {
        if self.value.len() > 0 {
            panic!(
                "Expected {} to be empty, but it has length {}.",
                &self.name,
                self.value.len()
            )
        }
    }

    /// Checks if the HashMap is not empty
    pub fn is_not_empty(&self) {
        if self.value.is_empty() {
            panic!("Expected {} to not to be empty, but it is.", &self.name)
        }
    }

    /// Checks if the HashMap contains the specified key
    pub fn contains_key(&'a self, expected_key: &'a K) -> ValueAssertions<'a, K, V> {
        if !&self.value.contains_key(expected_key) {
            panic!(
                "Expected {} to contain {}, but it does not.",
                &self.name, &expected_key
            );
        }

        let value = &self.value.get(expected_key);
        let value_for_key = value.unwrap();

        ValueAssertions {
            key: expected_key,
            value: value_for_key,
            hash_map_name: String::from(&self.name),
        }
    }

    /// Checks if the HashMap does not contain the specified key
    pub fn does_not_contain_key(&self, not_expected_key: K) {
        if self.value.contains_key(&not_expected_key) {
            panic!(
                "Expected {} to not to contain {}, but it does.",
                &self.name, &not_expected_key
            );
        }
    }
}
