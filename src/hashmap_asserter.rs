use super::*;
use std::collections::HashMap;
use std::fmt::Display;
use std::hash::Hash;

impl<K,V> Asserter<&HashMap<K,V>> where K : Eq + Hash + Display {
    pub fn has_length(&self, expected_length: usize) {
        if self.value.len() != expected_length {
            panic!("Expected {} to have length {}, but it has {}", &self.name, expected_length, self.value.len());
        }

    }
    pub fn contains_key(&self, expected_key: K) {
        if !&self.value.contains_key(&expected_key) {
            panic!("Expected {} to contain {}, but it does not.",&self.name, &expected_key);
        }
    }

    pub fn does_not_contain_key(&self, not_expected_key: K) {
        if *&self.value.contains_key(&not_expected_key) {
            panic!("Expected {} to not to contain {}, but it does.",&self.name, &not_expected_key);
        }
    }
}

//TODO: S - add this to tests folder
#[cfg(test)]
mod test_hashmap_asserter {
    use super::*;

    #[test]
    fn test_has_length() {
        let mut hash_map = HashMap::<String,String>::new();
        hash_map.insert(String::from("key"),String::from("value"));

        assert_that!(&hash_map).has_length(1);

        assert_that_code!(||assert_that!(&hash_map).has_length(2))
            .panics()
            .with_message("Expected &hash_map to have length 2, but it has 1");
    }

    #[test]
    fn test_contains_key() {
        let mut hash_map = HashMap::<String,String>::new();
        hash_map.insert(String::from("key"),String::from("value"));

        assert_that!(&hash_map).contains_key(String::from("key"));

        assert_that_code!(||assert_that!(&hash_map).contains_key(String::from("key2")))
            .panics()
            .with_message("Expected &hash_map to contain key2, but it does not.");
    }

    #[test]
    fn test_does_not_contain_key() {
        let mut hash_map = HashMap::<String,String>::new();
        hash_map.insert(String::from("key"),String::from("value"));

        assert_that!(&hash_map).does_not_contain_key(String::from("key2"));

        assert_that_code!(||assert_that!(&hash_map).does_not_contain_key(String::from("key")))
            .panics()
            .with_message("Expected &hash_map to not to contain key, but it does.");
    }


    //has_length
    // is_empty
    //contains_entry
    //does_not_contain_entry

    //TODO: normal iterator assertions works with hashmap. Somehow limit it.
}