use super::*;
use std::collections::HashMap;
use std::fmt::Display;
use std::hash::Hash;

//TODO: finish it
//TODO: do it for non reference type too?
impl<K,V> Asserter<&HashMap<K,V>> where K : Eq + Hash + Display {
    pub fn contains_key(&self, expected_key: K) {
        if !&self.value.contains_key(&expected_key) {
            panic!("Expected {} to contain {}, but it does not.",&self.name, &expected_key);
        }
    }
}

//TODO: S - add this to tests folder
#[cfg(test)]
mod test_hashmap_asserter {
    use super::*;

    #[test]
    fn test_contains_key() {
        let mut hash_map = HashMap::<String,String>::new();
        hash_map.insert(String::from("key"),String::from("value"));

        assert_that!(&hash_map).contains_key(String::from("key"));

        assert_that_code!(||assert_that!(&hash_map).contains_key(String::from("key2")))
            .panics()
            .with_message("Expected &hash_map to contain key2, but it does not.");
    }

    //TODO: normal iterator assertions works with hashmap. Somehow limit it.
}