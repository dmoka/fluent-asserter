use super::*;

pub trait IteratorAssertions<T> where T: Debug + PartialEq {
    //TODO: check if we need to use mutable, with a test with mutable variable
    fn contains(&self, expected_value: T);
    fn contains_all_of(&self, expected_values: &[T]);
    fn has_count(&self, expected_count: usize);
    fn does_not_contain_any(&self, not_expected_values: &[T]);
    fn is_empty(&self);
    fn is_not_empty(&self);
}

impl<T,K> IteratorAssertions<T> for Asserter<K> where T: Debug + PartialEq, K: IntoIterator<Item = T> + Clone {
    fn contains(&self, expected_value: T) {
        let contains = &self.value.clone().into_iter().any(|i| i==expected_value);
        if !contains {
            panic!("Expected iterator {:?} to contain {:?}, but it does not.",self.name,expected_value);
        }
    }

    fn contains_all_of(&self, expected_values: &[T]) {
        let mut missing_items = std::vec::Vec::<&T>::new();
        for expected_value in expected_values {
            let contains = &self.value.clone().into_iter().any(|i| &i==expected_value);
            if !contains {
                missing_items.push(expected_value);
            }       
        }

        if !missing_items.is_empty() {
            panic!("Expected iterator {:?} to contain items {:?}, but it does not contain {:?}.",self.name,expected_values, missing_items);
        }
    }

    fn has_count(&self, expected_count: usize) {
        let count = &self.value.clone().into_iter().count();
        if *count != expected_count {
            panic!("Expected iterator {:?} to have count '{}', but it has '{}'.", &self.name,expected_count,count);
        }
    }

    fn does_not_contain_any(&self, not_expected_values: &[T]) {
        let mut missing_items = std::vec::Vec::<&T>::new();

        for not_expected_value in not_expected_values {
            let contains = &self.value.clone().into_iter().any(|i| &i==not_expected_value);
            if *contains {
                missing_items.push(not_expected_value);
            }       
        }

        if !missing_items.is_empty() {
            panic!("Expected iterator {:?} to not contain items {:?}, but it contains {:?}.",self.name,not_expected_values, missing_items);
        }
    }

    fn is_empty(&self) {
        let is_empty = &self.value.clone().into_iter().next().is_none();
        
        if !*is_empty {
            panic!("Expected iterator {:?} to be empty, but it is not.",self.name);
        }
    }

    fn is_not_empty(&self) {
        let is_empty = &self.value.clone().into_iter().next().is_none();
        
        if *is_empty {
            panic!("Expected iterator {:?} to be not empty, but it is.",self.name);
        }
    }

}