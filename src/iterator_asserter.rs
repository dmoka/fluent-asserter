use super::*;

pub trait IteratorAssertions<T> where T: Debug + PartialEq {
    //TODO: check if we need to use mutable, with a test with mutable variable
    fn contains(&self, expected_value: T);
    fn contains_any(&self, expected_value: &[T]);
}

impl<T,K> IteratorAssertions<T> for Asserter<K> where T: Debug + PartialEq, K: IntoIterator<Item = T> + Clone {
    fn contains(&self, expected_value: T) {
        let contains = &self.value.clone().into_iter().any(|i| i==expected_value);
        if !contains {
            panic!("Expected iterator {:?} to contain {:?}, but it does not",self.name,expected_value);
        }
    }

    fn contains_any(&self, expected_values: &[T]) {
        for expected_value in expected_values {
            let contains = &self.value.clone().into_iter().any(|i| &i==expected_value);
            if !contains {
                panic!("Expected iterator {:?} to contain items {:?}, but it does not",self.name,expected_values);
            }       
        }
    }
}