use super::*;

//TODO: add And
impl<T> Asserter<T> where T : Into<String> + Clone{ //TODO: Display is also implemented for int,etc, so it would work
    pub fn contains(&self, expected_value_to_be_contained: &str) {
        let string = self.value.clone().into();
        let is_present = string.contains(expected_value_to_be_contained);
        
        if !is_present {
            panic!("The text {} is not present in string {}", expected_value_to_be_contained, string)
        }
    }

    pub fn starts_with(&self, expected_start: &str) {
        let string = self.value.clone().into();
        let starts_with = string.starts_with(expected_start);

        if !starts_with {
            panic!("The actual text {} does not start with {}", string, expected_start)
        }
    }

    pub fn ends_with(&self, expected_start: &str) {
        let string = self.value.clone().into();
        let ends_with = string.ends_with(expected_start);

        if !ends_with {
            panic!("The actual text {} does not end with {}", string, expected_start)
        }
    }

    pub fn is_empty(&self){
        let string = self.value.clone().into();

        if !string.is_empty() {
            panic!("The text {} is not empty", string)
        }
    }

    pub fn is_not_empty(&self){
        let string = self.value.clone().into();

        if string.is_empty() {
            panic!("The text {} is empty", string)
        }
    }

    pub fn has_length(&self, expected_length: usize){
        let string = self.value.clone().into();
        let len = string.len();

        assert_eq!(len, expected_length); //TODO: use custom panic
    }

    pub fn contains_all(&self, args: &[&str]) {
        //TODO: create ctor field value with string?
        let string = self.value.clone().into();
        let contains_all = args.into_iter().all(|&w| string.contains(&w));

        //TODO: add the words in the error message which are not present
        if !contains_all {
            panic!("The word {} does not contain all the words specified",string);
        }
    }

    pub fn contains_any(&self, args: &[&str]) {
        let string = self.value.clone().into();
        let contains_any = args.into_iter().any(|&w| string.contains(&w));

        if !contains_any {
            panic!("The word {} does not contain any of the words specified", string);
        }
    }
}