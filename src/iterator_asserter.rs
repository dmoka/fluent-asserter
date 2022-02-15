use super::*;

pub trait IteratorAssertions<T> where T: Debug + PartialEq {
    //TODO: check if we need to use mutable, with a test with mutable variable
    fn contains(&self, expected_value: T);
    fn contains_all(&self, expected_values: &[T]);
    fn has_count(&self, expected_count: usize);
    fn does_not_contain_any(&self, not_expected_values: &[T]);
    fn is_empty(&self);
    fn is_not_empty(&self);
}

pub trait IteratorSatisfiesAssertion<T> {
    fn satisfies_respectively(&self, asserter: Vec<Box<dyn Fn(&T)>>);
}

impl<T,K> IteratorAssertions<T> for Asserter<K> where T: Debug + PartialEq, K: IntoIterator<Item = T> + Clone {
    fn contains(&self, expected_value: T) {
        let contains = &self.value.clone().into_iter().any(|i| i==expected_value);
        if !contains {
            panic!("Expected iterator {:?} to contain {:?}, but it does not.",self.name,expected_value);
        }
    }

    fn contains_all(&self, expected_values: &[T]) {
        let mut missing_items = std::vec::Vec::<&T>::new();
        for expected_value in expected_values {
            let contains = contains(&self.value, expected_value);
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
            let contains = contains(&self.value, not_expected_value);
            if contains {
                missing_items.push(not_expected_value);
            }       
        }

        if !missing_items.is_empty() {
            panic!("Expected iterator {:?} to not contain items {:?}, but it contains {:?}.",self.name,not_expected_values, missing_items);
        }
    }

    fn is_empty(&self) {
        let is_empty = is_empty(&self.value);
        
        if !is_empty {
            panic!("Expected iterator {:?} to be empty, but it is not.",self.name);
        }
    }

    fn is_not_empty(&self) {
        let is_empty = is_empty(&self.value);
        
        if is_empty {
            panic!("Expected iterator {:?} to be not empty, but it is.",self.name);
        }
    }

}

fn is_empty<T,K>(iterator: &K) -> bool where K: Clone + IntoIterator<Item=T>, T: Debug + PartialEq {
    iterator.clone().into_iter().next().is_none()
}

fn contains<T,K>(iterator: &K, expected_value: &T) -> bool where K: Clone + IntoIterator<Item=T>, T: Debug + PartialEq {
    iterator.clone().into_iter().any(|i| i==*expected_value)
}

impl<T,K> IteratorSatisfiesAssertion<T> for Asserter<K> where K: IntoIterator<Item = T> + Clone {
    fn satisfies_respectively(&self, asserter: Vec<Box<dyn Fn(&T)>>) {//TODO: S - rename to asserters
        let iter = &self.value.clone().into_iter().collect::<Vec::<T>>();

        if iter.len() != asserter.len() {
            panic!("Expected number of items to be {}, but was {}.",asserter.len(), iter.len())
        }
        
        for i in 0..asserter.len() {
            asserter[i](&iter[i])
        }
    }
}

//rename to with_asserters?
#[macro_export]
macro_rules! with_asserters {
    ($($closure:expr),*)  => {
        vec![
            $( Box::new($closure) as Box<dyn for<'a> Fn(&'a _) -> _> ),*
        ]
    };
}