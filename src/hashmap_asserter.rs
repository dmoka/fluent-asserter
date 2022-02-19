use super::*;
use std::collections::HashMap;
use std::fmt::Display;
use std::hash::Hash;



//TODO: add trait

pub struct ValueAssertions<'a,K,V>  {
    key: &'a K,
    value: &'a V,
    hash_map_name: String
}

impl<'a,K,V> ValueAssertions<'a, K, V> where V: PartialEq + Display, K: Display {
    pub fn with_value(&'a self, expected_value: V) {
        if *self.value != expected_value {
            panic!("Expected {} to contain {} with value '{}', but it has the value '{}'.",&self.hash_map_name, &self.key, expected_value,&self.value)
        }
    }
}

impl<'a,K,V> Asserter<&HashMap<K,V>> where K : Eq + Hash + Display {
    pub fn has_length(&self, expected_length: usize) {
        if self.value.len() != expected_length {
            panic!("Expected {} to have length {}, but it has {}", &self.name, expected_length, self.value.len());
        }
    }

    pub fn is_empty(&self) {
        if self.value.len() > 0 {
            panic!("Expected {} to be empty, but it has length {}.",&self.name,self.value.len())
        }
    }
    pub fn is_not_empty(&self) {
        if self.value.len() == 0 {
            panic!("Expected {} to not to be empty, but it is.", &self.name)
        }
    }


    pub fn contains_key<'b>(&'a self, expected_key: &'a K) -> ValueAssertions<'a,K,V>{
        if !&self.value.contains_key(&expected_key) {
            panic!("Expected {} to contain {}, but it does not.",&self.name, &expected_key);
        }

        let value = &self.value.get(&expected_key);
        let value_for_key = value.unwrap();

        ValueAssertions {
            key: &expected_key,
            value: value_for_key,
            hash_map_name: String::from(&self.name)
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
    fn test_is_empty() {
        let mut hash_map = HashMap::<String,String>::new();

        assert_that!(&hash_map).is_empty();

        hash_map.insert(String::from("key"),String::from("value"));
        assert_that_code!(||assert_that!(&hash_map).is_empty())
            .panics()
            .with_message("Expected &hash_map to be empty, but it has length 1.")
    }

    #[test]
    fn test_is_not_empty() {
        let mut hash_map = HashMap::<String,String>::new();
        hash_map.insert(String::from("key"),String::from("value"));

        assert_that!(&hash_map).is_not_empty();

        let empty_hashmap = HashMap::<String,String>::new();
        assert_that_code!(||assert_that!(&empty_hashmap).is_not_empty())
            .panics()
            .with_message("Expected &empty_hashmap to not to be empty, but it is.")
    }

    #[test]
    fn test_contains_key() {
        let mut hash_map = HashMap::<String,String>::new();
        hash_map.insert(String::from("key"),String::from("value"));

        assert_that!(&hash_map).contains_key(&String::from("key"));
    }

    #[test]
    #[should_panic(expected="Expected &hash_map to contain key2, but it does not.")]
    fn test_contains_key_error_handling() {
        let mut hash_map = HashMap::<String,String>::new();
        hash_map.insert(String::from("key"),String::from("value"));

        assert_that!(&hash_map).contains_key(&String::from("key2"));
    }

    #[test]
    fn test_contains_key_with_value() {
        let mut hash_map = HashMap::<String,String>::new();
        hash_map.insert(String::from("key"),String::from("value"));

        assert_that!(&hash_map).contains_key(&String::from("key")).with_value(String::from("value"));

        assert_that_code!(||assert_that!(&hash_map).contains_key(&String::from("key")).with_value(String::from("value2")))
            .panics()
            .with_message("Expected &hash_map to contain key with value 'value2', but it has the value 'value'.");
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

    //TODO: normal iterator assertions works with hashmap. Somehow limit it.
}