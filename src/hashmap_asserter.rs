use super::*;
use std::collections::HashMap;

//TODO: finish it
//TODO: do it for non reference type too?
impl<K,V> Asserter<&HashMap<K,V>>  {
    pub fn contains_key(&self, expected_key: String) {
        /*if (self.value).contains_key(&expected_key) {

        }*/
    }
}

//TODO: S - add this to tests folder
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    #[ignore]
    fn test_is_true() {
        let mut hash_map = HashMap::<String,String>::new();
        hash_map.insert(String::from("key"),String::from("value"));

        assert_that!(&hash_map).contains_key(String::from("key"));

        assert_that_code!(||assert_that!(&hash_map).contains_key(String::from("key2")))
            .panics();


    }

    //TODO: normal iterator assertions works with hashmap. Somehow limit it.
}